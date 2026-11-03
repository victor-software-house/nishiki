//! Kotlin on the shared rule engine ([`crate::rules`]): the grammar's types.

use std::cell::{Cell, OnceCell};

use super::grammar::{Nested, Pat, Rule};
use super::scope::S;
use crate::rules::{self, Act, Captures, Grammar, Payload, table_scan};

/// The Kotlin lexer for one line.
pub(crate) type Lexer<'a> = rules::Lexer<'a, Kotlin>;
/// What a highlighter keeps from line to line.
pub(crate) type Scratch = rules::Scratch<Kotlin>;

/// What the Kotlin matchers remember about a line, so that a lookahead that
/// scans the rest of the line is asked once.
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
    /// where it starts and ends, and the last `]` in it. A search that starts
    /// inside the run ends where it does.
    pub(super) run: Cell<(usize, usize, Option<usize>)>,
}

/// The `kotlin` grammar that Shiki bundles.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Kotlin;

impl Grammar for Kotlin {
    type Scope = S;
    type Rule = Rule;
    type Pat = Pat;
    type Nested = Nested;
    type Extra = Extra;

    const NAME: &'static str = "Kotlin";
    const ROOT: Rule = Rule::Root;
    const SOURCE: S = S::Source;
    const GROUPS: usize = 6;
    const CAPTURED_EOL: bool = true;

    fn scope(scope: S) -> crate::scope::Scope {
        scope.scope()
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

    fn tokenize_nested(lexer: &mut Lexer<'_>, kind: Nested, from: usize, to: usize, base: u32) {
        lexer.tokenize_capture(kind, from, to, base);
    }
}
