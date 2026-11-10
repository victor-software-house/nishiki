//! Pascal on the shared rule engine ([`crate::rules`]): the grammar's types.

use std::cell::Cell;

use super::grammar::{Nested, Pattern, Rule};
use super::scope::S;
use crate::rules::{self, Act, Captures, Grammar, Payload, ScopeName, table_scan};
use crate::scope::Scopes;

/// The Pascal lexer for one line.
pub(crate) type Lexer<'a> = rules::Lexer<'a, Pascal>;
/// What a highlighter keeps from line to line.
pub(crate) type Scratch = rules::Scratch<Pascal>;

/// The longest word whose letters a [`Lowered`] holds.
/// The longest listed word, `lifetimestrategy`, has 16 letters.
pub(super) const LONGEST_WORD: usize = 32;

/// The word at a place, as the word lists compare it: where it ends, and its
/// letters as `(?i)` reads them.
/// The letters are `None` when the word holds a character above ASCII that
/// folds to no ASCII letters, or is longer than [`LONGEST_WORD`].
#[derive(Clone, Copy, Debug)]
pub(super) struct Lowered {
    /// Where the word starts.
    pub(super) start: usize,
    /// Where it ends.
    pub(super) end: usize,
    /// How many of `letters` it fills.
    pub(super) len: Option<usize>,
    /// Its letters, lowercase.
    pub(super) letters: [u8; LONGEST_WORD],
}

impl Default for Lowered {
    fn default() -> Self {
        Lowered {
            start: usize::MAX,
            end: 0,
            len: None,
            letters: [0; LONGEST_WORD],
        }
    }
}

/// What the Pascal matchers remember about a line.
/// Nine of the word patterns test the same word, so the last word lowered is
/// kept.
#[derive(Debug, Default)]
pub(crate) struct Extra {
    /// The last word lowered.
    pub(super) lowered: Cell<Lowered>,
}

/// The `pascal` grammar that Shiki bundles.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Pascal;

impl Grammar for Pascal {
    type Scope = S;
    const NO_SCOPES: Scopes<S> = S::NO_SCOPES;
    type Rule = Rule;
    type Pattern = Pattern;
    type Nested = Nested;
    type Extra = Extra;
    type Kept = ();

    const NAME: &'static str = "Pascal";
    const ROOT: Rule = Rule::Root;
    const SOURCE: crate::scope::Scope = S::Source.scope();
    const GROUPS: usize = 3;
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

    fn rule_payload(_: Rule) -> Payload {
        Payload::None
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
