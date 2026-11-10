//! Swift on the shared rule engine ([`crate::rules`]): the grammar's types and
//! what the matchers remember about a line.

use super::grammar::{Nested, Pattern, Rule};
use super::matchers;
use super::scope::S;
use crate::rules::{self, Act, Captures, CarriedState, Grammar, ScopeName, table_scan};
use crate::scope::Scopes;

/// The Swift lexer for one line.
pub(crate) type Lexer<'a> = rules::Lexer<'a, Swift>;
/// What a highlighter keeps from line to line.
pub(crate) type Scratch = rules::Scratch<Swift>;

/// The `swift` grammar that Shiki bundles.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Swift;

/// What the Swift matchers remember about a line: one memo for each file of
/// matchers.
/// A lookahead that scans the rest of the line is then asked once.
#[derive(Debug, Default)]
pub(crate) struct Extra {
    pub(super) declarations: matchers::declarations::Memo,
    pub(super) expressions: matchers::expressions::Memo,
    pub(super) literals: matchers::literals::Memo,
    pub(super) names: matchers::names::Memo,
    pub(super) regex_literals: matchers::regex_literals::Memo,
    pub(super) types: matchers::types::Memo,
}

impl Grammar for Swift {
    type Scope = S;
    const NO_SCOPES: Scopes<S> = S::NO_SCOPES;
    type Rule = Rule;
    type Pattern = Pattern;
    type Nested = Nested;
    type Extra = Extra;
    type Kept = Vec<(usize, usize)>;

    const NAME: &'static str = "Swift";
    const ROOT: Rule = Rule::Root;
    const MARKS_FIRST_LINE: bool = true;
    const SOURCE: crate::scope::Scope = S::Source.scope();
    const GROUPS: usize = 32;
    const CAPTURED_EOL: bool = true;

    fn scope(scope: S) -> ScopeName {
        ScopeName::Fixed(scope.scope())
    }

    fn extra(kept: Vec<(usize, usize)>) -> Extra {
        Extra {
            regex_literals: matchers::regex_literals::Memo::with_path(kept),
            ..Extra::default()
        }
    }

    fn keep(extra: Extra) -> Vec<(usize, usize)> {
        extra.regex_literals.into_path()
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
        rule.definition()
            .backref
            .map_or(CarriedState::None, CarriedState::Backref)
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
        lexer.retokenise(nested, from, to, base);
    }
}

#[cfg(test)]
impl Lexer<'_> {
    /// Tries `pattern` at `p`, for the tool that compares the matchers with
    /// Oniguruma.
    pub(super) fn probe(&mut self, pattern: Pattern, p: usize) -> Option<usize> {
        self.matcher(pattern, p)
    }
}
