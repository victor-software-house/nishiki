//! Number literals. Each expression matches a whole run of number
//! characters: digits with separators, a point, an exponent, a suffix and, in
//! the variants that mark mistakes, a word that no pattern takes. A group the
//! expression repeats keeps its last match.

use super::super::engine::Lexer;
use super::super::few::Few;

/// How a number's digits are written.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Digits {
    Decimal,
    Hex,
    Octal,
    Binary,
}

impl Digits {
    fn takes(self, b: u8) -> bool {
        match self {
            Digits::Decimal => b.is_ascii_digit(),
            Digits::Hex => b.is_ascii_hexdigit(),
            Digits::Octal => (b'0'..=b'7').contains(&b),
            Digits::Binary => b == b'0' || b == b'1',
        }
    }
}

/// The suffixes a number takes.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Suffix {
    /// `u`, `l`, `ll` and their combinations.
    Integer,
    /// `f` or `l`.
    Float,
}

/// The groups of one expression, in the order they occur.
struct Shape {
    kind: Digits,
    /// The prefix before the digits (`0`, `0[Bb]`, `0[Xx]`) as group 1.
    prefix: &'static [&'static [u8]],
    /// `(?=[.0-9])(?!0[BXbx])` first.
    decimal_start: bool,
    /// The digits before a point, as group and separator group, and whether
    /// they are required.
    int: (usize, usize, bool),
    /// A point (floats): its group.
    point: Option<usize>,
    /// The digits after the point: group and separator group.
    frac: Option<(usize, usize)>,
    /// The exponent's four groups and the letters that start it.
    exponent: Option<([usize; 4], &'static [u8])>,
    /// The suffix's group and kind.
    suffix: (usize, Suffix),
    /// The group of the word no pattern takes, and the characters its first
    /// character may not be.
    invalid: Option<(usize, Forbidden)>,
}

/// The test for a byte that the mistaken word may not start with.
type Forbidden = fn(u8) -> bool;

/// The characters a mistaken word may not start with, after a hexadecimal
/// float or integer.
fn hex_excluded(b: u8) -> bool {
    matches!(b, b'P' | b'p') || b.is_ascii_hexdigit()
}

/// After a decimal float.
fn float_excluded(b: u8) -> bool {
    b.is_ascii_digit() || matches!(b, b'E' | b'e')
}

/// After binary and octal integers.
fn digit_excluded(b: u8) -> bool {
    b.is_ascii_digit()
}

/// Where a number literal stands: its two grammar rules differ in one group.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum NumberIn {
    /// A preprocessor line, with no group for a word no pattern takes.
    Preprocessor,
    /// Code, where that word is a group.
    Code,
}

impl Lexer<'_> {
    /// `(?<=\h)'(?=\h)` at `i`.
    fn separator(&self, i: usize) -> bool {
        self.at(i) == b'\''
            && i < self.lim()
            && self.prev(i).is_some_and(|b| b.is_ascii_hexdigit())
            && self.at(i + 1).is_ascii_hexdigit()
    }

    /// `D(?:D|SEP)*`, and where the last separator in it is.
    fn digit_run_sep(&self, start: usize, kind: Digits) -> Option<(usize, Option<usize>)> {
        if start >= self.lim() || !kind.takes(self.s[start]) {
            return None;
        }
        let mut end = start + 1;
        let mut last = None;
        while end < self.lim() {
            if kind.takes(self.s[end]) {
                end += 1;
            } else if self.separator(end) {
                last = Some(end);
                end += 1;
            } else {
                break;
            }
        }
        Some((end, last))
    }

    /// The suffixes of an integer in the order the expression tries them,
    /// each not followed by a word character; or the floating one.
    ///
    /// An integer suffix is a letter, an optional second one and an optional
    /// `u`.
    fn suffix_ends(&self, pos: usize, kind: Suffix) -> Few<usize, 4> {
        let mut tried = Few::<usize, 4>::new();
        if pos < self.lim() {
            let b = self.s[pos];
            match kind {
                Suffix::Float => {
                    if matches!(b, b'F' | b'L' | b'f' | b'l') {
                        tried.push(pos + 1);
                    }
                }
                Suffix::Integer => self.integer_suffixes(pos, b, &mut tried),
            }
        }
        tried
            .into_iter()
            .filter(|&end| self.word_at(end) == 0)
            .collect()
    }

    /// The ends of the integer suffixes that start with `b` at `pos`, in the
    /// order the expression tries them.
    fn integer_suffixes(&self, pos: usize, b: u8, tried: &mut Few<usize, 4>) {
        if matches!(b, b'U' | b'u') {
            tried.push(pos + 1);
            for letter in *b"lL" {
                if self.at(pos + 1) == letter {
                    if self.at(pos + 2) == letter {
                        tried.push(pos + 3);
                    }
                    tried.push(pos + 2);
                }
            }
        }
        if matches!(b, b'L' | b'l') {
            let double = (self.at(pos + 1) == b).then_some(pos + 2);
            for end in [double, Some(pos + 1)].into_iter().flatten() {
                if matches!(self.at(end), b'U' | b'u') && end < self.lim() {
                    tried.push(end + 1);
                }
                tried.push(end);
            }
        }
        if matches!(b, b'F' | b'f') {
            tried.push(pos + 1);
        }
    }

    /// `(?<!')([Ee])(\+?)(-?)([0-9](?:[0-9]|(?<=\h)'(?=\h))*)`: where the
    /// exponent at `pos` ends, and its groups' extents.
    fn exponent_at(&self, pos: usize, letters: &[u8]) -> Option<(usize, [(usize, usize); 4])> {
        if pos >= self.lim() || self.prev(pos) == Some(b'\'') || !letters.contains(&self.s[pos]) {
            return None;
        }
        let mut at = pos + 1;
        let plus_start = at;
        if self.at(at) == b'+' && at < self.lim() {
            at += 1;
        }
        let plus = (plus_start, at);
        let minus_start = at;
        if self.at(at) == b'-' && at < self.lim() {
            at += 1;
        }
        let minus = (minus_start, at);
        let (end, _) = self.digit_run_sep(at, Digits::Decimal)?;
        Some((end, [(pos, pos + 1), plus, minus, (at, end)]))
    }

    /// The expression `shape` at `p`, which has to be where `\G` matches.
    ///
    /// In an octal shape, `(?:[0-7]|SEP)+`, a separator may follow the `0`
    /// already; an integer shape has only the empty choice where a float has
    /// its point.
    #[expect(clippy::too_many_lines, reason = "one flow for the twelve shapes")]
    fn shape_match(&mut self, p: usize, shape: &Shape) -> Option<usize> {
        if !self.at_g(p) {
            return None;
        }
        let mut caps = Few::<(usize, usize, usize), 11>::new();
        let mut pos = p;
        if !shape.prefix.is_empty() {
            let end = self.lit_any(pos, shape.prefix)?;
            caps.push((1, pos, end));
            pos = end;
        }
        if shape.decimal_start {
            let first = self.at(pos);
            if pos >= self.lim() || !(first == b'.' || first.is_ascii_digit()) {
                return None;
            }
            if first == b'0' && matches!(self.at(pos + 1), b'B' | b'X' | b'b' | b'x') {
                return None;
            }
        }
        let int = if shape.kind == Digits::Octal {
            let mut end = pos;
            let mut last = None;
            while end < self.lim() {
                if Digits::Octal.takes(self.s[end]) {
                    end += 1;
                } else if self.separator(end) {
                    last = Some(end);
                    end += 1;
                } else {
                    break;
                }
            }
            (end > pos).then_some((end, last))
        } else {
            self.digit_run_sep(pos, shape.kind)
        };
        let ints = [Some(int), (!shape.int.2).then_some(None)];
        for int in ints.into_iter().flatten() {
            if int.is_none() && shape.int.2 {
                return None;
            }
            let after_int = int.map_or(pos, |(end, _)| end);
            let mut with_int = caps;
            if let Some((end, sep)) = int {
                with_int.push((shape.int.0, pos, end));
                if let Some(at) = sep {
                    with_int.push((shape.int.1, at, at + 1));
                }
            }
            let after_point = if let Some(group) = shape.point {
                let digit = |b: u8| shape.kind.takes(b);
                let before = self.prev(after_int).is_some_and(digit);
                let ahead = self.at(after_int + 1);
                let is_dot = self.at(after_int) == b'.' && after_int < self.lim();
                if !(is_dot && (before || (after_int + 1 < self.lim() && digit(ahead)))) {
                    continue;
                }
                with_int.push((group, after_int, after_int + 1));
                after_int + 1
            } else {
                after_int
            };
            let fracs = [
                shape
                    .frac
                    .map(|_| self.digit_run_sep(after_point, shape.kind)),
                Some(None),
            ];
            for frac in fracs.into_iter().flatten() {
                let mut with_frac = with_int;
                let after_frac = match (frac, shape.frac) {
                    (Some((end, sep)), Some((group, sep_group))) => {
                        with_frac.push((group, after_point, end));
                        if let Some(at) = sep {
                            with_frac.push((sep_group, at, at + 1));
                        }
                        end
                    }
                    _ => after_point,
                };
                let exponents = [
                    shape
                        .exponent
                        .map(|(_, letters)| self.exponent_at(after_frac, letters)),
                    Some(None),
                ];
                for exponent in exponents.into_iter().flatten() {
                    let mut with_exp = with_frac;
                    let after_exp = match (exponent, shape.exponent) {
                        (Some((end, parts)), Some((groups, _))) => {
                            for (group, (a, b)) in groups.iter().zip(parts) {
                                with_exp.push((*group, a, b));
                            }
                            end
                        }
                        _ => after_frac,
                    };
                    let suffixes = self.suffix_ends(after_exp, shape.suffix.1);
                    for suffix in suffixes.into_iter().map(Some).chain([None]) {
                        let at = suffix.unwrap_or(after_exp);
                        let mut with_suffix = with_exp;
                        if let Some(end) = suffix {
                            with_suffix.push((shape.suffix.0, after_exp, end));
                        }
                        let mut invalid_end = None;
                        if let Some((_, excluded)) = shape.invalid {
                            let first = self.at(at);
                            let takes = self.word_at(at) > 0 && !(first < 0x80 && excluded(first));
                            if at < self.lim() && takes {
                                invalid_end = Some(self.skip_word(at));
                            }
                        }
                        for end in [invalid_end, Some(at)].into_iter().flatten() {
                            if !self.at_eol(end) {
                                continue;
                            }
                            for (group, a, b) in with_suffix {
                                self.cap(group, a, b);
                            }
                            if let (Some((group, _)), Some(_)) = (shape.invalid, invalid_end)
                                && end != at
                            {
                                self.cap(group, at, end);
                            }
                            return Some(end);
                        }
                    }
                }
            }
        }
        None
    }

    /// Number literal expression number `which` (0 to 5): hexadecimal float,
    /// decimal float, binary, octal, hexadecimal and decimal integer; with
    /// in code, the group for a word that no pattern takes.
    pub(super) fn number_shape(&mut self, p: usize, which: u8, place: NumberIn) -> Option<usize> {
        let invalid = place == NumberIn::Code;
        let shape = match which {
            0 => Shape {
                kind: Digits::Hex,
                prefix: &[b"0X", b"0x"],
                decimal_start: false,
                int: (2, 3, false),
                point: Some(4),
                frac: Some((5, 6)),
                exponent: Some(([7, 8, 9, 10], b"Pp")),
                suffix: (11, Suffix::Float),
                invalid: invalid.then_some((12, hex_excluded)),
            },
            1 => Shape {
                kind: Digits::Decimal,
                prefix: &[],
                decimal_start: true,
                int: (1, 2, false),
                point: Some(3),
                frac: Some((4, 5)),
                exponent: Some(([6, 7, 8, 9], b"Ee")),
                suffix: (10, Suffix::Float),
                invalid: invalid.then_some((11, float_excluded)),
            },
            2 => Shape {
                kind: Digits::Binary,
                prefix: &[b"0B", b"0b"],
                decimal_start: false,
                int: (2, 3, true),
                point: None,
                frac: None,
                exponent: None,
                suffix: (4, Suffix::Integer),
                invalid: invalid.then_some((5, digit_excluded)),
            },
            3 => Shape {
                kind: Digits::Octal,
                prefix: &[b"0"],
                decimal_start: false,
                int: (2, 3, true),
                point: None,
                frac: None,
                exponent: None,
                suffix: (4, Suffix::Integer),
                invalid: invalid.then_some((5, digit_excluded)),
            },
            4 => Shape {
                kind: Digits::Hex,
                prefix: &[b"0X", b"0x"],
                decimal_start: false,
                int: (2, 3, true),
                point: None,
                frac: None,
                exponent: Some(([4, 5, 6, 7], b"Pp")),
                suffix: (8, Suffix::Integer),
                invalid: invalid.then_some((9, hex_excluded)),
            },
            _ => Shape {
                kind: Digits::Decimal,
                prefix: &[],
                decimal_start: true,
                int: (1, 2, true),
                point: None,
                frac: None,
                exponent: Some(([3, 4, 5, 6], b"Ee")),
                suffix: (7, Suffix::Integer),
                invalid: invalid.then_some((8, float_excluded)),
            },
        };
        self.shape_match(p, &shape)
    }
}
