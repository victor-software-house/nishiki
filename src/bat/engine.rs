//! Batch on the shared rule engine ([`crate::rules`]): the grammar's types.

use std::convert::Infallible;

use super::grammar::{Pat, Rule};
use super::scope::S;
use crate::rules::{self, Act, Captures, Grammar, Payload, ScopeName, table_scan};
use crate::scope::Scopes;

/// The Batch lexer for one line.
pub(crate) type Lexer<'a> = rules::Lexer<'a, Bat>;
/// What a highlighter keeps from line to line.
pub(crate) type Scratch = rules::Scratch<Bat>;

/// What the Batch matchers remember about a line.
#[derive(Debug, Default)]
pub(crate) struct Extra {
    /// The line's last `%` and `!`, once a matcher has looked for them.
    pub(super) marks: Option<Marks>,
    /// Whether a Markdown fence embeds the line, where the grammar's
    /// injection does not apply.
    pub(super) embedded: bool,
}

/// Where the line's last `%` and `!` stand.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Marks {
    /// The last `%`, when the line has one.
    pub(super) percent: Option<usize>,
    /// The last `!`, when the line has one.
    pub(super) bang: Option<usize>,
}

/// The `bat` grammar that Shiki bundles.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Bat;

impl Grammar for Bat {
    type Scope = S;
    const NO_SCOPES: Scopes<S> = S::NO_SCOPES;
    type Rule = Rule;
    type Pat = Pat;
    type Nested = Infallible;
    type Extra = Extra;
    /// Whether a Markdown fence embeds the line, which `lex` sets.
    type Kept = bool;

    const NAME: &'static str = "Batch";
    const ROOT: Rule = Rule::Root;
    const SOURCE: crate::scope::Scope = S::Source.scope();
    const GROUPS: usize = 4;

    fn extra(kept: bool) -> Extra {
        Extra {
            marks: None,
            embedded: kept,
        }
    }

    fn keep(extra: Extra) -> bool {
        extra.embedded
    }

    fn scope(scope: S) -> ScopeName {
        ScopeName::Fixed(scope.scope())
    }

    fn rule_code(rule: Rule) -> u16 {
        rule.code()
    }

    fn rule_from_code(code: u16) -> Rule {
        Rule::from_code(code)
    }

    fn rule_name(rule: Rule) -> Scopes<S> {
        rule.data().name
    }

    fn rule_content(rule: Rule) -> Scopes<S> {
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

    fn tokenize_nested(_: &mut Lexer<'_>, nested: Infallible, _: usize, _: usize, _: u32) {
        match nested {}
    }
}
