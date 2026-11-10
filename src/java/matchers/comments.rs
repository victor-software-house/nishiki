//! Comments and Javadoc.

use crate::java::engine::Lexer;
use crate::rules::Chars;

impl Lexer<'_> {
    /// `(^[\t ]+)?(?=//)`: the blanks that start the line, then a `//`.
    pub(super) fn line_comment_lead(&mut self, p: usize) -> Option<usize> {
        if p == 0 {
            let blanks = self.blank_end(0);
            if blanks > 0 && self.starts(blanks, b"//") {
                self.capture(1, 0, blanks);
                return Some(blanks);
            }
        }
        self.starts(p, b"//").then_some(p)
    }

    /// `(?!\G)`.
    pub(super) fn not_anchor(&self, p: usize) -> Option<usize> {
        (!self.g).then_some(p)
    }

    /// `\n`, which only the end of the line has.
    pub(super) fn line_feed(&self, p: usize) -> Option<usize> {
        (p == self.lim()).then_some(p + 1)
    }

    /// `^\s*(/\*\*)(?!/)`.
    pub(super) fn javadoc_begin(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let start = self.space_end(0);
        let end = self.lit(start, b"/**")?;
        if self.at(end) == b'/' {
            return None;
        }
        self.capture(1, start, end);
        Some(end)
    }

    /// `@(author|deprecated|return|see|serial|since|version)\b`.
    pub(super) fn javadoc_tag(&self, p: usize) -> Option<usize> {
        if self.at(p) != b'@' {
            return None;
        }
        [
            &b"author"[..],
            b"deprecated",
            b"return",
            b"see",
            b"serial",
            b"since",
            b"version",
        ]
        .iter()
        .find_map(|word| {
            self.lit(p + 1, word)
                .filter(|&end| self.word_at(end).is_none())
        })
    }

    /// `(@param)\s+(\S+)` and `(@(?:exception|throws))\s+(\S+)`: a tag, white
    /// space and a word.
    pub(super) fn javadoc_named(&mut self, p: usize, tags: &[&[u8]]) -> Option<usize> {
        let tag_end = tags.iter().find_map(|tag| self.lit(p, tag))?;
        let name = self.space_end(tag_end);
        if name == tag_end {
            return None;
        }
        let end = self.non_space_end(name);
        if end == name {
            return None;
        }
        self.capture(1, p, tag_end);
        self.capture(2, name, end);
        Some(end)
    }

    /// The end of the run of `\S` characters from `i`.
    fn non_space_end(&self, mut i: usize) -> usize {
        while i < self.lim() && self.space_at(i) == 0 {
            i = self.char_end(i);
        }
        i
    }

    /// `\{(@link)\s+(\S+)?#([$\w]+\s*\([^()]*\)).*?}`.
    ///
    /// `(\S+)?#` ends at the last `#` of the word that a member follows.
    /// Without the word, it ends at the `#` that the white space ends at.
    pub(super) fn javadoc_link(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b'{' {
            return None;
        }
        let tag_end = self.lit(p + 1, b"@link")?;
        let first = self.space_end(tag_end);
        if first == tag_end {
            return None;
        }
        let run_end = self.non_space_end(first);
        let mut hash = run_end;
        loop {
            let found = if hash == first {
                (self.at(first) == b'#').then_some(first)
            } else {
                hash -= 1;
                (self.s.get(hash) == Some(&b'#')).then_some(hash)
            };
            if let Some(h) = found
                && let Some(end) = self.link_member(h + 1)
            {
                let close = self.s[end..self.lim()].iter().position(|&b| b == b'}')?;
                self.capture(1, p + 1, tag_end);
                if h > first {
                    self.capture(2, first, h);
                }
                self.capture(3, h + 1, end);
                return Some(end + close + 1);
            }
            if hash <= first {
                return None;
            }
        }
    }

    /// `[$\w]+\s*\([^()]*\)`: where it ends.
    fn link_member(&self, i: usize) -> Option<usize> {
        let name = self.name_end(i);
        if name == i {
            return None;
        }
        let open = self.space_end(name);
        if self.at(open) != b'(' {
            return None;
        }
        let close = self.s[open + 1..self.lim()]
            .iter()
            .position(|&b| b == b'(' || b == b')')?;
        let at = open + 1 + close;
        (self.s[at] == b')').then_some(at + 1)
    }
}
