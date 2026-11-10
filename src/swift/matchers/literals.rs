//! Matchers: literals.

use std::cell::Cell;

use super::super::engine::Lexer;
use crate::classes::{is_letter, is_mark, is_number, is_space};
use crate::rules::Chars;

/// What the lookahead `\S((?!\\\().)*(?=""")` found for a run of the line.
#[derive(Clone, Copy, Debug)]
struct Stop {
    /// Where the search started.
    from: usize,
    /// The first interpolation at or after `from`, or the end of the line.
    end: usize,
    /// The last `"""` that starts in `from..=stop`.
    best: Option<usize>,
}

/// Where the last `"""` of a line starts.
#[derive(Clone, Copy, Debug, Default)]
enum Triple {
    #[default]
    Unknown,
    None,
    At(usize),
}

/// What the matchers of this file remember about a line.
#[derive(Debug, Default)]
pub(in crate::swift) struct Memo {
    /// Where the last `"""` of the line starts, once searched.
    triple: Cell<Triple>,
    /// The hash count asked about last, and the last quote that has at least
    /// that many hashes after it.
    quote: Cell<Option<(usize, Option<usize>)>>,
    /// The last search for text before `"""` that `\(` interrupts.
    plain: Cell<Option<Stop>>,
    /// The last search for text before `"""` that `\#(` interrupts.
    raw: Cell<Option<Stop>>,
}

/// `[0-9_]`.
fn is_digit_or_underscore(b: u8) -> bool {
    b.is_ascii_digit() || b == b'_'
}

/// `[_\h]`.
fn is_hex_or_underscore(b: u8) -> bool {
    b.is_ascii_hexdigit() || b == b'_'
}

/// The class `[]()\[_{}\p{L}\p{N}\p{M}]` that the number rules look back at.
fn is_member_context(c: char) -> bool {
    matches!(c, ']' | '(' | ')' | '[' | '_' | '{' | '}')
        || is_letter(c)
        || is_number(c)
        || is_mark(c)
}

/// Whether a number may follow a member access such as `a.`: most of the
/// grammar's number patterns refuse it with a lookbehind.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum AfterMember {
    /// `(?<![]()\[_{}\p{L}\p{N}\p{M}]\.)` before the digits.
    Refused,
    /// No lookbehind.
    Allowed,
}

/// The two string literals whose text stops before an interpolation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum StringLiteral {
    /// `"""`, interpolated with `\(`.
    Plain,
    /// `#"""`, interpolated with `\#(`.
    Raw,
}

impl Lexer<'_> {
    /// Whether `.` and then a character of `[]()\[_{}\p{L}\p{N}\p{M}]` stand
    /// just before `p`: a member access, as in `a.0`, not a number.
    fn member_dot_before(&self, p: usize) -> bool {
        self.prev_is(p, b'.') && self.char_before(p - 1).is_some_and(is_member_context)
    }

    /// Where the digits start after `(\B-|\b)` and, when `after` refuses a
    /// member access, the lookbehind `(?<![]()\[_{}\p{L}\p{N}\p{M}]\.)`.
    ///
    /// A `-` that no word precedes is taken.
    /// Neither alternative can then hold elsewhere.
    /// Anything else needs `\b`, and the lookbehind only follows that
    /// alternative.
    /// The lookbehind reads the two characters before `p`.
    /// After a `-` they are never a member access.
    fn number_start(&self, p: usize, after: AfterMember) -> Option<usize> {
        if self.at(p) == b'-' && !self.word_before(p) {
            return Some(p + 1);
        }
        (self.boundary(p) && !(after == AfterMember::Refused && self.member_dot_before(p)))
            .then_some(p)
    }

    /// `\.[0-9]` at `i`.
    fn dot_digit(&self, i: usize) -> bool {
        self.at(i) == b'.' && self.at(i + 1).is_ascii_digit()
    }

    /// The end of the run of `[.\w]` from `i`.
    /// Nothing follows it in the regexes that use it.
    /// Giving back therefore cannot matter.
    fn dot_word_end(&self, mut i: usize) -> usize {
        let from = i;
        loop {
            if self.at(i) == b'.' && i < self.lim() {
                i += 1;
            } else if let Some(width) = self.word_at(i) {
                i += width;
            } else {
                break;
            }
        }
        self.spend(i - from);
        i
    }

    /// `0x\h[_\h]*` at `s`: the end of the hexadecimal digits.
    fn hex_run(&self, s: usize) -> Option<usize> {
        let first = self.lit(s, b"0x")?;
        self.at(first)
            .is_ascii_hexdigit()
            .then(|| self.skip(first + 1, is_hex_or_underscore))
    }

    /// `[-+]?[0-9][0-9_]*` at `k`: the end of an exponent's digits.
    fn decimal_exponent_digits(&self, k: usize) -> Option<usize> {
        let first = if matches!(self.at(k), b'-' | b'+') {
            k + 1
        } else {
            k
        };
        self.at(first)
            .is_ascii_digit()
            .then(|| self.skip(first + 1, is_digit_or_underscore))
    }

    /// `[Ee][-+]?[0-9][0-9_]*` at `i`.
    /// Giving back digits cannot help the callers, because `\b` fails between
    /// two word characters.
    fn decimal_exponent(&self, i: usize) -> Option<usize> {
        matches!(self.at(i), b'E' | b'e')
            .then(|| self.decimal_exponent_digits(i + 1))
            .flatten()
    }

    /// `prefix first rest*\b(?!\.[0-9])` after the number start.
    /// The run is taken whole.
    /// Inside it `\b` fails.
    /// Giving back therefore cannot help.
    fn radix_literal(
        &self,
        p: usize,
        prefix: &[u8],
        first: impl Fn(u8) -> bool,
        rest: impl Fn(u8) -> bool,
    ) -> Option<usize> {
        let s = self.number_start(p, AfterMember::Refused)?;
        let digits = self.lit(s, prefix)?;
        if !first(self.at(digits)) {
            return None;
        }
        let end = self.skip(digits + 1, rest);
        (self.boundary(end) && !self.dot_digit(end)).then_some(end)
    }

    /// `(?:[EPep][-+]?\w+)?[.\w]+` at `q`, for a malformed literal such as
    /// `0x_1p+5`.
    /// The exponent is tried first, with the longest `\w+` that leaves
    /// `[.\w]` a character.
    /// Then it is left out.
    fn exponent_then_word(&self, q: usize) -> Option<usize> {
        if matches!(self.at(q), b'E' | b'P' | b'e' | b'p') {
            let k = q + 1;
            if matches!(self.at(k), b'-' | b'+') {
                if let Some(end) = self.word_after_exponent(k + 1) {
                    return Some(end);
                }
            } else if let Some(end) = self.word_after_exponent(k) {
                return Some(end);
            }
        }
        let end = self.dot_word_end(q);
        (end > q).then_some(end)
    }

    /// `\w+[.\w]+` from `t`, as far as it reaches.
    fn word_after_exponent(&self, t: usize) -> Option<usize> {
        let word = self.word_end(t);
        if word == t {
            return None;
        }
        let last = if self.at(word) == b'.' {
            word
        } else {
            let width = self.char_before(word)?.len_utf8();
            word - width
        };
        (last > t).then(|| self.dot_word_end(last))
    }

    /// `delimiter\s*\n`: a backslash that ends a line of a multi-line string.
    /// Only the line feed at the end of the line can match `\n`.
    /// The rest of the line must therefore be white space.
    fn continuation(&self, p: usize, delimiter: &[u8]) -> Option<usize> {
        let from = self.lit(p, delimiter)?;
        let end = self.space_end(from);
        self.spend(end - from);
        (end == self.lim() + 1).then_some(end)
    }

    /// `\h{1,8}}` after `\u{`: the end of an escape.
    fn unicode_escape_end(&self, from: usize) -> Option<usize> {
        let digits = self.skip(from, |b| b.is_ascii_hexdigit());
        ((1..=8).contains(&(digits - from)) && self.at(digits) == b'}').then_some(digits + 1)
    }

    /// `.` at `i`: any character but the line feed.
    fn any_char(&self, i: usize) -> Option<usize> {
        (i < self.lim()).then(|| self.char_end(i))
    }

    /// Where the last `"""` of the line starts.
    fn last_triple(&self) -> Option<usize> {
        match self.x.literals.triple.get() {
            Triple::At(i) => return Some(i),
            Triple::None => return None,
            Triple::Unknown => {}
        }
        self.spend(self.lim());
        let found = self.s.windows(3).rposition(|w| w == b"\"\"\"");
        self.x
            .literals
            .triple
            .set(found.map_or(Triple::None, Triple::At));
        found
    }

    /// Whether a `"` at or after `from` has `hashes` or more `#` after it,
    /// which is where `(?:[^"]|"(?!#{hashes}))*$` stops short of the end of
    /// the line.
    fn quote_then_hashes(&self, from: usize, hashes: usize) -> bool {
        let last = match self.x.literals.quote.get() {
            Some((known, last)) if known == hashes => last,
            _ => {
                self.spend(self.lim());
                let mut run = 0;
                let mut found = None;
                for (i, &b) in self.s.iter().enumerate().rev() {
                    match b {
                        b'#' => run += 1,
                        b'"' if run >= hashes => {
                            found = Some(i);
                            break;
                        }
                        _ => run = 0,
                    }
                }
                self.x.literals.quote.set(Some((hashes, found)));
                found
            }
        };
        last.is_some_and(|q| q >= from)
    }

    /// `\S((?!\\\().)*(?=""")`, or with `\\#(` for a raw string: a character
    /// that is not white space, then characters up to an interpolation, then
    /// `"""`.
    /// The star gives back to the last `"""` before the interpolation.
    /// Every start in a run asks about the same stop.
    /// The search is therefore kept.
    fn string_text_before_triple(&self, p: usize, literal: StringLiteral) -> Option<usize> {
        let (c, start) = self.char_at(p).filter(|_| p < self.lim())?;
        if is_space(c) {
            return None;
        }
        let cell = match literal {
            StringLiteral::Raw => &self.x.literals.raw,
            StringLiteral::Plain => &self.x.literals.plain,
        };
        let found = match cell.get() {
            Some(known) if known.from <= start && start <= known.end => known,
            _ => {
                let interpolation: &[u8] = match literal {
                    StringLiteral::Raw => b"\\#(",
                    StringLiteral::Plain => b"\\(",
                };
                let stop = (start..self.lim())
                    .find(|&i| self.starts(i, interpolation))
                    .unwrap_or(self.lim());
                self.spend(stop - start);
                let best = (start..=stop).rev().find(|&i| self.starts(i, b"\"\"\""));
                let known = Stop {
                    from: start,
                    end: stop,
                    best,
                };
                cell.set(Some(known));
                known
            }
        };
        found.best.filter(|&end| end >= start)
    }

    /// `(\B-|\b)(?<![]()\[_{}\p{L}\p{N}\p{M}]\.)[0-9][0-9_]*(?=\.[0-9]|[Ee])(?:
    /// \.[0-9][0-9_]*)?(?:[Ee][-+]?[0-9][0-9_]*)?\b(?!\.[0-9])`, for
    /// `LiteralsNumericAt0`.
    pub(in crate::swift) fn rx_literals_numeric_at_0(&mut self, p: usize) -> Option<usize> {
        let s = self.number_start(p, AfterMember::Refused)?;
        if !self.at(s).is_ascii_digit() {
            return None;
        }
        let whole = self.skip(s + 1, is_digit_or_underscore);
        let fraction = self
            .dot_digit(whole)
            .then(|| self.skip(whole + 2, is_digit_or_underscore));
        if fraction.is_none() && !matches!(self.at(whole), b'E' | b'e') {
            return None;
        }
        for after_fraction in [fraction, Some(whole)].into_iter().flatten() {
            for end in [self.decimal_exponent(after_fraction), Some(after_fraction)]
                .into_iter()
                .flatten()
            {
                if self.boundary(end) && !self.dot_digit(end) {
                    return Some(end);
                }
            }
        }
        None
    }

    /// `(\B-|\b)(?<![]()\[_{}\p{L}\p{N}\p{M}]\.)(0x\h[_\h]*)(?:\.\h[_\h]*)?
    /// [Pp][-+]?[0-9][0-9_]*\b(?!\.[0-9])`, for ``LiteralsNumericAt1``.
    pub(in crate::swift) fn rx_literals_numeric_at_1(&mut self, p: usize) -> Option<usize> {
        let s = self.number_start(p, AfterMember::Refused)?;
        let mut q = self.hex_run(s)?;
        if self.at(q) == b'.' && self.at(q + 1).is_ascii_hexdigit() {
            q = self.skip(q + 2, is_hex_or_underscore);
        }
        if !matches!(self.at(q), b'P' | b'p') {
            return None;
        }
        let end = self.decimal_exponent_digits(q + 1)?;
        (self.boundary(end) && !self.dot_digit(end)).then_some(end)
    }

    /// `(\B-|\b)(?<![]()\[_{}\p{L}\p{N}\p{M}]\.)(0x\h[_\h]*)(?:\.\h[_\h]*)?
    /// [Pp][-+]?\w*\b(?!\.[0-9])`, for ``LiteralsNumericAt2``.
    ///
    /// `\b` holds at the end of the word.
    /// When `(?!\.[0-9])` fails there, `\w*` gives back down to nothing.
    /// `\b` then holds between the sign and the word.
    ///
    /// After the sign `\b` cannot hold.
    /// The sign is therefore left out.
    pub(in crate::swift) fn rx_literals_numeric_at_2(&mut self, p: usize) -> Option<usize> {
        let s = self.number_start(p, AfterMember::Refused)?;
        let mut q = self.hex_run(s)?;
        if self.at(q) == b'.' && self.at(q + 1).is_ascii_hexdigit() {
            q = self.skip(q + 2, is_hex_or_underscore);
        }
        if !matches!(self.at(q), b'P' | b'p') {
            return None;
        }
        let k = q + 1;
        if matches!(self.at(k), b'-' | b'+') {
            let after_sign = k + 1;
            let end = self.word_end(after_sign);
            if end > after_sign {
                return Some(if self.dot_digit(end) { after_sign } else { end });
            }
            return Some(k);
        }
        let end = self.word_end(k);
        (!self.dot_digit(end)).then_some(end)
    }

    /// `(\B-|\b)(?<![]()\[_{}\p{L}\p{N}\p{M}]\.)(0x\h[_\h]*)\.[0-9][.\w]*`, for
    /// `LiteralsNumericAt3`.
    pub(in crate::swift) fn rx_literals_numeric_at_3(&mut self, p: usize) -> Option<usize> {
        let s = self.number_start(p, AfterMember::Refused)?;
        let r = self.hex_run(s)?;
        if !self.dot_digit(r) {
            return None;
        }
        Some(self.dot_word_end(r + 2))
    }

    /// `(?<=\s|^)-?\.[0-9][.\w]*`, for `LiteralsNumericAt4`.
    pub(in crate::swift) fn rx_literals_numeric_at_4(&mut self, p: usize) -> Option<usize> {
        let after_space = p == 0 || self.char_before(p).is_some_and(is_space);
        if !after_space {
            return None;
        }
        let dot = if self.at(p) == b'-' { p + 1 } else { p };
        if !self.dot_digit(dot) {
            return None;
        }
        Some(self.dot_word_end(dot + 2))
    }

    /// `(\B-|\b)0[box]_[_\h]*(?:[EPep][-+]?\w+)?[.\w]+`, for
    /// `LiteralsNumericAt5`.
    ///
    /// `[_\h]*` gives back until the rest fits.
    /// Every position it gives back holds a hex digit or an underscore.
    /// `[.\w]+` takes either.
    /// The second try therefore always ends the search.
    pub(in crate::swift) fn rx_literals_numeric_at_5(&mut self, p: usize) -> Option<usize> {
        let s = self.number_start(p, AfterMember::Allowed)?;
        if self.at(s) != b'0'
            || !matches!(self.at(s + 1), b'b' | b'o' | b'x')
            || self.at(s + 2) != b'_'
        {
            return None;
        }
        let first = s + 3;
        let mut q = self.skip(first, is_hex_or_underscore);
        loop {
            if let Some(end) = self.exponent_then_word(q) {
                return Some(end);
            }
            if q == first {
                return None;
            }
            q -= 1;
        }
    }

    /// `(?<=[]()\[_{}\p{L}\p{N}\p{M}]\.)[0-9]+\b`, for `LiteralsNumericAt6`.
    pub(in crate::swift) fn rx_literals_numeric_at_6(&mut self, p: usize) -> Option<usize> {
        if !self.member_dot_before(p) {
            return None;
        }
        let end = self.skip(p, |b| b.is_ascii_digit());
        (end > p && self.boundary(end)).then_some(end)
    }

    /// `(\B-|\b)(?<![]()\[_{}\p{L}\p{N}\p{M}]\.)0b[01][01_]*\b(?!\.[0-9])`, for
    /// `LiteralsNumericAt7`.
    pub(in crate::swift) fn rx_literals_numeric_at_7(&mut self, p: usize) -> Option<usize> {
        self.radix_literal(
            p,
            b"0b",
            |b| matches!(b, b'0' | b'1'),
            |b| matches!(b, b'0' | b'1' | b'_'),
        )
    }

    /// `(\B-|\b)(?<![]()\[_{}\p{L}\p{N}\p{M}]\.)0o[0-7][0-7_]*\b(?!\.[0-9])`,
    /// for `LiteralsNumericAt8`.
    pub(in crate::swift) fn rx_literals_numeric_at_8(&mut self, p: usize) -> Option<usize> {
        self.radix_literal(
            p,
            b"0o",
            |b| matches!(b, b'0'..=b'7'),
            |b| matches!(b, b'0'..=b'7' | b'_'),
        )
    }

    /// `(\B-|\b)(?<![]()\[_{}\p{L}\p{N}\p{M}]\.)[0-9][0-9_]*\b(?!\.[0-9])`, for
    /// `LiteralsNumericAt9`.
    pub(in crate::swift) fn rx_literals_numeric_at_9(&mut self, p: usize) -> Option<usize> {
        self.radix_literal(p, b"", |b| b.is_ascii_digit(), is_digit_or_underscore)
    }

    /// `(\B-|\b)(?<![]()\[_{}\p{L}\p{N}\p{M}]\.)0x\h[_\h]*\b(?!\.[0-9])`, for
    /// `LiteralsNumericAt10`.
    pub(in crate::swift) fn rx_literals_numeric_at_10(&mut self, p: usize) -> Option<usize> {
        self.radix_literal(p, b"0x", |b| b.is_ascii_hexdigit(), is_hex_or_underscore)
    }

    /// `(\B-|\b)[0-9][.\w]*`, for `LiteralsNumericAt11`.
    pub(in crate::swift) fn rx_literals_numeric_at_11(&mut self, p: usize) -> Option<usize> {
        let s = self.number_start(p, AfterMember::Allowed)?;
        self.at(s)
            .is_ascii_digit()
            .then(|| self.dot_word_end(s + 1))
    }

    /// `"""`, for `LiteralsStringAt0`.
    pub(in crate::swift) fn rx_literals_string_at_0(&mut self, p: usize) -> Option<usize> {
        let end = self.lit(p, b"\"\"\"")?;
        self.cap(0, p, end);
        Some(end)
    }

    /// `"""(#*)`, for `LiteralsStringAt0End`.
    pub(in crate::swift) fn rx_literals_string_at_0_end(&mut self, p: usize) -> Option<usize> {
        let quotes = self.lit(p, b"\"\"\"")?;
        let end = self.skip(quotes, |b| b == b'#');
        self.cap(0, p, end);
        self.cap(1, quotes, end);
        Some(end)
    }

    /// `\G(?:.+(?=""")|.+)`, for `LiteralsStringAt0At0`,
    /// `LiteralsStringAt1At0`, `LiteralsStringAt2At0`.
    ///
    /// `.+` takes the rest of the line and gives back to the last `"""`
    /// that leaves it one character.
    pub(in crate::swift) fn rx_literals_string_at_0_at_0(&mut self, p: usize) -> Option<usize> {
        if !(self.g && p == self.pos) || p >= self.lim() {
            return None;
        }
        Some(
            self.last_triple()
                .filter(|&end| end > p)
                .unwrap_or(self.lim()),
        )
    }

    /// `\\\s*\n`, for `LiteralsStringAt0At1`.
    pub(in crate::swift) fn rx_literals_string_at_0_at_1(&mut self, p: usize) -> Option<usize> {
        self.continuation(p, b"\\")
    }

    /// `\\["'0\\nrt]`, for `LiteralsStringStringGutsAt0`.
    pub(in crate::swift) fn rx_literals_string_string_guts_at_0(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        let end = self.lit(p, b"\\")?;
        (self.at(end) != b'\n' && b"\"'0\\nrt".contains(&self.at(end))).then_some(end + 1)
    }

    /// `\\u\{\h{1,8}}`, for `LiteralsStringStringGutsAt1`.
    pub(in crate::swift) fn rx_literals_string_string_guts_at_1(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        let end = self.lit(p, b"\\u{")?;
        self.unicode_escape_end(end)
    }

    /// `\\\(`, for `LiteralsStringStringGutsAt2`.
    pub(in crate::swift) fn rx_literals_string_string_guts_at_2(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        let end = self.lit(p, b"\\(")?;
        self.cap(0, p, end);
        Some(end)
    }

    /// `\)`, for `LiteralsStringStringGutsAt2End`, `AttributesAt0End`,
    /// `LiteralsStringRawStringGutsAt2End`, `AttributesAt1End`,
    /// `AttributesAt2At0End`, `DeclarationsTypeEnumAssociatedValuesEnd`,
    /// `LiteralsAt7End`, `LiteralsRegularExpressionLiteralRegexGutsAt1End`,
    /// `LiteralsRegularExpressionLiteralGroupOrConditionalAt0End`,
    /// `LiteralsRegularExpressionLiteralGroupOrConditionalAt1End`,
    /// `LiteralsRegularExpressionLiteralGroupOrConditionalAt2End`,
    /// `BuiltinGlobalFunctionsAt0End`,
    /// `ExpressionsWithoutTrailingClosuresOrMemberReferencesAvailabilityConditionEnd`,
    /// `ExpressionsWithoutTrailingClosuresOrMemberReferencesFunctionOrMacroCallExpressionAt0End`,
    /// `ExpressionsWithoutTrailingClosuresOrMemberReferencesFunctionOrMacroCallExpressionAt1End`,
    /// `LiteralsStringRawStringGutsAt2At1End`,
    /// `LiteralsStringStringGutsAt2At1End`.
    pub(in crate::swift) fn rx_literals_string_string_guts_at_2_end(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        let end = self.lit(p, b")")?;
        self.cap(0, p, end);
        Some(end)
    }

    /// `\A^(#!).*$\n?`, for `CommentsAt0`.
    pub(in crate::swift) fn rx_comments_at_0(&mut self, p: usize) -> Option<usize> {
        if p != 0 || !self.first_line {
            return None;
        }
        let end = self.lit(p, b"#!")?;
        self.cap(1, p, end);
        Some(self.lim() + 1)
    }

    /// `/\*\*(?!/)`, for `CommentsAt1`.
    pub(in crate::swift) fn rx_comments_at_1(&mut self, p: usize) -> Option<usize> {
        let end = self.lit(p, b"/**")?;
        if self.at(end) == b'/' {
            return None;
        }
        self.cap(0, p, end);
        Some(end)
    }

    /// `\*/`, for `CommentsAt1End`, `CommentsNestedEnd`, `CommentsAt2End`,
    /// `CommentsAt3End`, `CommentsAt4`.
    pub(in crate::swift) fn rx_comments_at_1_end(&mut self, p: usize) -> Option<usize> {
        let end = self.lit(p, b"*/")?;
        self.cap(0, p, end);
        Some(end)
    }

    /// `/\*`, for `CommentsNested`, `CommentsAt3`.
    pub(in crate::swift) fn rx_comments_nested(&mut self, p: usize) -> Option<usize> {
        let end = self.lit(p, b"/*")?;
        self.cap(0, p, end);
        Some(end)
    }

    /// `/\*:`, for `CommentsAt2`.
    pub(in crate::swift) fn rx_comments_at_2(&mut self, p: usize) -> Option<usize> {
        let end = self.lit(p, b"/*:")?;
        self.cap(0, p, end);
        Some(end)
    }

    /// `(^[\t ]+)?(?=//)`, for `CommentsAt5`.
    pub(in crate::swift) fn rx_comments_at_5(&mut self, p: usize) -> Option<usize> {
        if p == 0 {
            let end = self.skip(0, |b| b == b' ' || b == b'\t');
            if end > 0 && self.starts(end, b"//") {
                self.cap(1, 0, end);
                return Some(end);
            }
        }
        self.starts(p, b"//").then_some(p)
    }

    /// `///`, for `CommentsAt5At0`.
    pub(in crate::swift) fn rx_comments_at_5_at_0(&mut self, p: usize) -> Option<usize> {
        let end = self.lit(p, b"///")?;
        self.cap(0, p, end);
        Some(end)
    }

    /// `$`, for `CommentsAt5At0End`, `CommentsAt5At1End`, `CommentsAt5At2End`.
    pub(in crate::swift) fn rx_comments_at_5_at_0_end(&mut self, p: usize) -> Option<usize> {
        self.at_eol(p).then_some(p)
    }

    /// `//:`, for `CommentsAt5At1`.
    pub(in crate::swift) fn rx_comments_at_5_at_1(&mut self, p: usize) -> Option<usize> {
        let end = self.lit(p, b"//:")?;
        self.cap(0, p, end);
        Some(end)
    }

    /// `//`, for `CommentsAt5At2`.
    pub(in crate::swift) fn rx_comments_at_5_at_2(&mut self, p: usize) -> Option<usize> {
        let end = self.lit(p, b"//")?;
        self.cap(0, p, end);
        Some(end)
    }

    /// `#"""(?!#)(?=(?:[^"]|"(?!#))*$)`, for `LiteralsStringAt1`.
    pub(in crate::swift) fn rx_literals_string_at_1(&mut self, p: usize) -> Option<usize> {
        let end = self.lit(p, b"#\"\"\"")?;
        if self.at(end) == b'#' || self.quote_then_hashes(end, 1) {
            return None;
        }
        self.cap(0, p, end);
        Some(end)
    }

    /// `"""#(#*)`, for `LiteralsStringAt1End`.
    pub(in crate::swift) fn rx_literals_string_at_1_end(&mut self, p: usize) -> Option<usize> {
        let quotes = self.lit(p, b"\"\"\"#")?;
        let end = self.skip(quotes, |b| b == b'#');
        self.cap(0, p, end);
        self.cap(1, quotes, end);
        Some(end)
    }

    /// `\\#\s*\n`, for `LiteralsStringAt1At1`.
    pub(in crate::swift) fn rx_literals_string_at_1_at_1(&mut self, p: usize) -> Option<usize> {
        self.continuation(p, b"\\#")
    }

    /// `\\#["'0\\nrt]`, for `LiteralsStringRawStringGutsAt0`.
    pub(in crate::swift) fn rx_literals_string_raw_string_guts_at_0(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        let end = self.lit(p, b"\\#")?;
        (self.at(end) != b'\n' && b"\"'0\\nrt".contains(&self.at(end))).then_some(end + 1)
    }

    /// `\\#u\{\h{1,8}}`, for `LiteralsStringRawStringGutsAt1`.
    pub(in crate::swift) fn rx_literals_string_raw_string_guts_at_1(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        let end = self.lit(p, b"\\#u{")?;
        self.unicode_escape_end(end)
    }

    /// `\\#\(`, for `LiteralsStringRawStringGutsAt2`.
    pub(in crate::swift) fn rx_literals_string_raw_string_guts_at_2(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        let end = self.lit(p, b"\\#(")?;
        self.cap(0, p, end);
        Some(end)
    }

    /// `(?<!#)(##+)"""(?!\1)(?=(?:[^"]|"(?!\1))*$)`, for `LiteralsStringAt2`.
    ///
    /// `(?!\1)`: the closing delimiter must not follow at once.
    pub(in crate::swift) fn rx_literals_string_at_2(&mut self, p: usize) -> Option<usize> {
        if self.prev_is(p, b'#') {
            return None;
        }
        let quotes = self.skip(p, |b| b == b'#');
        let hashes = quotes - p;
        if hashes < 2 {
            return None;
        }
        let end = self.lit(quotes, b"\"\"\"")?;
        if self.skip(end, |b| b == b'#') - end >= hashes || self.quote_then_hashes(end, hashes) {
            return None;
        }
        self.cap(0, p, end);
        self.cap(1, p, quotes);
        Some(end)
    }

    /// `"""\1(#*)`, for `LiteralsStringAt2End`.
    pub(in crate::swift) fn rx_literals_string_at_2_end(&mut self, p: usize) -> Option<usize> {
        let quotes = self.lit(p, b"\"\"\"")?;
        let hashes = quotes + self.payload_at(quotes)?;
        let end = self.skip(hashes, |b| b == b'#');
        self.cap(0, p, end);
        self.cap(1, hashes, end);
        Some(end)
    }

    /// `"`, for `LiteralsStringAt3`.
    pub(in crate::swift) fn rx_literals_string_at_3(&mut self, p: usize) -> Option<usize> {
        let end = self.lit(p, b"\"")?;
        self.cap(0, p, end);
        Some(end)
    }

    /// `"(#*)`, for `LiteralsStringAt3End`.
    pub(in crate::swift) fn rx_literals_string_at_3_end(&mut self, p: usize) -> Option<usize> {
        let quote = self.lit(p, b"\"")?;
        let end = self.skip(quote, |b| b == b'#');
        self.cap(0, p, end);
        self.cap(1, quote, end);
        Some(end)
    }

    /// `[\n\r]`, for `LiteralsStringAt3At0`, `LiteralsStringAt4At0`,
    /// `LiteralsStringAt5At0`.
    pub(in crate::swift) fn rx_literals_string_at_3_at_0(&mut self, p: usize) -> Option<usize> {
        matches!(self.at(p), b'\n' | b'\r').then_some(p + 1)
    }

    /// `\\.`, for `LiteralsStringStringGutsAt3`.
    pub(in crate::swift) fn rx_literals_string_string_guts_at_3(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        let after = self.lit(p, b"\\")?;
        self.any_char(after)
    }

    /// `(?<!#)(##+)"`, for `LiteralsStringAt4`.
    pub(in crate::swift) fn rx_literals_string_at_4(&mut self, p: usize) -> Option<usize> {
        if self.prev_is(p, b'#') {
            return None;
        }
        let quote = self.skip(p, |b| b == b'#');
        if quote - p < 2 {
            return None;
        }
        let end = self.lit(quote, b"\"")?;
        self.cap(0, p, end);
        self.cap(1, p, quote);
        Some(end)
    }

    /// `"\1(#*)`, for `LiteralsStringAt4End`.
    pub(in crate::swift) fn rx_literals_string_at_4_end(&mut self, p: usize) -> Option<usize> {
        let quote = self.lit(p, b"\"")?;
        let hashes = quote + self.payload_at(quote)?;
        let end = self.skip(hashes, |b| b == b'#');
        self.cap(0, p, end);
        self.cap(1, hashes, end);
        Some(end)
    }

    /// `#"`, for `LiteralsStringAt5`.
    pub(in crate::swift) fn rx_literals_string_at_5(&mut self, p: usize) -> Option<usize> {
        let end = self.lit(p, b"#\"")?;
        self.cap(0, p, end);
        Some(end)
    }

    /// `"#(#*)`, for `LiteralsStringAt5End`.
    pub(in crate::swift) fn rx_literals_string_at_5_end(&mut self, p: usize) -> Option<usize> {
        let quote = self.lit(p, b"\"#")?;
        let end = self.skip(quote, |b| b == b'#');
        self.cap(0, p, end);
        self.cap(1, quote, end);
        Some(end)
    }

    /// `\\#.`, for `LiteralsStringRawStringGutsAt3`.
    pub(in crate::swift) fn rx_literals_string_raw_string_guts_at_3(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        let after = self.lit(p, b"\\#")?;
        self.any_char(after)
    }

    /// `\bnil\b`, for `LiteralsAt3`.
    pub(in crate::swift) fn rx_literals_at_3(&mut self, p: usize) -> Option<usize> {
        self.keyword(p, b"nil")
    }

    /// `\B#((?:color|image|file)Literal)\b`, for `LiteralsAt4`.
    pub(in crate::swift) fn rx_literals_at_4(&mut self, p: usize) -> Option<usize> {
        if self.boundary(p) {
            return None;
        }
        let name = self.lit(p, b"#")?;
        let end = self.word_end(name);
        let word = self.text(name, end);
        let object = word.strip_suffix(b"Literal")?;
        matches!(object, b"color" | b"image" | b"file").then_some(end)
    }

    /// `\B#externalMacro\b`, for `LiteralsAt5`.
    pub(in crate::swift) fn rx_literals_at_5(&mut self, p: usize) -> Option<usize> {
        if self.boundary(p) {
            return None;
        }
        let end = self.lit(p, b"#externalMacro")?;
        self.word_at(end).is_none().then_some(end)
    }

    /// `\B#keyPath\b`, for `LiteralsAt6`.
    pub(in crate::swift) fn rx_literals_at_6(&mut self, p: usize) -> Option<usize> {
        if self.boundary(p) {
            return None;
        }
        let end = self.lit(p, b"#keyPath")?;
        self.word_at(end).is_none().then_some(end)
    }

    /// `\B(#selector)(\()(?:\s*([gs]etter)\s*(:))?`, for `LiteralsAt7`.
    pub(in crate::swift) fn rx_literals_at_7(&mut self, p: usize) -> Option<usize> {
        if self.boundary(p) {
            return None;
        }
        let name = self.lit(p, b"#selector")?;
        let paren = self.lit(name, b"(")?;
        self.cap(1, p, name);
        self.cap(2, name, paren);
        let accessor = self.space_end(paren);
        let accessor_end = accessor + 6;
        let colon = self.space_end(accessor_end);
        let tail = self.text(accessor, accessor_end);
        if (tail == b"getter" || tail == b"setter") && self.at(colon) == b':' {
            self.cap(3, accessor, accessor_end);
            self.cap(4, colon, colon + 1);
            return Some(colon + 1);
        }
        Some(paren)
    }

    /// `\S((?!\\#\().)*(?=""")`, for `LiteralsStringAt1At3`.
    pub(in crate::swift) fn rx_literals_string_at_1_at_3(&mut self, p: usize) -> Option<usize> {
        self.string_text_before_triple(p, StringLiteral::Raw)
    }

    /// `\S((?!\\\().)*(?=""")`, for `LiteralsStringAt0At3`.
    pub(in crate::swift) fn rx_literals_string_at_0_at_3(&mut self, p: usize) -> Option<usize> {
        self.string_text_before_triple(p, StringLiteral::Plain)
    }
}
