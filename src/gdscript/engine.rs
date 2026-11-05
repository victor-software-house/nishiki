//! GDScript on the shared rule engine ([`crate::rules`]): the grammar's types.

use std::convert::Infallible;

use super::grammar::{Pat, Rule};
use super::scope::S;
use crate::rules::{self, Act, Captures, Grammar, Payload, ScopeName, table_scan};
use crate::scope::Scopes;

/// The GDScript lexer for one line.
pub(crate) type Lexer<'a> = rules::Lexer<'a, Gdscript>;
/// What a highlighter keeps from line to line.
pub(crate) type Scratch = rules::Scratch<Gdscript>;

/// The `gdscript` grammar that Shiki bundles.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Gdscript;

impl Grammar for Gdscript {
    type Scope = S;
    const NO_SCOPES: Scopes<S> = S::NO_SCOPES;
    type Rule = Rule;
    type Pat = Pat;
    type Nested = Infallible;
    type Extra = ();
    type Kept = ();

    const NAME: &'static str = "GDScript";
    const ROOT: Rule = Rule::Root;
    const SOURCE: crate::scope::Scope = S::Source.scope();
    const GROUPS: usize = 6;
    /// A comment's `\n?`, a line continuation's `\s*$\n?` and a node path's
    /// trailing `\s*` take the line feed.
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
        rule.data().name
    }

    fn rule_content(rule: Rule) -> Scopes<S> {
        rule.data().content
    }

    fn rule_payload(rule: Rule) -> Payload {
        rule.payload()
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

    fn tokenize_nested(_: &mut Lexer<'_>, kind: Infallible, _: usize, _: usize, _: u32) {
        match kind {}
    }
}
