//! C++ on the shared rule engine ([`crate::rules`]): the grammar's types and
//! what its matchers remember about a line.

use std::cell::Cell;

use super::few::Few;
use super::grammar::{Nested, Pattern, Rule, Sc};
use super::limits::{MAX_ENDS, MAX_NESTED};
use super::matchers::{NEEDS, RAW_DELIMITER, RAW_DELIMITER_MAX};
use super::scope::S;
use crate::rules::{self, Act, Captures, Grammar, Payload, ScopeName, table_scan};
use crate::scope::Scopes;

/// The C++ lexer for one line.
pub(crate) type Lexer<'a> = rules::Lexer<'a, Cpp>;
/// What a highlighter keeps from line to line.
pub(crate) type Scratch = rules::Scratch<Cpp>;

/// The `cpp` grammar that Shiki bundles.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Cpp;

/// What the C++ matchers remember about a line.
pub(crate) struct Extra {
    /// The last run of white space found: where it starts, where it ends, and
    /// the text length and line feed it was found with.
    /// A search from inside the run ends where the run does.
    pub(super) blank_run: Cell<(usize, usize, usize, bool)>,
    /// How many of the matchers that loop by calling themselves are active.
    pub(super) loops: Cell<u32>,
    /// How many capture groups are being lexed again around this point.
    nested: u32,
    /// For each text of [`NEEDS`], the last search for it.
    /// It holds:
    /// - where the search started;
    /// - how long the text it searched was;
    /// - the first place at or after the start where the text lies.
    ///
    /// It answers every later search in a text of that length from a place up
    /// to that one.
    /// A capture group lexed again sees a shorter text.
    needs: [Cell<(usize, usize, Option<usize>)>; NEEDS.len()],
}

impl Default for Extra {
    fn default() -> Self {
        Extra {
            blank_run: Cell::new((usize::MAX, 0, 0, false)),
            loops: Cell::new(0),
            nested: 0,
            needs: [const { Cell::new((usize::MAX, 0, None)) }; NEEDS.len()],
        }
    }
}

impl Lexer<'_> {
    /// The first place at or after `p` where the text `NEEDS[need]` lies.
    fn next_need(&self, need: usize, p: usize) -> Option<usize> {
        let cell = &self.x.needs[need];
        let (from, len, found) = cell.get();
        if len == self.s.len() && from <= p && found.is_none_or(|q| q >= p) {
            return found;
        }
        let rest = self.s.get(p..).unwrap_or(&[]);
        let at = memchr::memmem::find(rest, NEEDS[need]);
        self.spend(at.unwrap_or(rest.len()));
        let found = at.map(|q| p + q);
        cell.set((p, self.s.len(), found));
        found
    }

    /// Whether the text `NEEDS[need]` lies inside the text at `p` or after
    /// it: a cheap test that an expression which always contains it can
    /// match.
    pub(crate) fn has_ahead(&self, need: usize, p: usize) -> bool {
        self.next_need(need, p)
            .is_some_and(|q| q + NEEDS[need].len() <= self.s.len())
    }

    /// The first [`MAX_ENDS`] places at or after `from` where the text
    /// `NEEDS[need]` lies.
    pub(crate) fn occurrences(&self, need: usize, from: usize) -> Few<usize, MAX_ENDS> {
        let text = NEEDS[need];
        let mut ends = Few::new();
        let mut at = from;
        while !ends.is_full() {
            let Some(q) = self.next_need(need, at) else {
                break;
            };
            if q + text.len() > self.s.len() {
                break;
            }
            ends.push(q);
            at = q + 1;
        }
        ends
    }
}

impl Cpp {
    /// The prefix of the name of a rule whose name is a prefix followed by
    /// the text of a begin group.
    /// The raw string is that rule.
    /// Its name ends in its delimiter.
    fn delimited_name(rule: Rule) -> Option<&'static str> {
        match rule.definition().name.list {
            [Sc::Group(prefix, _, "")] => Some(prefix),
            _ => None,
        }
    }
}

impl Grammar for Cpp {
    type Scope = Sc;
    const NO_SCOPES: Scopes<Sc> = Sc::NO_SCOPES;
    type Rule = Rule;
    type Pattern = Pattern;
    type Nested = Nested;
    type Extra = Extra;
    type Kept = ();

    const NAME: &'static str = "C++";
    const ROOT: Rule = Rule::Root;
    const SOURCE: crate::scope::Scope = S::Source.scope();
    const GROUPS: usize = 128;
    const CAPTURED_EOL: bool = true;
    const NAMED_PAYLOAD: usize = RAW_DELIMITER_MAX;

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
        if Self::delimited_name(rule).is_some() {
            Sc::NO_SCOPES
        } else {
            rule.definition().name
        }
    }

    fn rule_content(rule: Rule) -> Scopes<Sc> {
        rule.definition().content
    }

    fn rule_payload(rule: Rule) -> Payload {
        let delimiter = Self::delimited_name(rule).map(|_| RAW_DELIMITER);
        rule.definition()
            .backref
            .or(delimiter)
            .map_or(Payload::None, Payload::Backref)
    }

    fn rule_name_delimiter(rule: Rule) -> Option<&'static str> {
        Self::delimited_name(rule)
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
        lexer.matcher(pattern.rx(), p)
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
