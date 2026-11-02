//! Control statements: `switch`, `for`, `if` and the other statements that take
//! a parenthesised head.

use super::kind::Kind;
use super::lexer::Lexer;
use super::scope::S;

impl<const D: u8> Lexer<'_, D> {
    pub(crate) fn control_statement(&mut self, p: usize) -> Option<usize> {
        if !self.b(p).is_ascii_lowercase() {
            return None;
        }
        if let Some(r) = self.switch_statement(p).or_else(|| self.for_loop(p)) {
            return Some(r);
        }
        if let Some(e) = self.kw_of(p, &["catch", "finally", "throw", "try"]) {
            return self.tok(p, e, S::KeywordControlTrycatch);
        }
        if let Some(e) = self.kw_of(p, &["break", "continue", "goto"]) {
            let j = self.ws(e);
            if j > e
                && let Some(n) = self.ident(j)
                && self.kw_after(n)
            {
                self.one(p, e, &[S::KeywordControlLoop]);
                self.cap(Kind::Top, j, n, &[S::EntityNameLabel]);
                return Some(n);
            }
        }
        if let Some(e) = self.kw_of(p, &["break", "continue", "do", "goto", "while"]) {
            return self.tok(p, e, S::KeywordControlLoop);
        }
        if let Some(e) = self.kw(p, "return") {
            return self.open1(Kind::Return, p, e, &[S::KeywordControlFlow]);
        }
        if let Some(e) = self.kw_of(p, &["case", "default", "switch"]) {
            return self.tok(p, e, S::KeywordControlSwitch);
        }
        if let Some(r) = self.if_statement(p) {
            return Some(r);
        }
        if let Some(e) = self.kw_of(p, &["else", "if"]) {
            return self.tok(p, e, S::KeywordControlConditional);
        }
        if let Some(e) = self.kw(p, "with") {
            return self.tok(p, e, S::KeywordControlWith);
        }
        if let Some(e) = self.kw(p, "package") {
            return self.tok(p, e, S::KeywordControl);
        }
        if let Some(e) = self.kw(p, "debugger") {
            return self.tok(p, e, S::KeywordOtherDebugger);
        }
        None
    }

    fn switch_statement(&mut self, p: usize) -> Option<usize> {
        let e = self
            .kw_before(p)
            .then(|| self.starts(p, "switch").then_some(p + 6))
            .flatten()?;
        if !self.boundary(p) || self.b(self.ws(e)) != b'(' {
            return None;
        }
        self.open0(Kind::SwitchStmt, p)
    }

    /// The rules of a `switch` statement before its braces.
    pub(crate) fn switch_statement_rules(&mut self, p: usize) -> Option<usize> {
        if let Some(r) = self.comment(p) {
            return Some(r);
        }
        if self.kw_before(p) && self.starts(p, "switch") {
            let e = p + 6;
            let j = self.ws(e);
            if self.b(j) == b'(' && !self.full() {
                self.flush(p);
                self.cap(Kind::SwitchExpr, p, e, &[S::KeywordControlSwitch]);
                self.cap(Kind::SwitchExpr, j, j + 1, &[S::MetaBraceRound]);
                self.push(Kind::SwitchExpr);
                return Some(j + 1);
            }
        }
        if self.b(p) == b'{' {
            return self.open1(
                Kind::SwitchBlock,
                p,
                p + 1,
                &[S::PunctuationDefinitionBlock],
            );
        }
        None
    }

    pub(crate) fn switch_block_rules(&mut self, p: usize) -> Option<usize> {
        if let Some(e) = self.kw(p, "case") {
            return self.open1(Kind::CaseClause, p, e, &[S::KeywordControlSwitch]);
        }
        if self.starts(p, "default") && self.kw_before(p) && self.b(p + 7) == b':' {
            let e = p + 7;
            if self.kw_after(e) {
                return self.open1(Kind::CaseClause, p, e, &[S::KeywordControlSwitch]);
            }
        }
        if self.b(p) == b':' {
            let j = self.ws(p + 1);
            if self.b(j) == b'{' && !self.full() {
                self.flush(p);
                self.cap(
                    Kind::CaseBlock,
                    p,
                    p + 1,
                    &[
                        S::CaseClauseExpr,
                        S::PunctuationDefinitionSectionCaseStatement,
                    ],
                );
                self.cap(
                    Kind::CaseBlock,
                    j,
                    j + 1,
                    &[S::MetaBlock, S::PunctuationDefinitionBlock],
                );
                self.push(Kind::CaseBlock);
                return Some(j + 1);
            }
            return Some(self.tok2(
                p,
                p + 1,
                S::CaseClauseExpr,
                S::PunctuationDefinitionSectionCaseStatement,
            ));
        }
        self.statements(p)
    }

    fn tok2(&mut self, a: usize, b: usize, s1: S, s2: S) -> usize {
        self.one(a, b, &[s1, s2]);
        b
    }

    fn for_loop(&mut self, p: usize) -> Option<usize> {
        let e = self.kw(p, "for")?;
        let mut j = self.ws_comments(e);
        // `(\s+|(\s*/\*…\*/\s*))await`
        if j > e && self.starts(j, "await") {
            j = self.ws_comments(j + 5);
        }
        if self.b(j) != b'(' || self.full() {
            return None;
        }
        self.open1(Kind::ForLoop, p, e, &[S::KeywordControlLoop])
    }

    pub(crate) fn for_loop_rules(&mut self, p: usize) -> Option<usize> {
        if let Some(r) = self.comment(p) {
            return Some(r);
        }
        if let Some(e) = self.starts(p, "await").then_some(p + 5) {
            return self.tok(p, e, S::KeywordControlLoop);
        }
        if self.b(p) == b'(' {
            return self.open1(Kind::ForParen, p, p + 1, &[S::MetaBraceRound]);
        }
        None
    }

    pub(crate) fn for_paren_rules(&mut self, p: usize) -> Option<usize> {
        self.var_expr(p)
            .or_else(|| self.expression(p))
            .or_else(|| self.punctuation_semicolon(p))
    }

    /// `if (…)` not followed directly by a brace.
    fn if_statement(&mut self, p: usize) -> Option<usize> {
        if !(self.starts(p, "if") && self.kw_before(p) && self.boundary(p)) {
            return None;
        }
        let j = self.ws(p + 2);
        if self.b(j) != b'(' {
            return None;
        }
        let close = self.balanced(j, b'(', b')')?;
        // `\s*(?!\{)` backs off a blank, so only `){` fails.
        let ok = self.ws(close) > close || self.b(close) != b'{';
        if !ok {
            return None;
        }
        self.open0(Kind::IfOuter, p)
    }

    pub(crate) fn if_outer_rules(&mut self, p: usize) -> Option<usize> {
        if let Some(r) = self.comment(p) {
            return Some(r);
        }
        if self.kw_before(p) && self.starts(p, "if") {
            let j = self.ws(p + 2);
            if self.b(j) == b'(' && !self.full() {
                self.flush(p);
                self.cap(Kind::IfParen, p, p + 2, &[S::KeywordControlConditional]);
                self.cap(Kind::IfParen, j, j + 1, &[S::MetaBraceRound]);
                self.push(Kind::IfParen);
                return Some(j + 1);
            }
        }
        if let Some(r) = self.regex_after_paren(p) {
            return Some(r);
        }
        self.statements(p)
    }

    /// `(?<=\))\s*/(?![*/])(?=REGEX)`.
    fn regex_after_paren(&mut self, p: usize) -> Option<usize> {
        if self.prev_char(p) != Some(')') || self.full() {
            return None;
        }
        let q = self.ws(p);
        if self.b(q) != b'/' || matches!(self.b(q + 1), b'*' | b'/') || !self.regex_body(q, false) {
            return None;
        }
        self.flush(p);
        self.cap(
            Kind::Regex,
            q,
            q + 1,
            &[S::PunctuationDefinitionStringBegin],
        );
        self.push(Kind::Regex);
        Some(q + 1)
    }
}
