//! The function expressions: declarations, function types, their results,
//! and the built-in functions.

use super::super::engine::Lexer;
use super::types::Lead;
use crate::rules::{CachedRuns, Chars};

/// The built-in functions that `built_in_functions_at0` names.
const BUILT_INS: &[&[u8]] = &[
    b"append", b"cap", b"clear", b"close", b"complex", b"copy", b"delete", b"imag", b"len", b"max",
    b"min", b"panic", b"print", b"println", b"real", b"recover",
];

impl Lexer<'_> {
    /// `^\b(func)\b\s*(\([^)]+\)\s*)?(?:(\w+)(?=[(\[]))?`
    ///
    /// The optional groups end the expression.
    /// Each is taken when it matches and skipped otherwise.
    pub(in crate::go) fn function_declaration(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let word = self.keyword(p, b"func")?;
        let mut end = self.spaces(word);
        self.capture(1, p, word);
        if let Some(close) = self.filled_parens(end) {
            let after = self.spaces(close);
            self.capture(2, end, after);
            end = after;
        }
        if end < self.lim() && self.word_at(end).is_some() {
            let name = self.word_run(end);
            if name < self.lim() && matches!(self.at(name), b'(' | b'[') {
                self.capture(3, end, name);
                end = name;
            }
        }
        Some(end)
    }

    /// ```text
    /// (?<=\))\s*((?:CH)+{0,1}(?!B\b(?:struct|interface)\b)[-\]*.\[\w]+)?\s*(?=\{)
    /// ```
    pub(in crate::go) fn function_declaration_end(&mut self, p: usize) -> Option<usize> {
        if !self.after_close_paren(p) {
            return None;
        }
        let start = self.spaces(p);
        let result = self.after_chans(start, Lead::Tight, |lx, q, _| {
            if !lx.not_marked_word(q, &[b"interface", b"struct"]) {
                return None;
            }
            let end = lx.words_and(q, |b| matches!(b, b'-' | b']' | b'*' | b'.' | b'['));
            let open = lx.spaces(end);
            (end > q && lx.opens_curly(open)).then_some((end, open))
        });
        if let Some((end, open)) = result {
            self.capture(1, start, end);
            return Some(open);
        }
        self.opens_curly(start).then_some(start)
    }

    /// Whether `)` stands before `p`: `(?<=\))`.
    fn after_close_paren(&self, p: usize) -> bool {
        p > 0 && p <= self.lim() && self.at(p - 1) == b')'
    }

    /// Whether `{` stands at `p`.
    pub(in crate::go) fn opens_curly(&self, p: usize) -> bool {
        p < self.lim() && self.at(p) == b'{'
    }

    /// `(?<=\))\s*((?:\s*CH)+{0,1}[-\]*.<>\[\w]+\s*(?:/[*/].*)?)$`
    pub(in crate::go) fn function_declaration_at2(&mut self, p: usize) -> Option<usize> {
        if !self.after_close_paren(p) {
            return None;
        }
        let start = self.spaces(p);
        let end = self.after_chans(start, Lead::Spaces, |lx, q, _| {
            let end = lx.words_and(q, |b| {
                matches!(b, b'-' | b']' | b'*' | b'.' | b'<' | b'>' | b'[')
            });
            if end == q {
                return None;
            }
            let blank = lx.spaces(end);
            lx.trailing_comment(blank)
                .or_else(|| lx.at_eol(blank).then_some(blank))
        })?;
        self.capture(1, start, end);
        Some(end)
    }

    /// `\b(func)\b(\([^/]*?\)\s+\([^/]*?\))\s+(?=\{)`
    ///
    /// Each lazy run ends at the first `)` that lets the rest match.
    pub(in crate::go) fn functions_inline(&mut self, p: usize) -> Option<usize> {
        let word = self.keyword(p, b"func")?;
        if word >= self.lim() || self.at(word) != b'(' {
            return None;
        }
        let first_limit = self.find_byte(word + 1, b'/').unwrap_or(self.lim());
        let mut close = word + 1;
        while let Some(first) = self.find_byte(close, b')').filter(|&at| at < first_limit) {
            close = first + 1;
            let open = self.spaces(close);
            if open == close || open >= self.lim() || self.at(open) != b'(' {
                continue;
            }
            let second_limit = self.find_byte(open + 1, b'/').unwrap_or(self.lim());
            let mut inner = open + 1;
            while let Some(second) = self.find_byte(inner, b')').filter(|&at| at < second_limit) {
                inner = second + 1;
                let brace = self.spaces(inner);
                if brace > inner && self.opens_curly(brace) {
                    self.capture(1, p, word);
                    self.capture(2, word, inner);
                    return Some(brace);
                }
            }
        }
        None
    }

    /// `\b(func)\b(?=\()`
    pub(in crate::go) fn functions(&mut self, p: usize) -> Option<usize> {
        let word = self.keyword(p, b"func")?;
        (word < self.lim() && self.at(word) == b'(').then(|| {
            self.capture(1, p, word);
            word
        })
    }

    /// `(?<=\))(\s*(?:CH)+)?(\s*(?:A|B))?`, the results after a function's
    /// parameters, where
    /// A is `(?:B[*.\w]+)?(?:L|\([^)]+{0,1}\))?[*.\w]+{0,1}\s*(?=\{)` and
    /// B is `(?:B(?!\bfunc\b)[*.\w]+L?[*.\w]+{0,1}|\([^)]+{0,1}\))`, with L
    /// the bracketed list of [`Lexer::type_list`].
    ///
    /// Both groups are optional and end the expression.
    /// The first group therefore takes the longest prefix.
    pub(in crate::go) fn functions_end(&mut self, p: usize) -> Option<usize> {
        if !self.after_close_paren(p) {
            return None;
        }
        let blank = self.spaces(p);
        let chans = self.chans(blank, Lead::Tight);
        let start = if chans > blank {
            self.capture(1, p, chans);
            chans
        } else {
            p
        };
        if let Some(end) = self
            .result_before_body(start)
            .or_else(|| self.result_type(start))
        {
            self.capture(2, start, end);
            return Some(end);
        }
        Some(start)
    }

    /// The ends of `(?:B[*.\w]+)?` at `p`, longest first, ending with `p`.
    /// The marks are given back a character at a time.
    /// A `*` given back can start the run.
    fn marked_name_ends(
        &self,
        p: usize,
        mut each: impl FnMut(usize) -> Option<usize>,
    ) -> Option<usize> {
        let marks = self.marks(p);
        let mut last = usize::MAX;
        for start in (p..=marks).rev() {
            let end = self.star_dot_words(start);
            if end > start && end != last {
                last = end;
                if let Some(found) = each(end) {
                    return Some(found);
                }
            }
        }
        each(p)
    }

    /// Alternative A of [`Lexer::functions_end`] at `p`: where its `{` stands.
    fn result_before_body(&self, p: usize) -> Option<usize> {
        let start = self.spaces(p);
        let body = |at: usize| {
            let open = self.spaces(self.star_dot_words(at));
            self.opens_curly(open).then_some(open)
        };
        self.marked_name_ends(start, |name| {
            let list = self.type_list(name).or_else(|| self.parens(name));
            list.and_then(body).or_else(|| body(name))
        })
    }

    /// Alternative B of [`Lexer::functions_end`] at `p`: where it ends.
    fn result_type(&self, p: usize) -> Option<usize> {
        let start = self.spaces(p);
        let marks = self.marks(start);
        for name in (start..=marks).rev() {
            let end = self.star_dot_words(name);
            if end > name && self.keyword(name, b"func").is_none() {
                let list = self.type_list(end).unwrap_or(end);
                return Some(self.star_dot_words(list));
            }
        }
        self.parens(start)
    }

    /// `\b(append|capture|...|clear)\b(?=\()`
    pub(in crate::go) fn built_in_functions_at0(&mut self, p: usize) -> Option<usize> {
        let end = self.word_in(p, BUILT_INS)?;
        (end < self.lim() && self.at(end) == b'(').then(|| {
            self.capture(1, p, end);
            end
        })
    }

    /// `\b(new)\b(\()`
    pub(in crate::go) fn built_in_functions_at1(&mut self, p: usize) -> Option<usize> {
        let end = self.keyword(p, b"new")?;
        (end < self.lim() && self.at(end) == b'(').then(|| {
            self.capture(1, p, end);
            self.capture(2, end, end + 1);
            end + 1
        })
    }

    /// ```text
    /// \b(make)\b(\()((?:(?:CH)+(?:\([^)]+\))?)?B(?:(?!\bmap\b)[.\w]+)?(\[(?:\S+(?:,\s*\S+)*)?])?,?)?
    /// ```
    ///
    /// Every part of the third group is optional and ends the expression, so
    /// each is taken when it matches.
    pub(in crate::go) fn built_in_functions_at2(&mut self, p: usize) -> Option<usize> {
        let word = self.keyword(p, b"make")?;
        if word >= self.lim() || self.at(word) != b'(' {
            return None;
        }
        let start = word + 1;
        let mut i = self.chans(start, Lead::Tight);
        if i > start {
            i = self.filled_parens(i).unwrap_or(i);
        }
        i = self.marks(i);
        if self.keyword(i, b"map").is_none() {
            i = self.dot_words(i);
        }
        if let Some(end) = self.make_size(i) {
            self.capture(4, i, end);
            i = end;
        }
        if i < self.lim() && self.at(i) == b',' {
            i += 1;
        }
        self.capture(1, p, word);
        self.capture(2, word, start);
        self.capture(3, start, i);
        Some(i)
    }

    /// The end of `\[(?:\S+(?:,\s*\S+)*)?]` at `p`.
    ///
    /// Each `\S+` first takes a whole run of non-blanks, `]` and commas
    /// included.
    /// The list closes at the last `]` of the deepest run reached through runs
    /// that end with a comma.
    /// Failing that, it closes at the last `]` of an earlier run.
    fn make_size(&self, p: usize) -> Option<usize> {
        if p >= self.lim() || self.at(p) != b'[' {
            return None;
        }
        let open = p + 1;
        let mut found = None;
        let mut start = open;
        loop {
            let end = self.non_blanks(start);
            if end == start {
                break;
            }
            if let Some(close) = self.text(start + 1, end).iter().rposition(|&b| b == b']') {
                found = Some(start + 1 + close + 1);
            }
            if self.at(end - 1) != b',' {
                break;
            }
            start = self.spaces(end);
        }
        found.or_else(|| (open < self.lim() && self.at(open) == b']').then_some(open + 1))
    }

    /// The end of `\S*` at `p`, in the text.
    fn non_blanks(&self, p: usize) -> usize {
        let mut i = p;
        while i < self.lim() && self.space_at(i) == 0 {
            i = self.char_end(i);
        }
        self.spend(i - p + 1);
        i
    }

    /// `(\w+\s+)?([*.\w]+(?:\[(?:[*.\w]+(?:,\s+)?)+{0,1}])?)`
    pub(in crate::go) fn nested1_at0_at0(&mut self, p: usize) -> Option<usize> {
        if p < self.lim() && self.word_at(p).is_some() {
            let name = self.word_run(p);
            let blank = self.spaces(name);
            if blank > name
                && let Some(end) = self.parameter_type(blank)
            {
                self.capture(1, p, blank);
                self.capture(2, blank, end);
                return Some(end);
            }
        }
        let end = self.parameter_type(p)?;
        self.capture(2, p, end);
        Some(end)
    }

    /// The end of `[*.\w]+(?:\[(?:[*.\w]+(?:,\s+)?)+{0,1}])?` at `p`.
    fn parameter_type(&self, p: usize) -> Option<usize> {
        let name = self.star_dot_words(p);
        if name == p {
            return None;
        }
        if name >= self.lim() || self.at(name) != b'[' {
            return Some(name);
        }
        let mut i = name + 1;
        loop {
            let item = self.star_dot_words(i);
            if item == i {
                break;
            }
            i = item;
            if i < self.lim() && self.at(i) == b',' {
                let after = self.spaces(i + 1);
                if after > i + 1 {
                    i = after;
                }
            }
        }
        Some(if i < self.lim() && self.at(i) == b']' {
            i + 1
        } else {
            name
        })
    }
}
