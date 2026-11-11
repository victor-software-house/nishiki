//! The delimiter runs of a line, for looking up a span's closing run.
//!
//! Code spans and strikethroughs both open with a run of a delimiter.
//! They close at the first later run of the same length.
//! Without one, the opening run gives back one delimiter at a time.
//! The span then closes at the first later run of the longest length that fits.
//!
//! One sweep from the line's end finds that run for every delimiter.
//! The sweep keeps the first run of each length after the run it is at.
//!
//! A [`Closer`] adds each span's own rules for its closing run.
//! Strikethrough passes over an escaped run.
//! When the first run of a length fails strikethrough's check, no run of that
//! length closes.

use super::stops::{Opener, Step, Stops};
use super::titles::{Shut, Titles};

/// A delimiter whose runs open and close a span.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Delim {
    /// A code span's backtick.
    Backtick,
    /// A strikethrough's tilde.
    Tilde,
}

impl Delim {
    const fn byte(self) -> u8 {
        match self {
            Delim::Backtick => b'`',
            Delim::Tilde => b'~',
        }
    }
}

/// What a span asks of the run that closes it, besides its length.
pub(crate) trait Closer {
    /// The delimiter of the span's runs.
    const DELIM: Delim;
    /// The shortest run that opens the span.
    const SHORTEST: usize;
    /// Whether the run at `start` counts as a closing run.
    /// A run that does not count is passed over.
    fn eligible(&self, start: usize) -> bool;
    /// Whether the run from `start` to `end` closes the span.
    /// It is the first counting run of its length.
    /// When it does not close, no run of its length does.
    fn closes(&self, start: usize, end: usize) -> bool;
}

/// Where a span opens.
/// The opening delimiter is at `p`, and opening runs are at most `widest` long.
/// The text ends at `line_end`.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Open {
    pub(crate) p: usize,
    pub(crate) widest: usize,
    pub(crate) line_end: usize,
}

/// A span.
/// It holds the length of the opening run and the span of the closing run.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Span {
    pub(crate) open_len: usize,
    pub(crate) close: usize,
    pub(crate) end: usize,
}

/// One run of a delimiter.
#[derive(Clone, Copy, Debug)]
struct Run {
    start: usize,
    len: usize,
    /// The number of the line's delimiters before this run.
    first: usize,
    /// Whether it counts as a closing run, by [`Closer::eligible`].
    eligible: bool,
    /// Whether it closes as the first counting run of its length.
    /// [`Closer::closes`] decides, with the text running to the end of the
    /// line.
    closes: bool,
}

impl Run {
    fn end(self) -> usize {
        self.start + self.len
    }
}

/// The line's runs of each delimiter, and the closing run for each delimiter.
///
/// A span first scans ahead for a counting run as long as its widest opening.
/// That scan is exact, and it costs only the bytes up to the closing run.
/// A delimiter's table is built the first time such a scan misses on a line.
///
/// The box also holds emphasis's memo of stops and the image rule's memo of
/// title scans.
///
/// The tables are boxed and allocated on the first miss.
/// A highlighter that never needs them keeps an 8-byte field and allocates
/// nothing.
/// Once allocated, the box is reused from line to line.
#[derive(Debug, Default)]
pub(crate) struct Runs {
    held: Option<Box<Held>>,
}

/// Each delimiter's table and the line it describes, emphasis's memo of
/// stops, and the image rule's memo of title scans.
#[derive(Debug, Default)]
struct Held {
    backtick: Built,
    tilde: Built,
    stops: Stops,
    titles: Titles,
}

/// One delimiter's table and the line it describes.
#[derive(Debug, Default)]
struct Built {
    /// The address and length of the line that `table` describes.
    /// A Markdown lexer inside this line's HTML lexes a prefix of the line.
    /// The length distinguishes the two lines.
    line: Option<(usize, usize)>,
    table: Table,
}

impl Held {
    fn of(&mut self, delim: Delim) -> &mut Built {
        match delim {
            Delim::Backtick => &mut self.backtick,
            Delim::Tilde => &mut self.tilde,
        }
    }
}

impl Runs {
    /// Forgets the runs, the stops and the title scans of the last line.
    pub(crate) fn reset(&mut self) {
        if let Some(held) = &mut self.held {
            held.backtick.line = None;
            held.tilde.line = None;
            held.stops.forget();
            held.titles.forget();
        }
    }

    /// Where an image title's scan from `from` stops, through the line's memo
    /// of title scans.
    /// It also returns how many positions the scan stepped past, for the
    /// caller to spend.
    /// `text` is the line's address and length, and the end of its text.
    #[cold]
    #[inline(never)]
    pub(crate) fn title_stop(
        &mut self,
        text: (usize, usize, usize),
        shut: Shut,
        from: usize,
        is_stop: impl FnMut(usize) -> bool,
    ) -> (usize, usize) {
        self.held
            .get_or_insert_with(Box::default)
            .titles
            .stop(text, shut, from, is_stop)
    }

    /// Where a long emphasis look-ahead that `step` takes stops, from `start`.
    /// The memo of stops shares the tables' box, so one reset forgets both.
    /// The memo leaves the box during the walk.
    /// `step` then gets the runs, for the code spans it steps over.
    /// None means the line ran out of work.
    #[cold]
    #[inline(never)]
    pub(crate) fn walk_long(
        &mut self,
        text: (usize, usize, usize),
        opener: Opener,
        start: usize,
        mut step: impl FnMut(&mut Runs, usize) -> Step,
    ) -> Option<usize> {
        let mut stops = std::mem::take(&mut self.held.get_or_insert_with(Box::default).stops);
        let found = stops.walk(text, opener, start, |q| step(self, q));
        self.held.get_or_insert_with(Box::default).stops = stops;
        found
    }

    /// The span that opens at `open`.
    /// It also returns the steps it took, for the caller to spend.
    /// `at` checks runs in the text the rule sees.
    /// `full` checks runs in the text running to the end of the line.
    /// It is never inlined, and neither is its scan.
    /// Inlined, it pushed functions it does not call out of line.
    #[inline(never)]
    pub(crate) fn closing<C: Closer>(
        &mut self,
        line: &[u8],
        open: Open,
        at: &C,
        full: &C,
    ) -> (Option<Span>, usize) {
        let key = (line.as_ptr().addr(), line.len());
        if let Some(built) = self
            .held
            .as_deref_mut()
            .map(|held| held.of(C::DELIM))
            .filter(|built| built.line == Some(key))
        {
            return look_up(&built.table, open, at, 0);
        }
        match scan(line, open, at) {
            (Scanned::Settled(found), read) => (found, read),
            (Scanned::Missed, read) => {
                let missed = self.held.get_or_insert_with(Box::default).of(C::DELIM);
                let built = missed.table.build(line, full);
                missed.line = Some(key);
                look_up(&missed.table, open, at, read + built)
            }
        }
    }
}

/// The span at `open`, looked up in `table`.
/// The steps it returns are `spent` plus its lookups.
#[cold]
#[inline(never)]
fn look_up<C: Closer>(table: &Table, open: Open, at: &C, spent: usize) -> (Option<Span>, usize) {
    let (found, lookups) = table.closing(open, at);
    (found, spent + lookups)
}

/// What the first scan of a span found.
enum Scanned {
    /// The answer: a span, or none when no opening is long enough.
    Settled(Option<Span>),
    /// No answer at the widest opening; shorter openings need the table.
    Missed,
}

/// The first counting run as long as the widest opening at `open`.
/// It also returns the bytes it read.
#[inline(never)]
fn scan<C: Closer>(line: &[u8], open: Open, at: &C) -> (Scanned, usize) {
    let byte = C::DELIM.byte();
    let text = line.get(..open.line_end).unwrap_or(line);
    let run_end = |from: usize| {
        from + text
            .get(from..)
            .map_or(0, |rest| rest.iter().take_while(|&&b| b == byte).count())
    };
    let open_end = run_end(open.p);
    let open_len = open.widest.min(open_end - open.p);
    if open_len < C::SHORTEST {
        return (Scanned::Settled(None), 0);
    }
    let mut i = open_end;
    while let Some(found) = text.get(i..).and_then(|rest| memchr::memchr(byte, rest)) {
        let start = i + found;
        let stop = run_end(start);
        if stop - start == open_len && at.eligible(start) {
            if !at.closes(start, stop) {
                return (Scanned::Missed, stop - open.p);
            }
            let span = Span {
                open_len,
                close: start,
                end: stop,
            };
            return (Scanned::Settled(Some(span)), stop - open.p);
        }
        i = stop;
    }
    (Scanned::Missed, text.len().saturating_sub(open.p))
}

/// One delimiter's runs on the line, and each delimiter's closing run.
/// The table assumes the text runs to the end of the line.
#[derive(Debug, Default)]
struct Table {
    runs: Vec<Run>,
    /// For each delimiter of the line, in order, the index of its closing run
    /// in `runs`.
    closers: Vec<Option<usize>>,
    /// During the sweep, the first counting run of each length after the
    /// current run.
    first_of_len: Vec<Option<usize>>,
}

impl Table {
    /// Finds the runs of `full`'s delimiter in `line`, and each delimiter's
    /// closing run.
    /// It returns the bytes it read.
    #[cold]
    #[inline(never)]
    fn build<C: Closer>(&mut self, line: &[u8], full: &C) -> usize {
        let byte = C::DELIM.byte();
        self.runs.clear();
        let mut count = 0;
        let mut longest = 0;
        let mut i = 0;
        while let Some(found) = line.get(i..).and_then(|rest| memchr::memchr(byte, rest)) {
            let start = i + found;
            let len = line[start..].iter().take_while(|&&b| b == byte).count();
            self.runs.push(Run {
                start,
                len,
                first: count,
                eligible: full.eligible(start),
                closes: full.closes(start, start + len),
            });
            count += len;
            longest = longest.max(len);
            i = start + len;
        }
        self.closers.clear();
        self.closers.resize(count, None);
        self.first_of_len.clear();
        self.first_of_len.resize(longest + 1, None);
        for (index, run) in self.runs.iter().enumerate().rev() {
            let mut best = None;
            for r in C::SHORTEST..=run.len {
                if let Some(later) = self.first_of_len[r]
                    && self.runs[later].closes
                {
                    best = Some(later);
                }
                self.closers[run.first + run.len - r] = best;
            }
            if run.eligible {
                self.first_of_len[run.len] = Some(index);
            }
        }
        line.len()
    }

    /// The span at `open`.
    /// It also returns its lookups, for the caller to spend.
    ///
    /// The table assumes the text runs to the end of the line, so a run that
    /// ends past `open.line_end` does not count.
    /// When the closing run found is such a run, the lookup tries the next
    /// shorter length.
    /// The one run that reaches `open.line_end` is checked separately.
    /// It counts with its cut length, and the rule checks it in the cut text.
    fn closing<C: Closer>(&self, open: Open, at: &C) -> (Option<Span>, usize) {
        let Open {
            p,
            widest,
            line_end,
        } = open;
        let index = self.runs.partition_point(|run| run.start <= p);
        let Some(&opener) = index.checked_sub(1).and_then(|at| self.runs.get(at)) else {
            return (None, 1);
        };
        if p >= opener.end() || opener.end() > line_end {
            return (None, 1);
        }
        let first_want = widest.min(opener.end() - p);
        let mut lookups = 1;
        let mut found = None;
        let mut want = first_want;
        while want >= C::SHORTEST {
            let Some(later) = self.closers[opener.first + opener.len - want] else {
                break;
            };
            let run = self.runs[later];
            if run.end() < line_end {
                found = Some(Span {
                    open_len: run.len,
                    close: run.start,
                    end: run.end(),
                });
                break;
            }
            lookups += 1;
            want = run.len - 1;
        }
        let cut = self.runs.partition_point(|run| run.start < line_end);
        let Some(reach) = cut.checked_sub(1).filter(|&at| at >= index) else {
            return (found, lookups);
        };
        let run = self.runs[reach];
        let len = line_end - run.start;
        if run.end() < line_end
            || len < C::SHORTEST
            || len > first_want
            || found.is_some_and(|span: Span| span.open_len >= len)
            || !at.eligible(run.start)
            || !at.closes(run.start, line_end)
        {
            return (found, lookups);
        }
        let between = &self.runs[index..reach];
        lookups += between.len();
        if between
            .iter()
            .any(|earlier| earlier.eligible && earlier.len == len)
        {
            return (found, lookups);
        }
        let span = Span {
            open_len: len,
            close: run.start,
            end: line_end,
        };
        (Some(span), lookups)
    }
}

#[cfg(test)]
mod tests {
    use super::Runs;
    use crate::limits::{Limits, Steps};
    use crate::markdown::captures::Captures;
    use crate::markdown::inline::{code_span, strikethrough};
    use crate::markdown::text::{Cx, LineFacts};

    /// Every line of up to `max` bytes drawn from `alphabet`.
    fn lines(alphabet: &[u8], max: usize) -> Vec<Vec<u8>> {
        let mut all = vec![Vec::new()];
        let mut last = vec![Vec::new()];
        for _ in 0..max {
            last = last
                .iter()
                .flat_map(|line: &Vec<u8>| {
                    alphabet.iter().map(move |&b| {
                        let mut longer = line.clone();
                        longer.push(b);
                        longer
                    })
                })
                .collect();
            all.extend(last.iter().cloned());
        }
        all
    }

    /// The code span at `p`, found by the grammar's own scan.
    /// It tries each opening length from the whole run down.
    /// For each, it looks for the first later run of exactly that length.
    fn code_scanned(cx: &Cx<'_>, p: usize) -> Option<(usize, usize, usize)> {
        let line_end = cx.end.min(cx.line.len());
        let open_end = cx.skip(p, |b| b == b'`');
        for open_len in (1..=open_end - p).rev() {
            let mut i = open_end;
            while i < line_end {
                if cx.line[i] == b'`' {
                    let stop = cx.skip(i, |b| b == b'`').min(line_end);
                    if stop - i == open_len {
                        return Some((open_len, i, stop));
                    }
                    i = stop;
                } else {
                    i += 1;
                }
            }
        }
        None
    }

    /// The strikethrough at `p`, found by the grammar's own scan.
    /// It tries each opening length from the whole run down to two.
    /// For each, it looks for the first later unescaped run of exactly that
    /// length. That run closes unless `_` comes before it and a word after it.
    fn strike_scanned(cx: &Cx<'_>, p: usize) -> Option<(usize, usize, usize)> {
        if cx.before(p) == Some(b'\\') {
            return None;
        }
        let line_end = cx.end.min(cx.line.len());
        let run_end = cx.skip(p, |b| b == b'~');
        for open_len in (2..=run_end - p).rev() {
            let open_end = p + open_len;
            if cx.get(open_end) == Some(b'_')
                && open_end >= 3
                && cx.get(open_end - 1) == Some(b'~')
                && cx.get(open_end - 2) == Some(b'~')
                && cx.word_before(open_end - 2)
            {
                continue;
            }
            let mut i = run_end;
            while i < line_end {
                if cx.line[i] != b'~' {
                    i += 1;
                    continue;
                }
                let stop = cx.skip(i, |b| b == b'~').min(line_end);
                let start = i;
                i = stop;
                if stop - start != open_len || cx.before(start) == Some(b'\\') {
                    continue;
                }
                if cx.before(start) == Some(b'_') && cx.word_at(stop) {
                    break;
                }
                return Some((open_len, start, stop));
            }
        }
        None
    }

    /// The strikethrough at `p`, found by the rule, as its spans.
    fn strike_found(cx: &Cx<'_>, runs: &mut Runs, p: usize) -> Option<(usize, usize, usize)> {
        let mut captures = Captures::new();
        let end = strikethrough(cx, runs, p, &mut captures)?;
        let group = |index: usize| {
            captures
                .iter()
                .find_map(|(group, span)| (group == index).then_some(span))
        };
        let (_, open_end) = group(1)?;
        let (close, _) = group(3)?;
        Some((open_end - p, close, end))
    }

    /// The rule finds the span the scan finds.
    /// The test covers each line, each delimiter in it and each end of its
    /// text.
    /// It uses a fresh table, which takes the first scan.
    /// It also uses a table that the line's earlier spans built.
    fn check(
        alphabet: &[u8],
        max: usize,
        delim: u8,
        found: impl Fn(&Cx<'_>, &mut Runs, usize) -> Option<(usize, usize, usize)>,
        scanned: impl Fn(&Cx<'_>, usize) -> Option<(usize, usize, usize)>,
    ) {
        let mut fresh = Runs::default();
        let mut kept = Runs::default();
        for line in lines(alphabet, max) {
            let facts = LineFacts {
                steps: Steps::new(&Limits::default(), line.len()),
                before_start: None,
            };
            kept.reset();
            for end in 1..=line.len() + 1 {
                for p in (0..line.len().min(end)).filter(|&p| line[p] == delim) {
                    let cx = Cx {
                        line: &line,
                        end,
                        anchor: None,
                        facts: &facts,
                    };
                    let expected = scanned(&cx, p);
                    let case = format!(
                        "{:?} at {p}, ending at {end}",
                        String::from_utf8_lossy(&line)
                    );
                    fresh.reset();
                    assert_eq!(found(&cx, &mut fresh, p), expected, "first scan, {case}");
                    assert_eq!(found(&cx, &mut kept, p), expected, "kept table, {case}");
                }
            }
        }
    }

    /// A highlighter that never needs the tables pays one pointer for them.
    #[test]
    fn the_tables_cost_one_pointer_until_used() {
        assert_eq!(size_of::<Runs>(), size_of::<usize>());
    }

    #[test]
    fn code_spans_close_where_a_scan_closes_them() {
        check(
            b"`a",
            10,
            b'`',
            |cx, runs, p| code_span(cx, runs, p).map(|span| (span.open_len, span.close, span.end)),
            code_scanned,
        );
    }

    #[test]
    fn strikethroughs_close_where_a_scan_closes_them() {
        check(b"~a_\\", 7, b'~', strike_found, strike_scanned);
    }
}
