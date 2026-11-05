//! Strings, their escapes and their format placeholders.

use super::super::engine::Lexer;
use crate::classes::is_alpha;
use crate::rules::Chars;

impl Lexer<'_> {
    /// `(r)?("""|'''|["'])`: a string's opening quote.
    /// Without a quote after it, `(r)?` gives way, and the quote then fails at
    /// the `r`.
    pub(super) fn string_open(&mut self, p: usize) -> Option<usize> {
        let raw = self.starts(p, b"r").then_some(p);
        let quote = raw.map_or(p, |r| r + 1);
        let end = if self.starts(quote, b"\"\"\"") || self.starts(quote, b"'''") {
            quote + 3
        } else if quote < self.lim() && matches!(self.at(quote), b'"' | b'\'') {
            quote + 1
        } else {
            return None;
        };
        if let Some(raw) = raw {
            self.cap(1, raw, raw + 1);
        }
        self.cap(2, quote, end);
        Some(end)
    }

    /// `\\.`: an escape.
    pub(super) fn escape(&self, p: usize) -> Option<usize> {
        let after = self.lit(p, b"\\")?;
        (after < self.lim()).then(|| self.char_end(after))
    }

    /// `(%(\([\w\s]*\))?[-
    /// #+0]*(\d+|\*)?(\.(\d+|\*))?([Lhl])?[%EFGXa-giorsux])`:
    /// a `%` placeholder.
    ///
    /// No part gives back to a later one that could then match.
    /// A `0` the flags take, `\d+` would take as well, and the conversion
    /// letter accepts no character an earlier part takes.
    pub(super) fn percent_placeholder(&mut self, p: usize) -> Option<usize> {
        let mut at = self.lit(p, b"%")?;
        if self.starts(at, b"(") {
            let close = self.skip_words_and_spaces(at + 1);
            at = self.lit(close, b")")?;
        }
        at = self.skip(at, |b| matches!(b, b'-' | b' ' | b'#' | b'+' | b'0'));
        at = self.width(at);
        if self.starts(at, b".") {
            let precision = self.width(at + 1);
            if precision > at + 1 {
                at = precision;
            }
        }
        if at < self.lim() && matches!(self.at(at), b'L' | b'h' | b'l') {
            at += 1;
        }
        let conversion = self.at(at);
        let end = (at < self.lim()
            && matches!(
                conversion,
                b'%' | b'E' | b'F' | b'G' | b'X' | b'a'
                    ..=b'g' | b'i' | b'o' | b'r' | b's' | b'u' | b'x'
            ))
        .then_some(at + 1)?;
        self.cap(1, p, end);
        Some(end)
    }

    /// The end of `[\w\s]*` from `p`, within the line.
    fn skip_words_and_spaces(&self, mut p: usize) -> usize {
        loop {
            let width = self.word_at(p).unwrap_or_else(|| self.space_at(p));
            if width == 0 || p >= self.lim() {
                return p;
            }
            p += width;
        }
    }

    /// The end of `(\d+|\*)?` from `p`.
    fn width(&self, p: usize) -> usize {
        if self.starts(p, b"*") {
            p + 1
        } else {
            self.digit_end(p)
        }
    }

    /// The end of `\{\w*(\.[_[:alpha:]]\w*|\[[^]"']+])*(![ars])?` from the
    /// `{` at `p`: a placeholder's field and conversion, with the conversion
    /// as group 3.
    ///
    /// Each part ends where the next must start with a character it does not
    /// take, so none gives back.
    /// A `[^]"']+` that meets no `]` ends the repetition.
    fn field(&mut self, p: usize) -> Option<usize> {
        let mut at = self.word_end(self.lit(p, b"{")?);
        loop {
            if self.starts(at, b".")
                && let Some((c, end)) = self.char_at(at + 1)
                && (c == '_' || is_alpha(c))
            {
                at = self.word_end(end);
            } else if self.starts(at, b"[") {
                let close = self.skip(at + 1, |b| !matches!(b, b']' | b'"' | b'\''));
                if close == at + 1 || !self.starts(close, b"]") {
                    break;
                }
                at = close + 1;
            } else {
                break;
            }
        }
        if self.starts(at, b"!") && matches!(self.at(at + 1), b'a' | b'r' | b's') {
            self.cap(3, at, at + 2);
            at += 2;
        }
        Some(at)
    }

    /// `(\{\{|}}|\{field(:spec)?})`: a `{}` placeholder, with the format
    /// spec as group 4.
    pub(super) fn brace_placeholder(&mut self, p: usize) -> Option<usize> {
        if let Some(end) = self.lit(p, b"{{").or_else(|| self.lit(p, b"}}")) {
            self.cap(1, p, end);
            return Some(end);
        }
        let mut at = self.field(p)?;
        if self.starts(at, b":") {
            let spec = self.spec(at + 1)?;
            self.cap(4, at, spec);
            at = spec;
        }
        let end = self.lit(at, b"}")?;
        self.cap(1, p, end);
        Some(end)
    }

    /// The end of `\w?[<=>^]?[- +]?#?\d*,?(\.\d+)?[%EFGXb-gnosx]?` from `p`,
    /// when `}` follows it.
    ///
    /// Only `\w?` can take a character a later part needs: a digit for
    /// `\d*`, or a letter for the type.
    /// So it is tried with a character and then without one.
    fn spec(&self, p: usize) -> Option<usize> {
        if let Some(width) = self.word_at(p) {
            let end = self.spec_rest(p + width);
            if self.starts(end, b"}") {
                return Some(end);
            }
        }
        let end = self.spec_rest(p);
        self.starts(end, b"}").then_some(end)
    }

    /// The end of `[<=>^]?[- +]?#?\d*,?(\.\d+)?[%EFGXb-gnosx]?` from `p`.
    fn spec_rest(&self, p: usize) -> usize {
        let mut at = p;
        if matches!(self.at(at), b'<' | b'=' | b'>' | b'^') {
            at += 1;
        }
        if matches!(self.at(at), b'-' | b' ' | b'+') {
            at += 1;
        }
        if self.starts(at, b"#") {
            at += 1;
        }
        at = self.digit_end(at);
        if self.starts(at, b",") {
            at += 1;
        }
        if self.starts(at, b".") {
            let digits = self.digit_end(at + 1);
            if digits > at + 1 {
                at = digits;
            }
        }
        if matches!(
            self.at(at),
            b'%' | b'E' | b'F' | b'G' | b'X' | b'b'..=b'g' | b'n' | b'o' | b's' | b'x'
        ) {
            at += 1;
        }
        at
    }

    /// `(\{field(:)[^\n"'{}]*(?:\{[^\n"'}]*?}[^\n"'{}]*)*})`: a `{}`
    /// placeholder whose format spec holds `{…}`, with the `:` as group 4.
    ///
    /// `[^\n"'}]*?` stops at the first `}`, the only one it can reach.
    /// The runs stop at a `{` or a `}`, which only the next part takes.
    pub(super) fn nested_brace_placeholder(&mut self, p: usize) -> Option<usize> {
        let colon = self.field(p)?;
        let mut at = self.lit(colon, b":")?;
        let plain = |b: u8| !matches!(b, b'"' | b'\'' | b'{' | b'}');
        at = self.skip(at, plain);
        while self.starts(at, b"{") {
            let close = self.skip(at + 1, |b| !matches!(b, b'"' | b'\'' | b'}'));
            if !self.starts(close, b"}") {
                return None;
            }
            at = self.skip(close + 1, plain);
        }
        let end = self.lit(at, b"}")?;
        self.cap(4, colon, colon + 1);
        self.cap(1, p, end);
        Some(end)
    }
}
