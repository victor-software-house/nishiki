//! CoffeeScript on the shared rule engine ([`crate::rules`]): the grammar's
//! types.

use std::cell::Cell;

use super::grammar::{Nested, Pat, Rule};
use super::scope::S;
use crate::rules::{self, Act, BlankRuns, Captures, Grammar, ScopeName, table_scan};
use crate::scope::Scopes;

/// The CoffeeScript lexer for one line.
pub(crate) type Lexer<'a> = rules::Lexer<'a, Coffee>;
/// What a highlighter keeps from line to line.
pub(crate) type Scratch = rules::Scratch<Coffee>;

/// The `coffee` grammar that Shiki bundles.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Coffee;

/// What the CoffeeScript matchers remember about a line: the runs of name
/// characters found last.
/// Several patterns start with a name at the same place, and each reads the
/// same run.
#[derive(Debug, Default)]
pub(crate) struct Extra {
    /// The run of `[$\w]` found last.
    pub(super) names: Run,
    /// The run of `\w` found last.
    pub(super) words: Run,
    /// The script strings' JavaScript groups the line defers
    /// ([`Grammar::deferred`]).
    deferrals: Option<Box<rules::Deferrals>>,
}

/// A run found in a line's text: the text's end, which lexing a capture
/// group's text again moves, and where the run starts and ends.
#[derive(Debug, Default)]
pub(crate) struct Run(Cell<(usize, usize, usize)>);

impl Run {
    /// The end of the run that holds `p`, when it was found in a text that
    /// ends at `end`.
    pub(super) fn get(&self, end: usize, p: usize) -> Option<usize> {
        let (key, from, to) = self.0.get();
        (key == end && from <= p && p < to).then_some(to)
    }

    /// Remembers the run `from..to` of a text that ends at `end`.
    pub(super) fn set(&self, end: usize, from: usize, to: usize) {
        self.0.set((end, from, to));
    }
}

impl Grammar for Coffee {
    type Scope = S;
    const NO_SCOPES: Scopes<S> = S::NO_SCOPES;
    type Rule = Rule;
    type Pat = Pat;
    type Nested = Nested;
    type Extra = Extra;
    type Kept = Option<Box<rules::Deferrals>>;

    const NAME: &'static str = "CoffeeScript";
    const ROOT: Rule = Rule::Root;
    const SOURCE: crate::scope::Scope = S::Source.scope();
    const GROUPS: usize = 10;
    /// A heregex's character range, `(?:.|\\.)-…`, takes one blank with its
    /// `.` and then requires `-` ([`Pat::takes_one_blank`]).
    /// It matches only at the last blank of a run.
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
        rule.data().name
    }

    fn rule_content(rule: Rule) -> Scopes<S> {
        rule.data().content
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

    /// The script string's middle group, which JavaScript lexes
    /// ([`rules::Embeds::lex_nested`]).
    const DEFERS_NESTED: bool = true;

    fn defers_nested(kind: Nested) -> Option<crate::Language> {
        (kind == Rule::Javascript).then_some(crate::Language::JavaScript)
    }

    /// The deferrals' buffers go from line to line; the runs start empty.
    fn extra(kept: Self::Kept) -> Self::Extra {
        Extra {
            deferrals: kept,
            ..Extra::default()
        }
    }

    fn keep(extra: Self::Extra) -> Self::Kept {
        extra.deferrals
    }

    fn deferred(extra: &mut Self::Extra) -> Option<&mut Option<Box<rules::Deferrals>>> {
        Some(&mut extra.deferrals)
    }

    fn deferred_kept(kept: &mut Self::Kept) -> Option<&mut Option<Box<rules::Deferrals>>> {
        Some(kept)
    }

    const SKIPS_PLACES: bool = true;

    fn next_place(lexer: &Lexer<'_>, p: usize) -> usize {
        lexer.after_missed_word(p)
    }

    fn matcher(lexer: &mut Lexer<'_>, pat: Pat, p: usize) -> Option<usize> {
        lexer.matcher(pat, p)
    }

    fn tokenize_nested(lexer: &mut Lexer<'_>, kind: Nested, from: usize, to: usize, base: u32) {
        lexer.retokenise(kind, from, to, base);
        lexer.emit(to, base);
    }
}
