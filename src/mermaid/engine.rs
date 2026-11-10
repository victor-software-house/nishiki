//! Mermaid on the shared rule engine ([`crate::rules`]): the grammar's types.

use std::convert::Infallible;

use super::grammar::{Pat, Rule};
use super::scope::S;
use crate::rules::{self, Act, BlankRuns, Captures, Grammar, Payload, ScopeName, table_scan};
use crate::scope::Scopes;

/// The Mermaid lexer for one line.
pub(crate) type Lexer<'a> = rules::Lexer<'a, Mermaid>;
/// What a highlighter keeps from line to line.
pub(crate) type Scratch = rules::Scratch<Mermaid>;

/// The `mermaid` grammar that Shiki bundles.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Mermaid;

impl Grammar for Mermaid {
    type Scope = S;
    const NO_SCOPES: Scopes<S> = S::NO_SCOPES;
    type Rule = Rule;
    type Pat = Pat;
    type Nested = Infallible;
    type Extra = ();
    type Kept = ();

    const NAME: &'static str = "Mermaid";
    const ROOT: Rule = Rule::Root;
    const SOURCE: crate::scope::Scope = S::Source.scope();
    const GROUPS: usize = 20;
    /// A class member `\s([-#+~])?([-.\w]+)…` and a method head take one blank
    /// and then require a character that is not a blank
    /// ([`Pat::takes_one_blank`]).
    /// They match only at the last blank of a run.
    /// The pie slice `\s(.*)(:)` and the state's `\s+(})\s+` match at the
    /// first blank of a run too.
    const BLANK_RUNS: BlankRuns = BlankRuns::ToLast;

    fn takes_one_blank(pat: Pat) -> bool {
        pat.takes_one_blank()
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
        rule.definition().name
    }

    fn rule_content(rule: Rule) -> Scopes<S> {
        rule.definition().content
    }

    fn rule_payload(_: Rule) -> Payload {
        Payload::None
    }

    fn pat_act(pat: Pat) -> Act<Self> {
        pat.definition().act
    }

    fn pat_captures(pat: Pat) -> Captures<Self> {
        let definition = pat.definition();
        (definition.captures, definition.nested)
    }

    fn pat_definition(pat: Pat) -> (Act<Self>, Captures<Self>) {
        let definition = pat.definition();
        (definition.act, (definition.captures, definition.nested))
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
