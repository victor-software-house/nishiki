//! Numeric constants.
//!
//! The grammar opens a rule where a number starts, `(?<!\w)(?=\.??\d)`.
//! Six patterns inside it start at `\G` and must take the whole run of
//! number-like characters, as their closing lookahead says.
//! The seventh marks what none of them takes.
//! The rule ends where the run does.
//! The digit groups are lexed again for their `'` separators.

use super::engine::ObjcMemory;
use crate::rules::{Chars, Grammar, Lexer};

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

/// The groups of an exponent: the letter, `+`, `-` and the digits.
#[derive(Clone, Copy, Debug)]
struct ExpGroups {
    letter: usize,
    plus: usize,
    minus: usize,
    digits: usize,
}

/// The groups of a float's exponent: `([Pp])(\+)?(-)?(digits)` are 8 to 11.
const FLOAT_EXP: ExpGroups = ExpGroups {
    letter: 8,
    plus: 9,
    minus: 10,
    digits: 11,
};

/// The groups of an integer's exponent: 5 to 8.
const INT_EXP: ExpGroups = ExpGroups {
    letter: 5,
    plus: 6,
    minus: 7,
    digits: 8,
};

/// What follows the digits of one number pattern: its exponent and suffix,
/// and their groups.
#[derive(Clone, Copy, Debug)]
struct Shape {
    exp: Exp,
    suffix: Suffix,
    groups: ExpGroups,
    /// The group around the whole exponent.
    whole: usize,
    /// The suffix's group.
    last: usize,
}

/// A hexadecimal float: `CNumbersAt0`.
const HEX_FLOAT: Shape = Shape {
    exp: Exp::Hex,
    suffix: Suffix::Float,
    groups: FLOAT_EXP,
    whole: 7,
    last: 12,
};

/// A decimal float: `CNumbersAt1`.
const DEC_FLOAT: Shape = Shape {
    exp: Exp::Decimal,
    suffix: Suffix::Float,
    groups: FLOAT_EXP,
    whole: 7,
    last: 12,
};

/// A hexadecimal integer: `CNumbersAt4`.
const HEX_INT: Shape = Shape {
    exp: Exp::Hex,
    suffix: Suffix::Int,
    groups: INT_EXP,
    whole: 4,
    last: 9,
};

/// A decimal integer: `CNumbersAt5`.
const DEC_INT: Shape = Shape {
    exp: Exp::Decimal,
    suffix: Suffix::Int,
    groups: INT_EXP,
    whole: 4,
    last: 9,
};

impl<G: Grammar> Lexer<'_, G>
where
    G::Extra: ObjcMemory,
{
    /// The begin `(?<!\w)(?=\.??\d)`: zero width where a number starts.
    pub(super) fn number_begin(&self, p: usize) -> Option<usize> {
        if self.word_before(p) {
            return None;
        }
        let digit = |i: usize| {
            self.char_at(i)
                .is_some_and(|(c, _)| crate::classes::is_digit(c))
        };
        (digit(p) || (self.at(p) == b'.' && digit(p + 1))).then_some(p)
    }

    /// `['.0-9A-Z_a-z]` or `(?<=[EPep])[-+]` at `i`.
    pub(super) fn number_char(&self, i: usize) -> bool {
        let c = self.at(i);
        i < self.lim()
            && (c.is_ascii_alphanumeric()
                || matches!(c, b'\'' | b'.' | b'_')
                || (matches!(c, b'-' | b'+')
                    && matches!(self.prev(i), Some(b'E' | b'P' | b'e' | b'p'))))
    }

    /// The end of the run of number-like characters from `i`: the place where
    /// the closing lookahead holds.
    pub(super) fn number_run_end(&self, i: usize) -> usize {
        let mut j = i;
        while self.number_char(j) {
            j += 1;
        }
        self.spend(j - i);
        j
    }

    /// The seventh pattern: `(?:['.0-9A-Z_a-z]|(?<=[EPep])[-+])+`.
    pub(super) fn invalid_number(&self, p: usize) -> Option<usize> {
        let end = self.number_run_end(p);
        (end > p).then_some(end)
    }

    /// One of the six patterns, which starts at `\G` and takes the run from
    /// `p` to its end.
    pub(super) fn number_pattern(
        &mut self,
        p: usize,
        pattern: fn(&mut Self, usize, usize) -> bool,
    ) -> Option<usize> {
        if !self.at_g(p) {
            return None;
        }
        let b = self.number_run_end(p);
        pattern(self, p, b).then_some(b)
    }

    /// Whether `i` holds `'` between two hex digits: `(?<=\h)'(?=\h)`.
    pub(super) fn digit_sep(&self, i: usize) -> bool {
        self.at(i) == b'\''
            && i + 1 < self.lim()
            && self.prev(i).is_some_and(|c| c.is_ascii_hexdigit())
            && self.at(i + 1).is_ascii_hexdigit()
    }

    /// `digit(?:digit|((?<=\h)'(?=\h)))*` from `i`: its end, and the last
    /// separator, or `None` when the first digit is not there.
    fn run_digits(&self, i: usize, digit: fn(u8) -> bool) -> Option<(usize, Option<usize>)> {
        if i >= self.lim() || !digit(self.at(i)) {
            return None;
        }
        Some(self.run_more(i + 1, digit))
    }

    /// `(?:digit|((?<=\h)'(?=\h)))*` from `i`: its end, and the last
    /// separator.
    fn run_more(&self, i: usize, digit: fn(u8) -> bool) -> (usize, Option<usize>) {
        let mut j = i;
        let mut sep = None;
        loop {
            if j < self.lim() && digit(self.at(j)) {
                j += 1;
            } else if self.digit_sep(j) {
                sep = Some(j);
                j += 1;
            } else {
                break;
            }
        }
        self.spend(j - i);
        (j, sep)
    }

    /// Records digits as group `group` and their last separator as
    /// `group + 1`.
    fn cap_digits(&mut self, group: usize, start: usize, (end, sep): (usize, Option<usize>)) {
        self.cap(group, start, end);
        if let Some(sep) = sep {
            self.cap(group + 1, sep, sep + 1);
        }
    }

    /// `(?<!')([Pp])(\+)?(-)?([0-9](?:[0-9]|(?<=\h)'(?=\h))*)` at `i`, with
    /// `[Ee]` for a decimal exponent: where it ends.
    fn exponent_end(&self, i: usize, exp: Exp) -> Option<usize> {
        let c = self.at(i);
        let lead = match exp {
            Exp::Hex => matches!(c, b'P' | b'p'),
            Exp::Decimal => matches!(c, b'E' | b'e'),
        };
        if i >= self.lim() || !lead || self.prev(i) == Some(b'\'') {
            return None;
        }
        let mut j = i + 1;
        if self.at(j) == b'+' && j < self.lim() {
            j += 1;
        }
        if self.at(j) == b'-' && j < self.lim() {
            j += 1;
        }
        self.run_digits(j, |c| c.is_ascii_digit())
            .map(|(end, _)| end)
    }

    /// The groups of the exponent `i..end`; `whole` is the group around it,
    /// when the pattern names one.
    fn exponent_caps(&mut self, i: usize, end: usize, groups: ExpGroups, whole: usize) {
        self.cap(whole, i, end);
        self.cap(groups.letter, i, i + 1);
        let mut j = i + 1;
        if self.at(j) == b'+' {
            self.cap(groups.plus, j, j + 1);
            j += 1;
        }
        if self.at(j) == b'-' {
            self.cap(groups.minus, j, j + 1);
            j += 1;
        }
        self.cap(groups.digits, j, end);
    }

    /// Whether what follows the digits from `i` to `b` is nothing, or a
    /// suffix the number takes.
    /// A suffix needs `(?!\w)` after it.
    /// The run ends before a non-ASCII letter, which `\w` still takes, so
    /// `10ULé` has no suffix.
    fn suffix_ok(&self, i: usize, b: usize, suffix: Suffix) -> bool {
        let rest = self.text(i, b);
        let listed = match suffix {
            Suffix::Float => matches!(rest, [b'F' | b'L' | b'f' | b'l']),
            Suffix::Int => INT_SUFFIXES.contains(&rest),
        };
        i == b || (listed && self.word_at(b).is_none())
    }

    /// What follows the digits, when it is all that is left: a suffix,
    /// recorded as group `group`.
    fn tail(&mut self, i: usize, b: usize, suffix: Suffix, group: usize) -> bool {
        if !self.suffix_ok(i, b, suffix) {
            return false;
        }
        if i < b {
            self.cap(group, i, b);
        }
        true
    }

    /// `0` and one of the letters in `second`.
    fn at_prefix(&self, a: usize, second: [u8; 2]) -> bool {
        self.at(a) == b'0' && a + 1 < self.lim() && second.contains(&self.at(a + 1))
    }

    /// An exponent and then a suffix that ends the number, or a suffix
    /// alone.
    fn exponent_tail(&mut self, i: usize, b: usize, shape: Shape) -> bool {
        if let Some(end) = self.exponent_end(i, shape.exp)
            && self.suffix_ok(end, b, shape.suffix)
        {
            self.exponent_caps(i, end, shape.groups, shape.whole);
            return self.tail(end, b, shape.suffix, shape.last);
        }
        self.tail(i, b, shape.suffix, shape.last)
    }

    /// `\G(0[Xx])(\h…)?((?<=\h)\.|\.(?=\h))(\h…)?(exponent)?([FLfl](?!\w))?`,
    /// to the end of the run.
    pub(super) fn hex_float(&mut self, a: usize, b: usize) -> bool {
        if !self.at_prefix(a, *b"Xx") {
            return false;
        }
        let mut i = a + 2;
        let before = self.run_digits(i, |c| c.is_ascii_hexdigit());
        let dot_at = before.map_or(i, |(end, _)| end);
        let dot = dot_at < self.lim()
            && self.at(dot_at) == b'.'
            && (self.prev(dot_at).is_some_and(|c| c.is_ascii_hexdigit())
                || (dot_at + 1 < self.lim() && self.at(dot_at + 1).is_ascii_hexdigit()));
        if !dot {
            return false;
        }
        self.cap(1, a, a + 2);
        if let Some(run) = before {
            self.cap_digits(2, i, run);
        }
        self.cap(4, dot_at, dot_at + 1);
        i = dot_at + 1;
        if let Some(run) = self.run_digits(i, |c| c.is_ascii_hexdigit()) {
            self.cap_digits(5, i, run);
            i = run.0;
        }
        self.exponent_tail(i, b, HEX_FLOAT)
    }

    /// The first character of the decimal patterns: `(?=[.0-9])(?!0[BXbx])`.
    fn decimal_start(&self, a: usize) -> bool {
        if a >= self.lim() || !(self.at(a) == b'.' || self.at(a).is_ascii_digit()) {
            return false;
        }
        !(self.at_prefix(a, *b"Bb") || self.at_prefix(a, *b"Xx"))
    }

    /// `\G((?=[.0-9])(?!0[BXbx]))([0-9]…)?((?<=[0-9])\.|\.(?=[0-9]))([0-9]…)?
    /// (exponent)?([FLfl](?!\w))?`, to the end of the run.
    pub(super) fn dec_float(&mut self, a: usize, b: usize) -> bool {
        if !self.decimal_start(a) {
            return false;
        }
        let before = self.run_digits(a, |c| c.is_ascii_digit());
        let dot_at = before.map_or(a, |(end, _)| end);
        let dot = dot_at < self.lim()
            && self.at(dot_at) == b'.'
            && (self.prev(dot_at).is_some_and(|c| c.is_ascii_digit())
                || (dot_at + 1 < self.lim() && self.at(dot_at + 1).is_ascii_digit()));
        if !dot {
            return false;
        }
        self.cap(1, a, a);
        if let Some(run) = before {
            self.cap_digits(2, a, run);
        }
        self.cap(4, dot_at, dot_at + 1);
        let mut i = dot_at + 1;
        if let Some(run) = self.run_digits(i, |c| c.is_ascii_digit()) {
            self.cap_digits(5, i, run);
            i = run.0;
        }
        self.exponent_tail(i, b, DEC_FLOAT)
    }

    /// `\G(0[Bb])([01]…)(suffix(?!\w))?`, to the end of the run.
    pub(super) fn binary(&mut self, a: usize, b: usize) -> bool {
        if !self.at_prefix(a, *b"Bb") {
            return false;
        }
        let Some(run) = self.run_digits(a + 2, |c| c == b'0' || c == b'1') else {
            return false;
        };
        if !self.suffix_ok(run.0, b, Suffix::Int) {
            return false;
        }
        self.cap(1, a, a + 2);
        self.cap_digits(2, a + 2, run);
        self.tail(run.0, b, Suffix::Int, 4)
    }

    /// `\G(0)((?:[0-7]|SEP)+)(suffix(?!\w))?`, to the end of the run.
    pub(super) fn octal(&mut self, a: usize, b: usize) -> bool {
        if a >= self.lim() || self.at(a) != b'0' {
            return false;
        }
        let run = self.run_more(a + 1, |c| (b'0'..=b'7').contains(&c));
        if run.0 == a + 1 || !self.suffix_ok(run.0, b, Suffix::Int) {
            return false;
        }
        self.cap(1, a, a + 1);
        self.cap_digits(2, a + 1, run);
        self.tail(run.0, b, Suffix::Int, 4)
    }

    /// `\G(0[Xx])(\h…)(exponent)?(suffix(?!\w))?`, to the end of the run.
    pub(super) fn hex_int(&mut self, a: usize, b: usize) -> bool {
        if !self.at_prefix(a, *b"Xx") {
            return false;
        }
        let Some(run) = self.run_digits(a + 2, |c| c.is_ascii_hexdigit()) else {
            return false;
        };
        let fits = self
            .exponent_end(run.0, Exp::Hex)
            .is_some_and(|end| self.suffix_ok(end, b, Suffix::Int))
            || self.suffix_ok(run.0, b, Suffix::Int);
        if !fits {
            return false;
        }
        self.cap(1, a, a + 2);
        self.cap_digits(2, a + 2, run);
        self.exponent_tail(run.0, b, HEX_INT)
    }

    /// `\G((?=[.0-9])(?!0[BXbx]))([0-9]…)(exponent)?(suffix(?!\w))?`, to the
    /// end of the run.
    pub(super) fn dec_int(&mut self, a: usize, b: usize) -> bool {
        if !self.decimal_start(a) {
            return false;
        }
        let Some(run) = self.run_digits(a, |c| c.is_ascii_digit()) else {
            return false;
        };
        let fits = self
            .exponent_end(run.0, Exp::Decimal)
            .is_some_and(|end| self.suffix_ok(end, b, Suffix::Int))
            || self.suffix_ok(run.0, b, Suffix::Int);
        if !fits {
            return false;
        }
        self.cap(1, a, a);
        self.cap_digits(2, a, run);
        self.exponent_tail(run.0, b, DEC_INT)
    }
}
