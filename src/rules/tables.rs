//! For each rule, the patterns that can match at a byte, in the order the
//! grammar tries them, so the engine tries only those where a byte stands.

use std::ops::ControlFlow;

use super::{Act, BlankRuns, Lexer, RuleData, Start, Tabled};
use crate::fnv::FnvMap;
use crate::language::Halt;

/// Where a position lies, for the patterns that need a word boundary or the
/// start of the line.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Place {
    /// Anywhere else.
    Away,
    /// The start of the line.
    LineStart,
    /// Between two ASCII word characters.
    InWord,
}

impl Place {
    fn index(self) -> usize {
        match self {
            Place::Away => 0,
            Place::LineStart => 1,
            Place::InWord => 2,
        }
    }
}

/// The candidate patterns of one rule.
pub(crate) struct Tables<G: Tabled> {
    /// The patterns that can match where no byte stands, after the line
    /// feed or at the end of a last line without one
    /// ([`Start::matches_at_end`]), the end pattern first.
    all: Box<[G::Pat]>,
    /// The candidates for each byte, away from the start of the line (the
    /// first 256), at it (the next 256) and inside a word (the next 256).
    /// Bytes with the same candidates share one list.
    ///
    /// A rule of a [`BlankRuns::ToLast`] grammar can have one more list,
    /// after those three sets.
    /// It holds the patterns that take exactly one blank
    /// ([`Grammar::takes_one_blank`](super::Grammar::takes_one_blank)), tried
    /// at the last blank of a run.
    /// The list is there only when it is not empty.
    /// So the slice's length says whether the scan tries the last blank.
    /// It sits in this slice so that every grammar's tables keep one size.
    by_byte: Box<[&'static [G::Pat]]>,
}

impl<G: Tabled> Tables<G> {
    /// The tables of the rule whose data is `data`.
    ///
    /// Each pattern's start is read once.
    /// Each distinct list of candidates is stored once, in one allocation
    /// that lives as long as the program.
    /// The one-blank list of a [`BlankRuns::ToLast`] grammar is a second such
    /// allocation.
    /// The tables are built into statics that are never dropped, so both are
    /// leaked rather than owned.
    pub(crate) fn build(data: RuleData<G>) -> Self {
        let all: Vec<G::Pat> = data
            .end
            .into_iter()
            .chain(data.patterns.iter().copied())
            .collect();
        let starts: Vec<Start> = all.iter().map(|&pat| G::pat_start(pat)).collect();
        let (picks, spans) = candidate_lists(&starts);
        let lists: Vec<G::Pat> = picks
            .iter()
            .filter_map(|&index| all.get(index).copied())
            .collect();
        let lists: &'static [G::Pat] = Box::leak(lists.into_boxed_slice());
        let mut by_byte: Vec<&'static [G::Pat]> = spans
            .into_iter()
            .map(|(start, len)| lists.get(start..start + len).unwrap_or_default())
            .collect();
        if matches!(G::BLANK_RUNS, BlankRuns::ToLast) {
            let one_blank: Vec<G::Pat> = all
                .iter()
                .copied()
                .filter(|&pat| G::takes_one_blank(pat))
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
                .map(|(&pat, _)| pat)
                .collect(),
            by_byte: by_byte.into_boxed_slice(),
        }
    }

    /// The patterns that can match at a position where byte `byte` stands.
    pub(crate) fn candidates(&self, place: Place, byte: u8) -> &[G::Pat] {
        self.by_byte[place.index() * 256 + usize::from(byte)]
    }

    /// The patterns that can match where no byte stands, the end pattern
    /// first.
    pub(crate) fn all(&self) -> &[G::Pat] {
        &self.all
    }

    /// The patterns tried at the last blank of a run, the end pattern first
    /// when it is one of them.
    /// Empty for a grammar that does not use [`BlankRuns::ToLast`].
    pub(crate) fn one_blank(&self) -> &[G::Pat] {
        self.by_byte.get(3 * 256).copied().unwrap_or_default()
    }

    /// Whether the rule has patterns to try at the last blank of a run.
    /// It reads only the length of the byte table.
    pub(crate) fn tries_final_blank(&self) -> bool {
        self.by_byte.len() > 3 * 256
    }
}

/// For patterns with the starts `starts`, the positions of the candidates
/// of every place and byte, each distinct list once, and where each place
/// and byte's list lies among them. It works on positions, so it is built
/// once for every grammar.
///
/// Each place and byte gets a bit set of patterns, one bit per position.
/// A pattern sets its bit only at the bytes its start allows, so the work
/// follows the starts, not 768 times the patterns.
/// Equal sets share one list, which lists the positions in increasing order.
/// So the lists come out in the order a filter over the patterns gives.
fn candidate_lists(starts: &[Start]) -> (Vec<usize>, Vec<(usize, usize)>) {
    let words = starts.len().div_ceil(64).max(1);
    let mut sets = vec![0_u64; 3 * 256 * words];
    for (index, start) in starts.iter().enumerate() {
        let (word, bit) = (index / 64, 1_u64 << (index % 64));
        for place in [Place::Away, Place::LineStart, Place::InWord] {
            if (place != Place::LineStart && start.bol) || (place == Place::InWord && start.wb) {
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
/// cursor, trying at each position only the patterns whose start allows the
/// byte there.
///
/// The text a pattern sees ends after the line feed, where `$` and
/// lookaheads can still match, and the search stops at a halt
/// ([`Lexer::root_halts`]), to which a match starting there would lose, or
/// where no pattern matches and the halt after them holds. A
/// begin pattern of a rule the depth limit keeps closed is tried only to
/// record the limit, and its text stays plain.
/// A pattern that fails at a blank fails at the blanks after it, since the
/// blanks are consumed first, what follows decides, and no lookbehind or
/// boundary holds between two of them; so the search skips a run of blanks
/// at once.
/// A pattern that takes one blank and then a character that is not a blank
/// is the exception.
/// A [`BlankRuns::ToLast`] grammar tries those patterns at the run's last
/// blank.
/// It steps over a single blank by one byte, since that blank is the run's
/// last and was just tried.
pub(crate) fn table_scan<G: Tabled>(lexer: &mut Lexer<'_, G>) -> Option<(usize, G::Pat, usize)> {
    match lexer.root_halts() {
        Some(&(end, after)) => scan::<G, true>(lexer, end, after),
        None => scan::<G, false>(lexer, &|_: &[u8], _: usize| false, None),
    }
}

/// [`table_scan`], with the halt tests compiled in only when `HALTING`.
/// A line that no other language embeds pays nothing for them.
/// `end` is tested before the patterns at each place.
/// `after` is tested where none of them matched.
fn scan<G: Tabled, const HALTING: bool>(
    lexer: &mut Lexer<'_, G>,
    end: &dyn Halt,
    after: Option<&dyn Halt>,
) -> Option<(usize, G::Pat, usize)> {
    let tables = G::tables(lexer.top_rule());
    let full = lexer.at_depth_limit();
    let s = lexer.s;
    let lim = s.len();
    let nl = usize::from(lexer.nl);
    let mut p = lexer.pos;
    let mut visited = 0;
    let found = loop {
        if p > lim + nl {
            break None;
        }
        if HALTING && end.holds(s, p) {
            lexer.halt_here(p);
            break None;
        }
        visited += 1;
        if visited % 64 == 0 && lexer.over_budget() {
            break None;
        }
        let list = match s.get(p) {
            Some(&byte) => {
                let place = if p == 0 {
                    Place::LineStart
                } else if is_word_byte(byte) && is_word_byte(s[p - 1]) {
                    Place::InWord
                } else {
                    Place::Away
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
        for &pat in list {
            if full && matches!(G::pat_act(pat), Act::Begin(_)) {
                if lexer.try_pat(pat, p).is_some() {
                    lexer.record_depth();
                }
                continue;
            }
            if let Some(end) = lexer.try_pat(pat, p) {
                hit = Some((p, pat, end));
                break;
            }
        }
        if hit.is_some() {
            break hit;
        }
        if HALTING && after.is_some_and(|after| after.holds(s, p)) {
            lexer.halt_here(p);
            break None;
        }
        p = match s.get(p) {
            Some(b' ' | b'\t') => match G::BLANK_RUNS {
                BlankRuns::Skip => lexer.skip(p, |b| matches!(b, b' ' | b'\t')),
                BlankRuns::ToLast => {
                    if matches!(s.get(p + 1), Some(b' ' | b'\t')) {
                        let run_end = lexer.skip(p, |b| matches!(b, b' ' | b'\t'));
                        if tables.tries_final_blank()
                            && let ControlFlow::Break(found) =
                                at_final_blank::<G, HALTING>(lexer, tables, end, run_end - 1, full)
                        {
                            break found;
                        }
                        run_end
                    } else {
                        p + 1
                    }
                }
                BlankRuns::Visit if G::SKIPS_PLACES => G::next_place(lexer, p),
                BlankRuns::Visit => lexer.char_end(p),
            },
            Some(_) if G::SKIPS_PLACES => G::next_place(lexer, p),
            Some(_) => lexer.char_end(p),
            None => p + 1,
        };
    };
    lexer.spend(visited);
    found
}

/// The scan's step at `p`, the last blank of a run, for a
/// [`BlankRuns::ToLast`] grammar.
///
/// It breaks with `None` when the halt `end` holds at `p`.
/// It breaks with the first one-blank pattern that matches at `p`.
/// It continues when neither happens.
/// A begin pattern of a rule the depth limit keeps closed is tried only to
/// record the limit, as in the scan.
///
/// It stays out of line.
/// Inlined, its values stayed live across the scan's loop, and the loop
/// reloaded spilled registers at every byte.
#[inline(never)]
fn at_final_blank<G: Tabled, const HALTING: bool>(
    lexer: &mut Lexer<'_, G>,
    tables: &Tables<G>,
    end: &dyn Halt,
    p: usize,
    full: bool,
) -> ControlFlow<Option<(usize, G::Pat, usize)>> {
    if HALTING && end.holds(lexer.s, p) {
        lexer.halt_here(p);
        return ControlFlow::Break(None);
    }
    for &pat in tables.one_blank() {
        if full && matches!(G::pat_act(pat), Act::Begin(_)) {
            if lexer.try_pat(pat, p).is_some() {
                lexer.record_depth();
            }
            continue;
        }
        if let Some(found) = lexer.try_pat(pat, p) {
            return ControlFlow::Break(Some((p, pat, found)));
        }
    }
    ControlFlow::Continue(())
}

/// Whether `byte` is an ASCII word character.
fn is_word_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A start that allows the bytes `bytes`.
    fn start(bytes: &[u8], bol: bool, wb: bool) -> Start {
        let mut set = [0_u64; 4];
        for &byte in bytes {
            set[usize::from(byte >> 6)] |= 1 << (byte & 63);
        }
        Start {
            bytes: set,
            bol,
            wb,
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
            start(b"ab", false, true),
            start(b"a", true, false),
            start(b"bc", false, false),
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
    fn equal_lists_are_stored_once() {
        let starts = [start(b"xy", false, false)];
        let (lists, spans) = candidate_lists(&starts);
        assert_eq!(lists, [0]);
        assert_eq!(spans[usize::from(b'x')], spans[usize::from(b'y')]);
    }

    #[test]
    fn a_pattern_past_the_first_word_of_the_sets_keeps_its_position() {
        let mut starts: Vec<Start> = (0..70).map(|_| start(b"q", false, false)).collect();
        starts.push(start(b"r", false, false));
        let tables = candidate_lists(&starts);
        assert_eq!(list(&tables, Place::Away, b'r'), [70]);
        assert_eq!(list(&tables, Place::Away, b'q').len(), 70);
    }
}
