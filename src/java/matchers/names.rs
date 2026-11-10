//! The names of enum constants, record constructors and `catch` and
//! `throws` types, and the ends of the rules that hold them.

use crate::java::engine::Lexer;
use crate::rules::Chars;

impl Lexer<'_> {
    /// `\b(\w+)\b`: the name of an enum constant.
    pub(super) fn enum_constant(&mut self, p: usize) -> Option<usize> {
        if !self.boundary(p) || self.word_at(p).is_none() {
            return None;
        }
        let end = self.word_end(p);
        self.capture(1, p, end);
        Some(end)
    }

    /// `(,)|(?=[;}])`.
    pub(super) fn enum_constant_end(&mut self, p: usize) -> Option<usize> {
        if p >= self.lim() {
            return None;
        }
        match self.s[p] {
            b',' => {
                self.capture(1, p, p + 1);
                Some(p + 1)
            }
            b';' | b'}' => Some(p),
            _ => None,
        }
    }

    /// `(})|(?=;)`.
    pub(super) fn method_end(&mut self, p: usize) -> Option<usize> {
        if p >= self.lim() {
            return None;
        }
        match self.s[p] {
            b'}' => {
                self.capture(1, p, p + 1);
                Some(p + 1)
            }
            b';' => Some(p),
            _ => None,
        }
    }

    /// `(\w+)`: the name of a record's constructor.
    pub(super) fn constructor_name(&mut self, p: usize) -> Option<usize> {
        let end = self.word_end(p);
        (end > p).then(|| {
            self.capture(1, p, end);
            end
        })
    }

    /// `[$A-Z_a-z][$.0-9A-Z_a-z]*`: a type of `catch` or `throws`.
    pub(super) fn dotted_name(&self, p: usize) -> Option<usize> {
        let first = *self.s.get(p)?;
        if !(first.is_ascii_alphabetic() || first == b'_' || first == b'$') {
            return None;
        }
        Some(self.skip(p + 1, |b| {
            b.is_ascii_alphanumeric() || matches!(b, b'_' | b'$' | b'.')
        }))
    }

    /// The begin `[$A-Z_a-z][$.0-9A-Z_a-z]*` of a `catch` parameter's type,
    /// which keeps its text as group 0.
    pub(super) fn catch_type(&self, p: usize) -> Option<usize> {
        self.dotted_name(p)
    }

    /// `(\|)|(?=\))`.
    pub(super) fn catch_separator(&mut self, p: usize) -> Option<usize> {
        if p >= self.lim() {
            return None;
        }
        match self.s[p] {
            b'|' => {
                self.capture(1, p, p + 1);
                Some(p + 1)
            }
            b')' => Some(p),
            _ => None,
        }
    }

    /// `[$A-Z_a-z][$.0-9A-Z_a-z]*`: a type in a `throws` clause.
    pub(super) fn throws_type(&self, p: usize) -> Option<usize> {
        self.dotted_name(p)
    }
}
