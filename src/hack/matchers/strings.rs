//! Comments, doc comments, attributes, strings, regular expressions in
//! strings, SQL strings and heredocs.

use super::super::engine::Lexer;
use super::is_ident_start;
use crate::php_family::matchers::is_flag;
use crate::rules::Chars;

/// The SQL keywords a string starts with to embed SQL.
const SQL_KEYWORDS: [&[u8]; 7] = [
    b"SELECT", b"INSERT", b"UPDATE", b"DELETE", b"CREATE", b"REPLACE", b"ALTER",
];

/// The tags a doc comment names after `@`.
const DOC_TAGS: [&[u8]; 24] = [
    b"abstract",
    b"author",
    b"category",
    b"copyright",
    b"example",
    b"global",
    b"internal",
    b"license",
    b"link",
    b"package",
    b"param",
    b"return",
    b"see",
    b"since",
    b"static",
    b"subpackage",
    b"throws",
    b"todo",
    b"var",
    b"version",
    b"uses",
    b"deprecated",
    b"final",
    b"ignore",
];

impl Lexer<'_> {
    /// `/\*\*(?:#@\+)?\s*$`: a doc comment that opens at the end of a line.
    /// The trailing `\s*` takes the line feed.
    pub(super) fn doc_open(&self, p: usize) -> Option<usize> {
        let at = self.lit(p, b"/**")?;
        let at = self.lit(at, b"#@+").unwrap_or(at);
        let end = self.space_end(at);
        self.at_eol(end).then_some(end)
    }

    /// `^(?!\s*\*).*$\n?`: a doc comment's line that does not start with `*`,
    /// with its line feed.
    pub(super) fn doc_plain_line(&self, p: usize) -> Option<usize> {
        if p != 0 || self.at(self.space_end(0)) == b'*' {
            return None;
        }
        let lim = self.lim();
        Some(lim + usize::from(self.avail(lim)))
    }

    /// `^\s*\*\s*(@access)\s+((public|private|protected|internal)|(.+))\s*$`.
    /// A keyword that more text follows is read by `(.+)`, which runs to the
    /// end of the text.
    /// So the trailing `\s*` takes only the line feed.
    pub(super) fn doc_access(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let star = self.space_end(0);
        if self.at(star) != b'*' || star >= self.lim() {
            return None;
        }
        let tag = self.space_end(star + 1);
        let tag_end = self.lit(tag, b"@access")?;
        let text = self.space_end(tag_end);
        if text == tag_end || text >= self.lim() {
            return None;
        }
        self.capture(1, tag, tag_end);
        let words: [&[u8]; 4] = [b"public", b"private", b"protected", b"internal"];
        for word in words {
            if let Some(end) = self.lit(text, word) {
                let after = self.space_end(end);
                if self.at_eol(after) {
                    self.capture(3, text, end);
                    return Some(after);
                }
            }
        }
        self.capture(4, text, self.lim());
        Some(self.space_end(self.lim()))
    }

    /// `@(a(bstract|uthor)|…|ignore)\b`: a doc comment's tag.
    /// `\b` follows the word, so the run of `\w` after `@` must be one of the
    /// tags.
    pub(super) fn doc_tag(&self, p: usize) -> Option<usize> {
        if self.at(p) != b'@' {
            return None;
        }
        let end = self.word_end(p + 1);
        DOC_TAGS.contains(&self.text(p + 1, end)).then_some(end)
    }

    /// `\{(@(link)).+?}`.
    pub(super) fn doc_link(&mut self, p: usize) -> Option<usize> {
        let at = self.lit(p, b"{@link")?;
        if at >= self.lim() {
            return None;
        }
        let close = memchr::memchr(b'}', self.text(at + 1, self.lim()))?;
        self.capture(1, p + 1, at);
        Some(at + 1 + close + 1)
    }

    /// `(^[\t ]+)?(?=//)`: the blanks before a line comment that starts a
    /// line, or nothing before one elsewhere.
    pub(super) fn line_comment_lead(&mut self, p: usize) -> Option<usize> {
        if p == 0 {
            let blanks = self.skip(0, |b| b == b' ' || b == b'\t');
            if blanks > 0 {
                if !self.starts(blanks, b"//") {
                    return None;
                }
                self.capture(1, 0, blanks);
                return Some(blanks);
            }
        }
        self.starts(p, b"//").then_some(p)
    }

    /// `(?=^\s*<<[^<])`: a line that starts with an attribute.
    pub(super) fn attribute_line(&self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let at = self.space_end(0);
        (self.starts(at, b"<<") && self.avail(at + 2) && self.at(at + 2) != b'<').then_some(p)
    }

    /// `(<<)(?!<)`.
    pub(super) fn attribute_open(&mut self, p: usize) -> Option<usize> {
        let end = self.lit(p, b"<<")?;
        if self.at(end) == b'<' {
            return None;
        }
        self.capture(1, p, end);
        Some(end)
    }

    /// `(?<=re)"/(?=(\\.|[^"/])++/[ADSUXeimsux]*")` for `"`, and with `'` for
    /// `'`.
    ///
    /// The `++` is possessive: each step takes `\\.` when it can, or any
    /// character but the quote and `/`, the line feed included, and none
    /// gives back.
    pub(super) fn regex_open(&self, p: usize, quote: u8) -> Option<usize> {
        if p < 2 || self.text(p - 2, p) != b"re" || self.at(p) != quote || self.at(p + 1) != b'/' {
            return None;
        }
        let lim = self.lim();
        let mut q = p + 2;
        while q < lim {
            let b = self.at(q);
            if b == b'\\' && q + 1 < lim {
                q = self.char_end(q + 1);
            } else if b == quote || b == b'/' {
                break;
            } else {
                q = self.char_end(q);
            }
        }
        if q >= lim || self.at(q) != b'/' {
            return None;
        }
        let flags = self.skip(q + 1, is_flag);
        (self.at(flags) == quote && flags < lim).then_some(p + 2)
    }

    /// `\\{1,2}['\\]`.
    pub(super) fn single_regex_escape(&self, p: usize) -> Option<usize> {
        if self.at(p) != b'\\' {
            return None;
        }
        if self.at(p + 1) == b'\\' && matches!(self.at(p + 2), b'\'' | b'\\') {
            return Some(p + 3);
        }
        matches!(self.at(p + 1), b'\'' | b'\\').then_some(p + 2)
    }

    /// `(\{\$.*?})`.
    pub(super) fn brace_variable(&self, p: usize) -> Option<usize> {
        let at = self.lit(p, b"{$")?;
        let close = memchr::memchr(b'}', self.text(at, self.lim()))?;
        Some(at + close + 1)
    }

    /// `(\$[A-Z_a-z][0-9A-Z_a-z]*((->[A-Z_a-z][0-9A-Z_a-z]*)|(\
    /// [[0-9A-Z_a-z]+]))?)`.
    pub(super) fn simple_variable(&self, p: usize) -> Option<usize> {
        if self.at(p) != b'$' {
            return None;
        }
        let name = self.ident(p + 1)?;
        if self.starts(name, b"->")
            && let Some(member) = self.ident(name + 2)
        {
            return Some(member);
        }
        if self.at(name) == b'[' {
            let index = self.skip(name + 1, super::is_ident);
            if index > name + 1 && self.at(index) == b']' {
                return Some(index + 1);
            }
        }
        Some(name)
    }

    /// `"\s*(?=(SELECT|INSERT|UPDATE|DELETE|CREATE|REPLACE|ALTER)\b)` for `"`,
    /// and with `'` for `'`.
    pub(super) fn sql_open(&self, p: usize, quote: u8) -> Option<usize> {
        if self.at(p) != quote || p >= self.lim() {
            return None;
        }
        let at = self.space_end(p + 1);
        let end = self.word_end(at);
        SQL_KEYWORDS.contains(&self.text(at, end)).then_some(at)
    }

    /// `#(\\"|[^"])*(?="|$\n?)` and its kin, for the mark `#` or `--` in a SQL
    /// string whose own quote is `quote`.
    /// `[^"]` takes the line feed, so a comment that no quote ends takes it.
    pub(super) fn sql_comment(&self, p: usize, mark: &[u8], quote: u8) -> Option<usize> {
        let lim = self.lim();
        let mut q = self.lit(p, mark)?;
        loop {
            if q + 1 < lim && self.at(q) == b'\\' && self.at(q + 1) == quote {
                q += 2;
            } else if !self.avail(q) || q < lim && self.at(q) == quote {
                return Some(q);
            } else if q == lim {
                q += 1;
            } else {
                q = self.char_end(q);
            }
        }
    }

    /// `<<<\s*("?)([A-Z_a-z]+[0-9A-Z_a-z]*)(\1)\s*$` for `"`, and with `'` for
    /// `'`: a heredoc or a nowdoc, whose label is group 2.
    /// The trailing `\s*` takes the line feed.
    pub(super) fn heredoc_open(&mut self, p: usize, quote: u8) -> Option<usize> {
        let at = self.lit(p, b"<<<")?;
        let start = self.space_end(at);
        let opened = self.at(start) == quote && start < self.lim();
        let label = start + usize::from(opened);
        if !is_ident_start(self.at(label)) {
            return None;
        }
        let label_end = self.skip(label, super::is_ident);
        let close = if opened {
            (self.at(label_end) == quote).then_some(label_end + 1)?
        } else {
            label_end
        };
        let end = self.space_end(close);
        if !self.at_eol(end) {
            return None;
        }
        self.capture(2, label, label_end);
        Some(end)
    }

    /// `^(\2)(?=;?$)`: the line that closes a heredoc, its label alone or with
    /// a `;`.
    pub(super) fn heredoc_close(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let end = self.begin_text_at(0)?;
        let closes = self.at_eol(end) || self.at(end) == b';' && self.at_eol(end + 1);
        if !closes {
            return None;
        }
        self.capture(1, 0, end);
        Some(end)
    }
}
