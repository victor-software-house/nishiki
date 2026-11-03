//! The fast path of a list of rules: one automaton that finds where any of
//! them can start in a line.
//!
//! Each rule's pattern is widened to a plain expression (see `regex::relax`)
//! and the lot is compiled into one `regex-automata` searcher. It reports the
//! first position where some pattern may match, and the exact machine then
//! tries the patterns that can start with the byte there. A pattern that
//! depends on the document, or that matches nothing as a plain expression,
//! stays out of it and is searched alone.

use std::sync::Mutex;

use regex_automata::meta::{self, Cache};
use regex_automata::{Input, MatchKind};

use super::grammar::Grammar;
use super::regex::Regex;

/// What the automaton covers of one list of rules.
#[derive(Debug)]
pub(crate) struct Scanner {
    meta: Option<meta::Regex>,
    /// The position in the list of each pattern of the automaton.
    set: Vec<usize>,
    /// The positions in the list searched one by one.
    pub(crate) residual: Vec<usize>,
    /// Per byte, the numbers of the automaton's patterns that can start with
    /// it, ascending.
    starts: Vec<Vec<usize>>,
    /// Caches that highlighters have finished with, whose lazily built states
    /// the next highlighter starts from.
    spare: Mutex<Vec<Cache>>,
}

/// The most spare caches a scanner keeps.
const SPARE: usize = 4;

/// The widened patterns as one automaton, or the first error.
fn compile(sources: &[&str]) -> Result<meta::Regex, Box<meta::BuildError>> {
    meta::Regex::builder()
        .configure(
            meta::Config::new()
                .match_kind(MatchKind::LeftmostFirst)
                .nfa_size_limit(Some(1 << 24))
                .hybrid_cache_capacity(1 << 22),
        )
        .build_many(sources)
        .map_err(Box::new)
}

impl Scanner {
    /// The scanner of the rules `list` of `grammar`.
    ///
    /// A pattern the engine cannot compile never matches, so it is left out;
    /// when the automaton refuses the patterns together, each is tried alone
    /// and the ones it refuses are searched alone.
    pub(crate) fn build(grammar: &Grammar, list: &[usize]) -> Scanner {
        let mut widened: Vec<(usize, &str, &Regex)> = Vec::new();
        let mut residual = Vec::new();
        for (position, &rule) in list.iter().enumerate() {
            let regex = grammar
                .rules
                .get(rule)
                .and_then(|rule| grammar.patterns.get(rule.begin))
                .and_then(|slot| slot.regex());
            let Some(regex) = regex else { continue };
            match regex.relaxed() {
                Some(source) => widened.push((position, source, regex)),
                None => residual.push(position),
            }
        }
        let mut meta = compile(&widened.iter().map(|w| w.1).collect::<Vec<_>>());
        if meta.is_err() {
            widened.retain(|&(position, source, _)| {
                let fits = compile(&[source]).is_ok();
                if !fits {
                    residual.push(position);
                }
                fits
            });
            residual.sort_unstable();
            meta = compile(&widened.iter().map(|w| w.1).collect::<Vec<_>>());
        }
        let meta = meta.ok().filter(|_| !widened.is_empty());
        if meta.is_none() {
            residual.extend(widened.iter().map(|w| w.0));
            residual.sort_unstable();
            widened.clear();
        }
        let starts = (0..=255_u8)
            .map(|byte| {
                widened
                    .iter()
                    .enumerate()
                    .filter(|(_, w)| w.2.can_start_with(byte))
                    .map(|(index, _)| index)
                    .collect()
            })
            .collect();
        Scanner {
            meta,
            set: widened.iter().map(|w| w.0).collect(),
            residual,
            starts,
            spare: Mutex::new(Vec::new()),
        }
    }

    /// A cache for [`Scanner::candidate`], when there is an automaton: a
    /// spare one, with the states earlier highlighters built, if there is
    /// one.
    pub(crate) fn take_cache(&self) -> Option<Cache> {
        let meta = self.meta.as_ref()?;
        let spare = self.spare.lock().ok().and_then(|mut spare| spare.pop());
        Some(spare.unwrap_or_else(|| meta.create_cache()))
    }

    /// Keeps `cache` for the next highlighter.
    pub(crate) fn give_cache(&self, cache: Cache) {
        if let Ok(mut spare) = self.spare.lock()
            && spare.len() < SPARE
        {
            spare.push(cache);
        }
    }

    /// The first position at or after `from` in `text` where a pattern of the
    /// automaton may match, the number of the first such pattern, and where
    /// its widened match ends; or `None`.
    pub(crate) fn candidate(
        &self,
        cache: &mut Cache,
        text: &[u8],
        from: usize,
    ) -> Option<(usize, usize, usize)> {
        let input = Input::new(text).span(from..text.len());
        let found = self.meta.as_ref()?.search_with(cache, &input)?;
        Some((found.start(), found.pattern().as_usize(), found.end()))
    }

    /// The positions in the list of the automaton's patterns, from number
    /// `first` on, that can start with `byte`.
    pub(crate) fn starting_with(&self, byte: u8, first: usize) -> impl Iterator<Item = usize> {
        let numbers = &self.starts[usize::from(byte)];
        let skip = numbers.partition_point(|&number| number < first);
        numbers[skip..].iter().map(|&number| self.set[number])
    }
}
