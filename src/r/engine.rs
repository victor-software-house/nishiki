//! R on the shared rule engine ([`crate::rules`]): the grammar's types.

use super::grammar::{Pat, Rule};
use super::scope::S;
use crate::rules::{self, Act, Captures, Grammar, Payload, ScopeName, table_scan};
use crate::scope::Scopes;

/// The R lexer for one line.
pub(crate) type Lexer<'a> = rules::Lexer<'a, R>;
/// What a highlighter keeps from line to line.
pub(crate) type Scratch = rules::Scratch<R>;

/// The `r` grammar that Shiki bundles.
#[derive(Clone, Copy, Debug)]
pub(crate) struct R;

impl Grammar for R {
    type Scope = S;
    const NO_SCOPES: Scopes<S> = S::NO_SCOPES;
    type Rule = Rule;
    type Pat = Pat;
    type Nested = Rule;
    type Extra = ();
    type Kept = ();

    const NAME: &'static str = "R";
    const ROOT: Rule = Rule::Root;
    const SOURCE: crate::scope::Scope = S::Source.scope();
    /// A markdown link, `(\[)(?:(\w+)(:{2,3}))?(\w+)(\(\))?(])`, records six.
    const GROUPS: usize = 7;
    /// A roxygen fence's begin, `` (`{3,})\s*(.*) ``, and `@examples`, with
    /// `\s*` before the line's end, take the line feed.
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

    /// The condition of `@examplesIf` is the one group lexed again.
    /// Its patterns record no group of their own, so it nests one level.
    fn tokenize_nested(lexer: &mut Lexer<'_>, nested: Rule, from: usize, to: usize, base: u32) {
        lexer.tokenize_capture(nested, from, to, base);
    }
}
