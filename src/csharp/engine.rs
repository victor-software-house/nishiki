//! C# on the shared rule engine ([`crate::rules`]): the grammar's types.

use std::cell::Cell;

use super::grammar::{Nested, Pattern, Rule};
use super::scope::S;
use crate::rules::{self, Act, Captures, CarriedState, Grammar, ScopeName, table_scan};
use crate::scope::Scopes;

/// The C# lexer for one line.
pub(crate) type Lexer<'a> = rules::Lexer<'a, CSharp>;
/// What a highlighter keeps from line to line.
pub(crate) type Scratch = rules::Scratch<CSharp>;

/// The `csharp` grammar that Shiki bundles.
#[derive(Clone, Copy, Debug)]
pub(crate) struct CSharp;

/// What the C# matchers remember about a line.
#[derive(Debug, Default)]
pub(crate) struct Extra {
    /// The last run of blanks found: where it starts and ends, and the end of
    /// the text it was found in. A search from inside it ends where it does.
    pub(crate) blank_run: Cell<(usize, usize, usize)>,
    /// The last `,` or `:` before the end of the text, for the end it was
    /// found for: a capture lexed again ends before the line does.
    pub(crate) last_comma_or_colon: Cell<Option<(usize, Option<usize>)>>,
}

impl Grammar for CSharp {
    type Scope = S;
    const NO_SCOPES: Scopes<S> = S::NO_SCOPES;
    type Rule = Rule;
    type Pattern = Pattern;
    type Nested = Nested;
    type Extra = Extra;
    type Kept = ();

    const NAME: &'static str = "C#";
    const ROOT: Rule = Rule::Root;
    const MARKS_FIRST_LINE: bool = true;
    const SOURCE: crate::scope::Scope = S::Source.scope();
    const GROUPS: usize = 13;
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

    fn rule_while(rule: Rule) -> Option<Pattern> {
        rule.while_pattern()
    }

    fn rule_carried_state(_rule: Rule) -> CarriedState {
        CarriedState::None
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
        lexer.retokenise(nested.rule(), from, to, base);
    }
}
