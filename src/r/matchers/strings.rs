//! Raw strings and the escapes of quoted strings.

use super::super::engine::Lexer;
use crate::classes::is_digit;
use crate::rules::Chars;

impl Lexer<'_> {
    /// `[Rr]"(-*)\{` and the five other forms: a raw string's start, whose
    /// dashes the end repeats.
    /// The dashes give nothing back, since the bracket must follow them.
    pub(super) fn raw_open(&mut self, p: usize, (quote, open, _): (u8, u8, u8)) -> Option<usize> {
        if !matches!(self.at(p), b'R' | b'r') || self.at(p + 1) != quote {
            return None;
        }
        let dashes = self.skip(p + 2, |b| b == b'-');
        if self.at(dashes) != open {
            return None;
        }
        self.capture(1, p + 2, dashes);
        Some(dashes + 1)
    }

    /// `}\1"` and the five other forms: the bracket, the start's dashes and
    /// the quote.
    pub(super) fn raw_close(&self, p: usize, (quote, _, close): (u8, u8, u8)) -> Option<usize> {
        if self.at(p) != close {
            return None;
        }
        let dashes = self.lit(p + 1, self.begin_text().as_bytes())?;
        (self.at(dashes) == quote).then_some(dashes + 1)
    }

    /// ``\\["'\\`abefnrtv]``: a named escape.
    pub(super) fn escape_code(&self, p: usize) -> Option<usize> {
        (self.at(p) == b'\\'
            && matches!(
                self.at(p + 1),
                b'"' | b'\'' | b'\\' | b'`' | b'a' | b'b' | b'e' | b'f' | b'n' | b'r' | b't' | b'v'
            ))
        .then_some(p + 2)
    }

    /// The end of the run of `\h` digits from `i`.
    fn hex_end(&self, i: usize) -> usize {
        self.skip(i, |b| b.is_ascii_hexdigit())
    }

    /// `\\x\h+`: a hexadecimal escape.
    pub(super) fn escape_hex(&self, p: usize) -> Option<usize> {
        let digits = self.lit(p, b"\\x")?;
        let end = self.hex_end(digits);
        (end > digits).then_some(end)
    }

    /// `\\\d{1,3}`: an octal escape, of one to three `\d` characters.
    pub(super) fn escape_octal(&self, p: usize) -> Option<usize> {
        let mut end = self.lit(p, b"\\")?;
        for _ in 0..3 {
            match self.char_at(end) {
                Some((c, next)) if is_digit(c) => end = next,
                Some(_) | None => break,
            }
        }
        (end > p + 1).then_some(end)
    }

    /// `\\[Uu](?:\h+|\{\h+})`: a Unicode escape.
    pub(super) fn escape_unicode(&self, p: usize) -> Option<usize> {
        if self.at(p) != b'\\' || !matches!(self.at(p + 1), b'U' | b'u') {
            return None;
        }
        let digits = self.hex_end(p + 2);
        if digits > p + 2 {
            return Some(digits);
        }
        let open = self.lit(p + 2, b"{")?;
        let digits = self.hex_end(open);
        (digits > open && self.at(digits) == b'}').then_some(digits + 1)
    }

    /// `\\.`: a backslash and a character other than the line feed.
    pub(super) fn escape_any(&self, p: usize) -> Option<usize> {
        (self.at(p) == b'\\' && p + 1 < self.lim()).then(|| self.char_end(p + 1))
    }
}
