//! GraphQL on the shared rule engine ([`crate::rules`]).
//! The matchers answer with a [`Hit`].
//! The code here turns a hit into the spans the engine emits.
//!
//! At each position the search tries the end pattern of the innermost rule.
//! Then it tries the rule's patterns in order.
//! It takes the first that matches at the earliest position.
//! A rule marked `applyEndPatternLast` tries its end pattern after its
//! patterns.
//!
//! An interpolation hands its text to the JavaScript lexer.
//! The engine runs it up to the `}` that no JavaScript context waits for.

use std::cell::Cell;

use super::rules::{Pat, patterns};
use super::scope::S;
use super::state::Rule;
use crate::Language;
use crate::arena::Arena;
use crate::language::{Halts, NO_HALTS};
use crate::rules::{self, Act, Captures, Embedding, Embeds, FnHalt, Grammar, Payload, ScopeName};
use crate::scope::Scopes;
use crate::utf8::{char_len, is_continuation};

/// The GraphQL lexer for one line.
pub(super) type Lexer<'a> = rules::Lexer<'a, GraphQl>;
/// What a highlighter keeps from line to line.
pub(crate) type Scratch = rules::Scratch<GraphQl>;

/// The `graphql` grammar that Shiki bundles.
#[derive(Clone, Copy, Debug)]
pub(crate) struct GraphQl;

/// The capture groups the grammar tokenizes with patterns of their own: none.
#[derive(Clone, Copy, Debug)]
pub(crate) enum Nested {}

/// What the GraphQL matchers remember about a line.
#[derive(Debug, Default)]
pub(crate) struct Extra {
    /// The last run of white space found: where the scan started and where
    /// the run ends.
    /// A scan from any byte inside it is then one lookup.
    pub(super) space_run: Cell<(usize, usize)>,
    /// The last run of name characters found, in the same way.
    pub(super) word_run: Cell<(usize, usize)>,
}

/// A capture group that carries a scope.
/// `inner` is a second group over the same bytes, nested in it.
#[derive(Clone, Copy, Debug)]
pub(super) struct Cap {
    pub(super) from: usize,
    pub(super) to: usize,
    pub(super) scope: S,
    pub(super) inner: Option<S>,
}

/// A match: the bytes `start..end` and the capture groups in order.
#[derive(Clone, Copy, Debug)]
pub(super) struct Hit {
    pub(super) start: usize,
    pub(super) end: usize,
    pub(super) caps: [Option<Cap>; 5],
}

impl Hit {
    pub(super) fn new(start: usize, end: usize) -> Hit {
        Hit {
            start,
            end,
            caps: [None; 5],
        }
    }

    /// The match with a group `from..to` that adds `scope`, in slot `slot`.
    pub(super) fn cap(mut self, slot: usize, from: usize, to: usize, scope: S) -> Hit {
        self.caps[slot] = Some(Cap {
            from,
            to,
            scope,
            inner: None,
        });
        self
    }

    /// The match with a group `from..to` that adds `scope`.
    /// A nested group over the same bytes adds `inner`.
    pub(super) fn nested(mut self, slot: usize, from: usize, to: usize, scope: S, inner: S) -> Hit {
        self.caps[slot] = Some(Cap {
            from,
            to,
            scope,
            inner: Some(inner),
        });
        self
    }
}

impl Lexer<'_> {
    /// Records the capture groups of `hit` as spans.
    /// `hit` is the match the search takes.
    /// Returns where it starts and ends.
    #[inline]
    fn record(&mut self, hit: &Hit) -> (usize, usize) {
        self.clear_spans();
        for cap in hit.caps.iter().flatten() {
            if cap.from < cap.to {
                self.span_one(cap.from, cap.to, Some(cap.scope), None);
                if cap.inner.is_some() {
                    self.span_one(cap.from, cap.to, cap.inner, None);
                }
            }
        }
        (hit.start, hit.end)
    }

    /// The first pattern of `rule` that matches at `p`, with where its match
    /// starts and ends.
    /// The end pattern comes before the patterns `pats`.
    /// With `end_last`, it comes after them.
    #[inline]
    fn first_at(
        &mut self,
        rule: Rule,
        end_last: bool,
        pats: &'static [Pat],
        p: usize,
    ) -> Option<(Pat, usize, usize)> {
        if !end_last && let Some(hit) = &self.end_at(rule, p) {
            let (start, end) = self.record(hit);
            return Some((Pat::End, start, end));
        }
        for &pat in pats {
            if let Some(hit) = &self.pat_at(pat, p) {
                let (start, end) = self.record(hit);
                return Some((pat, start, end));
            }
        }
        if end_last && let Some(hit) = &self.end_at(rule, p) {
            let (start, end) = self.record(hit);
            return Some((Pat::End, start, end));
        }
        None
    }
}

impl Lexer<'_> {
    /// The first match from the position.
    ///
    /// The halts apply only while the root rule alone is open.
    /// Where the line's first halt holds, the line ends before the patterns.
    /// Where its second holds and nothing matched, the line ends after them.
    ///
    /// A rule the depth limit keeps closed leaves its text plain.
    /// An empty match would match again at the same place, so the search
    /// then goes on one character later.
    /// A scan for GraphQL that nothing embeds compiles without the halts.
    fn search<const HALTING: bool>(&mut self, halts: &Halts<'_>) -> Option<(usize, Pat, usize)> {
        let rule = self.top_rule();
        let end_last = Lexer::end_last(rule);
        let pats = patterns(rule);
        let end = self.lim() + usize::from(self.nl);
        let mut p = self.pos;
        'search: while p <= end {
            self.spend(1);
            if self.over_budget() {
                return None;
            }
            let boundary = p >= self.lim() || !is_continuation(self.s[p]);
            if HALTING && boundary && halts.0.holds(self.s, p) {
                self.halt_here(p);
                return None;
            }
            if boundary && let Some((pat, start, hit_end)) = self.first_at(rule, end_last, pats, p)
            {
                if pat.opens().is_some() && self.at_depth_limit() {
                    self.record_depth();
                    let next = if hit_end > start {
                        hit_end
                    } else {
                        start + self.s.get(start).map_or(1, |&lead| char_len(lead))
                    };
                    self.skip_plain(start, next);
                    p = self.pos;
                    continue 'search;
                }
                return Some((start, pat, hit_end));
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

impl Grammar for GraphQl {
    type Scope = S;
    const NO_SCOPES: Scopes<S> = S::NO_SCOPES;
    type Rule = Rule;
    type Pat = Pat;
    type Nested = Nested;
    type Extra = Extra;
    type Kept = ();

    const NAME: &'static str = "GraphQL";
    const ROOT: Rule = Rule::Root;
    const SOURCE: crate::scope::Scope = S::Source.scope();
    /// A string's quote carries the string's scope.
    /// Inside it, the quote carries its own.
    const NESTED_SPANS: bool = true;
    const EMBEDS: bool = true;

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

    fn rule_content(rule: Rule) -> Scopes<S> {
        rule.content()
    }

    fn rule_payload(rule: Rule) -> Payload {
        if rule == Rule::Interpolation {
            Payload::Embed(Language::JavaScript)
        } else {
            Payload::None
        }
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

    /// The first match from the position: see `search`.
    fn scan(lexer: &mut Lexer<'_>) -> Option<(usize, Pat, usize)> {
        match lexer.root_halts() {
            Some(halts) => lexer.search::<true>(halts),
            None => lexer.search::<false>(&NO_HALTS),
        }
    }

    fn matcher(lexer: &mut Lexer<'_>, pat: Pat, p: usize) -> Option<usize> {
        let hit = lexer.pat_at(pat, p)?;
        Some(lexer.record(&hit).1)
    }

    fn tokenize_nested(_: &mut Lexer<'_>, kind: Nested, _: usize, _: usize, _: u32) {
        match kind {}
    }
}

impl Embeds for GraphQl {
    type End = FnHalt;
    const HALTS: bool = true;

    /// An interpolation ends at its `}`, and nothing comes after the
    /// embedded grammar.
    fn embed_halts(_: &Embedding, _: &Arena) -> (FnHalt, Option<FnHalt>) {
        (closing_brace, None)
    }
}

/// The end of an interpolation: a `}`.
/// The JavaScript lexer tests it only while none of its own contexts waits
/// for one.
fn closing_brace(bytes: &[u8], p: usize) -> bool {
    bytes.get(p) == Some(&b'}')
}
