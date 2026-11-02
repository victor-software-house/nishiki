//! nishiki, a syntax highlighting engine written from scratch.
//!
//! nishiki splits each line of source into tokens and gives every token the
//! scope stack Shiki gives it, so a theme in Shiki's format colours the result
//! as Shiki would. Lexers resume line by line: the state after one line is the
//! input of the next, so an editor can re-highlight from any line, and a
//! streamed reply can be highlighted as it arrives.
//!
//! This release highlights Rust ([`Language::Rust`]).
//!
//! ```
//! use nishiki::{Highlighter, Language, State};
//!
//! let mut highlighter = Highlighter::new(Language::Rust);
//! let mut state = State::default();
//! let mut tokens = Vec::new();
//! let line = "let x = 42;";
//! let report = highlighter.line(line, &mut state, &mut tokens);
//! assert!(report.is_complete());
//!
//! let mut start = 0;
//! let mut innermost = Vec::new();
//! for token in &tokens {
//!     let scope = highlighter.scopes(token.stack()).next();
//!     innermost.push((&line[start..token.end()], scope.unwrap_or_default()));
//!     start = token.end();
//! }
//! assert_eq!(
//!     innermost,
//!     [
//!         ("let", "storage.type.rust"),
//!         (" ", "source.rust"),
//!         ("x", "variable.other.rust"),
//!         (" ", "source.rust"),
//!         ("=", "keyword.operator.assignment.equal.rust"),
//!         (" ", "source.rust"),
//!         ("42", "constant.numeric.decimal.rust"),
//!         (";", "punctuation.semi.rust"),
//!     ]
//! );
//! ```
//!
//! Every line is bounded by [`Limits`], which the caller can set, raise,
//! lower or turn off; each [`LineReport`] says which limits a line reached.

mod arena;
mod highlighter;
mod language;
mod limits;
mod rust;
mod scope;

pub use arena::Scopes;
pub use highlighter::Highlighter;
pub use language::Language;
pub use limits::{Budget, Limit, Limits, LineReport, Work};

/// The README's examples, compiled and run as doctests.
#[cfg(doctest)]
#[doc = include_str!("../README.md")]
struct ReadmeDoctests;

/// A run of bytes in one line that shares one scope stack.
///
/// A token starts where the previous token of its line ended, or at byte 0,
/// and ends at [`Token::end`]. The tokens of a line cover it without gaps.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Token {
    end: usize,
    stack: Stack,
}

impl Token {
    /// The byte offset in the line where the token ends, exclusive.
    #[must_use]
    pub fn end(self) -> usize {
        self.end
    }

    /// The token's scope stack, which [`Highlighter::scopes`] resolves to
    /// scope names.
    #[must_use]
    pub fn stack(self) -> Stack {
        self.stack
    }
}

/// An interned scope stack, which only the highlighter that produced it can
/// resolve, with [`Highlighter::scopes`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Stack(u32);

/// A lexer state between two lines: the contexts left open, such as a
/// string or a comment.
///
/// A small `Copy` handle into the state its [`Highlighter`] interned, so
/// equal states compare equal in one step. [`State::default`] starts a
/// document. A state belongs to the highlighter that produced it.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct State(u32);
