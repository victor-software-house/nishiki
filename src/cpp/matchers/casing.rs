//! The naming styles of the unknown-variable expression.

use super::super::engine::Lexer;
use super::super::few::Few;
use crate::rules::Chars;

impl Lexer<'_> {
    /// The end of the run from `p` of bytes that `class` takes.
    fn byte_run(&self, p: usize, class: impl Fn(u8) -> bool) -> usize {
        let mut end = p;
        while end < self.lim() && class(self.s[end]) {
            end += 1;
        }
        self.spend(end - p + 1);
        end
    }

    /// `\b(?:(?:(?:([0-9a-z]+)|([0-9A-Za-z]+_[0-9A-Za-z]*
    /// ))|([a-z]+[A-Z][0-9A-Za-z]*))|([A-Z][0-9A-Z_]*))\b|((?<!\w)ID(?!\w))`.
    ///
    /// The styles are tried in order: lower case and digits; snake case,
    /// letters and digits with an underscore; camel case, lower case letters
    /// and then a capital; upper case.
    pub(super) fn unknown_variable(&mut self, p: usize) -> Option<usize> {
        if self.boundary(p) {
            let lower = |b: u8| b.is_ascii_lowercase() || b.is_ascii_digit();
            let alnum = |b: u8| b.is_ascii_alphanumeric();
            let end = self.byte_run(p, lower);
            if end > p && self.boundary(end) {
                self.cap(1, p, end);
                return Some(end);
            }
            let first = self.byte_run(p, alnum);
            if first > p && self.at(first) == b'_' && first < self.lim() {
                let end = self.byte_run(first + 1, alnum);
                if self.boundary(end) {
                    self.cap(2, p, end);
                    return Some(end);
                }
            }
            let low = self.byte_run(p, |b| b.is_ascii_lowercase());
            if low > p && self.at(low).is_ascii_uppercase() && low < self.lim() {
                let end = self.byte_run(low + 1, alnum);
                if self.boundary(end) {
                    self.cap(3, p, end);
                    return Some(end);
                }
            }
            if self.at(p).is_ascii_uppercase() && p < self.lim() {
                let end = self.byte_run(p + 1, |b| {
                    b.is_ascii_uppercase() || b.is_ascii_digit() || b == b'_'
                });
                if self.boundary(end) {
                    self.cap(4, p, end);
                    return Some(end);
                }
            }
        }
        let end = self.word_ident(p)?;
        self.cap(5, p, end);
        Some(end)
    }

    /// The ways the naming styles and the identifier of the unknown-variable
    /// expression match at `p`, in order: the group's number from 1, and
    /// where the name ends.
    pub(super) fn variable_names(&self, p: usize) -> Few<(usize, usize), 5> {
        let mut out = Few::new();
        if self.boundary(p) {
            let lower = |b: u8| b.is_ascii_lowercase() || b.is_ascii_digit();
            let alnum = |b: u8| b.is_ascii_alphanumeric();
            let end = self.byte_run(p, lower);
            if end > p && self.boundary(end) {
                out.push((1, end));
            }
            let first = self.byte_run(p, alnum);
            if first > p && self.at(first) == b'_' && first < self.lim() {
                let end = self.byte_run(first + 1, alnum);
                if self.boundary(end) {
                    out.push((2, end));
                }
            }
            let low = self.byte_run(p, |b| b.is_ascii_lowercase());
            if low > p && self.at(low).is_ascii_uppercase() && low < self.lim() {
                let end = self.byte_run(low + 1, alnum);
                if self.boundary(end) {
                    out.push((3, end));
                }
            }
            if self.at(p).is_ascii_uppercase() && p < self.lim() {
                let end = self.byte_run(p + 1, |b| {
                    b.is_ascii_uppercase() || b.is_ascii_digit() || b == b'_'
                });
                if self.boundary(end) {
                    out.push((4, end));
                }
            }
        }
        if let Some(end) = self.word_ident(p) {
            out.push((5, end));
        }
        out
    }
}
