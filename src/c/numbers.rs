//! Numeric constants.
//!
//! The grammar matches a run of number-like characters, and then lexes that
//! run again with six patterns, each of which must match all of it, and a
//! seventh that marks what none of them takes.

use super::engine::{Lexer, Retok};
use super::scope::S;

/// The suffixes an integer pattern takes, which are all that follows its
/// digits: the grammar's alternation, spelled out.
const INT_SUFFIXES: &[&[u8]] = &[
    b"u", b"U", b"ul", b"ull", b"Ul", b"Ull", b"uL", b"uLL", b"UL", b"ULL", b"l", b"ll", b"lu",
    b"llu", b"lU", b"llU", b"L", b"LL", b"Lu", b"LLu", b"LU", b"LLU", b"f", b"F",
];

/// Whether an exponent is decimal, `e` and a power of ten, or hexadecimal,
/// `p` and a power of two.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Exp {
    Decimal,
    Hex,
}

/// Which suffixes a number takes: an integer's, or a float's.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Suffix {
    Int,
    Float,
}

/// What a number's exponent is made of, for decimal and for hexadecimal.
struct ExpScopes {
    unit: &'static [S],
    plus: &'static [S],
    minus: &'static [S],
    digits: &'static [S],
}

const DECIMAL_EXP: ExpScopes = ExpScopes {
    unit: &[S::UnitExpDecimal],
    plus: &[S::OpPlusExpDecimal],
    minus: &[S::OpMinusExpDecimal],
    digits: &[S::NumExpDecimal],
};

const HEX_EXP: ExpScopes = ExpScopes {
    unit: &[S::UnitExpHex],
    plus: &[S::OpPlusExpHex],
    minus: &[S::OpMinusExpHex],
    digits: &[S::NumExpHex],
};

const UNIT_HEX: &[S] = &[S::UnitHex];
const UNIT_BINARY: &[S] = &[S::UnitBinary];
const UNIT_OCTAL: &[S] = &[S::UnitOctal];
const NUM_HEX: &[S] = &[S::NumHex];
const NUM_DECIMAL: &[S] = &[S::NumDecimal];
const NUM_BINARY: &[S] = &[S::NumBinary];
const NUM_OCTAL: &[S] = &[S::NumOctal];
const POINT: &[S] = &[S::NumDecimalPoint];
const SUFFIX_FLOAT: &[S] = &[S::UnitSuffixFloat];
const SUFFIX_INT: &[S] = &[S::UnitSuffixInt];

impl Lexer<'_> {
    /// The outer match: `(?<!\w)\.?\d(?:['.0-9A-Z_a-z]|(?<=[EPep])[-+])*`.
    pub(super) fn number_match(&mut self, p: usize) -> bool {
        if p >= self.len_all() || self.word_before(p) {
            return false;
        }
        let first = if self.b(p) == b'.' && self.digit_at(p + 1) > 0 {
            p + 1 + self.digit_at(p + 1)
        } else if self.digit_at(p) > 0 {
            p + self.digit_at(p)
        } else {
            return false;
        };
        let mut i = first;
        while i < self.len_all() && self.number_char(i) {
            i += 1;
        }
        self.spend(i - p);
        self.cap_retok(p, i, &[], Retok::Number);
        self.plain(p, i, &[])
    }

    /// Lexes the number-like run `a..b` again, under `base`.
    pub(super) fn number(&mut self, a: usize, b: usize, base: u32) {
        let saved = (self.lim, self.nl, self.anchor);
        self.lim = b;
        self.nl = false;
        self.anchor = None;
        self.reset_runs();
        let from = self.sp.len();
        if self.number_patterns(a, b, from) {
            self.emit_spans(base, b, from);
            self.produce(base, b);
        } else {
            self.invalid_number(a, b, base);
        }
        self.sp.truncate(from);
        (self.lim, self.nl, self.anchor) = saved;
        self.reset_runs();
    }

    /// The seventh pattern: `(?:['.0-9A-Z_a-z]|(?<=[EPep])[-+])+`, which takes
    /// the run from its first byte of that class on.
    fn invalid_number(&mut self, a: usize, b: usize, base: u32) {
        let mut i = a;
        while i < b {
            let mut start = i;
            while start < b && !self.number_char(start) {
                start += self.width_at(start);
            }
            self.produce(base, start);
            let mut end = start;
            while end < b && self.number_char(end) {
                end += 1;
            }
            if end > start {
                let stack = self.push_scopes(base, &[S::InvalidNumeric]);
                self.produce(stack, end);
            }
            i = end.max(start);
        }
        self.produce(base, b);
    }

    /// `['.0-9A-Z_a-z]` or `(?<=[EPep])[-+]` at `i`.
    fn number_char(&self, i: usize) -> bool {
        let c = self.b(i);
        c.is_ascii_alphanumeric()
            || matches!(c, b'\'' | b'.' | b'_')
            || (matches!(c, b'-' | b'+') && matches!(self.prev(i), Some(b'E' | b'P' | b'e' | b'p')))
    }

    /// Whether `i` holds `'` between two hex digits: `(?<=\h)'(?=\h)`.
    pub(super) fn digit_sep(&self, i: usize) -> bool {
        self.b(i) == b'\''
            && i + 1 < self.lim
            && self.prev(i).is_some_and(|c| c.is_ascii_hexdigit())
            && self.b(i + 1).is_ascii_hexdigit()
    }

    /// `digit(?:digit|SEP)*` from `i`: its end, or `None` when the first
    /// digit is not there.
    fn run_digits(&self, i: usize, digit: fn(u8) -> bool) -> Option<usize> {
        if i >= self.lim || !digit(self.b(i)) {
            return None;
        }
        Some(self.run_more(i + 1, digit))
    }

    /// `(?:digit|SEP)*` from `i`.
    fn run_more(&self, i: usize, digit: fn(u8) -> bool) -> usize {
        let mut j = i;
        while (j < self.lim && digit(self.b(j))) || self.digit_sep(j) {
            j += 1;
        }
        self.spend(j - i);
        j
    }

    /// Digits with their separators, lexed again for the separators.
    fn cap_digits(&mut self, start: usize, end: usize, scopes: &'static [S]) {
        self.cap_retok(start, end, scopes, Retok::NumSep);
    }

    /// `(?<!')([Pp])(\+?)(-?)([0-9](?:[0-9]|(?<=\h)'(?=\h))*)`, with its
    /// captures, and where it ends.
    fn exponent(&mut self, i: usize, exp: Exp) -> Option<usize> {
        let c = self.b(i);
        let (lead, scopes) = match exp {
            Exp::Hex => (matches!(c, b'P' | b'p'), HEX_EXP),
            Exp::Decimal => (matches!(c, b'E' | b'e'), DECIMAL_EXP),
        };
        if i >= self.lim || !lead || self.prev(i) == Some(b'\'') {
            return None;
        }
        let mut j = i + 1;
        let plus = j < self.lim && self.b(j) == b'+';
        if plus {
            j += 1;
        }
        let minus = j < self.lim && self.b(j) == b'-';
        if minus {
            j += 1;
        }
        let end = self.run_digits(j, |c| c.is_ascii_digit())?;
        self.cap(i, i + 1, scopes.unit);
        if plus {
            self.cap(i + 1, i + 2, scopes.plus);
        }
        if minus {
            let at = i + 1 + usize::from(plus);
            self.cap(at, at + 1, scopes.minus);
        }
        self.cap_digits(j, end, scopes.digits);
        Some(end)
    }

    /// What follows the digits, when it is all that is left: a suffix.
    fn tail(&mut self, i: usize, b: usize, suffix: Suffix) -> bool {
        if i == b {
            return true;
        }
        let rest = self.text(i, b);
        let (ok, scopes) = match suffix {
            Suffix::Float => (matches!(rest, [b'F' | b'L' | b'f' | b'l']), SUFFIX_FLOAT),
            Suffix::Int => (INT_SUFFIXES.contains(&rest), SUFFIX_INT),
        };
        if ok {
            self.cap(i, b, scopes);
        }
        ok
    }

    /// Tries the six patterns on `a..b` in order, leaving the captures of
    /// the one that matches from `from` on.
    fn number_patterns(&mut self, a: usize, b: usize, from: usize) -> bool {
        let patterns: [fn(&mut Self, usize, usize) -> bool; 6] = [
            Self::hex_float,
            Self::dec_float,
            Self::binary,
            Self::octal,
            Self::hex_int,
            Self::dec_int,
        ];
        for pattern in patterns {
            self.sp.truncate(from);
            if pattern(self, a, b) {
                return true;
            }
        }
        self.sp.truncate(from);
        false
    }

    /// `0` and one of the letters in `second`.
    fn at_prefix(&self, a: usize, second: [u8; 2]) -> bool {
        self.b(a) == b'0' && a + 1 < self.lim && second.contains(&self.b(a + 1))
    }

    /// An exponent and then a suffix that ends the number, or a suffix
    /// alone.
    fn exponent_tail(&mut self, i: usize, b: usize, exp: Exp, suffix: Suffix) -> bool {
        let from = self.sp.len();
        if let Some(end) = self.exponent(i, exp) {
            if self.tail(end, b, suffix) {
                return true;
            }
            self.sp.truncate(from);
        }
        self.tail(i, b, suffix)
    }

    /// `\G(0[Xx])(\h…)?((?<=\h)\.|\.(?=\h))(\h…)?(exponent)?([FLfl](?!\w))?$`.
    fn hex_float(&mut self, a: usize, b: usize) -> bool {
        if !self.at_prefix(a, *b"Xx") {
            return false;
        }
        self.cap(a, a + 2, UNIT_HEX);
        let mut i = a + 2;
        if let Some(end) = self.run_digits(i, |c| c.is_ascii_hexdigit()) {
            self.cap_digits(i, end, NUM_HEX);
            i = end;
        }
        let dot = i < self.lim
            && self.b(i) == b'.'
            && (self.prev(i).is_some_and(|c| c.is_ascii_hexdigit())
                || (i + 1 < self.lim && self.b(i + 1).is_ascii_hexdigit()));
        if !dot {
            return false;
        }
        self.cap(i, i + 1, NUM_HEX);
        i += 1;
        if let Some(end) = self.run_digits(i, |c| c.is_ascii_hexdigit()) {
            self.cap_digits(i, end, NUM_HEX);
            i = end;
        }
        self.exponent_tail(i, b, Exp::Hex, Suffix::Float)
    }

    /// The first character of the decimal patterns: `(?=[.0-9])(?!0[BXbx])`.
    fn decimal_start(&self, a: usize) -> bool {
        if a >= self.lim || !(self.b(a) == b'.' || self.b(a).is_ascii_digit()) {
            return false;
        }
        !(self.at_prefix(a, *b"Bb") || self.at_prefix(a, *b"Xx"))
    }

    /// `\G(?=[.0-9])(?!0[BXbx])([0-9]…)?((?<=[0-9])\.|\.(?=[0-9]))([0-9]…)?
    /// (exponent)?([FLfl](?!\w))?$`.
    fn dec_float(&mut self, a: usize, b: usize) -> bool {
        if !self.decimal_start(a) {
            return false;
        }
        let mut i = a;
        if let Some(end) = self.run_digits(i, |c| c.is_ascii_digit()) {
            self.cap_digits(i, end, NUM_DECIMAL);
            i = end;
        }
        let dot = i < self.lim
            && self.b(i) == b'.'
            && (self.prev(i).is_some_and(|c| c.is_ascii_digit())
                || (i + 1 < self.lim && self.b(i + 1).is_ascii_digit()));
        if !dot {
            return false;
        }
        self.cap(i, i + 1, POINT);
        i += 1;
        if let Some(end) = self.run_digits(i, |c| c.is_ascii_digit()) {
            self.cap_digits(i, end, NUM_DECIMAL);
            i = end;
        }
        self.exponent_tail(i, b, Exp::Decimal, Suffix::Float)
    }

    /// `\G(0[Bb])([01]…)(suffix(?!\w))?$`.
    fn binary(&mut self, a: usize, b: usize) -> bool {
        if !self.at_prefix(a, *b"Bb") {
            return false;
        }
        let Some(end) = self.run_digits(a + 2, |c| c == b'0' || c == b'1') else {
            return false;
        };
        self.cap(a, a + 2, UNIT_BINARY);
        self.cap_digits(a + 2, end, NUM_BINARY);
        self.tail(end, b, Suffix::Int)
    }

    /// `\G(0)((?:[0-7]|SEP)+)(suffix(?!\w))?$`.
    fn octal(&mut self, a: usize, b: usize) -> bool {
        if a >= self.lim || self.b(a) != b'0' {
            return false;
        }
        let end = self.run_more(a + 1, |c| (b'0'..=b'7').contains(&c));
        if end == a + 1 {
            return false;
        }
        self.cap(a, a + 1, UNIT_OCTAL);
        self.cap_digits(a + 1, end, NUM_OCTAL);
        self.tail(end, b, Suffix::Int)
    }

    /// `\G(0[Xx])(\h…)(exponent)?(suffix(?!\w))?$`.
    fn hex_int(&mut self, a: usize, b: usize) -> bool {
        if !self.at_prefix(a, *b"Xx") {
            return false;
        }
        let Some(end) = self.run_digits(a + 2, |c| c.is_ascii_hexdigit()) else {
            return false;
        };
        self.cap(a, a + 2, UNIT_HEX);
        self.cap_digits(a + 2, end, NUM_HEX);
        self.exponent_tail(end, b, Exp::Hex, Suffix::Int)
    }

    /// `\G(?=[.0-9])(?!0[BXbx])([0-9]…)(exponent)?(suffix(?!\w))?$`.
    fn dec_int(&mut self, a: usize, b: usize) -> bool {
        if !self.decimal_start(a) {
            return false;
        }
        let Some(end) = self.run_digits(a, |c| c.is_ascii_digit()) else {
            return false;
        };
        self.cap_digits(a, end, NUM_DECIMAL);
        self.exponent_tail(end, b, Exp::Decimal, Suffix::Int)
    }
}
