//! Strings, interpolation and character literals.

use super::super::engine::Lexer;
use super::names::{is_name_rest, is_name_start};
use crate::classes::{is_lower_or_other_letter, is_title_or_upper};
use crate::rules::Chars;

/// Whether an interpolated string's `$` may be followed by `c` without ending
/// the string as an invalid escape:
/// `["$A-Z_a-{\p{Lt}\p{Lu}\p{Lo}\p{Nl}\p{Ll}]`.
///
/// `a-{` runs from `a` to `{`, so it holds the lower-case letters and the
/// `{` of `${`.
fn may_follow_dollar(c: char) -> bool {
    matches!(c, '"' | '$' | '_' | 'a'..='{') || is_title_or_upper(c) || is_lower_or_other_letter(c)
}

impl Lexer<'_> {
    /// `"""(?!")`.
    pub(super) fn triple_quote_end(&self, p: usize) -> Option<usize> {
        self.lit(p, b"\"\"\"").filter(|&end| self.at(end) != b'"')
    }

    /// `\b(raw)(""")` or `\b(raw)(")`, with `quote` the quotes.
    pub(super) fn raw_string(&mut self, p: usize, quote: &[u8]) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        let name = self.lit(p, b"raw")?;
        let end = self.lit(name, quote)?;
        self.capture(1, p, name);
        self.capture(2, name, end);
        Some(end)
    }

    /// `\b(plain)(""")` or `\b(plain)(")`, with `quote` the quotes: a string
    /// with a name before it.
    ///
    /// The name's operator tail holds no `"`.
    /// Giving any character back leaves a name or operator character before
    /// the quotes, so one end decides.
    pub(super) fn interpolated_string(&mut self, p: usize, quote: &[u8]) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        let name = self.plain_name(p)?;
        let end = self.lit(name, quote)?;
        self.capture(1, p, name);
        self.capture(2, name, end);
        Some(end)
    }

    /// The end of an interpolated string, `quote` its closing quotes:
    /// `(""")(?!")|\$\n|(\$[^"$A-Z_a-{…}])` or `(")|\$\n|(\$[^"$A-Z_a-{…}])`.
    ///
    /// `[^…]` takes any other character, the line feed included, but
    /// `\$\n` comes first.
    pub(super) fn interpolated_string_end(&mut self, p: usize, quote: &[u8]) -> Option<usize> {
        if let Some(end) = self.lit(p, quote) {
            if quote.len() == 1 || self.at(end) != b'"' {
                self.capture(1, p, end);
                return Some(end);
            }
            return None;
        }
        if self.at(p) != b'$' {
            return None;
        }
        if p + 1 == self.lim() {
            return self.nl.then_some(p + 2);
        }
        let (c, end) = self.char_at(p + 1)?;
        if may_follow_dollar(c) {
            return None;
        }
        self.capture(2, p, end);
        Some(end)
    }

    /// `\$["$]`.
    pub(super) fn dollar_escape(&self, p: usize) -> Option<usize> {
        (self.at(p) == b'$' && matches!(self.at(p + 1), b'"' | b'$')).then_some(p + 2)
    }

    /// `(\$)([$A-Z_a-z\p{Lt}\p{Lu}\p{Lo}\p{Nl}\p{Ll}][0-9A-Z_a-z\p{Lt}\p{Lu}\
    /// p{Lo}\p{Nl}\p{Ll}]*)`: the name after `$` takes no `$` after its first
    /// character.
    pub(super) fn interpolated_name(&mut self, p: usize) -> Option<usize> {
        let dollar = self.lit(p, b"$")?;
        let (c, next) = self.char_at(dollar)?;
        if !is_name_start(c) {
            return None;
        }
        let end = self.chars_while(next, |c| c != '$' && is_name_rest(c));
        self.capture(1, p, dollar);
        Some(end)
    }

    /// `\\(?:\\|u\h{4})`.
    pub(super) fn triple_escape(&self, p: usize) -> Option<usize> {
        let slash = self.lit(p, b"\\")?;
        match self.at(slash) {
            b'\\' => Some(slash + 1),
            b'u' => self.hex4(slash + 1),
            _ => None,
        }
    }

    /// `\\(?:["'\\bfnrt]|[0-7]{1,3}|u\h{4})`.
    pub(super) fn escape(&self, p: usize) -> Option<usize> {
        let slash = self.lit(p, b"\\")?;
        match self.at(slash) {
            b'"' | b'\'' | b'\\' | b'b' | b'f' | b'n' | b'r' | b't' => Some(slash + 1),
            b'0'..=b'7' => {
                let mut end = slash + 1;
                while end < slash + 3 && matches!(self.at(end), b'0'..=b'7') {
                    end += 1;
                }
                Some(end)
            }
            b'u' => self.hex4(slash + 1),
            _ => None,
        }
    }

    /// The end of `\h{4}` at `i`.
    fn hex4(&self, i: usize) -> Option<usize> {
        (i..i + 4)
            .all(|j| self.at(j).is_ascii_hexdigit())
            .then_some(i + 4)
    }

    /// `\\.`: a backslash and any character but the line feed.
    pub(super) fn any_escape(&self, p: usize) -> Option<usize> {
        let slash = self.lit(p, b"\\")?;
        (slash < self.lim()).then(|| self.char_end(slash))
    }

    /// `.`: any character but the line feed.
    pub(super) fn any_char(&self, p: usize) -> Option<usize> {
        (p < self.lim()).then(|| self.char_end(p))
    }

    /// `'|$`.
    pub(super) fn char_literal_end(&self, p: usize) -> Option<usize> {
        if self.at(p) == b'\'' && p < self.lim() {
            return Some(p + 1);
        }
        self.at_eol(p).then_some(p)
    }

    /// The end of the run of characters from `i` that are not `'`, the line
    /// feed after the text included.
    fn not_quote_end(&self, i: usize) -> usize {
        let mut end = self.skip(i, |b| b != b'\'');
        if end == self.lim() && self.nl {
            end += 1;
        }
        end
    }

    /// `[^']{2,}`: two or more characters that are not `'`, the line feed
    /// included.
    pub(super) fn not_quotes(&self, p: usize) -> Option<usize> {
        let end = self.not_quote_end(p);
        let second = if p < self.lim() {
            self.char_end(p)
        } else {
            p + 1
        };
        (end > second).then_some(end)
    }

    /// `(?<!')[^']`: one character that is not `'` and does not follow one.
    pub(super) fn not_quote(&self, p: usize) -> Option<usize> {
        if self.prev_is_byte(p, b'\'') {
            return None;
        }
        if p < self.lim() {
            return (self.at(p) != b'\'').then(|| self.char_end(p));
        }
        (p == self.lim() && self.nl).then_some(p + 1)
    }

    /// `(')'(')`.
    pub(super) fn quote_char(&mut self, p: usize) -> Option<usize> {
        let end = self.lit(p, b"'''")?;
        self.capture(1, p, p + 1);
        self.capture(2, p + 2, end);
        Some(end)
    }
}
