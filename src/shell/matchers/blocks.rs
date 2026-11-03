//! Loops, `case`, functions, groups, tests, here-documents and literal ends.

use super::is_blank;
use crate::shell::engine::{DynName, Lazy, Lexer, NameRun};
use crate::shell::scope::S;
use crate::shell::state::Rule;

/// The two-letter tests of `[ ]`.
const TEST_PAIRS: [&[u8]; 9] = [
    // The second is spelt with an escape, which the spelling checker needs.
    b"nt", b"\x6ft", b"ef", b"eq", b"ne", b"le", b"lt", b"ge", b"gt",
];

/// What may follow a keyword the grammar fences with separators.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::shell) enum Follow {
    /// `(?=[\&;\s]|$)`.
    Separator,
    /// `(?=[\&;\s]|$|\))`.
    SeparatorOrParen,
}

/// Which here-document operator a begin takes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::shell) enum Heredoc {
    /// `<<-`, whose body may be indented with tabs.
    Indented,
    /// `<<`.
    Plain,
}

impl Lexer<'_> {
    /// `(?<=^|[\&;\s])WORD` and then what `follow` allows.
    pub(in crate::shell) fn m_keyword_between(
        &mut self,
        p: usize,
        word: &[u8],
        scopes: &'static [S],
        follow: Follow,
    ) -> Option<usize> {
        if !self.after_separator(p) || !self.starts(p, word) {
            return None;
        }
        let e = p + word.len();
        let paren = follow == Follow::SeparatorOrParen && self.avail(e) && self.at(e) == b')';
        if !(self.before_separator(e) || paren) {
            return None;
        }
        self.span(p, e, scopes);
        Some(e)
    }

    /// `\bWORD\b`
    pub(in crate::shell) fn m_keyword_word(
        &mut self,
        p: usize,
        word: &[u8],
        scopes: &'static [S],
    ) -> Option<usize> {
        if !self.not_after_word(p) || !self.starts(p, word) || self.word_at(p + word.len()) {
            return None;
        }
        self.span(p, p + word.len(), scopes);
        Some(p + word.len())
    }

    /// `done` ending a loop.
    pub(in crate::shell) fn m_end_done(&mut self, p: usize) -> Option<usize> {
        self.m_keyword_between(p, b"done", &[S::KeywordControl], Follow::SeparatorOrParen)
    }

    /// `(?<=^|[\&;\s])(while|until)(?=[\&;\s]|$)`
    pub(in crate::shell) fn m_loop_while(&mut self, p: usize) -> Option<usize> {
        self.m_keyword_between(p, b"while", &[S::KeywordControl], Follow::Separator)
            .or_else(|| {
                self.m_keyword_between(p, b"until", &[S::KeywordControl], Follow::Separator)
            })
    }

    /// `(?<=^|[\&;\s])(for)\s+(.+?)\s+(in)(?=[\&;\s]|$)`. The variable is
    /// lazy: it ends where white space and `in` follow.
    pub(in crate::shell) fn m_loop_for(&mut self, p: usize) -> Option<usize> {
        if !self.after_separator(p) || !self.starts(p, b"for") {
            return None;
        }
        let w = self.skip_space(p + 3);
        if w == p + 3 || w >= self.lim {
            return None;
        }
        let e = self.find_from(Lazy::ForVariable, self.char_end(w), |lexer, e| {
            let q = lexer.skip_space(e);
            q > e && lexer.starts(q, b"in") && lexer.before_separator(q + 2)
        })?;
        let q = self.skip_space(e);
        self.span(p, p + 3, &[S::KeywordControl]);
        self.span_retok(w, e, &[S::VariableOtherLoop], Rule::RetokString);
        self.span(q, q + 2, &[S::KeywordControl]);
        Some(q + 2)
    }

    /// `(?<=^|[\&;\s])(select)\s+((?:[^\\\s]|\\.)+)(?=[\&;\s]|$)`
    pub(in crate::shell) fn m_loop_select(&mut self, p: usize) -> Option<usize> {
        if !self.after_separator(p) || !self.starts(p, b"select") {
            return None;
        }
        let w = self.skip_space(p + 6);
        if w == p + 6 {
            return None;
        }
        let mut ends = std::mem::take(&mut self.ends);
        ends.clear();
        let mut e = w;
        while e < self.lim {
            self.spend(1);
            let b = self.s[e];
            if b == b'\\' {
                if e + 1 >= self.lim {
                    break;
                }
                e = self.char_end(e + 1);
            } else if self.space_at(e) {
                break;
            } else {
                e = self.char_end(e);
            }
            ends.push(e);
        }
        let found = ends.iter().rev().copied().find(|&end| {
            self.spend(1);
            self.before_separator(end)
        });
        self.ends = ends;
        let e = found?;
        self.span(p, p + 6, &[S::KeywordControl]);
        self.span(w, e, &[S::VariableOtherLoop]);
        Some(e)
    }

    /// `\b(for)\b[\t ]*+(name)[\t ]*+\b(in)\b`
    pub(in crate::shell) fn m_for_in(&mut self, p: usize) -> Option<usize> {
        if !self.not_after_word(p) || !self.starts(p, b"for") || self.word_at(p + 3) {
            return None;
        }
        let n = self.blanks(p + 3);
        let ne = self.name_run(n)?;
        let i = self.blanks(ne);
        if !self.starts(i, b"in") || !self.boundary(i) || self.word_at(i + 2) {
            return None;
        }
        self.span(p, p + 3, &[S::KeywordControlFor]);
        self.span(n, ne, &[S::VariableOtherFor]);
        self.span(i, i + 2, &[S::KeywordControlIn]);
        Some(i + 2)
    }

    /// `\b(for)\b`
    pub(in crate::shell) fn m_for(&mut self, p: usize) -> Option<usize> {
        if !self.not_after_word(p) || !self.starts(p, b"for") || self.word_at(p + 3) {
            return None;
        }
        self.span(p, p + 3, &[S::KeywordControlFor]);
        Some(p + 3)
    }

    // `case`.

    /// `\b(case)\b[\t ]*+(.+?)[\t ]*+\b(in)\b`
    ///
    /// The expression is lazy: it ends where `in` follows.
    pub(in crate::shell) fn m_case(&mut self, p: usize) -> Option<usize> {
        if !self.not_after_word(p) || !self.starts(p, b"case") || self.word_at(p + 4) {
            return None;
        }
        let w = self.blanks(p + 4);
        if w >= self.lim {
            return None;
        }
        let e = self.find_from(Lazy::CaseWord, self.char_end(w), |lexer, e| {
            let i = lexer.skip(e, is_blank);
            lexer.starts(i, b"in") && lexer.boundary(i) && !lexer.word_at(i + 2)
        })?;
        let i = self.blanks(e);
        self.span(p, p + 4, &[S::KeywordControlCase]);
        self.span_retok(w, e, &[], Rule::RetokInitial);
        self.span(i, i + 2, &[S::KeywordControlIn]);
        Some(i + 2)
    }

    /// `[\t ]*+(\* *\))`
    pub(in crate::shell) fn m_case_default(&mut self, p: usize) -> Option<usize> {
        let j = self.blanks(p);
        if self.at(j) != b'*' || !self.avail(j) {
            return None;
        }
        let k = self.skip(j + 1, |b| b == b' ');
        if self.at(k) != b')' || !self.avail(k) {
            return None;
        }
        self.span(j, k + 1, &[S::KeywordOperatorPatternCaseDefault]);
        Some(k + 1)
    }

    /// `\besac\b` ahead.
    fn esac_ahead(&self, p: usize) -> bool {
        self.not_after_word(p) && self.starts(p, b"esac") && !self.word_at(p + 4)
    }

    /// `(?<!\))(?![\t ]*+(?:esac\b|$))`
    pub(in crate::shell) fn m_case_pattern_begin(&mut self, p: usize) -> Option<usize> {
        if self.prev(p) == Some(b')') {
            return None;
        }
        let j = self.blanks(p);
        if self.esac_ahead(j) || self.at_eol(j) {
            return None;
        }
        Some(p)
    }

    /// `(?<=\S)(\()`
    pub(in crate::shell) fn m_case_group(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b'(' || !self.avail(p) || self.prev(p).is_none() || self.space_before(p) {
            return None;
        }
        self.span(
            p,
            p + 1,
            &[
                S::PunctuationDefinitionGroup,
                S::PunctuationDefinitionRegexGroup,
            ],
        );
        Some(p + 1)
    }

    /// `(?<=\tin| in|[\t ]|;;)\(`
    pub(in crate::shell) fn m_case_paren_open(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b'(' || !self.avail(p) {
            return None;
        }
        let after_in = p >= 3 && matches!(&self.s[p - 3..p], b" in" | b"\tin");
        let after_pair = p >= 2 && &self.s[p - 2..p] == b";;";
        if !(self.prev(p).is_some_and(is_blank) || after_in || after_pair) {
            return None;
        }
        self.span(p, p + 1, &[S::KeywordOperatorPatternCase]);
        Some(p + 1)
    }

    /// `[^\t\n )*?@\[|]`
    pub(in crate::shell) fn m_case_char(&mut self, p: usize) -> Option<usize> {
        if p >= self.lim
            || matches!(
                self.s[p],
                b'\t' | b' ' | b')' | b'*' | b'?' | b'@' | b'[' | b'|'
            )
        {
            return None;
        }
        let e = self.char_end(p);
        self.span(p, e, &[S::StringUnquotedPattern, S::StringRegexpUnquoted]);
        Some(e)
    }

    /// `\besac\b`
    pub(in crate::shell) fn m_end_case(&mut self, p: usize) -> Option<usize> {
        if !self.esac_ahead(p) {
            return None;
        }
        self.span(p, p + 4, &[S::KeywordControlEsac]);
        Some(p + 4)
    }

    /// `(?=\besac\b)|(\))`
    pub(in crate::shell) fn m_end_case_pattern(&mut self, p: usize) -> Option<usize> {
        if self.esac_ahead(p) {
            return Some(p);
        }
        self.m_single(p, b')', &[S::KeywordOperatorPatternCase])
    }

    /// `(;;)|(?=\besac\b)`
    pub(in crate::shell) fn m_end_case_body(&mut self, p: usize) -> Option<usize> {
        if self.starts(p, b";;") {
            self.span(p, p + 2, &[S::PunctuationTerminatorStatementCase]);
            return Some(p + 2);
        }
        self.esac_ahead(p).then_some(p)
    }

    // Functions and groups.

    /// `[^\t\n\r "'()=]`
    fn is_function_name(b: u8) -> bool {
        !matches!(
            b,
            b'\t' | b'\n' | b'\r' | b' ' | b'"' | b'\'' | b'(' | b')' | b'='
        )
    }

    /// The end of the function-name characters from `i`. The run is
    /// remembered, so a search from inside it, as a pattern tried at every
    /// byte of a long word makes, does not scan it again.
    fn function_name_end(&self, i: usize) -> usize {
        let run = self.function_name_run.get();
        if run.lim == self.lim && run.start <= i && i < run.end {
            return run.end;
        }
        let e = self.skip(i, Self::is_function_name);
        self.function_name_run.set(NameRun {
            start: i,
            end: e,
            lim: self.lim,
        });
        e
    }

    /// `function_definition`'s begin.
    pub(in crate::shell) fn m_function(&mut self, p: usize) -> Option<usize> {
        let j = self.blanks(p);
        if self.starts(j, b"function") && self.not_after_word(j) && !self.word_at(j + 8) {
            let n0 = self.blanks(j + 8);
            let n1 = self.function_name_end(n0);
            if n1 > n0 {
                self.span(j, j + 8, &[S::StorageTypeFunction]);
                self.span(n0, n1, &[S::EntityNameFunction]);
                if self.at(n1) == b'(' && self.avail(n1) {
                    let c = self.blanks(n1 + 1);
                    if self.at(c) == b')' && self.avail(c) {
                        self.span(n1, n1 + 1, &[S::PunctuationDefinitionArguments]);
                        self.span(c, c + 1, &[S::PunctuationDefinitionArguments]);
                        return Some(c + 1);
                    }
                }
                return Some(n1);
            }
        }
        self.clear_spans();
        let n1 = self.function_name_end(j);
        if n1 == j {
            return None;
        }
        let o = self.blanks(n1);
        if self.at(o) != b'(' || !self.avail(o) {
            return None;
        }
        let c = self.blanks(o + 1);
        if self.at(c) != b')' || !self.avail(c) {
            return None;
        }
        self.span(j, n1, &[S::EntityNameFunction]);
        self.span(o, o + 1, &[S::PunctuationDefinitionArguments]);
        self.span(c, c + 1, &[S::PunctuationDefinitionArguments]);
        Some(c + 1)
    }

    /// `\G[\t\n ]`
    pub(in crate::shell) fn m_function_blank(&mut self, p: usize) -> Option<usize> {
        (self.at_g(p) && self.avail(p) && matches!(self.at(p), b'\t' | b'\n' | b' '))
            .then_some(p + 1)
    }

    /// `(?<![^\t ])(\{)(?![$\w])`
    pub(in crate::shell) fn m_group(&mut self, p: usize) -> Option<usize> {
        if !self.prev(p).is_none_or(is_blank) || self.at(p) != b'{' || !self.avail(p) {
            return None;
        }
        if self.avail(p + 1) && (self.at(p + 1) == b'$' || self.word_at(p + 1)) {
            return None;
        }
        self.span(p, p + 1, &[S::PunctuationDefinitionGroup]);
        Some(p + 1)
    }

    // Tests.

    /// `=[=~]?|!=?|[<>]|&&|\|\|`
    pub(in crate::shell) fn m_logical_op(&mut self, p: usize) -> Option<usize> {
        if !self.avail(p) {
            return None;
        }
        let next = self.at(p + 1);
        let live = self.avail(p + 1);
        let len = match self.at(p) {
            b'=' => 1 + usize::from(live && matches!(next, b'=' | b'~')),
            b'!' => 1 + usize::from(live && next == b'='),
            b'<' | b'>' => 1,
            b'&' if live && next == b'&' => 2,
            b'|' if live && next == b'|' => 2,
            _ => return None,
        };
        self.span(p, p + len, &[S::KeywordOperatorLogical]);
        Some(p + len)
    }

    /// A `[ ]` test such as `-eq` or `-f`:
    /// `(?<!\S)-(nt|…|[GLNOSa-hknopr-uwxz])\b`
    pub(in crate::shell) fn m_logical_test(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b'-' || !self.avail(p) || !(p == 0 || self.space_before(p)) {
            return None;
        }
        for pair in TEST_PAIRS {
            if self.starts(p + 1, pair) && !self.word_at(p + 3) {
                self.span(p, p + 3, &[S::KeywordOperatorLogical]);
                return Some(p + 3);
            }
        }
        let c = self.at(p + 1);
        if self.avail(p + 1)
            && (matches!(
                c,
                b'G' | b'L' | b'N' | b'O' | b'S' | b'k' | b'n' | b'o' | b'p' | b'w' | b'x' | b'z'
            ) || (b'a'..=b'h').contains(&c)
                || (b'r'..=b'u').contains(&c))
            && !self.word_at(p + 2)
        {
            self.span(p, p + 2, &[S::KeywordOperatorLogical]);
            return Some(p + 2);
        }
        None
    }

    // Here-documents.

    /// `((?<!<)<<-)` or `((?<!<)<<(?!<))`: where the operator ends.
    fn heredoc_operator(&self, p: usize, heredoc: Heredoc) -> Option<usize> {
        if self.prev(p) == Some(b'<') {
            return None;
        }
        match heredoc {
            Heredoc::Indented => self.starts(p, b"<<-").then_some(p + 3),
            Heredoc::Plain => (self.starts(p, b"<<")
                && !(self.avail(p + 2) && self.at(p + 2) == b'<'))
                .then_some(p + 2),
        }
    }

    /// `<<`, a quote, the delimiter and the quote again.
    pub(in crate::shell) fn m_heredoc_quoted(
        &mut self,
        p: usize,
        heredoc: Heredoc,
    ) -> Option<usize> {
        let a = self.heredoc_operator(p, heredoc)?;
        let b = self.blanks(a);
        let quote = self.at(b);
        if !self.avail(b) || !matches!(quote, b'"' | b'\'') {
            return None;
        }
        let d0 = self.blanks(b + 1);
        let mut e = d0;
        while e < self.lim && !matches!(self.s[e], b'"' | b'\'') {
            self.spend(1);
            e += 1;
        }
        if e == d0 || e >= self.lim || self.s[e] != quote {
            return None;
        }
        self.set_delimiter(d0, e)?;
        self.span(p, a, &[S::KeywordOperatorHeredoc]);
        self.span(b, b + 1, &[S::PunctuationDefinitionStringHeredocQuote]);
        self.span(d0, e, &[S::PunctuationDefinitionStringHeredocDelimiter]);
        self.span(e, e + 1, &[S::PunctuationDefinitionStringHeredocQuote]);
        self.span_retok(e + 1, self.lim, &[], Rule::RetokHeredocRest);
        Some(self.lim)
    }

    /// `<<` and an unquoted delimiter. Where no line feed follows, the
    /// lookahead after the delimiter needs one of `&;<`, so the delimiter
    /// ends before the last of them.
    pub(in crate::shell) fn m_heredoc_plain(
        &mut self,
        p: usize,
        heredoc: Heredoc,
    ) -> Option<usize> {
        let a = self.heredoc_operator(p, heredoc)?;
        let d0 = self.blanks(a);
        let mut e = self.skip(d0, |b| !matches!(b, b'\t' | b' ' | b'"' | b'\''));
        if e == d0 {
            return None;
        }
        if !self.avail(e) {
            self.spend(e - d0);
            e = (d0 + 1..e)
                .rev()
                .find(|&k| matches!(self.s[k], b'&' | b';' | b'<'))?;
        }
        self.set_delimiter(d0, e)?;
        self.span(p, a, &[S::KeywordOperatorHeredoc]);
        self.span(d0, e, &[S::PunctuationDefinitionStringHeredocDelimiter]);
        self.span_retok(e, self.lim, &[], Rule::RetokHeredocRest);
        Some(self.lim)
    }

    /// `^\t*DELIMITER(?=[\&;\s]|$)` ends a here-document.
    pub(in crate::shell) fn m_end_heredoc(&mut self, p: usize) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let rule = self.top_rule();
        let indent = matches!(rule, Rule::HeredocQuotedIndent | Rule::HeredocIndent);
        let start = if indent {
            self.skip(0, |b| b == b'\t')
        } else {
            0
        };
        let e = self.string_end(start, self.top_payload())?;
        if !self.before_separator(e) {
            return None;
        }
        if rule == Rule::HeredocQuotedIndent {
            self.span_dyn(
                0,
                e,
                &[],
                DynName {
                    prefix: "punctuation.definition.string.heredoc.",
                    start: 0,
                    end: e,
                    suffix: ".shell",
                },
            );
        } else {
            self.span(0, e, &[S::PunctuationDefinitionStringHeredocDelimiter]);
        }
        Some(e)
    }

    // Other ends.

    /// `(?<=\W)(?=#)(?!#\{)`
    pub(in crate::shell) fn m_backtick_comment(&mut self, p: usize) -> Option<usize> {
        let after_nonword = p > 0 && !self.word_before(p);
        let hash = self.at(p) == b'#' && self.avail(p);
        let brace = self.at(p + 1) == b'{' && self.avail(p + 1);
        (after_nonword && hash && !brace).then_some(p)
    }

    /// `\)\s*\)`
    pub(in crate::shell) fn m_end_arith_double(&mut self, p: usize) -> Option<usize> {
        if self.at(p) != b')' || !self.avail(p) {
            return None;
        }
        let j = self.skip_space(p + 1);
        if self.at(j) != b')' || !self.avail(j) {
            return None;
        }
        self.span(p, j + 1, &[S::PunctuationSectionArithmeticDouble]);
        Some(j + 1)
    }

    /// The literal that ends the innermost rule.
    pub(in crate::shell) fn m_end_lit(&mut self, p: usize) -> Option<usize> {
        let (text, scopes) = self.top_end_lit();
        if !self.starts(p, text) {
            return None;
        }
        self.span(p, p + text.len(), scopes);
        Some(p + text.len())
    }
}
