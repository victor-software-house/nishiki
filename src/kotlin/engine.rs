//! Kotlin on the shared rule engine ([`crate::rules`]): the grammar's types.

use std::cell::{Cell, OnceCell};
use std::sync::OnceLock;

use super::grammar::{Nested, Pat, Rule};
use super::scope::S;
use crate::rules::{self, Grammar, PatData, RuleData, Start, Tables};

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

    fn scope(scope: S) -> crate::scope::Scope {
        scope.scope()
    }

    fn rule_code(rule: Rule) -> u16 {
        rule.code()
    }

    fn rule_from_code(code: u16) -> Rule {
        Rule::from_code(code)
    }

    fn rule_data(rule: Rule) -> RuleData<Self> {
        rule.data()
    }

    fn pat_data(pat: Pat) -> PatData<Self> {
        pat.data()
    }

    fn pat_start(pat: Pat) -> Start {
        pat.start()
    }

    fn tables(rule: Rule) -> &'static Tables<Self> {
        static TABLES: [OnceLock<Tables<Kotlin>>; Rule::ALL.len()] =
            [const { OnceLock::new() }; Rule::ALL.len()];
        #[expect(
            clippy::as_conversions,
            reason = "a fieldless `repr(u8)` enum has no `From` impl"
        )]
        let index = rule as usize;
        TABLES[index].get_or_init(|| Tables::build(rule.data()))
    }

    fn matcher(lexer: &mut Lexer<'_>, pat: Pat, p: usize) -> Option<usize> {
        lexer.matcher(pat, p)
    }

    fn tokenize_nested(lexer: &mut Lexer<'_>, kind: Nested, from: usize, to: usize, base: u32) {
        lexer.tokenize_capture(kind, from, to, base);
    }
}
