//! Symbols, operators, names, labels, brackets and quasi-quotes.

use super::super::engine::Lexer;
use super::{is_name, is_op};
use crate::classes::{is_digit, is_lower, is_symbol_or_punctuation, is_title, is_title_or_upper};
use crate::rules::Chars;

/// `[_\p{Ll}\p{Lu}\p{Lt}\d]`: a letter, digit or `_`.
fn is_name_part(c: char) -> bool {
    c == '_' || is_lower(c) || is_title_or_upper(c) || is_digit(c)
}

/// ``[[_\p{Ll}\p{Lu}\p{Lt}\d\p{S}\p{P}]&&[^(,;\[`{]]``: what may not stand
/// before a prefix symbol or an overloaded label.
fn is_prefix_blocker(c: char) -> bool {
    (is_name_part(c) || is_symbol_or_punctuation(c))
        && !matches!(c, '(' | ',' | ';' | '[' | '`' | '{')
}

/// ``[[_\p{Ll}\p{Lu}\p{Lt}\d\p{S}\p{P}]&&[^#,;\[`{]]``: what must stand before
/// a run of hashes.
fn is_hash_before(c: char) -> bool {
    (is_name_part(c) || is_symbol_or_punctuation(c))
        && !matches!(c, '#' | ',' | ';' | '[' | '`' | '{')
}

/// ``[[_\p{Ll}\p{Lu}\p{Lt}\d\p{S}\p{P}]&&[^]),;`}]]``: what may not follow a
/// run of hashes.
fn is_hash_blocker(c: char) -> bool {
    (is_name_part(c) || is_symbol_or_punctuation(c))
        && !matches!(c, ']' | ')' | ',' | ';' | '`' | '}')
}

/// `[])_}\p{Ll}\p{Lu}\p{Lt}\d]`: what stands before an as-pattern's `@`.
fn is_as_before(c: char) -> bool {
    matches!(c, ']' | ')' | '}') || is_name_part(c)
}

/// `[(\[_{\p{Ll}\p{Lu}\p{Lt}\d]`: what follows an as-pattern's `@` or a
/// prefix symbol.
fn is_prefix_operand(c: char) -> bool {
    matches!(c, '(' | '[' | '{') || is_name_part(c)
}

/// `['._\p{Ll}\p{Lu}\p{Lt}\d]`: a character of an overloaded label.
fn is_label_part(c: char) -> bool {
    matches!(c, '\'' | '.') || is_name_part(c)
}

/// The reserved symbols, each with its group, in the grammar's order.
const RESERVED: [(usize, &str); 18] = [
    (1, ".."),
    (2, ":"),
    (3, "="),
    (4, "\\"),
    (5, "|"),
    (6, "<-"),
    (6, "←"),
    (7, "->"),
    (7, "→"),
    (8, "-<"),
    (8, "↢"),
    (9, "-<<"),
    (9, "⤛"),
    (10, ">-"),
    (10, "⤚"),
    (11, ">>-"),
    (11, "⤜"),
    (12, "∀"),
];

impl Lexer<'_> {
    /// `(?<![#@])-}`: a stray end of a block comment.
    pub(super) fn stray_close(&self, p: usize) -> Option<usize> {
        if self.prev_is(p, b'#') || self.prev_is(p, b'@') {
            return None;
        }
        self.lit(p, b"-}")
    }

    /// `(?<![OP])(\.\.)(?![OP])`, with group 1 around the dots.
    pub(super) fn record_wildcard(&mut self, p: usize) -> Option<usize> {
        if self.before_is(p, is_op) {
            return None;
        }
        let end = self.alone_after(p, &[b".."])?;
        self.cap(1, p, end);
        Some(end)
    }

    /// `(\()\s*(?!(?:--+|\.\.)\))(#+|[OP]+(?<!#))\s*(\))`: an operator in
    /// parentheses, with groups around each part.
    ///
    /// The hashes are tried first, then the operator characters.
    /// An operator that gives back its last hashes leaves a hash where the
    /// `)` must stand.
    pub(super) fn prefix_op(&mut self, p: usize) -> Option<usize> {
        if !(p < self.lim() && self.at(p) == b'(') {
            return None;
        }
        let start = self.space_end(p + 1);
        if self.dashes_close(start) || self.starts(start, b"..)") {
            return None;
        }
        let close = self
            .operator_ends(start)
            .into_iter()
            .flatten()
            .find_map(|end| {
                let close = self.space_end(end);
                (close < self.lim() && self.at(close) == b')').then_some((end, close))
            });
        let (end, close) = close?;
        self.cap(1, p, p + 1);
        self.cap(2, start, end);
        self.cap(3, close, close + 1);
        Some(close + 1)
    }

    /// `(?<![OP])(?:(\.\.)|(:)|(=)|…|(∀))(?![OP])`: a reserved symbol, with a
    /// group around it.
    pub(super) fn reserved_symbol(&mut self, p: usize) -> Option<usize> {
        if self.before_is(p, is_op) {
            return None;
        }
        let (group, end) = RESERVED.iter().find_map(|&(group, text)| {
            let end = self.alone_after(p, &[text.as_bytes()])?;
            Some((group, end))
        })?;
        self.cap(group, p, end);
        Some(end)
    }

    /// `(?<=[…])(#+)(?![…])`: a run of hashes after a name or a symbol, with
    /// group 1 around it.
    ///
    /// Giving back a hash leaves a hash after the rest.
    pub(super) fn hashes(&mut self, p: usize) -> Option<usize> {
        if !self.before_is(p, is_hash_before) || self.at(p) != b'#' || p >= self.lim() {
            return None;
        }
        let end = self.skip(p, |b| b == b'#');
        if self.here_is(end, is_hash_blocker) {
            return None;
        }
        self.cap(1, p, end);
        Some(end)
    }

    /// `(?<=[])_}\p{Ll}\p{Lu}\p{Lt}\d])(@)(?=[(\[_{\p{Ll}\p{Lu}\p{Lt}\d])`: an
    /// as-pattern's `@`, with group 1 around it.
    pub(super) fn as_pattern(&mut self, p: usize) -> Option<usize> {
        if !self.before_is(p, is_as_before)
            || self.at(p) != b'@'
            || !self.here_is(p + 1, is_prefix_operand)
        {
            return None;
        }
        self.cap(1, p, p + 1);
        Some(p + 1)
    }

    /// `(?<![…])(?:(~)|(!)|(-)|(\$)|(\$\$))(?=[(\[_{\p{Ll}\p{Lu}\p{Lt}\d])`: a
    /// prefix symbol, with a group around it.
    pub(super) fn prefix_symbol(&mut self, p: usize) -> Option<usize> {
        if self.before_is(p, is_prefix_blocker) {
            return None;
        }
        let symbols: [(usize, &[u8]); 5] = [(1, b"~"), (2, b"!"), (3, b"-"), (4, b"$"), (5, b"$$")];
        let (group, end) = symbols.iter().find_map(|&(group, text)| {
            let end = self.lit(p, text)?;
            self.here_is(end, is_prefix_operand).then_some((group, end))
        })?;
        self.cap(group, p, end);
        Some(end)
    }

    /// `\b(?<!')[\p{Lu}\p{Lt}]['_\p{Ll}\p{Lu}\p{Lt}\d]*(?!['.\w])`: a data
    /// constructor.
    ///
    /// Giving back a character leaves a name character after the rest.
    pub(super) fn data_constructor(&self, p: usize) -> Option<usize> {
        if self.word_before(p) || self.prev_is(p, b'\'') {
            return None;
        }
        let end = self.name_from(p, is_title_or_upper)?;
        (self.word_at(end).is_none() && !matches!(self.at(end), b'\'' | b'.')).then_some(end)
    }

    /// `((?:(?<!'')('')?[\p{Lu}\p{Lt}]…\.)*)(#+|[OP]+(?<!#))`: an infix
    /// operator, with groups around the qualifiers, the last qualifier's
    /// quotes and the operator.
    ///
    /// Giving back a qualifier leaves the operator at a letter or a quote.
    pub(super) fn infix_op(&mut self, p: usize) -> Option<usize> {
        let mut op = p;
        let mut quotes = None;
        loop {
            let at = if !(op >= 2 && self.text(op - 2, op) == b"''") && self.starts(op, b"''") {
                op + 2
            } else {
                op
            };
            match self.name_from(at, is_title_or_upper) {
                Some(name) if name < self.lim() && self.at(name) == b'.' => {
                    if at > op {
                        quotes = Some(op);
                    }
                    op = name + 1;
                }
                _ => break,
            }
        }
        let end = self.operator_ends(op).into_iter().flatten().next()?;
        self.cap(1, p, op);
        if let Some(quotes) = quotes {
            self.cap(2, quotes, quotes + 2);
        }
        self.cap(3, op, end);
        Some(end)
    }

    /// ``(`)((?:[\p{Lu}\p{Lt}]…\.)*)([_\p{Ll}\p{Lu}]…)(`)``: a function in
    /// backticks, with groups around each part.
    ///
    /// Giving back a qualifier leaves its dot where the backtick must stand.
    pub(super) fn infix_backticks(&mut self, p: usize) -> Option<usize> {
        let qualifiers = self.lit(p, b"`")?;
        let mut name = qualifiers;
        while let Some(end) = self.name_from(name, is_title_or_upper)
            && end < self.lim()
            && self.at(end) == b'.'
        {
            name = end + 1;
        }
        let (c, next) = self.char_here(name)?;
        if !(c == '_' || is_lower(c) || is_title_or_upper(c)) || is_title(c) {
            return None;
        }
        let close = self.class_end(next, is_name);
        let end = self.lit(close, b"`")?;
        self.cap(1, p, qualifiers);
        self.cap(2, qualifiers, name);
        self.cap(3, name, close);
        self.cap(4, close, end);
        Some(end)
    }

    /// `(?<![…])(#)(?:("(?:\\"|[^"])*")|['._\p{Ll}\p{Lu}\p{Lt}\d]+)`: an
    /// overloaded label, with groups around the hash and the string.
    pub(super) fn overloaded_label(&mut self, p: usize) -> Option<usize> {
        if self.before_is(p, is_prefix_blocker) {
            return None;
        }
        let at = self.lit(p, b"#")?;
        if let Some(end) = self.quoted_end(at) {
            self.cap(1, p, at);
            self.cap(2, at, end);
            return Some(end);
        }
        let end = self.class_end(at, is_label_part);
        if end == at {
            return None;
        }
        self.cap(1, p, at);
        Some(end)
    }

    /// `\b(?<!')[\p{Lu}\p{Lt}]['_\p{Ll}\p{Lu}\p{Lt}\d]*\.`: a module qualifier.
    pub(super) fn qualifier(&self, p: usize) -> Option<usize> {
        if self.word_before(p) || self.prev_is(p, b'\'') {
            return None;
        }
        let end = self.name_from(p, is_title_or_upper)?;
        (end < self.lim() && self.at(end) == b'.').then_some(end + 1)
    }

    /// `(\()(#)\s`, with groups around the parenthesis and the hash.
    pub(super) fn unboxed_open(&mut self, p: usize) -> Option<usize> {
        let hash = self.lit(p, b"(#")?;
        let width = self.space_at(hash);
        if width == 0 {
            return None;
        }
        self.cap(1, p, p + 1);
        self.cap(2, p + 1, hash);
        Some(hash + width)
    }

    /// The end of `\|\|?` at `i`.
    fn bars(&self, i: usize) -> Option<usize> {
        let bar = self.lit(i, b"|")?;
        Some(self.lit(bar, b"|").unwrap_or(bar))
    }

    /// `(\[)([x])?(\|\|?)` for the letters `letters`, the letter required
    /// unless `optional`: a quasi-quote's start, with groups around the
    /// bracket, the letter and the bars.
    ///
    /// A letter that no bar follows leaves the bar at a letter, where it
    /// fails.
    pub(super) fn quasi_quote(
        &mut self,
        p: usize,
        letters: &[u8],
        optional: bool,
    ) -> Option<usize> {
        let after = self.lit(p, b"[")?;
        let letter = (after < self.lim() && letters.contains(&self.at(after))).then_some(after);
        if letter.is_none() && !optional {
            return None;
        }
        let bars = letter.map_or(after, |at| at + 1);
        let end = self.bars(bars)?;
        self.cap(1, p, after);
        if let Some(at) = letter {
            self.cap(2, at, at + 1);
        }
        self.cap(3, bars, end);
        Some(end)
    }

    /// `(\[)(?:(\$\$)|(\$))?(['._[^\s\p{S}\p{P}]]*)(\|\|?)`: a quasi-quote
    /// with a quoter, with groups around each part.
    ///
    /// The quoter gives back no character that helps: a bar is no character
    /// of a quoter.
    pub(super) fn quasi_quoter(&mut self, p: usize) -> Option<usize> {
        let after = self.lit(p, b"[")?;
        let (group, quoter) = if let Some(end) = self.lit(after, b"$$") {
            (Some(2), end)
        } else if let Some(end) = self.lit(after, b"$") {
            (Some(3), end)
        } else {
            (None, after)
        };
        let mut name = quoter;
        while let Some((c, next)) = self.char_here(name)
            && (matches!(c, '\'' | '.' | '_')
                || (self.space_at(name) == 0 && !is_symbol_or_punctuation(c)))
        {
            name = next;
        }
        self.spend(name - quoter);
        let end = self.bars(name)?;
        self.cap(1, p, after);
        if let Some(group) = group {
            self.cap(group, after, quoter);
        }
        self.cap(4, quoter, name);
        self.cap(5, name, end);
        Some(end)
    }

    /// `\3]` or `\5]`: the bars that opened the quasi-quote, then `]`.
    pub(super) fn quasi_quote_end(&self, p: usize) -> Option<usize> {
        let bars = self.payload_at(p)?;
        self.lit(p + bars, b"]")
    }
}
