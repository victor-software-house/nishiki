//! The fast path of a list of rules: one automaton that finds where any of
//! them can start in a line.
//!
//! Each rule's pattern is widened to a plain expression (see `regex::relax`).
//! The patterns are compiled together into one `regex-automata` searcher.
//! The searcher reports the first position where some pattern may match.
//! The exact machine then tries the patterns that can start with the byte
//! there.
//!
//! Two kinds of pattern stay out of the searcher and are searched alone:
//!
//! - a pattern that depends on the document;
//! - a pattern that matches nothing as a plain expression.

use std::sync::Mutex;

use regex_automata::meta::{self, Cache};
use regex_automata::util::prefilter::Prefilter;
use regex_automata::{Input, MatchKind};
use regex_syntax::hir::literal::{ExtractKind, Extractor, Seq};
use regex_syntax::hir::{Hir, HirKind};

use super::grammar::Cells;
use super::regex::Sketch;

/// What the automaton covers of one list of rules.
#[derive(Debug)]
pub(crate) struct Scanner {
    /// The list's rules, in the order of the grammar.
    pub(crate) rules: &'static [usize],
    meta: Option<meta::Regex>,
    /// The position in the list of each pattern of the automaton.
    set: Vec<usize>,
    /// The positions in the list searched one by one.
    pub(crate) residual: Vec<usize>,
    /// Per byte, the numbers of the automaton's patterns that can start with
    /// it, ascending.
    starts: Vec<Vec<usize>>,
    /// Caches that highlighters have finished with.
    /// The next highlighter starts from their lazily built states.
    /// Each cache is boxed.
    /// A cache is about a kilobyte, and a highlighter keeps a slot for one
    /// cache per list.
    ///
    /// The move makes no copy and no allocation.
    #[expect(
        clippy::vec_box,
        reason = "a cache moves between this pool and a highlighter's slots as a box"
    )]
    spare: Mutex<Vec<Box<Cache>>>,
}

/// The most spare caches a scanner keeps.
const SPARE: usize = 4;

/// The most literals the prefilter's extraction keeps.
const LITERALS: usize = 64;

/// How a list's automaton gets its prefilter.
enum Prefixes {
    /// The patterns are literals, or their prefixes are exact.
    /// `regex-automata` then searches for them without a regex engine.
    /// Only its own extraction leads there, and it is cheap on literals.
    Literals,
    /// A prefilter nishiki extracted, or `None` for no prefilter.
    Extracted(Option<Prefilter>),
}

/// The widened patterns as one automaton, or the first error.
///
/// A list of literals keeps `regex-automata`'s own extraction.
/// Every other list takes a prefilter nishiki extracts, with at most
/// [`LITERALS`] literals.
/// `regex-automata`'s own extraction keeps up to 250 literals.
/// On CMake's first token that extraction cost 29 million instructions.
/// The lazy DFA's cache holds at most `dfa_cache` bytes, the grammar's
/// [`Grammar::dfa_cache`](super::grammar::Grammar::dfa_cache).
fn compile(hirs: &[&Hir], dfa_cache: usize) -> Result<meta::Regex, Box<meta::BuildError>> {
    let config = meta::Config::new()
        .match_kind(MatchKind::LeftmostFirst)
        .nfa_size_limit(Some(1 << 24))
        .hybrid_cache_capacity(dfa_cache);
    let config = match prefixes(hirs) {
        Prefixes::Literals => config,
        Prefixes::Extracted(prefilter) => config.auto_prefilter(false).prefilter(prefilter),
    };
    meta::Regex::builder()
        .configure(config)
        .build_many_from_hir(hirs)
        .map_err(Box::new)
}

/// The prefixes of `hirs`, at most [`LITERALS`] of them.
fn prefixes(hirs: &[&Hir]) -> Prefixes {
    if hirs.iter().all(|hir| literals(hir)) {
        return Prefixes::Literals;
    }
    let mut extractor = Extractor::new();
    extractor.kind(ExtractKind::Prefix).limit_total(LITERALS);
    let mut prefixes = Seq::empty();
    for hir in hirs {
        prefixes.union(&mut extractor.extract(hir));
    }
    if prefixes.is_exact() {
        return Prefixes::Literals;
    }
    prefixes.optimize_for_prefix_by_preference();
    Prefixes::Extracted(
        prefixes
            .literals()
            .and_then(|literals| Prefilter::new(MatchKind::LeftmostFirst, literals)),
    )
}

/// Whether `hir` is a literal or an alternation of literals.
fn literals(hir: &Hir) -> bool {
    match hir.kind() {
        HirKind::Literal(_) => true,
        HirKind::Alternation(alts) => alts
            .iter()
            .all(|alt| matches!(alt.kind(), HirKind::Literal(_))),
        HirKind::Empty
        | HirKind::Class(_)
        | HirKind::Look(_)
        | HirKind::Repetition(_)
        | HirKind::Capture(_)
        | HirKind::Concat(_) => false,
    }
}

impl Scanner {
    /// The scanner of the list `rules` of the grammar of `cells`, from the
    /// patterns in them.
    ///
    /// The build reads each pattern's sketch.
    /// It compiles no pattern's program.
    /// A pattern the engine cannot parse never matches, so it is left out.
    /// When the automaton refuses the patterns together, each is tried alone.
    /// The ones it refuses are searched alone.
    pub(crate) fn build(cells: &Cells, rules: &'static [usize]) -> Scanner {
        let mut widened: Vec<(usize, &Hir, &Sketch)> = Vec::new();
        let mut residual = Vec::new();
        for (position, &rule) in rules.iter().enumerate() {
            let Some(begin) = cells.grammar.rules.get(rule).map(|rule| rule.begin) else {
                continue;
            };
            let Some(sketch) = cells.sketch(begin) else {
                continue;
            };
            match sketch.relaxed() {
                Some(relaxed) => widened.push((position, relaxed, sketch)),
                None => residual.push(position),
            }
        }
        let dfa_cache = cells.grammar.dfa_cache;
        let mut meta = compile(&widened.iter().map(|w| w.1).collect::<Vec<_>>(), dfa_cache);
        if meta.is_err() {
            widened.retain(|&(position, relaxed, _)| {
                let fits = compile(&[relaxed], dfa_cache).is_ok();
                if !fits {
                    residual.push(position);
                }
                fits
            });
            residual.sort_unstable();
            meta = compile(&widened.iter().map(|w| w.1).collect::<Vec<_>>(), dfa_cache);
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
            rules,
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
    pub(crate) fn take_cache(&self) -> Option<Box<Cache>> {
        let meta = self.meta.as_ref()?;
        let spare = self.spare.lock().ok().and_then(|mut spare| spare.pop());
        Some(spare.unwrap_or_else(|| Box::new(meta.create_cache())))
    }

    /// Keeps `cache` for the next highlighter.
    pub(crate) fn give_cache(&self, cache: Box<Cache>) {
        if let Ok(mut spare) = self.spare.lock()
            && spare.len() < SPARE
        {
            spare.push(cache);
        }
    }

    /// Where a pattern of the automaton may first match in `text`, at or after
    /// `from`.
    /// Returns the position, the number of the first such pattern, and where
    /// its widened match ends.
    /// Returns `None` when no pattern may match.
    ///
    /// Inlined across codegen units.
    /// At 16 units it stayed a call from the tokenizer.
    #[inline]
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
