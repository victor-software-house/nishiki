//! Lua on the shared rule engine ([`crate::rules`]): the grammar's types.

use std::convert::Infallible;

use super::grammar::{Pattern, Rule};
use super::scope::S;
pub(crate) use crate::rules::embedded;
use crate::rules::{
    self, Act, Captures, CarriedState, Grammar, HasRuns, Runs, ScopeName, table_scan,
};
use crate::scope::Scopes;

/// The Lua lexer for one line.
pub(crate) type Lexer<'a> = rules::Lexer<'a, Lua>;
/// What a highlighter keeps from line to line.
pub(crate) type Scratch = rules::Scratch<Lua>;

/// The `lua` grammar that Shiki bundles.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Lua;

/// What the Lua matchers remember about a line.
#[derive(Debug, Default)]
pub(crate) struct Extra {
    /// The run of `\w` characters found last.
    runs: Runs,
}

impl HasRuns for Extra {
    fn runs(&self) -> &Runs {
        &self.runs
    }
}

impl Grammar for Lua {
    type Scope = S;
    const NO_SCOPES: Scopes<S> = S::NO_SCOPES;
    type Rule = Rule;
    type Pattern = Pattern;
    type Nested = Infallible;
    type Extra = Extra;
    type Kept = ();

    const NAME: &'static str = "Lua";
    const ROOT: Rule = Rule::Root;
    const SOURCE: crate::scope::Scope = S::Source.scope();
    const GROUPS: usize = 4;
    const CAPTURED_EOL: bool = true;
    const MARKS_FIRST_LINE: bool = true;

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

    const EMBEDS_ON_DEMAND: bool = true;

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
