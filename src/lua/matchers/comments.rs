//! Comments, the shebang line, and the ldoc tags of comments.

use super::super::engine::Lexer;
use crate::rules::Chars;

/// The ldoc tags the grammar names after `@`.
const LDOC_TAGS: &[&[u8]] = &[
    b"alias",
    b"annotation",
    b"author",
    b"charset",
    b"class",
    b"classmod",
    b"comment",
    b"constructor",
    b"copyright",
    b"description",
    b"example",
    b"export",
    b"factory",
    b"field",
    b"file",
    b"fixme",
    b"function",
    b"include",
    b"lfunction",
    b"license",
    b"local",
    b"module",
    b"name",
    b"param",
    b"pragma",
    b"private",
    b"raise",
    b"release",
    b"return",
    b"script",
    b"section",
    b"see",
    b"set",
    b"static",
    b"submodule",
    b"summary",
    b"tfield",
    b"thread",
    b"tparam",
    b"treturn",
    b"todo",
    b"topic",
    b"type",
    b"usage",
    b"warning",
    b"within",
];

impl Lexer<'_> {
    /// `\A(#!).*$\n?`: the first line when it starts with `#!`, with its line
    /// feed.
    pub(super) fn shebang(&mut self, p: usize) -> Option<usize> {
        if p != 0 || !self.first_line {
            return None;
        }
        let mark = self.lit(p, b"#!")?;
        self.cap(1, p, mark);
        let end = self.lim();
        Some(self.line_feed(end).unwrap_or(end))
    }

    /// `(^[\t ]+)?(?=--)`: the blanks that lead a line before a comment.
    /// The blanks are capture group 1.
    /// Without them, the match is empty before the comment.
    pub(super) fn comment_start(&mut self, p: usize) -> Option<usize> {
        if p == 0 {
            let blanks = self.tabs(0);
            if blanks > 0 && self.starts(blanks, b"--") {
                self.cap(1, 0, blanks);
                return Some(blanks);
            }
        }
        self.starts(p, b"--").then_some(p)
    }

    /// `(?!\G)((?!^)[\t ]+\n)?`: anywhere but where the comment began.
    /// The blanks that end a line, with its line feed, are capture group 1.
    pub(super) fn comment_end(&mut self, p: usize) -> Option<usize> {
        if self.at_g(p) {
            return None;
        }
        if p != 0 {
            let blanks = self.tabs(p);
            if blanks > p
                && let Some(end) = self.line_feed(blanks)
            {
                self.cap(1, p, end);
                return Some(end);
            }
        }
        Some(p)
    }

    /// `--\[(=*)\[`, with the `=` signs as capture group 1.
    /// The closing bracket repeats them.
    pub(super) fn long_comment_open(&mut self, p: usize) -> Option<usize> {
        let open = self.lit(p, b"--")?;
        self.long_bracket(open, 1)
    }

    /// `--\[(=*)\[@@@`: a long comment that holds Lua code.
    pub(super) fn code_comment_open(&mut self, p: usize) -> Option<usize> {
        let end = self.long_comment_open(p)?;
        self.lit(end, b"@@@")
    }

    /// `(--)?]\1]`
    pub(super) fn long_comment_close(&self, p: usize) -> Option<usize> {
        if let Some(after) = self.lit(p, b"--")
            && let Some(end) = self.long_bracket_close(after)
        {
            return Some(end);
        }
        self.long_bracket_close(p)
    }

    /// `\G[\t ]*(@)(alias|annotation|...|within)\b`: an ldoc tag where the
    /// comment's text starts, with `@` as capture group 1 and the tag as 2.
    pub(super) fn ldoc_tag(&mut self, p: usize) -> Option<usize> {
        if !self.at_g(p) {
            return None;
        }
        let at = self.tabs(p);
        let name = self.lit(at, b"@")?;
        let end = self.word_in(name, LDOC_TAGS)?;
        self.cap(1, at, name);
        self.cap(2, name, end);
        Some(end)
    }
}
