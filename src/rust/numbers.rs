//! Numeric literals.

use super::lexer::Lexer;
use super::scope::S;

/// The numeric type names, which are also the literal suffixes.
pub(super) const NUMERIC: [&[u8]; 14] = [
    b"f32", b"f64", b"i128", b"i16", b"i32", b"i64", b"i8", b"isize", b"u128", b"u16", b"u32",
    b"u64", b"u8", b"usize",
];

/// What follows the digits of a decimal literal.
struct Tail {
    end: usize,
    /// Exponent: `e` index, end of the sign, end of the digits.
    exponent: Option<(usize, usize, usize)>,
    /// Start and end of the type suffix.
    suffix: Option<(usize, usize)>,
}

fn is_digit_or_underscore(b: u8) -> bool {
    b.is_ascii_digit() || b == b'_'
}

impl Lexer<'_> {
    /// The end of a type suffix at `i` that ends on a word boundary.
    fn suffix_at(&self, i: usize) -> Option<usize> {
        NUMERIC
            .iter()
            .map(|suffix| i + suffix.len())
            .zip(NUMERIC)
            .find(|&(end, suffix)| self.starts_with(i, suffix) && self.boundary(end))
            .map(|(end, _)| end)
    }

    /// The exponent and suffix after the fraction ending at `f`, then the
    /// closing word boundary; the first reading that reaches one wins.
    fn tail(&self, f: usize) -> Option<Tail> {
        if matches!(self.at(f), b'E' | b'e') {
            let sign_end = f + 1 + usize::from(matches!(self.at(f + 1), b'+' | b'-'));
            let digits_end = self.skip(sign_end, is_digit_or_underscore);
            if digits_end > sign_end {
                let exponent = Some((f, sign_end, digits_end));
                if let Some(end) = self.suffix_at(digits_end) {
                    return Some(Tail {
                        end,
                        exponent,
                        suffix: Some((digits_end, end)),
                    });
                }
                if self.boundary(digits_end) {
                    return Some(Tail {
                        end: digits_end,
                        exponent,
                        suffix: None,
                    });
                }
            }
        }
        if let Some(end) = self.suffix_at(f) {
            return Some(Tail {
                end,
                exponent: None,
                suffix: Some((f, end)),
            });
        }
        self.boundary(f).then_some(Tail {
            end: f,
            exponent: None,
            suffix: None,
        })
    }

    /// `12`, `1_000`, `1.5`, `2e-3`, `7u8`; a trailing dot is kept only when
    /// a word boundary follows it.
    pub(crate) fn decimal(&mut self, p: usize) -> Option<usize> {
        if !self.at(p).is_ascii_digit() || !self.word_start(p) {
            return None;
        }
        let int_end = self.skip(p + 1, is_digit_or_underscore);
        let with_dot = (self.at(int_end) == b'.')
            .then(|| {
                let frac_end = self.skip(int_end + 1, is_digit_or_underscore);
                self.tail(frac_end).map(|tail| (frac_end, tail))
            })
            .flatten();
        let (dot, frac_end, tail) = match with_dot {
            Some((frac_end, tail)) => (Some(int_end), frac_end, tail),
            None => (None, int_end, self.tail(int_end)?),
        };
        self.gap(p);
        let base = S::NumDecimal;
        self.tok(int_end, &[base]);
        if let Some(dot) = dot {
            self.tok(dot + 1, &[base, S::PunctDot]);
        }
        self.tok(frac_end, &[base]);
        if let Some((e, sign_end, digits_end)) = tail.exponent {
            self.tok(e + 1, &[base, S::OpExp]);
            self.tok(sign_end, &[base, S::OpExpSign]);
            self.tok(digits_end, &[base, S::NumExpMantissa]);
        }
        if let Some((start, end)) = tail.suffix {
            self.tok(start, &[base]);
            self.tok(end, &[base, S::TypeNumeric]);
        }
        Some(tail.end)
    }

    /// `0x..`, `0o..` and `0b..` with an optional type suffix.
    pub(crate) fn radix(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b'0' || !self.word_start(p) {
            return None;
        }
        let (scope, digit): (S, fn(u8) -> bool) = match self.at(p + 1) {
            b'x' => (S::NumHex, |b| b.is_ascii_hexdigit() || b == b'_'),
            b'o' => (S::NumOct, |b| matches!(b, b'0'..=b'7' | b'_')),
            b'b' => (S::NumBin, |b| matches!(b, b'0' | b'1' | b'_')),
            _ => return None,
        };
        let digits_end = self.skip(p + 2, digit);
        if digits_end == p + 2 {
            return None;
        }
        let (end, suffix) = match self.suffix_at(digits_end) {
            Some(end) => (end, Some(digits_end)),
            None if self.boundary(digits_end) => (digits_end, None),
            None => return None,
        };
        self.gap(p);
        self.tok(suffix.unwrap_or(end), &[scope]);
        self.tok(end, &[scope, S::TypeNumeric]);
        Some(end)
    }
}
