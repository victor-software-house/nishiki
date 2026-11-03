//! The capture groups that the grammar tokenizes with patterns of their own.

use super::engine::Lexer;
use super::grammar::Nested;

impl Lexer<'_> {
    /// Emits the tokens of capture group text `from..to`, which sits under
    /// the stack `base`, and leaves the cursor at `to`.
    pub(super) fn tokenize_capture(&mut self, kind: Nested, from: usize, to: usize, base: u32) {
        self.retokenise(kind, from, to, base);
        self.emit(to, base);
    }
}
