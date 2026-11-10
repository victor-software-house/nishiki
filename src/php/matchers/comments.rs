//! Comments, and the tags and types of doc comments.

use super::super::engine::Lexer;
use super::is_qualified;
use crate::rules::Chars;

impl Lexer<'_> {
    /// `^\s*\*\s*(@access)\s+(?i:((p(?:ublic|rivate|rotected)))|(.+))\s*$`.
    /// The spaces after `@access` give back one at a time, so `.+` can start
    /// on the last of them when nothing follows.
    /// The trailing `\s*` takes the line feed.
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
        let mut at = self.space_end(tag_end);
        while at > tag_end {
            for word in [b"public".as_slice(), b"private", b"protected"] {
                if let Some(end) = self.ilit(at, word) {
                    let rest = self.space_end(end);
                    if self.at_eol(rest) {
                        self.capture(1, tag, tag_end);
                        self.capture(2, at, end);
                        self.capture(3, at, end);
                        return Some(rest);
                    }
                }
            }
            if at < self.lim() {
                self.capture(1, tag, tag_end);
                self.capture(4, at, self.lim());
                return Some(self.space_end(self.lim()));
            }
            at = self.char_start_before(at);
        }
        None
    }

    /// `(@(?:global|param|property(-(read|write))?|return|throws|var))\s+(?=[(?
    /// A-Z\\_a-z\x7F-\x{10FFFF}])`. A space past ASCII is in the class, so the
    /// spaces give back to the last such space when the character after them is
    /// not.
    pub(super) fn doc_typed_tag(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b'@' {
            return None;
        }
        let words: [&[u8]; 8] = [
            b"global",
            b"param",
            b"property-read",
            b"property-write",
            b"property",
            b"return",
            b"throws",
            b"var",
        ];
        for word in words {
            let Some(end) = self.lit(p + 1, word) else {
                continue;
            };
            let spaces = self.space_end(end);
            if spaces == end {
                continue;
            }
            let starts_type = |b: u8| {
                matches!(b, b'(' | b'?' | b'\\' | b'_') || b.is_ascii_alphabetic() || b >= 0x7f
            };
            let mut at = spaces;
            if !(at < self.lim() && starts_type(self.at(at))) {
                at = end;
                let mut back = spaces;
                while back > end + 1 {
                    back = self.char_start_before(back);
                    if back > end && self.at(back) >= 0x80 {
                        at = back;
                        break;
                    }
                }
                if at == end {
                    continue;
                }
            }
            self.capture(1, p, end);
            return Some(at);
        }
        None
    }

    /// `(\))(\[])?|(?=\*/)`.
    pub(super) fn doc_types_close(&mut self, p: usize) -> Option<usize> {
        if self.at(p) == b')' && p < self.lim() {
            self.capture(1, p, p + 1);
            if self.starts(p + 1, b"[]") {
                self.capture(2, p + 1, p + 3);
                return Some(p + 3);
            }
            return Some(p + 1);
        }
        self.starts(p, b"*/").then_some(p)
    }

    /// `(?i)\??[0-9\\_a-z\x7F-\x{10FFFF}]+([\&|]\??[0-9\\_a-z\x7F-\
    /// x{10FFFF}]+)*`.
    pub(super) fn doc_types(&self, p: usize) -> Option<usize> {
        let first = p + usize::from(self.at(p) == b'?');
        let mut end = self.qualified(first)?;
        while matches!(self.at(end), b'&' | b'|') {
            let next = end + 1 + usize::from(self.at(end + 1) == b'?');
            match self.qualified(next) {
                Some(more) => end = more,
                None => break,
            }
        }
        Some(end)
    }

    /// `(?i)([0-9\\_a-z\x7F-\x{10FFFF}]+)(\[])`.
    pub(super) fn doc_type_array(&mut self, p: usize) -> Option<usize> {
        let end = self.skip(p, is_qualified);
        if end == p || !self.starts(end, b"[]") {
            return None;
        }
        self.capture(1, p, end);
        self.capture(2, end, end + 2);
        Some(end + 2)
    }
}
