//! Java on the shared rule engine ([`crate::rules`]): the grammar's types and
//! the answers its lookaheads keep for a line, so that scanning a run of the
//! line stays linear.

use std::cell::Cell;

use super::grammar::{Nested, Pat, Rule};
use super::scope::S;
use crate::rules::{self, Act, Captures, Grammar, Payload, ScopeName, table_scan};

/// The Java lexer for one line.
pub(crate) type Lexer<'a> = rules::Lexer<'a, Java>;
/// What a highlighter keeps from line to line.
pub(crate) type Scratch = rules::Scratch<Java>;

/// The `java` grammar that Shiki bundles.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Java;

/// A run of the line, the lookahead's answer for it, kept for the searches
/// that start inside it: where the run starts and ends, and the place that
/// answers them, the last one in the run.
pub(crate) type RunMemo = Cell<(usize, usize, Option<usize>)>;

/// The answers of lookaheads that scan a run of the line.
#[derive(Debug)]
pub(crate) struct Memo {
    /// For a class, interface or enum keyword in a run of words.
    pub(crate) keyword: RunMemo,
    /// For a record keyword in a run of words.
    pub(crate) record: RunMemo,
    /// For a variable's name and what follows it: the run of characters
    /// they may be made of, and the last `$` in it.
    pub(crate) names: RunMemo,
    /// For a variable's generic arguments: the run of characters they may be
    /// made of, and the last `>` in it that the rest of the definition
    /// follows.
    pub(crate) generics: RunMemo,
    /// For an annotation: the last run of characters that are not `(` or
    /// white space, where it starts and ends.
    pub(crate) annotation: Cell<(usize, usize)>,
    /// For a method: the run with no `=` or `//`, and the last `(` in it.
    pub(crate) method: RunMemo,
    /// For a record's constructor: the run with no `(`, `=` or `//`, and the
    /// last `{` in it.
    pub(crate) constructor: RunMemo,
    /// Whether the last start of a word that follows a blank and precedes a
    /// `(` is known, and where it is.
    pub(crate) signature: Cell<(bool, Option<usize>)>,
}

impl Memo {
    fn new() -> Memo {
        let none = || Cell::new((usize::MAX, 0, None));
        Memo {
            keyword: none(),
            record: none(),
            names: none(),
            generics: none(),
            annotation: Cell::new((usize::MAX, 0)),
            method: none(),
            constructor: none(),
            signature: Cell::new((false, None)),
        }
    }
}

/// What the Java matchers remember about a line.
#[derive(Debug)]
pub(crate) struct Extra {
    /// The last run of word characters found: where it starts and ends. A
    /// search from inside it ends where it does.
    pub(crate) word_run: Cell<(usize, usize)>,
    /// The last run of `[$\w]` characters found, kept the same way.
    pub(crate) name_run: Cell<(usize, usize)>,
    /// The last run of ASCII identifier characters found, kept the same way.
    pub(crate) ascii_run: Cell<(usize, usize)>,
    /// What lookaheads over a whole run of the line found, kept so that a
    /// search from inside the run asks once.
    pub(crate) memo: Memo,
}

impl Default for Extra {
    fn default() -> Self {
        Extra {
            word_run: Cell::new((usize::MAX, 0)),
            name_run: Cell::new((usize::MAX, 0)),
            ascii_run: Cell::new((usize::MAX, 0)),
            memo: Memo::new(),
        }
    }
}

impl Grammar for Java {
    type Scope = S;
    type Rule = Rule;
    type Pat = Pat;
    type Nested = Nested;
    type Extra = Extra;
    type Kept = ();

    const NAME: &'static str = "Java";
    const ROOT: Rule = Rule::Root;
    const SOURCE: crate::scope::Scope = S::Source.scope();
    const GROUPS: usize = 7;

    fn scope(scope: S) -> ScopeName {
        ScopeName::Fixed(scope.scope())
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

    fn pat_captures(pat: Pat) -> (&'static [(u8, &'static [S])], &'static [(u8, Nested)]) {
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
