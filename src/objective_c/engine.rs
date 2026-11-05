//! Objective-C on the shared rule engine ([`crate::rules`]): the grammar's
//! types and what its matchers remember about a line.

use std::cell::Cell;

use super::grammar::{Nested, Pat, Rule, Sc};
use super::scope::S;
use crate::rules::{self, Act, Captures, Grammar, ScopeName, table_scan};
use crate::scope::Scopes;

/// The Objective-C lexer for one line.
pub(crate) type Lexer<'a> = rules::Lexer<'a, ObjectiveC>;
/// What a highlighter keeps from line to line.
pub(crate) type Scratch = rules::Scratch<ObjectiveC>;

/// How many capture groups can be lexed again inside one another.
/// Member access lexes its chain again with patterns that hold member access,
/// so the depth follows the text.
/// Past it, a group keeps its own scopes, and the line reports the depth
/// limit.
pub(crate) const MAX_NESTED: u32 = 64;

/// The `objective-c` grammar that Shiki bundles.
#[derive(Clone, Copy, Debug)]
pub(crate) struct ObjectiveC;

/// What the Objective-C matchers remember about a line.
#[derive(Debug)]
pub(crate) struct Extra {
    /// How many capture groups are being lexed again around this point.
    nested: u32,
    /// The last scoped name walked: where it starts and ends, and the length
    /// of the text it was walked in.
    /// The call patterns have no `\b` before the name, so they ask at every
    /// byte of a name.
    /// A walk entered at a later letter of this one ends where it does.
    pub(super) scoped: Cell<(usize, usize, usize)>,
    /// The last run of `\w` characters measured, in the same shape.
    /// Patterns with no `\b` before them measure a word from each of its
    /// bytes.
    pub(super) word_run: Cell<(usize, usize, usize)>,
    /// The last run of `[0-9A-Z_a-z]` measured, in the same shape.
    pub(super) ascii_run: Cell<(usize, usize, usize)>,
    /// The last word looked up in the word table: where it starts and ends,
    /// the length of the text, and the lists it belongs to.
    pub(super) listed: Cell<(usize, usize, usize, u32)>,
}

impl Default for Extra {
    fn default() -> Self {
        Extra {
            nested: 0,
            scoped: Cell::new((usize::MAX, 0, 0)),
            word_run: Cell::new((usize::MAX, 0, 0)),
            ascii_run: Cell::new((usize::MAX, 0, 0)),
            listed: Cell::new((usize::MAX, 0, 0, 0)),
        }
    }
}

impl Grammar for ObjectiveC {
    type Scope = Sc;
    const NO_SCOPES: Scopes<Sc> = Sc::NO_SCOPES;
    type Rule = Rule;
    type Pat = Pat;
    type Nested = Nested;
    type Extra = Extra;
    type Kept = ();

    const NAME: &'static str = "Objective-C";
    const ROOT: Rule = Rule::Root;
    const SOURCE: crate::scope::Scope = S::Source.scope();
    const GROUPS: usize = 13;
    const CAPTURED_EOL: bool = true;
    /// A parameter name, `(?<=[0-9A-Z_a-z] |…)\s*([A-Z_a-z]\w*)`, can start
    /// at the second blank of a run: the blank before it must follow a word.
    const SKIPS_BLANK_RUNS: bool = false;

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
        rule.data().name
    }

    fn rule_content(rule: Rule) -> Scopes<Sc> {
        rule.data().content
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
        lexer.matcher(pat.rx(), p)
    }

    /// Lexes the group again with its rule, under the stack the engine
    /// gives and the group's own scopes.
    /// Past [`MAX_NESTED`] groups lexed again around it, the text keeps the
    /// group's scopes alone.
    /// The line then reports the depth limit.
    fn tokenize_nested(lexer: &mut Lexer<'_>, kind: Nested, from: usize, to: usize, base: u32) {
        let inner = lexer.path(base, kind.scopes);
        if lexer.x.nested >= MAX_NESTED {
            lexer.record_depth();
            lexer.emit(to, inner);
            return;
        }
        lexer.x.nested += 1;
        lexer.retokenise(kind.rule, from, to, inner);
        lexer.x.nested -= 1;
    }
}
