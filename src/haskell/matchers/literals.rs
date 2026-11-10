//! Numbers, characters and strings.

use super::super::engine::Lexer;
use super::super::grammar::ESCAPES;
use super::is_name;
use crate::rules::Chars;

/// The single-character escapes after a backslash: `["\&'\\abfnrtv]`.
const SIMPLE_ESCAPES: &[u8] = b"\"&'\\abfnrtv";

impl Lexer<'_> {
    /// `\b(?<!')` at `p` before a digit: no word character and no quote right
    /// before it.
    fn number_start(&self, p: usize) -> bool {
        p < self.lim()
            && self.at(p).is_ascii_digit()
            && !self.word_before(p)
            && !self.prev_is(p, b'\'')
    }

    /// `\b(?!')` at `i` after a word character: no word character and no quote
    /// at it.
    fn number_after(&self, i: usize) -> bool {
        self.word_at(i).is_none() && self.at(i) != b'\''
    }

    /// The end of `[x][x_]*` at `i` for the digit class `digit`, after which
    /// no digit and no `_` stands.
    fn digits(&self, i: usize, digit: fn(u8) -> bool) -> Option<usize> {
        if !(i < self.lim() && digit(self.at(i))) {
            return None;
        }
        Some(self.skip(i + 1, |b| b == b'_' || digit(b)))
    }

    /// The end of `_*\h[_\h]*` at `i`.
    ///
    /// A `_` given back to `_*` leaves `\h` at a `_`, where it fails.
    fn underscored(&self, i: usize, digit: fn(u8) -> bool) -> Option<usize> {
        let first = self.skip(i, |b| b == b'_');
        self.digits(first, digit)
    }

    /// `\b(?<!')(?:([0-9][0-9_]*)|(0[Xx]_*\h[_\h]*)|(0[Oo]_*[0-7][0-7_]*
    /// )|(0[Bb]_*[01][01_]*))\b(?!')`, with a group around the number in each
    /// base.
    ///
    /// No digit given back helps: `\b` never holds between two of them.
    pub(super) fn integer(&mut self, p: usize) -> Option<usize> {
        if !self.number_start(p) {
            return None;
        }
        let decimal = self.digits(p, |b| b.is_ascii_digit())?;
        if self.number_after(decimal) {
            self.capture(1, p, decimal);
            return Some(decimal);
        }
        if self.at(p) != b'0' {
            return None;
        }
        let (group, digit): (usize, fn(u8) -> bool) = match self.at(p + 1) {
            b'X' | b'x' => (2, |b: u8| b.is_ascii_hexdigit()),
            b'O' | b'o' => (3, |b: u8| (b'0'..=b'7').contains(&b)),
            b'B' | b'b' => (4, |b: u8| b == b'0' || b == b'1'),
            _ => return None,
        };
        let end = self.underscored(p + 2, digit)?;
        if !self.number_after(end) {
            return None;
        }
        self.capture(group, p, end);
        Some(end)
    }

    /// The end of `[Ee][-+]?[0-9][0-9_]*`, or of `[Pp]…` for `p`, at `i`.
    fn exponent(&self, i: usize, letters: [u8; 2]) -> Option<usize> {
        if !(i < self.lim() && letters.contains(&self.at(i))) {
            return None;
        }
        let sign = if matches!(self.at(i + 1), b'-' | b'+') {
            i + 2
        } else {
            i + 1
        };
        self.digits(sign, |b| b.is_ascii_digit())
    }

    /// The end of the decimal float at `p`:
    /// `[0-9][0-9_]*\.[0-9][0-9_]*(?:[Ee][-+]?[0-9][0-9_]*)?|[0-9][0-9_]*
    /// [Ee][-+]?[0-9][0-9_]*`.
    fn decimal_float(&self, p: usize) -> Option<usize> {
        let whole = self.digits(p, |b| b.is_ascii_digit())?;
        if self.at(whole) == b'.'
            && let Some(fraction) = self.digits(whole + 1, |b| b.is_ascii_digit())
        {
            return Some(self.exponent(fraction, *b"Ee").unwrap_or(fraction));
        }
        self.exponent(whole, *b"Ee")
    }

    /// The end of the hexadecimal float at `p`:
    /// `0(?:[Xx]_*\h[_\h]*\.\h[_\h]*(?:[Pp][-+]?[0-9][0-9_]*)?|[Xx]_*\h[_\h]*
    /// [Pp][-+]?[0-9][0-9_]*)`.
    fn hex_float(&self, p: usize) -> Option<usize> {
        if !(self.at(p) == b'0' && matches!(self.at(p + 1), b'X' | b'x')) {
            return None;
        }
        let hex = |b: u8| b.is_ascii_hexdigit();
        let whole = self.underscored(p + 2, hex)?;
        if self.at(whole) == b'.'
            && let Some(fraction) = self.digits(whole + 1, hex)
        {
            return Some(self.exponent(fraction, *b"Pp").unwrap_or(fraction));
        }
        self.exponent(whole, *b"Pp")
    }

    /// `\b(?<!')(?:(decimal)|(hexadecimal))\b(?!')` for floats, with a group
    /// around each form.
    ///
    /// An exponent given back leaves its letter after the number, where `\b`
    /// fails, and so does a digit given back.
    pub(super) fn float(&mut self, p: usize) -> Option<usize> {
        if !self.number_start(p) {
            return None;
        }
        if let Some(end) = self.decimal_float(p).filter(|&end| self.number_after(end)) {
            self.capture(1, p, end);
            return Some(end);
        }
        let end = self.hex_float(p).filter(|&end| self.number_after(end))?;
        self.capture(2, p, end);
        Some(end)
    }

    /// The end of a named escape's name or simple escape character at `i`,
    /// for each of `ESCAPES` and then the simple ones, in order.
    fn escape_names(&self, i: usize) -> impl Iterator<Item = usize> + '_ {
        ESCAPES
            .iter()
            .filter_map(move |name| self.lit(i, name))
            .chain((i < self.lim() && SIMPLE_ESCAPES.contains(&self.at(i))).then_some(i + 1))
    }

    /// `(?<!['_\p{Ll}\p{Lu}\p{Lt}\d])(')(?:[
    /// -\[\]-~]|(\\(?:NUL|…|["\&'\\abfnrtv]))|(\\o[0-7]+)|(\\x\h+)|(\\\^[@
    /// -_]))(')`: a character literal, with groups around its quotes and each
    /// kind of escape.
    ///
    /// An escape name is tried in order until a quote follows it, as
    /// backtracking does.
    pub(super) fn char_literal(&mut self, p: usize) -> Option<usize> {
        if !(p < self.lim() && self.at(p) == b'\'') || self.before_is(p, is_name) {
            return None;
        }
        let body = p + 1;
        let close = |l: &Self, at: usize| (at < l.lim() && l.at(at) == b'\'').then_some(at);
        let (group, end) =
            if body < self.lim() && matches!(self.at(body), b' '..=b'[' | b']'..=b'~') {
                (None, close(self, body + 1)?)
            } else if self.lit(body, b"\\").is_some() {
                let after = body + 1;
                if let Some(end) = self.escape_names(after).find_map(|end| close(self, end)) {
                    (Some(2), end)
                } else if self.at(after) == b'o' {
                    let digits = self.skip(after + 1, |b| (b'0'..=b'7').contains(&b));
                    (digits > after + 1).then_some(())?;
                    (Some(3), close(self, digits)?)
                } else if self.at(after) == b'x' {
                    let digits = self.skip(after + 1, |b| b.is_ascii_hexdigit());
                    (digits > after + 1).then_some(())?;
                    (Some(4), close(self, digits)?)
                } else if self.at(after) == b'^'
                    && after + 1 < self.lim()
                    && (b'@'..=b'_').contains(&self.at(after + 1))
                {
                    (Some(5), close(self, after + 2)?)
                } else {
                    return None;
                }
            } else {
                return None;
            };
        self.capture(1, p, body);
        if let Some(group) = group {
            self.capture(group, body, end);
        }
        self.capture(6, end, end + 1);
        Some(end + 1)
    }

    /// `\\(NUL|…|["\&'\\abfnrtv])`: a named escape in a string, with group 1
    /// around the name.
    pub(super) fn named_escape(&mut self, p: usize) -> Option<usize> {
        let after = self.lit(p, b"\\")?;
        let end = self.escape_names(after).next()?;
        self.capture(1, after, end);
        Some(end)
    }

    /// `\\(?:o[0-7]+|x\h+|[0-9]+)`: a numeric escape.
    pub(super) fn numeric_escape(&self, p: usize) -> Option<usize> {
        let after = self.lit(p, b"\\")?;
        let (from, digit): (usize, fn(u8) -> bool) = match self.at(after) {
            b'o' => (after + 1, |b: u8| (b'0'..=b'7').contains(&b)),
            b'x' => (after + 1, |b: u8| b.is_ascii_hexdigit()),
            _ => (after, |b: u8| b.is_ascii_digit()),
        };
        let end = self.skip(from, digit);
        (end > from).then_some(end)
    }

    /// `\\\^[@-_]`: a control escape.
    pub(super) fn control_escape(&self, p: usize) -> Option<usize> {
        let caret = self.lit(p, b"\\^")?;
        (caret < self.lim() && (b'@'..=b'_').contains(&self.at(caret))).then_some(caret + 1)
    }

    /// `\\\s`: the backslash and blank that open a string gap.
    pub(super) fn gap_open(&self, p: usize) -> Option<usize> {
        let blank = self.lit(p, b"\\")?;
        let width = self.space_at(blank);
        (width > 0).then_some(blank + width)
    }

    /// `\S+`.
    pub(super) fn non_blanks(&self, p: usize) -> Option<usize> {
        let end = self.token_end(p);
        (end > p).then_some(end)
    }

    /// The end of `"(?:\\"|[^"])*"` at `i`: the first way it closes.
    pub(super) fn quoted_end(&self, i: usize) -> Option<usize> {
        self.quoted_try(i, |_, end| Some(end))
    }

    /// Each end of `"(?:\\"|[^"])*"` at `i`, in the order backtracking tries
    /// them, until `then` accepts one.
    ///
    /// The first quote that no backslash escapes closes the string first.
    /// Then each escaped quote, from the last to the first, closes it: its
    /// backslash goes to `[^"]` instead.
    pub(super) fn quoted_try<T>(
        &self,
        i: usize,
        mut then: impl FnMut(&Self, usize) -> Option<T>,
    ) -> Option<T> {
        if !(i < self.lim() && self.at(i) == b'"') {
            return None;
        }
        let mut at = i + 1;
        while at < self.lim() {
            match self.at(at) {
                b'"' => break,
                b'\\' if at + 1 < self.lim() && self.at(at + 1) == b'"' => at += 2,
                _ => at += 1,
            }
        }
        self.spend(at - i);
        if at < self.lim()
            && let Some(found) = then(self, at + 1)
        {
            return Some(found);
        }
        let mut quote = at;
        while quote > i + 1 {
            quote -= 1;
            if self.at(quote) == b'"'
                && self.at(quote - 1) == b'\\'
                && let Some(found) = then(self, quote + 1)
            {
                return Some(found);
            }
        }
        None
    }

    /// `"(?:\\"|[^"])*"`: a quoted string.
    pub(super) fn quoted(&self, p: usize) -> Option<usize> {
        self.quoted_end(p)
    }

    /// The end of the run of characters from `i` that are not `\s`.
    pub(super) fn token_end(&self, i: usize) -> usize {
        let mut at = i;
        while at < self.lim() && self.space_at(at) == 0 {
            at = self.char_end(at);
        }
        self.spend(at - i);
        at
    }
}
