//! The capture groups of a match, numbered as the grammar's regular
//! expressions number them.

/// The most groups any rule has (the link rules have 21).
const GROUPS: usize = 22;

/// The byte range of each capture group that took part in a match.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Caps([Option<(usize, usize)>; GROUPS]);

impl Caps {
    pub(crate) fn new() -> Self {
        Caps([None; GROUPS])
    }

    /// Records group `index` as the bytes from `start` to `end`.
    pub(crate) fn set(&mut self, index: usize, start: usize, end: usize) {
        self.0[index] = Some((start, end));
    }

    /// The numbered groups that took part, in order.
    pub(crate) fn iter(&self) -> impl Iterator<Item = (usize, (usize, usize))> + '_ {
        self.0
            .iter()
            .enumerate()
            .filter_map(|(index, span)| span.map(|span| (index, span)))
    }
}
