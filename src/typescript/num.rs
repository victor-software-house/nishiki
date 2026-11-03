//! Numeric literals, tried in the grammar's order.

use super::kind::Kind;
use super::lexer::Lexer;
use super::scope::S;

const PERIOD: S = S::MetaDelimiterDecimalPeriod;
const BIGINT: S = S::StorageTypeNumericBigint;

/// The letters that introduce a radix, its digit test and its scope.
type Radix = (&'static [u8], fn(u8) -> bool, S);

const RADIXES: [Radix; 3] = [
    (b"xX", |c| c.is_ascii_hexdigit(), S::ConstantNumericHex),
    (b"bB", |c| c == b'0' || c == b'1', S::ConstantNumericBinary),
    (b"oO", is_octal, S::ConstantNumericOctal),
];

fn is_octal(c: u8) -> bool {
    (b'0'..=b'7').contains(&c)
}

/// A numeric literal the grammar matches.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Number {
    pub(crate) end: usize,
    scope: S,
    /// The decimal point, which has a scope of its own.
    period: Option<(usize, usize)>,
    /// Where the `n` of a bigint suffix is.
    big: Option<usize>,
}

impl<const D: u8> Lexer<'_, D> {
    fn digits(&self, i: usize, valid: impl Fn(u8) -> bool) -> usize {
        let mut j = i;
        while valid(self.byte(j)) {
            j += 1;
        }
        j
    }

    /// `[0-9][0-9_]*`: its end.
    fn dec_digits(&self, i: usize) -> Option<usize> {
        self.byte(i)
            .is_ascii_digit()
            .then(|| self.digits(i + 1, |b| b.is_ascii_digit() || b == b'_'))
    }

    /// `(n)?`: the end of an optional bigint suffix, and where the `n` is.
    fn bigint(&self, i: usize) -> (usize, Option<usize>) {
        if self.byte(i) == b'n' {
            (i + 1, Some(i))
        } else {
            (i, None)
        }
    }

    /// `[Ee][-+]?[0-9][0-9_]*`: its end.
    fn exponent(&self, i: usize) -> Option<usize> {
        if !matches!(self.byte(i), b'e' | b'E') {
            return None;
        }
        let j = if matches!(self.byte(i + 1), b'-' | b'+') {
            i + 2
        } else {
            i + 1
        };
        self.dec_digits(j)
    }

    /// The numeric literal at `p`, as the grammar's alternatives match it.
    pub(crate) fn number_at(&self, p: usize) -> Option<Number> {
        let b = self.byte(p);
        if !(b.is_ascii_digit() || b == b'.') || self.prev_char(p) == Some('$') {
            return None;
        }
        self.radix_number(p)
            .or_else(|| self.decimal(p))
            .filter(|n| self.byte(n.end) != b'$')
    }

    /// The end of a numeric literal at `i`.
    pub(crate) fn number_end(&self, i: usize) -> Option<usize> {
        self.number_at(i).map(|n| n.end)
    }

    /// The `0x`, `0b`, `0o` and legacy octal forms.
    ///
    /// `0` and an optional `o`, then octal digits.
    fn radix_number(&self, p: usize) -> Option<Number> {
        if self.byte(p) != b'0' || !self.boundary(p) {
            return None;
        }
        for (letters, valid, scope) in RADIXES {
            if letters.contains(&self.byte(p + 1)) && valid(self.byte(p + 2)) {
                let e = self.digits(p + 3, |c| valid(c) || c == b'_');
                let (end, big) = self.bigint(e);
                if self.boundary(end) && self.byte(end) != b'$' {
                    return Some(Number {
                        end,
                        scope,
                        period: None,
                        big,
                    });
                }
            }
        }
        let start = if matches!(self.byte(p + 1), b'o' | b'O') {
            p + 2
        } else {
            p + 1
        };
        if is_octal(self.byte(start)) {
            let e = self.digits(start + 1, |c| is_octal(c) || c == b'_');
            let (end, big) = self.bigint(e);
            if self.boundary(end) && self.byte(end) != b'$' {
                return Some(Number {
                    end,
                    scope: S::ConstantNumericOctal,
                    period: None,
                    big,
                });
            }
        }
        None
    }

    /// The decimal alternatives, in the grammar's order: `1.5e3`, `1.e3`,
    /// `1e3`, `1.5`, `1.`, `.5e3`, `.5` and `12`.
    fn decimal(&self, p: usize) -> Option<Number> {
        const DECIMAL: S = S::ConstantNumericDecimal;
        let number = |end: usize, period: Option<(usize, usize)>, big: Option<usize>| {
            Some(Number {
                end,
                scope: DECIMAL,
                period,
                big,
            })
        };
        if let Some(i_end) = self.dec_digits(p).filter(|_| self.boundary(p)) {
            let dot = (i_end, i_end + 1);
            if self.byte(i_end) == b'.' {
                if let Some(x_end) = self.dec_digits(i_end + 1).and_then(|f| self.exponent(f)) {
                    let (end, big) = self.bigint(x_end);
                    if self.boundary(end) {
                        return number(end, Some(dot), big);
                    }
                }
                if let Some(x_end) = self.exponent(i_end + 1) {
                    let (end, big) = self.bigint(x_end);
                    if self.boundary(end) {
                        return number(end, Some(dot), big);
                    }
                }
            }
            if let Some(x_end) = self.exponent(i_end) {
                let (end, big) = self.bigint(x_end);
                if self.boundary(end) {
                    return number(end, None, big);
                }
            }
            if self.byte(i_end) == b'.' {
                if let Some(f_end) = self.dec_digits(i_end + 1) {
                    let (end, big) = self.bigint(f_end);
                    if self.boundary(end) {
                        return number(end, Some(dot), big);
                    }
                }
                let (end, big) = self.bigint(i_end + 1);
                if !self.boundary(end) {
                    return number(end, Some(dot), big);
                }
            }
        }
        if self.byte(p) == b'.'
            && !self.boundary(p)
            && let Some(f_end) = self.dec_digits(p + 1)
        {
            let dot = (p, p + 1);
            if let Some(x_end) = self.exponent(f_end) {
                let (end, big) = self.bigint(x_end);
                if self.boundary(end) {
                    return number(end, Some(dot), big);
                }
            }
            let (end, big) = self.bigint(f_end);
            if self.boundary(end) {
                return number(end, Some(dot), big);
            }
        }
        let i_end = self.dec_digits(p).filter(|_| self.boundary(p))?;
        let (end, big) = self.bigint(i_end);
        (self.boundary(end) && self.byte(end) != b'.').then_some(Number {
            end,
            scope: DECIMAL,
            period: None,
            big,
        })
    }

    pub(crate) fn numeric_literal(&mut self, p: usize) -> Option<usize> {
        let number = self.number_at(p)?;
        self.flush(p);
        let scope = number.scope;
        let mut at = p;
        if let Some((a, b)) = number.period {
            self.capture(Kind::Top, at, a, &[scope]);
            self.capture(Kind::Top, a, b, &[scope, PERIOD]);
            at = b;
        }
        if let Some(a) = number.big {
            self.capture(Kind::Top, at, a, &[scope]);
            self.capture(Kind::Top, a, a + 1, &[scope, BIGINT]);
            at = a + 1;
        }
        self.capture(Kind::Top, at, number.end, &[scope]);
        Some(number.end)
    }
}
