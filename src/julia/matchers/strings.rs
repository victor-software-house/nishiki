//! Strings, string macros, commands and their escapes and interpolations.

use super::super::chars::interpolated_start;
use super::super::engine::Lexer;
use crate::classes::{is_digit, is_space, is_word};
use crate::rules::Chars;

/// The quotes of a triple-quoted string.
const TRIPLE: &[u8] = b"\"\"\"";

impl Lexer<'_> {
    /// `(@doc)\s((?:doc)?""")|(doc""")`: a docstring that `@doc` or `doc`
    /// leads.
    /// A `doc` that no `"""` follows is given back.
    pub(super) fn doc_macro(&mut self, p: usize) -> Option<usize> {
        if self.starts(p, b"@doc") {
            let space = self.space_at(p + 4);
            let open = p + 4 + space;
            let quotes = if self.starts(open, b"doc") && self.starts(open + 3, TRIPLE) {
                open + 3
            } else {
                open
            };
            if space > 0 && self.starts(quotes, TRIPLE) {
                self.capture(1, p, p + 4);
                self.capture(2, open, quotes + 3);
                return Some(quotes + 3);
            }
        }
        self.lit(p, b"doc\"\"\"")
    }

    /// `(""") ?(->)?`: group 1 is the quotes, group 2 the arrow.
    pub(super) fn doc_macro_end(&mut self, p: usize) -> Option<usize> {
        let quotes = self.lit(p, TRIPLE)?;
        self.capture(1, p, quotes);
        let arrow = quotes + usize::from(self.at(quotes) == b' ');
        match self.lit(arrow, b"->") {
            Some(end) => {
                self.capture(2, arrow, end);
                Some(end)
            }
            None => Some(arrow),
        }
    }

    /// `^\s?(doc)?(""")\s?$`: a line that only opens a docstring.
    /// The `\s?` at the end can take the line feed, and then the match ends
    /// after it.
    pub(super) fn docstring(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let open = self.space_at(p);
        let quotes = if self.starts(open, b"doc") && self.starts(open + 3, TRIPLE) {
            self.capture(1, open, open + 3);
            open + 3
        } else {
            open
        };
        let end = self.lit(quotes, TRIPLE)?;
        self.capture(2, quotes, end);
        let space = self.space_at(end);
        if space > 0 && self.at_eol(end + space) {
            return Some(end + space);
        }
        self.at_eol(end).then_some(end)
    }

    /// `\\(\\|[0-3]\d{0,2}|[4-7]\d?|x\h{0,2}|u\h{0,4}|U\h{0,8}|.)`: an escape.
    /// `\d` takes any decimal digit, as Oniguruma's does.
    pub(super) fn escape(&self, p: usize) -> Option<usize> {
        if self.at(p) != b'\\' || p + 1 >= self.lim() {
            return None;
        }
        let q = p + 1;
        Some(match self.at(q) {
            b'0'..=b'3' => self.digits(q + 1, 2),
            b'4'..=b'7' => self.digits(q + 1, 1),
            b'x' => self.hex(q + 1, 2),
            b'u' => self.hex(q + 1, 4),
            b'U' => self.hex(q + 1, 8),
            _ => self.char_end(q),
        })
    }

    /// The end of up to `most` characters `\d` holds from `p`.
    fn digits(&self, mut p: usize, most: usize) -> usize {
        for _ in 0..most {
            match self.char_at(p) {
                Some((c, end)) if is_digit(c) => p = end,
                _ => break,
            }
        }
        p
    }

    /// The end of up to `most` hexadecimal digits from `p`.
    fn hex(&self, mut p: usize, most: usize) -> usize {
        for _ in 0..most {
            if !self.at(p).is_ascii_hexdigit() {
                break;
            }
            p += 1;
        }
        p
    }

    /// `\$(\()`: an interpolated expression, whose parenthesis is group 1.
    pub(super) fn interpolation(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b'$' || self.at(p + 1) != b'(' {
            return None;
        }
        self.capture(1, p + 1, p + 2);
        Some(p + 2)
    }

    /// `\$START REST*`, with `$` left out of both classes: an interpolated
    /// name, which a `$` ends.
    pub(super) fn interpolated_name(&self, p: usize) -> Option<usize> {
        if self.at(p) != b'$' {
            return None;
        }
        let start = interpolated_start(self.s, p + 1)?;
        let rest = self.rest_end(start);
        let dollar = memchr::memchr(b'$', self.s.get(start..rest)?);
        Some(dollar.map_or(rest, |k| start + k))
    }

    /// `(prefix)(quotes)`: a string macro the grammar names, whose name is
    /// group 1 and quotes group 2.
    pub(super) fn prefixed(&mut self, p: usize, prefix: &[u8], quotes: &[u8]) -> Option<usize> {
        let open = self.lit(p, prefix)?;
        let end = self.lit(open, quotes)?;
        self.capture(1, p, open);
        self.capture(2, open, end);
        Some(end)
    }

    /// `(i?cxx)(""")`: `icxx` gives its `i` back only for `cxx` to start at
    /// it, which cannot happen.
    pub(super) fn cxx(&mut self, p: usize) -> Option<usize> {
        let name = p + usize::from(self.at(p) == b'i');
        let open = self.lit(name, b"cxx")?;
        let end = self.lit(open, TRIPLE)?;
        self.capture(1, p, open);
        self.capture(2, open, end);
        Some(end)
    }

    /// `([\s\w]*)(""")`: the end of `py"""`, which takes the spaces and words
    /// before the quotes.
    /// Group 2 is the quotes.
    pub(super) fn python_end(&mut self, p: usize) -> Option<usize> {
        let mut q = p;
        while let Some((c, end)) = self.char_at(q)
            && (is_space(c) || is_word(c))
        {
            q = end;
        }
        self.spend(q - p);
        let end = self.lit(q, TRIPLE)?;
        self.capture(2, q, end);
        Some(end)
    }

    /// The number of backslashes from `p`.
    fn backslashes(&self, p: usize) -> usize {
        self.skip(p, |b| b == b'\\') - p
    }

    /// `(\\\\)*\\"`: an odd run of backslashes and a quote.
    /// An even run cannot match from its first backslash.
    pub(super) fn raw_escaped_quote(&self, p: usize) -> Option<usize> {
        let run = self.backslashes(p);
        (!run.is_multiple_of(2) && self.at(p + run) == b'"').then_some(p + run + 1)
    }

    /// `(\\\\)+(?=")`: an even run of backslashes before a quote.
    /// An odd run cannot match from its first backslash.
    pub(super) fn raw_escaped_backslashes(&self, p: usize) -> Option<usize> {
        let run = self.backslashes(p);
        (run > 0 && run.is_multiple_of(2) && self.at(p + run) == b'"').then_some(p + run)
    }

    /// `'(?!')`.
    pub(super) fn char_quote_end(&self, p: usize) -> Option<usize> {
        (self.at(p) == b'\'' && self.at(p + 1) != b'\'').then_some(p + 1)
    }

    /// `"(?!"")`.
    pub(super) fn quoted(&self, p: usize) -> Option<usize> {
        (self.at(p) == b'"' && !self.starts(p + 1, b"\"\"")).then_some(p + 1)
    }

    /// `(quotes)([imsx]{0,4})?`: the end of a regex, whose quotes are group 1
    /// and flags group 2.
    pub(super) fn regex_end(&mut self, p: usize, quotes: &[u8]) -> Option<usize> {
        let flags = self.lit(p, quotes)?;
        let mut end = flags;
        while end - flags < 4 && matches!(self.at(end), b'i' | b'm' | b's' | b'x') {
            end += 1;
        }
        self.capture(1, p, flags);
        self.capture(2, flags, end);
        Some(end)
    }

    /// `(?<!")(IDENT)quotes`: a string macro, whose name is group 1.
    pub(super) fn string_macro(&mut self, p: usize, quotes: &[u8]) -> Option<usize> {
        if p > 0 && self.at(p - 1) == b'"' {
            return None;
        }
        let name = self.ident(p)?;
        let end = self.lit(name, quotes)?;
        self.capture(1, p, name);
        Some(end)
    }

    /// `(quotes)(IDENT)?`: the end of a string macro or a command, whose
    /// quotes are group 1 and suffix group 2.
    pub(super) fn macro_end(&mut self, p: usize, quotes: &[u8]) -> Option<usize> {
        let suffix = self.lit(p, quotes)?;
        self.capture(1, p, suffix);
        match self.ident(suffix) {
            Some(end) => {
                self.capture(2, suffix, end);
                Some(end)
            }
            None => Some(suffix),
        }
    }

    /// `(?<![^\\]\\)(X)(IDENT)?`: [`Lexer::macro_end`] for one quote that a
    /// backslash escapes, unless a backslash escapes that one.
    pub(super) fn escapable_macro_end(&mut self, p: usize, quote: u8) -> Option<usize> {
        let escaped = p >= 2 && self.at(p - 1) == b'\\' && self.at(p - 2) != b'\\';
        if escaped {
            return None;
        }
        self.macro_end(p, &[quote])
    }

    /// ``(?<!`)(IDENT)?quotes``: a command, whose prefix is group 1.
    /// A prefix that no quotes follow is given back, and the quotes cannot
    /// start at its first character.
    pub(super) fn command(&mut self, p: usize, quotes: &[u8]) -> Option<usize> {
        if p > 0 && self.at(p - 1) == b'`' {
            return None;
        }
        if let Some(name) = self.ident(p) {
            let end = self.lit(name, quotes)?;
            self.capture(1, p, name);
            return Some(end);
        }
        self.lit(p, quotes)
    }
}
