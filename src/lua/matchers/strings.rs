//! Strings: quoted, long-bracketed and `ffi.cdef` ones, and their escapes.

use super::super::engine::Lexer;
use crate::rules::Chars;

impl Lexer<'_> {
    /// `quote[\t ]*|(?=\n)`: the closing quote and the blanks after it, or
    /// nothing before the line feed.
    pub(super) fn quote_end(&self, p: usize, quote: u8) -> Option<usize> {
        if self.at(p) == quote && p < self.lim() {
            return Some(self.tabs(p + 1));
        }
        (self.avail(p) && self.at(p) == b'\n').then_some(p)
    }

    /// `\[(=*)\[` at `p`, with the `=` signs as capture group `group`: the
    /// end of the bracket.
    pub(super) fn long_bracket(&mut self, p: usize, group: usize) -> Option<usize> {
        let open = self.lit(p, b"[")?;
        let equals = self.skip(open, |b| b == b'=');
        let end = self.lit(equals, b"[")?;
        self.cap(group, open, equals);
        Some(end)
    }

    /// `]\N]` at `p`, where `\N` is the `=` signs the rule opened with, which
    /// may be none: the end of the bracket.
    pub(super) fn long_bracket_close(&self, p: usize) -> Option<usize> {
        let inner = self.lit(p, b"]")?;
        let equals = self.payload_text().as_bytes();
        let after = if equals.is_empty() {
            inner
        } else {
            self.lit(inner, equals)?
        };
        self.lit(after, b"]")
    }

    /// `(?<=\.cdef)\s*(\[(=*)\[)`
    pub(super) fn cdef_open(&mut self, p: usize) -> Option<usize> {
        if p < 5 || self.text(p - 5, p) != b".cdef" {
            return None;
        }
        let open = self.space_end(p);
        let end = self.long_bracket(open, 2)?;
        self.cap(1, open, end);
        Some(end)
    }

    /// `(]\2])[\t ]*`
    pub(super) fn cdef_close(&mut self, p: usize) -> Option<usize> {
        let end = self.long_bracket_close(p)?;
        self.cap(1, p, end);
        Some(self.tabs(end))
    }

    /// `(?<!--)\[(=*)\[`
    pub(super) fn long_string_open(&mut self, p: usize) -> Option<usize> {
        if p >= 2 && self.text(p - 2, p) == b"--" {
            return None;
        }
        self.long_bracket(p, 1)
    }

    /// `]\1][\t ]*`
    pub(super) fn long_string_close(&self, p: usize) -> Option<usize> {
        Some(self.tabs(self.long_bracket_close(p)?))
    }

    /// `\\[\n"'\\abfnrtv]`, where the line feed after the line counts.
    pub(super) fn simple_escape(&self, p: usize) -> Option<usize> {
        let slash = self.lit(p, b"\\")?;
        (self.avail(slash) && b"\n\"'\\abfnrtv".contains(&self.at(slash))).then_some(slash + 1)
    }

    /// `\\z[\t\n ]*`, which takes the line feed after the line too.
    pub(super) fn skip_escape(&self, p: usize) -> Option<usize> {
        let end = self.tabs(self.lit(p, b"\\z")?);
        Some(self.line_feed(end).unwrap_or(end))
    }

    /// `\\\d{1,3}`: up to three digits of any script.
    pub(super) fn decimal_escape(&self, p: usize) -> Option<usize> {
        let mut end = self.lit(p, b"\\")?;
        for _ in 0..3 {
            match self.char_at(end) {
                Some((c, next)) if end < self.lim() && crate::classes::is_digit(c) => end = next,
                _ => break,
            }
        }
        (end > p + 1).then_some(end)
    }

    /// `\\x\h\h`
    pub(super) fn hex_escape(&self, p: usize) -> Option<usize> {
        let digits = self.lit(p, b"\\x")?;
        let end = self.skip(digits, |b| b.is_ascii_hexdigit()).min(digits + 2);
        (end == digits + 2).then_some(end)
    }

    /// `\\u\{\h+}`
    pub(super) fn unicode_escape(&self, p: usize) -> Option<usize> {
        let digits = self.lit(p, b"\\u{")?;
        let end = self.skip(digits, |b| b.is_ascii_hexdigit());
        if end == digits {
            return None;
        }
        self.lit(end, b"}")
    }

    /// `\\.`: a backslash and any character but the line feed.
    pub(super) fn any_escape(&self, p: usize) -> Option<usize> {
        let slash = self.lit(p, b"\\")?;
        if slash >= self.lim() {
            return None;
        }
        self.char_at(slash).map(|(_, end)| end)
    }
}
