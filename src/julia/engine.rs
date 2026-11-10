//! Julia on the shared rule engine ([`crate::rules`]): the grammar's types.

use std::cell::Cell;
use std::convert::Infallible;

use super::grammar::{Pattern, Rule};
use super::scope::S;
use crate::rules::{self, Act, Captures, CarriedState, Grammar, ScopeName, table_scan};
use crate::scope::Scopes;

/// The Julia lexer for one line.
pub(crate) type Lexer<'a> = rules::Lexer<'a, Julia>;
/// What a highlighter keeps from line to line.
pub(crate) type Scratch = rules::Scratch<Julia>;

/// The `julia` grammar that Shiki bundles.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Julia;

/// What the Julia matchers remember about a line: the run of identifier
/// characters found last.
/// Many patterns start with an identifier, and the scan tries each at every
/// character of one.
/// A search from inside the run ends where the run does.
#[derive(Debug)]
pub(crate) struct Extra {
    /// The end of the text the run was found in, with its line feed.
    /// Lexing a capture group's text again moves it.
    pub(super) end: Cell<usize>,
    /// Where the run starts and ends.
    pub(super) run: Cell<(usize, usize)>,
}

impl Default for Extra {
    fn default() -> Self {
        Extra {
            end: Cell::new(usize::MAX),
            run: Cell::new((usize::MAX, 0)),
        }
    }
}

impl Grammar for Julia {
    type Scope = S;
    const NO_SCOPES: Scopes<S> = S::NO_SCOPES;
    type Rule = Rule;
    type Pattern = Pattern;
    type Nested = Infallible;
    type Extra = Extra;
    type Kept = ();

    const NAME: &'static str = "Julia";
    const ROOT: Rule = Rule::Root;
    const SOURCE: crate::scope::Scope = S::Source.scope();
    const GROUPS: usize = 5;

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

    const EMBEDS: bool = true;

    fn rule_carried_state(rule: Rule) -> CarriedState {
        rule.payload()
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

    const SKIPS_PLACES: bool = true;

    fn next_place(lexer: &Lexer<'_>, p: usize) -> usize {
        lexer.after_missed_word(p)
    }

    fn matcher(lexer: &mut Lexer<'_>, pattern: Pattern, p: usize) -> Option<usize> {
        lexer.matcher(pattern, p)
    }

    fn tokenize_nested(_: &mut Lexer<'_>, nested: Infallible, _: usize, _: usize, _: u32) {
        match nested {}
    }
}
