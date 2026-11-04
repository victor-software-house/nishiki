//! Values: numbers, units, strings, comments, variables and the punctuation
//! of declarations.

use super::super::engine::Lexer;
use crate::rules::Chars;

/// Whether white space may come before what a matcher looks for.
#[derive(Clone, Copy, Debug)]
pub(in crate::less) enum Spaces {
    /// `\s*` first.
    Before,
    /// Nothing first.
    None,
}

/// Whether `b` is `-` or `+`.
fn sign(b: u8) -> bool {
    matches!(b, b'-' | b'+')
}

impl Lexer<'_> {
    /// `\s*(;)|(?=[)}])`, or `(;)|(?=[)}])`: the `;` in group 1.
    pub(super) fn semicolon_or_close(&mut self, p: usize, spaces: Spaces) -> Option<usize> {
        let at = match spaces {
            Spaces::Before => self.spaces(p),
            Spaces::None => p,
        };
        if self.at(at) == b';' && at < self.lim() {
            self.cap(1, at, at + 1);
            return Some(at + 1);
        }
        matches!(self.at(p), b')' | b'}').then_some(p)
    }

    /// `(?=\s*(;)|(?=[)}]))`.
    pub(super) fn semicolon_or_close_ahead(&self, p: usize) -> Option<usize> {
        (self.spaces_then(p, b";").is_some() || matches!(self.at(p), b')' | b'}')).then_some(p)
    }

    /// The end of `(\+_?)?:` at `p`. A `+_` that no `:` follows gives the
    /// `_` back, and then the `+` needs the `:` instead.
    fn merge_colon_end(&self, p: usize) -> Option<usize> {
        let at = if self.at(p) == b'+' {
            if self.at(p + 1) == b'_' { p + 2 } else { p + 1 }
        } else {
            p
        };
        if self.at(at) == b':' {
            return Some(at + 1);
        }
        (at == p + 2 && self.at(p + 1) == b':').then_some(p + 2)
    }

    /// `(((\+_?)?):)`, the whole in group 1.
    pub(super) fn merge_colon(&mut self, p: usize) -> Option<usize> {
        let end = self.merge_colon_end(p)?;
        self.cap(1, p, end);
        Some(end)
    }

    /// `([\t\s]*)` from `p`, in group `group`.
    pub(super) fn blanks_cap(&mut self, p: usize, group: usize) -> usize {
        let end = self.blanks(p);
        self.cap(group, p, end);
        end
    }

    /// `\b((?:\+_?)?:)([\t\s]*)`.
    pub(super) fn merge_colon_word(&mut self, p: usize) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        let end = self.merge_colon(p)?;
        Some(self.blanks_cap(end, 2))
    }

    /// `(?=(((\+_?)?):)[\t\s]*)`.
    pub(super) fn merge_colon_ahead(&self, p: usize) -> Option<usize> {
        self.merge_colon_end(p).map(|_| p)
    }

    /// `(!)\s*important`.
    pub(super) fn important(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b'!' {
            return None;
        }
        let end = self.lit(self.spaces(p + 1), b"important")?;
        self.cap(1, p, p + 1);
        Some(end)
    }

    /// `(//).*$\n?`: the rest of the line and its line feed.
    pub(super) fn comment_line(&mut self, p: usize) -> Option<usize> {
        self.lit(p, b"//")?;
        self.cap(1, p, p + 2);
        Some(self.lim() + usize::from(self.nl))
    }

    /// `(")|(\n)` for the quote `quote`.
    pub(super) fn string_end(&mut self, p: usize, quote: u8) -> Option<usize> {
        if self.at(p) == quote && p < self.lim() {
            self.cap(1, p, p + 1);
            return Some(p + 1);
        }
        self.line_feed(p, 2)
    }

    /// `(["'])|(\n)`.
    pub(super) fn string_end_any(&mut self, p: usize) -> Option<usize> {
        if matches!(self.at(p), b'"' | b'\'') && p < self.lim() {
            self.cap(1, p, p + 1);
            return Some(p + 1);
        }
        self.line_feed(p, 2)
    }

    /// `(\n)`, in group `group`: the line feed that ends the line.
    fn line_feed(&mut self, p: usize, group: usize) -> Option<usize> {
        (p == self.lim() && self.nl).then(|| {
            self.cap(group, p, p + 1);
            p + 1
        })
    }

    /// `(~)(["'])`.
    pub(super) fn escaped_string(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b'~' || !matches!(self.at(p + 1), b'"' | b'\'') || p + 1 >= self.lim() {
            return None;
        }
        self.cap(1, p, p + 1);
        self.cap(2, p + 1, p + 2);
        Some(p + 2)
    }

    /// `\\\s*\n`: a backslash and only white space after it on the line.
    pub(super) fn line_continuation(&self, p: usize) -> Option<usize> {
        if self.at(p) != b'\\' || p >= self.lim() || !self.nl {
            return None;
        }
        let end = self.spaces(p + 1);
        (end == self.lim() + 1).then_some(end)
    }

    /// `\\(\h{1,6}|.)`.
    pub(super) fn string_escape(&self, p: usize) -> Option<usize> {
        if self.at(p) != b'\\' || p >= self.lim() {
            return None;
        }
        let hex = self.bounded(p + 1, 6, |b| b.is_ascii_hexdigit());
        if hex > p + 1 {
            return Some(hex);
        }
        (p + 1 < self.lim()).then(|| self.char_end(p + 1))
    }

    /// `[^"'\s]`, or with the bytes of `also` excluded too.
    pub(super) fn unquoted_char(&self, p: usize, also: &[u8]) -> Option<usize> {
        let (_, end) = self.char_at(p)?;
        let b = self.at(p);
        (!matches!(b, b'"' | b'\'') && !also.contains(&b) && self.space_at(p) == 0).then_some(end)
    }

    /// `[-+]?\d+`.
    pub(super) fn integer(&self, p: usize) -> Option<usize> {
        let digits = if sign(self.at(p)) { p + 1 } else { p };
        let end = self.digits(digits);
        (end > digits).then_some(end)
    }

    /// `[-+]?(?:\d+\.\d+|\.?\d+)(?:[Ee][-+]?\d+)?` at `p`.
    ///
    /// A unit or `%` after it never starts with `e` and a digit or sign, so
    /// giving back the exponent or digits never lets one match.
    pub(super) fn number(&self, p: usize) -> Option<usize> {
        let start = if sign(self.at(p)) { p + 1 } else { p };
        let digits = self.digits(start);
        let mantissa =
            if digits > start && self.at(digits) == b'.' && self.digits(digits + 1) > digits + 1 {
                self.digits(digits + 1)
            } else if digits > start {
                digits
            } else if self.at(start) == b'.' && self.digits(start + 1) > start + 1 {
                self.digits(start + 1)
            } else {
                return None;
            };
        Some(self.exponent(mantissa).unwrap_or(mantissa))
    }

    /// The end of `[Ee][-+]?\d+` at `p`.
    fn exponent(&self, p: usize) -> Option<usize> {
        if !matches!(self.at(p), b'E' | b'e') {
            return None;
        }
        let digits = if sign(self.at(p + 1)) { p + 2 } else { p + 1 };
        let end = self.digits(digits);
        (end > digits).then_some(end)
    }

    /// The number of `[-+]?(?:\d+\.\d+|\.?\d+)(?:[Ee][-+]?\d+)?(%)`.
    pub(super) fn percentage(&mut self, p: usize) -> Option<usize> {
        let end = self
            .number(p)
            .filter(|&end| self.at(end) == b'%' && end < self.lim())?;
        self.cap(1, end, end + 1);
        Some(end + 1)
    }

    /// The number of
    /// `…(em|ex|ch|rem|vw|vh|vmin|vmax|cm|mm|[mq]|in|pt|pc|px|fr|dpi|dpcm|dppx|x)`.
    pub(super) fn length(&mut self, p: usize) -> Option<usize> {
        const UNITS: [&[u8]; 21] = [
            b"em", b"ex", b"ch", b"rem", b"vw", b"vh", b"vmin", b"vmax", b"cm", b"mm", b"m", b"q",
            b"in", b"pt", b"pc", b"px", b"fr", b"dpi", b"dpcm", b"dppx", b"x",
        ];
        let number = self.number(p)?;
        let unit = UNITS.iter().find_map(|unit| self.lit(number, unit))?;
        self.cap(1, number, unit);
        Some(unit)
    }

    /// `\b[-+]?0\b`.
    pub(super) fn zero(&self, p: usize) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        let zero = if sign(self.at(p)) { p + 1 } else { p };
        (self.at(zero) == b'0' && self.boundary(zero + 1)).then_some(zero + 1)
    }

    /// `(?i:[-+]?(?:\d*\.\d+(?:[Ee][-+]?\d+)*|[-+]?\d+)(UNIT))\b` for the
    /// units `units`, in the order the alternation tries them, the unit in
    /// group 1. No unit starts with `e`, so the exponents and digits are never
    /// given back. The second alternative, `[-+]?\d+`, takes a second sign
    /// after the first.
    ///
    /// Only a digit, `.`, a second sign or a byte above ASCII, which a digit
    /// of `\d` beyond ASCII starts, can follow the sign, so any other byte
    /// there rules the number out at once.
    pub(super) fn dimension(&mut self, p: usize, units: &[&[u8]]) -> Option<usize> {
        let start = if sign(self.at(p)) { p + 1 } else { p };
        let first = self.at(start);
        if !(first.is_ascii_digit()
            || first == b'.'
            || !first.is_ascii()
            || (start > p && sign(first)))
        {
            return None;
        }
        let digits = self.digits(start);
        let decimal =
            (self.at(digits) == b'.' && self.digits(digits + 1) > digits + 1).then(|| {
                let mut end = self.digits(digits + 1);
                while let Some(next) = self.exponent(end) {
                    end = next;
                }
                end
            });
        let second = if start > p && sign(self.at(start)) {
            start + 1
        } else {
            start
        };
        let integer = Some(self.digits(second)).filter(|&end| end > second);
        let (from, to) = decimal
            .and_then(|end| self.unit_after(end, units))
            .or_else(|| integer.and_then(|end| self.unit_after(end, units)))?;
        self.cap(1, from, to);
        Some(to)
    }

    /// The unit of `units` at `p` as `(?i)` reads it, that `\b` follows
    /// ([`Lexer::folded_end`]).
    fn unit_after(&self, p: usize, units: &[&[u8]]) -> Option<(usize, usize)> {
        units
            .iter()
            .find_map(|unit| self.folded_end(p, unit).filter(|&end| self.boundary(end)))
            .map(|end| (p, end))
    }

    /// `\b((?i:em|ex|ch|rem)|(?i:v(?:[hw]|min|max))|(?i:
    /// cm|mm|q|in|pt|pc|px|fr)|(?i:deg|grad|rad|turn)|(?i:m??s)|(?i:k??Hz)|(?i:
    /// dp(?:i|cm|px)))\b`.
    pub(super) fn unit_name(&self, p: usize) -> Option<usize> {
        const UNITS: [&[u8]; 27] = [
            b"em", b"ex", b"ch", b"rem", b"vh", b"vw", b"vmin", b"vmax", b"cm", b"mm", b"q", b"in",
            b"pt", b"pc", b"px", b"fr", b"deg", b"grad", b"rad", b"turn", b"s", b"ms", b"Hz",
            b"kHz", b"dpi", b"dpcm", b"dppx",
        ];
        if !self.boundary(p) {
            return None;
        }
        self.unit_after(p, &UNITS).map(|(_, end)| end)
    }

    /// `(#)(\h{3}|\h{4}|\h{6}|\h{8})\b`.
    pub(super) fn hex_color(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b'#' {
            return None;
        }
        let run = self.bounded(p + 1, 9, |b| b.is_ascii_hexdigit());
        let end = [3, 4, 6, 8]
            .into_iter()
            .map(|n| p + 1 + n)
            .find(|&end| end <= run && self.boundary(end))?;
        self.cap(1, p, p + 1);
        Some(end)
    }

    /// `(?i)(u\+)([0-9?a-f]{1,6}(?:(-)[0-9a-f]{1,6})?)`.
    pub(super) fn unicode_range(&mut self, p: usize) -> Option<usize> {
        if !matches!(self.at(p), b'u' | b'U') || self.at(p + 1) != b'+' {
            return None;
        }
        let start = p + 2;
        let first = self.bounded(start, 6, |b| b == b'?' || b.is_ascii_hexdigit());
        if first == start {
            return None;
        }
        self.cap(1, p, start);
        let mut end = first;
        if self.at(first) == b'-' {
            let second = self.bounded(first + 1, 6, |b| b.is_ascii_hexdigit());
            if second > first + 1 {
                self.cap(3, first, first + 1);
                end = second;
            }
        }
        self.cap(2, start, end);
        Some(end)
    }

    /// `([-+])?\d+{0,1}(n)(\s*([-+])\s*\d+)?|[-+]?\s*\d+`: the `An+B` of
    /// `:nth-child()`. `\d+{0,1}` repeats `\d+` at most once, so without
    /// the `n` the second alternative decides.
    pub(super) fn an_plus_b(&mut self, p: usize) -> Option<usize> {
        let digits = if sign(self.at(p)) { p + 1 } else { p };
        let a = self.digits(digits);
        if self.at(a) == b'n' && a < self.lim() {
            if digits > p {
                self.cap(1, p, digits);
            }
            self.cap(2, a, a + 1);
            let op = self.spaces(a + 1);
            if sign(self.at(op)) {
                let b = self.spaces(op + 1);
                let end = self.digits(b);
                if end > b {
                    self.cap(4, op, op + 1);
                    return Some(end);
                }
            }
            return Some(a + 1);
        }
        let digits = self.spaces(if sign(self.at(p)) { p + 1 } else { p });
        let end = self.digits(digits);
        (end > digits).then_some(end)
    }

    /// `\s*(%)(?=\()\s*`: `%` as a function.
    pub(super) fn format_function(&mut self, p: usize) -> Option<usize> {
        let at = self.spaces(p);
        if self.at(at) != b'%' || self.at(at + 1) != b'(' || at + 1 >= self.lim() {
            return None;
        }
        self.cap(1, at, at + 1);
        Some(at + 1)
    }

    /// `((pi)(\()(\)))`.
    pub(super) fn pi(&mut self, p: usize) -> Option<usize> {
        let end = self.lit(p, b"pi()")?;
        self.cap(1, p, end);
        self.cap(2, p, p + 2);
        self.cap(3, p + 2, p + 3);
        self.cap(4, p + 3, end);
        Some(end)
    }

    /// `\s*(;|(\.{3})|(?=\)))`.
    pub(super) fn assignment_end(&mut self, p: usize) -> Option<usize> {
        let at = self.spaces(p);
        if self.at(at) == b';' && at < self.lim() {
            self.cap(1, at, at + 1);
            return Some(at + 1);
        }
        if let Some(end) = self.lit(at, b"...") {
            self.cap(1, at, end);
            self.cap(2, at, end);
            return Some(end);
        }
        (self.at(at) == b')').then(|| {
            self.cap(1, at, at);
            at
        })
    }

    /// `\s*(=|(([<>])=?))\s*`.
    pub(super) fn comparison_operator(&mut self, p: usize) -> Option<usize> {
        let at = self.spaces(p);
        let end = match self.at(at) {
            b'=' => at + 1,
            b'<' | b'>' if at < self.lim() => {
                if self.at(at + 1) == b'=' {
                    at + 2
                } else {
                    at + 1
                }
            }
            _ => return None,
        };
        self.cap(1, at, end);
        Some(self.spaces(end))
    }

    /// `(([<>])=?)|[/=]`.
    pub(super) fn comparison(&self, p: usize) -> Option<usize> {
        match self.at(p) {
            b'<' | b'>' if p < self.lim() => {
                Some(if self.at(p + 1) == b'=' { p + 2 } else { p + 1 })
            }
            b'/' | b'=' if p < self.lim() => Some(p + 1),
            _ => None,
        }
    }

    /// `\btrue|false\b`: `\btrue`, or `false\b`.
    pub(super) fn true_or_false(&self, p: usize) -> Option<usize> {
        if self.boundary(p)
            && let Some(end) = self.lit(p, b"true")
        {
            return Some(end);
        }
        self.lit(p, b"false").filter(|&end| self.boundary(end))
    }

    /// `\b((?i)currentColor|transparent)\b`, both words ignoring case.
    pub(super) fn current_color(&self, p: usize) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        [b"currentColor".as_slice(), b"transparent"]
            .into_iter()
            .find_map(|word| self.folded_end(p, word).filter(|&end| self.boundary(end)))
    }

    /// `\b(sans-serif|serif|monospace|fantasy|cursive)\b(?=\s*[\n,;}])`.
    pub(super) fn generic_family(&self, p: usize) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        [
            b"sans-serif".as_slice(),
            b"serif",
            b"monospace",
            b"fantasy",
            b"cursive",
        ]
        .into_iter()
        .filter_map(|word| self.lit(p, word))
        .find(|&end| {
            let next = self.spaces(end);
            self.boundary(end)
                && (matches!(self.at(next), b',' | b';' | b'}') && next < self.lim()
                    || next > self.lim())
        })
    }

    /// `(@)(\{)([-\w]+)(})`.
    pub(super) fn interpolation(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b'@' || self.at(p + 1) != b'{' {
            return None;
        }
        let name = self.dash_word_run(p + 2)?;
        if self.at(name) != b'}' || name >= self.lim() {
            return None;
        }
        self.cap(1, p, p + 1);
        self.cap(2, p + 1, p + 2);
        self.cap(3, p + 2, name);
        self.cap(4, name, name + 1);
        Some(name + 1)
    }

    /// `\s*(@@?)([-\w]+)`.
    pub(super) fn variable(&mut self, p: usize) -> Option<usize> {
        let at = self.spaces(p);
        if self.at(at) != b'@' {
            return None;
        }
        let sigil = if self.at(at + 1) == b'@' {
            at + 2
        } else {
            at + 1
        };
        let end = self.dash_word_run(sigil)?;
        self.cap(1, at, sigil);
        self.cap(2, sigil, end);
        Some(end)
    }
}
