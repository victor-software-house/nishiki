//! Numbers.
//!
//! Each number pattern is `(?<!\w)([-+]?…)((?i:[gkmpt]b)?)\b`.
//! Group 1 is the number and group 2 its multiplier, such as `kb`.
//! A digit run in a number gives back no characters.
//! A digit follows any shorter run, and no part after a run starts with one,
//! so `\b` and the parts after it fail there.

use super::super::engine::Lexer;
use crate::rules::Chars;

/// The two-letter type suffixes, in the grammar's order.
const SUFFIXES: &[&[u8]] = &[b"UL", b"Ul", b"uL", b"ul", b"LU", b"Lu", b"lU", b"lu"];

/// Whether a number pattern has a dot before its exponent.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Dot {
    /// It has none: `[0-9_]+[Ee]`.
    Never,
    /// It has one: `[0-9_]+\.[Ee]`.
    Needed,
}

/// Whether `b` is in `[0-9_]`.
fn is_digit_or_underscore(b: u8) -> bool {
    b == b'_' || b.is_ascii_digit()
}

/// Whether `b` is in `[DFMdfm]`, the real type suffixes.
fn is_real_suffix(b: u8) -> bool {
    matches!(b, b'D' | b'F' | b'M' | b'd' | b'f' | b'm')
}

impl Lexer<'_> {
    /// Where a number's digits start: after `(?<!\w)` and an optional sign at
    /// `p`.
    fn number_start(&self, p: usize) -> Option<usize> {
        if self.word_before(p) {
            return None;
        }
        Some(if matches!(self.at(p), b'-' | b'+') {
            p + 1
        } else {
            p
        })
    }

    /// `((?i:[gkmpt]b)?)\b` after a number that ends at `number`: the end of
    /// the match, with groups 1 and 2 recorded.
    fn multiplier(&mut self, p: usize, number: usize) -> Option<usize> {
        let first = self.char_at(number).filter(|&(c, _)| {
            matches!(c.to_ascii_lowercase(), 'g' | 'k' | 'm' | 'p' | 't') || c == '\u{212a}'
        });
        if let Some((_, b)) = first
            && matches!(self.at(b), b'b' | b'B')
            && self.boundary(b + 1)
        {
            self.capture(1, p, number);
            self.capture(2, number, b + 1);
            return Some(b + 1);
        }
        if !self.boundary(number) {
            return None;
        }
        self.capture(1, p, number);
        self.capture(2, number, number);
        Some(number)
    }

    /// `[DFMdfm]?((?i:[gkmpt]b)?)\b` after a number that ends at `number`.
    ///
    /// The suffix is tried taken first, then left out.
    /// `m` is a suffix and starts the multiplier `mb`, so `1.5mb` leaves it
    /// to the multiplier.
    fn real_suffix(&mut self, p: usize, number: usize) -> Option<usize> {
        if is_real_suffix(self.at(number))
            && let Some(end) = self.multiplier(p, number + 1)
        {
            return Some(end);
        }
        self.multiplier(p, number)
    }

    /// `(?:[LUlu]|UL|Ul|uL|ul|LU|Lu|lU|lu)?((?i:[gkmpt]b)?)\b` after digits
    /// that end at `digits`.
    /// The one-letter suffix is tried first, then the two-letter ones, then
    /// none.
    fn integer_suffix(&mut self, p: usize, digits: usize) -> Option<usize> {
        if matches!(self.at(digits), b'L' | b'U' | b'l' | b'u')
            && let Some(end) = self.multiplier(p, digits + 1)
        {
            return Some(end);
        }
        if SUFFIXES.iter().any(|suffix| self.starts(digits, suffix))
            && let Some(end) = self.multiplier(p, digits + 2)
        {
            return Some(end);
        }
        self.multiplier(p, digits)
    }

    /// `(?<!\w)([-+]?0[Xx][_\h]+(?:[LUlu]|…)?)((?i:[gkmpt]b)?)\b`: a hex
    /// number.
    pub(super) fn hex(&mut self, p: usize) -> Option<usize> {
        let zero = self.number_start(p)?;
        if self.at(zero) != b'0' || !matches!(self.at(zero + 1), b'X' | b'x') {
            return None;
        }
        let digits = self.skip(zero + 2, |b| b == b'_' || b.is_ascii_hexdigit());
        if digits == zero + 2 {
            return None;
        }
        self.integer_suffix(p, digits)
    }

    /// `(?<!\w)([-+]?0[Bb][01_]+(?:[LUlu]|…)?)((?i:[gkmpt]b)?)\b`: a binary
    /// number.
    pub(super) fn binary(&mut self, p: usize) -> Option<usize> {
        let zero = self.number_start(p)?;
        if self.at(zero) != b'0' || !matches!(self.at(zero + 1), b'B' | b'b') {
            return None;
        }
        let digits = self.skip(zero + 2, |b| matches!(b, b'0' | b'1' | b'_'));
        if digits == zero + 2 {
            return None;
        }
        self.integer_suffix(p, digits)
    }

    /// `(?<!\w)([-+]?[0-9_]+{0,1}\.[0-9_]+(?:[Ee][0-9]+)?[DFMdfm]?)((?i:
    /// [gkmpt]b)?)\b`: a real number with a fraction.
    pub(super) fn real(&mut self, p: usize) -> Option<usize> {
        let start = self.number_start(p)?;
        let dot = self.skip(start, is_digit_or_underscore);
        if self.at(dot) != b'.' {
            return None;
        }
        let mut end = self.skip(dot + 1, is_digit_or_underscore);
        if end == dot + 1 {
            return None;
        }
        if matches!(self.at(end), b'E' | b'e') && self.at(end + 1).is_ascii_digit() {
            end = self.skip(end + 1, |b| b.is_ascii_digit());
        }
        self.real_suffix(p, end)
    }

    /// `(?<!\w)([-+]?[0-9_]+[Ee][0-9_]?+[DFMdfm]?)((?i:[gkmpt]b)?)\b` when
    /// `dot` is never, and the same with `\.` before `[Ee]` when it is needed:
    /// a number with an exponent.
    ///
    /// The `[0-9_]?+` is possessive: it keeps the one digit it takes.
    pub(super) fn exponent(&mut self, p: usize, dot: Dot) -> Option<usize> {
        let start = self.number_start(p)?;
        let mut end = self.skip(start, is_digit_or_underscore);
        if end == start {
            return None;
        }
        if dot == Dot::Needed {
            end = self.lit(end, b".")?;
        }
        if !matches!(self.at(end), b'E' | b'e') {
            return None;
        }
        end += 1;
        if is_digit_or_underscore(self.at(end)) {
            end += 1;
        }
        self.real_suffix(p, end)
    }

    /// `(?<!\w)([-+]?[0-9_]+\.?[DFMdfm])((?i:[gkmpt]b)?)\b`: an integer with
    /// a real type suffix.
    pub(super) fn typed_integer(&mut self, p: usize) -> Option<usize> {
        let start = self.number_start(p)?;
        let mut end = self.skip(start, is_digit_or_underscore);
        if end == start {
            return None;
        }
        if self.at(end) == b'.' && is_real_suffix(self.at(end + 1)) {
            end += 1;
        }
        if !is_real_suffix(self.at(end)) {
            return None;
        }
        self.multiplier(p, end + 1)
    }

    /// `(?<!\w)([-+]?[0-9_]+\.?(?:[LUlu]|…)?)((?i:[gkmpt]b)?)\b`: an integer.
    ///
    /// The dot is tried taken first, then left out, as in `1.x`, which is the
    /// number `1.`, and `1..2`, where `1` is.
    pub(super) fn integer(&mut self, p: usize) -> Option<usize> {
        let start = self.number_start(p)?;
        let digits = self.skip(start, is_digit_or_underscore);
        if digits == start {
            return None;
        }
        if self.at(digits) == b'.'
            && let Some(end) = self.integer_suffix(p, digits + 1)
        {
            return Some(end);
        }
        self.integer_suffix(p, digits)
    }
}
