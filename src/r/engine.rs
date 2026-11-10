//! R on the shared rule engine ([`crate::rules`]): the grammar's types.

use super::grammar::{Pattern, Rule};
use super::scope::S;
use crate::rules::{self, Act, Captures, CarriedState, Grammar, ScopeName, table_scan};
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
    type Pattern = Pattern;
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
        rule.definition().name
    }

    fn rule_content(rule: Rule) -> Scopes<S> {
        rule.definition().content
    }

    fn rule_carried_state(rule: Rule) -> CarriedState {
        rule.carried_state()
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

    fn matcher(lexer: &mut Lexer<'_>, pattern: Pattern, p: usize) -> Option<usize> {
        lexer.matcher(pattern, p)
    }

    /// The condition of `@examplesIf` is the one group lexed again.
    /// Its patterns record no group of their own, so it nests one level.
    fn tokenize_nested(lexer: &mut Lexer<'_>, nested: Rule, from: usize, to: usize, base: u32) {
        lexer.tokenize_capture(nested, from, to, base);
    }
}
