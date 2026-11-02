//! The text of strings, up to their escapes and closing quotes.

use memchr::{memchr, memchr2};

use super::lexer::Lexer;
use super::scope::S;
use super::state::Kind;

impl Lexer<'_> {
    /// Takes the text of a `"` or `"""` string up to its next escape or
    /// closing quote.
    pub(super) fn in_basic(&mut self, p: usize) -> usize {
        let multi = self.top() == Kind::BasicMulti;
        let Some(offset) = memchr2(b'"', b'\\', &self.s[p..]) else {
            self.spend(self.s.len() - p);
            return self.s.len();
        };
        let q = p + offset;
        self.spend(offset);
        if self.s[q] == b'"' {
            return if multi {
                self.multi_quote(q, b'"')
            } else {
                self.finish(q, q + 1, S::PunctStrEnd)
            };
        }
        match self.escape(q) {
            Some((len, scope)) => {
                self.tok_at(q, q + len, &[scope]);
                q + len
            }
            // A backslash that ends a `"""` line takes the next line's.
            None if multi => self.s.len(),
            None => {
                self.tok_at(q, q + 1, &[S::InvalidEscape]);
                q + 1
            }
        }
    }

    /// Takes the text of a `'` or `'''` string up to its closing quote.
    pub(super) fn in_literal(&mut self, p: usize) -> usize {
        let Some(offset) = memchr(b'\'', &self.s[p..]) else {
            self.spend(self.s.len() - p);
            return self.s.len();
        };
        let q = p + offset;
        self.spend(offset);
        if self.top() == Kind::LiteralMulti {
            self.multi_quote(q, b'\'')
        } else {
            self.finish(q, q + 1, S::PunctStrEnd)
        }
    }

    /// A run of `quote` at `q` in a triple-quoted string: three to five of
    /// them close it, and fewer are text.
    fn multi_quote(&mut self, q: usize, quote: u8) -> usize {
        let mut end = q;
        while self.at(end) == quote {
            end += 1;
        }
        self.spend(end - q);
        if end - q < 3 {
            return end;
        }
        self.finish(q, q + (end - q).min(5), S::PunctStrEnd)
    }

    /// The escape at the backslash at `q` in a `"` string: its length and
    /// scope, or `None` when the backslash ends the line.
    ///
    /// `\\(?:[btnfr"\\]|u\h{4}|U\h{8})` is an escape; a backslash and
    /// the character after it are an invalid one otherwise.
    pub(super) fn escape(&self, q: usize) -> Option<(usize, S)> {
        let hex = |digits: usize| (2..2 + digits).all(|k| self.at(q + k).is_ascii_hexdigit());
        match self.at(q + 1) {
            b'b' | b't' | b'n' | b'f' | b'r' | b'"' | b'\\' => Some((2, S::Escape)),
            b'u' if hex(4) => Some((6, S::Escape)),
            b'U' if hex(8) => Some((10, S::Escape)),
            _ if q + 1 < self.s.len() => Some((1 + self.width_at(q + 1), S::InvalidEscape)),
            _ => None,
        }
    }
}
