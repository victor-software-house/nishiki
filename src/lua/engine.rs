//! Lua on the shared rule engine ([`crate::rules`]): the grammar's types.

use std::convert::Infallible;

use super::grammar::{Pat, Rule};
use super::scope::S;
use crate::rules::{self, Act, Captures, Grammar, HasRuns, Payload, Runs, ScopeName, table_scan};

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
    type Rule = Rule;
    type Pat = Pat;
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

    fn rule_name(rule: Rule) -> &'static [S] {
        rule.data().name
    }

    fn rule_content(rule: Rule) -> &'static [S] {
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

    fn tokenize_nested(_: &mut Lexer<'_>, kind: Infallible, _: usize, _: usize, _: u32) {
        match kind {}
    }
}
