//! The matchers: for each regular expression of the grammar, the function that
//! decides whether it matches at a position and where.
//! A matcher records the capture groups the grammar names with
//! [`Lexer::capture`]. The engine names them.
//!
//! Each follows its expression as an Oniguruma-style backtracking matcher
//! would.
//! Where a quantifier could give characters back, the doc comment says why it
//! cannot matter.

mod chars;
mod code;
mod fstrings;
mod regexp;
mod strings;

use super::engine::Lexer;
use super::grammar::{Pattern, Split};

/// Whether a string's quotes are one character or three.
/// The grammar writes the two as separate rules.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Quotes {
    /// `'` or `"`.
    One,
    /// `'''` or `"""`.
    Three,
}

impl Lexer<'_> {
    /// Whether `pattern` matches at `p`, and where its match ends.
    pub(super) fn matcher(&mut self, pattern: Pattern, p: usize) -> Option<usize> {
        match pattern.split() {
            Split::Code(pattern) => self.code_matcher(pattern, p),
            Split::String(pattern) => self.string_matcher(pattern, p),
            Split::Fstring(pattern) => self.fstring_matcher(pattern, p),
            Split::Regexp(pattern) => self.regexp_matcher(pattern, p),
            Split::Flavoured(flavour, re) => self.flavoured(flavour, re, p),
        }
    }
}
