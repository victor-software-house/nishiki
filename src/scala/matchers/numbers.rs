//! Numbers.
//!
//! Each number ends with `\b`.
//! The digits, `_` and letters a number holds are all `\w`, so no position
//! inside a run of them has a `\b`.
//! A matcher therefore tries only the ends of its runs, in the order the
//! regular expression would backtrack through them.

use super::super::engine::Lexer;
use crate::rules::Chars;

/// Whether `b` is a type suffix, `[DFLdfl]`.
fn is_suffix(b: u8) -> bool {
    matches!(b, b'D' | b'F' | b'L' | b'd' | b'f' | b'l')
}

impl Lexer<'_> {
    /// The end of `[0-9][0-9_]*` at `i`.
    fn digits(&self, i: usize) -> Option<usize> {
        self.at(i)
            .is_ascii_digit()
            .then(|| self.skip(i + 1, |b| b.is_ascii_digit() || b == b'_'))
    }

    /// `[Ee]([-+])?[0-9][0-9_]*` at `i`.
    fn exponent(&self, i: usize) -> Option<usize> {
        if !matches!(self.at(i), b'E' | b'e') {
            return None;
        }
        let sign = i + 1 + usize::from(matches!(self.at(i + 1), b'-' | b'+'));
        self.digits(sign)
    }

    /// `[DFLdfl]?\b` after `i`: the suffix first, then none.
    fn suffix_and_boundary(&self, i: usize) -> Option<usize> {
        if is_suffix(self.at(i)) && self.boundary(i + 1) {
            return Some(i + 1);
        }
        self.boundary(i).then_some(i)
    }

    /// `(?:[Ee]([-+])?[0-9][0-9_]*)?[DFLdfl]?\b` after `i`: the exponent first,
    /// then none.
    fn exponent_tail(&self, i: usize) -> Option<usize> {
        self.exponent(i)
            .and_then(|end| self.suffix_and_boundary(end))
            .or_else(|| self.suffix_and_boundary(i))
    }

    /// `\b(([0-9][0-9_]*(\.[0-9][0-9_]*)?)([Ee]([-+])?[0-9][0-9_]*)?
    /// |[0-9][0-9_]*)[DFLdfl]?\b`.
    ///
    /// The second alternative is the first without its fraction and exponent,
    /// so the first decides.
    pub(super) fn decimal(&self, p: usize) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        let whole = self.digits(p)?;
        let fraction = (self.at(whole) == b'.')
            .then(|| self.digits(whole + 1))
            .flatten();
        fraction
            .and_then(|end| self.exponent_tail(end))
            .or_else(|| self.exponent_tail(whole))
    }

    /// `(\.[0-9][0-9_]*)([Ee]([-+])?[0-9][0-9_]*)?[DFLdfl]?\b`: no `\b` before.
    pub(super) fn point_decimal(&self, p: usize) -> Option<usize> {
        let fraction = self.lit(p, b".").and_then(|dot| self.digits(dot))?;
        self.exponent_tail(fraction)
    }

    /// `\b(0[Xx][_\h]*)\b`.
    ///
    /// `0`, `x` and every `[_\h]` are `\w`, so only the end of the run can
    /// hold the `\b`.
    pub(super) fn hex(&self, p: usize) -> Option<usize> {
        if !self.boundary(p) || self.at(p) != b'0' || !matches!(self.at(p + 1), b'X' | b'x') {
            return None;
        }
        let end = self.skip(p + 2, |b| b.is_ascii_hexdigit() || b == b'_');
        self.boundary(end).then_some(end)
    }

    /// `\b0[Bb][01]([01_]*[01])?[Ll]?\b`.
    ///
    /// The group ends at a `0` or `1`.
    /// Its greedy run backtracks to the last one, then to each earlier one,
    /// then to no group.
    pub(super) fn binary(&self, p: usize) -> Option<usize> {
        if !self.boundary(p) || self.at(p) != b'0' || !matches!(self.at(p + 1), b'B' | b'b') {
            return None;
        }
        let first = p + 2;
        if !matches!(self.at(first), b'0' | b'1') {
            return None;
        }
        let run = self.skip(first + 1, |b| matches!(b, b'0' | b'1' | b'_'));
        let mut end = run;
        while end > first + 1 {
            if matches!(self.at(end - 1), b'0' | b'1')
                && let Some(found) = self.long_suffix_and_boundary(end)
            {
                return Some(found);
            }
            end -= 1;
        }
        self.long_suffix_and_boundary(first + 1)
    }

    /// `[Ll]?\b` after `i`: the suffix first, then none.
    fn long_suffix_and_boundary(&self, i: usize) -> Option<usize> {
        if matches!(self.at(i), b'L' | b'l') && self.boundary(i + 1) {
            return Some(i + 1);
        }
        self.boundary(i).then_some(i)
    }
}
