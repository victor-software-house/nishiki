//! Comments, escapes and strings.

use super::super::engine::Lexer;
use crate::rules::Chars;

impl Lexer<'_> {
    /// `/\*`
    pub(in crate::css) fn comment_block(&mut self, p: usize) -> Option<usize> {
        self.lit(p, b"/*")
    }

    /// `\*/`
    pub(in crate::css) fn comment_block_end(&mut self, p: usize) -> Option<usize> {
        self.lit(p, b"*/")
    }

    /// `\\\h{1,6}`
    pub(in crate::css) fn escapes_at0(&mut self, p: usize) -> Option<usize> {
        self.hex_escape_end(p)
    }

    /// `\\$\s*`: a backslash that ends the line, and the line feed.
    pub(in crate::css) fn escapes_at1(&mut self, p: usize) -> Option<usize> {
        (self.s.get(p) == Some(&b'\\') && p + 1 == self.lim()).then(|| self.spaces(p + 1))
    }

    /// `^(?<!\G)`: the start of a line that is not where `\G` is.
    pub(in crate::css) fn escapes_at1_end(&mut self, p: usize) -> Option<usize> {
        (p == 0 && !(self.g && p == self.pos)).then_some(p)
    }

    /// `\\.`
    pub(in crate::css) fn escapes_at2(&mut self, p: usize) -> Option<usize> {
        (self.s.get(p) == Some(&b'\\') && p + 1 < self.lim()).then(|| self.char_end(p + 1))
    }

    /// The quote `quote`, as a string begins.
    fn quote(&self, p: usize, quote: u8) -> Option<usize> {
        (self.s.get(p) == Some(&quote)).then_some(p + 1)
    }

    /// `"|(?<!\\)(?=$|\n)` for the quote `quote`: the closing quote, or the
    /// end of a line that does not end in a backslash.
    fn string_closing(&self, p: usize, quote: u8) -> Option<usize> {
        if let Some(end) = self.quote(p, quote) {
            return Some(end);
        }
        let lim = self.lim();
        let ends = (p == lim && !(p > 0 && self.s.get(p - 1) == Some(&b'\\'))) || p == lim + 1;
        ends.then_some(p)
    }

    /// `(?:\G|^)(?=(?:[^"\\]|\\.)+$)` for the quote `quote`: where `\G` or
    /// the line start is, when the rest of the line has no unescaped quote.
    fn string_unclosed(&self, p: usize, quote: u8) -> Option<usize> {
        if !((self.g && p == self.pos) || p == 0) {
            return None;
        }
        let lim = self.lim();
        let mut i = p;
        while i < lim {
            let b = self.s[i];
            if b == quote {
                return None;
            }
            if b == b'\\' {
                if i + 1 >= lim {
                    return None;
                }
                i = self.char_end(i + 1);
            } else {
                i = self.char_end(i);
            }
        }
        self.spend(lim - p + 1);
        Some(p)
    }

    /// `"`
    pub(in crate::css) fn string_at0(&mut self, p: usize) -> Option<usize> {
        self.quote(p, b'"')
    }

    /// `"|(?<!\\)(?=$|\n)`
    pub(in crate::css) fn string_at0_end(&mut self, p: usize) -> Option<usize> {
        self.string_closing(p, b'"')
    }

    /// `(?:\G|^)(?=(?:[^"\\]|\\.)+$)`
    pub(in crate::css) fn string_at0_at0(&mut self, p: usize) -> Option<usize> {
        self.string_unclosed(p, b'"')
    }

    /// `$`
    pub(in crate::css) fn string_at0_at0_end(&mut self, p: usize) -> Option<usize> {
        self.at_eol(p).then_some(p)
    }

    /// `'`
    pub(in crate::css) fn string_at1(&mut self, p: usize) -> Option<usize> {
        self.quote(p, b'\'')
    }

    /// `'|(?<!\\)(?=$|\n)`
    pub(in crate::css) fn string_at1_end(&mut self, p: usize) -> Option<usize> {
        self.string_closing(p, b'\'')
    }

    /// `(?:\G|^)(?=(?:[^'\\]|\\.)+$)`
    pub(in crate::css) fn string_at1_at0(&mut self, p: usize) -> Option<usize> {
        self.string_unclosed(p, b'\'')
    }
}
