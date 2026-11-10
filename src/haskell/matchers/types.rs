//! Type signatures and their parts: units, tuples, lists, `forall`, type
//! variables, constructors and operators, type applications, and the lines
//! and places a signature starts and ends.

use super::super::engine::Lexer;
use super::{is_name, is_op, is_var_start};
use crate::classes::{is_symbol_or_punctuation, is_title_or_upper};
use crate::rules::Chars;

/// The arrows that end a signature or a function's declaration, in the
/// grammar's order: `<-|←`, `=`, `-<|↢`, `-<<|⤛`.
const ARROWS: [&str; 7] = ["<-", "←", "=", "-<", "↢", "-<<", "⤛"];

/// ``[[\p{S}\p{P}]&&[^"'(,;\[_`{]]``: a symbol or punctuation mark next to
/// which a signature's colons do not start one.
fn is_open_op(c: char) -> bool {
    is_symbol_or_punctuation(c)
        && !matches!(c, '"' | '\'' | '(' | ',' | ';' | '[' | '_' | '`' | '{')
}

/// ``[[\p{S}\p{P}]&&[^]"'),;_`}]]``: a symbol or punctuation mark that must not
/// stand before a function's colons.
fn is_close_op(c: char) -> bool {
    is_symbol_or_punctuation(c)
        && !matches!(c, ']' | '"' | '\'' | ')' | ',' | ';' | '_' | '`' | '}')
}

/// ``[[\p{S}\p{P}]&&[^]"'(),:;\[_`{}]]``: an operator character other than
/// `:`, the first of a function named by an operator.
fn is_op_not_colon(c: char) -> bool {
    c != ':' && is_op(c)
}

impl Lexer<'_> {
    /// `(')?` at `p` when `promoted`: where what follows the quote starts.
    fn after_quote(&self, p: usize, promoted: bool) -> (Option<usize>, usize) {
        if promoted && p < self.lim() && self.at(p) == b'\'' {
            (Some(p), p + 1)
        } else {
            (None, p)
        }
    }

    /// `(')?(\()\s*(\))`, or `(\()\s*(\))` when not `promoted`: the unit,
    /// with groups around the quote and each parenthesis.
    ///
    /// A quote that leads to no unit leaves `(` at a quote, where it fails.
    pub(super) fn unit(&mut self, p: usize, promoted: bool) -> Option<usize> {
        self.bracketed(p, promoted, b'(', b')', false)
    }

    /// `(')?(\()\s*,[,\s]*(\))`, or without the quote when not `promoted`:
    /// a tuple constructor, with groups around the quote and each
    /// parenthesis.
    pub(super) fn tuple(&mut self, p: usize, promoted: bool) -> Option<usize> {
        self.bracketed(p, promoted, b'(', b')', true)
    }

    /// `(')?(\[)\s*(])`, or without the quote when not `promoted`: the empty
    /// list, with groups around the quote and each bracket.
    pub(super) fn empty_list(&mut self, p: usize, promoted: bool) -> Option<usize> {
        self.bracketed(p, promoted, b'[', b']', false)
    }

    /// `(')?(x)\s*(y)`, with `,[,\s]*` before `y` for `commas`: the groups of
    /// [`Lexer::unit`], [`Lexer::tuple`] and [`Lexer::empty_list`].
    fn bracketed(
        &mut self,
        p: usize,
        promoted: bool,
        open: u8,
        close: u8,
        commas: bool,
    ) -> Option<usize> {
        let (quote, at) = self.after_quote(p, promoted);
        if !(at < self.lim() && self.at(at) == open) {
            return None;
        }
        let mut end = self.space_end(at + 1);
        if commas {
            if self.at(end) != b',' {
                return None;
            }
            end = self.skip(end, |b| b == b',' || b == b' ' || b == b'\t');
            end = self.commas_end(end);
        }
        if !(end < self.lim() && self.at(end) == close) {
            return None;
        }
        let first = usize::from(promoted);
        if let Some(quote) = quote {
            self.capture(1, quote, quote + 1);
        }
        self.capture(first + 1, at, at + 1);
        self.capture(first + 2, end, end + 1);
        Some(end + 1)
    }

    /// The end of `[,\s]*` from `i`.
    fn commas_end(&self, i: usize) -> usize {
        let mut at = i;
        loop {
            let width = if self.at(at) == b',' && at < self.lim() {
                1
            } else {
                self.space_at(at)
            };
            if width == 0 {
                return at;
            }
            at += width;
        }
    }

    /// `(\()(#)\s*(#)(\))`: the unboxed unit, with a group around each
    /// character.
    pub(super) fn unboxed_unit(&mut self, p: usize) -> Option<usize> {
        let hash = self.lit(p, b"(#")?;
        let close = self.space_end(hash);
        let end = self.lit(close, b"#)")?;
        self.capture(1, p, p + 1);
        self.capture(2, p + 1, hash);
        self.capture(3, close, close + 1);
        self.capture(4, close + 1, end);
        Some(end)
    }

    /// `(\()(#)\s*,[,\s]*(#)(\))`: an unboxed tuple constructor, with a group
    /// around each parenthesis and hash.
    pub(super) fn unboxed_tuple(&mut self, p: usize) -> Option<usize> {
        let hash = self.lit(p, b"(#")?;
        let comma = self.space_end(hash);
        let commas = self.lit(comma, b",")?;
        let close = self.commas_end(commas);
        let end = self.lit(close, b"#)")?;
        self.capture(1, p, p + 1);
        self.capture(2, p + 1, hash);
        self.capture(3, close, close + 1);
        self.capture(4, close + 1, end);
        Some(end)
    }

    /// `(::|∷)(?![OP])`.
    pub(super) fn double_colon(&self, p: usize) -> Option<usize> {
        self.alone_after(p, &[b"::", "∷".as_bytes()])
    }

    /// `\b(?<!')(forall|∀)\b(?!')`, with group 1 around the word.
    ///
    /// `∀` is no word character, so `\b` holds around it only between word
    /// characters.
    pub(super) fn forall(&mut self, p: usize) -> Option<usize> {
        if !self.boundary(p) || self.prev_is(p, b'\'') {
            return None;
        }
        let end = self
            .lit(p, b"forall")
            .or_else(|| self.lit(p, "∀".as_bytes()))?;
        if !self.boundary(end) || self.at(end) == b'\'' {
            return None;
        }
        self.capture(1, p, end);
        Some(end)
    }

    /// `(\.)|(->|→)`, with a group around the dot or the arrow.
    pub(super) fn forall_end(&mut self, p: usize) -> Option<usize> {
        if let Some(end) = self.lit(p, b".") {
            self.capture(1, p, end);
            return Some(end);
        }
        let end = self.lit(p, b"->").or_else(|| self.lit(p, "→".as_bytes()))?;
        self.capture(2, p, end);
        Some(end)
    }

    /// `\b(?<!')(?!(?:forall|deriving)\b(?!'))[_\p{Ll}]['_\p{Ll}\p{Lu}\p{Lt}\
    /// d]*`: a type variable.
    pub(super) fn type_variable(&self, p: usize) -> Option<usize> {
        if self.word_before(p) || self.prev_is(p, b'\'') {
            return None;
        }
        let keyword = [&b"forall"[..], b"deriving"].iter().any(|word| {
            self.lit(p, word)
                .is_some_and(|end| self.boundary(end) && self.at(end) != b'\'')
        });
        if keyword {
            return None;
        }
        self.name_from(p, is_var_start)
    }

    /// `'[^']'`: a promoted character.
    pub(super) fn promoted_char(&self, p: usize) -> Option<usize> {
        let at = self.lit(p, b"'")?;
        let (c, end) = self.char_here(at)?;
        if c == '\'' {
            return None;
        }
        self.lit(end, b"'")
    }

    /// Whether `(?<=[]",;\[{}\s])` holds at `p`: what may stand before a type
    /// application's `@`.
    fn application_before(&self, p: usize) -> bool {
        if p == 0 {
            return false;
        }
        let start = self.char_start_before(p);
        matches!(
            self.at(start),
            b']' | b'"' | b',' | b';' | b'[' | b'{' | b'}'
        ) || self.space_at(start) > 0
    }

    /// `(?<=[]",;\[{}\s])(@)(')?(x)` for the bracket `x`, `open`: a type
    /// application, with groups around the `@`, the quote and the bracket.
    pub(super) fn type_application(&mut self, p: usize, open: u8) -> Option<usize> {
        if !self.application_before(p) {
            return None;
        }
        let at = self.lit(p, b"@")?;
        let (quote, bracket) = self.after_quote(at, true);
        if !(bracket < self.lim() && self.at(bracket) == open) {
            return None;
        }
        self.capture(1, p, at);
        if let Some(quote) = quote {
            self.capture(2, quote, quote + 1);
        }
        self.capture(3, bracket, bracket + 1);
        Some(bracket + 1)
    }

    /// `(?<=[]",;\[{}\s])(@)(?=")`, with group 1 around the `@`.
    pub(super) fn type_application_string(&mut self, p: usize) -> Option<usize> {
        if !self.application_before(p) || !self.starts(p, b"@\"") {
            return None;
        }
        self.capture(1, p, p + 1);
        Some(p + 1)
    }

    /// `(?<=[]",;\[{}\s])(@)(?=['_\p{Ll}\p{Lu}\p{Lt}\d])`, with group 1 around
    /// the `@`.
    pub(super) fn type_application_name(&mut self, p: usize) -> Option<usize> {
        if !self.application_before(p) || self.at(p) != b'@' || !self.here_is(p + 1, is_name) {
            return None;
        }
        self.capture(1, p, p + 1);
        Some(p + 1)
    }

    /// The end of `(?:\b?[\p{Lu}\p{Lt}]['_\p{Ll}\p{Lu}\p{Lt}\d]*\.)*` from
    /// `i`, with `\b` before each name when `bounded`, and where its last
    /// name starts.
    ///
    /// A name that no dot follows is not part of it.
    fn qualifiers(&self, i: usize, bounded: bool) -> (usize, Option<usize>) {
        let mut end = i;
        let mut last = None;
        loop {
            if bounded && self.word_before(end) {
                return (end, last);
            }
            match self.name_from(end, is_title_or_upper) {
                Some(name) if self.at(name) == b'.' && name < self.lim() => {
                    last = Some(end);
                    end = name + 1;
                }
                _ => return (end, last),
            }
        }
    }

    /// The ends of `#+|[OP]+(?<!#)` at `i`, in the order backtracking tries
    /// them.
    /// The first is the run of hashes when one starts there.
    /// The second is the operator characters up to the last that is no hash.
    pub(super) fn operator_ends(&self, i: usize) -> [Option<usize>; 2] {
        let hashes = (self.at(i) == b'#' && i < self.lim()).then(|| self.skip(i, |b| b == b'#'));
        let mut end = self.class_end(i, is_op);
        while end > i && self.prev_is(end, b'#') {
            end -= 1;
        }
        [hashes, (end > i).then_some(end)]
    }

    /// `(?:(?<!')('))?((?:\b[\p{Lu}\p{Lt}]…\.)*)(?![#@]?-})(#+|[OP]+(?<!#))`:
    /// a type operator, with groups around the quote, the qualifiers and the
    /// operator.
    ///
    /// Giving back a qualifier leaves the operator at a letter, where it
    /// fails.
    pub(super) fn type_operator(&mut self, p: usize) -> Option<usize> {
        let quoted = !self.prev_is(p, b'\'') && self.at(p) == b'\'' && p < self.lim();
        let start = p + usize::from(quoted);
        let (op, _) = self.qualifiers(start, true);
        if self.starts(op, b"-}") || self.starts(op, b"#-}") || self.starts(op, b"@-}") {
            return None;
        }
        let end = self.operator_ends(op).into_iter().flatten().next()?;
        if quoted {
            self.capture(1, p, start);
        }
        self.capture(2, start, op);
        self.capture(3, op, end);
        Some(end)
    }

    /// ``(')?(`)((?:[\p{Lu}\p{Lt}]…\.)*)([\p{Lu}\p{Lt}]…)(`)``: a type
    /// operator in backticks, with groups around each part.
    ///
    /// Giving back a qualifier leaves its dot where the backtick must stand.
    pub(super) fn type_operator_backticks(&mut self, p: usize) -> Option<usize> {
        let (quote, tick) = self.after_quote(p, true);
        let qualifiers = self.lit(tick, b"`")?;
        let (name, _) = self.qualifiers(qualifiers, false);
        let close = self.name_from(name, is_title_or_upper)?;
        let end = self.lit(close, b"`")?;
        if let Some(quote) = quote {
            self.capture(1, quote, tick);
        }
        self.capture(2, tick, qualifiers);
        self.capture(3, qualifiers, name);
        self.capture(4, name, close);
        self.capture(5, close, end);
        Some(end)
    }

    /// `(')?((?:\b[\p{Lu}\p{Lt}]…\.)*)\b([\p{Lu}\p{Lt}]…)`: a type
    /// constructor, with groups around the quote, the qualifiers and the
    /// name.
    ///
    /// When no name follows the last dot, the last qualifier is given back to
    /// be the name.
    pub(super) fn type_constructor(&mut self, p: usize) -> Option<usize> {
        let (quote, start) = self.after_quote(p, true);
        let (after, last) = self.qualifiers(start, true);
        let next = (!self.word_before(after))
            .then(|| self.name_from(after, is_title_or_upper))
            .flatten();
        let (name, end) = if let Some(end) = next {
            (after, end)
        } else {
            let name = last?;
            (name, self.name_from(name, is_title_or_upper)?)
        };
        if let Some(quote) = quote {
            self.capture(1, quote, start);
        }
        self.capture(2, start, name);
        self.capture(3, name, end);
        Some(end)
    }

    /// `(')?(\()\s*((?:[\p{Lu}\p{Lt}]…\.)*)([OP]+)\s*(\))`: a type
    /// constructor that is an operator, with groups around each part.
    ///
    /// Giving back a qualifier leaves the operator at a letter.
    pub(super) fn type_constructor_operator(&mut self, p: usize) -> Option<usize> {
        let (quote, open) = self.after_quote(p, true);
        let mut qualifiers = 0;
        let found = self.paren_op(open, |l, at| {
            let (op, _) = l.qualifiers(at, false);
            l.ops(op, false)
        })?;
        let (start, end, close) = found;
        let (op, _) = self.qualifiers(start, false);
        qualifiers += op - start;
        if let Some(quote) = quote {
            self.capture(1, quote, open);
        }
        self.capture(2, open, open + 1);
        self.capture(3, start, start + qualifiers);
        self.capture(4, op, end);
        self.capture(5, close, close + 1);
        Some(close + 1)
    }

    /// `(\()(#)`, with groups around both characters.
    pub(super) fn hash_paren(&mut self, p: usize) -> Option<usize> {
        let end = self.lit(p, b"(#")?;
        self.capture(1, p, p + 1);
        self.capture(2, p + 1, end);
        Some(end)
    }

    /// `(#)(\))`, with groups around both characters.
    pub(super) fn hash_close(&mut self, p: usize) -> Option<usize> {
        let end = self.lit(p, b"#)")?;
        self.capture(1, p, p + 1);
        self.capture(2, p + 1, end);
        Some(end)
    }

    /// `(')?(x)` for the bracket `x`, `open`, with groups around the quote and
    /// the bracket.
    pub(super) fn promoted_open(&mut self, p: usize, open: u8) -> Option<usize> {
        let (quote, at) = self.after_quote(p, true);
        if !(at < self.lim() && self.at(at) == open) {
            return None;
        }
        if let Some(quote) = quote {
            self.capture(1, quote, at);
        }
        self.capture(2, at, at + 1);
        Some(at + 1)
    }

    /// `\s*(::|∷)(?![OP])\s*`, with group 1 around the colons.
    pub(super) fn spaced_double_colon(&mut self, p: usize) -> Option<usize> {
        let colons = self.space_end(p);
        let end = self.double_colon(colons)?;
        self.capture(1, colons, end);
        Some(self.space_end(end))
    }

    /// The end of the `fn` group at `i`:
    /// `[_\p{Ll}]['_\p{Ll}\p{Lu}\p{Lt}\d]*#*` or
    /// `\(\s*(?!--+\))[OP but :][OP]*\s*\)`.
    fn function_name(&self, i: usize) -> Option<usize> {
        if let Some(end) = self.name_from(i, is_var_start) {
            return Some(self.skip(end, |b| b == b'#'));
        }
        let (_, _, close) = self.paren_op(i, |l, at| {
            if l.dashes_close(at) || !l.here_is(at, is_op_not_colon) {
                return None;
            }
            l.ops(at, false)
        })?;
        Some(close + 1)
    }

    /// `^(\s*)(?<fn>NAME(?:\s*,\s*\g<fn>)?)\s*(?<![…])(::|∷)(?![…])`: the
    /// start of a function's type signature, with groups around the
    /// indentation, the names and the colons.
    ///
    /// The names run on while commas join them.
    /// Giving one back leaves a comma where the colons must stand.
    pub(super) fn fun_decl(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let indent = self.space_end(0);
        let mut names = self.function_name(indent)?;
        loop {
            let comma = self.space_end(names);
            let Some(next) = self
                .lit(comma, b",")
                .and_then(|after| self.function_name(self.space_end(after)))
            else {
                break;
            };
            names = next;
        }
        let colons = self.space_end(names);
        if self.before_is(colons, is_close_op) {
            return None;
        }
        let end = self
            .lit(colons, b"::")
            .or_else(|| self.lit(colons, "∷".as_bytes()))?;
        if self.here_is(end, is_open_op) {
            return None;
        }
        self.capture(1, 0, indent);
        self.capture(2, indent, names);
        self.capture(3, colons, end);
        Some(end)
    }

    /// Whether the character at `i` is no operator character, or is the line
    /// feed: ``[]"'(),;\[_`{}[^\p{S}\p{P}]]``.
    fn no_op_at(&self, i: usize) -> bool {
        match self.char_here(i) {
            Some((c, _)) => !is_op(c),
            None => self.at(i) == b'\n',
        }
    }

    /// `(?<![OP])(?:x)(?=[^OP])` for one of `arrows`: an arrow that stands
    /// alone at `p`.
    fn arrow_at(&self, p: usize, arrows: &[&str]) -> bool {
        !self.before_is(p, is_op)
            && arrows.iter().any(|arrow| {
                self.lit(p, arrow.as_bytes())
                    .is_some_and(|end| self.no_op_at(end))
            })
    }

    /// The end of a function's declaration: an arrow ahead,
    /// `(?=(?<![OP])(<-|←|=|-<|↢|-<<|⤛)[^OP])`, or the end of a declaration.
    pub(super) fn fun_decl_end(&self, p: usize) -> Option<usize> {
        if self.arrow_at(p, &ARROWS) {
            return Some(p);
        }
        self.declaration_end(p, None, b"'")
    }

    /// `^(\s*)(::|∷)(?![…])\s*`: a type signature on a line of its own, with
    /// groups around the indentation and the colons.
    pub(super) fn signature_line(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let indent = self.space_end(0);
        let end = self
            .lit(indent, b"::")
            .or_else(|| self.lit(indent, "∷".as_bytes()))?;
        if self.here_is(end, is_open_op) {
            return None;
        }
        self.capture(1, 0, indent);
        self.capture(2, indent, end);
        Some(self.space_end(end))
    }

    /// The end of a type signature, on a line of its own when `line`:
    /// `(?=#?\)|[],]|\b(?<!')(in|then|else|of)\b(?!')|([#@])-}|ARROW|[;
    /// }]|END)`. `ARROW` is `(?<![OP])([\\λ]|<-|←|=|-<|↢|-<<|⤛)` before a
    /// character that is no operator.
    /// `END` is `$` inline, and on a line of its own
    /// `^(?!\1\s*\S|\s*(?:$|\{-[^@]|--+(?![OP]).*$))`: a line not indented as
    /// far as the colons, and neither blank nor only a comment.
    pub(super) fn signature_end(&self, p: usize, line: bool) -> Option<usize> {
        let ends = self.starts(p, b")")
            || self.starts(p, b"#)")
            || self.starts(p, b"]")
            || self.starts(p, b",")
            || self.starts(p, b"#-}")
            || self.starts(p, b"@-}")
            || self.starts(p, b";")
            || self.starts(p, b"}")
            || [&b"in"[..], b"then", b"else", b"of"]
                .iter()
                .any(|word| self.keyword_at(p, word).is_some())
            || self.arrow_at(p, &["\\", "λ"])
            || self.arrow_at(p, &ARROWS);
        if ends {
            return Some(p);
        }
        if !line {
            return self.at_eol(p).then_some(p);
        }
        (p == 0 && !self.continues_signature()).then_some(p)
    }

    /// `\1\s*\S|\s*(?:$|\{-[^@]|--+(?![OP]).*$)` at the start of the line:
    /// whether the line is indented as far as the signature's colons, blank,
    /// or only a comment.
    fn continues_signature(&self) -> bool {
        let indent = self.payload_text().as_bytes();
        if self.s.starts_with(indent) && self.space_end(indent.len()) < self.lim() {
            return true;
        }
        self.continues_declaration()
    }

    /// `(?<![…])(::|∷)(?![…])`: a type signature inside a line, with group 1
    /// around the colons.
    pub(super) fn inline_signature(&mut self, p: usize) -> Option<usize> {
        if self.before_is(p, is_open_op) {
            return None;
        }
        let end = self.lit(p, b"::").or_else(|| self.lit(p, "∷".as_bytes()))?;
        if self.here_is(end, is_open_op) {
            return None;
        }
        self.capture(1, p, end);
        Some(end)
    }
}
