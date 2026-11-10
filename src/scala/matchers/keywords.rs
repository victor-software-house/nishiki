//! Keywords, the `end` markers, operators and punctuation.

use super::super::engine::Lexer;
use super::names::{is_op, is_upper_start};
use crate::rules::Chars;

/// Whether `c` is an operator character of the keyword patterns, which also
/// take `\`: `[-!#%\&*+/:<-@\\^|~\p{Sm}\p{So}]`.
fn is_keyword_op(c: char) -> bool {
    c == '\\' || is_op(c)
}

impl Lexer<'_> {
    /// `(?=\s*(/(?:/.*|\*(?!.*\*/\s*\S.*).*))?$)` at `i`: blanks, then an
    /// optional comment, then the end of the line.
    ///
    /// A `/*` comment qualifies unless a `*/` after it is followed by more
    /// than blanks.
    fn end_marker_tail(&self, i: usize) -> bool {
        let at = self.space_end(i);
        if self.at_eol(at) {
            return true;
        }
        if self.at(at) != b'/' {
            return false;
        }
        match self.at(at + 1) {
            b'/' => true,
            b'*' => !self.closed_before_text(at + 2),
            _ => false,
        }
    }

    /// Whether a `*/` from `i` on has a character after it that is not a
    /// blank, before the end of the line.
    fn closed_before_text(&self, mut i: usize) -> bool {
        while let Some(found) = self
            .s
            .get(i..)
            .and_then(|rest| memchr::memmem::find(rest, b"*/"))
        {
            let close = i + found + 2;
            let text = self.non_blank_at(close);
            if text {
                return true;
            }
            i = i + found + 1;
        }
        false
    }

    /// Whether a character that is not `\s` follows the blanks at `i` on
    /// the line.
    fn non_blank_at(&self, i: usize) -> bool {
        let at = self.space_end(i);
        at < self.lim()
    }

    /// `^\s*(end)\s+`: where the blanks after `end` end, and where `end`
    /// starts and ends.
    fn end_marker(&self, p: usize) -> Option<(usize, usize, usize)> {
        if p != 0 {
            return None;
        }
        let start = self.space_end(0);
        let word = self.lit(start, b"end")?;
        let after = self.space_end(word);
        (after > word).then_some((start, word, after))
    }

    /// `^\s*(end)\s+(if|while|for|match)(?=…)` with `words` the alternatives:
    /// the forms whose scope covers the whole match.
    pub(super) fn end_of(&self, p: usize, words: &[&[u8]]) -> Option<usize> {
        let (_, _, after) = self.end_marker(p)?;
        words.iter().find_map(|word| {
            self.lit(after, word)
                .filter(|&end| self.end_marker_tail(end))
        })
    }

    /// `^\s*(end)\s+(?:(new|extension)|(upper))(?=…)`.
    ///
    /// The upper-case name backtracks: a `/` of its operator tail can start
    /// the comment.
    pub(super) fn end_of_type(&mut self, p: usize) -> Option<usize> {
        let (start, word, after) = self.end_marker(p)?;
        let keyword = [b"new".as_slice(), b"extension"].iter().find_map(|name| {
            self.lit(after, name)
                .filter(|&end| self.end_marker_tail(end))
        });
        let (group, end) = match keyword {
            Some(end) => (2, end),
            None => (
                3,
                self.plain_end_where(after, is_upper_start, |end| self.end_marker_tail(end))?,
            ),
        };
        self.capture(1, start, word);
        self.capture(group, after, end);
        Some(end)
    }

    /// `^\s*(end)\s+(quoted|plain|[op]+)?(?=…)`.
    pub(super) fn end_of_name(&mut self, p: usize) -> Option<usize> {
        let (start, word, after) = self.end_marker(p)?;
        let name = self.quoted_first_where(after, |end| self.end_marker_tail(end));
        let end = match name {
            Some(end) => end,
            None if self.end_marker_tail(after) => after,
            None => return None,
        };
        self.capture(1, start, word);
        if end > after {
            self.capture(2, after, end);
        }
        Some(end)
    }

    /// The end of the run of keyword operator characters at `i`, and how many
    /// characters it holds, up to three.
    fn keyword_ops(&self, i: usize) -> (usize, usize) {
        let mut end = i;
        let mut count = 0;
        while let Some((c, next)) = self.char_at(end) {
            if !is_keyword_op(c) {
                break;
            }
            end = next;
            count = (count + 1).min(3);
        }
        self.spend(end - i);
        (end, count)
    }

    /// `([-!#%\&*+/:<-@\\^|~\p{Sm}\p{So}]){3,}`.
    pub(super) fn long_operator(&self, p: usize) -> Option<usize> {
        let (end, count) = self.keyword_ops(p);
        (count >= 3).then_some(end)
    }

    /// `([-!#%\&*+/:<-@\\^|~\p{Sm}\p{So}]{2,}|_\*)`.
    pub(super) fn pair_operator(&mut self, p: usize) -> Option<usize> {
        let (run, count) = self.keyword_ops(p);
        let end = if count >= 2 { run } else { self.lit(p, b"_*")? };
        self.capture(1, p, end);
        Some(end)
    }

    /// `(?<!_)([-!#%\&*+/:<-@\\^|~\p{Sm}\p{So}])`.
    pub(super) fn single_operator(&mut self, p: usize) -> Option<usize> {
        if self.prev_is_byte(p, b'_') {
            return None;
        }
        let (c, end) = self.char_at(p)?;
        if !is_keyword_op(c) {
            return None;
        }
        self.capture(1, p, end);
        Some(end)
    }

    /// `<%|=:=|<:<|<%<|>:|<:`, in that order: `<%` takes the start of `<%<`.
    pub(super) fn bounds(&self, p: usize) -> Option<usize> {
        [b"<%".as_slice(), b"=:=", b"<:<", b">:", b"<:"]
            .iter()
            .find_map(|text| self.lit(p, text))
    }

    /// `(?<!:):(?!:)`.
    pub(super) fn lone_colon(&self, p: usize) -> Option<usize> {
        (self.at(p) == b':'
            && p < self.lim()
            && !self.prev_is_byte(p, b':')
            && self.at(p + 1) != b':')
            .then_some(p + 1)
    }

    /// `[]()\[{}]`.
    pub(super) fn bracket(&self, p: usize) -> Option<usize> {
        matches!(self.at(p), b'[' | b']' | b'(' | b')' | b'{' | b'}').then_some(p + 1)
    }

    /// `(\(\))`.
    pub(super) fn empty_parentheses(&mut self, p: usize) -> Option<usize> {
        let end = self.lit(p, b"()")?;
        self.capture(1, p, end);
        Some(end)
    }

    /// `(')((?>plain|[op]+))(?!')`: the group is atomic, so the name does not
    /// backtrack.
    pub(super) fn quoted_symbol(&mut self, p: usize) -> Option<usize> {
        let quote = self.lit(p, b"'")?;
        let end = self.id(quote)?;
        if self.at(end) == b'\'' && end < self.lim() {
            return None;
        }
        self.capture(1, p, quote);
        self.capture(2, quote, end);
        Some(end)
    }

    /// `'(?=\s*\{(?!'))` when `open` is `{`, and `'(?=\s*\[(?!'))` when it is
    /// `[`: a staged quote.
    pub(super) fn staging_quote(&self, p: usize, open: u8) -> Option<usize> {
        let quote = self.lit(p, b"'")?;
        let at = self.space_end(quote);
        (self.at(at) == open && at < self.lim() && self.at(at + 1) != b'\'').then_some(quote)
    }

    /// `\$(?=\s*\{)`.
    pub(super) fn staging_splice(&self, p: usize) -> Option<usize> {
        let dollar = self.lit(p, b"$")?;
        let at = self.space_end(dollar);
        (self.at(at) == b'{' && at < self.lim()).then_some(dollar)
    }
}
