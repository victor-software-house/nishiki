//! The matchers: for each regular expression of the grammar, the function that
//! decides whether it matches at a position and where. A matcher records the
//! capture groups the grammar names with [`Lexer::cap`]; the engine names them.
//!
//! Each follows its expression as an Oniguruma-style backtracking matcher
//! would. Where a quantifier could give characters back, the doc comment says
//! why it cannot matter.

mod chars;
mod code;
mod fstrings;
mod regexp;
mod strings;

use super::engine::Lexer;
use super::grammar::{Family, Pat};

impl Lexer<'_> {
    /// Whether `pat` matches at `p`, and where its match ends.
    pub(super) fn matcher(&mut self, pat: Pat, p: usize) -> Option<usize> {
        match pat.family() {
            Family::Code => self.code_matcher(pat, p),
            Family::String => self.string_matcher(pat, p),
            Family::Fstring => self.fstring_matcher(pat, p),
            Family::Regexp => self.regexp_matcher(pat, p),
        }
    }
}
