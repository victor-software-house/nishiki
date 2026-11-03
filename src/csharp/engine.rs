//! C# on the shared rule engine ([`crate::rules`]): the grammar's types.

use std::cell::Cell;

use super::grammar::{Nested, Pat, Rule};
use super::scope::S;
use crate::rules::{self, Act, Captures, Grammar, Payload, table_scan};

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
    type Rule = Rule;
    type Pat = Pat;
    type Nested = Nested;
    type Extra = Extra;

    const NAME: &'static str = "C#";
    const ROOT: Rule = Rule::Root;
    const MARKS_FIRST_LINE: bool = true;
    const SOURCE: S = S::Source;
    const GROUPS: usize = 13;
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

    fn rule_while(rule: Rule) -> Option<Pat> {
        rule.while_pat()
    }

    fn rule_payload(_rule: Rule) -> Payload {
        Payload::None
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
        lexer.retokenise(kind.rule(), from, to, base);
    }
}
