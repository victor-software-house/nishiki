//! Numbers.

use super::super::engine::Lexer;
use crate::classes::is_digit;
use crate::rules::Chars;

impl Lexer<'_> {
    /// The end of the run of `[_\d]` characters from `i`.
    fn digits_and_underscores(&self, mut i: usize) -> usize {
        let from = i;
        while i < self.lim() {
            match self.char_at(i) {
                Some(('_', end)) => i = end,
                Some((c, end)) if is_digit(c) => i = end,
                _ => break,
            }
        }
        self.spend(i - from);
        i
    }

    /// `\b\d[_\d]*(\.[_\d]+)?(([Ee])\d+)?([Uu])?([FLf])?\b`.
    ///
    /// The runs take everything they can and giving any back cannot help:
    /// the next character would be a digit or an underscore, which neither
    /// starts a later part nor makes `\b` hold inside a word. The optional
    /// parts are tried taken before left out, the last one varying fastest,
    /// and the first combination that ends at a boundary is the match.
    pub(super) fn decimal(&self, p: usize) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        let (first, after) = self.char_at(p).filter(|_| p < self.lim())?;
        if !is_digit(first) {
            return None;
        }
        let whole = self.digits_and_underscores(after);
        let fraction = (self.at(whole) == b'.')
            .then(|| self.digits_and_underscores(whole + 1))
            .filter(|&end| end > whole + 1);
        let exponent = |from: usize| {
            matches!(self.at(from), b'E' | b'e')
                .then(|| self.digit_end(from + 1))
                .filter(|&end| end > from + 1)
        };
        let one_of =
            |from: usize, letters: &[u8]| letters.contains(&self.at(from)).then_some(from + 1);
        for after_fraction in [fraction, None] {
            let at = after_fraction.unwrap_or(whole);
            for after_exponent in [exponent(at), None] {
                let at = after_exponent.unwrap_or(at);
                for after_unsigned in [one_of(at, b"Uu"), None] {
                    let at = after_unsigned.unwrap_or(at);
                    for after_suffix in [one_of(at, b"FLf"), None] {
                        let end = after_suffix.unwrap_or(at);
                        if self.boundary(end) {
                            return Some(end);
                        }
                    }
                }
            }
        }
        None
    }

    /// `0([Xx])\h[_\h]*([Uu])?`: there is no `\b` before it or after it.
    pub(super) fn hex(&self, p: usize) -> Option<usize> {
        let digits = p + 2;
        if self.at(p) != b'0' || !matches!(self.at(p + 1), b'X' | b'x') {
            return None;
        }
        if !self.at(digits).is_ascii_hexdigit() {
            return None;
        }
        let end = self.skip(digits + 1, |b| b.is_ascii_hexdigit() || b == b'_');
        Some(if matches!(self.at(end), b'U' | b'u') {
            end + 1
        } else {
            end
        })
    }

    /// `0([Bb])[01][01_]*`.
    pub(super) fn binary(&self, p: usize) -> Option<usize> {
        let digits = p + 2;
        if self.at(p) != b'0' || !matches!(self.at(p + 1), b'B' | b'b') {
            return None;
        }
        if !matches!(self.at(digits), b'0' | b'1') {
            return None;
        }
        Some(self.skip(digits + 1, |b| matches!(b, b'0' | b'1' | b'_')))
    }
}
