//! Variables, operators and keywords.

use super::super::PhpFamily;
use crate::rules::{Chars, Lexer};

/// `(?i)[$_a-z]`, by character: `ſ` and the Kelvin sign fold to letters.
pub(crate) fn starts_word(c: char) -> bool {
    c == '$' || c == '_' || c.is_ascii_alphabetic() || c == 'ſ' || c == '\u{212a}'
}

#[cfg_attr(
    nishiki_unstable,
    expect(private_bounds, reason = "the bound and the methods are crate-private")
)]
impl<G: PhpFamily> Lexer<'_, G> {
    /// `\$\{(?=.*?})`.
    pub(crate) fn dollar_brace(&self, p: usize) -> Option<usize> {
        let open = self.lit(p, b"${")?;
        (open..self.lim())
            .any(|at| self.at(at) == b'}')
            .then_some(open)
    }

    /// `(?i)(::)\s*(class)\b`.
    pub(crate) fn class_constant_name(&mut self, p: usize) -> Option<usize> {
        let colons = self.lit(p, b"::")?;
        let at = self.space_end(colons);
        let end = self.ilit(at, b"class")?;
        if self.word_at(end).is_some() {
            return None;
        }
        self.capture(1, p, colons);
        self.capture(2, at, end);
        Some(end)
    }

    /// `(?i)(=)(&)|(&)(?=[$_a-z])`.
    pub(crate) fn reference(&mut self, p: usize) -> Option<usize> {
        if self.starts(p, b"=&") {
            self.capture(1, p, p + 1);
            self.capture(2, p + 1, p + 2);
            return Some(p + 2);
        }
        if self.at(p) != b'&' || !self.char_at(p + 1).is_some_and(|(c, _)| starts_word(c)) {
            return None;
        }
        self.capture(3, p, p + 1);
        Some(p + 1)
    }

    /// `===?|!==?|<>`.
    pub(crate) fn equals_or_differs(&self, p: usize) -> Option<usize> {
        if let Some(end) = self.lit(p, b"==").or_else(|| self.lit(p, b"!=")) {
            return Some(end + usize::from(self.at(end) == b'='));
        }
        self.lit(p, b"<>")
    }

    /// `<=>?|>=|[<>]`.
    pub(crate) fn relation(&self, p: usize) -> Option<usize> {
        if let Some(end) = self.lit(p, b"<=") {
            return Some(end + usize::from(self.at(end) == b'>'));
        }
        self.lit(p, b">=")
            .or_else(|| matches!(self.at(p), b'<' | b'>').then_some(p + 1))
    }

    /// `(?i)\b(instanceof)\s+(?=[$\\_a-z])`.
    pub(crate) fn instanceof(&mut self, p: usize) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        let end = self.ilit(p, b"instanceof")?;
        let at = self.space_end(end);
        let next = self
            .char_at(at)
            .is_some_and(|(c, _)| c == '\\' || starts_word(c));
        if at == end || !next {
            return None;
        }
        self.capture(1, p, end);
        Some(at)
    }

    /// `\bswitch\b(?!\s*\(.*\)\s*:)`: a `switch` whose condition is not
    /// followed by `:` on the line.
    pub(crate) fn switch(&self, p: usize) -> Option<usize> {
        let end = self.keyword(p, b"switch")?;
        let open = self.space_end(end);
        if self.at(open) == b'(' && open < self.lim() {
            let colon_after = (open + 1..self.lim())
                .filter(|&at| self.at(at) == b')')
                .any(|close| self.at(self.space_end(close + 1)) == b':');
            if colon_after {
                return None;
            }
        }
        Some(end)
    }
}
