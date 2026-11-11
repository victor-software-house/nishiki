//! Comments and the dartdoc inside documentation comments.

use super::super::engine::Lexer;
use crate::rules::Chars;

impl Lexer<'_> {
    /// `((//).*)$`: a line comment, group 1 to the end of the line.
    pub(super) fn line_comment(&mut self, p: usize) -> Option<usize> {
        self.lit(p, b"//")?;
        let end = self.lim();
        self.capture(1, p, end);
        Some(end)
    }

    /// Where the `///` of a documentation line ends, when the line from `i`
    /// is `\s*///`.
    fn doc_slashes(&self, i: usize) -> Option<usize> {
        self.lit(self.space_end(i), b"///")
    }

    /// Where the `*` of a block comment line ends, when the line from `i` is
    /// `\s*\*`.
    fn doc_star(&self, i: usize) -> Option<usize> {
        self.lit(self.space_end(i), b"*")
    }

    /// `^(?!\s*///)`: a line that does not go on with `///` ends a
    /// documentation comment.
    pub(super) fn doc_comment_end(&self, p: usize) -> Option<usize> {
        (p == 0 && self.doc_slashes(p).is_none()).then_some(p)
    }

    /// `^\s*///\s*(```)` when `lead` is `///`, and `^\s*\*\s*(```)` when it
    /// is `*`: a line that opens a fenced code block.
    pub(super) fn fence_open(&self, p: usize, lead: &[u8]) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let lead = self.lit(self.space_end(p), lead)?;
        self.lit(self.space_end(lead), b"```")
    }

    /// `^(?:\s*///\s*(```)|(?!\s*///))`: the line that closes a fenced code
    /// block of `///` lines, or the first line that is not one.
    pub(super) fn triple_fence_close(&self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        match self.doc_slashes(p) {
            Some(slashes) => self.lit(self.space_end(slashes), b"```"),
            None => Some(p),
        }
    }

    /// `^(?:\s*\*\s*(```)|(?=\s*\*/))`: the line that closes a fenced code
    /// block of a block comment, or the line that closes the comment.
    pub(super) fn block_fence_close(&self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let star = self.doc_star(p)?;
        if let Some(end) = self.lit(self.space_end(star), b"```") {
            return Some(end);
        }
        (self.at(star) == b'/').then_some(p)
    }

    /// `^\s*///\s*(?!\s*```)`: a code line inside a fenced code block of
    /// `///` lines.
    ///
    /// Giving blanks back cannot help.
    /// The `\s*` of the lookahead takes them again.
    pub(super) fn triple_code_line(&self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let end = self.space_end(self.doc_slashes(p)?);
        (!self.starts(end, b"```")).then_some(end)
    }

    /// `^\s*\*\s*(?!(\s*```|/))`: a code line inside a fenced code block of a
    /// block comment.
    ///
    /// When a `/` follows the blanks, the match gives the last blank back.
    /// A blank then follows, and the blanks after it reach the `/` instead of a
    /// fence.
    pub(super) fn block_code_line(&self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let star = self.doc_star(p)?;
        let end = self.space_end(star);
        if self.starts(end, b"```") {
            return None;
        }
        if self.at(end) != b'/' {
            return Some(end);
        }
        (end > star).then(|| self.char_start_before(end))
    }

    /// `(\[.*?])`: a reference in brackets, to the first `]`.
    pub(super) fn doc_reference(&self, p: usize) -> Option<usize> {
        if self.at(p) != b'[' {
            return None;
        }
        let rest = self.s.get(p + 1..)?;
        let close = memchr::memchr(b']', rest)?;
        self.spend(close);
        Some(p + 1 + close + 1)
    }

    /// `` `[^\n`]+` ``: inline code, to the next backtick.
    pub(super) fn doc_code_span(&self, p: usize) -> Option<usize> {
        if self.at(p) != b'`' {
            return None;
        }
        let rest = self.s.get(p + 1..)?;
        let close = memchr::memchr(b'`', rest)?;
        self.spend(close);
        (close > 0).then_some(p + 1 + close + 1)
    }

    /// `(?:\*|//)\s{4,}(.*?)(?=($|\*/))`: an indented code line, group 1 from
    /// after the blanks to the end of the line or the first `*/`.
    ///
    /// The `\s{4,}` counts characters.
    /// It may take the line feed, and the match then ends after it.
    pub(super) fn doc_indented_code(&mut self, p: usize) -> Option<usize> {
        let lead = match self.at(p) {
            b'*' => p + 1,
            _ => self.lit(p, b"//")?,
        };
        let mut blanks = 0;
        let mut code = lead;
        loop {
            let width = self.space_at(code);
            if width == 0 {
                break;
            }
            code += width;
            blanks += 1;
        }
        if blanks < 4 {
            return None;
        }
        let end = match self.s.get(code..) {
            Some(rest) => memchr::memmem::find(rest, b"*/").map_or(self.lim(), |at| code + at),
            None => code,
        };
        self.spend(end.saturating_sub(code));
        self.capture(1, code, end);
        Some(end)
    }
}
