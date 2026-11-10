//! Control statements: `switch`, `for`, `if` and the other statements that take
//! a parenthesised head.

use super::lexer::Lexer;
use super::regex::RegexForm;
use super::scope::S;
use super::state::Rule;

impl<const D: u8> Lexer<'_, D> {
    pub(crate) fn control_statement(&mut self, p: usize) -> Option<usize> {
        if !self.byte(p).is_ascii_lowercase() {
            return None;
        }
        if let Some(r) = self.switch_statement(p).or_else(|| self.for_loop(p)) {
            return Some(r);
        }
        if let Some(e) = self.keyword_of(p, &["catch", "finally", "throw", "try"]) {
            return self.token_rule(p, e, S::KeywordControlTrycatch);
        }
        if let Some(e) = self.keyword_of(p, &["break", "continue", "goto"]) {
            let j = self.ws(e);
            if j > e
                && let Some(n) = self.ident(j)
                && self.keyword_after(n)
            {
                self.token(p, e, &[S::KeywordControlLoop]);
                self.capture(Rule::Top, j, n, &[S::EntityNameLabel]);
                return Some(n);
            }
        }
        if let Some(e) = self.keyword_of(p, &["break", "continue", "do", "goto", "while"]) {
            return self.token_rule(p, e, S::KeywordControlLoop);
        }
        if let Some(e) = self.keyword(p, "return") {
            return self.open_token(Rule::Return, p, e, &[S::KeywordControlFlow]);
        }
        if let Some(e) = self.keyword_of(p, &["case", "default", "switch"]) {
            return self.token_rule(p, e, S::KeywordControlSwitch);
        }
        if let Some(r) = self.if_statement(p) {
            return Some(r);
        }
        if let Some(e) = self.keyword_of(p, &["else", "if"]) {
            return self.token_rule(p, e, S::KeywordControlConditional);
        }
        if let Some(e) = self.keyword(p, "with") {
            return self.token_rule(p, e, S::KeywordControlWith);
        }
        if let Some(e) = self.keyword(p, "package") {
            return self.token_rule(p, e, S::KeywordControl);
        }
        if let Some(e) = self.keyword(p, "debugger") {
            return self.token_rule(p, e, S::KeywordOtherDebugger);
        }
        None
    }

    fn switch_statement(&mut self, p: usize) -> Option<usize> {
        let e = self
            .keyword_before(p)
            .then(|| self.starts(p, "switch").then_some(p + "switch".len()))
            .flatten()?;
        if !self.boundary(p) || self.byte(self.ws(e)) != b'(' {
            return None;
        }
        self.open_empty(Rule::SwitchStatement, p)
    }

    /// The rules of a `switch` statement before its braces.
    pub(crate) fn switch_statement_rules(&mut self, p: usize) -> Option<usize> {
        if let Some(r) = self.comment(p) {
            return Some(r);
        }
        if self.keyword_before(p) && self.starts(p, "switch") {
            let e = p + "switch".len();
            let j = self.ws(e);
            if self.byte(j) == b'(' && !self.full() {
                self.flush(p);
                self.capture(Rule::SwitchExpression, p, e, &[S::KeywordControlSwitch]);
                self.capture(Rule::SwitchExpression, j, j + 1, &[S::MetaBraceRound]);
                self.push(Rule::SwitchExpression);
                return Some(j + 1);
            }
        }
        if self.byte(p) == b'{' {
            return self.open_token(
                Rule::SwitchBlock,
                p,
                p + 1,
                &[S::PunctuationDefinitionBlock],
            );
        }
        None
    }

    pub(crate) fn switch_block_rules(&mut self, p: usize) -> Option<usize> {
        if let Some(e) = self.keyword(p, "case") {
            return self.open_token(Rule::CaseClause, p, e, &[S::KeywordControlSwitch]);
        }
        if self.starts(p, "default")
            && self.keyword_before(p)
            && self.byte(p + "default".len()) == b':'
        {
            let e = p + "default".len();
            if self.keyword_after(e) {
                return self.open_token(Rule::CaseClause, p, e, &[S::KeywordControlSwitch]);
            }
        }
        if self.byte(p) == b':' {
            let j = self.ws(p + 1);
            if self.byte(j) == b'{' && !self.full() {
                self.flush(p);
                self.capture(
                    Rule::CaseBlock,
                    p,
                    p + 1,
                    &[
                        S::CaseClauseExpr,
                        S::PunctuationDefinitionSectionCaseStatement,
                    ],
                );
                self.capture(
                    Rule::CaseBlock,
                    j,
                    j + 1,
                    &[S::MetaBlock, S::PunctuationDefinitionBlock],
                );
                self.push(Rule::CaseBlock);
                return Some(j + 1);
            }
            self.token(
                p,
                p + 1,
                &[
                    S::CaseClauseExpr,
                    S::PunctuationDefinitionSectionCaseStatement,
                ],
            );
            return Some(p + 1);
        }
        self.statements(p)
    }

    /// A `for` loop, with an optional `await` after
    /// `(\s+|(\s*/\*…\*/\s*))`.
    fn for_loop(&mut self, p: usize) -> Option<usize> {
        let e = self.keyword(p, "for")?;
        let mut j = self.ws_comments(e);
        if j > e && self.starts(j, "await") {
            j = self.ws_comments(j + "await".len());
        }
        if self.byte(j) != b'(' || self.full() {
            return None;
        }
        self.open_token(Rule::ForLoop, p, e, &[S::KeywordControlLoop])
    }

    pub(crate) fn for_loop_rules(&mut self, p: usize) -> Option<usize> {
        if let Some(r) = self.comment(p) {
            return Some(r);
        }
        if let Some(e) = self.starts(p, "await").then_some(p + "await".len()) {
            return self.token_rule(p, e, S::KeywordControlLoop);
        }
        if self.byte(p) == b'(' {
            return self.open_token(Rule::ForParen, p, p + 1, &[S::MetaBraceRound]);
        }
        None
    }

    pub(crate) fn for_paren_rules(&mut self, p: usize) -> Option<usize> {
        self.variable_expression(p)
            .or_else(|| self.expression(p))
            .or_else(|| self.punctuation_semicolon(p))
    }

    /// `if (…)` not followed directly by a brace.
    ///
    /// `\s*(?!\{)` backs off a blank.
    /// Only `){` therefore fails.
    fn if_statement(&mut self, p: usize) -> Option<usize> {
        if !(self.starts(p, "if") && self.keyword_before(p) && self.boundary(p)) {
            return None;
        }
        let j = self.ws(p + 2);
        if self.byte(j) != b'(' {
            return None;
        }
        let close = self.balanced(j, b'(', b')')?;
        let ok = self.ws(close) > close || self.byte(close) != b'{';
        if !ok {
            return None;
        }
        self.open_empty(Rule::IfOuter, p)
    }

    pub(crate) fn if_outer_rules(&mut self, p: usize) -> Option<usize> {
        if let Some(r) = self.comment(p) {
            return Some(r);
        }
        if self.keyword_before(p) && self.starts(p, "if") {
            let j = self.ws(p + 2);
            if self.byte(j) == b'(' && !self.full() {
                self.flush(p);
                self.capture(Rule::IfParen, p, p + 2, &[S::KeywordControlConditional]);
                self.capture(Rule::IfParen, j, j + 1, &[S::MetaBraceRound]);
                self.push(Rule::IfParen);
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
        if self.byte(q) != b'/'
            || matches!(self.byte(q + 1), b'*' | b'/')
            || !self.regex_body(q, RegexForm::Loose)
        {
            return None;
        }
        self.flush(p);
        self.capture(
            Rule::Regex,
            q,
            q + 1,
            &[S::PunctuationDefinitionStringBegin],
        );
        self.push(Rule::Regex);
        Some(q + 1)
    }
}
