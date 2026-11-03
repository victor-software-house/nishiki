//! The numeric literals: the decimal, hexadecimal, binary and octal forms of
//! the grammar's two long expressions, each anchored at `\G` and held to the
//! end of the line.
//!
//! Every run of digits is followed by a character it cannot take (a point,
//! an exponent letter, `i` or the end of the line), so no run gives digits
//! back, and an alternative either reaches the end of the line or fails.

use super::super::engine::Lexer;
use crate::rules::Chars;

/// A run of digits with single `_` separators between hex digits.
#[derive(Clone, Copy, Debug)]
struct Digits {
    start: usize,
    end: usize,
    /// The last separator the run took, which the expressions capture.
    separator: Option<usize>,
}

/// `([Ee])(\+?)(-?)(digits)`: where the letter stands, where each sign ends,
/// and where the digits end.
#[derive(Clone, Copy, Debug)]
struct Exponent {
    letter: usize,
    plus: usize,
    minus: usize,
    end: usize,
}

/// Which digits a point stands between: `(?<=[0-9])\.|\.(?=[0-9])` or the
/// same with `\h`.
#[derive(Clone, Copy, Debug)]
enum Radix {
    Decimal,
    Hex,
}

fn decimal(b: u8) -> bool {
    b.is_ascii_digit()
}

fn hex(b: u8) -> bool {
    b.is_ascii_hexdigit()
}

fn binary(b: u8) -> bool {
    matches!(b, b'0' | b'1')
}

fn octal(b: u8) -> bool {
    matches!(b, b'0'..=b'7')
}

impl Lexer<'_> {
    /// `D(?:D|((?<=\h)_(?=\h)))*` at `p`, where `digit` accepts a `D`.
    fn digits(&self, p: usize, digit: fn(u8) -> bool) -> Option<Digits> {
        if p >= self.lim() || !digit(self.at(p)) {
            return None;
        }
        let mut i = p + 1;
        let mut separator = None;
        while i < self.lim() {
            let b = self.at(i);
            if digit(b) {
                i += 1;
            } else if b == b'_' && self.hex_at(i - 1) && self.hex_at(i + 1) {
                separator = Some(i);
                i += 1;
            } else {
                break;
            }
        }
        self.spend(i - p);
        Some(Digits {
            start: p,
            end: i,
            separator,
        })
    }

    /// Records `digits` as group `group`, and its last separator as the group
    /// after it.
    fn record_digits(&mut self, group: usize, digits: Digits) {
        self.cap(group, digits.start, digits.end);
        if let Some(at) = digits.separator {
            self.cap(group + 1, at, at + 1);
        }
    }

    /// `(?<!_)([letters])(\+?)(-?)([0-9](?:[0-9]|(?<=\h)_(?=\h))*)` at `p`.
    fn exponent(&self, p: usize, letters: [u8; 2]) -> Option<Exponent> {
        if p >= self.lim() || !letters.contains(&self.at(p)) || self.prev(p) == Some(b'_') {
            return None;
        }
        let plus = p + 1 + usize::from(p + 1 < self.lim() && self.at(p + 1) == b'+');
        let minus = plus + usize::from(plus < self.lim() && self.at(plus) == b'-');
        let digits = self.digits(minus, decimal)?;
        Some(Exponent {
            letter: p,
            plus,
            minus,
            end: digits.end,
        })
    }

    /// Records `exponent` as the four groups from `group`.
    fn record_exponent(&mut self, group: usize, exponent: Exponent) {
        self.cap(group, exponent.letter, exponent.letter + 1);
        self.cap(group + 1, exponent.letter + 1, exponent.plus);
        self.cap(group + 2, exponent.plus, exponent.minus);
        self.cap(group + 3, exponent.minus, exponent.end);
    }

    /// The end of `(i(?!\w))?` at `p`.
    fn imaginary(&self, p: usize) -> usize {
        p + usize::from(p < self.lim() && self.at(p) == b'i' && self.word_at(p + 1).is_none())
    }

    /// Records the imaginary suffix `p..end` as `group` when there is one.
    fn record_imaginary(&mut self, group: usize, p: usize, end: usize) {
        if end > p {
            self.cap(group, p, end);
        }
    }

    /// Whether a point between `radix` digits stands at `p`.
    fn point(&self, p: usize, radix: Radix) -> bool {
        let digit = match radix {
            Radix::Decimal => decimal,
            Radix::Hex => hex,
        };
        p < self.lim()
            && self.at(p) == b'.'
            && (self.prev(p).is_some_and(digit) || (p + 1 < self.lim() && digit(self.at(p + 1))))
    }

    /// `(?=[.0-9])(?!0[BOXbox])` at `p`.
    fn plain_decimal_start(&self, p: usize) -> bool {
        p < self.lim()
            && matches!(self.at(p), b'.' | b'0'..=b'9')
            && !(self.at(p) == b'0'
                && p + 1 < self.lim()
                && matches!(self.at(p + 1), b'B' | b'O' | b'X' | b'b' | b'o' | b'x'))
    }

    /// `0[Xx]` at `p`.
    fn hex_prefix(&self, p: usize) -> bool {
        self.starts(p, b"0x") || self.starts(p, b"0X")
    }

    /// The end of `_?` at `p`.
    fn underscore(&self, p: usize) -> usize {
        p + usize::from(p < self.lim() && self.at(p) == b'_')
    }

    /// The decimal and hexadecimal floating-point literals:
    /// `\G(?:(?:(?:(?:(?:D1|D2)|D3)|H1)|H2)|H3)`, where D1 is digits, a point
    /// and optional digits and exponent; D2 digits and an exponent; D3 a
    /// point, digits and an optional exponent; H1 a hexadecimal mantissa with
    /// a point and a `p` exponent; H2 one without the point; H3 one that
    /// starts with the point. Each ends with `(i(?!\w))?(?:\n|$)`.
    pub(in crate::go) fn nested4_at0_at0(&mut self, p: usize) -> Option<usize> {
        if !self.at_anchor(p) {
            return None;
        }
        self.decimal_with_point(p)
            .or_else(|| self.decimal_with_exponent(p))
            .or_else(|| self.decimal_from_point(p))
            .or_else(|| self.hex_with_point(p))
            .or_else(|| self.hex_with_exponent(p))
            .or_else(|| self.hex_from_point(p))
    }

    /// D1: groups 1 to 10.
    fn decimal_with_point(&mut self, p: usize) -> Option<usize> {
        if !self.plain_decimal_start(p) {
            return None;
        }
        let whole = self.digits(p, decimal)?;
        if !self.point(whole.end, Radix::Decimal) {
            return None;
        }
        let fraction = self.digits(whole.end + 1, decimal);
        let after = fraction.map_or(whole.end + 1, |digits| digits.end);
        let exponent = self.exponent(after, *b"Ee");
        let before_i = exponent.map_or(after, |exponent| exponent.end);
        let after_i = self.imaginary(before_i);
        let end = self.eol_end(after_i)?;
        self.record_digits(1, whole);
        self.cap(3, whole.end, whole.end + 1);
        if let Some(digits) = fraction {
            self.record_digits(4, digits);
        }
        if let Some(exponent) = exponent {
            self.record_exponent(6, exponent);
        }
        self.record_imaginary(10, before_i, after_i);
        Some(end)
    }

    /// D2: groups 11 to 17.
    fn decimal_with_exponent(&mut self, p: usize) -> Option<usize> {
        if !self.plain_decimal_start(p) {
            return None;
        }
        let whole = self.digits(p, decimal)?;
        let exponent = self.exponent(whole.end, *b"Ee")?;
        let after_i = self.imaginary(exponent.end);
        let end = self.eol_end(after_i)?;
        self.record_digits(11, whole);
        self.record_exponent(13, exponent);
        self.record_imaginary(17, exponent.end, after_i);
        Some(end)
    }

    /// D3: groups 18 to 25.
    fn decimal_from_point(&mut self, p: usize) -> Option<usize> {
        if !self.point(p, Radix::Decimal) {
            return None;
        }
        let fraction = self.digits(p + 1, decimal)?;
        let exponent = self.exponent(fraction.end, *b"Ee");
        let before_i = exponent.map_or(fraction.end, |exponent| exponent.end);
        let after_i = self.imaginary(before_i);
        let end = self.eol_end(after_i)?;
        self.cap(18, p, p + 1);
        self.record_digits(19, fraction);
        if let Some(exponent) = exponent {
            self.record_exponent(21, exponent);
        }
        self.record_imaginary(25, before_i, after_i);
        Some(end)
    }

    /// H1: groups 26 to 36.
    fn hex_with_point(&mut self, p: usize) -> Option<usize> {
        if !self.hex_prefix(p) {
            return None;
        }
        let whole = self.digits(self.underscore(p + 2), hex)?;
        if !self.point(whole.end, Radix::Hex) {
            return None;
        }
        let fraction = self.digits(whole.end + 1, hex);
        let after = fraction.map_or(whole.end + 1, |digits| digits.end);
        let exponent = self.exponent(after, *b"Pp")?;
        let after_i = self.imaginary(exponent.end);
        let end = self.eol_end(after_i)?;
        self.cap(26, p, p + 2);
        self.record_digits(27, whole);
        self.cap(29, whole.end, whole.end + 1);
        if let Some(digits) = fraction {
            self.record_digits(30, digits);
        }
        self.record_exponent(32, exponent);
        self.record_imaginary(36, exponent.end, after_i);
        Some(end)
    }

    /// H2: groups 37 to 44.
    fn hex_with_exponent(&mut self, p: usize) -> Option<usize> {
        if !self.hex_prefix(p) {
            return None;
        }
        let whole = self.digits(self.underscore(p + 2), hex)?;
        let exponent = self.exponent(whole.end, *b"Pp")?;
        let after_i = self.imaginary(exponent.end);
        let end = self.eol_end(after_i)?;
        self.cap(37, p, p + 2);
        self.record_digits(38, whole);
        self.record_exponent(40, exponent);
        self.record_imaginary(44, exponent.end, after_i);
        Some(end)
    }

    /// H3: groups 45 to 53.
    fn hex_from_point(&mut self, p: usize) -> Option<usize> {
        if !self.hex_prefix(p) || !self.point(p + 2, Radix::Hex) {
            return None;
        }
        let fraction = self.digits(p + 3, hex)?;
        let exponent = self.exponent(fraction.end, *b"Pp")?;
        let after_i = self.imaginary(exponent.end);
        let end = self.eol_end(after_i)?;
        self.cap(45, p, p + 2);
        self.cap(46, p + 2, p + 3);
        self.record_digits(47, fraction);
        self.record_exponent(49, exponent);
        self.record_imaginary(53, exponent.end, after_i);
        Some(end)
    }

    /// The integer literals: `\G(?:(?:(?:D|B)|O)|X)`, where D is decimal
    /// digits, B `0b` and binary digits, O `0`, an optional `o` and octal
    /// digits, and X `0x` and hexadecimal digits. Each ends with
    /// `(i(?!\w))?(?:\n|$)`.
    pub(in crate::go) fn nested4_at0_at1(&mut self, p: usize) -> Option<usize> {
        if !self.at_anchor(p) {
            return None;
        }
        self.integer_decimal(p)
            .or_else(|| self.integer_prefixed(p, 4, *b"Bb", binary))
            .or_else(|| self.integer_octal(p))
            .or_else(|| self.integer_prefixed(p, 12, *b"Xx", hex))
    }

    /// D: groups 1 to 3.
    fn integer_decimal(&mut self, p: usize) -> Option<usize> {
        if !self.plain_decimal_start(p) {
            return None;
        }
        let whole = self.digits(p, decimal)?;
        let after_i = self.imaginary(whole.end);
        let end = self.eol_end(after_i)?;
        self.record_digits(1, whole);
        self.record_imaginary(3, whole.end, after_i);
        Some(end)
    }

    /// B or X: `(0[letters])_?(digits)(i(?!\w))?(?:\n|$)`, as the four groups
    /// from `group`.
    fn integer_prefixed(
        &mut self,
        p: usize,
        group: usize,
        letters: [u8; 2],
        digit: fn(u8) -> bool,
    ) -> Option<usize> {
        if !(self.at(p) == b'0' && p + 1 < self.lim() && letters.contains(&self.at(p + 1))) {
            return None;
        }
        let digits = self.digits(self.underscore(p + 2), digit)?;
        let after_i = self.imaginary(digits.end);
        let end = self.eol_end(after_i)?;
        self.cap(group, p, p + 2);
        self.record_digits(group + 1, digits);
        self.record_imaginary(group + 3, digits.end, after_i);
        Some(end)
    }

    /// O: `(0[Oo]?)_?((?:[0-7]|(sep))+)(i(?!\w))?(?:\n|$)`, groups 8 to 11.
    fn integer_octal(&mut self, p: usize) -> Option<usize> {
        if p >= self.lim() || self.at(p) != b'0' {
            return None;
        }
        let prefix =
            p + 1 + usize::from(p + 1 < self.lim() && matches!(self.at(p + 1), b'O' | b'o'));
        let digits = self.digits(self.underscore(prefix), octal)?;
        let after_i = self.imaginary(digits.end);
        let end = self.eol_end(after_i)?;
        self.cap(8, p, prefix);
        self.record_digits(9, digits);
        self.record_imaginary(11, digits.end, after_i);
        Some(end)
    }
}
