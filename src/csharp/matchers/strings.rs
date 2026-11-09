//! Strings, characters, interpolations and their escapes.

use super::super::engine::Lexer;
use crate::rules::Chars;

/// The literal an escape stands in.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::csharp) enum Literal {
    String,
    Char,
}

impl Lexer<'_> {
    /// The end of the run of `byte` from `p`, which may be empty.
    fn run_of(&self, p: usize, byte: u8) -> usize {
        self.skip(p, |b| b == byte)
    }

    /// `\$+"""""+`, or with `dollars` more dollar signs and `quotes` quotes
    /// at least: the end of the quotes.
    pub(in crate::csharp) fn raw_interpolated_start(
        &self,
        p: usize,
        dollars: usize,
        quotes: usize,
    ) -> Option<usize> {
        let after = self.run_of(p, b'$');
        if after - p < dollars {
            return None;
        }
        let end = self.run_of(after, b'"');
        (end - after >= quotes).then_some(end)
    }

    /// `"""+` with at least `quotes` quotes: the end of the quotes.
    pub(in crate::csharp) fn quotes_at_least(&self, p: usize, quotes: usize) -> Option<usize> {
        let end = self.run_of(p, b'"');
        (end - p >= quotes).then_some(end)
    }

    /// Whether two characters that are not `{` stand before `p`, or `p` is
    /// the start of the line: `(?<=[^{][^{]|^)`.
    fn two_non_braces_before(&self, p: usize) -> bool {
        if p == 0 {
            return true;
        }
        let Some(first) = self.char_before(p) else {
            return false;
        };
        let Some(second) = self.char_before(p - first.len_utf8()) else {
            return false;
        };
        first != '{' && second != '{'
    }

    /// Whether a character that is not `{` stands before `p`, or `p` is the
    /// start of the line: `(?<=[^{]|^)`.
    fn non_brace_before(&self, p: usize) -> bool {
        p == 0 || self.char_before(p).is_some_and(|c| c != '{')
    }

    /// `(?<=[^{][^{]|^)(\{*)(\{\{)(?!\{)`: the whole run of braces is the
    /// match, and its last two are the interpolation's.
    pub(in crate::csharp) fn double_raw_interpolation(&mut self, p: usize) -> Option<usize> {
        if !self.two_non_braces_before(p) {
            return None;
        }
        let end = self.run_of(p, b'{');
        if end - p < 2 {
            return None;
        }
        self.cap(1, p, end - 2);
        self.cap(2, end - 2, end);
        Some(end)
    }

    /// `(?<=[^{]|^)(\{*)(\{)(?!\{)`
    pub(in crate::csharp) fn raw_interpolation(&mut self, p: usize) -> Option<usize> {
        if !self.non_brace_before(p) {
            return None;
        }
        let end = self.run_of(p, b'{');
        if end == p {
            return None;
        }
        self.cap(1, p, end - 1);
        self.cap(2, end - 1, end);
        Some(end)
    }

    /// `(?<=[^{]|^)((?:\{\{)*)(\{)(?!\{)`: a run of braces of odd length,
    /// the last of which opens the interpolation.
    pub(in crate::csharp) fn interpolation(&mut self, p: usize) -> Option<usize> {
        if !self.non_brace_before(p) {
            return None;
        }
        let end = self.run_of(p, b'{');
        if end == p || (end - p).is_multiple_of(2) {
            return None;
        }
        self.cap(1, p, end - 1);
        self.cap(2, end - 1, end);
        Some(end)
    }

    /// `(?:\$@|@\$)"`
    pub(in crate::csharp) fn verbatim_interpolated_start(&self, p: usize) -> Option<usize> {
        let after = self.lit(p, b"$@").or_else(|| self.lit(p, b"@$"))?;
        self.lit(after, b"\"")
    }

    /// `"(?=[^"])`: a quote that no quote follows. The line feed after the
    /// line is a character that is not a quote.
    pub(in crate::csharp) fn verbatim_end(&self, p: usize) -> Option<usize> {
        let end = self.lit(p, b"\"")?;
        (self.avail(end) && self.at(end) != b'"').then_some(end)
    }

    /// `(")|([^\n\\{])$` and its kin.
    /// The closing quote is group 1.
    /// Otherwise the last character of the line is group 2, unless it is one
    /// of `excluded` or the line feed.
    pub(in crate::csharp) fn string_end(
        &mut self,
        p: usize,
        quote: u8,
        excluded: &[u8],
    ) -> Option<usize> {
        if self.at(p) == quote && p < self.lim() {
            self.cap(1, p, p + 1);
            return Some(p + 1);
        }
        let (c, end) = self.cp(p)?;
        if c == '\n' || (c.is_ascii() && excluded.contains(&self.s[p])) || !self.at_eol(end) {
            return None;
        }
        self.cap(2, p, end);
        Some(end)
    }

    /// `\\(x\h{1,4}|U\h{8}|u\h{4}|.)` in a string, and the same without the
    /// `U` form in a character literal: the end of the escape.
    pub(in crate::csharp) fn character_escape(&self, p: usize, literal: Literal) -> Option<usize> {
        let slash = self.lit(p, b"\\")?;
        let hex = |from: usize, most: usize| -> usize {
            let mut at = from;
            while at < from + most && at < self.lim() && self.s[at].is_ascii_hexdigit() {
                at += 1;
            }
            at
        };
        match self.at(slash) {
            b'x' if slash < self.lim() => {
                let end = hex(slash + 1, 4);
                if end > slash + 1 {
                    return Some(end);
                }
            }
            b'U' if literal == Literal::String && slash < self.lim() => {
                if hex(slash + 1, 8) == slash + 9 {
                    return Some(slash + 9);
                }
            }
            b'u' if slash < self.lim() && hex(slash + 1, 4) == slash + 5 => {
                return Some(slash + 5);
            }
            _ => {}
        }
        let (_, end) = self.cp(slash)?;
        Some(end)
    }

    /// `(?<!@)"`
    pub(in crate::csharp) fn string_start(&self, p: usize) -> Option<usize> {
        let end = self.lit(p, b"\"")?;
        (self.prev(p) != Some(b'@')).then_some(end)
    }
}
