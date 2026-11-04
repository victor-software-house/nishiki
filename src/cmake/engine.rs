//! CMake on the shared rule engine ([`crate::rules`]): the grammar's
//! types.

use std::cell::Cell;

use super::grammar::{Nested, Pat, Rule};
use super::scope::S;
use crate::rules::{self, Act, Captures, Grammar, Payload, ScopeName, table_scan};
use crate::scope::Scopes;

/// The CMake lexer for one line.
pub(crate) type Lexer<'a> = rules::Lexer<'a, CMake>;
/// What a highlighter keeps from line to line.
pub(crate) type Scratch = rules::Scratch<CMake>;

/// The longest word whose letters a [`Lowered`] holds.
pub(super) const LONGEST_WORD: usize = 64;

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

/// What the CMake matchers remember about a line.
/// Each of the word patterns asks about the same word, so the last word
/// lowered is kept.
#[derive(Debug, Default)]
pub(crate) struct Extra {
    /// The last word lowered.
    pub(super) lowered: Cell<Lowered>,
}

/// The `cmake` grammar that Shiki bundles.
#[derive(Clone, Copy, Debug)]
pub(crate) struct CMake;

impl Grammar for CMake {
    type Scope = S;
    const NO_SCOPES: Scopes<S> = S::NO_SCOPES;
    type Rule = Rule;
    type Pat = Pat;
    type Nested = Nested;
    type Extra = Extra;
    type Kept = ();

    const NAME: &'static str = "CMake";
    const ROOT: Rule = Rule::Root;
    const SOURCE: crate::scope::Scope = S::Source.scope();
    const GROUPS: usize = 2;

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

    fn tokenize_nested(_: &mut Lexer<'_>, kind: Nested, _: usize, _: usize, _: u32) {
        match kind {}
    }
}
