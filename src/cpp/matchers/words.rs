//! Keywords, documentation tags, string prefixes and the like.

use super::super::engine::Lexer;
use super::chars::{is_digit, is_space};
use super::{RAW_DELIMITER, RAW_DELIMITER_MAX};
use crate::rules::Chars;

/// The prefixes of a raw string, as the expressions spell them.
#[derive(Clone, Copy)]
enum RawPrefix {
    /// `(?:[LUu]8?)?R`.
    Any,
    /// `(?:u8??|[LU])?R`, whose `8` is lazy.
    LazyU8,
}

impl Lexer<'_> {
    /// `(?<!\w)word(?!\w)`: where the word ends.
    pub(super) fn word_lit(&self, p: usize, word: &[u8]) -> Option<usize> {
        self.word_in(p, &[word])
    }

    /// `(?<!\w)word(?!\w)` as capture group 1.
    pub(super) fn word_group(&mut self, p: usize, word: &[u8]) -> Option<usize> {
        let end = self.word_lit(p, word)?;
        self.cap(1, p, end);
        Some(end)
    }

    /// `\\(?:u\h{4}|U\h{8})`: a universal character name.
    pub(super) fn ucn(&self, p: usize) -> Option<usize> {
        if self.at(p) != b'\\' || p >= self.lim() {
            return None;
        }
        let count = match self.at(p + 1) {
            b'u' => 4,
            b'U' => 8,
            _ => return None,
        };
        let end = p + 2 + count;
        (end <= self.lim() && self.s[p + 2..end].iter().all(u8::is_ascii_hexdigit)).then_some(end)
    }

    /// `(?<!\.)\.\.\.(?!\.)`.
    pub(super) fn ellipsis(&self, p: usize) -> Option<usize> {
        (self.lit(p, b"...").is_some() && self.prev(p) != Some(b'.') && self.at(p + 3) != b'.')
            .then_some(p + 3)
    }

    /// `(?<=(?:\W|^)for ?\()`.
    pub(super) fn after_for_paren(&self, p: usize) -> bool {
        if self.prev(p) != Some(b'(') {
            return false;
        }
        let start_ok = |at: usize| at == 0 || !self.word_before(at);
        let open = p - 1;
        (self.ends_with_at(open, b"for") && start_ok(open - 3))
            || (self.ends_with_at(open, b"for ") && start_ok(open - 4))
    }

    /// `(using)\s+(?!namespace\b)`.
    ///
    /// The white space gives back its last character.
    /// That character is not `namespace`.
    pub(super) fn using_name(&mut self, p: usize) -> Option<usize> {
        let word_end = self.lit(p, b"using")?;
        let first = self.ch(word_end).filter(|&(c, _)| is_space(c))?.1;
        let all = self.skip_space(word_end);
        let namespace_at = |lexer: &Self, q: usize| {
            lexer
                .lit(q, b"namespace")
                .is_some_and(|e| lexer.boundary(e))
        };
        let end = if namespace_at(self, all) {
            let last = self.ch_before(all)?;
            let back = all - last.len_utf8();
            if back < first {
                return None;
            }
            back
        } else {
            all
        };
        self.cap(1, p, word_end);
        Some(end)
    }

    /// `\G\s+{0,1}(\()([^(]*)(\))`.
    pub(super) fn macro_parameters(&mut self, p: usize) -> Option<usize> {
        if !self.at_g(p) {
            return None;
        }
        let open = self.skip_space(p);
        if self.at(open) != b'(' || open >= self.lim() {
            return None;
        }
        let body = open + 1;
        let stop = self.s[body.min(self.lim())..]
            .iter()
            .position(|&b| b == b'(')
            .map_or(self.lim(), |i| body + i);
        let close = self.s[body.min(stop)..stop]
            .iter()
            .rposition(|&b| b == b')')?
            + body;
        self.cap(1, open, open + 1);
        self.cap(2, body, close);
        self.cap(3, close, close + 1);
        Some(close + 1)
    }

    /// `((?<!\w)defined(?!\w))(\()`.
    pub(super) fn defined_paren(&mut self, p: usize) -> Option<usize> {
        let end = self.word_lit(p, b"defined")?;
        if self.at(end) != b'(' || end >= self.lim() {
            return None;
        }
        self.cap(1, p, end);
        self.cap(2, end, end + 1);
        Some(end + 1)
    }

    /// `(\\x0*\h{2}(?!\h))|(\\x\h*)`.
    pub(super) fn hex_escape(&mut self, p: usize) -> Option<usize> {
        if !self.starts(p, b"\\x") {
            return None;
        }
        let digits = p + 2;
        let end = digits
            + self.s[digits..]
                .iter()
                .take_while(|b| b.is_ascii_hexdigit())
                .count();
        let n = end - digits;
        if n >= 2 && self.s[digits..end - 2].iter().all(|&b| b == b'0') {
            self.cap(1, p, end);
        } else {
            self.cap(2, p, end);
        }
        Some(end)
    }

    /// `((?<=[!*/\s])[@\\]X)\s+(\S+)` for the tags `tags`, the first of which
    /// that fits: group 1 is the tag, group 2 the word after it.
    pub(super) fn doc_tag(&mut self, p: usize, tags: &[&[u8]]) -> Option<usize> {
        let before_ok = self
            .ch_before(p)
            .is_some_and(|c| matches!(c, '!' | '*' | '/') || is_space(c));
        if !before_ok || !matches!(self.at(p), b'@' | b'\\') || p >= self.lim() {
            return None;
        }
        for tag in tags {
            let Some(tag_end) = self.lit(p + 1, tag) else {
                continue;
            };
            let word = self.skip_space(tag_end);
            if word == tag_end {
                continue;
            }
            let end = self.span_of(word, |c| !is_space(c));
            if end == word {
                continue;
            }
            self.cap(1, p, tag_end);
            self.cap(2, word, end);
            return Some(end);
        }
        None
    }

    /// `((?:[LUu]8?)?R)"` with the optional prefix as group 1.
    /// It returns where the quote starts.
    ///
    /// The prefixes are tried in the grammar's order.
    fn raw_prefix(&mut self, p: usize, prefix: RawPrefix) -> Option<usize> {
        let candidates: &[&[u8]] = match prefix {
            RawPrefix::LazyU8 => &[b"uR", b"u8R", b"LR", b"UR", b"R"],
            RawPrefix::Any => &[b"LR", b"L8R", b"UR", b"U8R", b"uR", b"u8R", b"R"],
        };
        let end = candidates.iter().find_map(|c| self.lit(p, c))?;
        self.cap(1, p, end);
        Some(end)
    }

    /// `((?:[LUu]8?)?R)"(?:glsl|GLSL)\(` and the like: the start of a raw
    /// string with a language tag.
    pub(super) fn raw_tagged(&mut self, p: usize, tags: &[&[u8]]) -> Option<usize> {
        let r = self.raw_prefix(p, RawPrefix::Any)?;
        if self.at(r) != b'"' {
            return None;
        }
        tags.iter().find_map(|tag| {
            let t = self.lit(r + 1, tag)?;
            (self.at(t) == b'(' && t < self.lim()).then_some(t + 1)
        })
    }

    /// `((?:u8??|[LU])?R)"(?:([^\t ()\\]{0,16})|([^\t ()\\]*))\(`.
    pub(super) fn raw_delimited(&mut self, p: usize) -> Option<usize> {
        let r = self.raw_prefix(p, RawPrefix::LazyU8)?;
        if self.at(r) != b'"' || r >= self.lim() {
            return None;
        }
        let from = r + 1;
        let end = self.span_of(from, |c| !matches!(c, '\t' | ' ' | '(' | ')' | '\\'));
        if self.at(end) != b'(' || end >= self.lim() {
            return None;
        }
        if end - from <= RAW_DELIMITER_MAX {
            self.cap(2, from, end);
        } else {
            self.cap(3, from, end);
        }
        self.cap(usize::from(RAW_DELIMITER), from, end);
        Some(end + 1)
    }

    /// `(?<!\w)(?:(extern)\s+)?(template)\s+`.
    pub(super) fn explicit_instantiation(&mut self, p: usize) -> Option<usize> {
        if self.word_before(p) {
            return None;
        }
        let try_template = |lexer: &mut Self, at: usize| {
            let end = lexer.lit(at, b"template")?;
            let after = lexer.skip_space(end);
            (after > end).then(|| {
                lexer.cap(2, at, end);
                after
            })
        };
        if let Some(e) = self.lit(p, b"extern") {
            let after = self.skip_space(e);
            if after > e
                && let Some(end) = try_template(self, after)
            {
                self.cap(1, p, e);
                return Some(end);
            }
        }
        try_template(self, p)
    }

    /// `(template)\s+{0,1}(<)` after `(?<!\w)`.
    pub(super) fn template_angle(&mut self, p: usize) -> Option<usize> {
        let end = self.word_prefix(p, b"template")?;
        let q = self.skip_space(end);
        if self.at(q) != b'<' || q >= self.lim() {
            return None;
        }
        self.cap(1, p, end);
        self.cap(2, q, q + 1);
        Some(q + 1)
    }

    /// `(?<!\w)` and then `word`, without a boundary after it.
    pub(super) fn word_prefix(&self, p: usize, word: &[u8]) -> Option<usize> {
        if self.word_before(p) {
            return None;
        }
        self.lit(p, word)
    }

    /// `[A-Z_a-z][0-9A-Z_a-z]*` at `p`.
    pub(super) fn ascii_ident(&self, p: usize) -> Option<usize> {
        let first = self.at(p);
        if p >= self.lim() || !(first.is_ascii_alphabetic() || first == b'_') {
            return None;
        }
        let mut end = p + 1;
        while end < self.lim() && (self.s[end].is_ascii_alphanumeric() || self.s[end] == b'_') {
            end += 1;
        }
        Some(end)
    }

    /// `(?:[A-Z_a-z]|\\(?:u\h{4}|U\h{8}))(?:[0-9A-Z_a-z]|\\(?:u\h{4}|U\h{8}))*
    /// `.
    pub(super) fn ident_ucn(&self, p: usize) -> Option<usize> {
        let first = self.at(p);
        let mut end = if p < self.lim() && (first.is_ascii_alphabetic() || first == b'_') {
            p + 1
        } else {
            self.ucn(p)?
        };
        loop {
            let b = self.at(end);
            if end < self.lim() && (b.is_ascii_alphanumeric() || b == b'_') {
                end += 1;
            } else if let Some(next) = self.ucn(end) {
                end = next;
            } else {
                return Some(end);
            }
        }
    }

    /// The end of the `\d` character at `p`, or `None`.
    pub(super) fn digit_char(&self, p: usize) -> Option<usize> {
        self.one(p, is_digit)
    }

    /// `\\(["'?\\abefnprtv]|[0-3][0-7]{0,2}|[4-7]\d?|x\h{0,2}|u\h{0,4}|U\h{0,
    /// 8})`.
    pub(super) fn escaped_char(&self, p: usize) -> Option<usize> {
        if self.at(p) != b'\\' || p >= self.lim() {
            return None;
        }
        let lead = self.at(p + 1);
        if p + 1 >= self.lim() {
            return None;
        }
        let after = p + 2;
        let hex = |max: usize| {
            let mut end = after;
            while end < after + max && end < self.lim() && self.s[end].is_ascii_hexdigit() {
                end += 1;
            }
            end
        };
        if b"\"'?\\abefnprtv".contains(&lead) {
            return Some(after);
        }
        if (b'0'..=b'3').contains(&lead) {
            let mut end = after;
            while end < after + 2 && end < self.lim() && (b'0'..=b'7').contains(&self.s[end]) {
                end += 1;
            }
            return Some(end);
        }
        if (b'4'..=b'7').contains(&lead) {
            return Some(self.digit_char(after).unwrap_or(after));
        }
        match lead {
            b'x' => Some(hex(2)),
            b'u' => Some(hex(4)),
            b'U' => Some(hex(8)),
            _ => None,
        }
    }

    /// `(?<!\w)typedef(?!\w)` as group 1, white space, and `kw` ahead as a
    /// whole word.
    pub(super) fn typedef_before(&mut self, p: usize, kw: &[u8]) -> Option<usize> {
        let end = self.word_lit(p, b"typedef")?;
        let q = self.skip_space(end);
        self.word_lit(q, kw)?;
        self.cap(1, p, end);
        Some(q)
    }

    /// `\b(__asm__|asm)\b\s+{0,1}((?:volatile)?)`.
    pub(super) fn asm_begin(&mut self, p: usize) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        let word = [&b"__asm__"[..], b"asm"].iter().find_map(|w| {
            let end = self.lit(p, w)?;
            self.boundary(end).then_some(end)
        })?;
        let q = self.skip_space(word);
        self.cap(1, p, word);
        let end = self.lit(q, b"volatile").unwrap_or(q);
        self.cap(2, q, end);
        Some(end)
    }

    /// `__attribute(?:__)?\s*\(\s*\(`.
    pub(super) fn gcc_attribute(&self, p: usize) -> Option<usize> {
        let word = self.lit(p, b"__attribute")?;
        let with = self.lit(word, b"__");
        let tail = |lexer: &Self, from: usize| {
            let a = lexer.skip_space(from);
            if lexer.at(a) != b'(' || a >= lexer.lim() {
                return None;
            }
            let b = lexer.skip_space(a + 1);
            (lexer.at(b) == b'(' && b < lexer.lim()).then_some(b + 1)
        };
        with.and_then(|w| tail(self, w))
            .or_else(|| tail(self, word))
    }

    /// `(?<!\w)(?:struct|class|union|enum)(?!\w)` and its kin are `kw_in`;
    /// `[A-Z]+:` and the like are in `basic`.
    pub(super) fn static_assert_comma(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b',' || p >= self.lim() {
            return None;
        }
        self.cap(1, p, p + 1);
        Some(self.skip_space(p + 1))
    }

    /// `(?:['.0-9A-Z_a-z]|(?<=[EPep])[-+])+`.
    pub(super) fn number_tail(&self, p: usize) -> Option<usize> {
        let mut end = p;
        while end < self.lim() {
            let b = self.s[end];
            let ok = b == b'\''
                || b == b'.'
                || b.is_ascii_alphanumeric()
                || b == b'_'
                || ((b == b'-' || b == b'+')
                    && end > 0
                    && matches!(self.s[end - 1], b'E' | b'P' | b'e' | b'p'));
            if !ok {
                break;
            }
            end += 1;
        }
        self.spend(end - p + 1);
        (end > p).then_some(end)
    }

    /// `(?<!\w)\.?\d(?:['.0-9A-Z_a-z]|(?<=[EPep])[-+])*`.
    pub(super) fn number_literal(&self, p: usize) -> Option<usize> {
        if self.word_before(p) {
            return None;
        }
        let digit = |lexer: &Self, at: usize| lexer.digit_char(at);
        let start = if self.at(p) == b'.' && p < self.lim() {
            digit(self, p + 1).or_else(|| digit(self, p))?
        } else {
            digit(self, p)?
        };
        Some(self.number_tail(start).unwrap_or(start))
    }

    /// `(?<!\\)(?:(?=\n)|(?<=(?:^|[^\\])\n)(?=$))`: the end of a line that no
    /// backslash continues, before the line feed or after it.
    pub(super) fn logical_line_end(&self, p: usize) -> bool {
        if !self.nl {
            return false;
        }
        let lim = self.lim();
        (p == lim && self.prev(p) != Some(b'\\'))
            || (p == lim + 1 && (lim == 0 || self.s[lim - 1] != b'\\'))
    }
}
