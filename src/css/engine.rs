//! CSS on the shared rule engine ([`crate::rules`]): the grammar's types and
//! the answers its lookaheads keep for a line, so that scanning a run of the
//! line stays linear.

use std::cell::{Cell, OnceCell};
use std::sync::OnceLock;

use super::ahead::Ahead;
use super::grammar::{Nested, Pat, Rule};
use super::scope::S;
use crate::rules::{self, Grammar, PatData, RuleData, Start, Tables};

/// The CSS lexer for one line.
pub(crate) type Lexer<'a> = rules::Lexer<'a, Css>;
/// What a highlighter keeps from line to line.
pub(crate) type Scratch = rules::Scratch<Css>;

/// The `css` grammar that Shiki bundles.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Css;

/// What the CSS matchers remember about a line.
#[derive(Debug)]
pub(crate) struct Extra {
    /// The last run of `[-\w]` characters found: where it starts and ends. A
    /// search from inside it ends where it does.
    pub(crate) word_run: Cell<(usize, usize)>,
    /// The last run of identifier characters found, kept the same way when
    /// it has no escape, which would make a search from inside it differ.
    pub(crate) ident_run: Cell<(usize, usize)>,
    /// The last run of white space found, kept the same way.
    pub(crate) space_run: Cell<(usize, usize)>,
    /// The last number found: where the search started, and where the number
    /// ends, so the patterns of its units ask once.
    pub(crate) number: Cell<(usize, Option<usize>)>,
    /// Where the bytes the lookaheads search for are.
    pub(crate) ahead: OnceCell<Ahead>,
}

impl Default for Extra {
    fn default() -> Self {
        Extra {
            word_run: Cell::new((usize::MAX, 0)),
            ident_run: Cell::new((usize::MAX, 0)),
            space_run: Cell::new((usize::MAX, 0)),
            number: Cell::new((usize::MAX, None)),
            ahead: OnceCell::new(),
        }
    }
}

impl Grammar for Css {
    type Scope = S;
    type Rule = Rule;
    type Pat = Pat;
    type Nested = Nested;
    type Extra = Extra;

    const NAME: &'static str = "CSS";
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
        static TABLES: [OnceLock<Tables<Css>>; Rule::ALL.len()] =
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
