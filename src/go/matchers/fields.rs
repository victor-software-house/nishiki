//! The fields of structs and interfaces, and the declarations the language
//! server's hover text shows.

use super::super::engine::Lexer;
use super::params::Commas;
use super::types::Lead;
use crate::rules::Chars;

impl Lexer<'_> {
    /// `\b(\w+(?:,\s*\b\w+)*(?:\s*CH)+{0,1}\s*B)\b(struct)\b\s*(\{)`
    pub(in crate::go) fn struct_variable_types_fields_multi_at0(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        self.fields_then(p, b"struct", b'{')
    }

    /// `\b(\w+(?:,\s*\b\w+)*(?:\s*CH)+{0,1}\s*B)\b(interface)\b\s*(\{)`
    pub(in crate::go) fn struct_variable_types_fields_multi_at1(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        self.fields_then(p, b"interface", b'{')
    }

    /// `\b(\w+(?:,\s*\b\w+)*(?:\s*CH)+{0,1}\s*B)\b(func)\b\s*(\()`
    pub(in crate::go) fn struct_variable_types_fields_multi_at2(
        &mut self,
        p: usize,
    ) -> Option<usize> {
        self.fields_then(p, b"func", b'(')
    }

    /// Field names, a `chan` prefix, marks, then `word` and `open`.
    ///
    /// A shorter list of names leaves a comma or a word character, where
    /// neither a prefix nor `word` can start.
    fn fields_then(&mut self, p: usize, word: &[u8], open: u8) -> Option<usize> {
        if self.word_before(p) {
            return None;
        }
        let names = self.names(p, Self::word_name, Commas::Tight)?;
        let (marks, end, at) = self.after_chans(names, Lead::Spaces, |lx, q, _| {
            let marks = lx.marks(lx.spaces(q));
            let end = lx.keyword(marks, word)?;
            let at = lx.spaces(end);
            (at < lx.lim() && lx.at(at) == open).then_some((marks, end, at))
        })?;
        self.cap(1, p, marks);
        self.cap(2, marks, end);
        self.cap(3, at, at + 1);
        Some(at + 1)
    }

    /// Whether `{` stands before `p`: `(?<=\{)`.
    fn after_open_curly(&self, p: usize) -> bool {
        p > 0 && p <= self.lim() && self.at(p - 1) == b'{'
    }

    /// The end of `[]*.\[\w]+\s*(?=})` at `q`, and where the `}` stands.
    fn type_before_close(&self, q: usize) -> Option<(usize, usize)> {
        let end = self.type_run(q);
        let close = self.spaces(end);
        (end > q && close < self.lim() && self.at(close) == b'}').then_some((end, close))
    }

    /// `(?<=\{)\s*((?:\s*CH)+{0,1}[]*.\[\w]+)\s*(?=})`
    pub(in crate::go) fn struct_variables_types_fields_at1(&mut self, p: usize) -> Option<usize> {
        if !self.after_open_curly(p) {
            return None;
        }
        let start = self.spaces(p);
        let (end, close) =
            self.after_chans(start, Lead::Spaces, |lx, q, _| lx.type_before_close(q))?;
        self.cap(1, start, end);
        Some(close)
    }

    /// `(?:\w+,\s*)+{0,1}\w+\s+` at `p`: field names and the blanks after
    /// them.
    /// It returns the end of the blanks.
    fn field_names(&self, p: usize) -> Option<usize> {
        let names = self.names(p, Self::word_name, Commas::Tight)?;
        let blank = self.spaces(names);
        (blank > names).then_some(blank)
    }

    /// `(?<=\{)\s*((?:\w+,\s*)+{0,1}\w+\s+)((?:\s*CH)+{0,1}[]*.\[\w]+)\s*(?=})`
    pub(in crate::go) fn struct_variables_types_fields_at2(&mut self, p: usize) -> Option<usize> {
        if !self.after_open_curly(p) {
            return None;
        }
        let start = self.spaces(p);
        let names = self.field_names(start)?;
        let (end, close) =
            self.after_chans(names, Lead::Spaces, |lx, q, _| lx.type_before_close(q))?;
        self.cap(1, start, names);
        self.cap(2, names, end);
        Some(close)
    }

    /// The end of `` [^"/`\s]+ `` at `p`.
    fn field_type_end(&self, p: usize) -> usize {
        let mut i = p;
        while i < self.lim() && !matches!(self.at(i), b'"' | b'/' | b'`') && self.space_at(i) == 0 {
            i = self.char_end(i);
        }
        self.spend(i - p + 1);
        i
    }

    /// `` ((?:\w+,\s*)+{0,1}\w+\s+)?((?:\s*CH)+{0,1}[^"/`\s]+;?) ``
    ///
    /// The type's run takes `;`, so the `;?` after it never does.
    pub(in crate::go) fn nested34_at0(&mut self, p: usize) -> Option<usize> {
        let field_type = |lx: &Self, start: usize| {
            lx.after_chans(start, Lead::Spaces, |lx, q, _| {
                let end = lx.field_type_end(q);
                (end > q).then_some(end)
            })
        };
        if let Some(names) = self.field_names(p)
            && let Some(end) = field_type(self, names)
        {
            self.cap(1, p, names);
            self.cap(2, names, end);
            return Some(end);
        }
        let end = field_type(self, p)?;
        self.cap(2, p, end);
        Some(end)
    }

    /// `` (?<=\{)((?:\s*(?:N\s+)?(?:\s*CH)+{0,1}[^"/`\s]+;?)+)\s*(?=}) ``, with
    /// N field names.
    ///
    /// The repeats match any run of blanks and of the type's characters that
    /// holds one of the latter, and a backtracking matcher reaches the
    /// rightmost end before a `}` first: the last `}` before a character
    /// the type cannot take, less the blanks before it.
    pub(in crate::go) fn struct_variables_types_fields_at3(&mut self, p: usize) -> Option<usize> {
        if !self.after_open_curly(p) {
            return None;
        }
        let mut stop = p;
        while stop < self.lim() && !matches!(self.at(stop), b'"' | b'/' | b'`') {
            stop += 1;
        }
        self.spend(stop - p + 1);
        let close = p + self.text(p, stop).iter().rposition(|&b| b == b'}')?;
        let mut end = close;
        while end > p && self.space_at(self.char_start_before(end)) > 0 {
            end = self.char_start_before(end);
        }
        if end == p {
            return None;
        }
        self.cap(1, p, end);
        Some(close)
    }

    /// `` ((?:\s*CH)+{0,1}[*.\w]+\s*)(?:(?=["/`])|$) ``
    pub(in crate::go) fn struct_variables_types_fields_at4(&mut self, p: usize) -> Option<usize> {
        let end = self.after_chans(p, Lead::Spaces, |lx, q, _| {
            let name = lx.star_dot_words(q);
            let end = lx.spaces(name);
            let stops = end < lx.lim() && matches!(lx.at(end), b'"' | b'/' | b'`');
            (name > q && (stops || lx.at_eol(end))).then_some(end)
        })?;
        self.cap(1, p, end);
        Some(end)
    }

    /// `` \b(\w+(?:\s*,\s*\b\w+)*)\s*([^"/`]+) ``
    ///
    /// When nothing the last group can take follows the names and blanks,
    /// the matcher gives back the last blank, or else the last character of
    /// the last name, or else the last name and its comma.
    pub(in crate::go) fn struct_variables_types_fields_at5(&mut self, p: usize) -> Option<usize> {
        if self.word_before(p) {
            return None;
        }
        let names = self.names(p, Self::word_name, Commas::Spaced)?;
        let blank = self.spaces(names);
        let rest = self.untyped_end(blank);
        if rest > blank {
            self.cap(1, p, names);
            self.cap(2, blank, rest);
            return Some(rest);
        }
        if blank > names {
            self.cap(1, p, names);
            self.cap(2, self.char_start_before(blank), blank);
            return Some(blank);
        }
        let last = self.char_start_before(names);
        let names_start = self.last_name_start(names);
        if last > names_start {
            self.cap(1, p, last);
            self.cap(2, last, names);
            return Some(names);
        }
        if names_start == p {
            return None;
        }
        let shorter = self.names_before(p, names_start)?;
        let rest_start = self.spaces(shorter);
        self.cap(1, p, shorter);
        self.cap(2, rest_start, names);
        Some(names)
    }

    /// The end of `` [^"/`]* `` at `p`, with the line feed.
    fn untyped_end(&self, p: usize) -> usize {
        let stop = self
            .s
            .get(p..self.lim())
            .and_then(|rest| memchr::memchr3(b'"', b'/', b'`', rest))
            .map_or(self.text_end(), |at| p + at);
        self.spend(stop.saturating_sub(p) + 1);
        stop.max(p)
    }

    /// Where the name that ends at `end` starts.
    fn last_name_start(&self, end: usize) -> usize {
        let mut start = end;
        while start > 0 && self.word_before(start) {
            start = self.char_start_before(start);
        }
        start
    }

    /// The end of the list of names from `p` that stops before the name at
    /// `last`: the list without its last name.
    fn names_before(&self, p: usize, last: usize) -> Option<usize> {
        let mut end = self.word_name(p);
        loop {
            let comma = self.spaces(end);
            let next = self.spaces(comma + 1);
            if next >= last {
                return (end > p).then_some(end);
            }
            end = self.word_name(next);
        }
    }

    /// `(?<=^\bfield\b)\s+([*.\w]+)\s+([\s\S]+)`
    pub(in crate::go) fn hover_at0(&mut self, p: usize) -> Option<usize> {
        if !self.follows_first_word(p, b"field") {
            return None;
        }
        let name = self.spaces(p);
        let name_end = self.star_dot_words(name);
        if name == p || name_end == name {
            return None;
        }
        let rest = self.rest_after_blanks(name_end)?;
        self.cap(1, name, name_end);
        self.cap(2, rest, self.text_end());
        Some(self.text_end())
    }

    /// `(?<=^\breturns\b)\s+([\s\S]+)`
    pub(in crate::go) fn hover_at1(&mut self, p: usize) -> Option<usize> {
        if !self.follows_first_word(p, b"returns") {
            return None;
        }
        let rest = self.rest_after_blanks(p)?;
        self.cap(1, rest, self.text_end());
        Some(self.text_end())
    }

    /// `(?<=^\bword\b)` at `p`.
    fn follows_first_word(&self, p: usize, word: &[u8]) -> bool {
        p == word.len() && self.starts(0, word) && self.word_at(p).is_none()
    }

    /// Where `[\s\S]+` starts after `\s+` at `p`.
    /// It starts after the blanks, or on the last of them when they reach the
    /// end.
    fn rest_after_blanks(&self, p: usize) -> Option<usize> {
        let blank = self.spaces(p);
        if blank == p {
            return None;
        }
        if self.avail(blank) {
            return Some(blank);
        }
        let last = self.char_start_before(blank);
        (last > p).then_some(last)
    }
}
