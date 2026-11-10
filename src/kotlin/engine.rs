//! Kotlin on the shared rule engine ([`crate::rules`]): the grammar's types.

use std::cell::{Cell, OnceCell};

use super::grammar::{Nested, Pattern, Rule};
use super::scope::S;
use crate::rules::{self, Act, Captures, Grammar, Payload, ScopeName, table_scan};
use crate::scope::Scopes;

/// The Kotlin lexer for one line.
pub(crate) type Lexer<'a> = rules::Lexer<'a, Kotlin>;
/// What a highlighter keeps from line to line.
pub(crate) type Scratch = rules::Scratch<Kotlin>;

/// What the Kotlin matchers remember about a line.
/// A lookahead that scans the rest of the line is then asked once.
#[derive(Debug, Default)]
pub(crate) struct Extra {
    /// The last `>` of the line, once looked for: a balanced angle group
    /// cannot close after it.
    pub(super) last_angle_close: OnceCell<Option<usize>>,
    /// The last `)` or `>` of the line, once looked for.
    pub(super) last_bracket_close: OnceCell<Option<usize>>,
    /// The last `}` of the line, once looked for.
    pub(super) last_brace: OnceCell<Option<usize>>,
    /// The last run of characters that are not blanks that a matcher scanned:
    /// where it starts and ends, and the last `]` in it.
    /// A search that starts inside the run ends where the run does.
    pub(super) run: Cell<(usize, usize, Option<usize>)>,
}

/// The `kotlin` grammar that Shiki bundles.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Kotlin;

impl Grammar for Kotlin {
    type Scope = S;
    const NO_SCOPES: Scopes<S> = S::NO_SCOPES;
    type Rule = Rule;
    type Pattern = Pattern;
    type Nested = Nested;
    type Extra = Extra;
    type Kept = ();

    const NAME: &'static str = "Kotlin";
    const ROOT: Rule = Rule::Root;
    const SOURCE: crate::scope::Scope = S::Source.scope();
    const GROUPS: usize = 6;
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

    fn tokenize_nested(lexer: &mut Lexer<'_>, nested: Nested, from: usize, to: usize, base: u32) {
        lexer.tokenize_capture(nested, from, to, base);
    }
}
