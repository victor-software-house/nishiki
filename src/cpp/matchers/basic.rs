//! The short expressions: operators, escapes, comment markers and the like.

use super::super::engine::Lexer;
use super::RAW_DELIMITER_MAX;
use super::chars::{is_space, is_word};
use crate::cpp::types::PREDEFINED;
use crate::rules::Chars;

impl Lexer<'_> {
    /// One of the bytes of `set`, a character class such as `[&^|~]`.
    pub(super) fn one_of(&self, p: usize, set: &[u8]) -> Option<usize> {
        (p < self.lim() && set.contains(&self.s[p])).then_some(p + 1)
    }

    /// Whether the byte `b` stands at `p`.
    pub(super) fn ahead(&self, p: usize, b: u8) -> bool {
        self.at(p) == b
    }

    /// `text` at `p` as capture group `group`: where it ends.
    pub(super) fn group_lit(&mut self, p: usize, group: usize, text: &[u8]) -> Option<usize> {
        let end = self.lit(p, text)?;
        self.cap(group, p, end);
        Some(end)
    }

    /// `(?=(?<!\\)\n)`: the end of a line that no backslash continues.
    pub(super) fn macro_line_end(&self, p: usize) -> bool {
        self.nl && p == self.lim() && self.prev(p) != Some(b'\\')
    }

    /// `!=|<=|>=|==|[<>]`.
    pub(super) fn compare_operator(&self, p: usize) -> Option<usize> {
        self.lit_any(p, &[b"!=", b"<=", b">=", b"=="])
            .or_else(|| self.one_of(p, b"<>"))
    }

    /// `(?<=\{|<%|\?\?<)`: whether `{`, `<%` or `??<` ends at `p`.
    pub(super) fn after_open(&self, p: usize) -> bool {
        self.prev(p) == Some(b'{') || self.ends_with_at(p, b"<%") || self.ends_with_at(p, b"??<")
    }

    /// `(?<=\n)(?<!\\\n)`: after the line feed, unless a backslash ends the
    /// line.
    pub(super) fn after_line(&self, p: usize) -> bool {
        self.nl && p == self.lim() + 1 && !(self.lim() > 0 && self.s[self.lim() - 1] == b'\\')
    }

    /// `(?<!:):(?!:)`.
    pub(super) fn single_colon(&self, p: usize) -> Option<usize> {
        let ok = self.at(p) == b':'
            && p < self.lim()
            && self.prev(p) != Some(b':')
            && self.at(p + 1) != b':';
        ok.then_some(p + 1)
    }

    /// `\G ?`.
    pub(super) fn anchor_space(&self, p: usize) -> Option<usize> {
        self.at_g(p)
            .then(|| p + usize::from(p < self.lim() && self.s[p] == b' '))
    }

    /// `\\.`: a backslash and a character that is not the line feed.
    pub(super) fn backslash_any(&self, p: usize) -> Option<usize> {
        if self.at(p) != b'\\' || p >= self.lim() || p + 1 >= self.lim() {
            return None;
        }
        self.ch(p + 1).map(|(_, end)| end)
    }

    /// `\\\n`: a backslash that ends the line.
    pub(super) fn line_continuation(&self, p: usize) -> Option<usize> {
        (self.nl && p + 1 == self.lim() && self.s[p] == b'\\').then_some(p + 2)
    }

    /// `\\[0-7]{1,3}`.
    pub(super) fn octal_escape(&self, p: usize) -> Option<usize> {
        if self.at(p) != b'\\' || p >= self.lim() {
            return None;
        }
        let mut end = p + 1;
        while end < p + 4 && end < self.lim() && (b'0'..=b'7').contains(&self.s[end]) {
            end += 1;
        }
        (end > p + 1).then_some(end)
    }

    /// `\\["'?\\abfnrtv]`.
    pub(super) fn simple_escape(&self, p: usize) -> Option<usize> {
        if self.at(p) != b'\\' || p >= self.lim() {
            return None;
        }
        let next = self.at(p + 1);
        (p + 1 < self.lim() && b"\"'?\\abfnrtv".contains(&next)).then_some(p + 2)
    }

    /// `\S+`.
    pub(super) fn non_space_run(&self, p: usize) -> Option<usize> {
        let end = self.span_of(p, |c| !is_space(c));
        (end > p).then_some(end)
    }

    /// `[^*]*\n`: the rest of the line, when it has no `*`.
    pub(super) fn rest_without_star(&self, p: usize) -> Option<usize> {
        let ok = self.nl && p <= self.lim() && !self.s[p..].contains(&b'*');
        ok.then(|| self.lim() + 1)
    }

    /// `\)\s*\)`.
    pub(super) fn close_close(&self, p: usize) -> Option<usize> {
        if self.at(p) != b')' || p >= self.lim() {
            return None;
        }
        let q = self.skip_space(p + 1);
        (self.at(q) == b')' && q < self.lim()).then_some(q + 1)
    }

    /// `(R?)(")`: group 1 is the `R`, group 2 the quote.
    pub(super) fn raw_prefix_quote(&mut self, p: usize) -> Option<usize> {
        let q = if self.at(p) == b'R' && p < self.lim() {
            self.cap(1, p, p + 1);
            p + 1
        } else {
            self.cap(1, p, p);
            p
        };
        (self.at(q) == b'"' && q < self.lim()).then(|| {
            self.cap(2, q, q + 1);
            q + 1
        })
    }

    /// `[!*]*\*/`: the end of a documentation comment.
    pub(super) fn doc_end(&self, p: usize) -> Option<usize> {
        let end = self.span_of(p, |c| c == '!' || c == '*');
        (end > p && self.s[end - 1] == b'*' && self.at(end) == b'/' && end < self.lim())
            .then_some(end + 1)
    }

    /// `\s*+(//)` and `\s*+(/\*)`: white space and the comment marker, which is
    /// group 1.
    pub(super) fn lead_slashes(&mut self, p: usize, marker: &[u8]) -> Option<usize> {
        let q = self.skip_space(p);
        let end = self.lit(q, marker)?;
        self.cap(1, q, end);
        Some(end)
    }

    /// `\s*(?=;)` and the like: white space, then `b` ahead.
    pub(super) fn spaces_then(&self, p: usize, b: u8) -> Option<usize> {
        let q = self.skip_space(p);
        (self.at(q) == b).then_some(q)
    }

    /// `(?<=}|%>|\?\?>)\s*`.
    pub(super) fn after_close_spaces(&self, p: usize) -> Option<usize> {
        let ok = self.prev(p) == Some(b'}')
            || self.ends_with_at(p, b"%>")
            || self.ends_with_at(p, b"??>");
        ok.then(|| self.skip_space(p))
    }

    /// `(?<=\h)'(?=\h)`: a digit separator.
    pub(super) fn digit_separator(&self, p: usize) -> Option<usize> {
        let hex = |b: Option<u8>| b.is_some_and(|b| b.is_ascii_hexdigit());
        let ok =
            self.at(p) == b'\'' && p < self.lim() && hex(self.prev(p)) && hex(Some(self.at(p + 1)));
        ok.then_some(p + 1)
    }

    /// `(?<=\w)\s+{0,1}<`.
    pub(super) fn word_spaces_angle(&self, p: usize) -> Option<usize> {
        if !self.word_before(p) {
            return None;
        }
        let q = self.skip_space(p);
        (self.at(q) == b'<' && q < self.lim()).then_some(q + 1)
    }

    /// `((?:u8??|[LU])?)"`: an encoding prefix, which is group 1, and a quote.
    pub(super) fn string_prefix_quote(&mut self, p: usize, mark: u8) -> Option<usize> {
        let quote = |lexer: &Self, i: usize| lexer.at(i) == mark && i < lexer.lim();
        let prefix = if self.at(p) == b'u' && p < self.lim() {
            if quote(self, p + 1) {
                Some(p + 1)
            } else if self.at(p + 1) == b'8' && quote(self, p + 2) {
                Some(p + 2)
            } else {
                None
            }
        } else if matches!(self.at(p), b'L' | b'U') && p < self.lim() && quote(self, p + 1) {
            Some(p + 1)
        } else {
            None
        };
        let end = match prefix {
            Some(end) => end,
            None if quote(self, p) => p,
            None => return None,
        };
        self.cap(1, p, end);
        Some(end + 1)
    }

    /// `[$A-Z_a-z][-$\w]*`.
    pub(super) fn pragma_name(&self, p: usize) -> Option<usize> {
        let first = self.at(p);
        if p >= self.lim() || !(first == b'$' || first == b'_' || first.is_ascii_alphabetic()) {
            return None;
        }
        Some(self.span_of(p + 1, |c| c == '-' || c == '$' || is_word(c)))
    }

    /// `\b__([A-Z_]+)__\b`.
    /// `\b(__cplusplus|…)\b`: one of the listed predefined macros.
    pub(super) fn predefined_macro(&mut self, p: usize) -> Option<usize> {
        let end = self.word_in(p, PREDEFINED)?;
        self.cap(1, p, end);
        Some(end)
    }

    pub(super) fn predefined_probably(&mut self, p: usize) -> Option<usize> {
        if self.boundary(p) && !self.starts(p, b"__") {
            return None;
        }
        if !self.boundary(p) || !self.starts(p, b"__") {
            return None;
        }
        let end = self.span_of(p + 2, |c| c.is_ascii_uppercase() || c == '_');
        if end < p + 5 || !self.ends_with_at(end, b"__") || self.word_at(end).is_some() {
            return None;
        }
        self.cap(1, p + 2, end - 2);
        Some(end)
    }

    /// `\b[A-Z]+:|@[_a-z]+:`.
    pub(super) fn todo_label(&self, p: usize) -> Option<usize> {
        if self.boundary(p) {
            let end = self.span_of(p, |c| c.is_ascii_uppercase());
            if end > p && self.at(end) == b':' && end < self.lim() {
                return Some(end + 1);
            }
        }
        if self.at(p) != b'@' || p >= self.lim() {
            return None;
        }
        let end = self.span_of(p + 1, |c| c == '_' || c.is_ascii_lowercase());
        (end > p + 1 && self.at(end) == b':' && end < self.lim()).then_some(end + 1)
    }

    /// `^\s+{1,0}(//[!/]+)`: a documentation line comment, group 1.
    pub(super) fn doc_line_lead(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let q = self.skip_space(0);
        if !self.starts(q, b"//") {
            return None;
        }
        let end = self.span_of(q + 2, |c| c == '!' || c == '/');
        if end == q + 2 {
            return None;
        }
        self.cap(1, q, end);
        Some(end)
    }

    /// `\)(\2)(\3)"` and a user-defined suffix: the end of a raw string
    /// whose start had group 2 as its delimiter, then the optional suffix
    /// in its two forms, `[A-Za-z]…` and `_…`, as groups 4 and 5.
    ///
    /// The delimiter is the rule's payload.
    /// One longer than 16 bytes is group 3 of the start and of the end.
    /// Their group 2 is then empty.
    ///
    /// The suffix: one letter or UCN (`_` for the second form), then word
    /// characters and UCNs.
    pub(super) fn raw_close(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b')' || p >= self.lim() {
            return None;
        }
        let name = self.lit(p + 1, self.payload_text().as_bytes())?;
        if self.at(name) != b'"' || name >= self.lim() {
            return None;
        }
        let quote = name + 1;
        self.cap(1, p, quote);
        if name - (p + 1) > RAW_DELIMITER_MAX {
            self.cap(2, p + 1, p + 1);
            self.cap(3, p + 1, name);
        } else {
            self.cap(2, p + 1, name);
            self.cap(3, name, name);
        }
        let lead = self.at(quote);
        let first = if quote < self.lim() && lead.is_ascii_alphabetic() {
            Some((4, quote + 1))
        } else if quote < self.lim() && lead == b'_' {
            Some((5, quote + 1))
        } else {
            self.ucn(quote).map(|e| (4, e))
        };
        let Some((group, mut end)) = first else {
            return Some(quote);
        };
        loop {
            if let Some(e) = self.ucn(end) {
                end = e;
            } else if end < self.lim()
                && (self.at(end).is_ascii_alphanumeric() || self.at(end) == b'_')
            {
                end += 1;
            } else {
                break;
            }
        }
        self.cap(group, quote, end);
        Some(end)
    }

    /// `((?:[LUu]8?)?R)"\(`: the start of a raw string, group 1 its prefix.
    ///
    /// The prefix is optional: without it the `R` must stand at `p`.
    pub(super) fn raw_open(&mut self, p: usize) -> Option<usize> {
        let mut r = p;
        if matches!(self.at(p), b'L' | b'U' | b'u') && p < self.lim() {
            r = p + 1;
            if self.at(r) == b'8' && r < self.lim() {
                r += 1;
            }
        }
        if !(self.at(r) == b'R' && r < self.lim()) {
            if r != p && self.at(p + 1) == b'R' {
                r = p + 1;
            } else if r == p || self.at(p) != b'R' {
                return None;
            } else {
                r = p;
            }
        }
        let end = r + 1;
        if !self.starts(end, b"\"(") {
            return None;
        }
        self.cap(1, p, end);
        Some(end + 2)
    }

    /// `(?<=}|%>|\?\?>)`: whether `}`, `%>` or `??>` ends at `p`.
    pub(super) fn after_close(&self, p: usize) -> bool {
        self.prev(p) == Some(b'}') || self.ends_with_at(p, b"%>") || self.ends_with_at(p, b"??>")
    }

    /// `(/\*)((?:[^*]++|\*+(?!/))*+(\*/))`: a comment that ends in the line,
    /// with the marker as group 1, the rest as group 2 and the end as group 3.
    pub(super) fn inline_comment(&mut self, p: usize) -> Option<usize> {
        if !self.starts(p, b"/*") {
            return None;
        }
        let at = memchr::memmem::find(&self.s[p + 2..], b"*/")?;
        let close = p + 2 + at;
        self.cap(1, p, p + 2);
        self.cap(2, p + 2, close + 2);
        self.cap(3, close, close + 2);
        Some(close + 2)
    }

    /// `\s*+(/\*)((?:[^*]++|\*+(?!/))*+(\*/))\s*+`: a comment with the white
    /// space around it.
    pub(super) fn spaced_comment(&mut self, p: usize) -> Option<usize> {
        let q = self.skip_space(p);
        let end = self.inline_comment(q)?;
        Some(self.skip_space(end))
    }

    /// `(/\*[!*]+(?=\s))(.+)([!*]*\*/)`: a documentation comment that ends in
    /// its line.
    ///
    /// `.+` takes the line and gives back until `[!*]*\*/` fits: the last `*/`
    /// that leaves a character in the body.
    pub(super) fn doc_block_one(&mut self, p: usize) -> Option<usize> {
        if !self.starts(p, b"/*") {
            return None;
        }
        let marker = self.span_of(p + 2, |c| c == '!' || c == '*');
        if marker == p + 2 || self.space_at(marker) == 0 {
            return None;
        }
        let body = self.s.get(marker..)?;
        let at = memchr::memmem::rfind(body, b"*/")?;
        let close = marker + at;
        if close == marker {
            return None;
        }
        self.cap(1, p, marker);
        self.cap(2, marker, close);
        self.cap(3, close, close + 2);
        Some(close + 2)
    }

    /// `\s+{1,0}/\*[!*]+(?:(?:\n|$)|(?=\s))`: the start of a documentation
    /// comment that goes on in the next lines.
    pub(super) fn doc_block_begin(&self, p: usize) -> Option<usize> {
        let q = self.skip_space(p);
        if !self.starts(q, b"/*") {
            return None;
        }
        let end = self.span_of(q + 2, |c| c == '!' || c == '*');
        if end == q + 2 {
            return None;
        }
        if self.nl && end == self.lim() {
            Some(end + 1)
        } else if self.at_eol(end) || self.space_at(end) > 0 {
            Some(end)
        } else {
            None
        }
    }

    /// `(?<!\w)(template)\s+{0,1}(<)(.*)(>)\s+{0,1}$`.
    ///
    /// `(.*)(>)\s*$`: the last character of the line that is not white space
    /// has to be the `>`.
    pub(super) fn template_isolated(&mut self, p: usize) -> Option<usize> {
        let word = self.word_prefix(p, b"template")?;
        let open = self.skip_space(word);
        if self.at(open) != b'<' || open >= self.lim() {
            return None;
        }
        let mut last = self.lim();
        while last > open + 1 {
            let c = self.ch_before(last)?;
            if !is_space(c) {
                break;
            }
            last -= c.len_utf8();
        }
        if last <= open + 1 || self.s[last - 1] != b'>' {
            return None;
        }
        self.cap(1, p, word);
        self.cap(2, open, open + 1);
        self.cap(3, open + 1, last - 1);
        self.cap(4, last - 1, last);
        Some(self.lim() + usize::from(self.nl))
    }

    /// `([A-Z_a-z][0-9A-Z_a-z]*|(?<=[])]))?(\[)(?!])`.
    pub(super) fn square_brackets(&mut self, p: usize) -> Option<usize> {
        let open_ok = |lexer: &Self, at: usize| {
            lexer.at(at) == b'[' && at < lexer.lim() && lexer.at(at + 1) != b']'
        };
        if let Some(end) = self.ascii_ident(p)
            && open_ok(self, end)
        {
            self.cap(1, p, end);
            self.cap(2, end, end + 1);
            return Some(end + 1);
        }
        if !open_ok(self, p) {
            return None;
        }
        self.cap(2, p, p + 1);
        Some(p + 1)
    }

    /// `(?:(?<=}|%>|\?\?>)\s+{0,1}(;)|(;))|(?=[];=>\[])`.
    pub(super) fn typedef_end(&mut self, p: usize) -> Option<usize> {
        if self.after_close(p) {
            let q = self.skip_space(p);
            if self.at(q) == b';' && q < self.lim() {
                self.cap(1, q, q + 1);
                return Some(q + 1);
            }
        }
        if self.at(p) == b';' && p < self.lim() {
            self.cap(2, p, p + 1);
            return Some(p + 1);
        }
        (p < self.lim() && b"];=>[".contains(&self.s[p])).then_some(p)
    }

    /// `((?:[-%*+]|(?<!\()/)=)|((?:[\&^]|<<|>>|\|)=)|(=)`.
    pub(super) fn assign_operators(&mut self, p: usize) -> Option<usize> {
        self.assign_at(p, 1)
    }

    /// The assignment operators with their groups from `base`.
    pub(super) fn assign_at(&mut self, p: usize, base: usize) -> Option<usize> {
        let (group, end) = self.assign_op(p)?;
        self.cap(base + group, p, end);
        Some(end)
    }

    /// The assignment operator at `p`: which of its three groups it takes
    /// (arithmetic, bitwise or shift, plain `=`), counted from 0, and where
    /// it ends.
    pub(super) fn assign_op(&self, p: usize) -> Option<(usize, usize)> {
        let lead = self.at(p);
        if p >= self.lim() {
            return None;
        }
        let arithmetic = matches!(lead, b'-' | b'%' | b'*' | b'+')
            || (lead == b'/' && self.prev(p) != Some(b'('));
        if arithmetic && self.at(p + 1) == b'=' {
            return Some((0, p + 2));
        }
        let len = if matches!(lead, b'&' | b'^' | b'|') {
            1
        } else if self.starts(p, b"<<") || self.starts(p, b">>") {
            2
        } else {
            0
        };
        if len > 0 && self.at(p + len) == b'=' {
            return Some((1, p + len + 1));
        }
        (lead == b'=').then_some((2, p + 1))
    }
}
