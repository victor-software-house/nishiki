//! The capture groups of a match, numbered as the grammar's regular
//! expressions number them.

/// The most groups any rule has (the link rules have 21).
const GROUPS: usize = 22;

/// The deepest the scoped groups of one match nest inside each other. No
/// rule of the grammar nests deeper, which debug builds assert; a deeper
/// group would take no scope of its own.
pub(crate) const NESTING: usize = 4;

/// The byte range of each capture group that took part in a match, in group
/// order.
///
/// Clearing is one store, so a lexer keeps one table and clears it before
/// each rule it tries instead of building a table per attempt.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Caps {
    len: usize,
    groups: [(u8, u32, u32); GROUPS],
}

impl Caps {
    pub(crate) fn new() -> Self {
        Caps {
            len: 0,
            groups: [(0, 0, 0); GROUPS],
        }
    }

    pub(crate) fn clear(&mut self) {
        self.len = 0;
    }

    /// Records group `index` as the bytes from `start` to `end`. A full
    /// table keeps the groups it has: no rule sets more than [`GROUPS`],
    /// which debug builds assert.
    pub(crate) fn set(&mut self, index: usize, start: usize, end: usize) {
        debug_assert!(self.len < GROUPS, "group {index} past {GROUPS} groups");
        if self.len == GROUPS {
            return;
        }
        let (group, start, end) = (
            u8::try_from(index).unwrap_or(u8::MAX),
            u32::try_from(start).unwrap_or(u32::MAX),
            u32::try_from(end).unwrap_or(u32::MAX),
        );
        let mut at = self.len;
        while at > 0 && self.groups[at - 1].0 > group {
            self.groups[at] = self.groups[at - 1];
            at -= 1;
        }
        self.groups[at] = (group, start, end);
        self.len += 1;
    }

    /// The numbered groups that took part, in order.
    pub(crate) fn iter(&self) -> impl Iterator<Item = (usize, (usize, usize))> + '_ {
        self.groups[..self.len].iter().map(|&(index, start, end)| {
            (
                usize::from(index),
                (
                    usize::try_from(start).unwrap_or(usize::MAX),
                    usize::try_from(end).unwrap_or(usize::MAX),
                ),
            )
        })
    }
}
