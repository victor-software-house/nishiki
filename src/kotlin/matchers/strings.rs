//! Strings, characters and template expressions.

use super::super::engine::Lexer;
use crate::rules::Chars;

impl Lexer<'_> {
    /// `(?<!")"(?!")`: a quote with no quote on either side.
    pub(super) fn string_begin(&self, p: usize) -> Option<usize> {
        (self.at(p) == b'"' && !self.prev_is(p, b'"') && self.at(p + 1) != b'"').then_some(p + 1)
    }

    /// `(?<!")""(?!")`: an empty string.
    pub(super) fn string_empty(&self, p: usize) -> Option<usize> {
        (self.at(p) == b'"'
            && self.at(p + 1) == b'"'
            && !self.prev_is(p, b'"')
            && self.at(p + 2) != b'"')
            .then_some(p + 2)
    }

    /// `\\.`: a backslash and the character after it.
    /// That character is not the line feed.
    pub(super) fn escape(&self, p: usize) -> Option<usize> {
        (self.at(p) == b'\\' && p + 1 < self.lim()).then(|| self.char_end(p + 1))
    }

    /// `(?<!\\)\$\w+\b`.
    pub(super) fn template_variable(&self, p: usize) -> Option<usize> {
        if self.at(p) != b'$' || self.prev_is(p, b'\\') {
            return None;
        }
        let end = self.word_end(p + 1);
        (end > p + 1).then_some(end)
    }

    /// `(?<!\\)(\$\{)`.
    pub(super) fn template_begin(&mut self, p: usize) -> Option<usize> {
        if self.prev_is(p, b'\\') {
            return None;
        }
        let end = self.lit(p, b"${")?;
        self.capture(1, p, end);
        Some(end)
    }

    /// `(})`.
    pub(super) fn template_end(&mut self, p: usize) -> Option<usize> {
        let end = self.lit(p, b"}")?;
        self.capture(1, p, end);
        Some(end)
    }
}
