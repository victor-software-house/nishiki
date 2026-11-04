//! Makefiles on the shared rule engine ([`crate::rules`]).
//! The matchers answer with a [`Hit`].
//! The glue here turns each hit into the spans the engine emits.

use super::rules::{Pat, patterns};
use super::scope::S;
use super::state::Rule;
use crate::language::{Halts, NO_HALTS};
use crate::rules::{self, Act, Captures, DynName, Grammar, ScopeName};
use crate::utf8::char_len;

/// The Makefile lexer for one line.
pub(super) type Lexer<'a> = rules::Lexer<'a, Make>;
/// What a highlighter keeps from line to line.
pub(crate) type Scratch = rules::Scratch<Make>;

/// The `make` grammar that Shiki bundles.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Make;

/// The capture groups the grammar tokenizes with patterns of their own: none.
/// A span lexed again names its rule.
#[derive(Clone, Copy, Debug)]
pub(crate) enum Nested {}

/// What the Makefile matchers remember about a line.
#[derive(Debug, Default)]
pub(crate) struct Extra {
    /// Where the last match started.
    start: usize,
}

/// The scope a capture group adds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum CapName {
    /// The group has no scope of its own.
    /// It is lexed again.
    None,
    One(S),
    /// `keyword.control.` and the flags of a recipe line.
    /// The group holds the flags.
    Flags,
}

/// A capture group of a match that carries a scope or is lexed again.
#[derive(Clone, Copy, Debug)]
pub(super) struct Cap {
    pub(super) from: usize,
    pub(super) to: usize,
    pub(super) name: CapName,
    pub(super) again: Option<Rule>,
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
        self.caps[slot] = Some(Cap {
            from,
            to,
            name: CapName::One(scope),
            again: None,
        });
        self
    }

    /// The match with a group `from..to` in slot `slot` and the way it is
    /// scoped or lexed again.
    pub(super) fn group(
        mut self,
        slot: usize,
        from: usize,
        to: usize,
        name: CapName,
        again: Option<Rule>,
    ) -> Hit {
        self.caps[slot] = Some(Cap {
            from,
            to,
            name,
            again,
        });
        self
    }
}

impl Lexer<'_> {
    /// Tries `pat` at `p`.
    /// On a match, it records the capture groups as spans and returns where
    /// the match ends.
    #[expect(
        clippy::inline_always,
        reason = "measured: the matchers are the hot loop"
    )]
    #[inline(always)]
    fn hit(&mut self, pat: Pat, p: usize) -> Option<usize> {
        let hit = if pat == Pat::End {
            self.end_at(self.top_rule(), p)?
        } else {
            self.pat_at(pat, p)?
        };
        self.x.start = hit.start;
        self.clear_spans();
        for cap in hit.caps.iter().flatten() {
            if cap.from >= cap.to {
                continue;
            }
            if cap.name == CapName::Flags && cap.again.is_none() {
                self.span_dyn(
                    cap.from,
                    cap.to,
                    &[],
                    DynName {
                        prefix: "keyword.control.",
                        start: cap.from,
                        end: cap.to,
                        suffix: ".makefile",
                    },
                );
            } else {
                let one = match cap.name {
                    CapName::One(scope) => Some(scope),
                    CapName::None | CapName::Flags => None,
                };
                self.span_one(cap.from, cap.to, one, cap.again);
            }
        }
        Some(hit.end)
    }
}

impl Grammar for Make {
    type Scope = S;
    type Rule = Rule;
    type Pat = Pat;
    type Nested = Nested;
    type Extra = Extra;
    type Kept = ();

    const NAME: &'static str = "Makefile";
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

    fn rule_content(rule: Rule) -> &'static [S] {
        rule.content()
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

    /// The first match from the position (see [`scan`]).
    fn scan(lexer: &mut Lexer<'_>) -> Option<(usize, Pat, usize)> {
        match lexer.root_halts() {
            Some(halts) => scan::<true>(lexer, halts),
            None => scan::<false>(lexer, &NO_HALTS),
        }
    }

    fn matcher(lexer: &mut Lexer<'_>, pat: Pat, p: usize) -> Option<usize> {
        lexer.hit(pat, p)
    }

    fn tokenize_nested(_: &mut Lexer<'_>, kind: Nested, _: usize, _: usize, _: u32) {
        match kind {}
    }
}

/// The first match from the position.
/// At each position the end pattern of the innermost rule comes first, then
/// the rule's patterns in order.
/// A rule the depth limit keeps closed leaves its text plain.
/// That rule matched nothing.
/// It would match again at the same place.
/// The search goes on one character later instead.
/// The halt test is compiled in only when `HALTING`.
/// A Makefile that no other grammar embeds pays nothing for it.
fn scan<const HALTING: bool>(
    lexer: &mut Lexer<'_>,
    halts: &Halts<'_>,
) -> Option<(usize, Pat, usize)> {
    let rule = lexer.top_rule();
    let end = lexer.lim() + usize::from(lexer.nl);
    let mut p = lexer.pos;
    'search: while p <= end {
        if HALTING && halts.0.holds(lexer.s, p) {
            lexer.halt_here(p);
            return None;
        }
        lexer.spend(1);
        if lexer.over_budget() {
            return None;
        }
        if p >= lexer.lim() || lexer.s[p] & 0xc0 != 0x80 {
            if let Some(end) = lexer.hit(Pat::End, p) {
                return Some((lexer.x.start, Pat::End, end));
            }
            for &pat in patterns(rule) {
                let Some(hit_end) = lexer.hit(pat, p) else {
                    continue;
                };
                if pat.opens().is_some() && lexer.at_depth_limit() {
                    lexer.record_depth();
                    let start = lexer.x.start;
                    let next = if hit_end > start {
                        hit_end
                    } else {
                        start + lexer.s.get(start).map_or(1, |&lead| char_len(lead))
                    };
                    lexer.skip_plain(start, next);
                    lexer.spend(1);
                    if lexer.over_budget() {
                        return None;
                    }
                    p = lexer.pos;
                    continue 'search;
                }
                return Some((lexer.x.start, pat, hit_end));
            }
        }
        p += 1;
    }
    None
}
