//! For each rule, the patterns that can match at a byte, in the order the
//! grammar tries them, so the engine tries only those where a byte stands.

use std::collections::HashMap;

use super::{Act, Lexer, RuleData, Start, Tabled};
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
    /// feed or at the end of a last line without one: all of them, the end
    /// pattern first, less those that need a second byte
    /// ([`Grammar::pat_next`](super::Grammar::pat_next)).
    all: Box<[G::Pat]>,
    /// The candidates for each byte, away from the start of the line (the
    /// first 256), at it (the next 256) and inside a word (the last 256).
    /// Bytes with the same candidates share one list.
    by_byte: Box<[&'static [G::Pat]]>,
}

impl<G: Tabled> Tables<G> {
    /// The tables of the rule whose data is `data`.
    ///
    /// Each pattern's start is read once, and each distinct list of
    /// candidates is stored once, in one allocation that lives as long as
    /// the program: the tables are built into statics that are never
    /// dropped, so it is leaked rather than owned.
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
        Tables {
            all: all
                .into_iter()
                .filter(|&pat| G::pat_next(pat).is_none())
                .collect(),
            by_byte: spans
                .into_iter()
                .map(|(start, len)| lists.get(start..start + len).unwrap_or_default())
                .collect(),
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
}

/// For patterns with the starts `starts`, the positions of the candidates
/// of every place and byte, each distinct list once, and where each place
/// and byte's list lies among them. It works on positions, so it is built
/// once for every grammar.
fn candidate_lists(starts: &[Start]) -> (Vec<usize>, Vec<(usize, usize)>) {
    let mut lists = Vec::new();
    let mut seen: HashMap<Vec<usize>, (usize, usize)> = HashMap::new();
    let mut spans = Vec::with_capacity(768);
    let mut picks = Vec::new();
    for place in [Place::Away, Place::LineStart, Place::InWord] {
        for byte in 0..=255_u8 {
            picks.clear();
            picks.extend(
                starts
                    .iter()
                    .enumerate()
                    .filter(|(_, start)| {
                        (place == Place::LineStart || !start.bol)
                            && (place != Place::InWord || !start.wb)
                            && start.bytes[usize::from(byte >> 6)] >> (byte & 63) & 1 == 1
                    })
                    .map(|(index, _)| index),
            );
            let span = if let Some(&span) = seen.get(&picks) {
                span
            } else {
                let span = (lists.len(), picks.len());
                lists.extend_from_slice(&picks);
                seen.insert(picks.clone(), span);
                span
            };
            spans.push(span);
        }
    }
    (lists, spans)
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
            Some(b' ' | b'\t') => lexer.skip(p, |b| matches!(b, b' ' | b'\t')),
            Some(_) if G::SKIPS_PLACES => G::next_place(lexer, p),
            Some(_) => lexer.char_end(p),
            None => p + 1,
        };
    };
    lexer.spend(visited);
    found
}

/// Whether `byte` is an ASCII word character.
fn is_word_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}
