//! The searches that ask about the rest of the line, shared by the texts of
//! one line.
//!
//! An embedding language can lex many texts of one line with this lexer,
//! such as HTML's event handlers, one per attribute.
//! A search of the line per text would read the line once per text.
//!
//! Instead, the second text of a line and those after it read a list of the
//! line's matches.
//! The list grows forward and is read once per line.

use super::lexer::Opened;

/// What the TypeScript lexer keeps from one line to the next.
#[derive(Debug, Default)]
pub(crate) struct Scratch {
    /// The contexts opened on the current line (see [`Opened`]).
    pub(crate) opened: Vec<Opened>,
    /// The searches of the current line.
    pub(crate) searches: Searches,
}

/// The searches every text of one line shares.
///
/// Every text an embedding language gives this lexer is the line's text or a
/// prefix of it.
/// Each starts where the line does.
/// A position then means the same byte in every text of the line.
#[derive(Debug, Default)]
pub(crate) struct Searches {
    /// The number of the line the last lexer started on, as
    /// [`crate::scratch::Scratch::line`] numbers it.
    started: u64,
    /// The number of the line the lists hold.
    listed: u64,
    /// Where the line's backticks are.
    ticks: Matches,
    /// Where the line's `(` and `<` are.
    opens: Matches,
}

/// The bytes a search looks for.
#[derive(Clone, Copy, Debug)]
enum Find {
    /// A backtick.
    Tick,
    /// A `(` or a `<`.
    Open,
}

/// The positions of one search's matches in the line, up to where the line
/// has been read.
#[derive(Debug, Default)]
struct Matches {
    at: Vec<usize>,
    until: usize,
}

impl Searches {
    /// The last backtick and the last `(` or `<` in `bytes[from..]`, where
    /// `bytes` is a text of line `line`, and the number of bytes read to find
    /// them.
    ///
    /// The first text of a line searches its own bytes, as a document of one
    /// text per line does on every line.
    /// The texts after it read the line's lists of matches.
    #[inline]
    pub(crate) fn last(
        &mut self,
        line: u64,
        bytes: &[u8],
        from: usize,
    ) -> (Option<usize>, Option<usize>, usize) {
        if self.started != line {
            self.started = line;
            let text = bytes.get(from..).unwrap_or_default();
            let tick = memchr::memrchr(b'`', text);
            let open = memchr::memrchr2(b'(', b'<', text);
            let read = |found: Option<usize>| found.map_or(text.len(), |at| text.len() - at);
            return (
                tick.map(|at| from + at),
                open.map(|at| from + at),
                read(tick) + read(open),
            );
        }
        self.listed_last(line, bytes, from)
    }

    /// [`Searches::last`] for a text after the first of its line, from the
    /// line's lists.
    /// It starts the lists when the line has none yet.
    #[cold]
    #[inline(never)]
    fn listed_last(
        &mut self,
        line: u64,
        bytes: &[u8],
        from: usize,
    ) -> (Option<usize>, Option<usize>, usize) {
        if self.listed != line {
            self.listed = line;
            self.ticks.clear();
            self.opens.clear();
        }
        let (tick, tick_read) = self.ticks.last(Find::Tick, bytes, from);
        let (open, open_read) = self.opens.last(Find::Open, bytes, from);
        (tick, open, tick_read + open_read)
    }
}

impl Matches {
    /// Forgets the line before.
    fn clear(&mut self) {
        self.at.clear();
        self.until = 0;
    }

    /// The last match of `find` in `bytes[from..]`, and the number of bytes
    /// read.
    /// It first reads on to the end of `bytes` where no text of the line has
    /// read yet.
    fn last(&mut self, find: Find, bytes: &[u8], from: usize) -> (Option<usize>, usize) {
        let start = self.until;
        let mut read = 0;
        if let Some(rest) = bytes.get(start..) {
            let at = &mut self.at;
            match find {
                Find::Tick => at.extend(memchr::memchr_iter(b'`', rest).map(|i| start + i)),
                Find::Open => at.extend(memchr::memchr2_iter(b'(', b'<', rest).map(|i| start + i)),
            }
            self.until = bytes.len();
            read = rest.len();
        }
        let before = self.at.partition_point(|&i| i < bytes.len());
        let last = self.at[..before].last().copied().filter(|&i| i >= from);
        (last, read)
    }
}
