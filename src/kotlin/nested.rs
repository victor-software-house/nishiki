//! The capture groups that the grammar tokenizes with patterns of their own.

use super::engine::Lexer;
use super::grammar::Nested;
use super::scope::S;
use crate::rules::Chars;

impl Lexer<'_> {
    /// Emits the tokens of capture group text `from..to`, which sits under
    /// the stack `base`, and leaves the cursor at `to`.
    pub(super) fn tokenize_capture(&mut self, kind: Nested, from: usize, to: usize, base: u32) {
        let to = to.min(self.s.len());
        match kind {
            Nested::TypeParameter => self.type_parameters(from, to, base),
        }
        self.emit(to, base);
    }

    /// Every run of `\w` characters is a type: the grammar's `\b\w+\b` comes
    /// before its `in` and `out` pattern, so those two words are types too.
    /// A run that starts inside a word, where the capture starts, has no `\b`
    /// before it.
    fn type_parameters(&mut self, from: usize, to: usize, base: u32) {
        let mut i = from;
        while i < to {
            if self.word_at(i).is_none() {
                i = self.char_end(i);
                continue;
            }
            let end = self.word_end(i).min(to);
            if i != from || !self.word_before(i) {
                self.emit(i, base);
                let node = self.path(base, &[S::EntNameType]);
                self.emit(end, node);
            }
            i = end;
        }
    }
}
