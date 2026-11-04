//! Swift on the shared rule engine ([`crate::rules`]): the grammar's types and
//! what the matchers remember about a line.

use super::grammar::{Nested, Pat, Rule};
use super::matchers;
use super::scope::S;
use crate::rules::{self, Act, Captures, Grammar, Payload, ScopeName, table_scan};

/// The Swift lexer for one line.
pub(crate) type Lexer<'a> = rules::Lexer<'a, Swift>;
/// What a highlighter keeps from line to line.
pub(crate) type Scratch = rules::Scratch<Swift>;

/// The `swift` grammar that Shiki bundles.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Swift;

/// What the Swift matchers remember about a line: one memo for each file of
/// matchers.
/// A lookahead that scans the rest of the line is then asked once.
#[derive(Debug, Default)]
pub(crate) struct Extra {
    pub(super) declarations: matchers::declarations::Memo,
    pub(super) expressions: matchers::expressions::Memo,
    pub(super) literals: matchers::literals::Memo,
    pub(super) names: matchers::names::Memo,
    pub(super) regex_literals: matchers::regex_literals::Memo,
    pub(super) types: matchers::types::Memo,
}

impl Grammar for Swift {
    type Scope = S;
    type Rule = Rule;
    type Pat = Pat;
    type Nested = Nested;
    type Extra = Extra;
    type Kept = Vec<(usize, usize)>;

    const NAME: &'static str = "Swift";
    const ROOT: Rule = Rule::Root;
    const MARKS_FIRST_LINE: bool = true;
    const SOURCE: crate::scope::Scope = S::Source.scope();
    const GROUPS: usize = 32;
    const CAPTURED_EOL: bool = true;

    fn scope(scope: S) -> ScopeName {
        ScopeName::Fixed(scope.scope())
    }

    fn extra(kept: Vec<(usize, usize)>) -> Extra {
        Extra {
            regex_literals: matchers::regex_literals::Memo::with_path(kept),
            ..Extra::default()
        }
    }

    fn keep(extra: Extra) -> Vec<(usize, usize)> {
        extra.regex_literals.into_path()
    }

    fn rule_code(rule: Rule) -> u16 {
        rule.code()
    }

    fn rule_from_code(code: u16) -> Rule {
        Rule::from_code(code)
    }

    fn rule_name(rule: Rule) -> &'static [S] {
        rule.data().name
    }

    fn rule_content(rule: Rule) -> &'static [S] {
        rule.data().content
    }

    fn rule_payload(rule: Rule) -> Payload {
        rule.data().backref.map_or(Payload::None, Payload::Backref)
    }

    fn pat_act(pat: Pat) -> Act<Self> {
        pat.data().act
    }

    fn pat_captures(pat: Pat) -> Captures<Self> {
        let data = pat.data();
        (data.captures, data.nested)
    }

    fn pat_data(pat: Pat) -> (Act<Self>, Captures<Self>) {
        let data = pat.data();
        (data.act, (data.captures, data.nested))
    }

    fn scan(lexer: &mut Lexer<'_>) -> Option<(usize, Pat, usize)> {
        table_scan(lexer)
    }

    fn matcher(lexer: &mut Lexer<'_>, pat: Pat, p: usize) -> Option<usize> {
        lexer.matcher(pat, p)
    }

    fn tokenize_nested(lexer: &mut Lexer<'_>, kind: Nested, from: usize, to: usize, base: u32) {
        lexer.retokenise(kind, from, to, base);
    }
}

#[cfg(test)]
impl Lexer<'_> {
    /// Tries `pat` at `p`, for the tool that compares the matchers with
    /// Oniguruma.
    pub(super) fn probe(&mut self, pat: Pat, p: usize) -> Option<usize> {
        self.matcher(pat, p)
    }
}
