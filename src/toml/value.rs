//! Values: numbers, dates and times, booleans, and the openers of strings,
//! arrays and inline tables.

use super::lexer::{Lexer, Mode};
use super::scope::S;
use super::state::Rule;
use crate::classes::space_width;

/// Whether a byte is a digit of some radix.
type Digit = fn(u8) -> bool;

/// A digit of a decimal integer.
fn is_decimal(b: u8) -> bool {
    b.is_ascii_digit()
}

/// A digit of a `0x` integer.
fn is_hex(b: u8) -> bool {
    b.is_ascii_hexdigit()
}

/// A digit of a `0o` integer.
fn is_octal(b: u8) -> bool {
    matches!(b, b'0'..=b'7')
}

/// A digit of a `0b` integer.
fn is_binary(b: u8) -> bool {
    matches!(b, b'0' | b'1')
}

impl<H: Mode> Lexer<'_, H> {
    /// The value that starts at `p`, if one does.
    pub(super) fn value(&mut self, p: usize) -> Option<usize> {
        match self.s[p] {
            quote @ (b'"' | b'\'') => {
                let multi = self.at(p + 1) == quote && self.at(p + 2) == quote;
                let rule = match (quote, multi) {
                    (b'"', false) => Rule::Basic,
                    (b'"', true) => Rule::MultilineBasic,
                    (_, false) => Rule::Literal,
                    (_, true) => Rule::MultilineLiteral,
                };
                self.value_starts();
                Some(self.begin(p, p + if multi { 3 } else { 1 }, rule, S::PunctStrBegin))
            }
            b'[' => {
                self.value_starts();
                Some(self.begin(p, p + 1, Rule::Array, S::PunctArrayBegin))
            }
            b'{' => {
                self.value_starts();
                Some(self.begin(p, p + 1, Rule::InlineTable, S::PunctInlineBegin))
            }
            _ => {
                let (end, scope) = self.scalar(p)?;
                self.value_starts();
                self.tok_at(p, end, &[scope]);
                Some(end)
            }
        }
    }

    /// The value after a key's `=`: white space, then one value.
    /// Where no value matches, the first character is plain text.
    /// The lines' own rules read the rest.
    ///
    /// A second `=` matches `=[=\s]*\S?`.
    /// It takes the white space after it and the next character along.
    pub(super) fn value_after_equals(&mut self, equals: usize) -> usize {
        let p = self.skip_space(equals);
        if p >= self.s.len() {
            return p;
        }
        if let Some(end) = self.value(p) {
            return end;
        }
        if self.s[p] != b'=' {
            return p + self.width_at(p);
        }
        let mut end = p;
        loop {
            let width = space_width(self.s, end);
            if width > 0 {
                end += width;
            } else if self.at(end) == b'=' {
                end += 1;
            } else {
                break;
            }
        }
        self.steps.spend(end - p);
        if end < self.s.len() {
            end += self.width_at(end);
        }
        end
    }

    /// A date, time, number or boolean at `p`: its end and scope.
    fn scalar(&self, p: usize) -> Option<(usize, S)> {
        match self.s[p] {
            b'0'..=b'9' => self
                .date_time(p)
                .map(|end| (end, S::Date))
                .or_else(|| self.time(p).map(|end| (end, S::Time)))
                .or_else(|| self.number(p)),
            b'+' | b'-' => self.number(p),
            b'i' | b'n' => self.special(p).map(|end| (end, S::Special)),
            b't' | b'f' => self.boolean(p),
            _ => None,
        }
    }

    /// `true` and `false`, with no check on what follows.
    fn boolean(&self, p: usize) -> Option<(usize, S)> {
        [&b"true"[..], b"false"]
            .into_iter()
            .find(|word| self.s[p..].starts_with(word))
            .map(|word| (p + word.len(), S::Boolean))
    }

    /// `[+-]?(?:inf|nan)`, with no check on what follows.
    fn special(&self, p: usize) -> Option<usize> {
        let i = p + usize::from(matches!(self.at(p), b'+' | b'-'));
        let rest = self.s.get(i..)?;
        (rest.starts_with(b"inf") || rest.starts_with(b"nan")).then_some(i + 3)
    }

    /// An integer, a float or a prefixed integer at `p`, or `inf` and `nan`
    /// after a sign.
    fn number(&self, p: usize) -> Option<(usize, S)> {
        if let Some(end) = self.special(p) {
            return Some((end, S::Special));
        }
        let signed = matches!(self.at(p), b'+' | b'-');
        let i = p + usize::from(signed);
        if !signed && self.at(i) == b'0' {
            let radix: Option<(S, Digit)> = match self.at(i + 1) {
                b'x' => Some((S::Hex, is_hex)),
                b'o' => Some((S::Octal, is_octal)),
                b'b' => Some((S::Binary, is_binary)),
                _ => None,
            };
            if let Some((scope, is_digit)) = radix
                && is_digit(self.at(i + 2))
            {
                return Some((self.digits_with(i + 2, is_digit), scope));
            }
        }
        let mut end = match self.at(i) {
            b'0' => i + 1,
            b'1'..=b'9' => self.digits_with(i, is_decimal),
            _ => return None,
        };
        if !matches!(self.at(end), b'.' | b'e' | b'E') {
            return Some((end, S::Integer));
        }
        if self.at(end) == b'.' && self.at(end + 1).is_ascii_digit() {
            end = self.digits_with(end + 1, is_decimal);
        }
        if matches!(self.at(end), b'e' | b'E') {
            let sign = usize::from(matches!(self.at(end + 1), b'+' | b'-'));
            if self.at(end + 1 + sign).is_ascii_digit() {
                end = self.digits_with(end + 1 + sign, is_decimal);
            }
        }
        Some((end, S::Float))
    }

    /// The end of `digit(_?digit)*` from `i`, where a digit stands.
    fn digits_with(&self, mut i: usize, is_digit: Digit) -> usize {
        let from = i;
        loop {
            if is_digit(self.at(i)) {
                i += 1;
            } else if self.at(i) == b'_' && is_digit(self.at(i + 1)) {
                i += 2;
            } else {
                self.steps.spend(i - from);
                return i;
            }
        }
    }

    /// The two ASCII digits at `i` as a number, if both are digits.
    fn two(&self, i: usize) -> Option<u8> {
        let (a, b) = (self.at(i), self.at(i + 1));
        (a.is_ascii_digit() && b.is_ascii_digit()).then(|| (a - b'0') * 10 + (b - b'0'))
    }

    /// `HH:MM:SS` at `i`, and where it ends, with the hour at most 24, the
    /// minute at most 59 and the second at most 60.
    fn clock(&self, i: usize) -> Option<usize> {
        let ok = self.two(i)? <= 24
            && self.at(i + 2) == b':'
            && self.two(i + 3)? <= 59
            && self.at(i + 5) == b':'
            && self.two(i + 6)? <= 60;
        ok.then_some(i + 8)
    }

    /// `\.\d+` at `i`: where it ends, or `i` when it is not there.
    fn fraction(&self, i: usize) -> usize {
        if self.at(i) != b'.' || !self.at(i + 1).is_ascii_digit() {
            return i;
        }
        let mut end = i + 1;
        while self.at(end).is_ascii_digit() {
            end += 1;
        }
        self.steps.spend(end - i);
        end
    }

    /// A local time, `HH:MM:SS` with an optional fraction.
    fn time(&self, p: usize) -> Option<usize> {
        let end = self.clock(p)?;
        Some(self.fraction(end))
    }

    /// A date, with a time and an offset after it when they are there.
    fn date_time(&self, p: usize) -> Option<usize> {
        let digits = (0..4).all(|k| self.at(p + k).is_ascii_digit());
        let month = self.two(p + 5)?;
        let day = self.two(p + 8)?;
        if !digits
            || self.at(p + 4) != b'-'
            || !(1..=12).contains(&month)
            || self.at(p + 7) != b'-'
            || !(1..=31).contains(&day)
        {
            return None;
        }
        let date = p + 10;
        if !matches!(self.at(date), b'T' | b't' | b' ') {
            return Some(date);
        }
        let Some(clock) = self.clock(date + 1) else {
            return Some(date);
        };
        let end = self.fraction(clock);
        if self.at(end) == b'Z' {
            return Some(end + 1);
        }
        let offset = matches!(self.at(end), b'+' | b'-')
            && self.two(end + 1).is_some_and(|hour| hour <= 24)
            && self.at(end + 3) == b':'
            && self.two(end + 4).is_some_and(|minute| minute <= 59);
        Some(if offset { end + 6 } else { end })
    }
}
