//! Declarations: namespaces, `use` and arrays.

use super::super::PhpFamily;
use crate::rules::{Chars, Lexer};

#[cfg_attr(
    nishiki_unstable,
    expect(private_bounds, reason = "the bound and the methods are crate-private")
)]
impl<G: PhpFamily> Lexer<'_, G> {
    /// `(?i)(?:^|(?<=<\?php))\s*(namespace)\s+([0-9\\_a-z\x7F-\x{10FFFF}]+)(?=\
    /// s*;)` with `named`, else `(?i)(?:^|(?<=<\?php))\s*(namespace)\s+`.
    pub(crate) fn namespace_statement(&mut self, p: usize, named: bool) -> Option<usize> {
        let after_tag = p >= 5 && self.text(p - 5, p).eq_ignore_ascii_case(b"<?php");
        if p != 0 && !after_tag {
            return None;
        }
        let word = self.space_end(p);
        let word_end = self.ilit(word, b"namespace")?;
        let at = self.space_end(word_end);
        if at == word_end {
            return None;
        }
        if !named {
            self.capture(1, word, word_end);
            return Some(at);
        }
        let (at, end) = self.after_spaces(word_end, true, Self::qualified)?;
        if self.at(self.space_end(end)) != b';' {
            return None;
        }
        self.capture(1, word, word_end);
        self.capture(2, at, end);
        Some(end)
    }

    /// `(?i)\b(as)\s+`.
    pub(crate) fn use_as(&mut self, p: usize) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        self.word_then_spaces(p, b"as")
    }

    /// `(array)\s*(\()`.
    pub(crate) fn array_paren(&mut self, p: usize) -> Option<usize> {
        let end = self.lit(p, b"array")?;
        let open = self.space_end(end);
        if self.at(open) != b'(' {
            return None;
        }
        self.capture(1, p, end);
        self.capture(2, open, open + 1);
        Some(open + 1)
    }
}
