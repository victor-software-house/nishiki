//! Operators.

use super::super::engine::Lexer;
use crate::rules::Chars;

impl Lexer<'_> {
    /// `(===?|!==?|<=|>=|[<>])`.
    pub(super) fn comparison(&self, p: usize) -> Option<usize> {
        let equals = |end: usize| Some(if self.at(end) == b'=' { end + 1 } else { end });
        if let Some(end) = self.lit(p, b"==").or_else(|| self.lit(p, b"!=")) {
            return equals(end);
        }
        self.lit(p, b"<=")
            .or_else(|| self.lit(p, b">="))
            .or_else(|| matches!(self.at(p), b'<' | b'>').then_some(p + 1))
    }

    /// `([-%*+/]=)`.
    pub(super) fn arithmetic_assignment(&self, p: usize) -> Option<usize> {
        let operator = self.arithmetic(p)?;
        self.lit(operator, b"=")
    }

    /// `([-%*+/])`.
    pub(super) fn arithmetic(&self, p: usize) -> Option<usize> {
        matches!(self.at(p), b'-' | b'%' | b'*' | b'+' | b'/').then_some(p + 1)
    }

    /// `(!|&&|\|\|)`.
    pub(super) fn logical(&self, p: usize) -> Option<usize> {
        self.lit(p, b"!")
            .or_else(|| self.lit(p, b"&&"))
            .or_else(|| self.lit(p, b"||"))
    }

    /// `(--|\+\+)`.
    pub(super) fn increment(&self, p: usize) -> Option<usize> {
        self.lit(p, b"--").or_else(|| self.lit(p, b"++"))
    }
}
