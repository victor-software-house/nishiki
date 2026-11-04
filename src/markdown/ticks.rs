//! The backtick runs of a line, for looking up a code span's closing run.
//!
//! A code span opens with a run of `r` backticks.
//! It closes at the first later run of exactly `r` backticks.
//! Without one, the opening run gives back one backtick at a time.
//! The span then closes at the first later run of the longest length up to `r`.
//! One sweep from the line's end finds that run for every backtick.
//! The sweep keeps the first run of each length after the run it is at.

/// One run of backticks.
#[derive(Clone, Copy, Debug)]
struct Run {
    start: usize,
    len: usize,
    /// The number of backticks before this run on the line.
    first: usize,
}

impl Run {
    fn end(self) -> usize {
        self.start + self.len
    }
}

/// A code span.
/// It holds the length of the opening run and the span of the closing run.
#[derive(Clone, Copy, Debug)]
pub(crate) struct CodeSpan {
    pub(crate) open_len: usize,
    pub(crate) close: usize,
    pub(crate) end: usize,
}

/// The line's backtick runs, and the closing run for each backtick.
///
/// A code span first scans ahead for a run as long as its opening run.
/// That scan is exact, and it costs only the bytes up to the closing run.
/// The table is built the first time such a scan fails on a line.
/// Later code spans on the line look their closing run up in it.
///
/// The table is boxed and allocated on the first miss.
/// A highlighter that never needs it keeps an 8-byte field and allocates
/// nothing. Once allocated, the box is reused from line to line.
#[derive(Debug, Default)]
pub(crate) struct Ticks {
    held: Option<Box<Held>>,
}

/// The table and the line it describes.
#[derive(Debug, Default)]
struct Held {
    /// The address and length of the line that `table` describes.
    /// A Markdown lexer inside this line's HTML lexes a prefix of the line.
    /// The length tells the two lines apart.
    line: Option<(usize, usize)>,
    table: Table,
}

impl Ticks {
    /// Forgets the runs of the last line.
    pub(crate) fn reset(&mut self) {
        if let Some(held) = &mut self.held {
            held.line = None;
        }
    }

    /// The code span that opens at `p`, with the text ending at `line_end`.
    /// It also returns the steps it took, for the caller to spend.
    /// It is never inlined.
    #[inline(never)]
    pub(crate) fn code_span(
        &mut self,
        line: &[u8],
        p: usize,
        line_end: usize,
    ) -> (Option<CodeSpan>, usize) {
        let key = (line.as_ptr().addr(), line.len());
        if let Some(built) = self.held.as_deref().filter(|built| built.line == Some(key)) {
            return look_up(&built.table, p, line_end, 0);
        }
        match scan(line, p, line_end) {
            (Some(span), read) => (Some(span), read),
            (None, read) => {
                let missed = self.held.get_or_insert_with(Box::default);
                let built = missed.table.build(line);
                missed.line = Some(key);
                look_up(&missed.table, p, line_end, read + built)
            }
        }
    }
}

/// The code span at `p`, looked up in `table`.
/// The steps it returns are `spent` plus its lookups.
#[cold]
#[inline(never)]
fn look_up(table: &Table, p: usize, line_end: usize, spent: usize) -> (Option<CodeSpan>, usize) {
    let (found, lookups) = table.closing(p, line_end);
    (found, spent + lookups)
}

/// The first later run as long as the whole run at `p`.
/// The text ends at `line_end`.
/// It also returns the bytes it read.
#[inline(never)]
fn scan(line: &[u8], p: usize, line_end: usize) -> (Option<CodeSpan>, usize) {
    let text = line.get(..line_end).unwrap_or(line);
    let run_end = |at: usize| {
        at + text
            .get(at..)
            .map_or(0, |rest| rest.iter().take_while(|&&b| b == b'`').count())
    };
    let open_end = run_end(p);
    let open_len = open_end - p;
    let mut i = open_end;
    while let Some(found) = text.get(i..).and_then(|rest| memchr::memchr(b'`', rest)) {
        let start = i + found;
        let stop = run_end(start);
        if stop - start == open_len {
            let span = CodeSpan {
                open_len,
                close: start,
                end: stop,
            };
            return (Some(span), stop - p);
        }
        i = stop;
    }
    (None, text.len().saturating_sub(p))
}

/// The line's runs, and each backtick's closing run.
/// The table assumes the text runs to the end of the line.
#[derive(Debug, Default)]
struct Table {
    runs: Vec<Run>,
    /// For each backtick of the line, in order, the index of its closing run in
    /// `runs`.
    closers: Vec<Option<usize>>,
    /// During the sweep, the first run of each length after the current run.
    first_of_len: Vec<Option<usize>>,
}

impl Table {
    /// Finds the runs of `line` and the closing run of each backtick.
    /// Returns the bytes it read.
    #[cold]
    #[inline(never)]
    fn build(&mut self, line: &[u8]) -> usize {
        self.runs.clear();
        let mut ticks = 0;
        let mut longest = 0;
        let mut i = 0;
        while let Some(found) = line.get(i..).and_then(|rest| memchr::memchr(b'`', rest)) {
            let start = i + found;
            let len = line[start..].iter().take_while(|&&b| b == b'`').count();
            self.runs.push(Run {
                start,
                len,
                first: ticks,
            });
            ticks += len;
            longest = longest.max(len);
            i = start + len;
        }
        self.closers.clear();
        self.closers.resize(ticks, None);
        self.first_of_len.clear();
        self.first_of_len.resize(longest + 1, None);
        for (index, run) in self.runs.iter().enumerate().rev() {
            let mut best = None;
            for r in 1..=run.len {
                if let Some(later) = self.first_of_len[r] {
                    best = Some(later);
                }
                self.closers[run.first + run.len - r] = best;
            }
            self.first_of_len[run.len] = Some(index);
        }
        line.len()
    }

    /// The code span that opens at `p`, with the text ending at `line_end`.
    /// It also returns its lookups, for the caller to spend.
    ///
    /// The table assumes the text runs to the end of the line.
    /// A run that ends past `line_end` does not count.
    /// When the closing run found is such a run, the lookup tries the next
    /// shorter length. A run that `line_end` cuts counts with its cut length.
    fn closing(&self, p: usize, line_end: usize) -> (Option<CodeSpan>, usize) {
        let index = self.runs.partition_point(|run| run.start <= p);
        let Some(open) = index.checked_sub(1).and_then(|at| self.runs.get(at)) else {
            return (None, 1);
        };
        if p >= open.end() || open.end() > line_end {
            return (None, 1);
        }
        let r = open.end() - p;
        let mut lookups = 1;
        let mut found = None;
        let mut want = r;
        while want > 0 {
            let Some(at) = self.closers[open.first + open.len - want] else {
                break;
            };
            let run = self.runs[at];
            if run.end() <= line_end {
                found = Some(CodeSpan {
                    open_len: run.len,
                    close: run.start,
                    end: run.end(),
                });
                break;
            }
            lookups += 1;
            want = run.len - 1;
        }
        if self.runs.last().is_none_or(|last| last.end() <= line_end) {
            return (found, lookups);
        }
        let cut = self.runs.partition_point(|run| run.start < line_end);
        if cut > index
            && let Some(&run) = cut.checked_sub(1).and_then(|at| self.runs.get(at))
            && run.end() > line_end
        {
            let len = line_end - run.start;
            if len <= r && found.is_none_or(|span| len > span.open_len) {
                found = Some(CodeSpan {
                    open_len: len,
                    close: run.start,
                    end: line_end,
                });
            }
        }
        (found, lookups)
    }
}

#[cfg(test)]
mod tests {
    use super::{CodeSpan, Table, Ticks};

    /// The code span at `p`, found by the grammar's own scan.
    /// It tries each opening length from the whole run down.
    /// For each, it looks for the first later run of exactly that length.
    fn scanned(line: &[u8], p: usize, line_end: usize) -> Option<(usize, usize, usize)> {
        let run_end = |at: usize| {
            line[at..line_end]
                .iter()
                .take_while(|&&b| b == b'`')
                .count()
                + at
        };
        let open_end = run_end(p);
        for open_len in (1..=open_end - p).rev() {
            let mut i = open_end;
            while i < line_end {
                if line[i] == b'`' {
                    let stop = run_end(i);
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

    /// The first scan and the table find the span the grammar's scan finds.
    /// The test covers every line of up to ten backticks and letters.
    /// It tries every backtick and every end of the text after it.
    #[test]
    fn the_lookup_finds_the_span_a_scan_finds() {
        let mut ticks = Ticks::default();
        let mut table = Table::default();
        for len in 1..=10 {
            for bits in 0..1_u32 << len {
                let line: Vec<u8> = (0..len)
                    .map(|i| if (bits >> i) & 1 == 1 { b'`' } else { b'a' })
                    .collect();
                table.build(&line);
                for p in (0..len).filter(|&p| line[p] == b'`') {
                    for line_end in p + 1..=len {
                        let expected = scanned(&line, p, line_end);
                        ticks.reset();
                        let first = ticks.code_span(&line, p, line_end).0.map(parts);
                        let looked_up = table.closing(p, line_end).0.map(parts);
                        let case = format!(
                            "{:?} at {p}, ending at {line_end}",
                            String::from_utf8_lossy(&line)
                        );
                        assert_eq!(first, expected, "first lookup, {case}");
                        assert_eq!(looked_up, expected, "table, {case}");
                    }
                }
            }
        }
    }

    /// A highlighter that never needs the table pays one pointer for it.
    #[test]
    fn the_table_costs_one_pointer_until_used() {
        assert_eq!(size_of::<Ticks>(), size_of::<usize>());
    }

    fn parts(span: CodeSpan) -> (usize, usize, usize) {
        (span.open_len, span.close, span.end)
    }
}
