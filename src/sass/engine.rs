//! Sass on the shared rule engine ([`crate::rules`]): the grammar's types.

use std::convert::Infallible;

use super::grammar::{Pattern, Rule};
use super::scope::S;
use crate::rules::{self, Act, BlankRuns, Captures, CarriedState, Grammar, ScopeName, table_scan};
use crate::scope::Scopes;

/// The Sass lexer for one line.
pub(crate) type Lexer<'a> = rules::Lexer<'a, Sass>;
/// What a highlighter keeps from line to line.
pub(crate) type Scratch = rules::Scratch<Sass>;

/// The `sass` grammar that Shiki bundles.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Sass;

impl Grammar for Sass {
    type Scope = S;
    const NO_SCOPES: Scopes<S> = S::NO_SCOPES;
    type Rule = Rule;
    type Pattern = Pattern;
    type Nested = Infallible;
    type Extra = ();
    type Kept = ();

    const NAME: &'static str = "Sass";
    const ROOT: Rule = Rule::Root;
    const SOURCE: crate::scope::Scope = S::Source.scope();
    const GROUPS: usize = 4;
    /// The operators `\s-\s`, `\s-(?=\$)` and `\s-(?=\()` and the end
    /// `(?=\s\(|and\(|\),)` take one blank and then require a character that
    /// is not a blank ([`Pattern::takes_one_blank`]).
    /// They match only at the last blank of a run.
    const BLANK_RUNS: BlankRuns = BlankRuns::ToLast;

    fn takes_one_blank(pattern: Pattern) -> bool {
        pattern.takes_one_blank()
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

    fn rule_carried_state(rule: Rule) -> CarriedState {
        rule.carried_state()
    }

    fn pattern_act(pattern: Pattern) -> Act<Self> {
        pattern.definition().act
    }

    fn pattern_captures(pattern: Pattern) -> Captures<Self> {
        let definition = pattern.definition();
        (definition.captures, definition.nested)
    }

    fn pattern_definition(pattern: Pattern) -> (Act<Self>, Captures<Self>) {
        let definition = pattern.definition();
        (definition.act, (definition.captures, definition.nested))
    }

    fn scan(lexer: &mut Lexer<'_>) -> Option<(usize, Pattern, usize)> {
        table_scan(lexer)
    }

    fn matcher(lexer: &mut Lexer<'_>, pattern: Pattern, p: usize) -> Option<usize> {
        lexer.matcher(pattern, p)
    }

    fn tokenize_nested(_: &mut Lexer<'_>, nested: Infallible, _: usize, _: usize, _: u32) {
        match nested {}
    }
}
