//! Strings, here-strings, their escapes and the groups that open with a sigil.

use super::super::engine::Lexer;
use crate::rules::Chars;

/// Whether `c` is in `["“”„]`, the double quotes.
fn is_double_quote(c: char) -> bool {
    matches!(c, '"' | '\u{201c}' | '\u{201d}' | '\u{201e}')
}

/// Whether `c` is in `['‘-‛]`, the single quotes.
fn is_single_quote(c: char) -> bool {
    matches!(c, '\'' | '\u{2018}'..='\u{201b}')
}

/// Whether `c` is in ``["$'0`abefnrtv‘-„]``, the characters an escape may take
/// after its backtick.
fn is_escaped(c: char) -> bool {
    matches!(
        c,
        '"' | '$' | '\'' | '0' | '`' | 'a' | 'b' | 'e' | 'f' | 'n' | 'r' | 't' | 'v'
    ) || matches!(c, '\u{2018}'..='\u{201e}')
}

/// Whether `c` is in `(?i)[A-Z]`: a letter, or a character that folds to
/// one, as `ſ` and the Kelvin sign do.
fn is_folded_letter(c: char) -> bool {
    c.is_ascii_alphabetic() || matches!(c, 'ſ' | '\u{212a}')
}

impl Lexer<'_> {
    /// The end of the character at `i` when `class` accepts it.
    fn char_in(&self, i: usize, class: fn(char) -> bool) -> Option<usize> {
        self.char_at(i)
            .filter(|&(c, _)| class(c))
            .map(|(_, end)| end)
    }

    /// `["“”„]`: a double quote.
    pub(super) fn double_quote(&self, p: usize) -> Option<usize> {
        self.char_in(p, is_double_quote)
    }

    /// `['‘-‛]`: a single quote.
    pub(super) fn single_quote(&self, p: usize) -> Option<usize> {
        self.char_in(p, is_single_quote)
    }

    /// `(@["“”„])\s*$` or `(@['‘-‛])\s*$`, as `quote` reads the quote: a
    /// here-string's opening line, group 1 around `@` and the quote.
    pub(super) fn here_open(
        &mut self,
        p: usize,
        quote: fn(&Self, usize) -> Option<usize>,
    ) -> Option<usize> {
        if self.at(p) != b'@' {
            return None;
        }
        let quoted = quote(self, p + 1)?;
        let end = self.space_end(quoted);
        if !self.at_eol(end) {
            return None;
        }
        self.cap(1, p, quoted);
        Some(end)
    }

    /// `^["“”„]@` or `^['‘-‛]@`, as `quote` reads the quote: a here-string's
    /// closing line.
    pub(super) fn here_close(
        &self,
        p: usize,
        quote: fn(&Self, usize) -> Option<usize>,
    ) -> Option<usize> {
        if p != 0 {
            return None;
        }
        self.lit(quote(self, p)?, b"@")
    }

    /// `` `["$'0`abefnrtv‘-„] ``: an escape.
    pub(super) fn escape(&self, p: usize) -> Option<usize> {
        if self.at(p) != b'`' {
            return None;
        }
        self.char_in(p + 1, is_escaped)
    }

    /// `` `u\{(?:(?:10)?(\h){1,4}|0?\g<1>{1,5})} ``: a Unicode escape.
    ///
    /// The hex digits must reach the `}`.
    /// Up to five match, and six when they start with `0` or `10`.
    pub(super) fn unicode_escape(&self, p: usize) -> Option<usize> {
        let digits = self.lit(p, b"`u{")?;
        let close = self.skip(digits, |b| b.is_ascii_hexdigit());
        let count = close - digits;
        let fits = match count {
            1..=5 => true,
            6 => self.at(digits) == b'0' || self.starts(digits, b"10"),
            _ => false,
        };
        (fits && self.at(close) == b'}').then_some(close + 1)
    }

    /// `` `u(?:\{\h{0,6}.)? ``: a Unicode escape that is not valid.
    ///
    /// The `.` takes the character after up to six hex digits.
    /// At the end of the text it takes the last digit instead.
    pub(super) fn bad_unicode_escape(&self, p: usize) -> Option<usize> {
        let u = self.lit(p, b"`u")?;
        if self.at(u) != b'{' {
            return Some(u);
        }
        let digits = u + 1;
        let hex = self.skip(digits, |b| b.is_ascii_hexdigit()).min(digits + 6);
        if hex < self.lim() {
            return Some(self.char_end(hex));
        }
        Some(if hex > digits { hex } else { u })
    }

    /// `` `\s*$ ``: a backtick that continues the line.
    pub(super) fn line_continuation(&self, p: usize) -> Option<usize> {
        if self.at(p) != b'`' {
            return None;
        }
        let end = self.space_end(p + 1);
        self.at_eol(end).then_some(end)
    }

    /// `(?i)\b[-%+.0-9A-Z_]+@[-.0-9A-Z]+\.[A-Z]{2,64}\b`: an e-mail address,
    /// which the grammar leaves unnamed inside a string.
    ///
    /// The domain's characters include `.`.
    /// The domain therefore gives characters back to each `.` in it, the last
    /// first, until two to sixty-four letters and `\b` follow it.
    pub(super) fn address(&self, p: usize) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        let local = self.class_end(p, |c| {
            matches!(c, '-' | '%' | '+' | '.' | '_') || c.is_ascii_digit() || is_folded_letter(c)
        });
        if local == p || self.at(local) != b'@' {
            return None;
        }
        let domain_at = local + 1;
        let domain = self.class_end(domain_at, |c| {
            matches!(c, '-' | '.') || c.is_ascii_digit() || is_folded_letter(c)
        });
        let mut end = domain;
        while let Some(dot) = self
            .s
            .get(domain_at + 1..end)
            .and_then(|text| memchr::memrchr(b'.', text))
        {
            let dot = domain_at + 1 + dot;
            if let Some(found) = self.top_level(dot + 1) {
                return Some(found);
            }
            end = dot;
        }
        None
    }

    /// `[A-Z]{2,64}\b` under `(?i)` at `i`.
    ///
    /// Giving letters back cannot help: `\b` does not hold between two.
    fn top_level(&self, i: usize) -> Option<usize> {
        let mut end = i;
        let mut letters = 0;
        while let Some(next) = self.char_in(end, is_folded_letter) {
            end = next;
            letters += 1;
        }
        ((2..=64).contains(&letters) && self.boundary(end)).then_some(end)
    }

    /// `(x)(y)` for the bytes `sigil` and `open`, such as `(@)(\{)`, with
    /// groups 1 and 2 around them.
    pub(super) fn sigil_open(&mut self, p: usize, sigil: u8, open: u8) -> Option<usize> {
        if self.at(p) != sigil || self.at(p + 1) != open {
            return None;
        }
        self.cap(1, p, p + 1);
        self.cap(2, p + 1, p + 2);
        Some(p + 2)
    }

    /// `$(`, with the groups `((\$))(\()` names when `groups` is 3, and those
    /// of `(((\$)))((\())` when it is 5.
    /// The groups nest.
    /// The first half of them, rounded up, hold the `$`, and the rest the `(`.
    pub(super) fn substatement(&mut self, p: usize, groups: usize) -> Option<usize> {
        if !self.starts(p, b"$(") {
            return None;
        }
        let dollar = groups.div_ceil(2);
        for group in 1..=groups {
            if group <= dollar {
                self.cap(group, p, p + 1);
            } else {
                self.cap(group, p + 1, p + 2);
            }
        }
        Some(p + 2)
    }
}
