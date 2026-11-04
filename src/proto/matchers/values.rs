//! Numbers and strings.

use super::Lexer;
use crate::rules::Chars;

impl Lexer<'_> {
    /// A number:
    ///
    /// ```text
    /// \b((0([Xx])\h*)|(([0-9]+\.?[0-9]*)|(\.[0-9]+))(([Ee])([-+])?[0-9]+)?)\b
    /// ```
    ///
    /// Every run here holds word characters, so giving one back leaves a word
    /// character on both sides of the closing `\b`.
    /// Two places hold a `\b` with fewer bytes:
    /// - after the `.` of `[0-9]+\.?`, when the fraction's digits all give
    ///   themselves back: `1.5x` ends as `1.`;
    /// - before the `.`, when the `\.?` gives itself back.
    ///
    /// The branch `\.[0-9]+` starts with a non-word byte, so its `\b` needs a
    /// word character before the `.`.
    pub(super) fn number(&self, p: usize) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        if self.at(p) == b'0' && matches!(self.at(p + 1), b'X' | b'x') {
            let end = self.skip(p + 2, |b| b.is_ascii_hexdigit());
            if self.boundary(end) {
                return Some(end);
            }
        }
        if self.at(p) == b'.' {
            if !self.at(p + 1).is_ascii_digit() {
                return None;
            }
            let digits = self.skip(p + 1, |b| b.is_ascii_digit());
            return [self.exponent_end(digits), Some(digits)]
                .into_iter()
                .flatten()
                .find(|&end| self.boundary(end));
        }
        if !self.at(p).is_ascii_digit() {
            return None;
        }
        let whole = self.skip(p, |b| b.is_ascii_digit());
        let ends = if self.at(whole) == b'.' {
            let fraction = self.skip(whole + 1, |b| b.is_ascii_digit());
            [
                self.exponent_end(fraction),
                Some(fraction),
                Some(whole + 1),
                Some(whole),
            ]
        } else {
            [self.exponent_end(whole), Some(whole), None, None]
        };
        ends.into_iter().flatten().find(|&end| self.boundary(end))
    }

    /// `([Ee])([-+])?[0-9]+` from `i`.
    /// A sign with no digit after it leaves the sign itself, which is no
    /// digit, so the exponent fails.
    fn exponent_end(&self, i: usize) -> Option<usize> {
        if !matches!(self.at(i), b'E' | b'e') {
            return None;
        }
        let digits = if matches!(self.at(i + 1), b'-' | b'+') {
            i + 2
        } else {
            i + 1
        };
        self.at(digits)
            .is_ascii_digit()
            .then(|| self.skip(digits, |b| b.is_ascii_digit()))
    }

    /// `(["'])(?:\\.|[^\\])*?\1`: a string, up to the first quote like its
    /// opening one.
    ///
    /// The lazy loop tries the closing quote before each character.
    /// `\\.` and `[^\\]` start with different bytes, so the loop has one way to
    /// go on.
    /// `.` takes no line feed, so a backslash before the line's end stops the
    /// string; `[^\\]` takes the line feed, after which no quote can stand.
    pub(super) fn string(&self, p: usize) -> Option<usize> {
        let quote = self.at(p);
        if !matches!(quote, b'"' | b'\'') {
            return None;
        }
        let mut i = p + 1;
        let found = loop {
            match self.at(i) {
                b if b == quote => break Some(i + 1),
                b'\\' if i + 1 < self.lim() => i = self.char_end(i + 1),
                b'\\' => break None,
                _ if i < self.lim() => i = self.char_end(i),
                _ => break None,
            }
        };
        self.spend(i - p);
        found
    }
}
