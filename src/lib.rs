//! nishiki, a syntax highlighting engine written from scratch.
//!
//! nishiki splits each line of source into tokens and gives every token the
//! scope stack Shiki gives it, so a theme in Shiki's format colours the result
//! as Shiki would. Lexers resume line by line: the state after one line is the
//! input of the next, so an editor can re-highlight from any line.
//!
//! This release highlights Rust ([`rust`]).
//!
//! ```
//! use nishiki::rust::{Highlighter, State};
//!
//! let mut highlighter = Highlighter::new();
//! let mut state = State::default();
//! let mut tokens = Vec::new();
//! let line = "let x = 42;";
//! highlighter.line(line, &mut state, &mut tokens);
//!
//! let mut start = 0;
//! let mut innermost = Vec::new();
//! for token in &tokens {
//!     let scopes = highlighter.scopes(token.stack());
//!     innermost.push((&line[start..token.end()], scopes[scopes.len() - 1]));
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

pub mod rust;

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

    /// The token's scope stack. The highlighter that produced the token
    /// resolves it to scope names.
    #[must_use]
    pub fn stack(self) -> Stack {
        self.stack
    }
}

/// An interned scope stack, which only the highlighter that produced it can
/// resolve, such as with [`rust::Highlighter::scopes`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Stack(usize);
