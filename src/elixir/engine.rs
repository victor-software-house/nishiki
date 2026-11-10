//! Elixir on the shared rule engine ([`crate::rules`]).
//! The module holds the grammar's types, what its matchers remember about a
//! line, and where HTML in a sigil hands the line back.

use std::cell::Cell;
use std::convert::Infallible;

use super::grammar::{Pattern, Rule};
use super::scope::S;
use crate::arena::Arena;
use crate::classes::is_space;
use crate::language::Halt;
pub(crate) use crate::rules::embedded;
use crate::rules::{self, Act, BlankRuns, Captures, CarriedState, Grammar, ScopeName, table_scan};
use crate::scope::Scopes;
use crate::utf8::multibyte_at;

/// The Elixir lexer for one line.
pub(crate) type Lexer<'a> = rules::Lexer<'a, Elixir>;
/// What a highlighter keeps from line to line.
pub(crate) type Scratch = rules::Scratch<Elixir>;

/// The `elixir` grammar that Shiki bundles.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Elixir;

/// What the Elixir matchers remember about a line.
#[derive(Debug)]
pub(crate) struct Extra {
    /// Where the line's last `->` starts, for the length of the text it was
    /// found in.
    /// `fn` asks whether a `->` follows it, so a line of many `fn` would read
    /// the rest of the line at each.
    /// It is `(usize::MAX, _)` until a matcher first reads it.
    pub(super) last_arrow: Cell<(usize, Option<usize>)>,
}

impl Default for Extra {
    fn default() -> Self {
        Extra {
            last_arrow: Cell::new((usize::MAX, None)),
        }
    }
}

impl Grammar for Elixir {
    type Scope = S;
    const NO_SCOPES: Scopes<S> = S::NO_SCOPES;
    type Rule = Rule;
    type Pattern = Pattern;
    type Nested = Infallible;
    type Extra = Extra;
    type Kept = ();

    const NAME: &'static str = "Elixir";
    const ROOT: Rule = Rule::Root;
    const SOURCE: crate::scope::Scope = S::Source.scope();
    const GROUPS: usize = 5;
    /// A comment's `\n` and a `def` head's `\s*` take the line feed.
    const CAPTURED_EOL: bool = true;
    /// A default argument, `\s(\\\\)`, takes one blank and then requires
    /// `\\` ([`Pattern::takes_one_blank`]).
    /// It matches only at the last blank of a run.
    const BLANK_RUNS: BlankRuns = BlankRuns::ToLast;

    fn takes_one_blank(pattern: Pattern) -> bool {
        pattern.takes_one_blank()
    }

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

    const EMBEDS_ON_DEMAND: bool = true;

    fn rule_carried_state(rule: Rule) -> CarriedState {
        rule.carried_state()
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

    fn tokenize_nested(_: &mut Lexer<'_>, nested: Infallible, _: usize, _: usize, _: u32) {
        match nested {}
    }
}

/// HTML in a `~H"""` or `~L"""` sigil goes back to Elixir at the sigil's
/// end, `\s*"""`.
/// The sigil lists no patterns of its own beside HTML.
impl rules::Embeds for Elixir {
    type End = TripleQuote;
    const HALTS: bool = true;

    fn embed_halts(_: &rules::Embedding, _: &Arena) -> (TripleQuote, Option<rules::FnHalt>) {
        (TripleQuote, None)
    }
}

/// `\s*"""`: the end of a `~H` or `~L` sigil, which may come mid-line.
#[derive(Clone, Copy, Debug)]
pub(crate) struct TripleQuote;

impl Halt for TripleQuote {
    fn first(&self, s: &[u8], from: usize, to: usize) -> Option<usize> {
        (from..=to).find(|&p| self.holds(s, p))
    }

    fn holds(&self, s: &[u8], p: usize) -> bool {
        let mut at = p;
        while let Some(&b) = s.get(at) {
            let (c, width) = if b < 0x80 {
                (char::from(b), 1)
            } else {
                match multibyte_at(s, at, b) {
                    Some(found) => found,
                    None => break,
                }
            };
            if !is_space(c) {
                break;
            }
            at += width;
        }
        s.get(at..).is_some_and(|rest| rest.starts_with(b"\"\"\""))
    }
}
