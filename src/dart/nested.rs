//! The capture group that the grammar tokenizes with patterns of its own.

use super::engine::Lexer;
use super::grammar::{Nested, Rule};

impl Lexer<'_> {
    /// Emits the tokens of capture group text `from..to`, and leaves the
    /// cursor at `to`.
    /// The text sits under the stack `base`.
    pub(super) fn tokenize_capture(&mut self, kind: Nested, from: usize, to: usize, base: u32) {
        let to = to.min(self.s.len());
        match kind {
            Nested::TypeArgs => self.retokenise(Rule::TypeArgsGroup, from, to, base),
        }
        self.emit(to, base);
    }
}
