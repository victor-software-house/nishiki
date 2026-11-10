//! Strings and their interpolations.

use super::super::engine::Lexer;
use crate::rules::Chars;

impl Lexer<'_> {
    /// `(?<!r)"""` when `quotes` is `"""`, and `(?<!r)'''` when it is `'''`.
    pub(super) fn triple_open(&self, p: usize, quotes: &[u8]) -> Option<usize> {
        if self.prev_is(p, b'r') {
            return None;
        }
        self.lit(p, quotes)
    }

    /// `"""(?!")` when `quote` is `"`, and `'''(?!')` when it is `'`.
    pub(super) fn triple_close(&self, p: usize, quote: u8) -> Option<usize> {
        let end = self.lit(p, &[quote; 3])?;
        (self.at(end) != quote).then_some(end)
    }

    /// `(?<!\|r)"` when `quote` is `"`, and `(?<!\|r)'` when it is `'`.
    /// The grammar's lookbehind is for the two characters `|r`.
    pub(super) fn quote_open(&self, p: usize, quote: u8) -> Option<usize> {
        if self.at(p) != quote || (p >= 2 && self.text(p - 2, p) == b"|r") {
            return None;
        }
        Some(p + 1)
    }

    /// `\n`: the line feed at the end of the line.
    pub(super) fn line_feed(&self, p: usize) -> Option<usize> {
        (p == self.lim() && self.avail(p)).then_some(p + 1)
    }

    /// `\$([0-9A-Z_a-z]+)`, with group 1 around the name.
    pub(super) fn interpolated_name(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b'$' {
            return None;
        }
        let end = self.skip(p + 1, |b| b == b'_' || b.is_ascii_alphanumeric());
        if end == p + 1 {
            return None;
        }
        self.capture(1, p + 1, end);
        Some(end)
    }

    /// `\\.`: a backslash and the character after it.
    /// That character is not the line feed.
    pub(super) fn escape(&self, p: usize) -> Option<usize> {
        (self.at(p) == b'\\' && p + 1 < self.lim()).then(|| self.char_end(p + 1))
    }
}
