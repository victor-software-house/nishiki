//! Numbers: the grammar's four expressions, each followed as an
//! Oniguruma-style backtracking matcher would.
//!
//! A run of digits with underscores, `[0-9]([0-9_]*[0-9])?`, takes its
//! longest match; a shorter one is followed by a digit or an underscore,
//! which none of the expressions allow there, so it never matters.

use crate::java::engine::Lexer;

/// Whether a part of a number is required, optional or not there.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Part {
    Required,
    Optional,
    Absent,
}

impl Lexer<'_> {
    /// The end of `class+(_|class)*` that ends in a digit of `class`, from
    /// `i`: `\h([_\h]*\h)?` and its kin.
    fn digit_run(&self, i: usize, class: fn(u8) -> bool) -> Option<usize> {
        if !class(self.at(i)) || i >= self.lim() {
            return None;
        }
        let mut end = i + 1;
        let mut last = end;
        while end < self.lim() && (class(self.s[end]) || self.s[end] == b'_') {
            end += 1;
            if class(self.s[end - 1]) {
                last = end;
            }
        }
        self.spend(end - i);
        Some(last)
    }

    /// `[Ee][-+]?[0-9]([0-9_]*[0-9])?`.
    fn exponent(&self, i: usize) -> Option<usize> {
        if !matches!(self.at(i), b'E' | b'e') {
            return None;
        }
        let sign = usize::from(matches!(self.at(i + 1), b'+' | b'-'));
        self.digit_run(i + 1 + sign, |b| b.is_ascii_digit())
    }

    /// Where `(?!\$)` holds after a match that ends at `end`.
    fn not_dollar(&self, end: usize) -> Option<usize> {
        (self.at(end) != b'$').then_some(end)
    }

    /// `\b(?<!\$)0([Xx])...\b(?!\$)`: a hexadecimal integer or float.
    pub(super) fn hex_number(&self, p: usize) -> Option<usize> {
        if !self.boundary(p) || self.prev_is(p, b'$') || self.at(p) != b'0' {
            return None;
        }
        if !matches!(self.at(p + 1), b'X' | b'x') {
            return None;
        }
        let digits = p + 2;
        let hex: fn(u8) -> bool = |b| b.is_ascii_hexdigit();
        // `(?<!\.)\h([_\h]*\h)?[Ll]?(?!\.)`
        if let Some(run) = self.digit_run(digits, hex) {
            let end = run + usize::from(matches!(self.at(run), b'L' | b'l'));
            if self.at(end) != b'.' && self.word_at(end).is_none() {
                return self.not_dollar(end);
            }
        }
        // `(\h([_\h]*\h)?\.?|(\h([_\h]*\h)?)?\.\h([_\h]*\h)?)[Pp][-+]?
        // [0-9]([0-9_]*[0-9])?[DFdf]?`
        let whole = self.digit_run(digits, hex);
        let mantissas = [
            whole.map(|end| end + usize::from(self.at(end) == b'.')),
            whole,
            whole
                .or(Some(digits))
                .filter(|&end| self.at(end) == b'.')
                .and_then(|end| self.digit_run(end + 1, hex)),
        ];
        for mantissa in mantissas.into_iter().flatten() {
            if !matches!(self.at(mantissa), b'P' | b'p') {
                continue;
            }
            let sign = usize::from(matches!(self.at(mantissa + 1), b'+' | b'-'));
            let Some(exponent) = self.digit_run(mantissa + 1 + sign, |b| b.is_ascii_digit()) else {
                continue;
            };
            for end in [
                exponent + usize::from(matches!(self.at(exponent), b'D' | b'F' | b'd' | b'f')),
                exponent,
            ] {
                if self.word_at(end).is_none() {
                    return self.not_dollar(end);
                }
            }
        }
        None
    }

    /// `\b(?<!\$)0([Bb])[01]([01_]*[01])?[Ll]?\b(?!\$)` and the octal
    /// `\b(?<!\$)0[0-7]([0-7_]*[0-7])?[Ll]?\b(?!\$)`.
    pub(super) fn radix_number(&self, p: usize, binary: bool) -> Option<usize> {
        if !self.boundary(p) || self.prev_is(p, b'$') || self.at(p) != b'0' {
            return None;
        }
        let (digits, class): (usize, fn(u8) -> bool) = if binary {
            if !matches!(self.at(p + 1), b'B' | b'b') {
                return None;
            }
            (p + 2, |b| matches!(b, b'0' | b'1'))
        } else {
            (p + 1, |b| matches!(b, b'0'..=b'7'))
        };
        let run = self.digit_run(digits, class)?;
        let end = run + usize::from(matches!(self.at(run), b'L' | b'l'));
        if self.word_at(end).is_some() {
            return None;
        }
        self.not_dollar(end)
    }

    /// The tail of a decimal float after its digits: an exponent, then a
    /// suffix, then `\b`.
    fn float_tail(&self, i: usize, exponent: Part, suffix: Part) -> Option<usize> {
        let with = (exponent != Part::Absent)
            .then(|| self.exponent(i))
            .flatten();
        let without = (exponent != Part::Required).then_some(i);
        for after in [with, without].into_iter().flatten() {
            let suffixed = matches!(self.at(after), b'D' | b'F' | b'd' | b'f');
            if suffix != Part::Absent && suffixed && self.word_at(after + 1).is_none() {
                return Some(after + 1);
            }
            if suffix != Part::Required && self.word_at(after).is_none() {
                return Some(after);
            }
        }
        None
    }

    /// `(?<!\$)(...)(?!\$)`: a decimal integer or float, whose eight
    /// alternatives are tried in the grammar's order.
    pub(super) fn decimal_number(&self, p: usize) -> Option<usize> {
        if self.prev_is(p, b'$') {
            return None;
        }
        let digit: fn(u8) -> bool = |b| b.is_ascii_digit();
        let start_digits = self.boundary(p).then(|| self.digit_run(p, digit)).flatten();
        let after_dot = start_digits
            .filter(|&end| self.at(end) == b'.')
            .map(|end| end + 1);
        let alternatives = [
            // `\b[0-9]...\.\B(?!\.)`
            after_dot.filter(|&end| !self.boundary(end) && self.at(end) != b'.'),
            // `\b[0-9]...\.([Ee]...)[DFdf]?\b`
            after_dot.and_then(|end| self.float_tail(end, Part::Required, Part::Optional)),
            // `\b[0-9]...\.([Ee]...)?[DFdf]\b`
            after_dot.and_then(|end| self.float_tail(end, Part::Optional, Part::Required)),
            // `\b[0-9]...\.([0-9]...)([Ee]...)?[DFdf]?\b`
            after_dot
                .and_then(|end| self.digit_run(end, digit))
                .and_then(|end| self.float_tail(end, Part::Optional, Part::Optional)),
            // `(?<!\.)\B\.[0-9]...([Ee]...)?[DFdf]?\b`
            self.leading_dot_float(p),
            // `\b[0-9]...([Ee]...)[DFdf]?\b`
            start_digits.and_then(|end| self.float_tail(end, Part::Required, Part::Optional)),
            // `\b[0-9]...([Ee]...)?[DFdf]\b`
            start_digits.and_then(|end| self.float_tail(end, Part::Optional, Part::Required)),
            // `\b(0|[1-9]([0-9_]*[0-9])?)(?!\.)[Ll]?\b`
            self.decimal_integer(p),
        ];
        alternatives
            .into_iter()
            .flatten()
            .find_map(|end| self.not_dollar(end))
    }

    /// `(?<!\.)\B\.[0-9]([0-9_]*[0-9])?([Ee][-+]?...)?[DFdf]?\b`.
    fn leading_dot_float(&self, p: usize) -> Option<usize> {
        if self.at(p) != b'.' || self.prev_is(p, b'.') || self.boundary(p) {
            return None;
        }
        let end = self.digit_run(p + 1, |b| b.is_ascii_digit())?;
        self.float_tail(end, Part::Optional, Part::Optional)
    }

    /// `\b(0|[1-9]([0-9_]*[0-9])?)(?!\.)[Ll]?\b`.
    fn decimal_integer(&self, p: usize) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        let run = if self.at(p) == b'0' {
            p + 1
        } else if (b'1'..=b'9').contains(&self.at(p)) {
            self.digit_run(p, |b| b.is_ascii_digit())?
        } else {
            return None;
        };
        if self.at(run) == b'.' {
            return None;
        }
        let end = run + usize::from(matches!(self.at(run), b'L' | b'l'));
        self.word_at(end).is_none().then_some(end)
    }
}
