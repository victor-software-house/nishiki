//! The matchers of the grammar's sixteen number patterns.
//!
//! Each is `(?i)`, so the base letters and the exponent letters match in
//! either case.
//! A run of digits is `[d][d_]*`: a digit of the base, then digits or `_`.
//! Giving back a character of such a run leaves a `\w` where a `\b` or a
//! letter must stand, so the runs are read greedily.

use super::super::engine::Lexer;
use crate::rules::Chars;

/// The base of a number pattern's digits.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::asm) enum Base {
    /// `[01]`, with the prefix `0b` or `0y`, or the suffix `b` or `y`.
    Binary,
    /// `[0-7]`, with the prefix `0o` or `0q`, or the suffix `o` or `q`.
    Octal,
    /// `\h`, with the prefix `0h` or `0x`, or the suffix `h` or `x`.
    Hex,
}

impl Base {
    /// Whether `b` is a digit of the base.
    fn digit(self, b: u8) -> bool {
        match self {
            Base::Binary => matches!(b, b'0' | b'1'),
            Base::Octal => matches!(b, b'0'..=b'7'),
            Base::Hex => b.is_ascii_hexdigit(),
        }
    }

    /// Whether `b` is one of the base's two letters, in either case.
    fn letter(self, b: u8) -> bool {
        let pair: &[u8] = match self {
            Base::Binary => b"by",
            Base::Octal => b"oq",
            Base::Hex => b"hx",
        };
        pair.contains(&b.to_ascii_lowercase())
    }
}

impl Lexer<'_> {
    /// `[d][d_]*` at `i` for the digits `digit`: where the run ends, or
    /// `None` when no digit starts it.
    fn digits(&self, i: usize, digit: impl Fn(u8) -> bool) -> Option<usize> {
        if !digit(self.at(i)) {
            return None;
        }
        Some(self.skip(i + 1, |b| digit(b) || b == b'_'))
    }

    /// `[0-9][0-9_]*` at `i`.
    fn decimal_digits(&self, i: usize) -> Option<usize> {
        self.digits(i, |b| b.is_ascii_digit())
    }

    /// `letter[-+]?[0-9][0-9_]*` at `i`, an exponent after the letter
    /// `letter`, lowercase.
    ///
    /// Without the sign, the digits would have to start at it.
    fn exponent(&self, i: usize, letter: u8) -> Option<usize> {
        if !self.at(i).eq_ignore_ascii_case(&letter) {
            return None;
        }
        let sign = i + 1 + usize::from(matches!(self.at(i + 1), b'+' | b'-'));
        self.decimal_digits(sign)
    }

    /// Whether the regex `\b` holds at `i`.
    fn edge(&self, i: usize) -> bool {
        self.boundary(i)
    }

    /// `(?:[d][d_]*)?(?:letter[-+]?[0-9][0-9_]*)?\b`, optional as a whole,
    /// after the `.` that ends at `dot`.
    ///
    /// The group first tries the fraction and the exponent, then the
    /// fraction alone, which needs a `\b` after it.
    /// Every other way through the group, and leaving it out, ends at `dot`.
    fn fraction(&self, dot: usize, digit: impl Fn(u8) -> bool, letter: u8) -> usize {
        let fraction = self.digits(dot, digit).unwrap_or(dot);
        if let Some(end) = self.exponent(fraction, letter)
            && self.edge(end)
        {
            return end;
        }
        if self.edge(fraction) {
            return fraction;
        }
        dot
    }

    /// `ConstantsAt0`, `ConstantsAt2` and `ConstantsAt7`:
    /// `(?i)\b0[letters][d][d_]*\.` and the optional fraction, with a `p`
    /// exponent.
    pub(super) fn prefixed_fraction(&self, p: usize, base: Base) -> Option<usize> {
        let dot = self.prefixed_run(p, base)?;
        let after = self.lit(dot, b".")?;
        Some(self.fraction(after, |b| base.digit(b), b'p'))
    }

    /// `ConstantsAt1`, `ConstantsAt3` and `ConstantsAt8`:
    /// `(?i)\b0[letters][d][d_]*p[-+]?[0-9][0-9_]*\b`.
    pub(super) fn prefixed_exponent(&self, p: usize, base: Base) -> Option<usize> {
        let run = self.prefixed_run(p, base)?;
        let end = self.exponent(run, b'p')?;
        self.edge(end).then_some(end)
    }

    /// `\b0[letters][d][d_]*` at `p`: where the digits end.
    fn prefixed_run(&self, p: usize, base: Base) -> Option<usize> {
        if !self.edge(p) || self.at(p) != b'0' || !base.letter(self.at(p + 1)) {
            return None;
        }
        self.digits(p + 2, |b| base.digit(b))
    }

    /// `ConstantsAt4`:
    /// `(?i)\b(?:0[dt])?[0-9][0-9_]*\.` and the optional fraction, with an
    /// `e` exponent.
    ///
    /// The optional `0d` or `0t` is tried first, then left out, which reads
    /// the `0` as the first digit.
    pub(super) fn decimal_fraction(&self, p: usize) -> Option<usize> {
        if !self.edge(p) {
            return None;
        }
        let starts = [
            (self.at(p) == b'0' && matches!(self.at(p + 1).to_ascii_lowercase(), b'd' | b't'))
                .then_some(p + 2),
            Some(p),
        ];
        let dot = starts.into_iter().flatten().find_map(|start| {
            self.decimal_digits(start)
                .filter(|&run| self.at(run) == b'.')
        })?;
        Some(self.fraction(dot + 1, |b| b.is_ascii_digit(), b'e'))
    }

    /// `ConstantsAt5`: `(?i)\b[0-9][0-9_]*e[-+]?[0-9][0-9_]*\b`.
    pub(super) fn decimal_exponent(&self, p: usize) -> Option<usize> {
        if !self.edge(p) {
            return None;
        }
        let run = self.decimal_digits(p)?;
        let end = self.exponent(run, b'e')?;
        self.edge(end).then_some(end)
    }

    /// `ConstantsAt6`: `(?i)\b[0-9][0-9_]*p(?:[0-9][0-9_]*)?\b`, packed BCD.
    ///
    /// Without the digits after the `p`, a digit would follow it, where no
    /// `\b` stands.
    pub(super) fn packed_bcd(&self, p: usize) -> Option<usize> {
        if !self.edge(p) {
            return None;
        }
        let run = self.decimal_digits(p)?;
        if !self.at(run).eq_ignore_ascii_case(&b'p') {
            return None;
        }
        let end = self.decimal_digits(run + 1).unwrap_or(run + 1);
        self.edge(end).then_some(end)
    }

    /// `\$[0-9]_?(?:\h[_\h]*)?` at `p`: where the hex digits end.
    ///
    /// The `_?` takes a `_` first.
    /// Left out, the `_` would stand where a hex digit, a `.` or a `p` must.
    fn dollar_run(&self, p: usize) -> Option<usize> {
        if self.at(p) != b'$' || !self.at(p + 1).is_ascii_digit() {
            return None;
        }
        let digit = p + 2 + usize::from(self.at(p + 2) == b'_');
        Some(
            self.digits(digit, |b| b.is_ascii_hexdigit())
                .unwrap_or(digit),
        )
    }

    /// `ConstantsAt9`: `(?i)\$[0-9]_?(?:\h[_\h]*)?\.` and the optional
    /// fraction, with a `p` exponent.
    pub(super) fn dollar_fraction(&self, p: usize) -> Option<usize> {
        let run = self.dollar_run(p)?;
        let after = self.lit(run, b".")?;
        Some(self.fraction(after, |b| b.is_ascii_hexdigit(), b'p'))
    }

    /// `ConstantsAt10`: `(?i)\$[0-9]_?\h[_\h]*p[-+]?[0-9][0-9_]*\b`.
    ///
    /// Here the hex digits are required.
    pub(super) fn dollar_exponent(&self, p: usize) -> Option<usize> {
        if self.at(p) != b'$' || !self.at(p + 1).is_ascii_digit() {
            return None;
        }
        let digit = p + 2 + usize::from(self.at(p + 2) == b'_');
        let run = self.digits(digit, |b| b.is_ascii_hexdigit())?;
        let end = self.exponent(run, b'p')?;
        self.edge(end).then_some(end)
    }

    /// `ConstantsAt14`: `(?i)\$[0-9]_?(?:\h[_\h]*)?\b`.
    pub(super) fn dollar_hex(&self, p: usize) -> Option<usize> {
        let run = self.dollar_run(p)?;
        self.edge(run).then_some(run)
    }

    /// `ConstantsAt11`, `ConstantsAt12` and `ConstantsAt15`:
    /// `(?i)\b(?:0[letters][d][d_]*|[d][d_]*[letters])\b`.
    ///
    /// The suffix form's run cannot take a base letter, so its letter is the
    /// character after the run.
    pub(super) fn integer(&self, p: usize, base: Base) -> Option<usize> {
        if let Some(run) = self.prefixed_run(p, base)
            && self.edge(run)
        {
            return Some(run);
        }
        if !self.edge(p) {
            return None;
        }
        let run = self.digits(p, |b| base.digit(b))?;
        (base.letter(self.at(run)) && self.edge(run + 1)).then_some(run + 1)
    }

    /// `ConstantsAt13`: `(?i)\b(?:0[dt][0-9][0-9_]*|[0-9][0-9_]*[dt]?)\b`.
    ///
    /// Without the optional `d` or `t`, that letter would stand after the
    /// run, where no `\b` stands.
    pub(super) fn decimal(&self, p: usize) -> Option<usize> {
        if !self.edge(p) {
            return None;
        }
        if self.at(p) == b'0'
            && matches!(self.at(p + 1).to_ascii_lowercase(), b'd' | b't')
            && let Some(run) = self.decimal_digits(p + 2)
            && self.edge(run)
        {
            return Some(run);
        }
        let run = self.decimal_digits(p)?;
        let end = run + usize::from(matches!(self.at(run).to_ascii_lowercase(), b'd' | b't'));
        self.edge(end).then_some(end)
    }
}
