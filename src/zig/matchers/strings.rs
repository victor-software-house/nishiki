//! Escapes and character literals.

use super::Lexer;

/// Whether `b` is an octal digit, `[0-7]`.
fn is_octal(b: u8) -> bool {
    matches!(b, b'0'..=b'7')
}

impl Lexer<'_> {
    /// `\\(["'\\nrt]|(x\h{2})|(u\{\h+}))`: an escape the language knows.
    /// The `\h+` stops at the first byte that is not hex, which must be `}`.
    pub(super) fn escape(&self, p: usize) -> Option<usize> {
        if self.at(p) != b'\\' {
            return None;
        }
        let q = p + 1;
        match self.at(q) {
            b'"' | b'\'' | b'\\' | b'n' | b'r' | b't' => Some(q + 1),
            b'x' => (self.at(q + 1).is_ascii_hexdigit() && self.at(q + 2).is_ascii_hexdigit())
                .then_some(q + 3),
            b'u' if self.at(q + 1) == b'{' && self.at(q + 2).is_ascii_hexdigit() => {
                let close = self.skip(q + 2, |b| b.is_ascii_hexdigit());
                (self.at(close) == b'}').then_some(close + 1)
            }
            _ => None,
        }
    }

    /// A character literal:
    ///
    /// ```text
    /// '([^'\\]|\\(x\h{2}|[012][0-7]{0,2}|3[0-6][0-7]?|37[0-7]?|[4-7][0-7]?|.))'
    /// ```
    ///
    /// The escape's branches can end at several places, and the closing `'`
    /// decides among them.
    /// [`Lexer::escape_ends`] lists those places in the order Oniguruma tries
    /// them.
    pub(super) fn char_literal(&self, p: usize) -> Option<usize> {
        if self.at(p) != b'\'' {
            return None;
        }
        let q = p + 1;
        match self.at(q) {
            b'\\' => self
                .escape_ends(q + 1)
                .into_iter()
                .flatten()
                .find(|&end| self.at(end) == b'\'')
                .map(|end| end + 1),
            b'\'' => None,
            _ if q < self.lim() => {
                let end = self.char_end(q);
                (self.at(end) == b'\'').then_some(end + 1)
            }
            _ => None,
        }
    }

    /// Where each branch of
    /// `x\h{2}|[012][0-7]{0,2}|3[0-6][0-7]?|37[0-7]?|[4-7][0-7]?|.`
    /// can end, from `q`, in the order a backtracking matcher tries them.
    /// A greedy count tries its longest run first.
    fn escape_ends(&self, q: usize) -> [Option<usize>; 5] {
        let b = self.at(q);
        let octal = |i: usize| is_octal(self.at(i));
        let x =
            (b == b'x' && self.at(q + 1).is_ascii_hexdigit() && self.at(q + 2).is_ascii_hexdigit())
                .then_some(q + 3);
        let (long, mid, short) = match b {
            b'0'..=b'2' => (
                (octal(q + 1) && octal(q + 2)).then_some(q + 3),
                octal(q + 1).then_some(q + 2),
                Some(q + 1),
            ),
            b'3' if matches!(self.at(q + 1), b'0'..=b'7') => {
                (octal(q + 2).then_some(q + 3), Some(q + 2), None)
            }
            b'4'..=b'7' => (None, octal(q + 1).then_some(q + 2), Some(q + 1)),
            _ => (None, None, None),
        };
        let any = (q < self.lim()).then(|| self.char_end(q));
        [x, long, mid, short, any]
    }
}
