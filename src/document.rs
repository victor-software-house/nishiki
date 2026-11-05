//! A document highlighted as it arrives, delta by delta.
//!
//! The design is the `streaming` change under `openspec/changes/`.
//! A completed line's tokens depend only on the state before it and its own
//! text, so a completed line is lexed once.
//! Only the last line, still being written, is lexed again on each delta.
//! After an edit, lexing runs from the first changed line until a line's
//! state before it equals the state it was last lexed from.

use std::fmt;
use std::ops::Range;

use crate::{Highlighter, Language, Limits, LineReport, State, Theme, Token};

/// The tests name several languages, so they need them all.
/// The two conditions sit apart, because clippy lets tests use `expect` only
/// under a plain `#[cfg(test)]`.
#[cfg(test)]
#[cfg(feature = "all")]
mod tests;

/// The garbage tokens below which [`Document`] never packs its token runs.
/// Past it, a pack runs once garbage outweighs the tokens in use, so packing
/// costs at most one copy of each token it keeps.
const COMPACT_FLOOR: usize = 4096;

/// A document highlighted as it arrives, such as a model's reply or a file a
/// tool writes.
///
/// It owns one [`Highlighter`], the text, and each line's tokens and end
/// state.
/// [`Document::push`], [`Document::push_str`] and [`Document::replace`]
/// change the text and mark lines stale.
/// [`Document::lex`] lexes the stale lines, so a caller can lex its viewport
/// first and the rest later.
/// [`Document::take_changes`] tells a renderer which lines to draw again.
///
/// Once the document is finished and lexed to its end, every line's tokens
/// equal those [`Highlighter::line`] gives it when the whole text is lexed
/// line by line.
///
/// Lines end at `\n`, and a `\r` before it belongs to the line ending, as in
/// Shiki.
/// A document of `n` line feeds has `n + 1` lines.
#[derive(Debug)]
pub struct Document {
    highlighter: Highlighter,
    theme: Theme,
    text: String,
    /// The bytes of a UTF-8 character that a delta left unfinished.
    carry: Carry,
    /// One entry per completed line: every line but the last.
    lines: Vec<Entry>,
    /// The tokens of the completed lines, each line's in one run.
    /// A line lexed again appends a new run, and its old run becomes garbage.
    /// A stale line keeps the tokens it had, or none.
    tokens: Vec<Token>,
    /// How many tokens in `tokens` no line refers to.
    garbage: usize,
    /// How many completed lines, from the first, are lexed.
    lexed: usize,
    /// The last line: its tokens and what they were lexed from.
    tail: LastLine,
    /// When a long last line is lexed again.
    tail_rule: Tail,
    stream: Stream,
    /// The lines changed since the last [`Document::take_changes`].
    dirty: Option<Dirty>,
    /// The line count at the last [`Document::take_changes`].
    taken_lines: usize,
    /// How many times each kind of line was lexed, for the tests.
    #[cfg(test)]
    counts: Counts,
}

/// How many completed lines and last lines were lexed.
#[cfg(test)]
#[derive(Clone, Copy, Debug, Default)]
struct Counts {
    completed: usize,
    tail: usize,
    tail_bytes: usize,
}

/// A completed line: where its text ends, where the next line starts, where
/// its run of tokens starts in [`Document::tokens`] and how long it is, the
/// state it was last lexed from, its end state and its report.
/// `from` is `None` until the line is lexed, and again after its text
/// changes.
/// `state` and `report` hold only while `from` is set.
#[derive(Clone, Copy, Debug)]
struct Entry {
    end: usize,
    next: usize,
    first_token: usize,
    token_count: usize,
    from: Option<State>,
    state: State,
    report: LineReport,
}

/// The last line's tokens, and the text and state they were lexed from.
#[derive(Debug, Default)]
struct LastLine {
    tokens: Vec<Token>,
    lexed: Option<Lexed>,
}

/// What the last line was lexed from: the byte where the line starts, the
/// bytes lexed, the bytes it held when lexing last ran or chose not to, the
/// state before it, and what lexing gave.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Lexed {
    start: usize,
    len: usize,
    seen: usize,
    from: State,
    to: State,
    report: LineReport,
}

/// Whether more text may arrive.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Stream {
    Open,
    Finished,
}

/// Up to three bytes of an unfinished UTF-8 character, and one byte of room
/// to finish it.
#[derive(Clone, Copy, Debug, Default)]
struct Carry {
    bytes: [u8; 4],
    len: usize,
}

/// The changed lines: from line `start`, down to all but the last `keep`
/// lines.
/// Counting the unchanged lines from the bottom keeps the span right while
/// lines are added above them.
#[derive(Clone, Copy, Debug)]
struct Dirty {
    start: usize,
    keep: usize,
}

/// When a long last line is lexed again as it grows.
///
/// Lexing the last line on every delta costs the square of its length.
/// For a line of 100 KiB written four bytes at a time, that is over a
/// gigabyte of lexing.
/// Past `short` bytes, [`Tail::Amortized`] lexes it again only once it has
/// grown by `1 / growth` since its last lexing.
/// Past its first `short` bytes, a line then costs at most `growth + 1`
/// times its length.
/// The bytes past the last lexing have no tokens until the next one.
/// Both numbers count bytes, never time, so the same input gives the same
/// tokens on every machine.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Tail {
    /// Lex the last line again on every delta.
    EveryDelta,
    /// Lex it again on every delta while it is at most `short` bytes, then
    /// only once it has grown by `1 / growth`.
    /// A `growth` of zero lexes it again only when it ends.
    Amortized {
        /// The length up to which every delta lexes the line again.
        short: usize,
        /// The share of its length a line grows by before it is lexed again.
        growth: usize,
    },
}

impl Default for Tail {
    /// `Tail::Amortized { short: 1024, growth: 8 }`.
    fn default() -> Tail {
        Tail::Amortized {
            short: 1024,
            growth: 8,
        }
    }
}

/// How far [`Document::lex`] lexes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Until {
    /// Through the line of this index.
    Line(usize),
    /// Through the last line.
    End,
}

/// The lines whose text or tokens changed since the last
/// [`Document::take_changes`].
///
/// Lines `start..old_end` of the previous render became lines
/// `start..new_end`.
/// The lines before `start` and after the two ends did not change.
/// The span may hold lines that ended up unchanged.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct Changes {
    /// The first changed line.
    pub start: usize,
    /// The end of the changed lines, in the previous render's numbering.
    pub old_end: usize,
    /// The end of the changed lines, in the current numbering.
    pub new_end: usize,
}

/// Why [`Document::replace`] refused an edit.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum EditError {
    /// The range ends past the text, or starts after it ends.
    OutOfBounds,
    /// An end of the range falls inside a character.
    NotCharBoundary,
}

impl fmt::Display for EditError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EditError::OutOfBounds => f.write_str("the range is outside the text"),
            EditError::NotCharBoundary => f.write_str("the range cuts a character"),
        }
    }
}

impl std::error::Error for EditError {}

/// How final a line's tokens are.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum LineKind {
    /// Lexed from the exact state before it: final unless an edit reaches it.
    Exact,
    /// The last line of an open document, lexed as if it ended there.
    /// Its last token may still change as text arrives, so a renderer can
    /// show the bytes from `stable` in the default colour.
    /// Under [`Tail::Amortized`], the tokens of a long line may cover only
    /// part of it.
    Provisional {
        /// The byte where the line's last token starts.
        stable: usize,
    },
    /// Not lexed since its text changed.
    /// Its tokens, if any, are the ones it had and may cover only part of it.
    Stale,
}

/// One line of a [`Document`]: its text, its tokens and how final they are.
#[derive(Clone, Copy, Debug)]
pub struct DocumentLine<'a> {
    text: &'a str,
    tokens: &'a [Token],
    kind: LineKind,
    report: LineReport,
}

impl<'a> DocumentLine<'a> {
    /// The line's text, without its line ending.
    #[must_use]
    pub fn text(&self) -> &'a str {
        self.text
    }

    /// The line's tokens, whose ends are byte offsets in
    /// [`DocumentLine::text`].
    #[must_use]
    pub fn tokens(&self) -> &'a [Token] {
        self.tokens
    }

    /// How final the tokens are.
    #[must_use]
    pub fn kind(&self) -> LineKind {
        self.kind
    }

    /// The limits lexing the line reached.
    pub fn report(&self) -> LineReport {
        self.report
    }
}

impl Document {
    /// An empty document of `language`, styled with `theme`, with
    /// [`Limits::default`].
    #[must_use]
    pub fn new(language: Language, theme: Theme) -> Document {
        Document::new_with_limits(language, theme, Limits::default())
    }

    /// An empty document of `language`, styled with `theme`, with `limits`.
    #[must_use]
    pub fn new_with_limits(language: Language, theme: Theme, limits: Limits) -> Document {
        Document {
            highlighter: Highlighter::new_with_limits(language, limits),
            theme,
            text: String::new(),
            carry: Carry::default(),
            lines: Vec::new(),
            tokens: Vec::new(),
            garbage: 0,
            lexed: 0,
            tail: LastLine::default(),
            tail_rule: Tail::default(),
            stream: Stream::Open,
            dirty: None,
            taken_lines: 1,
            #[cfg(test)]
            counts: Counts::default(),
        }
    }

    /// Appends bytes that may end inside a UTF-8 character.
    ///
    /// The bytes of an unfinished character wait for the next delta.
    /// A byte sequence that is not UTF-8 becomes U+FFFD, as
    /// [`String::from_utf8_lossy`] reads it.
    /// Pushing after [`Document::finish`] opens the document again.
    pub fn push(&mut self, delta: &[u8]) {
        let mut rest = delta;
        while self.carry.len > 0 {
            let Some((&byte, after)) = rest.split_first() else {
                return;
            };
            rest = after;
            if let Some(slot) = self.carry.bytes.get_mut(self.carry.len) {
                *slot = byte;
                self.carry.len += 1;
            }
            let carried = self.carry;
            let pending = carried.bytes.get(..carried.len).unwrap_or_default();
            let unfinished = self.append_lossy(pending);
            self.carry = Carry::default();
            self.keep_unfinished(unfinished);
        }
        let unfinished = self.append_lossy(rest);
        self.keep_unfinished(unfinished);
    }

    /// Appends text.
    /// Pushing after [`Document::finish`] opens the document again.
    pub fn push_str(&mut self, delta: &str) {
        if self.carry.len > 0 {
            self.push(delta.as_bytes());
        } else {
            self.append(delta);
        }
    }

    /// Ends the stream: the last line stops being provisional.
    ///
    /// The bytes of an unfinished UTF-8 character become U+FFFD.
    pub fn finish(&mut self) {
        if self.carry.len > 0 {
            self.carry = Carry::default();
            self.append("\u{FFFD}");
        }
        self.stream = Stream::Finished;
        self.mark(self.lines.len(), self.line_count());
    }

    /// Lexes every stale line, in order, through the line `until` names.
    ///
    /// A line that the stream completed without changing its text since it
    /// was last lexed as the last line keeps those tokens, without being
    /// lexed again.
    pub fn lex(&mut self, until: Until) {
        let last = match until {
            Until::Line(index) => index,
            Until::End => usize::MAX,
        };
        while self.lexed < self.lines.len() && self.lexed <= last {
            let index = self.lexed;
            let before = self.state_before(index);
            if self
                .lines
                .get(index)
                .is_some_and(|line| line.from == Some(before))
            {
                self.lexed = index + 1;
                continue;
            }
            self.lex_completed(index);
        }
        if self.lexed == self.lines.len() && last >= self.lines.len() {
            self.lex_tail();
        }
        self.compact();
    }

    /// Replaces the bytes of `range` with `text`.
    ///
    /// The lines from the one `range` starts in become stale.
    /// [`Document::lex`] lexes the changed lines, then lexes on until a
    /// line's state before it equals the state it was last lexed from.
    /// That line and every unchanged line after it keep their tokens.
    ///
    /// # Errors
    ///
    /// [`EditError::OutOfBounds`] when `range` ends past the text or starts
    /// after it ends, and [`EditError::NotCharBoundary`] when either end
    /// falls inside a character.
    /// The document is unchanged then.
    pub fn replace(&mut self, range: Range<usize>, text: &str) -> Result<(), EditError> {
        if range.start > range.end || range.end > self.text.len() {
            return Err(EditError::OutOfBounds);
        }
        if !self.text.is_char_boundary(range.start) || !self.text.is_char_boundary(range.end) {
            return Err(EditError::NotCharBoundary);
        }
        let count = self.lines.len();
        let first = self.line_of(range.start);
        let last = self.line_of(range.end);
        let region_start = self.start_of(first);
        let removed = (last + 1).min(count) - first;
        for line in self.lines.get(first..first + removed).unwrap_or_default() {
            self.garbage += line.token_count;
        }
        self.text.replace_range(range.clone(), text);
        let grew = text.len() >= range.len();
        let by = text.len().abs_diff(range.len());
        let region_end = match self.lines.get(last) {
            Some(line) if grew => line.next + by,
            Some(line) => line.next - by,
            None => self.text.len(),
        };
        let added = self.split(first, removed, region_start, region_end);
        for line in self.lines.get_mut(first + added..).unwrap_or_default() {
            if grew {
                line.end += by;
                line.next += by;
            } else {
                line.end -= by;
                line.next -= by;
            }
        }
        let end = first + added;
        self.lexed = self.lexed.min(first);
        if last >= count {
            self.tail.lexed = None;
            self.tail.tokens.clear();
            self.mark(first, self.line_count());
        } else {
            if let Some(lexed) = self.tail.lexed.as_mut() {
                lexed.start = if grew {
                    lexed.start + by
                } else {
                    lexed.start - by
                };
            }
            self.mark(first, end);
        }
        Ok(())
    }

    /// Starts a fresh highlighter and marks every line stale.
    ///
    /// The highlighter interns each distinct state and stack once.
    /// Under many edits it can fill to [`Limits::nodes`], and a line's
    /// [`LineReport`] then names that limit.
    /// A reset empties it, at the cost of lexing the document again.
    pub fn reset(&mut self) {
        self.highlighter.renew();
        self.tokens.clear();
        self.garbage = 0;
        for line in &mut self.lines {
            line.first_token = 0;
            line.token_count = 0;
            line.from = None;
        }
        self.lexed = 0;
        self.tail.lexed = None;
        self.tail.tokens.clear();
        self.mark(0, self.line_count());
    }

    /// The lines whose text or tokens changed since the last call, or `None`
    /// when no line changed.
    pub fn take_changes(&mut self) -> Option<Changes> {
        let dirty = self.dirty.take()?;
        let count = self.line_count();
        let old_end = self.taken_lines.saturating_sub(dirty.keep);
        let new_end = count.saturating_sub(dirty.keep);
        self.taken_lines = count;
        Some(Changes {
            start: dirty.start.min(old_end).min(new_end),
            old_end,
            new_end,
        })
    }

    /// The document's text, without the bytes of an unfinished character.
    #[must_use]
    pub fn text(&self) -> &str {
        &self.text
    }

    /// The number of lines: one more than the line feeds.
    #[must_use]
    pub fn line_count(&self) -> usize {
        self.lines.len() + 1
    }

    /// The line of `index`, or `None` past the last line.
    #[must_use]
    pub fn line(&self, index: usize) -> Option<DocumentLine<'_>> {
        if index < self.lines.len() {
            return self.completed_line(index);
        }
        if index == self.lines.len() {
            return Some(self.tail_line());
        }
        None
    }

    /// The highlighter, whose [`Highlighter::scopes`] names a token's stack.
    #[must_use]
    pub fn highlighter(&self) -> &Highlighter {
        &self.highlighter
    }

    /// Sets when a long last line is lexed again as it grows.
    pub fn set_tail(&mut self, tail: Tail) {
        self.tail_rule = tail;
    }

    /// The theme the document is styled with.
    #[must_use]
    pub fn theme(&self) -> &Theme {
        &self.theme
    }

    /// The index of the line that holds byte `offset`; the last line for the
    /// text's end.
    fn line_of(&self, offset: usize) -> usize {
        self.lines.partition_point(|line| line.next <= offset)
    }

    /// Replaces `removed` entries from `first` with one entry for each line
    /// feed in the text from `start` to `end`, and gives the number added.
    fn split(&mut self, first: usize, removed: usize, start: usize, end: usize) -> usize {
        let bytes = self.text.as_bytes();
        let region = bytes.get(start..end).unwrap_or_default();
        let mut line_start = start;
        let entries = memchr::memchr_iter(b'\n', region).map(|at| {
            let feed = start + at;
            let end = if feed > line_start && bytes.get(feed - 1) == Some(&b'\r') {
                feed - 1
            } else {
                feed
            };
            line_start = feed + 1;
            Entry {
                end,
                next: feed + 1,
                first_token: 0,
                token_count: 0,
                from: None,
                state: State::default(),
                report: LineReport::default(),
            }
        });
        let before = self.lines.len();
        self.lines.splice(first..first + removed, entries);
        self.lines.len() + removed - before
    }

    /// Packs the token runs once garbage outweighs the tokens in use.
    fn compact(&mut self) {
        if self.garbage < COMPACT_FLOOR || self.garbage * 2 < self.tokens.len() {
            return;
        }
        let mut packed = Vec::with_capacity(self.tokens.len() - self.garbage);
        for line in &mut self.lines {
            let run = self
                .tokens
                .get(line.first_token..line.first_token + line.token_count)
                .unwrap_or_default();
            line.first_token = packed.len();
            packed.extend_from_slice(run);
        }
        self.tokens = packed;
        self.garbage = 0;
    }

    /// Appends text that holds no unfinished character.
    fn append(&mut self, delta: &str) {
        if delta.is_empty() {
            return;
        }
        let first = self.lines.len();
        let from = self.text.len();
        self.text.push_str(delta);
        self.stream = Stream::Open;
        for at in memchr::memchr_iter(b'\n', delta.as_bytes()) {
            let feed = from + at;
            let start = self.lines.last().map_or(0, |line| line.next);
            let end = if feed > start && self.text.as_bytes().get(feed - 1) == Some(&b'\r') {
                feed - 1
            } else {
                feed
            };
            self.lines.push(Entry {
                end,
                next: feed + 1,
                first_token: 0,
                token_count: 0,
                from: None,
                state: State::default(),
                report: LineReport::default(),
            });
        }
        self.mark(first, self.line_count());
    }

    /// Appends `bytes` as text, U+FFFD for each sequence that is not UTF-8,
    /// and gives back the bytes of a character they leave unfinished.
    fn append_lossy<'b>(&mut self, mut bytes: &'b [u8]) -> &'b [u8] {
        loop {
            match std::str::from_utf8(bytes) {
                Ok(text) => {
                    self.append(text);
                    return &[];
                }
                Err(error) => {
                    let (valid, rest) = bytes.split_at(error.valid_up_to());
                    if let Ok(text) = std::str::from_utf8(valid) {
                        self.append(text);
                    }
                    let Some(bad) = error.error_len() else {
                        return rest;
                    };
                    self.append("\u{FFFD}");
                    bytes = rest.get(bad..).unwrap_or_default();
                }
            }
        }
    }

    /// Keeps the bytes of an unfinished character for the next delta.
    /// `str::from_utf8` leaves at most three.
    fn keep_unfinished(&mut self, unfinished: &[u8]) {
        let mut carry = Carry::default();
        if let Some(slots) = carry.bytes.get_mut(..unfinished.len()) {
            slots.copy_from_slice(unfinished);
            carry.len = unfinished.len();
        }
        self.carry = carry;
    }

    /// The byte where the line of `index` starts.
    fn start_of(&self, index: usize) -> usize {
        index
            .checked_sub(1)
            .and_then(|above| self.lines.get(above))
            .map_or(0, |line| line.next)
    }

    /// The state before the line of `index`, which must follow lexed lines.
    fn state_before(&self, index: usize) -> State {
        index
            .checked_sub(1)
            .and_then(|above| self.lines.get(above))
            .map_or(State::default(), |line| line.state)
    }

    /// The bytes of the last line that are lexed: all of it, but a `\r` that
    /// may begin a line ending while more text can arrive.
    fn tail_len(&self) -> usize {
        let start = self.start_of(self.lines.len());
        let len = self.text.len().saturating_sub(start);
        match self.stream {
            Stream::Open if self.text.ends_with('\r') => len.saturating_sub(1),
            Stream::Open | Stream::Finished => len,
        }
    }

    /// Lexes the completed line of `index`, the first stale one.
    fn lex_completed(&mut self, index: usize) {
        let Some(&entry) = self.lines.get(index) else {
            return;
        };
        let first_token = self.tokens.len();
        let start = self.start_of(index);
        let from = self.state_before(index);
        let len = entry.end.saturating_sub(start);
        let (state, report) = match self.tail.lexed {
            Some(lexed) if lexed.start == start && lexed.len == len && lexed.from == from => {
                self.tokens.extend_from_slice(&self.tail.tokens);
                (lexed.to, lexed.report)
            }
            Some(_) | None => {
                let mut state = from;
                let text = self.text.get(start..entry.end).unwrap_or_default();
                let report = self.highlighter.line(text, &mut state, &mut self.tokens);
                #[cfg(test)]
                {
                    self.counts.completed += 1;
                }
                (state, report)
            }
        };
        if self.tail.lexed.is_some_and(|lexed| lexed.start == start) {
            self.tail.lexed = None;
            self.tail.tokens.clear();
        }
        self.garbage += entry.token_count;
        if let Some(line) = self.lines.get_mut(index) {
            line.first_token = first_token;
            line.token_count = self.tokens.len() - first_token;
            line.from = Some(from);
            line.state = state;
            line.report = report;
        }
        self.lexed = index + 1;
        self.mark(index, index + 1);
    }

    /// Lexes the last line, unless its tokens already match its text.
    fn lex_tail(&mut self) {
        let index = self.lines.len();
        let start = self.start_of(index);
        let len = self.tail_len();
        let from = self.state_before(index);
        if let Some(lexed) = self
            .tail
            .lexed
            .filter(|lexed| lexed.start == start && lexed.from == from)
            && (lexed.len == len || self.defers(lexed.len, len))
        {
            self.tail.lexed = Some(Lexed { seen: len, ..lexed });
            return;
        }
        self.tail.tokens.clear();
        let mut state = from;
        let text = self.text.get(start..start + len).unwrap_or_default();
        let report = self
            .highlighter
            .line(text, &mut state, &mut self.tail.tokens);
        #[cfg(test)]
        {
            self.counts.tail += 1;
            self.counts.tail_bytes += len;
        }
        self.tail.lexed = Some(Lexed {
            start,
            len,
            seen: len,
            from,
            to: state,
            report,
        });
        self.mark(index, index + 1);
    }

    /// Whether the last line, lexed through `lexed` bytes and now `len`
    /// bytes long, waits for more text before it is lexed again.
    fn defers(&self, lexed: usize, len: usize) -> bool {
        match (self.stream, self.tail_rule) {
            (Stream::Finished, Tail::EveryDelta | Tail::Amortized { .. })
            | (Stream::Open, Tail::EveryDelta) => false,
            (Stream::Open, Tail::Amortized { short, growth }) => {
                len > short && lexed < len && (len - lexed).saturating_mul(growth) < lexed
            }
        }
    }

    /// The completed line of `index`.
    fn completed_line(&self, index: usize) -> Option<DocumentLine<'_>> {
        let entry = self.lines.get(index)?;
        let text = self.text.get(self.start_of(index)..entry.end)?;
        let tokens = self
            .tokens
            .get(entry.first_token..entry.first_token + entry.token_count)
            .unwrap_or_default();
        if index < self.lexed {
            return Some(DocumentLine {
                text,
                tokens,
                kind: LineKind::Exact,
                report: entry.report,
            });
        }
        Some(DocumentLine {
            text,
            tokens,
            kind: LineKind::Stale,
            report: LineReport::default(),
        })
    }

    /// The last line.
    fn tail_line(&self) -> DocumentLine<'_> {
        let index = self.lines.len();
        let start = self.start_of(index);
        let len = self.tail_len();
        let text = self.text.get(start..start + len).unwrap_or_default();
        let from = self.state_before(index);
        let Some(lexed) = self.tail.lexed.filter(|lexed| lexed.start == start) else {
            return DocumentLine {
                text,
                tokens: &[],
                kind: LineKind::Stale,
                report: LineReport::default(),
            };
        };
        let current = lexed.seen == len && lexed.from == from && self.lexed == index;
        let whole = lexed.len == len;
        let tokens = self.tail.tokens.as_slice();
        let kind = match (current, self.stream, whole) {
            (false, Stream::Open | Stream::Finished, _) | (true, Stream::Finished, false) => {
                LineKind::Stale
            }
            (true, Stream::Finished, true) => LineKind::Exact,
            (true, Stream::Open, _) => LineKind::Provisional {
                stable: tokens
                    .len()
                    .checked_sub(2)
                    .and_then(|before| tokens.get(before))
                    .map_or(0, |token| token.end()),
            },
        };
        DocumentLine {
            text,
            tokens,
            kind,
            report: lexed.report,
        }
    }

    /// Records that lines `first..end` changed.
    fn mark(&mut self, first: usize, end: usize) {
        let keep = self.line_count().saturating_sub(end);
        self.dirty = Some(match self.dirty {
            Some(dirty) => Dirty {
                start: dirty.start.min(first),
                keep: dirty.keep.min(keep),
            },
            None => Dirty { start: first, keep },
        });
    }
}
