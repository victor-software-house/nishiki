//! CoffeeScript on the shared rule engine ([`crate::rules`]): the grammar's
//! types.

use std::cell::Cell;

use super::grammar::{Nested, Pat, Rule};
use super::scope::S;
use crate::rules::{self, Act, BlankRuns, Captures, Grammar, ScopeName, table_scan};
use crate::scope::Scopes;

/// The CoffeeScript lexer for one line, of the dialect `D` numbers.
pub(crate) type Lexer<'a, const D: u8 = PLAIN> = rules::Lexer<'a, Coffee<D>>;
/// What a highlighter keeps from line to line.
pub(crate) type Scratch<const D: u8 = PLAIN> = rules::Scratch<Coffee<D>>;

/// The lexer's const parameter for CoffeeScript on its own.
pub(crate) const PLAIN: u8 = 0;
/// The lexer's const parameter for the CoffeeScript that Svelte embeds.
/// Svelte's grammar injects its `$` prefixes there
/// (`L:(source.ts, source.js, source.coffee)`).
/// Each rule then tries the two patterns of [`Pat::SVELTE`] first, before
/// its end pattern.
#[cfg(feature = "lang-svelte")]
pub(crate) const SVELTE: u8 = 1;

/// The `coffee` grammar that Shiki bundles, as the dialect `D` numbers.
/// Each dialect is its own instance of the rule engine, so CoffeeScript on its
/// own runs no code for Svelte's injection.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Coffee<const D: u8 = PLAIN>;

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

/// A run found in a line's text.
/// It holds the text's end, and where the run starts and ends.
/// Lexing a capture group's text again moves the text's end.
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

/// A dialect's grammar: [`Grammar`] with CoffeeScript's types.
/// The matchers are generic over the dialect and bound on this trait, so the
/// types they read stay CoffeeScript's.
///
/// rustc cannot prove that a `Coffee<D>` falls outside Objective-C's generic
/// matchers, `impl<G: Grammar> Lexer<'_, G> where G::Extra: ObjcMemory`.
/// So no CoffeeScript matcher shares a name with one of theirs.
pub(crate) trait Dialect:
    Grammar<Scope = S, Rule = Rule, Pat = Pat, Nested = Nested, Extra = Extra>
{
}

impl<G: Grammar<Scope = S, Rule = Rule, Pat = Pat, Nested = Nested, Extra = Extra>> Dialect for G {}

/// The grammar's impl for one dialect.
/// Each dialect has an impl of its own, and the dialects share the grammar's
/// definitions: an act converts to the dialect's type unchanged.
///
/// A generic impl would let rustc export every function its body reaches.
/// Every caller of `Arena::child` and `Language::lex_embedded` would then
/// call them through the GOT.
/// That costs JSON 0.4% at one codegen unit.
macro_rules! dialect {
    ($dialect:ident) => {
        impl Grammar for Coffee<$dialect> {
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
                rule.definition().name
            }

            fn rule_content(rule: Rule) -> Scopes<S> {
                rule.definition().content
            }

            fn pat_act(pat: Pat) -> Act<Self> {
                pat.definition().act.dialect()
            }

            fn pat_captures(pat: Pat) -> Captures<Self> {
                let definition = pat.definition();
                (definition.captures, definition.nested)
            }

            fn pat_definition(pat: Pat) -> (Act<Self>, Captures<Self>) {
                let definition = pat.definition();
                (
                    definition.act.dialect(),
                    (definition.captures, definition.nested),
                )
            }

            fn scan(lexer: &mut Lexer<'_, $dialect>) -> Option<(usize, Pat, usize)> {
                table_scan(lexer)
            }

            /// The script string's middle group, which JavaScript lexes
            /// ([`rules::Embeds::lex_nested`]).
            const DEFERS_NESTED: bool = true;

            fn defers_nested(nested: Nested) -> Option<crate::Language> {
                (nested == Rule::Javascript).then_some(crate::Language::JavaScript)
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

            fn next_place(lexer: &Lexer<'_, $dialect>, p: usize) -> usize {
                lexer.after_missed_word(p)
            }

            fn matcher(lexer: &mut Lexer<'_, $dialect>, pat: Pat, p: usize) -> Option<usize> {
                lexer.pat_matches(pat, p)
            }

            fn tokenize_nested(
                lexer: &mut Lexer<'_, $dialect>,
                nested: Nested,
                from: usize,
                to: usize,
                base: u32,
            ) {
                lexer.retokenise(nested, from, to, base);
                lexer.emit(to, base);
            }
        }
    };
}

dialect!(PLAIN);
#[cfg(feature = "lang-svelte")]
dialect!(SVELTE);
