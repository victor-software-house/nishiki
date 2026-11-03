//! The matchers of keywords: the storage modifiers, and a keyword alone, as
//! a group or before a blank.

use crate::java::engine::Lexer;
use crate::rules::Chars;

/// The storage modifiers, in the grammar's order.
const MODIFIERS: &[&[u8]] = &[
    b"public",
    b"private",
    b"protected",
    b"static",
    b"final",
    b"native",
    b"synchronized",
    b"abstract",
    b"threadsafe",
    b"transient",
    b"volatile",
    b"default",
    b"strictfp",
    b"sealed",
    b"non-sealed",
];

impl Lexer<'_> {
    /// `\b(public|...|non-sealed)\b`.
    pub(super) fn storage_modifier(&self, p: usize) -> Option<usize> {
        self.keyword_of(p, MODIFIERS)
    }

    /// `\b(word)\b` with the word as group 1.
    pub(super) fn keyword_group(&mut self, p: usize, word: &[u8]) -> Option<usize> {
        let end = self.keyword(p, word)?;
        self.cap(1, p, end);
        Some(end)
    }

    /// `(word)\s`: the word and one white space character.
    pub(super) fn keyword_blank(&mut self, p: usize, word: &[u8]) -> Option<usize> {
        let end = self.lit(p, word)?;
        let width = self.space_at(end);
        if width == 0 {
            return None;
        }
        self.cap(1, p, end);
        Some(end + width)
    }
}
