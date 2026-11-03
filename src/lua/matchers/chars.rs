//! The character classes and small assertions the matchers share.

use super::super::engine::Lexer;
use crate::rules::Chars;

/// `[A-Z_a-z]`
pub(super) fn name_start(b: u8) -> bool {
    b.is_ascii_alphabetic() || b == b'_'
}

/// `[0-9A-Z_a-z]`
pub(super) fn name_part(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

/// `[*A-Z_a-z]`, the first character of a documented type's name.
pub(super) fn type_start(b: u8) -> bool {
    b == b'*' || name_start(b)
}

/// `[-*.0-9A-Z_a-z]`, the rest of a documented type's name.
pub(super) fn type_part(b: u8) -> bool {
    matches!(b, b'-' | b'*' | b'.') || name_part(b)
}

/// `[-0-9A-Z_a-z]`, a diagnostic's name.
pub(super) fn diagnostic(b: u8) -> bool {
    b == b'-' || name_part(b)
}

impl Lexer<'_> {
    /// `[\t ]*` from `p`: the end of the run.
    pub(super) fn tabs(&self, p: usize) -> usize {
        self.skip(p, |b| matches!(b, b'\t' | b' '))
    }

    /// `[A-Z_a-z][0-9A-Z_a-z]*` at `p`: the end of the name.
    pub(super) fn ascii_name(&self, p: usize) -> Option<usize> {
        (p < self.lim() && name_start(self.at(p))).then(|| self.skip(p + 1, name_part))
    }

    /// `\b[A-Z_a-z][0-9A-Z_a-z]*\b`: a name with nothing of a word around it.
    /// The name's run is the longest, and a shorter one would leave a name
    /// character next, which `\b` refuses.
    pub(super) fn bounded_name(&self, p: usize) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        self.ascii_name(p)
            .filter(|&end| self.word_at(end).is_none())
    }

    /// `\b(start part*)`: a run whose first byte is of `start` and the rest
    /// of `part`, after a word boundary, as capture group 1.
    pub(super) fn bounded_run(
        &mut self,
        p: usize,
        start: fn(u8) -> bool,
        part: fn(u8) -> bool,
    ) -> Option<usize> {
        if !self.boundary(p) || p >= self.lim() || !start(self.at(p)) {
            return None;
        }
        let end = self.skip(p + 1, part);
        self.cap(1, p, end);
        Some(end)
    }

    /// The regex `(?=[set])` at `p`, where the line feed after the line is a
    /// character of the text.
    pub(super) fn ahead_in(&self, p: usize, set: &[u8]) -> bool {
        self.avail(p) && set.contains(&self.at(p))
    }

    /// One byte of `set` at `p`: the end of it.
    pub(super) fn byte_of(&self, p: usize, set: &[u8]) -> Option<usize> {
        (p < self.lim() && set.contains(&self.at(p))).then_some(p + 1)
    }

    /// `\n`, the line feed after the line: the end of it.
    pub(super) fn line_feed(&self, p: usize) -> Option<usize> {
        (self.avail(p) && self.at(p) == b'\n').then_some(p + 1)
    }

    /// `(?<![^.]\.|:)`: no member access or method call stands before `p`.
    pub(super) fn not_member(&self, p: usize) -> bool {
        if self.prev_is(p, b':') {
            return false;
        }
        !(self.prev_is(p, b'.') && self.char_before(p - 1).is_some_and(|c| c != '.'))
    }

    /// `(?<!\.)` at `p` and `(?!\.)` at `end`: the dots `p..end` are a whole
    /// run of dots, not part of a longer one.
    pub(super) fn whole_dots(&self, p: usize, end: usize) -> bool {
        if self.prev_is(p, b'.') {
            return false;
        }
        !self.avail(end) || self.at(end) != b'.'
    }
}
