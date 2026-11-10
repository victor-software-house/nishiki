//! Objective-C++ on the shared rule engine ([`crate::rules`]): the grammar's
//! types and what a line keeps for its matchers.

use super::grammar::{Expr, Nested, Pattern, Rule, Sc};
use super::scope::S;
use crate::objective_c::{Memory, ObjcMemory};
use crate::rules::{self, Act, BlankRuns, Captures, CarriedState, Grammar, ScopeName, table_scan};
use crate::scope::Scopes;

/// The Objective-C++ lexer for one line.
pub(crate) type Lexer<'a> = rules::Lexer<'a, ObjectiveCpp>;
/// What a highlighter keeps from line to line.
pub(crate) type Scratch = rules::Scratch<ObjectiveCpp>;

/// How many capture groups can be lexed again inside one another.
/// Member access lexes its chain again with patterns that hold member access,
/// so the depth follows the text.
/// Past it, a group keeps its own scopes, and the line reports the depth
/// limit.
pub(crate) const MAX_NESTED: u32 = 64;

/// The `objective-cpp` grammar that Shiki bundles.
#[derive(Clone, Copy, Debug)]
pub(crate) struct ObjectiveCpp;

/// What a line of Objective-C++ keeps for its matchers.
#[derive(Debug, Default)]
pub(crate) struct Extra {
    /// How many capture groups are being lexed again around this point.
    nested: u32,
    /// The Objective-C matchers' memory.
    memory: Memory,
}

impl ObjcMemory for Extra {
    #[inline]
    fn memory(&self) -> &Memory {
        &self.memory
    }
}

impl Grammar for ObjectiveCpp {
    type Scope = Sc;
    const NO_SCOPES: Scopes<Sc> = Sc::NO_SCOPES;
    type Rule = Rule;
    type Pattern = Pattern;
    type Nested = Nested;
    type Extra = Extra;
    type Kept = ();

    const NAME: &'static str = "Objective-C++";
    const ROOT: Rule = Rule::Root;
    const SOURCE: crate::scope::Scope = S::Source.scope();
    const GROUPS: usize = 13;
    const CAPTURED_EOL: bool = true;
    /// A parameter name, `(?<=[0-9A-Z_a-z] |…)\s*([A-Z_a-z]\w*)`, can start
    /// at the second blank of a run: the blank before it must follow a word.
    const BLANK_RUNS: BlankRuns = BlankRuns::Visit;

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

    /// A raw string keeps its delimiter, which its end repeats.
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

    /// Objective-C's matchers answer the expressions the grammar shares with
    /// Objective-C; this module's answer the rest.
    fn matcher(lexer: &mut Lexer<'_>, pattern: Pattern, p: usize) -> Option<usize> {
        match pattern.expr() {
            Expr::Shared(rx) => lexer.matcher(rx, p),
            Expr::Own(rx) => lexer.own_matcher(rx, p),
        }
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
