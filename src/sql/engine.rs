//! SQL on the shared rule engine ([`crate::rules`]).
//! The matchers answer with a [`Hit`].
//! The glue here turns each hit into the spans the engine emits.
//!
//! At each position the search tries the end pattern of the innermost rule,
//! then the rule's patterns in order.
//! In a bracket, the patterns come before the end pattern.
//! The search takes the first pattern that matches at the earliest position.

use std::cell::Cell;

use super::rules::{Pattern, patterns};
use super::scope::S;
use super::state::Rule;
use crate::language::{Halts, NO_HALTS};
use crate::rules::{self, Act, Captures, Grammar, ScopeName};
use crate::scope::Scopes;
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
    /// found the byte.
    /// A line of openers that never close then scans to its end only once.
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
pub(super) struct Capture {
    pub(super) from: usize,
    pub(super) to: usize,
    pub(super) scope: S,
}

/// A match: the bytes `start..end` and the capture groups in order.
#[derive(Clone, Copy, Debug)]
pub(super) struct Hit {
    pub(super) start: usize,
    pub(super) end: usize,
    pub(super) captures: [Option<Capture>; 4],
}

impl Hit {
    pub(super) fn new(start: usize, end: usize) -> Hit {
        Hit {
            start,
            end,
            captures: [None; 4],
        }
    }

    /// The match with a group `from..to` that adds `scope`, in slot `slot`.
    pub(super) fn capture(mut self, slot: usize, from: usize, to: usize, scope: S) -> Hit {
        self.captures[slot] = Some(Capture { from, to, scope });
        self
    }
}

/// A byte that closes a match.
/// The search finds it once per line, however many openers there are.
#[derive(Clone, Copy, Debug)]
pub(super) enum Closer {
    /// The `'` that closes a single-quoted string.
    Quote,
    /// The `}` that closes an interpolation.
    Brace,
}

impl Lexer<'_> {
    /// The first `closer` at or after `from` in the text.
    /// The last search for it answers when that search covers `from`.
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

    /// Tries `pattern` at `p`.
    /// On a match, it records the capture groups as spans and returns where
    /// the match ends.
    ///
    /// It is `#[inline]`, as `pattern_at` is.
    /// The scan's whole dispatch for a candidate pattern then inlines at the
    /// default sixteen codegen units (`bench/AGENTS.md` rule 13).
    #[inline]
    fn hit(&mut self, pattern: Pattern, p: usize) -> Option<usize> {
        let hit = self.pattern_at(pattern, p)?;
        self.x.start = hit.start;
        self.clear_spans();
        for capture in hit.captures.iter().flatten() {
            if capture.from < capture.to {
                self.span_one(capture.from, capture.to, Some(capture.scope), None);
            }
        }
        Some(hit.end)
    }

    /// The first pattern of `rule` that matches at `p`, after or before its
    /// end pattern.
    #[expect(
        clippy::inline_always,
        reason = "measured at one codegen unit: called from both instances of the scan, \
                  the halting one and the plain one, it stopped being inlined, at a call per place"
    )]
    #[inline(always)]
    fn first_at(&mut self, rule: Rule, p: usize) -> Option<(Pattern, usize)> {
        if rule != Rule::Bracket
            && let Some(end) = self.hit(Pattern::End, p)
        {
            return Some((Pattern::End, end));
        }
        for &pattern in patterns(rule) {
            if let Some(end) = self.hit(pattern, p) {
                return Some((pattern, end));
            }
        }
        if rule == Rule::Bracket {
            return self.hit(Pattern::End, p).map(|end| (Pattern::End, end));
        }
        None
    }
}

impl Grammar for Sql {
    type Scope = S;
    const NO_SCOPES: Scopes<S> = S::NO_SCOPES;
    type Rule = Rule;
    type Pattern = Pattern;
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

    fn rule_name(rule: Rule) -> Scopes<S> {
        rule.name()
    }

    fn rule_content(_: Rule) -> Scopes<S> {
        S::NO_SCOPES
    }

    fn pattern_act(pattern: Pattern) -> Act<Self> {
        match (pattern, pattern.opens()) {
            (Pattern::End, _) => Act::End,
            (_, Some(rule)) => Act::Begin(rule),
            (_, None) => Act::Match(pattern.name()),
        }
    }

    fn pattern_captures(_: Pattern) -> Captures<Self> {
        (&[], &[])
    }

    /// The first match from the position: see `search`.
    fn scan(lexer: &mut Lexer<'_>) -> Option<(usize, Pattern, usize)> {
        match lexer.root_halts() {
            Some(halts) => lexer.search::<true>(halts),
            None => lexer.search::<false>(&NO_HALTS),
        }
    }

    fn matcher(lexer: &mut Lexer<'_>, pattern: Pattern, p: usize) -> Option<usize> {
        lexer.hit(pattern, p)
    }

    fn tokenize_nested(_: &mut Lexer<'_>, nested: Nested, _: usize, _: usize, _: u32) {
        match nested {}
    }
}

impl Lexer<'_> {
    /// The first match from the position.
    /// The bytes the rule's skip passes start none of its patterns and end it
    /// nowhere.
    /// A rule the depth limit keeps closed leaves its text plain.
    /// That rule matched nothing.
    /// It would match again at the same place.
    /// The search goes on one character later instead.
    ///
    /// When `HALTING`, a language that embeds SQL takes the text back at a
    /// place where `halt` holds before the patterns.
    /// It also takes it back where `after` holds once the patterns fail there.
    /// Only the root is open then.
    /// Without `HALTING` the tests compile out.
    /// SQL that nothing embeds pays nothing for them.
    fn search<const HALTING: bool>(
        &mut self,
        halts: &Halts<'_>,
    ) -> Option<(usize, Pattern, usize)> {
        let rule = self.top_rule();
        let end = self.lim() + usize::from(self.nl);
        let mut p = self.pos;
        'search: while p <= end {
            self.spend(1);
            if self.over_budget() {
                return None;
            }
            p = if HALTING {
                self.root_skip::<true>(p, halts)
            } else {
                self.skip_to(rule, p)
            };
            if p > end {
                break;
            }
            let boundary = p >= self.lim() || !is_continuation(self.s[p]);
            if HALTING && boundary && halts.0.holds(self.s, p) {
                self.halt_here(p);
                return None;
            }
            if boundary && let Some((pattern, hit_end)) = self.first_at(rule, p) {
                if pattern.opens().is_some() && self.at_depth_limit() {
                    self.record_depth();
                    let start = self.x.start;
                    let next = if hit_end > start {
                        hit_end
                    } else {
                        start + self.s.get(start).map_or(1, |&lead| char_len(lead))
                    };
                    self.skip_plain(start, next);
                    p = self.pos;
                    continue 'search;
                }
                return Some((self.x.start, pattern, hit_end));
            }
            if HALTING && boundary && halts.1.is_some_and(|after| after.holds(self.s, p)) {
                self.halt_here(p);
                return None;
            }
            p += 1;
        }
        None
    }
}
