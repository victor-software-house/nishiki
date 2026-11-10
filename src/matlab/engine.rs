//! MATLAB on the shared rule engine ([`crate::rules`]): the grammar's types
//! and what its matchers remember about a line.

use std::cell::Cell;

use super::grammar::{Nested, Pat, Rule, Sc};
use super::scope::S;
use crate::rules::{self, Act, BlankRuns, Captures, Grammar, ScopeName, table_scan};
use crate::scope::Scopes;

/// The MATLAB lexer for one line.
pub(crate) type Lexer<'a> = rules::Lexer<'a, Matlab>;
/// What a highlighter keeps from line to line.
pub(crate) type Scratch = rules::Scratch<Matlab>;

/// How many capture groups can be lexed again inside one another.
/// A `classdef` line lexes its attributes again, and their strings lex their
/// own text, so the depth stays small; the bound only stops a runaway.
/// Past it, a group keeps its own scopes, and the line reports the depth
/// limit.
pub(crate) const MAX_NESTED: u32 = 64;

/// The `matlab` grammar that Shiki bundles.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Matlab;

/// What a line of MATLAB keeps for its matchers.
#[derive(Debug)]
pub(crate) struct Extra {
    /// How many capture groups are being lexed again around this point.
    nested: u32,
    /// The last run of name characters measured: where it starts and ends,
    /// the length of the text it was measured in, and its class.
    /// Indexing braces and validators have no `\b` before their name, so they
    /// measure a name from each of its bytes.
    /// A walk entered at a later byte of this run ends where it does.
    pub(super) run: Cell<(usize, usize, usize, u8)>,
}

impl Default for Extra {
    fn default() -> Self {
        Extra {
            nested: 0,
            run: Cell::new((usize::MAX, 0, 0, 0)),
        }
    }
}

impl Grammar for Matlab {
    type Scope = Sc;
    const NO_SCOPES: Scopes<Sc> = Sc::NO_SCOPES;
    type Rule = Rule;
    type Pat = Pat;
    type Nested = Nested;
    type Extra = Extra;
    type Kept = ();

    const NAME: &'static str = "MATLAB";
    const ROOT: Rule = Rule::Root;
    const SOURCE: crate::scope::Scope = S::Source.scope();
    const GROUPS: usize = 29;
    const CAPTURED_EOL: bool = true;
    /// The parens and indexing braces end at `(?<!\.\.\.).\n`, which matches
    /// only at the last blank of a run that ends the line
    /// ([`Pat::takes_one_blank`]).
    const BLANK_RUNS: BlankRuns = BlankRuns::ToLast;

    fn takes_one_blank(pat: Pat) -> bool {
        pat.takes_one_blank()
    }

    fn scope(scope: Sc) -> ScopeName {
        match scope {
            Sc::S(scope) => ScopeName::Fixed(scope.scope()),
        }
    }

    fn rule_code(rule: Rule) -> u16 {
        rule.code()
    }

    fn rule_from_code(code: u16) -> Rule {
        Rule::from_code(code)
    }

    fn rule_name(rule: Rule) -> Scopes<Sc> {
        rule.definition().name
    }

    fn rule_content(rule: Rule) -> Scopes<Sc> {
        rule.definition().content
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
        lexer.matcher(pat.rx(), p)
    }

    /// Lexes the group again with its rule, under the stack the engine
    /// gives and the group's own scopes.
    /// Past [`MAX_NESTED`] groups lexed again around it, the text keeps the
    /// group's scopes alone.
    /// The line then reports the depth limit.
    fn tokenize_nested(lexer: &mut Lexer<'_>, nested: Nested, from: usize, to: usize, base: u32) {
        let inner = lexer.path(base, nested.scopes);
        if lexer.x.nested >= MAX_NESTED {
            lexer.record_depth();
            lexer.emit(to, inner);
            return;
        }
        lexer.x.nested += 1;
        lexer.retokenise(nested.rule, from, to, inner);
        lexer.x.nested -= 1;
    }
}
