//! Comments, Scaladoc, the script header and the `//> using` directive.

use super::super::engine::Lexer;
use crate::rules::Chars;

/// The Scaladoc tags that `@(return|see|…)\b` lists, in its order.
const DOC_TAGS: &[&[u8]] = &[
    b"return",
    b"see",
    b"note",
    b"example",
    b"constructor",
    b"usecase",
    b"author",
    b"version",
    b"since",
    b"todo",
    b"deprecated",
    b"migration",
    b"define",
    b"inheritdoc",
    b"groupname",
    b"groupprio",
    b"groupdesc",
    b"group",
    b"contentDiagram",
    b"documentable",
    b"syntax",
];

impl Lexer<'_> {
    /// The end of the run of tabs and spaces from `i`.
    pub(super) fn blank_end(&self, i: usize) -> usize {
        self.skip(i, |b| b == b' ' || b == b'\t')
    }

    /// `(^[\t ]+)?(?=//)`: the blanks that start the line, then a `//`.
    pub(super) fn line_comment_lead(&mut self, p: usize) -> Option<usize> {
        if p == 0 {
            let blanks = self.blank_end(0);
            if blanks > 0 && self.starts(blanks, b"//") {
                self.cap(1, 0, blanks);
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

    /// `(?<=[\n;])`: just after a `;` or after the line feed.
    pub(super) fn after_semicolon_or_line_feed(&self, p: usize) -> Option<usize> {
        (self.prev_is_byte(p, b';') || p == self.lim() + 1).then_some(p)
    }

    /// `^\s*(/\*\*)(?!/)`.
    ///
    /// `\s*` gives blanks back only to a `/**` that starts with a blank.
    pub(super) fn scaladoc_begin(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let open = self.space_end(0);
        let end = self.lit(open, b"/**")?;
        if self.at(end) == b'/' {
            return None;
        }
        self.cap(1, open, end);
        Some(end)
    }

    /// `(@param)\s+(\S+)` when `tag` is `@param`, and
    /// `(@t(?:param|hrows))\s+(\S+)` when `tags` lists `@tparam` and `@throws`.
    ///
    /// Both runs are greedy, and a blank cannot start the name.
    pub(super) fn doc_tag_name(&mut self, p: usize, tags: &[&[u8]]) -> Option<usize> {
        let tag = tags.iter().find_map(|tag| self.lit(p, tag))?;
        let name = self.space_end(tag);
        if name == tag {
            return None;
        }
        let end = self.non_space_end(name);
        if end == name {
            return None;
        }
        self.cap(1, p, tag);
        self.cap(2, name, end);
        Some(end)
    }

    /// The end of the run of characters from `i` that are not `\s`.
    pub(super) fn non_space_end(&self, mut i: usize) -> usize {
        while i < self.lim() && self.space_at(i) == 0 {
            i = self.char_end(i);
        }
        i
    }

    /// `@(return|see|…|syntax)\b`: each tag is tried in order, with the `\b`
    /// after it.
    pub(super) fn doc_tag(&self, p: usize) -> Option<usize> {
        let tag = self.lit(p, b"@")?;
        DOC_TAGS
            .iter()
            .find_map(|word| self.lit(tag, word).filter(|&end| self.boundary(end)))
    }

    /// `(\[\[)([^]]+)(]])`.
    ///
    /// `[^]]+` runs to the first `]`, which must start `]]`.
    pub(super) fn doc_link(&mut self, p: usize) -> Option<usize> {
        let open = self.lit(p, b"[[")?;
        let close = open + memchr::memchr(b']', self.s.get(open..)?)?;
        if close == open {
            return None;
        }
        let end = self.lit(close, b"]]")?;
        self.cap(1, p, open);
        self.cap(2, open, close);
        self.cap(3, close, end);
        Some(end)
    }

    /// `^#!(.*)$`.
    pub(super) fn script_header(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let bang = self.lit(0, b"#!")?;
        let end = self.lim();
        self.cap(1, bang, end);
        Some(end)
    }

    /// `^\s*(//>)\s*(using)[^\n\S]+(\S+)?`.
    ///
    /// `[^\n\S]+` is the blanks before the line feed.
    /// `\S+` cannot start with a blank, so no run gives anything back.
    pub(super) fn using_directive(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let open = self.space_end(0);
        let marker = self.lit(open, b"//>")?;
        let word = self.space_end(marker);
        let using = self.lit(word, b"using")?;
        let mut blanks = using;
        while blanks < self.lim() && self.space_at(blanks) > 0 {
            blanks = self.char_end(blanks);
        }
        if blanks == using {
            return None;
        }
        let end = self.non_space_end(blanks);
        self.cap(1, open, marker);
        self.cap(2, word, using);
        if end > blanks {
            self.cap(3, blanks, end);
        }
        Some(end)
    }

    /// `[^,\s]+`.
    pub(super) fn directive_value(&self, p: usize) -> Option<usize> {
        let mut i = p;
        while i < self.lim() && self.at(i) != b',' && self.space_at(i) == 0 {
            i = self.char_end(i);
        }
        (i > p).then_some(i)
    }
}
