//! The matchers: for each regular expression of the grammar, the function that
//! decides whether it matches at a position and where.
//! A matcher records the capture groups the grammar names with [`Lexer::cap`].
//! The engine names them.
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
use super::grammar::{Pat, Split};

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
    /// Whether `pat` matches at `p`, and where its match ends.
    pub(super) fn matcher(&mut self, pat: Pat, p: usize) -> Option<usize> {
        match pat.split() {
            Split::Code(pat) => self.code_matcher(pat, p),
            Split::String(pat) => self.string_matcher(pat, p),
            Split::Fstring(pat) => self.fstring_matcher(pat, p),
            Split::Regexp(pat) => self.regexp_matcher(pat, p),
            Split::Flavoured(flavour, kind) => self.flavoured(flavour, kind, p),
        }
    }
}
