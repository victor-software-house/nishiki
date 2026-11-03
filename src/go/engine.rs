//! Go on the shared rule engine ([`crate::rules`]): the grammar's types and
//! the answers its lookaheads keep for a line, so that scanning a run of the
//! line stays linear.

use std::cell::Cell;
use std::sync::OnceLock;

use super::grammar::{Pat, Rule};
use super::scope::S;
use crate::rules::{self, Act, Captures, Grammar, Payload, Start, Tabled, Tables, table_scan};

/// The Go lexer for one line.
pub(crate) type Lexer<'a> = rules::Lexer<'a, Go>;
/// What a highlighter keeps from line to line.
pub(crate) type Scratch = rules::Scratch<Go>;

/// The `go` grammar that Shiki bundles.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Go;

/// What the Go matchers remember about a line.
#[derive(Debug)]
pub(crate) struct Extra {
    /// The end of the text the runs below were found in, with its line feed:
    /// lexing a capture group's text again moves it, and a run found up to
    /// another end is not the run up to this one.
    pub(crate) runs_end: Cell<usize>,
    /// The last run of `\w` characters found: where it starts and ends. A
    /// search from inside it ends where it does.
    pub(crate) word_run: Cell<(usize, usize)>,
    /// The last run of white space found, kept the same way.
    pub(crate) space_run: Cell<(usize, usize)>,
    /// The last run found of each class the hot matchers scan, kept the same
    /// way: `\w` and `[*.\w]` items, the comma lists of both, and `[.\w]`.
    pub(crate) runs: [Cell<(usize, usize)>; 5],
}

impl Default for Extra {
    fn default() -> Self {
        Extra {
            runs_end: Cell::new(usize::MAX),
            word_run: Cell::new((usize::MAX, 0)),
            space_run: Cell::new((usize::MAX, 0)),
            runs: [const { Cell::new((usize::MAX, 0)) }; 5],
        }
    }
}

impl Grammar for Go {
    type Scope = S;
    type Rule = Rule;
    type Pat = Pat;
    type Nested = Rule;
    type Extra = Extra;

    const NAME: &'static str = "Go";
    const ROOT: Rule = Rule::Root;
    const GROUPS: usize = 54;
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

    fn tokenize_nested(lexer: &mut Lexer<'_>, kind: Rule, from: usize, to: usize, base: u32) {
        lexer.tokenize_capture(kind, from, to, base);
    }
}

impl Tabled for Go {
    fn pat_start(pat: Pat) -> Start {
        pat.start()
    }

    fn tables(rule: Rule) -> &'static Tables<Self> {
        static TABLES: [OnceLock<Tables<Go>>; Rule::ALL.len()] =
            [const { OnceLock::new() }; Rule::ALL.len()];
        #[expect(
            clippy::as_conversions,
            reason = "a fieldless `repr(u8)` enum has no `From` impl"
        )]
        let index = rule as usize;
        TABLES[index].get_or_init(|| Tables::build(rule.data()))
    }
}
