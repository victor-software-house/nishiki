//! Haskell on the shared rule engine ([`crate::rules`]): the grammar's types.

use super::grammar::{Nested, Pattern, Rule, Sc};
use super::scope::S;
use crate::rules::{self, Act, Captures, Grammar, Payload, ScopeName, table_scan};
use crate::scope::Scopes;

/// The Haskell lexer for one line.
pub(crate) type Lexer<'a> = rules::Lexer<'a, Haskell>;
/// What a highlighter keeps from line to line.
pub(crate) type Scratch = rules::Scratch<Haskell>;

/// The `haskell` grammar that Shiki bundles.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Haskell;

impl Grammar for Haskell {
    type Scope = Sc;
    const NO_SCOPES: Scopes<Sc> = Sc::NO_SCOPES;
    type Rule = Rule;
    type Pattern = Pattern;
    type Nested = Nested;
    type Extra = ();
    type Kept = ();

    const NAME: &'static str = "Haskell";
    const ROOT: Rule = Rule::Root;
    const SOURCE: crate::scope::Scope = S::Source.scope();
    const GROUPS: usize = 16;
    const CAPTURED_EOL: bool = true;

    fn scope(scope: Sc) -> ScopeName {
        match scope {
            Sc::S(scope) => ScopeName::Fixed(scope.scope()),
            Sc::Group(prefix, group, suffix) => ScopeName::Capture {
                prefix,
                group,
                suffix,
            },
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

    fn rule_payload(rule: Rule) -> Payload {
        rule.definition()
            .backref
            .map_or(Payload::None, Payload::Backref)
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

    fn tokenize_nested(lexer: &mut Lexer<'_>, nested: Rule, from: usize, to: usize, base: u32) {
        lexer.tokenize_capture(nested, from, to, base);
    }
}
