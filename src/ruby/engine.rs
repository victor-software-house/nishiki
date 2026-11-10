//! Ruby on the shared rule engine ([`crate::rules`]): the grammar's types.

use std::cell::Cell;

use super::grammar::{Pat, Rule};
use super::scope::S;
pub(crate) use crate::rules::embedded;
use crate::rules::{self, Act, Captures, Grammar, HasRuns, Payload, Runs, ScopeName, table_scan};
use crate::scope::Scopes;

/// The Ruby lexer for one line.
pub(crate) type Lexer<'a> = rules::Lexer<'a, Ruby>;
/// What a highlighter keeps from line to line.
pub(crate) type Scratch = rules::Scratch<Ruby>;

/// The `ruby` grammar that Shiki bundles.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Ruby;

/// What the Ruby matchers remember about a line.
#[derive(Debug, Default)]
pub(crate) struct Extra {
    /// The run of `\w` characters found last.
    runs: Runs,
    /// The last scan for a regular expression's closing `/`.
    /// It holds the end of the text it ran in, with its line feed, where it
    /// started, and the `/` it stopped at, if any.
    pub(crate) regexp_close: Cell<(usize, usize, Option<usize>)>,
}

impl HasRuns for Extra {
    fn runs(&self) -> &Runs {
        &self.runs
    }
}

impl Grammar for Ruby {
    type Scope = S;
    const NO_SCOPES: Scopes<S> = S::NO_SCOPES;
    type Rule = Rule;
    type Pat = Pat;
    type Nested = Rule;
    type Extra = Extra;
    type Kept = ();

    const NAME: &'static str = "Ruby";
    const ROOT: Rule = Rule::Root;
    const SOURCE: crate::scope::Scope = S::Source.scope();
    const GROUPS: usize = 16;
    const CAPTURED_EOL: bool = true;

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

    const WHILES: bool = Rule::HAS_WHILE;

    fn rule_while(rule: Rule) -> Option<Pat> {
        rule.while_pat()
    }

    const EMBEDS: bool = true;

    #[expect(
        clippy::inline_always,
        reason = "measured at one codegen unit: a hint left it out of line, at a call on every rule opened and closed"
    )]
    #[inline(always)]
    fn rule_payload(rule: Rule) -> Payload {
        rule.payload()
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

    fn tokenize_nested(lexer: &mut Lexer<'_>, nested: Rule, from: usize, to: usize, base: u32) {
        lexer.tokenize_capture(nested, from, to, base);
    }
}
