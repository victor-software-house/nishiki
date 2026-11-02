//! The nesting depths the grammar's own lookahead expressions have.
//!
//! They are facts of the grammar, not safety limits: a group nested deeper than
//! the grammar's regular expressions spell does not match, so the lexer must
//! not match it either. Safety limits come from `Limits`.

/// The levels of nested brackets the grammar's lookahead expressions spell
/// out; a group nested deeper does not match.
pub(crate) const GRAMMAR_NESTING: usize = 3;

/// The levels of nested `<…>` in the grammar's type-argument lookaheads.
pub(crate) const GENERIC_NESTING: usize = 3;
