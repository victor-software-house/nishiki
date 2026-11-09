//! Comments and doc comments.

use super::super::PhpFamily;
use crate::rules::{Chars, Lexer};

impl<G: PhpFamily> Lexer<'_, G> {
    /// `/\*\*(?=\s)`.
    pub(crate) fn doc_comment(&self, p: usize) -> Option<usize> {
        let end = self.lit(p, b"/**")?;
        (self.space_at(end) > 0).then_some(end)
    }

    /// `(^\s+)?(?=//)` for `/`, and `(^\s+)?(?=#)(?!#\[)` for `#`.
    /// At the line's start the spaces before the comment are group 1.
    /// A shorter run of them would leave a space where the comment must be.
    pub(crate) fn line_comment(&mut self, p: usize, mark: u8) -> Option<usize> {
        let opens = |lexer: &Self, at: usize| match mark {
            b'/' => lexer.starts(at, b"//"),
            _ => lexer.at(at) == b'#' && at < lexer.lim() && !lexer.starts(at, b"#["),
        };
        if p == 0 {
            let end = self.space_end(0);
            if end > 0 && opens(self, end) {
                self.cap(1, 0, end);
                return Some(end);
            }
        }
        opens(self, p).then_some(p)
    }

    /// `^(?!\s*\*).*?(?:(?=\*/)|$\n?)`: a doc comment's line that does not
    /// start with `*`, up to the comment's end or the line's, with its line
    /// feed.
    pub(crate) fn doc_line(&self, p: usize) -> Option<usize> {
        if p != 0 || self.at(self.space_end(0)) == b'*' && self.space_end(0) < self.lim() {
            return None;
        }
        let mut at = 0;
        while at < self.lim() {
            if self.starts(at, b"*/") {
                return Some(at);
            }
            at += 1;
        }
        self.spend(at);
        Some(if self.avail(at) { at + 1 } else { at })
    }

    /// `@(api|abstract|…|ignore)\b`: a doc comment's tag.
    pub(crate) fn php_doc_tag(&self, p: usize) -> Option<usize> {
        if self.at(p) != b'@' {
            return None;
        }
        let at = p + 1;
        if let Some(end) = self.lit(at, b"property") {
            for suffix in [b"-read".as_slice(), b"-write"] {
                if let Some(long) = self.lit(end, suffix)
                    && self.word_at(long).is_none()
                {
                    return Some(long);
                }
            }
            return self.word_at(end).is_none().then_some(end);
        }
        let words: [&[u8]; 29] = [
            b"api",
            b"abstract",
            b"author",
            b"category",
            b"copyright",
            b"example",
            b"global",
            b"inheritDoc",
            b"inheritdoc",
            b"internal",
            b"license",
            b"link",
            b"method",
            b"package",
            b"param",
            b"return",
            b"see",
            b"since",
            b"source",
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
        words.iter().find_map(|word| {
            self.lit(at, word)
                .filter(|&end| self.word_at(end).is_none())
        })
    }

    /// `\{(@(link|inherit[Dd]oc)).+?}`.
    pub(crate) fn doc_inline_tag(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b'{' || self.at(p + 1) != b'@' {
            return None;
        }
        let tag = [b"link".as_slice(), b"inheritDoc", b"inheritdoc"]
            .iter()
            .find_map(|word| self.lit(p + 2, word))?;
        let close = (tag + 1..self.lim()).find(|&at| self.at(at) == b'}')?;
        self.cap(1, p + 1, tag);
        Some(close + 1)
    }
}
