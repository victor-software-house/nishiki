//! The matchers: for each regular expression of the grammar, the function that
//! decides whether it matches at a position and where.
//! A matcher records the capture groups the grammar names with [`Lexer::cap`].
//! The engine names them.
//! `dispatch.rs` says which matcher answers each pattern.
//!
//! Each follows its expression as an Oniguruma-style backtracking matcher
//! would.
//! Where a quantifier could give characters back, the doc comment says why it
//! cannot matter.

pub(super) mod declarations;
pub(super) mod expressions;
pub(super) mod literals;
pub(super) mod names;
pub(super) mod regex_literals;
pub(super) mod types;
pub(super) mod words;
