//! Matchers: expressions.

use std::cell::Cell;

use super::super::engine::Lexer;
use super::names::{Dots, Ident, is_op_continue, is_op_head};
use crate::classes::{is_letter, is_mark, is_number, is_space};
use crate::rules::Chars;

/// What the matchers of this file remember about a line.
///
/// Each field is the last run of a scan that the matchers repeat from many
/// starts inside one run, so a hostile line stays linear.
#[derive(Debug, Default)]
pub(in crate::swift) struct Memo {
    /// A run of `\w` characters, `from..end`.
    word: Cell<Option<(usize, usize)>>,
    /// A run of `\s` characters, `from..end`.
    spaces: Cell<Option<(usize, usize)>>,
    /// A run of `[^),\s]`, `from..end`.
    plain: Cell<Option<(usize, usize)>>,
    /// The first `)` or `,` from `from` on, or the end of the line: `(from,
    /// at)`.
    close: Cell<Option<(usize, usize)>>,
    /// A chain `[0-9]+(?:\.[0-9]+)*`: its start, its end and its last `.`.
    version: Cell<Option<(usize, usize, Option<usize>)>>,
}

/// Which operator the six operator patterns want, by the spacing around it.
#[derive(Clone, Copy, Debug)]
enum Spacing {
    /// After an opening character or a space, and before something else.
    Prefix,
    /// After something else, and before a closing character or a space.
    Postfix,
    /// Either.
    Any,
}

/// A platform name an availability condition takes, with or without
/// `ApplicationExtension`.
fn is_platform(word: &[u8]) -> bool {
    let base = word.strip_suffix(b"ApplicationExtension").unwrap_or(word);
    [
        b"iOS".as_slice(),
        b"macOS",
        b"OSX",
        b"watchOS",
        b"tvOS",
        b"visionOS",
        b"UIKitForMac",
    ]
    .contains(&base)
}

/// Which `#if` a compiler-control head reads: any condition, or the literal
/// `false`, which the grammar lexes as a disabled block.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Condition {
    /// `^\s*(#)(if|elseif)\s+`.
    Any,
    /// The same, then `(false)\b`.
    False,
}

impl Lexer<'_> {
    /// The end of the `\s*` from `i`, kept for the run it ends.
    pub(in crate::swift) fn spaces(&self, i: usize) -> usize {
        let memo = &self.x.expressions.spaces;
        if let Some((from, end)) = memo.get()
            && from <= i
            && i < end
        {
            return end;
        }
        let end = self.space_end(i);
        if end > i {
            memo.set(Some((i, end)));
        }
        end
    }

    /// The end of the `\w*` from `i`, kept for the run it ends.
    fn word_run(&self, i: usize) -> usize {
        let memo = &self.x.expressions.word;
        if let Some((from, end)) = memo.get()
            && from <= i
            && i < end
        {
            return end;
        }
        let end = self.word_end(i);
        if end > i {
            memo.set(Some((i, end)));
        }
        end
    }

    /// The end of the `[^),\s]*` from `i`, kept for the run it ends.
    fn plain_end(&self, i: usize) -> usize {
        let memo = &self.x.expressions.plain;
        if let Some((from, end)) = memo.get()
            && from <= i
            && i < end
        {
            return end;
        }
        let mut end = i;
        while end < self.lim() && !matches!(self.at(end), b')' | b',') && self.space_at(end) == 0 {
            end = self.char_end(end);
        }
        self.spend(end - i);
        if end > i {
            memo.set(Some((i, end)));
        }
        end
    }

    /// The first `)` or `,` at or after `i` on the line, which is where
    /// `.*?(?=[),])` stops.
    ///
    /// `.` stops at the line feed, so a line without one has no answer.
    fn close_from(&self, i: usize) -> Option<usize> {
        let memo = &self.x.expressions.close;
        let at = match memo.get() {
            Some((from, at)) if from <= i && i <= at => at,
            _ => {
                let at = self.skip(i, |b| !matches!(b, b')' | b','));
                memo.set(Some((i, at)));
                at
            }
        };
        (at < self.lim()).then_some(at)
    }

    /// The end of the first `//` or `/*` at or after `i`, or of the line: where
    /// `.*?(?=$|//|/\*)` stops.
    fn comment_or_eol(&self, mut i: usize) -> usize {
        loop {
            i = self.skip(i, |b| b != b'/');
            if i >= self.lim() || matches!(self.at(i + 1), b'/' | b'*') {
                return i;
            }
            i += 1;
        }
    }

    /// The end of `[0-9]+(?:\.[0-9]+)*\b` at `i`, after the give-back the
    /// backtracking does.
    ///
    /// The chain ends in a digit, so `\b` holds when no word character
    /// follows. When one does, the only shorter end that helps is before the
    /// last `.[0-9]+`, where a `.` follows.
    pub(in crate::swift) fn version_end(&self, i: usize) -> Option<usize> {
        let memo = &self.x.expressions.version;
        let (full, last_dot) = match memo.get() {
            Some((from, end, dot)) if from < i && i < end && self.prev_is(i, b'.') => {
                (end, dot.filter(|&d| d > i))
            }
            _ => {
                let mut end = self.skip(i, |b| b.is_ascii_digit());
                if end == i {
                    return None;
                }
                let mut dot = None;
                while self.at(end) == b'.' && self.at(end + 1).is_ascii_digit() {
                    dot = Some(end);
                    end = self.skip(end + 1, |b| b.is_ascii_digit());
                }
                memo.set(Some((i, end, dot)));
                (end, dot)
            }
        };
        if self.word_at(full).is_none() {
            Some(full)
        } else {
            last_dot
        }
    }

    /// An identifier after an optional `#`: the name and where its own text
    /// starts.
    fn hashed_ident(&self, i: usize) -> Option<(Ident, usize)> {
        let start = i + usize::from(self.at(i) == b'#');
        self.ident_at(start).map(|id| (id, start))
    }

    /// Records the groups of an identifier's opening and closing backtick.
    fn cap_expr_quotes(&mut self, open: usize, id: Ident, quote: usize, close: usize) {
        self.cap(quote, open, id.body);
        self.cap(close, id.tail, id.end);
    }

    /// The `#` of a line that starts with `^\s*#`.
    fn line_hash(&self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let hash = self.spaces(0);
        (self.at(hash) == b'#').then_some(hash)
    }

    /// The parts of `^\s*(#)(if|elseif)\s+`, and for `Condition::False` of
    /// `(false)\b` after it: the `#`, the keyword's end, the end of the
    /// space and the end of `false`.
    fn compiler_control_head(
        &self,
        p: usize,
        condition: Condition,
    ) -> Option<(usize, usize, usize, usize)> {
        let hash = self.line_hash(p)?;
        for word in [b"if".as_slice(), b"elseif"] {
            let Some(e) = self.lit(hash + 1, word) else {
                continue;
            };
            let sp = self.spaces(e);
            if sp == e {
                continue;
            }
            if condition == Condition::Any {
                return Some((hash, e, sp, sp));
            }
            if let Some(f) = self.keyword(sp, b"false") {
                return Some((hash, e, sp, f));
            }
        }
        None
    }

    /// `\b(name)\s*(\()\s*(?:(one of words)|\w+)\s*(\))`: the condition
    /// `arch(…)` and `os(…)`.
    ///
    /// The word is `\w+` whole, because a listed word that is a prefix of it
    /// leaves a word character where `\s*\)` must continue.
    fn condition_call(&mut self, p: usize, name: &[u8], words: &[&[u8]]) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        let word = self.lit(p, name)?;
        let open = self.spaces(word);
        if self.at(open) != b'(' {
            return None;
        }
        let arg = self.spaces(open + 1);
        let e = self.word_run(arg);
        if e == arg {
            return None;
        }
        let close = self.spaces(e);
        if self.at(close) != b')' {
            return None;
        }
        self.cap(1, p, word);
        self.cap(2, open, open + 1);
        if words.contains(&self.text(arg, e)) {
            self.cap(3, arg, e);
        }
        self.cap(4, close, close + 1);
        Some(close + 1)
    }

    /// `\b(name)\s*(\()`.
    fn word_and_open(&mut self, p: usize, name: &[u8]) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        let word = self.lit(p, name)?;
        let open = self.spaces(word);
        if self.at(open) != b'(' {
            return None;
        }
        self.cap(1, p, word);
        self.cap(2, open, open + 1);
        Some(open + 1)
    }

    /// `((@)name)(\()`.
    fn attribute_open(&mut self, p: usize, name: &[u8]) -> Option<usize> {
        let e = self.lit(p, name)?;
        if self.at(e) != b'(' {
            return None;
        }
        self.cap(1, p, e);
        self.cap(2, p, p + 1);
        self.cap(3, e, e + 1);
        Some(e + 1)
    }

    /// The text before `p` that lets a prefix operator start there:
    /// `(?<=^|[(,:;\[{\s])`.
    fn before_open(&self, p: usize) -> bool {
        p == 0
            || self
                .char_before(p)
                .is_some_and(|c| matches!(c, '(' | ',' | ':' | ';' | '[' | '{') || is_space(c))
    }

    /// What lets a postfix operator end: `[]),:;}\s]|\z`, with the line feed
    /// counted as a space. `\z` holds at the end of a text that no line feed
    /// follows, as in a capture lexed again.
    fn before_close(&self, p: usize) -> bool {
        p >= self.lim()
            || matches!(self.at(p), b']' | b')' | b',' | b':' | b';' | b'}')
            || self.space_at(p) > 0
    }

    /// The possessive run `(?:(?!//|/\*|\*/)[operator character])++` from `i`,
    /// which never gives a character back, so one pass is the answer.
    fn operator_run(&self, i: usize, dots: Dots) -> Option<usize> {
        let mut end = i;
        while let Some((c, next)) = self.char_at(end)
            && !(self.starts(end, b"//") || self.starts(end, b"/*") || self.starts(end, b"*/"))
            && (is_op_head(c) || is_op_continue(c) || (dots == Dots::Taken && c == '.'))
        {
            end = next;
        }
        self.spend(end - i);
        (end > i).then_some(end)
    }

    /// `\G(lookbehind)(\.)?(operator run)++(lookahead)` with the spacing that
    /// `spacing` names; when `dots` takes them the run follows a `.` and may
    /// contain more.
    fn operator_at(&mut self, p: usize, dots: Dots, spacing: Spacing) -> Option<usize> {
        if !(self.g && p == self.pos) {
            return None;
        }
        match spacing {
            Spacing::Prefix if !self.before_open(p) => return None,
            Spacing::Postfix if self.before_open(p) => return None,
            Spacing::Prefix | Spacing::Postfix | Spacing::Any => {}
        }
        let start = match dots {
            Dots::Taken => (self.at(p) == b'.').then_some(p + 1)?,
            Dots::Refused => p,
        };
        let end = self.operator_run(start, dots)?;
        match spacing {
            Spacing::Prefix if self.before_close(end) => return None,
            Spacing::Postfix if !self.before_close(end) => return None,
            Spacing::Prefix | Spacing::Postfix | Spacing::Any => {}
        }
        self.cap(0, p, end);
        Some(end)
    }
}

impl Lexer<'_> {
    /// `^\s*(#)(if|elseif)\s+(false)\b.*?(?=$|//|/\*)`, for
    /// `CompilerControlAt0`.
    pub(in crate::swift) fn rx_compiler_control_at_0(&mut self, p: usize) -> Option<usize> {
        let (hash, kw, sp, f) = self.compiler_control_head(p, Condition::False)?;
        let end = self.comment_or_eol(f);
        self.cap(0, p, end);
        self.cap(1, hash, hash + 1);
        self.cap(2, hash + 1, kw);
        self.cap(3, sp, f);
        Some(end)
    }

    /// `(?=^\s*(#(e(?:lseif|lse|ndif)))\b)`, for `CompilerControlAt0End`.
    pub(in crate::swift) fn rx_compiler_control_at_0_end(&mut self, p: usize) -> Option<usize> {
        let hash = self.line_hash(p)?;
        self.word_in(hash + 1, &[b"elseif", b"else", b"endif"])?;
        Some(p)
    }

    /// `^\s*(#)(if|elseif)\s+`, for `CompilerControlAt1`.
    pub(in crate::swift) fn rx_compiler_control_at_1(&mut self, p: usize) -> Option<usize> {
        let (hash, kw, sp, _) = self.compiler_control_head(p, Condition::Any)?;
        self.cap(1, hash, hash + 1);
        self.cap(2, hash + 1, kw);
        Some(sp)
    }

    /// `(?=\s*/[*/])|$`, for `CompilerControlAt1End`.
    pub(in crate::swift) fn rx_compiler_control_at_1_end(&mut self, p: usize) -> Option<usize> {
        let q = self.spaces(p);
        if self.at(q) == b'/' && matches!(self.at(q + 1), b'/' | b'*') {
            return Some(p);
        }
        self.at_eol(p).then_some(p)
    }

    /// `(&&|\|\|)`, for `CompilerControlAt1At0`.
    pub(in crate::swift) fn rx_compiler_control_at_1_at_0(&mut self, p: usize) -> Option<usize> {
        (self.starts(p, b"&&") || self.starts(p, b"||")).then_some(p + 2)
    }

    /// `\b(true|false)\b`, for `CompilerControlAt1At1`, `LiteralsBoolean`.
    pub(in crate::swift) fn rx_compiler_control_at_1_at_1(&mut self, p: usize) -> Option<usize> {
        self.keyword(p, b"true")
            .or_else(|| self.keyword(p, b"false"))
    }

    /// `\b(arch)\s*(\()\s*(?:
    /// (arm|arm64|powerpc64|powerpc64le|i386|x86_64|s390x)|\w+)\s*(\))`, for
    /// `CompilerControlAt1At2`.
    pub(in crate::swift) fn rx_compiler_control_at_1_at_2(&mut self, p: usize) -> Option<usize> {
        self.condition_call(
            p,
            b"arch",
            &[
                b"arm",
                b"arm64",
                b"powerpc64",
                b"powerpc64le",
                b"i386",
                b"x86_64",
                b"s390x",
            ],
        )
    }

    /// `\b(os)\s*(\()\s*(?:
    /// (macOS|OSX|iOS|tvOS|watchOS|visionOS|Android|Linux|FreeBSD|Windows|PS4)|\
    /// w+)\s*(\))`, for `CompilerControlAt1At3`.
    pub(in crate::swift) fn rx_compiler_control_at_1_at_3(&mut self, p: usize) -> Option<usize> {
        self.condition_call(
            p,
            b"os",
            &[
                b"macOS",
                b"OSX",
                b"iOS",
                b"tvOS",
                b"watchOS",
                b"visionOS",
                b"Android",
                b"Linux",
                b"FreeBSD",
                b"Windows",
                b"PS4",
            ],
        )
    }

    /// `\b(canImport)\s*(\()([_\p{L}][_\p{L}\p{N}\p{M}]*)(\))`, for
    /// `CompilerControlAt1At4`.
    pub(in crate::swift) fn rx_compiler_control_at_1_at_4(&mut self, p: usize) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        let word = self.lit(p, b"canImport")?;
        let open = self.spaces(word);
        if self.at(open) != b'(' {
            return None;
        }
        let (c, next) = self.char_at(open + 1)?;
        if !(c == '_' || is_letter(c)) {
            return None;
        }
        let name_end = self.ident_tail_end(next);
        if self.at(name_end) != b')' {
            return None;
        }
        self.cap(1, p, word);
        self.cap(2, open, open + 1);
        self.cap(3, open + 1, name_end);
        self.cap(4, name_end, name_end + 1);
        Some(name_end + 1)
    }

    /// `\b(targetEnvironment)\s*(\()`, for `CompilerControlAt1At5`.
    pub(in crate::swift) fn rx_compiler_control_at_1_at_5(&mut self, p: usize) -> Option<usize> {
        self.word_and_open(p, b"targetEnvironment")
    }

    /// `(\))|$`, for `CompilerControlAt1At5End`, `CompilerControlAt1At6End`.
    pub(in crate::swift) fn rx_compiler_control_at_1_at_5_end(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        if self.at(p) == b')' {
            self.cap(1, p, p + 1);
            return Some(p + 1);
        }
        self.at_eol(p).then_some(p)
    }

    /// `\b(simulator|UIKitForMac)\b`, for `CompilerControlAt1At5At0`.
    pub(in crate::swift) fn rx_compiler_control_at_1_at_5_at_0(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        self.word_in(p, &[b"simulator", b"UIKitForMac"])
    }

    /// `\b(swift|compiler)\s*(\()`, for `CompilerControlAt1At6`.
    pub(in crate::swift) fn rx_compiler_control_at_1_at_6(&mut self, p: usize) -> Option<usize> {
        self.word_and_open(p, b"swift")
            .or_else(|| self.word_and_open(p, b"compiler"))
    }

    /// `>=|<`, for `CompilerControlAt1At6At0`.
    pub(in crate::swift) fn rx_compiler_control_at_1_at_6_at_0(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        if self.starts(p, b">=") {
            Some(p + 2)
        } else {
            self.starts(p, b"<").then_some(p + 1)
        }
    }

    /// `\b[0-9]+(?:\.[0-9]+)*\b`, for `CompilerControlAt1At6At1`,
    /// `AttributesAt0At1At0`.
    pub(in crate::swift) fn rx_compiler_control_at_1_at_6_at_1(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        self.version_end(p)
    }

    /// `^\s*(#)(e(?:lse|ndif))(.*?)(?=$|//|/\*)`, for `CompilerControlAt2`.
    pub(in crate::swift) fn rx_compiler_control_at_2(&mut self, p: usize) -> Option<usize> {
        let hash = self.line_hash(p)?;
        let e = self
            .lit(hash + 1, b"else")
            .or_else(|| self.lit(hash + 1, b"endif"))?;
        let end = self.comment_or_eol(e);
        self.cap(1, hash, hash + 1);
        self.cap(2, hash + 1, e);
        self.cap(3, e, end);
        Some(end)
    }

    /// `^\s*(#)(sourceLocation)((\()([^)]*)(\)))(.*?)(?=$|//|/\*)`, for
    /// `CompilerControlAt3`.
    pub(in crate::swift) fn rx_compiler_control_at_3(&mut self, p: usize) -> Option<usize> {
        let hash = self.line_hash(p)?;
        let word = self.lit(hash + 1, b"sourceLocation")?;
        if self.at(word) != b'(' {
            return None;
        }
        let close = self.skip(word + 1, |b| b != b')');
        if close >= self.lim() {
            return None;
        }
        let end = self.comment_or_eol(close + 1);
        self.cap(1, hash, hash + 1);
        self.cap(2, hash + 1, word);
        self.cap(4, word, word + 1);
        self.cap(5, word + 1, close);
        self.cap(6, close, close + 1);
        self.cap(7, close + 1, end);
        Some(end)
    }

    /// `((@)available)(\()`, for `AttributesAt0`.
    pub(in crate::swift) fn rx_attributes_at_0(&mut self, p: usize) -> Option<usize> {
        self.attribute_open(p, b"@available")
    }

    /// `\b(swift|(?:iOS|macOS|OSX|watchOS|tvOS|visionOS|UIKitForMac)(?:
    /// ApplicationExtension)?)\b(?:\s+([0-9]+(?:\.[0-9]+)*)\b)?`, for
    /// `AttributesAt0At0`.
    pub(in crate::swift) fn rx_attributes_at_0_at_0(&mut self, p: usize) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        let e = self.word_end(p);
        let word = self.text(p, e);
        if word != b"swift" && !is_platform(word) {
            return None;
        }
        self.cap(1, p, e);
        let q = self.spaces(e);
        if q > e
            && let Some(v) = self.version_end(q)
        {
            self.cap(2, q, v);
            return Some(v);
        }
        Some(e)
    }

    /// `\b(introduced|deprecated|obsoleted)\s*(:)\s*`, for `AttributesAt0At1`.
    /// The grammar spells the three words with a shared suffix; this is the
    /// same language.
    pub(in crate::swift) fn rx_attributes_at_0_at_1(&mut self, p: usize) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        let word = [b"introduced".as_slice(), b"deprecated", b"obsoleted"]
            .iter()
            .find_map(|w| self.lit(p, w))?;
        let colon = self.spaces(word);
        if self.at(colon) != b':' {
            return None;
        }
        self.cap(1, p, word);
        self.cap(2, colon, colon + 1);
        Some(self.spaces(colon + 1))
    }

    /// `\b(message|renamed)\s*(:)\s*(?=")`, for `AttributesAt0At2`.
    pub(in crate::swift) fn rx_attributes_at_0_at_2(&mut self, p: usize) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        let word = self
            .lit(p, b"message")
            .or_else(|| self.lit(p, b"renamed"))?;
        let colon = self.spaces(word);
        if self.at(colon) != b':' {
            return None;
        }
        let end = self.spaces(colon + 1);
        if self.at(end) != b'"' {
            return None;
        }
        self.cap(1, p, word);
        self.cap(2, colon, colon + 1);
        Some(end)
    }

    /// `((@)objc)(\()`, for `AttributesAt1`.
    pub(in crate::swift) fn rx_attributes_at_1(&mut self, p: usize) -> Option<usize> {
        self.attribute_open(p, b"@objc")
    }

    /// `\w*(?::(?:\w*:)*(\w*))?`, for `AttributesAt1At0`.
    #[expect(
        clippy::unnecessary_wraps,
        reason = "every matcher answers with an optional end, and this pattern always matches"
    )]
    pub(in crate::swift) fn rx_attributes_at_1_at_0(&mut self, p: usize) -> Option<usize> {
        let mut end = self.word_run(p);
        if self.at(end) == b':' {
            let mut start = end + 1;
            let mut word = self.word_run(start);
            while self.at(word) == b':' {
                start = word + 1;
                word = self.word_run(start);
            }
            self.cap(1, start, word);
            end = word;
        }
        Some(end)
    }

    /// ``(@)(?<q>`?)[_\p{L}][_\p{L}\p{N}\p{M}]*(\k<q>)``, for `AttributesAt2`.
    pub(in crate::swift) fn rx_attributes_at_2(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b'@' {
            return None;
        }
        let id = self.ident_at(p + 1)?;
        self.cap(0, p, id.end);
        self.cap(1, p, p + 1);
        self.cap_expr_quotes(p + 1, id, 2, 3);
        Some(id.end)
    }

    /// `(?!\G\()`, for `AttributesAt2End`.
    pub(in crate::swift) fn rx_attributes_at_2_end(&mut self, p: usize) -> Option<usize> {
        (!(self.g && p == self.pos && self.at(p) == b'(')).then_some(p)
    }

    /// `\(`, for `AttributesAt2At0`, `DeclarationsAvailableTypesTupleType`,
    /// `ExpressionsWithoutTrailingClosuresOrMemberReferencesParenthesizedExpression`,
    /// `LiteralsStringRawStringGutsAt2At1`, `LiteralsStringStringGutsAt2At1`.
    pub(in crate::swift) fn rx_attributes_at_2_at_0(&mut self, p: usize) -> Option<usize> {
        if self.at(p) == b'(' {
            self.cap(0, p, p + 1);
            Some(p + 1)
        } else {
            None
        }
    }

    /// `\b(?:((?:throws\s+|rethrows\s+)async)|((?:|re)throws)|(async))\b`, for
    /// `AsyncThrows`.
    pub(in crate::swift) fn rx_async_throws(&mut self, p: usize) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        for word in [b"throws".as_slice(), b"rethrows"] {
            if let Some(e) = self.lit(p, word) {
                let q = self.spaces(e);
                if q > e
                    && let Some(a) = self.lit(q, b"async")
                    && self.word_at(a).is_none()
                {
                    self.cap(1, p, a);
                    return Some(a);
                }
            }
        }
        if let Some(e) = self
            .keyword(p, b"throws")
            .or_else(|| self.keyword(p, b"rethrows"))
        {
            self.cap(2, p, e);
            return Some(e);
        }
        let e = self.keyword(p, b"async")?;
        self.cap(3, p, e);
        Some(e)
    }

    /// `\$[0-9]+`, for
    /// `ExpressionsWithoutTrailingClosuresOrMemberReferencesClosureParameter`.
    pub(in crate::swift) fn rx_expressions_without_trailing_closures_or_member_references_closure_parameter(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        if self.at(p) != b'$' {
            return None;
        }
        let end = self.skip(p + 1, |b| b.is_ascii_digit());
        (end > p + 1).then_some(end)
    }

    /// `\b(is\b|as([!?]\B|\b))`, for `OperatorsAt0`.
    pub(in crate::swift) fn rx_operators_at_0(&mut self, p: usize) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        if let Some(e) = self.keyword(p, b"is") {
            return Some(e);
        }
        let e = self.lit(p, b"as")?;
        if matches!(self.at(e), b'!' | b'?') && self.word_at(e + 1).is_none() {
            return Some(e + 1);
        }
        self.word_at(e).is_none().then_some(e)
    }

    /// `(?=(?<oph>[-!%\&*+/<-?^|~¡-§©«¬®°±¶»¿×÷‖‗†-‧‰-‾⁁-⁓⁕-⁞←-⏿─-❵➔-⯿⸀-⹿、。〃
    /// 〈-〰])|\.(\g<oph>|[.̀-ͯ᷀-᷿⃐-⃿︀-️︠-︯\x{E0100}-\x{E01EF}]))`, for `OperatorsAt1`.
    pub(in crate::swift) fn rx_operators_at_1(&mut self, p: usize) -> Option<usize> {
        let (c, end) = self.char_at(p)?;
        if is_op_head(c) {
            return Some(p);
        }
        if c == '.' {
            let (d, _) = self.char_at(end)?;
            if is_op_head(d) || d == '.' || is_op_continue(d) {
                return Some(p);
            }
        }
        None
    }

    /// `\G(?<=^|[(,:;\[{\s])((?!(//|/\*|\*/))([-!%\&*+/<-?^|~¡-§©«¬®°±¶»¿×÷̀-ͯ᷀-᷿‖‗
    /// †-‧‰-‾⁁-⁓⁕-⁞⃐-⃿←-⏿─-❵➔-⯿⸀-⹿、。〃〈-〰︀-️︠-︯\x{E0100}-\x{E01EF}]))++(?![]),:;
    /// }\s]|\z)`, for `OperatorsAt1At0`.
    pub(in crate::swift) fn rx_operators_at_1_at_0(&mut self, p: usize) -> Option<usize> {
        self.operator_at(p, Dots::Refused, Spacing::Prefix)
    }

    /// `\G(?<!^|[(,:;\[{\s])((?!(//|/\*|\*/))([-!%\&*+/<-?^|~¡-§©«¬®°±¶»¿×÷̀-ͯ᷀-᷿‖‗
    /// †-‧‰-‾⁁-⁓⁕-⁞⃐-⃿←-⏿─-❵➔-⯿⸀-⹿、。〃〈-〰︀-️︠-︯\x{E0100}-\x{E01EF}]))++(?=[]),:;
    /// }\s]|\z)`, for `OperatorsAt1At1`.
    pub(in crate::swift) fn rx_operators_at_1_at_1(&mut self, p: usize) -> Option<usize> {
        self.operator_at(p, Dots::Refused, Spacing::Postfix)
    }

    /// `\G((?!(//|/\*|\*/))([-!%\&*+/<-?^|~¡-§©«¬®°±¶»¿×÷̀-ͯ᷀-᷿‖‗†-‧‰-‾⁁-⁓⁕
    /// -⁞⃐-⃿←-⏿─-❵➔-⯿⸀-⹿、。〃〈-〰︀-️︠-︯\x{E0100}-\x{E01EF}]))++`, for
    /// `OperatorsAt1At2`.
    pub(in crate::swift) fn rx_operators_at_1_at_2(&mut self, p: usize) -> Option<usize> {
        self.operator_at(p, Dots::Refused, Spacing::Any)
    }

    /// `\G(?<=^|[(,:;\[{\s])\.((?!(//|/\*|\*/))([-!%\&*+./<-?^|~¡-§©«¬®°±¶»¿
    /// ×÷̀-ͯ᷀-᷿‖‗†-‧‰-‾⁁-⁓⁕-⁞⃐-⃿←-⏿─-❵➔-⯿⸀-⹿、。〃〈-〰︀-️︠-︯\x{E0100}-\x{E01EF}]))++(?!
    /// []),:;}\s]|\z)`, for `OperatorsAt1At3`.
    pub(in crate::swift) fn rx_operators_at_1_at_3(&mut self, p: usize) -> Option<usize> {
        self.operator_at(p, Dots::Taken, Spacing::Prefix)
    }

    /// `\G(?<!^|[(,:;\[{\s])\.((?!(//|/\*|\*/))([-!%\&*+./<-?^|~¡-§©«¬®°±¶»¿
    /// ×÷̀-ͯ᷀-᷿‖‗†-‧‰-‾⁁-⁓⁕-⁞⃐-⃿←-⏿─-❵➔-⯿⸀-⹿、。〃〈-〰︀-️︠-︯\x{E0100}-\x{E01EF}]))++(?
    /// =[]),:;}\s]|\z)`, for `OperatorsAt1At4`.
    pub(in crate::swift) fn rx_operators_at_1_at_4(&mut self, p: usize) -> Option<usize> {
        self.operator_at(p, Dots::Taken, Spacing::Postfix)
    }

    /// `\G\.((?!(//|/\*|\*/))([-!%\&*+./<-?^|~¡-§©«¬®°±¶»¿×÷̀-ͯ᷀-᷿‖‗†-‧‰-‾⁁-⁓⁕
    /// -⁞⃐-⃿←-⏿─-❵➔-⯿⸀-⹿、。〃〈-〰︀-️︠-︯\x{E0100}-\x{E01EF}]))++`, for
    /// `OperatorsAt1At5`.
    pub(in crate::swift) fn rx_operators_at_1_at_5(&mut self, p: usize) -> Option<usize> {
        self.operator_at(p, Dots::Taken, Spacing::Any)
    }

    /// ``((?<q1>`?)[_\p{L}][_\p{L}\p{N}\p{M}]*(\k<q1>))\(((((?<q2>`?)[_\
    /// p{L}][_\ p{L}\p{N}\p{M}]*(\k<q2>)):)+)\)``, for
    /// `ExpressionsWithoutTrailingClosuresOrMemberReferencesCompoundName`.
    pub(in crate::swift) fn rx_expressions_without_trailing_closures_or_member_references_compound_name(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        let id = self.ident_at(p)?;
        let open = id.end;
        if self.at(open) != b'(' {
            return None;
        }
        let mut i = open + 1;
        let mut count = 0;
        while let Some(part) = self.ident_at(i)
            && self.at(part.end) == b':'
        {
            count += 1;
            i = part.end + 1;
        }
        if count == 0 || self.at(i) != b')' {
            return None;
        }
        self.cap(1, p, id.end);
        self.cap_expr_quotes(p, id, 2, 3);
        self.cap(4, open + 1, i);
        Some(i + 1)
    }

    /// `(?<!\.)\b(if|guard|switch|for)\b`, for `ConditionalsAt0`.
    pub(in crate::swift) fn rx_conditionals_at_0(&mut self, p: usize) -> Option<usize> {
        if self.prev_is(p, b'.') {
            return None;
        }
        self.word_in(p, &[b"if", b"guard", b"switch", b"for"])
            .inspect(|&e| self.cap(1, p, e))
    }

    /// `(?=\{)`, for `ConditionalsAt0End`.
    pub(in crate::swift) fn rx_conditionals_at_0_end(&mut self, p: usize) -> Option<usize> {
        (self.at(p) == b'{').then_some(p)
    }

    /// `(?<!\.)\b(while)\b`, for `ConditionalsAt1`.
    pub(in crate::swift) fn rx_conditionals_at_1(&mut self, p: usize) -> Option<usize> {
        if self.prev_is(p, b'.') {
            return None;
        }
        self.keyword(p, b"while").inspect(|&e| self.cap(1, p, e))
    }

    /// `(?=\{)|$`, for `ConditionalsAt1End`.
    pub(in crate::swift) fn rx_conditionals_at_1_end(&mut self, p: usize) -> Option<usize> {
        (self.at(p) == b'{' || self.at_eol(p)).then_some(p)
    }

    /// `\B(#(?:un)?available)(\()`, for
    /// `ExpressionsWithoutTrailingClosuresOrMemberReferencesAvailabilityCondition`.
    pub(in crate::swift) fn rx_expressions_without_trailing_closures_or_member_references_availability_condition(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        if self.boundary(p) {
            return None;
        }
        let word = self
            .lit(p, b"#unavailable")
            .or_else(|| self.lit(p, b"#available"))?;
        if self.at(word) != b'(' {
            return None;
        }
        self.cap(1, p, word);
        self.cap(2, word, word + 1);
        Some(word + 1)
    }

    /// `\s*\b((?:iOS|macOS|OSX|watchOS|tvOS|visionOS|UIKitForMac)(?:
    /// ApplicationExtension)?)\b\s+([0-9]+(?:\.[0-9]+)*)\b`, for
    /// `ExpressionsWithoutTrailingClosuresOrMemberReferencesAvailabilityConditionAt0`.
    pub(in crate::swift) fn rx_expressions_without_trailing_closures_or_member_references_availability_condition_at_0(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        let start = self.spaces(p);
        if !self.boundary(start) {
            return None;
        }
        let e = self.word_end(start);
        if !is_platform(self.text(start, e)) {
            return None;
        }
        let q = self.spaces(e);
        if q == e {
            return None;
        }
        let v = self.version_end(q)?;
        self.cap(1, start, e);
        self.cap(2, q, v);
        Some(v)
    }

    /// `(\*)\s*(.*?)(?=[),])`, for
    /// `ExpressionsWithoutTrailingClosuresOrMemberReferencesAvailabilityConditionAt1`.
    pub(in crate::swift) fn rx_expressions_without_trailing_closures_or_member_references_availability_condition_at_1(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        if self.at(p) != b'*' {
            return None;
        }
        let e = self.spaces(p + 1);
        let q = self.close_from(e)?;
        self.cap(1, p, p + 1);
        self.cap(2, e, q);
        Some(q)
    }

    /// `[^),\s]+`, for
    /// `ExpressionsWithoutTrailingClosuresOrMemberReferencesAvailabilityConditionAt2`.
    pub(in crate::swift) fn rx_expressions_without_trailing_closures_or_member_references_availability_condition_at_2(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        let end = self.plain_end(p);
        (end > p).then_some(end)
    }

    /// ``(#?(?<q>`?)[_\p{L}][_\p{L}\p{N}\p{M}]*(\k<q>))\s*(\()``, for
    /// `ExpressionsWithoutTrailingClosuresOrMemberReferencesFunctionOrMacroCallExpressionAt0`.
    pub(in crate::swift) fn rx_expressions_without_trailing_closures_or_member_references_function_or_macro_call_expression_at_0(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        let (id, s) = self.hashed_ident(p)?;
        let open = self.spaces(id.end);
        if self.at(open) != b'(' {
            return None;
        }
        self.cap(1, p, id.end);
        self.cap_expr_quotes(s, id, 2, 3);
        self.cap(4, open, open + 1);
        Some(open + 1)
    }

    /// ``((?<q>`?)[_\p{L}][_\p{L}\p{N}\p{M}]*(\k<q>))\s*(:)``, for
    /// `ExpressionsWithoutTrailingClosuresOrMemberReferencesExpressionElementListAt1`.
    pub(in crate::swift) fn rx_expressions_without_trailing_closures_or_member_references_expression_element_list_at_1(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        let id = self.ident_at(p)?;
        let colon = self.spaces(id.end);
        if self.at(colon) != b':' {
            return None;
        }
        self.cap(1, p, id.end);
        self.cap_expr_quotes(p, id, 2, 3);
        self.cap(4, colon, colon + 1);
        Some(colon + 1)
    }

    /// ``(?<=[])>_`}\p{L}\p{N}\p{M}])\s*(\()``, for
    /// `ExpressionsWithoutTrailingClosuresOrMemberReferencesFunctionOrMacroCallExpressionAt1`.
    pub(in crate::swift) fn rx_expressions_without_trailing_closures_or_member_references_function_or_macro_call_expression_at_1(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        let c = self.char_before(p)?;
        if !(matches!(c, ']' | ')' | '>' | '_' | '`' | '}')
            || is_letter(c)
            || is_number(c)
            || is_mark(c))
        {
            return None;
        }
        let open = self.spaces(p);
        if self.at(open) != b'(' {
            return None;
        }
        self.cap(1, open, open + 1);
        Some(open + 1)
    }

    /// ``(#(?<q>`?)[_\p{L}][_\p{L}\p{N}\p{M}]*(\k<q>))``, for
    /// `ExpressionsWithoutTrailingClosuresOrMemberReferencesMacroExpansion`.
    pub(in crate::swift) fn rx_expressions_without_trailing_closures_or_member_references_macro_expansion(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        if self.at(p) != b'#' {
            return None;
        }
        self.ident_at(p + 1).map(|id| id.end)
    }

    /// ``(?<=[_`\p{L}\p{N}\p{M}])\s*(\[)``, for
    /// `ExpressionsWithoutTrailingClosuresOrMemberReferencesSubscriptExpression`.
    pub(in crate::swift) fn rx_expressions_without_trailing_closures_or_member_references_subscript_expression(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        let c = self.char_before(p)?;
        if !(matches!(c, '_' | '`') || is_letter(c) || is_number(c) || is_mark(c)) {
            return None;
        }
        let open = self.spaces(p);
        if self.at(open) != b'[' {
            return None;
        }
        self.cap(1, open, open + 1);
        Some(open + 1)
    }

    /// `(\))\s*((?:\b(?:async|throws|rethrows)\s)*)`, for
    /// `ExpressionsWithoutTrailingClosuresOrMemberReferencesParenthesizedExpressionEnd`.
    pub(in crate::swift) fn rx_expressions_without_trailing_closures_or_member_references_parenthesized_expression_end(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        if self.at(p) != b')' {
            return None;
        }
        let mut end = self.spaces(p + 1);
        let from = end;
        while self.boundary(end) {
            let Some(word) = [b"async".as_slice(), b"throws", b"rethrows"]
                .iter()
                .find_map(|w| self.lit(end, w))
            else {
                break;
            };
            let width = self.space_at(word);
            if width == 0 {
                break;
            }
            end = word + width;
        }
        self.cap(1, p, p + 1);
        self.cap(2, from, end);
        Some(end)
    }

    /// ``(#?(?<q>`?)[_\p{L}][_\p{L}\p{N}\p{M}]*(\k<q>))(?=\s*\{)``, for
    /// `ExpressionsTrailingClosureAt0`.
    pub(in crate::swift) fn rx_expressions_trailing_closure_at_0(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        let (id, s) = self.hashed_ident(p)?;
        let brace = self.spaces(id.end);
        if self.at(brace) != b'{' {
            return None;
        }
        self.cap(1, p, id.end);
        self.cap_expr_quotes(s, id, 2, 3);
        Some(id.end)
    }

    /// ``((?<q>`?)[_\p{L}][_\p{L}\p{N}\p{M}]*(\k<q>))\s*(:)(?=\s*\{)``, for
    /// `ExpressionsTrailingClosureAt1`.
    pub(in crate::swift) fn rx_expressions_trailing_closure_at_1(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        let id = self.ident_at(p)?;
        let colon = self.spaces(id.end);
        if self.at(colon) != b':' {
            return None;
        }
        let brace = self.spaces(colon + 1);
        if self.at(brace) != b'{' {
            return None;
        }
        self.cap(1, p, id.end);
        self.cap_expr_quotes(p, id, 2, 3);
        self.cap(4, colon, colon + 1);
        Some(colon + 1)
    }

    /// ``(?<=\.)((?<q>`?)[_\p{L}][_\p{L}\p{N}\p{M}]*(\k<q>))``, for
    /// `MemberReferenceAt0`.
    pub(in crate::swift) fn rx_member_reference_at_0(&mut self, p: usize) -> Option<usize> {
        if !self.prev_is(p, b'.') {
            return None;
        }
        let id = self.ident_at(p)?;
        self.cap(1, p, id.end);
        self.cap_expr_quotes(p, id, 2, 3);
        Some(id.end)
    }

    /// `(?:(\*)|\b(deprecated|unavailable|noasync)\b)\s*(.*?)(?=[),])`, for
    /// `AttributesAt0At3`.
    pub(in crate::swift) fn rx_attributes_at_0_at_3(&mut self, p: usize) -> Option<usize> {
        let after = if self.at(p) == b'*' {
            p + 1
        } else {
            self.word_in(p, &[b"deprecated", b"unavailable", b"noasync"])?
        };
        let e = self.spaces(after);
        let q = self.close_from(e)?;
        if self.at(p) == b'*' {
            self.cap(1, p, p + 1);
        } else {
            self.cap(2, p, after);
        }
        self.cap(3, e, q);
        Some(q)
    }
}
