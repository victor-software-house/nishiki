//! The capture groups that the grammar tokenizes with patterns of their own.

use super::engine::Lexer;
use super::grammar::Rule;

impl Lexer<'_> {
    /// Emits the tokens of capture group text `from..to`, and leaves the
    /// cursor at `to`.
    /// The text sits under the stack `base`.
    pub(super) fn tokenize_capture(&mut self, kind: Rule, from: usize, to: usize, base: u32) {
        self.retokenise(kind, from, to, base);
        self.emit(to, base);
    }
}
