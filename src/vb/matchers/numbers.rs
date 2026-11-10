//! Numbers, the expression of
//! [`Pattern::Numbers`](super::super::grammar::Pattern::Numbers).

use super::super::engine::Lexer;
use crate::rules::Chars;

impl Lexer<'_> {
    /// A number at `p`, with its sign.
    ///
    /// Without the sign, `\b` before a digit needs a non-word before it, and
    /// before a `.` it needs a word before it.
    /// With the sign, `\b` needs a digit after it: giving the sign back leaves
    /// `-` where a digit or a `.` must stand.
    pub(super) fn number(&self, p: usize) -> Option<usize> {
        if self.starts(p, b"-") {
            let start = p + 1;
            return self
                .at(start)
                .is_ascii_digit()
                .then(|| self.unsigned(start))
                .flatten();
        }
        if !self.boundary(p) {
            return None;
        }
        self.unsigned(p)
    }

    /// The number after the sign at `s`, where `\b` holds.
    ///
    /// The final `\b` decides which end wins.
    /// An end inside a run of digits has a digit on each side, where `\b`
    /// fails.
    /// So a run gives back either nothing or all its digits.
    /// So `1.5é` reads as `1.`: after `5` the `é` breaks `\b`, and after the
    /// bare `.` it holds.
    fn unsigned(&self, s: usize) -> Option<usize> {
        if self.starts(s, b"0") && matches!(self.at(s + 1), b'X' | b'x') && s + 1 < self.lim() {
            let hex = self.skip(s + 2, |b| b.is_ascii_hexdigit());
            if let Some(end) = self.suffixed(hex) {
                return Some(end);
            }
        }
        if self.at(s).is_ascii_digit() && s < self.lim() {
            let int = self.skip(s, |b| b.is_ascii_digit());
            if self.starts(int, b".") {
                let fraction = self.skip(int + 1, |b| b.is_ascii_digit());
                if fraction > int + 1
                    && let Some(end) = self.exponent_then_suffix(fraction)
                {
                    return Some(end);
                }
                if let Some(end) = self.exponent_then_suffix(int + 1) {
                    return Some(end);
                }
            }
            return self.exponent_then_suffix(int);
        }
        if self.starts(s, b".") && self.at(s + 1).is_ascii_digit() && s + 1 < self.lim() {
            let fraction = self.skip(s + 1, |b| b.is_ascii_digit());
            return self.exponent_then_suffix(fraction);
        }
        None
    }

    /// `(([Ee])([-+])?[0-9]+)?` and then the suffix and `\b`, after the
    /// mantissa that ends at `m`.
    /// The exponent's digits give back all or nothing, as the mantissa's do.
    fn exponent_then_suffix(&self, m: usize) -> Option<usize> {
        if m < self.lim() && matches!(self.at(m), b'E' | b'e') {
            let digits = if matches!(self.at(m + 1), b'+' | b'-') && m + 1 < self.lim() {
                m + 2
            } else {
                m + 1
            };
            let end = self.skip(digits, |b| b.is_ascii_digit());
            if end > digits
                && let Some(found) = self.suffixed(end)
            {
                return Some(found);
            }
        }
        self.suffixed(m)
    }

    /// `([Ll]|UL|ul|[FUfu])?\b` at `e`: the suffixes in order, then none.
    fn suffixed(&self, e: usize) -> Option<usize> {
        let one = (e < self.lim() && matches!(self.at(e), b'L' | b'l')).then_some(e + 1);
        let two = self.lit(e, b"UL").or_else(|| self.lit(e, b"ul"));
        let other =
            (e < self.lim() && matches!(self.at(e), b'F' | b'U' | b'f' | b'u')).then_some(e + 1);
        [one, two, other, Some(e)]
            .into_iter()
            .flatten()
            .find(|&end| self.boundary(end))
    }
}
