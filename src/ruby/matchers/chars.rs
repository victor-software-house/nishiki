//! The runs of characters Ruby's expressions are built from, on the shared
//! character classes of [`Chars`].

use super::super::engine::Lexer;
use crate::rules::{CachedRuns, Chars};

impl Lexer<'_> {
    /// Whether the byte at `p` of the text is in `class`.
    pub(in crate::ruby) fn byte_in(&self, p: usize, class: impl Fn(u8) -> bool) -> bool {
        p < self.lim() && class(self.at(p))
    }

    /// Whether `[A-Z_a-z]` stands at `p`: a name's first character.
    pub(in crate::ruby) fn name_start(&self, p: usize) -> bool {
        self.byte_in(p, |b| b.is_ascii_alphabetic() || b == b'_')
    }

    /// The end of `[A-Z_a-z]\w*` at `p`: a name.
    pub(in crate::ruby) fn name_end(&self, p: usize) -> Option<usize> {
        self.name_start(p).then(|| self.word_run(p + 1))
    }

    /// The end of `[_a-z][0-9A-Z_a-z]*` at `p`: a local variable's name.
    pub(in crate::ruby) fn local_end(&self, p: usize) -> Option<usize> {
        self.byte_in(p, |b| b.is_ascii_lowercase() || b == b'_')
            .then(|| self.skip(p + 1, |b| b.is_ascii_alphanumeric() || b == b'_'))
    }

    /// Whether the character at `p` is a `\s`.
    pub(in crate::ruby) fn space_char(&self, p: usize) -> bool {
        self.space_at(p) > 0
    }

    /// The end of the character at `p`, the line feed included.
    pub(in crate::ruby) fn char_end_nl(&self, p: usize) -> usize {
        if p < self.lim() {
            self.char_end(p)
        } else {
            p + 1
        }
    }

    /// `(?<=^|\s)` at `p`.
    pub(in crate::ruby) fn after_blank_or_start(&self, p: usize) -> bool {
        p == 0 || self.char_before(p).is_some_and(crate::classes::is_space)
    }

    /// The end of `\bword` at `p`, for a `word` that ends in a character that
    /// is not `\w`.
    /// No `\b` follows such a word.
    pub(in crate::ruby) fn word_then(&self, p: usize, word: &[u8]) -> Option<usize> {
        self.boundary(p).then(|| self.lit(p, word)).flatten()
    }

    /// `(?![!?])` at `p`.
    pub(in crate::ruby) fn no_bang(&self, p: usize) -> bool {
        !self.byte_in(p, |b| b == b'!' || b == b'?')
    }

    /// `(?<!\.)` at `p`.
    pub(in crate::ruby) fn not_after_dot(&self, p: usize) -> bool {
        !self.prev_is(p, b'.')
    }
}
