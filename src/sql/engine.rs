//! SQL on the shared rule engine ([`crate::rules`]): the matchers answer with
//! a [`Hit`], which the glue here turns into the spans the engine emits.
//!
//! At each position the search tries the end pattern of the innermost rule
//! and then the rule's patterns in order, except in a bracket, whose patterns
//! come first, and takes the first that matches at the earliest position.

use std::cell::Cell;

use super::rules::{Pat, patterns};
use super::scope::S;
use super::state::Rule;
use crate::rules::{self, Act, Captures, Grammar, ScopeName};
use crate::utf8::{char_len, is_continuation};

/// The SQL lexer for one line.
pub(super) type Lexer<'a> = rules::Lexer<'a, Sql>;
/// What a highlighter keeps from line to line.
pub(crate) type Scratch = rules::Scratch<Sql>;

/// The `sql` grammar that Shiki bundles.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Sql;

/// The capture groups the grammar tokenizes with patterns of their own: none.
#[derive(Clone, Copy, Debug)]
pub(crate) enum Nested {}

/// What the SQL matchers remember about a line.
#[derive(Debug)]
pub(crate) struct Extra {
    /// Where the last match started.
    start: usize,
    /// The last run of word characters measured: where it starts and ends.
    /// The rules of a word ask about the same one in turn.
    pub(super) run: Cell<(usize, usize)>,
    /// The last search for each [`Closer`]: where it started and where it
    /// found the byte, so a line of openers that never close does not scan
    /// to its end for each.
    quote_find: Cell<(usize, Option<usize>)>,
    brace_find: Cell<(usize, Option<usize>)>,
}

impl Default for Extra {
    /// Caches that answer for no position.
    fn default() -> Self {
        Extra {
            start: 0,
            run: Cell::new((usize::MAX, 0)),
            quote_find: Cell::new((usize::MAX, None)),
            brace_find: Cell::new((usize::MAX, None)),
        }
    }
}

/// A capture group of a match that adds a scope.
#[derive(Clone, Copy, Debug)]
pub(super) struct Cap {
    pub(super) from: usize,
    pub(super) to: usize,
    pub(super) scope: S,
}

/// A match: the bytes `start..end` and the capture groups in order.
#[derive(Clone, Copy, Debug)]
pub(super) struct Hit {
    pub(super) start: usize,
    pub(super) end: usize,
    pub(super) caps: [Option<Cap>; 4],
}

impl Hit {
    pub(super) fn new(start: usize, end: usize) -> Hit {
        Hit {
            start,
            end,
            caps: [None; 4],
        }
    }

    /// The match with a group `from..to` that adds `scope`, in slot `slot`.
    pub(super) fn cap(mut self, slot: usize, from: usize, to: usize, scope: S) -> Hit {
        self.caps[slot] = Some(Cap { from, to, scope });
        self
    }
}

/// A byte that closes a match, which the search finds once per line however
/// many openers there are.
#[derive(Clone, Copy, Debug)]
pub(super) enum Closer {
    /// The `'` that closes a single-quoted string.
    Quote,
    /// The `}` that closes an interpolation.
    Brace,
}

impl Lexer<'_> {
    /// The first `closer` at or after `from` in the text, answered from the
    /// last search for it when that covers `from`.
    pub(super) fn find_closer(&self, closer: Closer, from: usize) -> Option<usize> {
        let (cell, byte) = match closer {
            Closer::Quote => (&self.x.quote_find, b'\''),
            Closer::Brace => (&self.x.brace_find, b'}'),
        };
        let (start, found) = cell.get();
        if start <= from && found.is_none_or(|at| at >= from) {
            return found;
        }
        let rest = self.s.get(from..self.lim())?;
        let found = memchr::memchr(byte, rest).map(|at| at + from);
        self.spend(found.map_or(rest.len(), |at| at - from));
        cell.set((from, found));
        found
    }

    /// Tries `pat` at `p` and, on a match, records its capture groups as
    /// spans and returns where it ends.
    fn hit(&mut self, pat: Pat, p: usize) -> Option<usize> {
        let hit = self.pat_at(pat, p)?;
        self.x.start = hit.start;
        self.clear_spans();
        for cap in hit.caps.iter().flatten() {
            if cap.from < cap.to {
                self.span_one(cap.from, cap.to, Some(cap.scope), None);
            }
        }
        Some(hit.end)
    }

    /// The first pattern of `rule` that matches at `p`, after or before its
    /// end pattern.
    fn first_at(&mut self, rule: Rule, p: usize) -> Option<(Pat, usize)> {
        if rule != Rule::Bracket
            && let Some(end) = self.hit(Pat::End, p)
        {
            return Some((Pat::End, end));
        }
        for &pat in patterns(rule) {
            if let Some(end) = self.hit(pat, p) {
                return Some((pat, end));
            }
        }
        if rule == Rule::Bracket {
            return self.hit(Pat::End, p).map(|end| (Pat::End, end));
        }
        None
    }
}

impl Grammar for Sql {
    type Scope = S;
    type Rule = Rule;
    type Pat = Pat;
    type Nested = Nested;
    type Extra = Extra;
    type Kept = ();

    const NAME: &'static str = "SQL";
    const ROOT: Rule = Rule::Root;
    const SOURCE: crate::scope::Scope = S::Source.scope();

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
        rule.name()
    }

    fn rule_content(_: Rule) -> &'static [S] {
        &[]
    }

    fn pat_act(pat: Pat) -> Act<Self> {
        match (pat, pat.opens()) {
            (Pat::End, _) => Act::End,
            (_, Some(rule)) => Act::Begin(rule),
            (_, None) => Act::Match(pat.name()),
        }
    }

    fn pat_captures(_: Pat) -> Captures<Self> {
        (&[], &[])
    }

    /// The first match from the position. The bytes the rule's skip passes
    /// start none of its patterns and end it nowhere. A rule the depth limit
    /// keeps closed leaves its text plain, and since a rule that matched
    /// nothing would match again where it did, the search goes on one
    /// character later.
    fn scan(lexer: &mut Lexer<'_>) -> Option<(usize, Pat, usize)> {
        let rule = lexer.top_rule();
        let end = lexer.lim() + usize::from(lexer.nl);
        let mut p = lexer.pos;
        'search: while p <= end {
            lexer.spend(1);
            if lexer.over_budget() {
                return None;
            }
            p = lexer.skip_to(rule, p);
            if p > end {
                break;
            }
            if (p >= lexer.lim() || !is_continuation(lexer.s[p]))
                && let Some((pat, hit_end)) = lexer.first_at(rule, p)
            {
                if pat.opens().is_some() && lexer.at_depth_limit() {
                    lexer.record_depth();
                    let start = lexer.x.start;
                    let next = if hit_end > start {
                        hit_end
                    } else {
                        start + lexer.s.get(start).map_or(1, |&lead| char_len(lead))
                    };
                    lexer.skip_plain(start, next);
                    p = lexer.pos;
                    continue 'search;
                }
                return Some((lexer.x.start, pat, hit_end));
            }
            p += 1;
        }
        None
    }

    fn matcher(lexer: &mut Lexer<'_>, pat: Pat, p: usize) -> Option<usize> {
        lexer.hit(pat, p)
    }

    fn tokenize_nested(_: &mut Lexer<'_>, kind: Nested, _: usize, _: usize, _: u32) {
        match kind {}
    }
}
