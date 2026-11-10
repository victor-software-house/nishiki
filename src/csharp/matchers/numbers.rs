//! Numeric literals.
//! The grammar finds the text of a number with one expression.
//! It then lexes that text again with five, tried in order.
//! Each must match all of it (`$`).
//!
//! The integer, fraction and exponent parts of the first expression repeat
//! and are optional.
//! Oniguruma therefore backtracks through them.
//! The matchers try the same ways in the same order.

use super::super::engine::Lexer;
use crate::classes::is_digit;
use crate::rules::Chars;

/// The spellings of a number's suffix in the hexadecimal, binary and
/// integer patterns.
const INTEGER_SUFFIXES: &[&[u8]] = &[
    b"u", b"U", b"ul", b"uL", b"Ul", b"UL", b"l", b"lu", b"lU", b"L", b"Lu", b"LU", b"d", b"D",
    b"f", b"F", b"m", b"M",
];

/// The spellings of a suffix in the first pattern.
const REAL_SUFFIXES: &[&[u8]] = &[b"d", b"D", b"f", b"F", b"m", b"M"];

impl Lexer<'_> {
    /// `(?<=\h)_(?=\h)` at `i`: an underscore between two hexadecimal digits.
    fn separator_at(&self, i: usize) -> bool {
        self.at(i) == b'_'
            && i < self.lim()
            && self.prev(i).is_some_and(|b| b.is_ascii_hexdigit())
            && self.at(i + 1).is_ascii_hexdigit()
            && i + 1 < self.lim()
    }

    /// `[0-9](?:[0-9]|(?<=\h)_(?=\h))*` from `p`: the end of the run, or `p`
    /// when no digit stands there.
    /// Every position after the first digit is a place where the run can end.
    /// A caller that must give characters back therefore does it one byte at a
    /// time.
    fn decimal_run(&self, p: usize) -> usize {
        if !self.at(p).is_ascii_digit() || p >= self.lim() {
            return p;
        }
        let mut i = p + 1;
        while i < self.lim() && (self.s[i].is_ascii_digit() || self.separator_at(i)) {
            i += 1;
        }
        self.spend(i - p);
        i
    }

    /// The last separator of `from..to`, recorded as capture group `group`.
    fn last_separator(&mut self, group: usize, from: usize, to: usize) {
        if let Some(i) = (from..to).rev().find(|&i| self.s[i] == b'_') {
            self.capture(group, i, i + 1);
        }
    }

    /// `(?=[.0-9])(?!0[BXbx])` at `p`, with `\G`.
    fn number_start(&self, p: usize) -> bool {
        let b = self.at(p);
        self.at_g(p)
            && p < self.lim()
            && (b == b'.' || b.is_ascii_digit())
            && !(b == b'0' && matches!(self.at(p + 1), b'B' | b'X' | b'b' | b'x'))
    }

    /// Whether the text from `p` to the end is one of `spellings`.
    fn suffix_to_end(&self, p: usize, spellings: &[&[u8]]) -> bool {
        spellings.contains(&self.text(p, self.lim()))
    }

    /// `(suffix)?$` after the digits at `p`: the end, with the suffix as
    /// capture group `group` when there is one.
    fn suffix_then_end(&mut self, p: usize, group: usize, spellings: &[&[u8]]) -> Option<usize> {
        if self.at_eol(p) {
            return Some(p);
        }
        if self.suffix_to_end(p, spellings) {
            self.capture(group, p, self.lim());
            return Some(self.lim());
        }
        None
    }

    /// `((?<!_)([Ee])(\+?)(-?)([0-9](?:[0-9]|(?<=\h)_(?=\h))*))?`: the end of
    /// the exponent at `p` and its capture groups numbered from `base`,
    /// when one is there.
    fn exponent(&mut self, p: usize, base: usize) -> Option<usize> {
        if self.prev(p) == Some(b'_') || !matches!(self.at(p), b'E' | b'e') || p >= self.lim() {
            return None;
        }
        let mut q = p + 1;
        let plus = (self.at(q) == b'+' && q < self.lim()).then_some(q);
        if plus.is_some() {
            q += 1;
        }
        let minus = (self.at(q) == b'-' && q < self.lim()).then_some(q);
        if minus.is_some() {
            q += 1;
        }
        let end = self.decimal_run(q);
        if end == q {
            return None;
        }
        self.capture(base, p, p + 1);
        if let Some(at) = plus {
            self.capture(base + 1, at, at + 1);
        }
        if let Some(at) = minus {
            self.capture(base + 2, at, at + 1);
        }
        self.capture(base + 3, q, end);
        Some(end)
    }

    /// The first pattern of a number: integer, fraction and exponent parts,
    /// and a `d`, `f` or `m` suffix.
    ///
    /// Oniguruma gives the integer and fraction runs back one byte at a time
    /// when the rest fails.
    /// A byte given back is a digit or a separator.
    /// Neither the point, the exponent, the suffix nor `$` matches there.
    /// Only the longest runs can therefore succeed, and the matcher tries
    /// those alone.
    ///
    /// The point is `((?<=[0-9])|\.(?=[0-9]))`: an empty match after a digit,
    /// or a dot before one.
    pub(in crate::csharp) fn real_number(&mut self, p: usize) -> Option<usize> {
        if !self.number_start(p) {
            return None;
        }
        let int_end = self.decimal_run(p);
        if self.prev(int_end).is_some_and(|b| b.is_ascii_digit())
            && let Some(end) = self.fraction_and_tail(p, int_end, int_end, int_end)
        {
            return Some(end);
        }
        let point = self.at(int_end) == b'.'
            && int_end < self.lim()
            && self.at(int_end + 1).is_ascii_digit()
            && int_end + 1 < self.lim();
        if point {
            return self.fraction_and_tail(p, int_end, int_end, int_end + 1);
        }
        None
    }

    /// The fraction, exponent and suffix after the point at `point_at`, whose
    /// fraction starts at `frac`, with the integer part `p..int_end`.
    fn fraction_and_tail(
        &mut self,
        p: usize,
        int_end: usize,
        point_at: usize,
        frac: usize,
    ) -> Option<usize> {
        let frac_end = self.decimal_run(frac);
        let end = self.real_tail(frac_end)?;
        if int_end > p {
            self.capture(2, p, int_end);
            self.last_separator(3, p, int_end);
        }
        self.capture(4, point_at, frac);
        if frac_end > frac {
            self.capture(5, frac, frac_end);
            self.last_separator(6, frac, frac_end);
        }
        Some(end)
    }

    /// `((?<!_)([Ee])(\+?)(-?)(...))?([DFMdfm](?!\w))?$` at `p`.
    fn real_tail(&mut self, p: usize) -> Option<usize> {
        if let Some(after) = self.exponent(p, 8)
            && let Some(end) = self.suffix_then_end(after, 12, REAL_SUFFIXES)
        {
            return Some(end);
        }
        self.suffix_then_end(p, 12, REAL_SUFFIXES)
    }

    /// `\G(0[Bb])([01_](?:[01_]|...)*)(suffix)?$`
    pub(in crate::csharp) fn binary_number(&mut self, p: usize) -> Option<usize> {
        if !self.at_g(p) || self.at(p) != b'0' || !matches!(self.at(p + 1), b'B' | b'b') {
            return None;
        }
        let digits = self.skip(p + 2, |b| matches!(b, b'0' | b'1' | b'_'));
        if digits == p + 2 {
            return None;
        }
        let end = self.suffix_then_end(digits, 4, INTEGER_SUFFIXES)?;
        self.capture(1, p, p + 2);
        self.capture(2, p + 2, digits);
        Some(end)
    }

    /// `\G(0[Xx])(\h(?:\h|((?<=\h)_(?=\h)))*)(suffix)?$`
    pub(in crate::csharp) fn hex_number(&mut self, p: usize) -> Option<usize> {
        if !self.at_g(p) || self.at(p) != b'0' || !matches!(self.at(p + 1), b'X' | b'x') {
            return None;
        }
        if !self.at(p + 2).is_ascii_hexdigit() || p + 2 >= self.lim() {
            return None;
        }
        let mut i = p + 3;
        while i < self.lim() && (self.s[i].is_ascii_hexdigit() || self.separator_at(i)) {
            i += 1;
        }
        self.spend(i - p);
        let end = self.suffix_then_end(i, 4, INTEGER_SUFFIXES)?;
        self.capture(1, p, p + 2);
        self.capture(2, p + 2, i);
        self.last_separator(3, p + 2, i);
        Some(end)
    }

    /// `\G((?=[.0-9])(?!0[BXbx]))([0-9](?:[0-9]|...)*)(exponent)?(suffix)?$`
    pub(in crate::csharp) fn integer_number(&mut self, p: usize) -> Option<usize> {
        if !self.number_start(p) {
            return None;
        }
        let int_end = self.decimal_run(p);
        if int_end == p {
            return None;
        }
        let tail = |lexer: &mut Self| -> Option<usize> {
            if let Some(after) = lexer.exponent(int_end, 5)
                && let Some(end) = lexer.suffix_then_end(after, 9, INTEGER_SUFFIXES)
            {
                return Some(end);
            }
            lexer.suffix_then_end(int_end, 9, INTEGER_SUFFIXES)
        };
        let end = tail(self)?;
        self.capture(2, p, int_end);
        self.last_separator(3, p, int_end);
        Some(end)
    }

    /// `(?:[0-9A-Z_a-z]|(?<=[Ee])[-+]|\.\d)+` from `p`: the end of the run,
    /// which may be empty.
    fn number_word_run(&self, mut i: usize) -> usize {
        let from = i;
        while i < self.lim() {
            let b = self.s[i];
            if b.is_ascii_alphanumeric()
                || b == b'_'
                || (matches!(b, b'+' | b'-') && matches!(self.prev(i), Some(b'E' | b'e')))
            {
                i += 1;
            } else if b == b'.' {
                match self.cp(i + 1) {
                    Some((c, end)) if is_digit(c) => i = end,
                    _ => break,
                }
            } else {
                break;
            }
        }
        self.spend(i - from);
        i
    }

    /// The last pattern: whatever else is made of the number's characters.
    pub(in crate::csharp) fn invalid_number(&self, p: usize) -> Option<usize> {
        let end = self.number_word_run(p);
        (end > p).then_some(end)
    }

    /// `(?<!\w)\.?\d(?:[0-9A-Z_a-z]|(?<=[Ee])[-+]|\.\d)*`: the text of a
    /// number, as capture group 0.
    pub(in crate::csharp) fn numeric_literal(&self, p: usize) -> Option<usize> {
        if self.word_before(p) {
            return None;
        }
        let digit = if self.at(p) == b'.' && p < self.lim() {
            p + 1
        } else {
            p
        };
        let (c, after) = self.cp(digit)?;
        if !is_digit(c) {
            return None;
        }
        Some(self.number_word_run(after))
    }

    /// `(?<=\h)_(?=\h)`
    pub(in crate::csharp) fn thousands_separator(&self, p: usize) -> Option<usize> {
        self.separator_at(p).then_some(p + 1)
    }
}
