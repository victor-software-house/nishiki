//! Comments, strings and characters: the contexts that span text, and the
//! rules that open them.

use memchr::{memchr, memchr2, memchr3};

use super::lexer::Lexer;
use super::scope::S;
use super::state::Rule;
use crate::utf8;

impl Lexer<'_> {
    // Rules that open a context or a single-line comment.

    /// `//` and `///` to the end of the line.
    pub(crate) fn line_comment(&mut self, p: usize) -> Option<usize> {
        if !self.starts_with(p, b"//") {
            return None;
        }
        let (slashes, scope) = if self.at(p + 2) == b'/' {
            (3, S::CommentLineDoc)
        } else {
            (2, S::CommentLine)
        };
        self.gap(p);
        self.tok(p + slashes, &[scope, S::PunctDefComment]);
        self.tok(self.s.len(), &[scope]);
        Some(self.s.len())
    }

    /// `/**/`, `/**` and `/*`.
    pub(crate) fn block_comment_start(&mut self, p: usize) -> Option<usize> {
        if !self.starts_with(p, b"/*") {
            return None;
        }
        self.gap(p);
        if self.starts_with(p + 2, b"*/") {
            self.tok(p + 4, &[S::CommentBlock]);
            return Some(p + 4);
        }
        let doc = self.at(p + 2) == b'*';
        let width = if doc { 3 } else { 2 };
        if !self.push(if doc { Rule::Doc } else { Rule::Block })
            && matches!(self.top(), Rule::Block | Rule::Doc)
        {
            self.set_carried(self.carried().saturating_add(1));
        }
        self.tok(p + width, &[]);
        Some(p + width)
    }

    /// `"`, `b"`, `r#"`, `br"`, `'` and `b'`.
    pub(crate) fn string_start(&mut self, p: usize, b: u8) -> Option<usize> {
        let prefix = usize::from(b == b'b');
        match self.at(p + prefix) {
            b'"' => {
                self.gap(p);
                if !self.push(Rule::String) {
                    return None;
                }
                self.tok(p + prefix, &[S::StrByteRaw]);
                self.tok(p + prefix + 1, &[S::PunctDefString]);
                Some(p + prefix + 1)
            }
            b'\'' => {
                self.gap(p);
                if !self.push(Rule::Char) {
                    return None;
                }
                self.tok(p + prefix, &[S::StrByteRaw]);
                self.tok(p + prefix + 1, &[S::PunctDefChar]);
                Some(p + prefix + 1)
            }
            b'r' => self.raw_string_start(p, prefix),
            _ => None,
        }
    }

    fn raw_string_start(&mut self, p: usize, prefix: usize) -> Option<usize> {
        let r_end = p + prefix + 1;
        let hashes_end = self.skip(r_end, |b| b == b'#');
        let hashes = u32::try_from(hashes_end - r_end).ok()?;
        if self.at(hashes_end) != b'"' {
            return None;
        }
        self.gap(p);
        if !self.push_with(Rule::Raw, hashes) {
            return None;
        }
        self.tok(r_end, &[S::StrByteRaw]);
        self.tok(hashes_end, &[S::PunctDefStringRaw]);
        self.tok(hashes_end + 1, &[S::PunctDefString]);
        Some(hashes_end + 1)
    }

    // Contexts that span text.
    // Each returns the position to continue from.

    pub(crate) fn in_string(&mut self, p: usize) -> usize {
        let Some(i) = memchr3(b'"', b'\\', b'{', &self.s[p..]) else {
            return self.s.len();
        };
        let q = p + i;
        match self.s[q] {
            b'"' => {
                self.gap(q);
                self.tok(q + 1, &[S::PunctDefString]);
                self.pop();
                q + 1
            }
            b'\\' => self.escape(q).unwrap_or(q + 1),
            _ => self.interpolation(q).unwrap_or(q + 1),
        }
    }

    pub(crate) fn in_char(&mut self, p: usize) -> usize {
        let Some(i) = memchr2(b'\'', b'\\', &self.s[p..]) else {
            return self.s.len();
        };
        let q = p + i;
        if self.s[q] == b'\'' {
            self.gap(q);
            self.tok(q + 1, &[S::PunctDefChar]);
            self.pop();
            return q + 1;
        }
        self.escape(q).unwrap_or(q + 1)
    }

    pub(crate) fn in_raw_string(&mut self, p: usize) -> usize {
        let hashes = usize::try_from(self.carried()).unwrap_or(usize::MAX);
        let mut from = p;
        while let Some(i) = memchr(b'"', &self.s[from..]) {
            let q = from + i;
            self.spend(hashes.min(self.s.len() - q));
            if self.s[q + 1..].len() >= hashes
                && self.s[q + 1..q + 1 + hashes].iter().all(|&b| b == b'#')
            {
                self.gap(q);
                self.tok(q + 1, &[S::PunctDefString]);
                self.tok(q + 1 + hashes, &[S::PunctDefStringRaw]);
                self.pop();
                return q + 1 + hashes;
            }
            from = q + 1;
        }
        self.s.len()
    }

    pub(crate) fn in_block_comment(&mut self, p: usize) -> usize {
        let Some(i) = memchr2(b'*', b'/', &self.s[p..]) else {
            return self.s.len();
        };
        let q = p + i;
        if self.s[q] == b'*' {
            if self.at(q + 1) != b'/' {
                return q + 1;
            }
            self.gap(q);
            self.tok(q + 2, &[]);
            match self.carried() {
                0 => self.pop(),
                over => self.set_carried(over - 1),
            }
            return q + 2;
        }
        self.block_comment_start(q).unwrap_or(q + 1)
    }

    // Rules inside strings.

    /// `\n`, `\x41`, `\u{1F600}`.
    /// Otherwise, a backslash and then any character.
    fn escape(&mut self, q: usize) -> Option<usize> {
        let hex = |b: u8| b.is_ascii_hexdigit();
        let (c1, c2) = (self.at(q + 1), self.at(q + 2));
        if c1 == b'x' && (b'0'..=b'7').contains(&c2) && hex(self.at(q + 3)) {
            self.gap(q);
            self.tok(q + 1, &[S::Escape, S::EscBackslash]);
            self.tok(q + 4, &[S::Escape, S::EscBit]);
            return Some(q + 4);
        }
        if c1 == b'u' && c2 == b'{' {
            let digits_end = self.skip(q + 3, hex);
            if (4..=6).contains(&(digits_end - q - 3)) && self.at(digits_end) == b'}' {
                self.gap(q);
                self.tok(q + 1, &[S::Escape, S::EscBackslash]);
                self.tok(q + 2, &[S::Escape, S::EscUnicode]);
                self.tok(q + 3, &[S::Escape, S::EscUnicode, S::EscUnicodePunct]);
                self.tok(digits_end, &[S::Escape, S::EscUnicode]);
                self.tok(
                    digits_end + 1,
                    &[S::Escape, S::EscUnicode, S::EscUnicodePunct],
                );
                return Some(digits_end + 1);
            }
        }
        if q + 1 >= self.s.len() {
            return None;
        }
        let end = q + 1 + utf8::char_len(c1);
        self.gap(q);
        self.tok(q + 1, &[S::Escape, S::EscBackslash]);
        self.tok(end.min(self.s.len()), &[S::Escape]);
        Some(end.min(self.s.len()))
    }

    /// `{` to `}` on one line, with no quote or brace between.
    fn interpolation(&mut self, q: usize) -> Option<usize> {
        let close = memchr3(b'"', b'{', b'}', &self.s[q + 1..])?;
        let r = q + 1 + close;
        if self.s[r] != b'}' {
            return None;
        }
        self.gap(q);
        self.tok(q + 1, &[S::MetaInterp, S::PunctDefInterp]);
        self.tok(r, &[S::MetaInterp]);
        self.tok(r + 1, &[S::MetaInterp, S::PunctDefInterp]);
        Some(r + 1)
    }
}
