//! The bracketed groups and names that several expressions share.

use std::cell::OnceCell;

use memchr::memchr;

use super::super::engine::Lexer;
use crate::rules::Chars;

impl Lexer<'_> {
    /// A name, or a name in backticks, at `i`: `\b\w+\b` or a run of
    /// characters that are not backticks, between two of them.
    ///
    /// The backticks hold at least one character that is not a backtick, so
    /// the closing one is the first after the first character.
    pub(super) fn name_at(&self, i: usize) -> Option<usize> {
        if self.word_at(i).is_some() && self.boundary(i) {
            return Some(self.word_end(i));
        }
        if self.at(i) != b'`' || self.at(i + 1) == b'`' {
            return None;
        }
        let rest = self.text(i + 1, self.lim());
        let close = memchr(b'`', rest);
        self.spend(close.map_or(rest.len(), |offset| offset + 1));
        close.map(|offset| i + 1 + offset + 1)
    }

    /// `<([^<>]|\g<GROUP>)+>` at `i`: balanced angle brackets with something
    /// between each pair.
    /// The alternatives share no first character.
    /// The match is therefore the first place the depth returns to zero.
    /// A pair with nothing in it fails the whole group.
    pub(super) fn angle_group(&self, i: usize) -> Option<usize> {
        if self.at(i) != b'<' || self.last_close(&self.x.last_angle_close, b">")? < i {
            return None;
        }
        let mut depth = 0_usize;
        let mut at = i;
        loop {
            let rest = self.text(at, self.lim());
            let offset = rest.iter().position(|&b| matches!(b, b'<' | b'>'));
            self.spend(offset.unwrap_or(rest.len()) + 1);
            at += offset?;
            if self.s[at] == b'<' {
                depth += 1;
            } else {
                if self.s[at - 1] == b'<' {
                    return None;
                }
                depth -= 1;
                if depth == 0 {
                    return Some(at + 1);
                }
            }
            at += 1;
        }
    }

    /// `[(<]([^"'()<>]|\g<GROUP>)+[)>]` at `i`: balanced brackets, each pair
    /// closed by either closer, with no quote inside and something between
    /// each pair.
    pub(super) fn bracket_group(&self, i: usize) -> Option<usize> {
        if !matches!(self.at(i), b'(' | b'<')
            || self.last_close(&self.x.last_bracket_close, b")>")? < i
        {
            return None;
        }
        let mut depth = 0_usize;
        let mut at = i;
        loop {
            let rest = self.text(at, self.lim());
            let offset = rest
                .iter()
                .position(|&b| matches!(b, b'(' | b'<' | b')' | b'>' | b'"' | b'\''));
            self.spend(offset.unwrap_or(rest.len()) + 1);
            at += offset?;
            match self.s[at] {
                b'(' | b'<' => depth += 1,
                b')' | b'>' => {
                    if matches!(self.s[at - 1], b'(' | b'<') {
                        return None;
                    }
                    depth -= 1;
                    if depth == 0 {
                        return Some(at + 1);
                    }
                }
                _ => return None,
            }
            at += 1;
        }
    }

    /// The last of `bytes` in the line, found once and kept in `memo`.
    pub(in crate::kotlin) fn last_close(
        &self,
        memo: &OnceCell<Option<usize>>,
        bytes: &[u8],
    ) -> Option<usize> {
        *memo.get_or_init(move || {
            let line = self.text(0, self.lim());
            self.spend(line.len());
            line.iter().rposition(|b| bytes.contains(b))
        })
    }
}
