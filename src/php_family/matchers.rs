//! The matchers the PHP family's grammars share: each answers for an
//! expression that two of them spell the same way.
//! Scopes come from each grammar's tables, so a matcher is shared even where
//! the grammars name its match differently.
//!
//! This file holds what PHP and Hack share.
//! The submodules hold PHP's matchers for the expressions that Blade's copy
//! of PHP spells the same way.

use super::PhpFamily;
use crate::case_fold;
use crate::rules::{Chars, Lexer};

#[cfg(any(feature = "lang-php", feature = "lang-blade"))]
mod comments;
#[cfg(any(feature = "lang-php", feature = "lang-blade"))]
mod declarations;
#[cfg(any(feature = "lang-php", feature = "lang-blade"))]
mod expressions;
#[cfg(any(feature = "lang-php", feature = "lang-blade"))]
mod strings;

/// `[_a-z\x7F-\x{10FFFF}]` under `(?i)`, by byte: every byte of a character
/// past ASCII is at least `0x80`.
pub(crate) fn is_name_start(b: u8) -> bool {
    b == b'_' || b.is_ascii_alphabetic() || b >= 0x7f
}

/// `[0-9_a-z\x7F-\x{10FFFF}]` under `(?i)`, by byte.
pub(crate) fn is_name(b: u8) -> bool {
    is_name_start(b) || b.is_ascii_digit()
}

/// `[ADSUXeimsux]`: a flag after a regular expression.
pub(crate) fn is_flag(b: u8) -> bool {
    matches!(
        b,
        b'A' | b'D' | b'S' | b'U' | b'X' | b'e' | b'i' | b'm' | b's' | b'u' | b'x'
    )
}

/// `[0-9\\_a-z\x7F-\x{10FFFF}]` under `(?i)`: a character of a qualified
/// name.
#[cfg(any(feature = "lang-php", feature = "lang-blade"))]
pub(crate) fn is_qualified(b: u8) -> bool {
    is_name(b) || b == b'\\'
}

/// The shared matchers, on the rule engine's lexer of either grammar.
#[cfg_attr(
    nishiki_unstable,
    expect(private_bounds, reason = "the bound and the methods are crate-private")
)]
impl<G: PhpFamily> Lexer<'_, G> {
    /// `[_a-z\x7F-\x{10FFFF}][0-9_a-z\x7F-\x{10FFFF}]*` under `(?i)`: a name.
    #[cfg(any(feature = "lang-php", feature = "lang-hack"))]
    pub(crate) fn name(&self, p: usize) -> Option<usize> {
        (p < self.lim() && is_name_start(self.at(p))).then(|| self.skip(p + 1, is_name))
    }

    /// The end of the literal `word` at `p`, without regard to case.
    pub(crate) fn ilit(&self, p: usize, word: &[u8]) -> Option<usize> {
        case_fold::literal_end(self.text(0, self.lim()), p, word)
    }

    /// `then` at the end of the `\s*` from `from`, or of the `\s+` when
    /// `one_or_more`, for a part that a name can start: where `then` starts
    /// and what it gives.
    ///
    /// A space past ASCII, such as U+00A0, is both `\s` and a name character.
    /// When `then` refuses the end of the run, Oniguruma gives the run back
    /// one space at a time, and `then` is tried at each.
    /// A `\s+` keeps one space.
    /// Only a space past ASCII is tried, since no name starts at an ASCII
    /// space.
    #[cfg(any(feature = "lang-php", feature = "lang-blade"))]
    pub(crate) fn after_spaces<R>(
        &self,
        from: usize,
        one_or_more: bool,
        mut then: impl FnMut(&Self, usize) -> Option<R>,
    ) -> Option<(usize, R)> {
        let mut at = self.space_end(from);
        if one_or_more && at == from {
            return None;
        }
        if let Some(found) = then(self, at) {
            return Some((at, found));
        }
        while let Some(last) = self.text(from, at).iter().rposition(|&b| b >= 0x80) {
            at = self.char_start_before(from + last + 1);
            if one_or_more && at == from {
                return None;
            }
            if let Some(found) = then(self, at) {
                return Some((at, found));
            }
        }
        None
    }

    /// `\?>`, where PHP's code ends.
    pub(crate) fn php_close(&self, p: usize) -> bool {
        self.starts(p, b"?>")
    }

    /// `(?i)(word)\s+`, with the word as group 1.
    pub(crate) fn word_then_spaces(&mut self, p: usize, word: &[u8]) -> Option<usize> {
        let end = self.ilit(p, word)?;
        let at = self.space_end(end);
        if at == end {
            return None;
        }
        self.capture(1, p, end);
        Some(at)
    }

    /// `\n|(?=\?>)`.
    pub(crate) fn comment_end(&self, p: usize) -> Option<usize> {
        if p == self.lim() && self.avail(p) {
            Some(p + 1)
        } else {
            self.php_close(p).then_some(p)
        }
    }

    /// `(@xlink)\s+(.+)\s*$`.
    /// When the spaces run to the line's end, they give back their last one
    /// to `.+`.
    /// The trailing `\s*` takes the line feed.
    pub(crate) fn doc_xlink(&mut self, p: usize) -> Option<usize> {
        let tag_end = self.lit(p, b"@xlink")?;
        let spaces = self.space_end(tag_end);
        if spaces == tag_end {
            return None;
        }
        let text = if spaces < self.lim() {
            spaces
        } else {
            self.char_start_before(self.lim())
        };
        if text <= tag_end || text >= self.lim() {
            return None;
        }
        self.capture(1, p, tag_end);
        self.capture(2, text, self.lim());
        Some(self.space_end(self.lim()))
    }

    /// `(/)([ADSUXeimsux]*)(")` for `"`, and with `'` for `'`.
    pub(crate) fn regex_close(&self, p: usize, quote: u8) -> Option<usize> {
        if self.at(p) != b'/' {
            return None;
        }
        let flags = self.skip(p + 1, is_flag);
        (self.at(flags) == quote && flags < self.lim()).then_some(flags + 1)
    }

    /// `(\\){1,2}[]$.\[^{}]`.
    pub(crate) fn regex_escape(&self, p: usize) -> Option<usize> {
        let special = |b: u8| matches!(b, b']' | b'$' | b'.' | b'[' | b'^' | b'{' | b'}');
        if self.at(p) != b'\\' {
            return None;
        }
        if self.at(p + 1) == b'\\' && special(self.at(p + 2)) {
            return Some(p + 3);
        }
        special(self.at(p + 1)).then_some(p + 2)
    }

    /// `(\{)\d+(,\d+)?(})`.
    pub(crate) fn regex_repeat(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b'{' {
            return None;
        }
        let digits = self.digit_end(p + 1);
        if digits == p + 1 {
            return None;
        }
        let mut close = digits;
        if self.at(digits) == b',' {
            let more = self.digit_end(digits + 1);
            if more > digits + 1 {
                close = more;
            }
        }
        if self.at(close) != b'}' {
            return None;
        }
        self.capture(1, p, p + 1);
        self.capture(3, close, close + 1);
        Some(close + 1)
    }

    /// `\\[0-7]{1,3}`.
    pub(crate) fn octal_escape(&self, p: usize) -> Option<usize> {
        if self.at(p) != b'\\' {
            return None;
        }
        let digits = (p + 1..p + 4)
            .take_while(|&at| matches!(self.at(at), b'0'..=b'7'))
            .count();
        (digits > 0).then_some(p + 1 + digits)
    }

    /// `\\x\h{1,2}`.
    pub(crate) fn hex_escape(&self, p: usize) -> Option<usize> {
        let at = self.lit(p, b"\\x")?;
        let digits = (at..at + 2)
            .take_while(|&k| self.at(k).is_ascii_hexdigit())
            .count();
        (digits > 0).then_some(at + digits)
    }

    /// `'(?=((\\')|[^"'])*("|$))` and its kin: a quote `quote` inside a SQL
    /// string whose own quote is `own`.
    /// It matches when the rest of the string holds no unescaped `quote`
    /// before its end or the line's.
    pub(crate) fn sql_quote(&self, p: usize, quote: u8, own: u8) -> Option<usize> {
        if self.at(p) != quote || p >= self.lim() {
            return None;
        }
        let mut at = p + 1;
        loop {
            let b = self.at(at);
            if at >= self.lim() {
                return Some(p + 1);
            }
            if b == b'\\' && self.at(at + 1) == quote && at + 1 < self.lim() {
                at += 2;
            } else if b == own {
                return Some(p + 1);
            } else if b == quote {
                return None;
            } else {
                at = self.char_end(at);
            }
        }
    }

    /// `(\$)((_(COOKIE|FILES|GET|POST|REQUEST))|arg([cv]))\b`.
    pub(crate) fn global_variable(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b'$' {
            return None;
        }
        let names: [&[u8]; 7] = [
            b"_COOKIE",
            b"_FILES",
            b"_GET",
            b"_POST",
            b"_REQUEST",
            b"argc",
            b"argv",
        ];
        let end = names.iter().find_map(|name| {
            self.lit(p + 1, name)
                .filter(|&end| self.word_at(end).is_none())
        })?;
        self.capture(1, p, p + 1);
        Some(end)
    }

    /// `(\$)((GLOBALS|_(ENV|SERVER|SESSION)))`.
    pub(crate) fn superglobal(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b'$' {
            return None;
        }
        let names: [&[u8]; 4] = [b"GLOBALS", b"_ENV", b"_SERVER", b"_SESSION"];
        let end = names.iter().find_map(|name| self.lit(p + 1, name))?;
        self.capture(1, p, p + 1);
        Some(end)
    }

    /// `<<|>>|[\&^|~]`.
    pub(crate) fn bitwise(&self, p: usize) -> Option<usize> {
        self.lit(p, b"<<")
            .or_else(|| self.lit(p, b">>"))
            .or_else(|| matches!(self.at(p), b'&' | b'^' | b'|' | b'~').then_some(p + 1))
    }

    /// `\b(catch)\s*(\()`.
    pub(crate) fn catch(&mut self, p: usize) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        let end = self.lit(p, b"catch")?;
        let open = self.space_end(end);
        if self.at(open) != b'(' {
            return None;
        }
        self.capture(1, p, end);
        self.capture(2, open, open + 1);
        Some(open + 1)
    }

    /// `(?i)(use)\s*(\()`.
    pub(crate) fn closure_use(&mut self, p: usize) -> Option<usize> {
        let end = self.ilit(p, b"use")?;
        let open = self.space_end(end);
        if self.at(open) != b'(' {
            return None;
        }
        self.capture(1, p, end);
        self.capture(2, open, open + 1);
        Some(open + 1)
    }
    /// `(?i)\bword\b`.
    #[cfg(any(feature = "lang-php", feature = "lang-blade"))]
    pub(crate) fn ikeyword(&self, p: usize, word: &[u8]) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        let end = self.ilit(p, word)?;
        self.word_at(end).is_none().then_some(end)
    }

    /// `(?i)\b(a|b|…)\b`, for words of which none is the start of a later one
    /// that the same run of `\w` could be.
    pub(crate) fn iword_in(&self, p: usize, words: &[&[u8]]) -> Option<usize> {
        if !self.boundary(p) || self.word_at(p).is_none() {
            return None;
        }
        let end = self.word_end(p);
        let mut buf = [0; 32];
        let len = case_fold::lower_into(self.text(p, end), &mut buf)?;
        let word = buf.get(..len)?;
        words.contains(&word).then_some(end)
    }

    /// `\[(?:\^?])?`.
    pub(crate) fn regex_class_open(&self, p: usize) -> Option<usize> {
        if self.at(p) != b'[' {
            return None;
        }
        if self.at(p + 1) == b'^' && self.at(p + 2) == b']' {
            return Some(p + 3);
        }
        Some(p + 1 + usize::from(self.at(p + 1) == b']'))
    }

    /// `(array)(\()(\))`.
    ///
    /// Its body sat in PHP's dispatch before the move.
    /// As a call, it cost 0.04% of PHP's instructions.
    #[expect(
        clippy::inline_always,
        reason = "with `#[inline]` it stayed a call from PHP's dispatch at one codegen unit"
    )]
    #[inline(always)]
    pub(crate) fn empty_array(&mut self, p: usize) -> Option<usize> {
        let end = self.lit(p, b"array()")?;
        self.capture(1, p, p + 5);
        self.capture(2, p + 5, p + 6);
        self.capture(3, p + 6, end);
        Some(end)
    }

    /// `\b(catch|try|throw|exception|finally)\b`.
    pub(crate) fn exception_keyword(&self, p: usize) -> Option<usize> {
        self.word_in(p, &[b"catch", b"try", b"throw", b"exception", b"finally"])
    }

    /// `[$*+^]`: an operator of a regular expression in a string.
    pub(crate) fn regex_operator(&self, p: usize) -> Option<usize> {
        matches!(self.at(p), b'$' | b'*' | b'+' | b'^').then_some(p + 1)
    }

    /// `\\[]'\[\\]`: an escape in a character class of a single-quoted
    /// regular expression.
    pub(crate) fn regex_class_escape(&self, p: usize) -> Option<usize> {
        (self.at(p) == b'\\' && matches!(self.at(p + 1), b']' | b'\'' | b'[' | b'\\'))
            .then_some(p + 2)
    }

    /// ``\\["'\\`]``: an escape in a SQL string.
    pub(crate) fn sql_escape(&self, p: usize) -> Option<usize> {
        (self.at(p) == b'\\' && matches!(self.at(p + 1), b'"' | b'\'' | b'\\' | b'`'))
            .then_some(p + 2)
    }

    /// `\\['\\]`: an escape in a single-quoted string.
    pub(crate) fn single_quote_escape(&self, p: usize) -> Option<usize> {
        (self.at(p) == b'\\' && matches!(self.at(p + 1), b'\'' | b'\\')).then_some(p + 2)
    }
    /// `X|(?=\?>)` for the byte `close`.
    #[cfg(any(feature = "lang-php", feature = "lang-blade"))]
    pub(crate) fn close_or_php(&self, p: usize, close: u8) -> Option<usize> {
        if self.at(p) == close && p < self.lim() {
            Some(p + 1)
        } else {
            self.php_close(p).then_some(p)
        }
    }

    /// `\S+`.
    #[cfg(any(feature = "lang-php", feature = "lang-blade"))]
    pub(crate) fn non_spaces(&self, p: usize) -> Option<usize> {
        let mut at = p;
        while at < self.lim() && self.space_at(at) == 0 {
            at = self.char_end(at);
        }
        (at > p).then_some(at)
    }

    /// `\s+(?=word\b)`: the spaces before a keyword.
    /// A shorter run would leave a space before the word, so the whole run is
    /// the only match.
    #[cfg(any(feature = "lang-php", feature = "lang-blade"))]
    pub(crate) fn spaces_before(&self, p: usize, word: &[u8]) -> Option<usize> {
        let end = self.space_end(p);
        (end > p
            && self
                .lit(end, word)
                .is_some_and(|after| self.word_at(after).is_none()))
        .then_some(end)
    }

    /// `[0-9\\_a-z\x7F-\x{10FFFF}]+` under `(?i)`: a qualified name, which may
    /// start with a digit.
    #[cfg(any(feature = "lang-php", feature = "lang-blade"))]
    pub(crate) fn qualified(&self, p: usize) -> Option<usize> {
        let end = self.skip(p, is_qualified);
        (end > p).then_some(end)
    }
}
