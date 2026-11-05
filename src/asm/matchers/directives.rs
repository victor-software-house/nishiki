//! The matchers of comments and the preprocessor.
//!
//! A directive line starts `^\s*[#%]\s*`.
//! The directive is a word of a list followed by `\b`, and every word is made
//! of `\w`, so the directive is the whole run of `\w` there.

use super::super::engine::Lexer;
use super::super::words;
use crate::rules::Chars;

/// What a directive line's word must be followed by.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::asm) enum After {
    /// `\b`.
    Boundary,
    /// `\b\s+`: at least one blank, which may be the line feed.
    Blanks,
}

impl Lexer<'_> {
    /// `(;|(^|\s)#\s).*$`: a comment, from `;`, or from `#` and a blank at
    /// the start of the line or after a blank, to the end of the line.
    ///
    /// The blank after `#` may be the line feed, which the match then takes.
    pub(super) fn line_comment(&self, p: usize) -> Option<usize> {
        let rest = if self.at(p) == b';' {
            p + 1
        } else {
            let hash = if p == 0 && self.at(p) == b'#' {
                p
            } else {
                let width = self.space_at(p);
                if width == 0 || p >= self.lim() {
                    return None;
                }
                p + width
            };
            if self.at(hash) != b'#' {
                return None;
            }
            let width = self.space_at(hash + 1);
            if width == 0 {
                return None;
            }
            hash + 1 + width
        };
        Some(rest.max(self.lim()))
    }

    /// `^\s*[#%]\s*` at `p`: where the directive's word starts.
    ///
    /// Giving back a blank of either `\s*` would leave a blank where `[#%]`
    /// or the word must stand.
    fn directive_head(&self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let mark = self.space_end(p);
        if !matches!(self.at(mark), b'#' | b'%') || mark >= self.lim() {
            return None;
        }
        Some(self.space_end(mark + 1))
    }

    /// A directive line whose word is one of `words`, as group 1, followed as
    /// `after` says.
    pub(super) fn directive_line(
        &mut self,
        p: usize,
        words: &[&[u8]],
        after: After,
    ) -> Option<usize> {
        let word = self.directive_head(p)?;
        let end = self.word_end(word);
        if end == word || words.binary_search(&self.text(word, end)).is_err() {
            return None;
        }
        self.cap(1, word, end);
        match after {
            After::Boundary => Some(end),
            After::Blanks => {
                let blanks = self.space_end(end);
                (blanks > end).then_some(blanks)
            }
        }
    }

    /// `^\s*[#%]\s*if\s+0\b`: the start of a comment that `#if 0` opens.
    pub(super) fn if_zero(&self, p: usize) -> Option<usize> {
        let word = self.directive_head(p)?;
        let blanks = self.lit(word, b"if")?;
        let zero = self.space_end(blanks);
        if zero == blanks || self.at(zero) != b'0' {
            return None;
        }
        (self.word_at(zero + 1).is_none()).then_some(zero + 1)
    }

    /// `(?=/[*/])|$`: before a comment, or at the end of the line.
    pub(super) fn directive_end(&self, p: usize) -> Option<usize> {
        (self.at(p) == b'/' && matches!(self.at(p + 1), b'*' | b'/') || self.at_eol(p)).then_some(p)
    }

    /// `(?>\\\s*\n)`: a backslash, blanks and the line feed, which keeps the
    /// directive open on the next line.
    ///
    /// The atomic group holds the whole expression, so `\s*` may still give
    /// back the line feed inside it.
    pub(super) fn continuation(&self, p: usize) -> Option<usize> {
        let blanks = self.lit(p, b"\\")?;
        let end = self.unicode_blanks(blanks);
        (end == self.lim() && self.nl).then_some(end + 1)
    }

    /// The end of the `\s` characters from `i` before the end of the text.
    fn unicode_blanks(&self, mut i: usize) -> usize {
        let from = i;
        while i < self.lim() {
            let width = self.space_at(i);
            if width == 0 {
                break;
            }
            i += width;
        }
        self.spend(i - from);
        i
    }

    /// `((%)(…)\s*(\())`: a preprocessor function's name after `%`, as
    /// group 3, and its open parenthesis.
    ///
    /// No `\b` follows the name, but `\s*\(` does, and every name is made of
    /// `\w`, so the name is the whole run of `\w`.
    pub(super) fn function(&mut self, p: usize) -> Option<usize> {
        let name = self.lit(p, b"%")?;
        let end = self.word_end(name);
        if end == name
            || words::PREPROCESSOR_FUNCTIONS_AT0
                .binary_search(&self.text(name, end))
                .is_err()
        {
            return None;
        }
        let open = self.lit(self.space_end(end), b"(")?;
        self.cap(3, name, end);
        Some(open)
    }

    /// `(\))|$`: a preprocessor function's close parenthesis, as group 1, or
    /// the end of the line.
    pub(super) fn function_end(&mut self, p: usize) -> Option<usize> {
        if self.at(p) == b')' {
            self.cap(1, p, p + 1);
            return Some(p + 1);
        }
        self.at_eol(p).then_some(p)
    }
}
