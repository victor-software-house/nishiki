//! Operators and annotations.

use super::super::engine::Lexer;
use crate::rules::Chars;

impl Lexer<'_> {
    /// `(?<!\$)\b(is!?)\b(?!\$)`.
    ///
    /// The `!` is taken only when a word character follows it.
    /// The `\b` after it needs one.
    pub(super) fn is_operator(&self, p: usize) -> Option<usize> {
        if self.prev_is(p, b'$') || !self.boundary(p) {
            return None;
        }
        let end = self.lit(p, b"is")?;
        if self.at(end) == b'!' && self.word_at(end + 1).is_some() {
            return Some(end + 1);
        }
        (self.word_at(end).is_none() && self.at(end) != b'$').then_some(end)
    }

    /// `(<<|>>>?|[\&^|~])`.
    pub(super) fn bitwise(&self, p: usize) -> Option<usize> {
        match self.at(p) {
            b'<' => self.lit(p, b"<<"),
            b'>' => self.shift_right(p),
            b'&' | b'^' | b'|' | b'~' => Some(p + 1),
            _ => None,
        }
    }

    /// `>>>?`.
    fn shift_right(&self, p: usize) -> Option<usize> {
        let end = self.lit(p, b">>")?;
        Some(if self.at(end) == b'>' { end + 1 } else { end })
    }

    /// `(([\&^|]|<<|>>>?)=)`.
    pub(super) fn bitwise_assignment(&self, p: usize) -> Option<usize> {
        let operator = match self.at(p) {
            b'&' | b'^' | b'|' => p + 1,
            b'<' => self.lit(p, b"<<")?,
            b'>' => {
                let end = self.lit(p, b">>")?;
                if self.at(end) == b'>' && self.at(end + 1) == b'=' {
                    return Some(end + 2);
                }
                end
            }
            _ => return None,
        };
        self.lit(operator, b"=")
    }

    /// `(==|!=|<=?|>=?)`.
    pub(super) fn comparison(&self, p: usize) -> Option<usize> {
        match self.at(p) {
            b'=' | b'!' => self.lit(p + 1, b"="),
            b'<' | b'>' => Some(if self.at(p + 1) == b'=' { p + 2 } else { p + 1 }),
            _ => None,
        }
    }

    /// `(([-%*+/~])=)`.
    pub(super) fn arithmetic_assignment(&self, p: usize) -> Option<usize> {
        if !matches!(self.at(p), b'-' | b'%' | b'*' | b'+' | b'/' | b'~') {
            return None;
        }
        self.lit(p + 1, b"=")
    }

    /// `([-*+/]|~/|%)`.
    pub(super) fn arithmetic(&self, p: usize) -> Option<usize> {
        match self.at(p) {
            b'-' | b'*' | b'+' | b'/' | b'%' => Some(p + 1),
            b'~' => self.lit(p + 1, b"/"),
            _ => None,
        }
    }

    /// `(!|&&|\|\|)`.
    pub(super) fn logical(&self, p: usize) -> Option<usize> {
        match self.at(p) {
            b'!' => Some(p + 1),
            b'&' => self.lit(p, b"&&"),
            b'|' => self.lit(p, b"||"),
            _ => None,
        }
    }

    /// `@[A-Za-z]+`.
    pub(super) fn annotation(&self, p: usize) -> Option<usize> {
        if self.at(p) != b'@' {
            return None;
        }
        let end = self.skip(p + 1, |b| b.is_ascii_alphabetic());
        (end > p + 1).then_some(end)
    }
}
