//! Numbers, character literals, escapes, interpolation and the delimiters of
//! percent literals.

use super::super::engine::Lexer;
use crate::rules::Chars;

impl Lexer<'_> {
    /// The end of `D(?>_?D)*` at `p`, where `digit` accepts a `D`: digits that
    /// single `_`s may separate.
    /// Each `_` that no digit follows ends the run.
    fn separated_digits(
        &self,
        p: usize,
        digit: impl Fn(&Self, usize) -> Option<usize>,
    ) -> Option<usize> {
        let mut end = digit(self, p)?;
        loop {
            let after = if self.byte_in(end, |b| b == b'_') {
                end + 1
            } else {
                end
            };
            match digit(self, after) {
                Some(next) => end = next,
                None => return Some(end),
            }
        }
    }

    /// The end of the `\d` character at `p`.
    fn one_digit(&self, p: usize) -> Option<usize> {
        let end = self.digit_end(p);
        (end > p).then(|| self.char_end(p))
    }

    /// The end of the ASCII digit at `p` that `class` accepts.
    fn ascii_digit(&self, p: usize, class: impl Fn(u8) -> bool) -> Option<usize> {
        self.byte_in(p, class).then_some(p + 1)
    }

    /// The end of the exponent `[Ee][-+]?\d(?>_?\d)*` at `p`.
    fn exponent_end(&self, p: usize) -> Option<usize> {
        if !self.byte_in(p, |b| b == b'E' || b == b'e') {
            return None;
        }
        let sign = p + 1 + usize::from(self.byte_in(p + 1, |b| b == b'-' || b == b'+'));
        self.separated_digits(sign, Self::one_digit)
    }

    /// `\b(\d(?>_?\d)*(\.(?![^\s\d])(?>_?\d)*)?([Ee][-+]?\d(?>_?\d)*)?|0(?:…))\
    /// b`
    ///
    /// Only the whole run of digits can be followed by a point, an exponent
    /// or a `\b`, since the digits and `_` are all `\w`.
    /// Of the fraction, the whole run can take an exponent.
    /// The point alone can end at `\b` before a digit.
    pub(in crate::ruby) fn number(&mut self, p: usize) -> Option<usize> {
        if self.word_before(p) {
            return None;
        }
        self.decimal_number(p).or_else(|| self.prefixed_number(p))
    }

    /// The first alternative of [`Lexer::number`].
    fn decimal_number(&self, p: usize) -> Option<usize> {
        let whole = self.separated_digits(p, Self::one_digit)?;
        let point = whole + 1;
        let fraction_allowed = self.byte_in(whole, |b| b == b'.')
            && (!self.avail(point) || self.space_char(point) || self.one_digit(point).is_some());
        if fraction_allowed {
            let fraction = self
                .separated_digits(point, Self::one_digit)
                .unwrap_or(point);
            if let Some(end) = self
                .exponent_end(fraction)
                .filter(|&end| self.boundary(end))
            {
                return Some(end);
            }
            if self.boundary(fraction) {
                return Some(fraction);
            }
            if fraction > point {
                return Some(point);
            }
        }
        if let Some(end) = self.exponent_end(whole).filter(|&end| self.boundary(end)) {
            return Some(end);
        }
        self.boundary(whole).then_some(whole)
    }

    /// The second alternative of [`Lexer::number`]:
    ///
    /// ```text
    /// 0(?:[Xx]\h(?>_?\h)*|[Oo]?[0-7](?>_?[0-7])*|[Bb][01](?>_?[01])*|[Dd]\d(?>_?\d)*)
    /// ```
    fn prefixed_number(&self, p: usize) -> Option<usize> {
        if !self.byte_in(p, |b| b == b'0') {
            return None;
        }
        let letter = p + 1;
        let end = if self.byte_in(letter, |b| b == b'X' || b == b'x') {
            self.separated_digits(letter + 1, |lx, at| {
                lx.ascii_digit(at, |b| b.is_ascii_hexdigit())
            })
        } else if self.byte_in(letter, |b| b == b'B' || b == b'b') {
            self.separated_digits(letter + 1, |lx, at| {
                lx.ascii_digit(at, |b| b == b'0' || b == b'1')
            })
        } else if self.byte_in(letter, |b| b == b'D' || b == b'd') {
            self.separated_digits(letter + 1, Self::one_digit)
        } else {
            let start = letter + usize::from(self.byte_in(letter, |b| b == b'O' || b == b'o'));
            self.separated_digits(start, |lx, at| {
                lx.ascii_digit(at, |b| matches!(b, b'0'..=b'7'))
            })
        }?;
        self.boundary(end).then_some(end)
    }

    /// `(?<!\w)\?(\\(x\h{1,2}(?!\h)\b|0[0-7]{0,2}(?![0-7])\b|[^0CMx])|(\\
    /// [CM]-)+\w|[^\\\s])`: a character literal such as `?a` or `?\n`.
    pub(in crate::ruby) fn character_literal(&mut self, p: usize) -> Option<usize> {
        if self.word_before(p) || !self.byte_in(p, |b| b == b'?') {
            return None;
        }
        let next = p + 1;
        if self.byte_in(next, |b| b == b'\\') {
            if let Some(end) = self.escaped_character(next + 1) {
                return Some(end);
            }
            return self.control_character(next);
        }
        (self.avail(next) && !self.space_char(next)).then(|| self.char_end_nl(next))
    }

    /// `x\h{1,2}(?!\h)\b|0[0-7]{0,2}(?![0-7])\b|[^0CMx]` at `p`, after a
    /// backslash.
    fn escaped_character(&self, p: usize) -> Option<usize> {
        let bounded = |end: usize, digit: fn(u8) -> bool| {
            (!self.byte_in(end, digit) && self.boundary(end)).then_some(end)
        };
        if self.byte_in(p, |b| b == b'x') {
            let hex = |b: u8| b.is_ascii_hexdigit();
            let first = self.ascii_digit(p + 1, hex)?;
            let end = self.ascii_digit(first, hex).unwrap_or(first);
            return bounded(end, hex);
        }
        if self.byte_in(p, |b| b == b'0') {
            let octal = |b: u8| matches!(b, b'0'..=b'7');
            let first = self.ascii_digit(p + 1, octal).unwrap_or(p + 1);
            let end = self.ascii_digit(first, octal).unwrap_or(first);
            return bounded(end, octal);
        }
        (self.avail(p) && !self.byte_in(p, |b| matches!(b, b'C' | b'M')))
            .then(|| self.char_end_nl(p))
    }

    /// `(\\[CM]-)+\w` at `p`.
    fn control_character(&self, p: usize) -> Option<usize> {
        let mut end = p;
        while self.byte_in(end, |b| b == b'\\')
            && self.byte_in(end + 1, |b| b == b'C' || b == b'M')
            && self.byte_in(end + 2, |b| b == b'-')
        {
            end += 3;
        }
        if end == p {
            return None;
        }
        self.word_at(end).map(|width| end + width)
    }

    /// `\\(?:[0-7]{1,3}|x[A-Fa-f\d]{1,2}|.)`
    pub(in crate::ruby) fn escaped_char(&mut self, p: usize) -> Option<usize> {
        if !self.byte_in(p, |b| b == b'\\') {
            return None;
        }
        let next = p + 1;
        let octal = |b: u8| matches!(b, b'0'..=b'7');
        if self.byte_in(next, octal) {
            let mut end = next + 1;
            while end < next + 3 && self.byte_in(end, octal) {
                end += 1;
            }
            return Some(end);
        }
        if self.byte_in(next, |b| b == b'x') {
            let hex = |at: usize| {
                if self.byte_in(at, |b| matches!(b, b'A'..=b'F' | b'a'..=b'f')) {
                    Some(at + 1)
                } else {
                    self.one_digit(at)
                }
            };
            if let Some(first) = hex(next + 1) {
                return Some(hex(first).unwrap_or(first));
            }
        }
        (next < self.lim()).then(|| self.char_end(next))
    }

    /// `\\[chars]`: a backslash and one of `chars`.
    pub(in crate::ruby) fn escaped_one_of(&mut self, p: usize, chars: &[u8]) -> Option<usize> {
        (self.byte_in(p, |b| b == b'\\') && self.byte_in(p + 1, |b| chars.contains(&b)))
            .then_some(p + 2)
    }

    /// `\\.`
    pub(in crate::ruby) fn escaped_any(&mut self, p: usize) -> Option<usize> {
        (self.byte_in(p, |b| b == b'\\') && p + 1 < self.lim()).then(|| self.char_end(p + 1))
    }

    /// `(\W)` at `p`: the delimiter of a percent literal, recorded as group 1.
    /// The end repeats it.
    fn any_delimiter(&mut self, p: usize) -> Option<usize> {
        if !self.avail(p) || self.word_at(p).is_some() {
            return None;
        }
        let end = self.char_end_nl(p);
        self.capture(1, p, end);
        Some(end)
    }

    /// `opener(\W)`: a percent literal whose delimiter is any character that
    /// is not `\w`.
    pub(in crate::ruby) fn percent_any(&mut self, p: usize, opener: &[u8]) -> Option<usize> {
        let delimiter = self.lit(p, opener)?;
        self.any_delimiter(delimiter)
    }

    /// `%[Qx](\W)`
    pub(in crate::ruby) fn percent_quoted_any(&mut self, p: usize) -> Option<usize> {
        let letter = self.lit(p, b"%")?;
        if !self.byte_in(letter, |b| b == b'Q' || b == b'x') {
            return None;
        }
        self.any_delimiter(letter + 1)
    }

    /// `%([^=\w\s])`
    pub(in crate::ruby) fn percent_bare(&mut self, p: usize) -> Option<usize> {
        let delimiter = self.lit(p, b"%")?;
        if self.byte_in(delimiter, |b| b == b'=') || self.space_char(delimiter) {
            return None;
        }
        self.any_delimiter(delimiter)
    }

    /// `%[Qx]?open`
    pub(in crate::ruby) fn percent_quoted(&mut self, p: usize, open: u8) -> Option<usize> {
        let letter = self.lit(p, b"%")?;
        let at = letter + usize::from(self.byte_in(letter, |b| b == b'Q' || b == b'x'));
        self.byte_in(at, |b| b == open).then_some(at + 1)
    }

    /// `\1`: the delimiter the percent literal opened with.
    pub(in crate::ruby) fn repeated_delimiter(&mut self, p: usize) -> Option<usize> {
        self.begin_text_at(p).map(|width| p + width)
    }

    /// The end of `[eimnosux]*` at `p`: a regular expression's flags.
    fn regexp_flags(&self, p: usize) -> usize {
        self.skip(p, |b| {
            matches!(b, b'e' | b'i' | b'm' | b'n' | b'o' | b's' | b'u' | b'x')
        })
    }

    /// `\1[eimnosux]*`
    pub(in crate::ruby) fn repeated_delimiter_flags(&mut self, p: usize) -> Option<usize> {
        let end = self.repeated_delimiter(p)?;
        Some(self.regexp_flags(end))
    }

    /// `close[eimnosux]*`
    pub(in crate::ruby) fn close_with_flags(&mut self, p: usize, close: u8) -> Option<usize> {
        self.byte_in(p, |b| b == close)
            .then(|| self.regexp_flags(p + 1))
    }

    /// `(?<!\()/=`
    pub(in crate::ruby) fn divide_assign(&mut self, p: usize) -> Option<usize> {
        (!self.prev_is(p, b'('))
            .then(|| self.lit(p, b"/="))
            .flatten()
    }

    /// `(?<=\{)(\s+)`
    pub(in crate::ruby) fn blanks_after_brace(&mut self, p: usize) -> Option<usize> {
        if !self.prev_is(p, b'{') {
            return None;
        }
        let end = self.space_end(p);
        (end > p).then_some(end)
    }
}
