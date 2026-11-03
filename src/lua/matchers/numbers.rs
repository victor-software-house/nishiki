//! Numbers: hexadecimal and decimal, with fractions and exponents.

use super::super::engine::Lexer;
use crate::rules::Chars;

impl Lexer<'_> {
    /// `(?<![.\w\d])`: no `.` or word character stands before `p`.
    fn number_start(&self, p: usize) -> bool {
        !self.prev_is(p, b'.') && !self.word_before(p)
    }

    /// `0[Xx]` at `p`: the end of it.
    fn hex_prefix(&self, p: usize) -> Option<usize> {
        let zero = self.lit(p, b"0")?;
        (zero < self.lim() && matches!(self.at(zero), b'X' | b'x')).then_some(zero + 1)
    }

    /// `\h+` at `p`: the end of the run, when it has a digit.
    fn hex_run(&self, p: usize) -> Option<usize> {
        let end = self.skip(p, |b| b.is_ascii_hexdigit());
        (end > p).then_some(end)
    }

    /// `([Ee]-?\d*)?([Pp][-+]\d+)?` from `p`, each part taken when it
    /// matches: nothing follows them, so none gives anything back.
    fn exponents(&self, p: usize) -> usize {
        let end = self.decimal_exponent(p);
        if end < self.lim() && matches!(self.at(end), b'P' | b'p') {
            let sign = end + 1;
            if sign < self.lim() && matches!(self.at(sign), b'-' | b'+') {
                let digits = self.digit_end(sign + 1);
                if digits > sign + 1 {
                    return digits;
                }
            }
        }
        end
    }

    /// `([Ee]-?\d*)?` from `p`: the end of the exponent, or `p`.
    fn decimal_exponent(&self, p: usize) -> usize {
        if p >= self.lim() || !matches!(self.at(p), b'E' | b'e') {
            return p;
        }
        let minus = if self.at(p + 1) == b'-' && p + 1 < self.lim() {
            p + 2
        } else {
            p + 1
        };
        self.digit_end(minus)
    }

    /// `(?<![.\w\d])0[Xx]\h+(\.\h*)?([Ee]-?\d*)?([Pp][-+]\d+)?`
    pub(super) fn hex_number(&self, p: usize) -> Option<usize> {
        if !self.number_start(p) {
            return None;
        }
        let digits = self.hex_run(self.hex_prefix(p)?)?;
        let fraction = if self.at(digits) == b'.' && digits < self.lim() {
            self.skip(digits + 1, |b| b.is_ascii_hexdigit())
        } else {
            digits
        };
        Some(self.exponents(fraction))
    }

    /// `(?<![.\w\d])0[Xx]\.\h+([Ee]-?\d*)?([Pp][-+]\d+)?`
    pub(super) fn hex_fraction(&self, p: usize) -> Option<usize> {
        if !self.number_start(p) {
            return None;
        }
        let point = self.lit(self.hex_prefix(p)?, b".")?;
        Some(self.exponents(self.hex_run(point)?))
    }

    /// `(?<![.\w\d])0[Xx]\h+(?![.0-9EPep])`. The run gives digits back until
    /// the byte after it is not one of the class; a digit given back is
    /// refused unless it is a letter other than `E` or `e`.
    pub(super) fn hex_integer(&self, p: usize) -> Option<usize> {
        if !self.number_start(p) {
            return None;
        }
        let start = self.hex_prefix(p)?;
        let longest = self.hex_run(start)?;
        (start + 1..=longest)
            .rev()
            .find(|&end| !self.ahead_number_tail(end))
    }

    /// The regex `(?=[.0-9EPep])` at `p`.
    fn ahead_number_tail(&self, p: usize) -> bool {
        p < self.lim() && matches!(self.at(p), b'.' | b'0'..=b'9' | b'E' | b'P' | b'e' | b'p')
    }

    /// `(?<![.\w\d])\d+(\.\d*)?([Ee]-?\d*)?`
    pub(super) fn decimal_number(&self, p: usize) -> Option<usize> {
        if !self.number_start(p) {
            return None;
        }
        let digits = self.digit_end(p);
        if digits == p {
            return None;
        }
        let fraction = if self.at(digits) == b'.' && digits < self.lim() {
            self.digit_end(digits + 1)
        } else {
            digits
        };
        Some(self.decimal_exponent(fraction))
    }

    /// `(?<![.\w\d])\.\d+([Ee]-?\d*)?`
    pub(super) fn decimal_fraction(&self, p: usize) -> Option<usize> {
        if !self.number_start(p) {
            return None;
        }
        let point = self.lit(p, b".")?;
        let digits = self.digit_end(point);
        (digits > point).then(|| self.decimal_exponent(digits))
    }

    /// `(?<![.\w\d])\d+(?![.0-9EPep])`. The run gives digits back until the
    /// character after it is not one of the class: an ASCII digit given back
    /// is refused, another script's digit is not.
    pub(super) fn decimal_integer(&self, p: usize) -> Option<usize> {
        if !self.number_start(p) {
            return None;
        }
        let mut end = self.digit_end(p);
        while end > p {
            if !self.ahead_number_tail(end) {
                return Some(end);
            }
            end = self.char_start_before(end);
        }
        None
    }
}
