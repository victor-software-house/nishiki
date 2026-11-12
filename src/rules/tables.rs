//! For each rule, the patterns that can match at a byte, in the order the
//! grammar tries them.
//!
//! The engine tries only those where a byte stands.

#[cfg(any(nishiki_unstable, feature = "_blank-runs-to-last"))]
use std::ops::ControlFlow;

use super::{Act, BlankRuns, Lexer, RuleDefinition, Start, Tabled, WideSpaceLookbehind};
use crate::fnv::FnvMap;
use crate::language::Halt;

/// Where a position lies, for the patterns that need a word boundary or the
/// start of the line.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Place {
    /// Anywhere else.
    Away,
    /// The start of the line.
    LineStart,
    /// Between two ASCII word characters (`is_word_byte`), past the first
    /// position the scan tries.
    ///
    /// A pattern is left out here in two cases:
    ///
    /// - it cannot match inside a word;
    /// - a match inside a word could also start at the word's start, which the
    ///   scan reached first.
    ///
    /// The first position has no such start behind it.
    /// So it counts as [`Place::Away`] in a grammar with a pattern that is
    /// left out here only past that position ([`Tabled::WORDS_AT_START`]).
    ///
    /// Away holds every pattern this place holds.
    /// Only a `^` leaves a pattern out of Away.
    /// `oracle/rule-grammar.ts` fails on a start that lists this place without
    /// Away.
    InWord,
}

impl Place {
    const fn index(self) -> usize {
        match self {
            Place::Away => 0,
            Place::LineStart => 1,
            Place::InWord => 2,
        }
    }
}

/// A set of [`Place`]s: where in the line a match of a pattern can start.
#[derive(Clone, Copy, Debug)]
pub(crate) struct PlaceSet(u8);

impl PlaceSet {
    /// The set of `places`.
    pub(crate) const fn of(mut places: &[Place]) -> Self {
        let mut bits = 0;
        while let [place, rest @ ..] = places {
            bits |= 1 << place.index();
            places = rest;
        }
        PlaceSet(bits)
    }

    /// Whether the set holds `place`.
    fn contains(self, place: Place) -> bool {
        self.0 >> place.index() & 1 == 1
    }
}

/// The candidate patterns of one rule.
pub(crate) struct Tables<G: Tabled> {
    /// The patterns that can match where no byte stands, the end pattern
    /// first.
    /// No byte stands after the line feed, or at the end of a last line
    /// without one ([`Start::matches_at_end`]).
    all: Box<[G::Pattern]>,
    /// The candidates for each byte, in three sets of 256:
    ///
    /// - away from the start of the line, the first 256;
    /// - at the start of the line, the next 256;
    /// - inside a word, the next 256.
    ///
    /// Bytes with the same candidates share one list.
    ///
    /// A rule of a [`BlankRuns::ToLast`] grammar can have one more list,
    /// after those three sets.
    /// It holds the patterns that take exactly one blank
    /// ([`Grammar::takes_one_blank`](super::Grammar::takes_one_blank)).
    /// The scan tries them at the last blank of a run.
    ///
    /// The list is there only when it is not empty.
    /// So the slice's length marks whether the scan tries the last blank.
    /// It sits in this slice so that every grammar's tables keep one size.
    by_byte: Box<[&'static [G::Pattern]]>,
}

impl<G: Tabled> Tables<G> {
    /// The tables of the rule that `definition` defines.
    ///
    /// Each pattern's start is read once.
    /// Each distinct list of candidates is stored once, in one allocation
    /// that lives as long as the program.
    /// The one-blank list of a [`BlankRuns::ToLast`] grammar is a second such
    /// allocation.
    /// The tables are built into statics that are never dropped, so both are
    /// leaked rather than owned.
    pub(crate) fn build(definition: RuleDefinition<G>) -> Self {
        let all: Vec<G::Pattern> = definition
            .end
            .into_iter()
            .chain(definition.patterns.iter().copied())
            .collect();
        let starts: Vec<Start> = all
            .iter()
            .map(|&pattern| G::pattern_start(pattern))
            .collect();
        let (picks, spans) = candidate_lists(&starts);
        let lists: Vec<G::Pattern> = picks
            .iter()
            .filter_map(|&index| all.get(index).copied())
            .collect();
        let lists: &'static [G::Pattern] = Box::leak(lists.into_boxed_slice());
        #[cfg_attr(
            not(any(nishiki_unstable, feature = "_blank-runs-to-last")),
            expect(unused_mut, reason = "only a `BlankRuns::ToLast` grammar adds a list")
        )]
        let mut by_byte: Vec<&'static [G::Pattern]> = spans
            .into_iter()
            .map(|(start, len)| lists.get(start..start + len).unwrap_or_default())
            .collect();
        #[cfg(any(nishiki_unstable, feature = "_blank-runs-to-last"))]
        if matches!(G::BLANK_RUNS, BlankRuns::ToLast) {
            let one_blank: Vec<G::Pattern> = all
                .iter()
                .copied()
                .filter(|&pattern| G::takes_one_blank(pattern))
                .collect();
            if !one_blank.is_empty() {
                by_byte.push(Box::leak(one_blank.into_boxed_slice()));
            }
        }
        Tables {
            all: all
                .iter()
                .zip(&starts)
                .filter(|(_, start)| start.matches_at_end)
                .map(|(&pattern, _)| pattern)
                .collect(),
            by_byte: by_byte.into_boxed_slice(),
        }
    }

    /// The patterns that can match at a position where byte `byte` stands.
    pub(crate) fn candidates(&self, place: Place, byte: u8) -> &[G::Pattern] {
        self.by_byte[place.index() * 256 + usize::from(byte)]
    }

    /// The patterns that can match where no byte stands, the end pattern
    /// first.
    pub(crate) fn all(&self) -> &[G::Pattern] {
        &self.all
    }

    /// The patterns tried at the last blank of a run, the end pattern first
    /// when it is one of them.
    /// Empty for a grammar that does not use [`BlankRuns::ToLast`].
    #[cfg(any(nishiki_unstable, feature = "_blank-runs-to-last"))]
    pub(crate) fn one_blank(&self) -> &[G::Pattern] {
        self.by_byte.get(3 * 256).copied().unwrap_or_default()
    }

    /// Whether the rule has patterns to try at the last blank of a run.
    /// It reads only the length of the byte table.
    #[cfg(any(nishiki_unstable, feature = "_blank-runs-to-last"))]
    pub(crate) fn tries_final_blank(&self) -> bool {
        self.by_byte.len() > 3 * 256
    }
}

/// For patterns with the starts `starts`, the positions of the candidates
/// of every place and byte.
///
/// It returns each distinct list once, and where each place and byte's list
/// lies among them.
/// It works on positions, so it is built once for every grammar.
///
/// Each place and byte gets a bit set of patterns, one bit per position.
/// A pattern sets its bit only at the bytes its start allows.
/// So the work follows the starts, not 768 times the patterns.
///
/// Equal sets share one list, which lists the positions in increasing order.
/// So the lists come out in the order a filter over the patterns gives.
fn candidate_lists(starts: &[Start]) -> (Vec<usize>, Vec<(usize, usize)>) {
    let words = starts.len().div_ceil(64).max(1);
    let mut sets = vec![0_u64; 3 * 256 * words];
    for (index, start) in starts.iter().enumerate() {
        let (word, bit) = (index / 64, 1_u64 << (index % 64));
        for place in [Place::Away, Place::LineStart, Place::InWord] {
            if !start.places.contains(place) {
                continue;
            }
            for byte in set_bits(&start.bytes) {
                if let Some(set) = sets.get_mut((place.index() * 256 + byte) * words + word) {
                    *set |= bit;
                }
            }
        }
    }
    let mut lists = Vec::new();
    let mut seen: FnvMap<&[u64], (usize, usize)> = FnvMap::default();
    let spans = sets
        .chunks_exact(words)
        .map(|set| {
            *seen.entry(set).or_insert_with(|| {
                let first = lists.len();
                lists.extend(set_bits(set));
                (first, lists.len() - first)
            })
        })
        .collect();
    (lists, spans)
}

/// The positions of the set bits of `words`, bit `i` of word `w` being
/// position `64 * w + i`, in increasing order.
fn set_bits(words: &[u64]) -> impl Iterator<Item = usize> + '_ {
    words.iter().enumerate().flat_map(|(word, &bits)| {
        std::iter::successors(Some(bits), |&left| Some(left & left.wrapping_sub(1)))
            .take_while(|&left| left != 0)
            .map(move |left| word * 64 + usize::try_from(left.trailing_zeros()).unwrap_or(0))
    })
}

/// The first pattern of the innermost rule that matches at or after the
/// cursor.
///
/// At each position it tries only the patterns whose start allows the byte
/// there.
///
/// The text a pattern sees ends after the line feed, where `$` and
/// lookaheads can still match.
/// The search stops at a halt ([`Lexer::root_halts`]) in two cases:
///
/// - a match starting there would lose to the halt;
/// - no pattern matches there and the halt after them holds.
///
/// A begin pattern of a rule the depth limit keeps closed is tried only to
/// record the limit, and its text stays plain.
///
/// A line whose document has root patterns for `$base` slots scans with the
/// halt tests ([`scan_tables`]).
/// Below the root, its halt never holds.
///
/// A pattern that fails at a blank fails at the blanks after it.
/// The blanks are consumed first and what follows decides.
/// No lookbehind or boundary holds between two of them.
/// So the search skips a run of blanks at once.
///
/// A pattern that takes one blank and then a character that is not a blank
/// is the exception.
/// A [`BlankRuns::ToLast`] grammar tries those patterns at the run's last
/// blank.
/// It steps over a single blank by one byte, since that blank is the run's
/// last and has been tried.
pub(crate) fn table_scan<G: Tabled + WideSpaceLookbehind>(
    lexer: &mut Lexer<'_, G>,
) -> Option<(usize, G::Pattern, usize)> {
    match lexer.root_halts() {
        Some(&(end, after)) => scan::<G, true>(lexer, end, after),
        None if G::DEPTH_HALTS
            && let Some((deep, loses_ties)) = lexer.deep_halt() =>
        {
            if loses_ties {
                scan::<G, true>(lexer, crate::language::NO_HALTS.0, Some(deep))
            } else {
                scan::<G, true>(lexer, deep, None)
            }
        }
        None if G::BASE_SLOTS && lexer.base_root().is_some() => {
            scan::<G, true>(lexer, crate::language::NO_HALTS.0, None)
        }
        None => scan::<G, false>(lexer, &|_: &[u8], _: usize| false, None),
    }
}

/// The tables of the innermost rule, and where a `$base` slot holds.
///
/// A line that another document embeds scans a rule with a `$base` include
/// with the rule's base tables ([`Tabled::base_tables`]).
/// The slot then holds where the document's root patterns first match
/// ([`Halt::root`](crate::language::Halt::root)).
/// The scan stops there, as at a halt, unless a pattern listed before the slot
/// matches first.
/// Then the document goes on from there.
///
/// The root is asked once per scan, from the scan's start.
/// Its answer is the first match at or after that place.
/// The scan only moves forward and ends at the slot, so the answer holds for
/// the whole scan.
/// The next scan starts after its match and asks again.
///
/// A grammar with no slot compiles to its plain tables.
///
/// Only a halting scan has slots.
/// [`table_scan`] sends a line whose document has a root into one.
/// A document of the grammar itself then scans as a grammar with no slot.
#[expect(
    clippy::inline_always,
    reason = "a grammar with no slot must fold the tables and the place as before"
)]
#[inline(always)]
fn scan_tables<G: Tabled, const HALTING: bool>(
    lexer: &Lexer<'_, G>,
) -> (&'static Tables<G>, Option<usize>) {
    let rule = lexer.top_rule();
    if G::BASE_SLOTS
        && HALTING
        && let Some(root) = lexer.base_root()
        && let Some(tables) = G::base_tables(rule)
    {
        let anchor = lexer.g.then_some(lexer.pos);
        return (tables, root.first(lexer.s, lexer.nl, lexer.pos, anchor));
    }
    (G::tables(rule), None)
}

/// [`table_scan`], with the halt tests compiled in only when `HALTING`.
/// A line that no other language embeds pays nothing for them.
/// `end` is tested before the patterns at each place.
/// `after` is tested where none of them matched.
///
/// A grammar with [`Tabled::WORDS_AT_START`] takes the first position
/// apart, before the loop, and counts it as away from words when it lies
/// inside one.
/// So the loop has no test for its first position.
///
/// # The step as a macro
///
/// The local macro `step!` holds the loop's body.
/// The scan expands it once before the loop, only in such a grammar, and
/// once in the loop.
/// Every other grammar compiles the first copy out.
/// Its loop is the code it had before the first position was told apart.
///
/// Each other shape cost every grammar's loop or PowerShell's, measured at
/// one codegen unit on 2026-11-05:
///
/// | Shape | Cost |
/// |:--|:--|
/// | A test for the first position in the loop, against a position known only at run time | About 30 grammars ran 0.5% to 1.9% more instructions. |
/// | A flag on [`Tables`] | The struct grew from 32 to 40 bytes, and every grammar ran 0.1% to 0.2% more. |
/// | The first step as a function out of line, with a test before the loop | Go ran 1.1% more, and PowerShell 0.7% more under a per-grammar constant, about seven instructions per scan. |
#[expect(
    clippy::too_many_lines,
    reason = "it keeps the shape measured above; the gates on its blank-run arms add the lines"
)]
fn scan<G: Tabled + WideSpaceLookbehind, const HALTING: bool>(
    lexer: &mut Lexer<'_, G>,
    end: &dyn Halt,
    after: Option<&dyn Halt>,
) -> Option<(usize, G::Pattern, usize)> {
    let (tables, base_at) = scan_tables::<G, HALTING>(lexer);
    let full = lexer.at_depth_limit();
    let s = lexer.s;
    let lim = s.len();
    let nl = usize::from(lexer.nl);
    let mut p = lexer.pos;
    let mut visited = 0;
    macro_rules! step {
        ($scan:lifetime, $in_word:expr) => {
            if p > lim + nl {
                break $scan None;
            }
            if HALTING && end.holds(s, p) {
                lexer.halt_here(p);
                break $scan None;
            }
            visited += 1;
            if visited % 64 == 0 && lexer.over_budget() {
                break $scan None;
            }
            let list = match s.get(p) {
                Some(&byte) => {
                    let place = match place_of(s, p, byte) {
                        Place::InWord => $in_word,
                        place @ (Place::Away | Place::LineStart) => place,
                    };
                    tables.candidates(place, byte)
                }
                None if p == lim && nl == 1 => {
                    let place = if p == 0 {
                        Place::LineStart
                    } else {
                        Place::Away
                    };
                    tables.candidates(place, b'\n')
                }
                None => tables.all(),
            };
            let mut hit = None;
            for &pattern in list {
                if G::BASE_SLOTS && HALTING && G::is_base_slot(pattern) {
                    if base_at == Some(p) {
                        lexer.halt_here(p);
                        break $scan None;
                    }
                    continue;
                }
                if full && matches!(G::pattern_act(pattern), Act::Begin(_)) {
                    if lexer.try_pattern(pattern, p).is_some() {
                        lexer.record_depth();
                    }
                    continue;
                }
                if let Some(end) = lexer.try_pattern(pattern, p) {
                    hit = Some((p, pattern, end));
                    break;
                }
            }
            if hit.is_some() {
                break $scan hit;
            }
            if HALTING && after.is_some_and(|after| after.holds(s, p)) {
                lexer.halt_here(p);
                break $scan None;
            }
            p = match s.get(p) {
                Some(b' ' | b'\t') => match G::BLANK_RUNS {
                    BlankRuns::Skip => lexer.skip(p, |b| matches!(b, b' ' | b'\t')),
                    #[cfg(any(nishiki_unstable, feature = "_blank-runs-to-last"))]
                    BlankRuns::ToLast => {
                        if matches!(s.get(p + 1), Some(b' ' | b'\t')) {
                            let run_end = lexer.skip(p, |b| matches!(b, b' ' | b'\t'));
                            if tables.tries_final_blank()
                                && let ControlFlow::Break(found) = at_final_blank::<G, HALTING>(
                                    lexer,
                                    tables,
                                    end,
                                    run_end - 1,
                                    full,
                                )
                            {
                                break $scan found;
                            }
                            run_end
                        } else {
                            p + 1
                        }
                    }
                    #[cfg(any(nishiki_unstable, feature = "lang-objective-c"))]
                    BlankRuns::Visit if G::SKIPS_PLACES => G::next_place(lexer, p),
                    #[cfg(any(nishiki_unstable, feature = "lang-objective-c"))]
                    BlankRuns::Visit => lexer.char_end(p),
                },
                Some(&byte @ (0x0b | 0x0c | b'\r'))
                    if matches!(G::BLANK_RUNS, BlankRuns::Skip) && !G::SKIPS_PLACES =>
                {
                    lexer.skip(p, |b| b == byte)
                }
                Some(_) if G::SKIPS_PLACES => G::next_place(lexer, p),
                Some(_) if matches!(G::BLANK_RUNS, BlankRuns::Skip) => lexer.pass_char(
                    p,
                    G::MIXED_BLANK_RUNS && !HALTING,
                    G::LOOKS_BEHIND_AT_A_WIDE_SPACE,
                ),
                Some(_) => lexer.char_end(p),
                None => p + 1,
            };
        };
    }
    let found = 'scan: {
        if G::WORDS_AT_START {
            step!('scan, Place::Away);
        }
        loop {
            step!('scan, Place::InWord);
        }
    };
    lexer.spend(visited);
    found
}

/// The scan's step at `p`, the last blank of a run, for a
/// [`BlankRuns::ToLast`] grammar.
///
/// It breaks with `None` when the halt `end` holds at `p`.
/// It breaks with the first one-blank pattern that matches at `p`.
/// When neither happens, it continues.
/// A begin pattern of a rule the depth limit keeps closed is tried only to
/// record the limit, as in the scan.
///
/// It stays out of line.
/// Inlined, its values stayed live across the scan's loop, and the loop
/// reloaded spilled registers at every byte.
#[inline(never)]
#[cfg(any(nishiki_unstable, feature = "_blank-runs-to-last"))]
fn at_final_blank<G: Tabled, const HALTING: bool>(
    lexer: &mut Lexer<'_, G>,
    tables: &Tables<G>,
    end: &dyn Halt,
    p: usize,
    full: bool,
) -> ControlFlow<Option<(usize, G::Pattern, usize)>> {
    if HALTING && end.holds(lexer.s, p) {
        lexer.halt_here(p);
        return ControlFlow::Break(None);
    }
    for &pattern in tables.one_blank() {
        if full && matches!(G::pattern_act(pattern), Act::Begin(_)) {
            if lexer.try_pattern(pattern, p).is_some() {
                lexer.record_depth();
            }
            continue;
        }
        if let Some(found) = lexer.try_pattern(pattern, p) {
            return ControlFlow::Break(Some((p, pattern, found)));
        }
    }
    ControlFlow::Continue(())
}

/// The place of position `p` of `s`, where byte `byte` stands.
#[inline]
fn place_of(s: &[u8], p: usize, byte: u8) -> Place {
    if p == 0 {
        Place::LineStart
    } else if is_word_byte(byte) && is_word_byte(s[p - 1]) {
        Place::InWord
    } else {
        Place::Away
    }
}

/// Whether `byte` is an ASCII word character.
fn is_word_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every place.
    const ANYWHERE: &[Place] = &[Place::Away, Place::LineStart, Place::InWord];

    /// A start that allows the bytes `bytes` at the places `places`.
    fn start(bytes: &[u8], places: &[Place]) -> Start {
        let mut set = [0_u64; 4];
        for &byte in bytes {
            set[usize::from(byte >> 6)] |= 1 << (byte & 63);
        }
        Start {
            bytes: set,
            places: PlaceSet::of(places),
            matches_at_end: false,
        }
    }

    /// The list of `place` and `byte`.
    fn list(tables: &(Vec<usize>, Vec<(usize, usize)>), place: Place, byte: u8) -> &[usize] {
        let (lists, spans) = tables;
        let (first, len) = spans[place.index() * 256 + usize::from(byte)];
        &lists[first..first + len]
    }

    #[test]
    fn each_place_and_byte_lists_the_patterns_its_starts_allow_in_order() {
        let starts = [
            start(b"ab", &[Place::Away, Place::LineStart]),
            start(b"a", &[Place::LineStart]),
            start(b"bc", ANYWHERE),
        ];
        let tables = candidate_lists(&starts);
        assert_eq!(list(&tables, Place::Away, b'a'), [0]);
        assert_eq!(list(&tables, Place::LineStart, b'a'), [0, 1]);
        assert_eq!(list(&tables, Place::InWord, b'a'), [0_usize; 0]);
        assert_eq!(list(&tables, Place::Away, b'b'), [0, 2]);
        assert_eq!(list(&tables, Place::InWord, b'b'), [2]);
        assert_eq!(list(&tables, Place::InWord, b'c'), [2]);
        assert_eq!(list(&tables, Place::Away, b'z'), [0_usize; 0]);
    }

    #[test]
    fn a_position_between_two_word_bytes_is_inside_a_word() {
        let s = b"ab cd";
        assert_eq!(place_of(s, 0, b'a'), Place::LineStart);
        assert_eq!(place_of(s, 1, b'b'), Place::InWord);
        assert_eq!(place_of(s, 3, b'c'), Place::Away);
        assert_eq!(place_of(s, 4, b'd'), Place::InWord);
    }

    #[test]
    fn equal_lists_are_stored_once() {
        let starts = [start(b"xy", ANYWHERE)];
        let (lists, spans) = candidate_lists(&starts);
        assert_eq!(lists, [0]);
        assert_eq!(spans[usize::from(b'x')], spans[usize::from(b'y')]);
    }

    #[test]
    fn a_pattern_that_needs_a_byte_before_it_is_left_out_at_the_line_start() {
        let starts = [
            start(b"a", &[Place::Away, Place::InWord]),
            start(b"a", ANYWHERE),
        ];
        let tables = candidate_lists(&starts);
        assert_eq!(list(&tables, Place::LineStart, b'a'), [1]);
        assert_eq!(list(&tables, Place::Away, b'a'), [0, 1]);
        assert_eq!(list(&tables, Place::InWord, b'a'), [0, 1]);
    }

    #[test]
    fn a_pattern_past_the_first_word_of_the_sets_keeps_its_position() {
        let mut starts: Vec<Start> = (0..70).map(|_| start(b"q", ANYWHERE)).collect();
        starts.push(start(b"r", ANYWHERE));
        let tables = candidate_lists(&starts);
        assert_eq!(list(&tables, Place::Away, b'r'), [70]);
        assert_eq!(list(&tables, Place::Away, b'q').len(), 70);
    }
}
