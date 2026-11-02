//! Comments and the tags of a `KDoc` comment.

use super::super::engine::Lexer;
use crate::classes::is_space;
use crate::rules::Chars;

/// `@(return|constructor|receiver|sample|see|author|since|suppress)\b`: the
/// tags the grammar names.
const TAGS: &[&[u8]] = &[
    b"return",
    b"constructor",
    b"receiver",
    b"sample",
    b"see",
    b"author",
    b"since",
    b"suppress",
];

impl Lexer<'_> {
    /// `/\*(?!\*)`: the start of a block comment that is not a `KDoc`.
    pub(super) fn block_comment_begin(&self, p: usize) -> Option<usize> {
        let end = self.lit(p, b"/*")?;
        (self.at(end) != b'*').then_some(end)
    }

    /// The end of the run of `\S` characters from `i`. Searches ask about the
    /// same run from every start inside it, so the last run is kept, with its
    /// last `]`.
    fn non_space_end(&self, i: usize) -> usize {
        let (start, end, _) = self.x.run.get();
        if start <= i && i < end {
            return end;
        }
        let mut at = i;
        let mut square = None;
        while at < self.lim() {
            match self.char_at(at) {
                Some((c, next)) if !is_space(c) => {
                    if c == ']' {
                        square = Some(at);
                    }
                    at = next;
                }
                _ => break,
            }
        }
        self.spend(at - i);
        self.x.run.set((i, at, square));
        at
    }

    /// `@(return|constructor|receiver|sample|see|author|since|suppress)\b`.
    pub(super) fn javadoc_tag(&self, p: usize) -> Option<usize> {
        if self.at(p) != b'@' {
            return None;
        }
        let end = self.word_end(p + 1);
        TAGS.contains(&self.text(p + 1, end)).then_some(end)
    }

    /// `(@param|@property)\s+(\S+)` and `(@exception|@throws)\s+(\S+)`: the
    /// tag, its blanks and the run after them. Giving blanks back cannot help,
    /// because the next character would be a blank and `\S+` needs another.
    pub(super) fn javadoc_named(&mut self, p: usize, tags: &[&[u8]]) -> Option<usize> {
        let tag = tags.iter().find_map(|tag| self.lit(p, tag))?;
        let start = self.space_end(tag);
        if start == tag {
            return None;
        }
        let end = self.non_space_end(start);
        if end == start {
            return None;
        }
        self.cap(1, p, tag);
        self.cap(2, start, end);
        Some(end)
    }

    /// `(@param)\[(\S+)]`: `\S+` takes the whole run and gives back until a
    /// `]` follows, so the last `]` of the run closes the match, and the name
    /// has at least one character.
    pub(super) fn javadoc_bracketed(&mut self, p: usize) -> Option<usize> {
        let tag = self.lit(p, b"@param")?;
        let open = self.lit(tag, b"[")?;
        self.non_space_end(open);
        let first = self.char_end(open);
        let close = self.x.run.get().2.filter(|&close| close >= first)?;
        self.cap(1, p, tag);
        self.cap(2, open, close);
        Some(close + 1)
    }

    /// `\{(@link)\s+(\S+)?#([$\w]+\s*\([^()]*\)).*}`.
    ///
    /// The optional `\S+` takes the whole run and gives back until a `#`
    /// follows, so the last `#` of the run is tried first and the earlier
    /// ones after it, and the group is left out last.
    pub(super) fn javadoc_link(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b'{' {
            return None;
        }
        let tag = self.lit(p + 1, b"@link")?;
        let start = self.space_end(tag);
        if start == tag {
            return None;
        }
        let end = self.non_space_end(start);
        let first = self.char_end(start).min(end);
        let marks = self.text(first, end);
        let mut found = None;
        for (offset, _) in marks.iter().enumerate().rev().filter(|&(_, &b)| b == b'#') {
            found = self
                .link_tail(first + offset + 1)
                .map(|tail| (first + offset, tail));
            if found.is_some() {
                break;
            }
        }
        if found.is_none() && self.at(start) == b'#' {
            found = self.link_tail(start + 1).map(|tail| (start, tail));
        }
        let (mark, (close, brace)) = found?;
        self.cap(1, p + 1, tag);
        if mark > start {
            self.cap(2, start, mark);
        }
        self.cap(3, mark + 1, close);
        Some(brace)
    }

    /// `([$\w]+\s*\([^()]*\)).*}` from `i`, after the `#`: where the group
    /// ends and where the last `}` of the line ends the match. Giving back
    /// part of the name or of `[^()]*` cannot help, because the next
    /// character would be a name character or not a parenthesis.
    fn link_tail(&self, i: usize) -> Option<(usize, usize)> {
        let mut name = i;
        loop {
            if self.s.get(name) == Some(&b'$') {
                name += 1;
            } else if let Some(width) = self.word_at(name) {
                name += width;
            } else {
                break;
            }
        }
        if name == i {
            return None;
        }
        let open = self.space_end(name);
        if self.at(open) != b'(' {
            return None;
        }
        let rest = self.text(open + 1, self.lim());
        let offset = rest.iter().position(|&b| matches!(b, b'(' | b')'));
        self.spend(offset.unwrap_or(rest.len()) + 1);
        let close = open + 1 + offset?;
        if self.s[close] != b')' {
            return None;
        }
        let brace = self
            .last_close(&self.x.last_brace, b"}")
            .filter(|&brace| brace > close)?;
        Some((close + 1, brace + 1))
    }
}
