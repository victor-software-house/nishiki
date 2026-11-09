//! The number patterns.
//!
//! Every run they take holds only ASCII word characters.
//! A run that gives a character back leaves a word character on both sides
//! of the `\b` that ends the pattern, so that `\b` fails.
//! Only the optional parts, and the pairs of an exponent sign, change where a
//! match can end.

use std::cell::Cell;

use super::{Lexer, is_word};
use crate::rules::Chars;

/// Whether `b` is in `[_\h]`.
fn is_hex_run(b: u8) -> bool {
    b == b'_' || b.is_ascii_hexdigit()
}

/// Whether `b` is in `[0-9_]`.
fn is_decimal_run(b: u8) -> bool {
    b == b'_' || b.is_ascii_digit()
}

/// Whether `b` is an exponent sign, `[-+]`.
fn is_sign(b: u8) -> bool {
    matches!(b, b'-' | b'+')
}

impl Lexer<'_> {
    /// `\b0x\h[_\h]*(\.\h[_\h]*)?([Pp][-+]?[_\h]+)?\b`.
    pub(super) fn hex_float(&self, p: usize) -> Option<usize> {
        if !self.boundary(p) || !self.starts(p, b"0x") || !self.at(p + 2).is_ascii_hexdigit() {
            return None;
        }
        let whole = self.skip(p + 3, is_hex_run);
        self.float_tail(
            whole,
            |b| b.is_ascii_hexdigit(),
            is_hex_run,
            |b| matches!(b, b'P' | b'p'),
        )
    }

    /// `\b[0-9][0-9_]*(\.[0-9][0-9_]*)?([Ee][-+]?[0-9_]+)?\b`.
    pub(super) fn float(&self, p: usize) -> Option<usize> {
        if !self.boundary(p) || !self.at(p).is_ascii_digit() {
            return None;
        }
        let whole = self.skip(p + 1, is_decimal_run);
        self.float_tail(
            whole,
            |b| b.is_ascii_digit(),
            is_decimal_run,
            |b| matches!(b, b'E' | b'e'),
        )
    }

    /// The optional fraction and exponent from `whole`, then the `\b`.
    ///
    /// Each optional part either takes its whole run or is left out.
    /// With a fraction, the ends tried are: fraction and exponent, fraction
    /// alone, then neither.
    /// Without the fraction a `.` follows `whole`, so no exponent can start
    /// there.
    fn float_tail(
        &self,
        whole: usize,
        digit: impl Fn(u8) -> bool,
        run: impl Fn(u8) -> bool + Copy,
        exponent: impl Fn(u8) -> bool,
    ) -> Option<usize> {
        let exponent_end = |i: usize| {
            if !exponent(self.at(i)) {
                return None;
            }
            let digits = if is_sign(self.at(i + 1)) {
                i + 2
            } else {
                i + 1
            };
            run(self.at(digits)).then(|| self.skip(digits, run))
        };
        let fraction = (self.at(whole) == b'.' && digit(self.at(whole + 1)))
            .then(|| self.skip(whole + 2, run));
        let ends = if let Some(end) = fraction {
            [exponent_end(end), Some(end), Some(whole)]
        } else {
            [exponent_end(whole), Some(whole), None]
        };
        ends.into_iter().flatten().find(|&end| self.boundary(end))
    }

    /// `\b[0-9][0-9_]*\b`.
    pub(super) fn decimal(&self, p: usize) -> Option<usize> {
        if !self.boundary(p) || !self.at(p).is_ascii_digit() {
            return None;
        }
        let end = self.skip(p + 1, is_decimal_run);
        self.boundary(end).then_some(end)
    }

    /// `\b0x[_\h]+\b`, `\b0o[0-7_]+\b` and `\b0b[01_]+\b`: a prefix, then a
    /// run of `digit` and `_`.
    pub(super) fn prefixed(
        &self,
        p: usize,
        prefix: &[u8],
        digit: impl Fn(u8) -> bool,
    ) -> Option<usize> {
        if !self.boundary(p) || !self.starts(p, prefix) {
            return None;
        }
        let from = p + prefix.len();
        let end = self.skip(from, |b| b == b'_' || digit(b));
        (end > from && self.boundary(end)).then_some(end)
    }

    /// An invalid number:
    ///
    /// ```text
    /// \b[0-9](([EPep][-+])|[0-9A-Z_a-z])*(\.(([EPep][-+])|[0-9A-Z_a-z])*)?([EPep][-+])?[0-9A-Z_a-z]*\b
    /// ```
    ///
    /// The two starred groups take units: a sign pair such as `e+`, tried
    /// first, or one word character.
    /// [`Lexer::units_back`] walks the ends of the first group in the order a
    /// backtracking matcher tries them.
    /// Only the longest run of the first group can end at a `.`, so the
    /// fraction is tried at most once.
    pub(super) fn invalid_number(&self, p: usize) -> Option<usize> {
        if !self.boundary(p) || !self.at(p).is_ascii_digit() {
            return None;
        }
        let run = Cell::new((usize::MAX, 0));
        self.units_back(p + 1, |end| {
            if self.at(end) == b'.' {
                let fraction = self.units_back(end + 1, |i| self.number_tail(i, &run));
                if fraction.is_some() {
                    return fraction;
                }
            }
            self.number_tail(end, &run)
        })
    }

    /// The first end of a run of units from `from` for which `then` matches,
    /// in the order a backtracking matcher tries them.
    ///
    /// The longest run comes first.
    /// Going back, a run that ended with a sign pair next tries that pair's
    /// letter alone, then stops before the pair.
    /// A run that ended with one character next stops before it.
    ///
    /// A sign enters a run only as the second byte of a pair.
    /// So the byte before an end shows which kind of unit ended there.
    fn units_back(&self, from: usize, then: impl Fn(usize) -> Option<usize>) -> Option<usize> {
        let mut end = from;
        loop {
            if matches!(self.at(end), b'E' | b'P' | b'e' | b'p') && is_sign(self.at(end + 1)) {
                end += 2;
            } else if is_word(self.at(end)) {
                end += 1;
            } else {
                break;
            }
        }
        self.spend(end - from);
        if let Some(found) = then(end) {
            return Some(found);
        }
        while end > from {
            let pair = is_sign(self.at(end - 1));
            if pair {
                if let Some(found) = then(end - 1) {
                    return Some(found);
                }
                end -= 2;
            } else {
                end -= 1;
            }
            if let Some(found) = then(end) {
                return Some(found);
            }
        }
        None
    }

    /// `([EPep][-+])?[0-9A-Z_a-z]*\b` from `i`.
    ///
    /// The run before `\b` gives back to where it started.
    /// Inside the run a word character stands on both sides, so only the
    /// run's end and its start can hold the `\b`.
    ///
    /// `run` keeps the last run measured, as its start and end.
    /// The ends tried go back a byte or two at a time, so a run is measured
    /// once, not once per end.
    fn number_tail(&self, i: usize, run: &Cell<(usize, usize)>) -> Option<usize> {
        let word_run = |from: usize| {
            let (start, known) = run.get();
            let end =
                if from <= start && start <= known && (from..start).all(|k| is_word(self.at(k))) {
                    known
                } else {
                    self.skip(from, is_word)
                };
            run.set((from, end));
            if self.boundary(end) {
                Some(end)
            } else {
                self.boundary(from).then_some(from)
            }
        };
        if matches!(self.at(i), b'E' | b'P' | b'e' | b'p')
            && is_sign(self.at(i + 1))
            && let Some(end) = word_run(i + 2)
        {
            return Some(end);
        }
        word_run(i)
    }
}
