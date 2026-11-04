//! Where the characters that lookaheads search for are in a line.
//! The answers are kept from one search to the next.
//! `[^{]*;` and its kin, tried at each position of a run, then scan the line
//! once and allocate nothing.

use std::cell::Cell;

/// The last search for each byte the CSS lookaheads look for: where it
/// started and what it found.
#[derive(Debug)]
pub(crate) struct Ahead {
    last: [Cell<(usize, Option<usize>)>; Find::COUNT],
}

impl Default for Ahead {
    fn default() -> Self {
        Ahead {
            last: [const { Cell::new((usize::MAX, None)) }; Find::COUNT],
        }
    }
}

/// A byte a lookahead searches for.
#[derive(Clone, Copy, Debug)]
pub(crate) enum Find {
    /// `{`
    Open,
    /// `}`
    Close,
    /// `;`
    Semicolon,
    /// `"`
    Quote,
    /// `:`
    Colon,
}

impl Find {
    /// How many bytes there are to search for.
    const COUNT: usize = 5;

    /// Its slot in [`Ahead`].
    fn slot(self) -> usize {
        match self {
            Find::Open => 0,
            Find::Close => 1,
            Find::Semicolon => 2,
            Find::Quote => 3,
            Find::Colon => 4,
        }
    }

    /// The byte itself.
    fn byte(self) -> u8 {
        match self {
            Find::Open => b'{',
            Find::Close => b'}',
            Find::Semicolon => b';',
            Find::Quote => b'"',
            Find::Colon => b':',
        }
    }
}

impl Ahead {
    /// The first `what` in `s` at or after `from`, and how many bytes the
    /// search read.
    /// A search that starts at or after the last one for the same byte, and
    /// not past what it found, has its answer already.
    /// It reads nothing.
    pub(crate) fn next(&self, s: &[u8], what: Find, from: usize) -> (Option<usize>, usize) {
        let slot = &self.last[what.slot()];
        let (start, found) = slot.get();
        if start <= from && found.is_none_or(|at| from <= at) {
            return (found, 0);
        }
        let rest = s.get(from..).unwrap_or(&[]);
        let hit = memchr::memchr(what.byte(), rest);
        slot.set((from, hit.map(|at| from + at)));
        (hit.map(|at| from + at), hit.map_or(rest.len(), |at| at + 1))
    }
}
