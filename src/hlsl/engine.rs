//! HLSL on the shared rule engine ([`crate::rules`]): the grammar's types.

use std::cell::Cell;

use super::grammar::{Nested, Pattern, Rule};
use super::scope::S;
use crate::rules::{self, Act, Captures, CarriedState, Grammar, ScopeName, table_scan};
use crate::scope::Scopes;

/// The HLSL lexer for one line.
pub(crate) type Lexer<'a> = rules::Lexer<'a, Hlsl>;
/// What a highlighter keeps from line to line.
pub(crate) type Scratch = rules::Scratch<Hlsl>;

/// The word at a place, as the word patterns look it up.
#[derive(Clone, Copy, Debug)]
pub(super) struct Word {
    /// Where the word starts.
    pub(super) start: usize,
    /// Where it ends, or `None` when `\b` does not hold before it.
    pub(super) end: Option<usize>,
    /// The [`super::words::bit`]s of the word patterns that list it.
    pub(super) bits: u64,
}

impl Default for Word {
    fn default() -> Self {
        Word {
            start: usize::MAX,
            end: None,
            bits: 0,
        }
    }
}

/// What the HLSL matchers remember about a line.
/// Up to 36 word patterns test the same word, so the last word looked up is
/// kept.
#[derive(Debug, Default)]
pub(crate) struct Extra {
    /// The last word looked up.
    pub(super) word: Cell<Word>,
}

/// The `hlsl` grammar that Shiki bundles.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Hlsl;

impl Grammar for Hlsl {
    type Scope = S;
    const NO_SCOPES: Scopes<S> = S::NO_SCOPES;
    type Rule = Rule;
    type Pattern = Pattern;
    type Nested = Nested;
    type Extra = Extra;
    type Kept = ();

    const NAME: &'static str = "HLSL";
    const ROOT: Rule = Rule::Root;
    const SOURCE: crate::scope::Scope = S::Source.scope();
    const GROUPS: usize = 1;
    const SKIPS_PLACES: bool = true;

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

    fn rule_carried_state(_: Rule) -> CarriedState {
        CarriedState::None
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

    fn next_place(lexer: &Lexer<'_>, p: usize) -> usize {
        lexer.after_missed_word(p)
    }

    fn matcher(lexer: &mut Lexer<'_>, pattern: Pattern, p: usize) -> Option<usize> {
        lexer.matcher(pattern, p)
    }

    fn tokenize_nested(_: &mut Lexer<'_>, nested: Nested, _: usize, _: usize, _: u32) {
        match nested {}
    }
}
