//! The per-line lexer: cursor, token emission and the context stack.

use memchr::{memchr2, memmem};

use super::Dialect;
use super::scope::S;
use super::state::Kind;
use crate::classes::{digit_width, is_ascii_space, space_width, word_before, word_width};
use crate::emit::Emitter;
use crate::language::{Context, Halt};
use crate::limits::{Finished, Limit, Steps};
use crate::utf8::char_at;

/// Lexes one line.
/// `pos` is the end of the last token.
/// The bytes between it and the cursor are plain text of the current context.
/// Its mode `H` says whether it stops where the halts of a language that
/// embeds JSON hold (see [`super::lex_until`]).
/// [`NoHalts`] has no size, and its tests compile out.
pub(crate) struct Lexer<'a, H> {
    s: &'a [u8],
    /// The tokens, the contexts and the end of the last token.
    pub(super) em: Emitter<'a>,
    dialect: Dialect,
    /// The work this line has spent and may spend.
    pub(super) steps: Steps,
    /// Whether the lexer halts, and where it did.
    mode: H,
}

/// Whether a lexer stops where the halts of a language that embeds JSON
/// hold.
pub(crate) trait Mode<'a> {
    /// Whether the lexer tests halts at the top of the document.
    const HALTS: bool;

    /// The halts, for a lexer that tests them.
    fn halts(&self) -> Option<Halts<'a>>;

    /// Where the line halted, from the start of the lexed text.
    fn halted(&self) -> Option<usize>;

    /// Records that the line halts at `p`.
    fn halt(&mut self, p: usize);
}

/// The mode of a lexer for a JSON document, which never halts.
pub(crate) struct NoHalts;

impl<'a> Mode<'a> for NoHalts {
    const HALTS: bool = false;

    fn halts(&self) -> Option<Halts<'a>> {
        None
    }

    fn halted(&self) -> Option<usize> {
        None
    }

    fn halt(&mut self, _: usize) {}
}

/// The mode of a lexer for JSON that another language embeds.
pub(crate) struct Embedded<'a> {
    /// The halts of the language that embeds JSON.
    halts: Halts<'a>,
    /// Where the line halted.
    halted: Option<usize>,
}

impl<'a> Embedded<'a> {
    /// The mode that stops where `halts` hold.
    pub(crate) fn new(halts: Halts<'a>) -> Self {
        Embedded {
            halts,
            halted: None,
        }
    }
}

impl<'a> Mode<'a> for Embedded<'a> {
    const HALTS: bool = true;

    fn halts(&self) -> Option<Halts<'a>> {
        Some(self.halts)
    }

    fn halted(&self) -> Option<usize> {
        self.halted
    }

    fn halt(&mut self, p: usize) {
        self.halted = Some(p);
    }
}

/// The halts of a language that embeds JSON.
#[derive(Clone, Copy)]
pub(crate) struct Halts<'a> {
    /// The whole line, in which the halts read their places.
    pub(crate) line: &'a [u8],
    /// Where the lexed text starts in the line.
    pub(crate) start: usize,
    /// The halt of the embedding rule's end and of the patterns before JSON.
    pub(crate) end: &'a dyn Halt,
    /// The halt of the patterns after JSON.
    pub(crate) after: Option<&'a dyn Halt>,
}

impl<'a, H: Mode<'a>> Lexer<'a, H> {
    pub(crate) fn new(
        cx: &'a mut Context<'_>,
        dialect: Dialect,
        s: &'a [u8],
        state: u32,
        mode: H,
    ) -> Self {
        let steps = Steps::new(cx.limits, s.len());
        Lexer {
            s,
            em: Emitter::new(cx, state, 0),
            dialect,
            steps,
            mode,
        }
    }

    /// Where the line halted, from the start of the lexed text.
    pub(crate) fn halted(&self) -> Option<usize> {
        self.mode.halted()
    }

    /// The state after the line, with its count given back to the lender.
    pub(crate) fn finish_line(self) -> Finished {
        self.steps.finish(self.em.state)
    }

    pub(crate) fn run(&mut self) {
        let len = self.s.len();
        let mut p = 0;
        while p < len {
            self.steps.spend(1);
            if self.steps.exhausted() {
                self.em.report.record(Limit::Work);
                break;
            }
            let from = p;
            p = match self.top() {
                Kind::Str | Kind::Key => self.in_string(p),
                Kind::Block | Kind::Doc => self.in_comment(p),
                Kind::Top if H::HALTS => match self.at_top(p) {
                    Some(next) => next,
                    None => break,
                },
                Kind::Top | Kind::Array | Kind::Object | Kind::Pair => self.in_code(p),
            };
            debug_assert!(p > from, "the JSON lexer made no progress at byte {from}");
        }
        self.gap(self.mode.halted().unwrap_or(len));
    }

    /// A step at the top of the document for a lexer that halts.
    ///
    /// Only there do the embedding language's patterns stand beside JSON's.
    /// Inside an array, an object, a string or a comment, JSON's rule alone
    /// is open, as in Shiki.
    /// - `end` is tested first, at the step's place and at each of its blanks.
    /// - `after` is tested where no JSON token starts: at a blank, and at a
    ///   character no value takes.
    ///
    /// It returns where the next step begins, or `None` where the line halts.
    fn at_top(&mut self, p: usize) -> Option<usize> {
        let halts = self.mode.halts()?;
        let at = halts.start + p;
        if halts.end.holds(halts.line, at) {
            self.mode.halt(p);
            return None;
        }
        let b = self.s[p];
        let blank = if b < 0x80 {
            usize::from(is_ascii_space(b))
        } else {
            space_width(self.s, p)
        };
        if blank > 0 {
            let next = self.skip_space(p);
            let last = halts.start + next - 1;
            let end = (next > p + blank)
                .then(|| halts.end.first(halts.line, at + blank, last))
                .flatten();
            let after = halts
                .after
                .and_then(|after| after.first(halts.line, at, last));
            let halt = match (end, after) {
                (Some(end), Some(after)) => Some(end.min(after)),
                (end, after) => end.or(after),
            };
            if let Some(halt) = halt {
                self.mode.halt(halt - halts.start);
                return None;
            }
            return Some(next);
        }
        if let Some(end) = self.value(p, b) {
            return Some(end);
        }
        if halts.after.is_some_and(|after| after.holds(halts.line, at)) {
            self.mode.halt(p);
            return None;
        }
        Some(p + self.width_at(p))
    }

    // Bytes.

    /// The byte at `i`, or 0 past the end.
    fn at(&self, i: usize) -> u8 {
        self.s.get(i).copied().unwrap_or(0)
    }

    /// The width of the character at `i`, at least 1.
    fn width_at(&self, i: usize) -> usize {
        char_at(self.s, i).map_or(1, |(_, width)| width)
    }

    // Emission.

    /// Emits plain text of the current context up to `to`.
    fn gap(&mut self, to: usize) {
        self.tok(to, &[]);
    }

    /// Emits the bytes up to `to` with the current context plus `leaf`.
    fn tok(&mut self, to: usize, leaf: &[S]) {
        self.em
            .tok(to, leaf.iter().map(|scope| scope.scope(self.dialect)));
    }

    // Contexts.

    fn top(&self) -> Kind {
        Kind::from_code(self.em.frame.rule)
    }

    /// Opens a context.
    /// It returns false when a limit keeps the context closed.
    fn push(&mut self, kind: Kind) -> bool {
        let dialect = self.dialect;
        let scopes = kind.scopes().iter().map(|scope| scope.scope(dialect));
        self.em.push(kind.code(), 0, scopes)
    }

    /// Opens `kind` at `p`.
    /// Its begin text ends at `end` and carries `leaf`.
    /// The text stays plain when a limit keeps the context closed.
    fn begin(&mut self, p: usize, end: usize, kind: Kind, leaf: S) -> usize {
        self.gap(p);
        if self.push(kind) {
            self.tok(end, &[leaf]);
        }
        end
    }

    /// Emits the end text up to `end` with `leaf` and closes the context.
    fn finish(&mut self, p: usize, end: usize, leaf: S) -> usize {
        self.gap(p);
        self.tok(end, &[leaf]);
        self.em.pop();
        end
    }

    // Code.

    /// Takes the next match at or after `p` in an array, object, value or
    /// document context.
    ///
    /// Strings and comments take their text in `run`, never here.
    fn in_code(&mut self, p: usize) -> usize {
        let b = self.s[p];
        if b < 0x80 {
            if is_ascii_space(b) {
                return self.skip_space(p);
            }
        } else {
            let width = space_width(self.s, p);
            if width > 0 {
                return self.skip_space(p);
            }
        }
        match self.top() {
            Kind::Top => self.value(p, b).unwrap_or(p + self.width_at(p)),
            Kind::Array => self.in_array(p, b),
            Kind::Object => self.in_object(p, b),
            Kind::Pair | Kind::Key | Kind::Str | Kind::Block | Kind::Doc => self.in_pair(p, b),
        }
    }

    fn skip_space(&mut self, mut p: usize) -> usize {
        let from = p;
        while p < self.s.len() {
            let width = if self.s[p] < 0x80 {
                usize::from(is_ascii_space(self.s[p]))
            } else {
                space_width(self.s, p)
            };
            if width == 0 {
                break;
            }
            p += width;
        }
        self.steps.spend(p - from);
        p
    }

    /// An invalid character `[^…\s]`: one whole character.
    fn invalid(&mut self, p: usize, scope: S) -> usize {
        let end = p + self.width_at(p);
        self.gap(p);
        self.tok(end, &[scope]);
        end
    }

    fn in_array(&mut self, p: usize, b: u8) -> usize {
        if b == b']' {
            return self.finish(p, p + 1, S::PunctArrayEnd);
        }
        if let Some(end) = self.value(p, b) {
            return end;
        }
        if b == b',' {
            self.gap(p);
            self.tok(p + 1, &[S::PunctArraySep]);
            return p + 1;
        }
        self.invalid(p, S::InvalidArraySep)
    }

    fn in_object(&mut self, p: usize, b: u8) -> usize {
        match b {
            b'}' => self.finish(p, p + 1, S::PunctDictEnd),
            b'"' => self.begin(p, p + 1, Kind::Key, S::PunctKeyBegin),
            b'/' => self
                .comment(p)
                .unwrap_or_else(|| self.invalid(p, S::InvalidDictSep)),
            b':' => self.begin(p, p + 1, Kind::Pair, S::PunctKeyValue),
            _ => self.invalid(p, S::InvalidDictSep),
        }
    }

    /// In the value after a colon, `,` and the `}` that follows end it.
    fn in_pair(&mut self, p: usize, b: u8) -> usize {
        match b {
            b',' => self.finish(p, p + 1, S::PunctPair),
            b'}' => {
                self.gap(p);
                self.em.pop();
                self.in_code(p)
            }
            _ => self
                .value(p, b)
                .unwrap_or_else(|| self.invalid(p, S::InvalidDictSep)),
        }
    }

    /// The `value` rules at `p`, where the byte is `b`: a constant, number,
    /// string, array, object or comment.
    fn value(&mut self, p: usize, b: u8) -> Option<usize> {
        match b {
            b'"' => Some(self.begin(p, p + 1, Kind::Str, S::PunctStrBegin)),
            b'[' => Some(self.begin(p, p + 1, Kind::Array, S::PunctArrayBegin)),
            b'{' => Some(self.begin(p, p + 1, Kind::Object, S::PunctDictBegin)),
            b'/' => self.comment(p),
            b'-' | b'0'..=b'9' => self.number(p),
            b't' | b'f' | b'n' => self.constant(p),
            _ => None,
        }
    }

    /// `-?(?:0|[1-9]\d*)(?:(?:\.\d+)?(?:[Ee][-+]?\d+)?)?`.
    fn number(&mut self, p: usize) -> Option<usize> {
        let mut i = p + usize::from(self.at(p) == b'-');
        match self.at(i) {
            b'0' => i += 1,
            b'1'..=b'9' => i = self.digits(i),
            _ => return None,
        }
        if self.at(i) == b'.' {
            let end = self.digits(i + 1);
            if end > i + 1 {
                i = end;
            }
        }
        if matches!(self.at(i), b'E' | b'e') {
            let from = i + 1 + usize::from(matches!(self.at(i + 1), b'-' | b'+'));
            let end = self.digits(from);
            if end > from {
                i = end;
            }
        }
        self.steps.spend(i - p);
        self.gap(p);
        self.tok(i, &[S::Number]);
        Some(i)
    }

    /// The end of the run of `\d` characters from `i`.
    fn digits(&self, mut i: usize) -> usize {
        while let width @ 1.. = digit_width(self.s, i) {
            i += width;
        }
        i
    }

    /// `\b(?:true|false|null)\b`.
    fn constant(&mut self, p: usize) -> Option<usize> {
        let word: &[u8] = match self.at(p) {
            b't' => b"true",
            b'f' => b"false",
            _ => b"null",
        };
        let end = p + word.len();
        if self.s.get(p..end) != Some(word) || word_before(self.s, p) || word_width(self.s, end) > 0
        {
            return None;
        }
        self.gap(p);
        self.tok(end, &[S::Constant]);
        Some(end)
    }

    /// `/**`, `/*` and `//` comments at `p`, where the byte is `/`.
    fn comment(&mut self, p: usize) -> Option<usize> {
        match (self.at(p + 1), self.at(p + 2)) {
            (b'*', b'*') if self.at(p + 3) != b'/' => {
                Some(self.begin(p, p + 3, Kind::Doc, S::PunctComment))
            }
            (b'*', _) => Some(self.begin(p, p + 2, Kind::Block, S::PunctComment)),
            (b'/', _) => {
                let len = self.s.len();
                self.gap(p);
                self.tok(p + 2, &[S::CommentLine, S::PunctComment]);
                self.tok(len, &[S::CommentLine]);
                Some(len)
            }
            _ => None,
        }
    }

    // Strings and comments.

    /// Takes the text of a string up to its next escape or closing quote.
    ///
    /// `\\(?:["/\\bfnrt]|u\h{4})` is an escape.
    /// `\\.` is any other escape.
    /// A backslash at the end of the line is no escape.
    fn in_string(&mut self, p: usize) -> usize {
        let Some(offset) = memchr2(b'"', b'\\', &self.s[p..]) else {
            self.steps.spend(self.s.len() - p);
            return self.s.len();
        };
        let q = p + offset;
        self.steps.spend(offset);
        if self.s[q] == b'"' {
            let leaf = if self.top() == Kind::Key {
                S::PunctKeyEnd
            } else {
                S::PunctStrEnd
            };
            return self.finish(q, q + 1, leaf);
        }
        let escape = match self.at(q + 1) {
            b'"' | b'/' | b'\\' | b'b' | b'f' | b'n' | b'r' | b't' => Some((2, S::Escape)),
            b'u' if (2..6).all(|k| self.at(q + k).is_ascii_hexdigit()) => Some((6, S::Escape)),
            _ if q + 1 < self.s.len() => Some((1 + self.width_at(q + 1), S::InvalidEscape)),
            _ => None,
        };
        let Some((len, scope)) = escape else {
            return self.s.len();
        };
        self.gap(q);
        self.tok(q + len, &[scope]);
        q + len
    }

    /// Takes the text of a block comment up to its `*/`.
    fn in_comment(&mut self, p: usize) -> usize {
        let Some(offset) = memmem::find(&self.s[p..], b"*/") else {
            self.steps.spend(self.s.len() - p);
            return self.s.len();
        };
        self.steps.spend(offset);
        self.finish(p + offset, p + offset + 2, S::PunctComment)
    }
}
