//! Values: numbers, units, operators, colours, functions and modules.

use super::{Lexer, is_ident};
use crate::rules::Chars;

/// The units, in the grammar's order.
/// The first that the text starts with wins, with no word boundary after it.
const UNITS: &[&[u8]] = &[
    b"ch", b"cm", b"deg", b"dpcm", b"dpi", b"dppx", b"em", b"ex", b"grad", b"Hz", b"in", b"kHz",
    b"mm", b"ms", b"pc", b"pt", b"px", b"rad", b"rem", b"s", b"turn", b"vh", b"vmax", b"vmin",
    b"vw", b"fr", b"%",
];

impl Lexer<'_> {
    /// `([-.])?[0-9]+(\.[0-9]+)?`.
    /// A sign or dot with no digit after it gives itself back, and then no
    /// digit stands at `p`.
    pub(super) fn numeric(&self, p: usize) -> Option<usize> {
        let digits = if matches!(self.at(p), b'-' | b'.') {
            p + 1
        } else {
            p
        };
        if !self.at(digits).is_ascii_digit() {
            return None;
        }
        let whole = self.skip(digits, |b| b.is_ascii_digit());
        if self.at(whole) == b'.' && self.at(whole + 1).is_ascii_digit() {
            return Some(self.skip(whole + 1, |b| b.is_ascii_digit()));
        }
        Some(whole)
    }

    /// `\+|\s-\s|\s-(?=\$)|(?<=\()-(?=\$)|\s-(?=\()|[!%*/<=>~]`.
    pub(super) fn operator(&self, p: usize) -> Option<usize> {
        match self.at(p) {
            b'+' | b'!' | b'%' | b'*' | b'/' | b'<' | b'=' | b'>' | b'~' => return Some(p + 1),
            b'-' => {
                return (self.prev(p) == Some(b'(') && self.at(p + 1) == b'$').then_some(p + 1);
            }
            _ => {}
        }
        let blank = self.space_at(p);
        let dash = p + blank;
        if blank == 0 || self.at(dash) != b'-' {
            return None;
        }
        let after = self.space_at(dash + 1);
        if after > 0 {
            return Some(dash + 1 + after);
        }
        matches!(self.at(dash + 1), b'$' | b'(').then_some(dash + 1)
    }

    /// `(?<=[}\d])(ch|cm|...|fr|%)`: a unit after a `}` or a digit.
    pub(super) fn unit(&self, p: usize) -> Option<usize> {
        if self.prev(p) != Some(b'}') && !self.digit_before(p) {
            return None;
        }
        UNITS.iter().find_map(|unit| self.lit(p, unit))
    }

    /// `(#)(\h{3,4}|\h{6}|\h{8})\b`.
    ///
    /// The branches end after four, three, six or eight hex digits, tried in
    /// that order.
    /// The first end before a word boundary wins.
    pub(super) fn rgb_value(&self, p: usize) -> Option<usize> {
        let from = self.lit(p, b"#")?;
        let hex = self.skip(from, |b| b.is_ascii_hexdigit()) - from;
        [4, 3, 6, 8]
            .into_iter()
            .filter(|&count| count <= hex)
            .map(|count| from + count)
            .find(|&end| self.boundary(end))
    }

    /// `(?<=[(,:|\s])(?!url|format|attr)[-0-9A-Z_a-z][-\w]*(?=\()`.
    /// Giving back the run leaves a `[-\w]` character where the `(` must
    /// stand.
    pub(super) fn function_name(&self, p: usize) -> Option<usize> {
        let after = matches!(self.prev(p), Some(b'(' | b',' | b':' | b'|')) || self.space_before(p);
        if !after
            || [b"url".as_slice(), b"format", b"attr"]
                .iter()
                .any(|word| self.starts(p, word))
        {
            return None;
        }
        if !is_ident(self.at(p)) {
            return None;
        }
        let end = self.dash_word_end(p + 1);
        (self.at(end) == b'(').then_some(end)
    }

    /// `([-\w]+?)(\.)`: a module name and its dot.
    /// `.` is no `[-\w]`, so the lazy run stops at the first dot after it.
    pub(super) fn module(&mut self, p: usize) -> Option<usize> {
        let end = self.dash_word_end(p);
        if end == p || self.at(end) != b'.' {
            return None;
        }
        self.cap(1, p, end);
        self.cap(2, end, end + 1);
        Some(end + 1)
    }
}
