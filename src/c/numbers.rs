//! Numeric constants.
//!
//! The grammar matches a run of number-like characters.
//! It then lexes that run again with seven patterns.
//! Each of the first six must match the whole run.
//! The seventh marks what none of them takes.

use super::engine::Lexer;
use super::scope::S;
use super::state::R;
use crate::scope::{Scopes, scopes};

/// The suffixes an integer pattern takes: the grammar's alternation, spelled
/// out.
/// A suffix is all that follows the digits.
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
    unit: Scopes<S>,
    plus: Scopes<S>,
    minus: Scopes<S>,
    digits: Scopes<S>,
}

const DECIMAL_EXP: ExpScopes = ExpScopes {
    unit: scopes![S::UnitExpDecimal],
    plus: scopes![S::OpPlusExpDecimal],
    minus: scopes![S::OpMinusExpDecimal],
    digits: scopes![S::NumExpDecimal],
};

const HEX_EXP: ExpScopes = ExpScopes {
    unit: scopes![S::UnitExpHex],
    plus: scopes![S::OpPlusExpHex],
    minus: scopes![S::OpMinusExpHex],
    digits: scopes![S::NumExpHex],
};

const UNIT_HEX: Scopes<S> = scopes![S::UnitHex];
const UNIT_BINARY: Scopes<S> = scopes![S::UnitBinary];
const UNIT_OCTAL: Scopes<S> = scopes![S::UnitOctal];
const NUM_HEX: Scopes<S> = scopes![S::NumHex];
const NUM_DECIMAL: Scopes<S> = scopes![S::NumDecimal];
const NUM_BINARY: Scopes<S> = scopes![S::NumBinary];
const NUM_OCTAL: Scopes<S> = scopes![S::NumOctal];
const POINT: Scopes<S> = scopes![S::NumDecimalPoint];
const SUFFIX_FLOAT: Scopes<S> = scopes![S::UnitSuffixFloat];
const SUFFIX_INT: Scopes<S> = scopes![S::UnitSuffixInt];

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
        self.span_retok(p, i, S::NO_SCOPES, R::Number);
        self.plain(p, i)
    }

    /// One of the six patterns of a number-like run, `pattern`, tried at `p`.
    /// Each is anchored with `\G` at the start of the run and must take all
    /// of it.
    /// It matches only from where the search of the run began up to the run's
    /// end.
    pub(super) fn number_pattern(
        &mut self,
        p: usize,
        pattern: fn(&mut Self, usize, usize) -> bool,
    ) -> bool {
        let b = self.lim();
        p == self.pos && pattern(self, p, b) && self.plain(p, b)
    }

    /// The seventh pattern: `(?:['.0-9A-Z_a-z]|(?<=[EPep])[-+])+`.
    pub(super) fn invalid_number(&mut self, p: usize) -> bool {
        let mut end = p;
        while end < self.lim() && self.number_char(end) {
            end += 1;
        }
        self.spend(end - p);
        end > p && self.plain(p, end)
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
            && i + 1 < self.lim()
            && self.prev(i).is_some_and(|c| c.is_ascii_hexdigit())
            && self.b(i + 1).is_ascii_hexdigit()
    }

    /// `digit(?:digit|SEP)*` from `i`: its end, or `None` when the first
    /// digit is not there.
    fn run_digits(&self, i: usize, digit: fn(u8) -> bool) -> Option<usize> {
        if i >= self.lim() || !digit(self.b(i)) {
            return None;
        }
        Some(self.run_more(i + 1, digit))
    }

    /// `(?:digit|SEP)*` from `i`.
    fn run_more(&self, i: usize, digit: fn(u8) -> bool) -> usize {
        let mut j = i;
        while (j < self.lim() && digit(self.b(j))) || self.digit_sep(j) {
            j += 1;
        }
        self.spend(j - i);
        j
    }

    /// Digits with their separators, lexed again for the separators.
    fn capture_digits(&mut self, start: usize, end: usize, scopes: Scopes<S>) {
        self.span_retok(start, end, scopes, R::NumSep);
    }

    /// `(?<!')([Pp])(\+?)(-?)([0-9](?:[0-9]|(?<=\h)'(?=\h))*)` at `i`, with
    /// `[Ee]` for a decimal exponent: where it ends.
    fn exponent_end(&self, i: usize, exp: Exp) -> Option<usize> {
        let c = self.b(i);
        let lead = match exp {
            Exp::Hex => matches!(c, b'P' | b'p'),
            Exp::Decimal => matches!(c, b'E' | b'e'),
        };
        if i >= self.lim() || !lead || self.prev(i) == Some(b'\'') {
            return None;
        }
        let mut j = i + 1;
        if j < self.lim() && self.b(j) == b'+' {
            j += 1;
        }
        if j < self.lim() && self.b(j) == b'-' {
            j += 1;
        }
        self.run_digits(j, |c| c.is_ascii_digit())
    }

    /// The captures of the exponent `i..end`.
    fn exponent_spans(&mut self, i: usize, end: usize, exp: Exp) {
        let scopes = match exp {
            Exp::Hex => HEX_EXP,
            Exp::Decimal => DECIMAL_EXP,
        };
        self.span(i, i + 1, scopes.unit);
        let mut j = i + 1;
        if self.b(j) == b'+' {
            self.span(j, j + 1, scopes.plus);
            j += 1;
        }
        if self.b(j) == b'-' {
            self.span(j, j + 1, scopes.minus);
            j += 1;
        }
        self.capture_digits(j, end, scopes.digits);
    }

    /// Whether what follows the digits from `i` to `b` is nothing, or a
    /// suffix the number takes.
    /// A suffix comes with its scopes.
    fn suffix(&self, i: usize, b: usize, suffix: Suffix) -> Option<Scopes<S>> {
        let rest = self.text(i, b);
        let (ok, scopes) = match suffix {
            Suffix::Float => (matches!(rest, [b'F' | b'L' | b'f' | b'l']), SUFFIX_FLOAT),
            Suffix::Int => (INT_SUFFIXES.contains(&rest), SUFFIX_INT),
        };
        (i == b || ok).then_some(scopes)
    }

    /// What follows the digits, when it is all that is left: a suffix.
    fn tail(&mut self, i: usize, b: usize, suffix: Suffix) -> bool {
        let Some(scopes) = self.suffix(i, b, suffix) else {
            return false;
        };
        self.span(i, b, scopes);
        true
    }

    /// `0` and one of the letters in `second`.
    fn at_prefix(&self, a: usize, second: [u8; 2]) -> bool {
        self.b(a) == b'0' && a + 1 < self.lim() && second.contains(&self.b(a + 1))
    }

    /// An exponent and then a suffix that ends the number, or a suffix
    /// alone.
    fn exponent_tail(&mut self, i: usize, b: usize, exp: Exp, suffix: Suffix) -> bool {
        if let Some(end) = self.exponent_end(i, exp)
            && self.suffix(end, b, suffix).is_some()
        {
            self.exponent_spans(i, end, exp);
            return self.tail(end, b, suffix);
        }
        self.tail(i, b, suffix)
    }

    /// `\G(0[Xx])(\h…)?((?<=\h)\.|\.(?=\h))(\h…)?(exponent)?([FLfl](?!\w))?$`.
    pub(super) fn hex_float(&mut self, a: usize, b: usize) -> bool {
        if !self.at_prefix(a, *b"Xx") {
            return false;
        }
        self.span(a, a + 2, UNIT_HEX);
        let mut i = a + 2;
        if let Some(end) = self.run_digits(i, |c| c.is_ascii_hexdigit()) {
            self.capture_digits(i, end, NUM_HEX);
            i = end;
        }
        let dot = i < self.lim()
            && self.b(i) == b'.'
            && (self.prev(i).is_some_and(|c| c.is_ascii_hexdigit())
                || (i + 1 < self.lim() && self.b(i + 1).is_ascii_hexdigit()));
        if !dot {
            return false;
        }
        self.span(i, i + 1, NUM_HEX);
        i += 1;
        if let Some(end) = self.run_digits(i, |c| c.is_ascii_hexdigit()) {
            self.capture_digits(i, end, NUM_HEX);
            i = end;
        }
        self.exponent_tail(i, b, Exp::Hex, Suffix::Float)
    }

    /// The first character of the decimal patterns: `(?=[.0-9])(?!0[BXbx])`.
    fn decimal_start(&self, a: usize) -> bool {
        if a >= self.lim() || !(self.b(a) == b'.' || self.b(a).is_ascii_digit()) {
            return false;
        }
        !(self.at_prefix(a, *b"Bb") || self.at_prefix(a, *b"Xx"))
    }

    /// `\G(?=[.0-9])(?!0[BXbx])([0-9]…)?((?<=[0-9])\.|\.(?=[0-9]))([0-9]…)?
    /// (exponent)?([FLfl](?!\w))?$`.
    pub(super) fn dec_float(&mut self, a: usize, b: usize) -> bool {
        if !self.decimal_start(a) {
            return false;
        }
        let mut i = a;
        if let Some(end) = self.run_digits(i, |c| c.is_ascii_digit()) {
            self.capture_digits(i, end, NUM_DECIMAL);
            i = end;
        }
        let dot = i < self.lim()
            && self.b(i) == b'.'
            && (self.prev(i).is_some_and(|c| c.is_ascii_digit())
                || (i + 1 < self.lim() && self.b(i + 1).is_ascii_digit()));
        if !dot {
            return false;
        }
        self.span(i, i + 1, POINT);
        i += 1;
        if let Some(end) = self.run_digits(i, |c| c.is_ascii_digit()) {
            self.capture_digits(i, end, NUM_DECIMAL);
            i = end;
        }
        self.exponent_tail(i, b, Exp::Decimal, Suffix::Float)
    }

    /// `\G(0[Bb])([01]…)(suffix(?!\w))?$`.
    pub(super) fn binary(&mut self, a: usize, b: usize) -> bool {
        if !self.at_prefix(a, *b"Bb") {
            return false;
        }
        let Some(end) = self.run_digits(a + 2, |c| c == b'0' || c == b'1') else {
            return false;
        };
        self.span(a, a + 2, UNIT_BINARY);
        self.capture_digits(a + 2, end, NUM_BINARY);
        self.tail(end, b, Suffix::Int)
    }

    /// `\G(0)((?:[0-7]|SEP)+)(suffix(?!\w))?$`.
    pub(super) fn octal(&mut self, a: usize, b: usize) -> bool {
        if a >= self.lim() || self.b(a) != b'0' {
            return false;
        }
        let end = self.run_more(a + 1, |c| (b'0'..=b'7').contains(&c));
        if end == a + 1 {
            return false;
        }
        self.span(a, a + 1, UNIT_OCTAL);
        self.capture_digits(a + 1, end, NUM_OCTAL);
        self.tail(end, b, Suffix::Int)
    }

    /// `\G(0[Xx])(\h…)(exponent)?(suffix(?!\w))?$`.
    pub(super) fn hex_int(&mut self, a: usize, b: usize) -> bool {
        if !self.at_prefix(a, *b"Xx") {
            return false;
        }
        let Some(end) = self.run_digits(a + 2, |c| c.is_ascii_hexdigit()) else {
            return false;
        };
        self.span(a, a + 2, UNIT_HEX);
        self.capture_digits(a + 2, end, NUM_HEX);
        self.exponent_tail(end, b, Exp::Hex, Suffix::Int)
    }

    /// `\G(?=[.0-9])(?!0[BXbx])([0-9]…)(exponent)?(suffix(?!\w))?$`.
    pub(super) fn dec_int(&mut self, a: usize, b: usize) -> bool {
        if !self.decimal_start(a) {
            return false;
        }
        let Some(end) = self.run_digits(a, |c| c.is_ascii_digit()) else {
            return false;
        };
        self.capture_digits(a, end, NUM_DECIMAL);
        self.exponent_tail(end, b, Exp::Decimal, Suffix::Int)
    }
}
