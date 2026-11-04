//! Numbers.

use super::super::engine::Lexer;
use crate::rules::Chars;

/// Whether `b` is in `[0-9_]`.
fn is_digit_or_underscore(b: u8) -> bool {
    b == b'_' || b.is_ascii_digit()
}

impl Lexer<'_> {
    /// The grammar's number, between `(?<!\$)\b` and `\b(?!\$)`:
    /// - `0([Xx])\h[_\h]*`;
    /// - or `[0-9][0-9_]*\.?[0-9_]*` or `\.[0-9][0-9_]*`, with an optional
    ///   exponent, `([Ee])([-+])?[0-9][0-9_]*`.
    ///
    /// Every run the pattern repeats is of word characters.
    /// `\b` cannot hold inside one, so only the end of each run is a candidate.
    /// The candidates are tried in the order the backtracking matcher tries
    /// them.
    pub(super) fn number(&self, p: usize) -> Option<usize> {
        if self.prev_is(p, b'$') || !self.boundary(p) {
            return None;
        }
        if self.at(p) == b'0'
            && matches!(self.at(p + 1), b'X' | b'x')
            && self.at(p + 2).is_ascii_hexdigit()
        {
            let end = self.skip(p + 3, |b| b == b'_' || b.is_ascii_hexdigit());
            if self.number_ends(end) {
                return Some(end);
            }
        }
        if self.at(p).is_ascii_digit() {
            return self.decimal(p);
        }
        if self.at(p) == b'.' && self.at(p + 1).is_ascii_digit() {
            let end = self.skip(p + 2, is_digit_or_underscore);
            return self
                .exponent(end)
                .or_else(|| self.number_ends(end).then_some(end));
        }
        None
    }

    /// `[0-9][0-9_]*\.?[0-9_]*` with the optional exponent and the `\b(?!\$)`
    /// after it, from the digit at `p`.
    ///
    /// With the `.` taken, the fraction is tried in full, then empty.
    /// Each with the exponent first.
    /// Then the integer part is tried without the `.`.
    fn decimal(&self, p: usize) -> Option<usize> {
        let int = self.skip(p + 1, is_digit_or_underscore);
        if self.at(int) == b'.' {
            let fraction = self.skip(int + 1, is_digit_or_underscore);
            if let Some(end) = self.exponent(fraction) {
                return Some(end);
            }
            if self.number_ends(fraction) {
                return Some(fraction);
            }
            if fraction > int + 1 && self.number_ends(int + 1) {
                return Some(int + 1);
            }
        }
        self.exponent(int)
            .or_else(|| self.number_ends(int).then_some(int))
    }

    /// `([Ee])([-+])?[0-9][0-9_]*` at `i` with the `\b(?!\$)` after it.
    fn exponent(&self, i: usize) -> Option<usize> {
        if !matches!(self.at(i), b'E' | b'e') {
            return None;
        }
        let digit = if matches!(self.at(i + 1), b'-' | b'+') {
            i + 2
        } else {
            i + 1
        };
        if !self.at(digit).is_ascii_digit() {
            return None;
        }
        let end = self.skip(digit + 1, is_digit_or_underscore);
        self.number_ends(end).then_some(end)
    }

    /// `\b(?!\$)` at `i`.
    fn number_ends(&self, i: usize) -> bool {
        self.boundary(i) && self.at(i) != b'$'
    }
}
