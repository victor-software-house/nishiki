//! The per-line lexer: cursor, token emission, the context stack and the
//! rules that read a table's lines.

use std::cell::Cell;

use memchr::{memchr2, memrchr};

use super::scope::S;
use super::state::Kind;
use crate::arena::{Arena, Frame};
use crate::limits::{Limit, Limits, LineReport};
use crate::utf8::char_len;
use crate::{Stack, Token, classes};

/// Lexes one line. `pos` is the end of the last token; the bytes between it
/// and the cursor are plain text of the current context.
pub(crate) struct Lexer<'a> {
    pub(super) s: &'a [u8],
    out: &'a mut Vec<Token>,
    arena: &'a mut Arena,
    limits: &'a Limits,
    report: &'a mut LineReport,
    pos: usize,
    /// The current state, and its frame.
    state: u32,
    frame: Frame,
    /// Steps spent on this line, counted in scans too.
    steps: Cell<u64>,
    /// Steps this line may spend; `u64::MAX` for no limit.
    budget: u64,
    /// The first byte that is not white space: a table header, or a comment
    /// that gets the leading-space scope, starts here at the earliest.
    first: usize,
    /// The last `=` of the line, which a key must precede.
    pub(super) last_eq: Option<usize>,
    /// The last answer of a search for the first `"` that `\s*=` follows
    /// after a byte: the byte searched from and the answer, `usize::MAX` for
    /// none, which holds for every byte between them.
    pub(super) quote_eq: Cell<(usize, usize)>,
    /// No key that is only bare names, dots and white space starts before
    /// this byte, because one that started earlier failed where this ends.
    pub(super) no_key_before: usize,
    /// A key made of names with a quoted one among them failed to end in an
    /// `=`: the start of the next of its names and the byte where it failed.
    /// A key that starts at that name fails the same way.
    pub(super) failed_chain: Cell<(usize, usize)>,
}

impl<'a> Lexer<'a> {
    pub(crate) fn new(
        arena: &'a mut Arena,
        limits: &'a Limits,
        s: &'a [u8],
        out: &'a mut Vec<Token>,
        state: u32,
        report: &'a mut LineReport,
    ) -> Self {
        let frame = arena.frame(state);
        let budget = limits.work.map_or(u64::MAX, |work| work.steps(s.len()));
        let mut lexer = Lexer {
            s,
            out,
            arena,
            limits,
            report,
            pos: 0,
            state,
            frame,
            steps: Cell::new(0),
            budget,
            first: 0,
            last_eq: memrchr(b'=', s),
            quote_eq: Cell::new((0, 0)),
            no_key_before: 0,
            failed_chain: Cell::new((usize::MAX, 0)),
        };
        lexer.first = lexer.skip_space(0);
        lexer
    }

    pub(crate) fn into_state(self) -> u32 {
        self.state
    }

    pub(crate) fn run(&mut self) {
        let len = self.s.len();
        let mut p = 0;
        while p < len {
            self.spend(1);
            if self.steps.get() > self.budget {
                self.report.record(Limit::Work);
                break;
            }
            let from = p;
            p = match self.top() {
                Kind::Basic | Kind::BasicMulti => self.in_basic(p),
                Kind::Literal | Kind::LiteralMulti => self.in_literal(p),
                Kind::Top => self.in_table(p),
                Kind::Array => self.in_array(p),
                Kind::Inline => self.in_inline(p),
            };
            debug_assert!(p > from, "the TOML lexer made no progress at byte {from}");
        }
        self.gap(len);
    }

    // Bytes.

    /// The byte at `i`, or 0 past the end.
    pub(super) fn at(&self, i: usize) -> u8 {
        self.s.get(i).copied().unwrap_or(0)
    }

    /// The width of the character at `i`, at least 1.
    pub(super) fn width_at(&self, i: usize) -> usize {
        let Some(&lead) = self.s.get(i) else {
            return 1;
        };
        char_len(lead).min(self.s.len() - i)
    }

    /// How many bytes of white space, as the regex `\s` reads it, start at
    /// `i`; 0 when none does.
    pub(super) fn space_at(&self, i: usize) -> usize {
        let Some(&b) = self.s.get(i) else {
            return 0;
        };
        if b < 0x80 {
            return usize::from(matches!(b, b' ' | b'\t' | b'\n' | 0x0b | 0x0c | b'\r'));
        }
        let width = self.width_at(i);
        match self
            .s
            .get(i..i + width)
            .and_then(|bytes| std::str::from_utf8(bytes).ok())
            .and_then(|text| text.chars().next())
        {
            Some(c) if classes::is_space(c) => width,
            _ => 0,
        }
    }

    /// The end of the run of white space that starts at `i`.
    pub(super) fn skip_space(&self, mut i: usize) -> usize {
        let from = i;
        loop {
            let width = self.space_at(i);
            if width == 0 {
                self.spend(i - from);
                return i;
            }
            i += width;
        }
    }

    /// Counts `bytes` read ahead of the cursor against the line's work.
    pub(super) fn spend(&self, bytes: usize) {
        let bytes = u64::try_from(bytes).unwrap_or(u64::MAX);
        self.steps.set(self.steps.get().saturating_add(bytes));
    }

    // Emission.

    /// Emits plain text of the current context up to `to`.
    pub(super) fn gap(&mut self, to: usize) {
        self.tok(to, &[]);
    }

    /// Emits the bytes up to `to` with the current context plus `leaf`.
    pub(super) fn tok(&mut self, to: usize, leaf: &[S]) {
        if to <= self.pos {
            return;
        }
        let mut stack = self.frame.stack;
        for scope in leaf {
            let Some(child) = self.arena.child(stack, scope.scope()) else {
                self.report.record(Limit::Nodes);
                break;
            };
            stack = child;
        }
        self.out.push(Token {
            end: to,
            stack: Stack(stack),
        });
        self.pos = to;
    }

    /// Emits the bytes from `from` to `to` with `leaf`, after plain text up
    /// to `from`.
    pub(super) fn tok_at(&mut self, from: usize, to: usize, leaf: &[S]) {
        self.gap(from);
        self.tok(to, leaf);
    }

    // Contexts.

    pub(super) fn top(&self) -> Kind {
        Kind::from_code(self.frame.kind)
    }

    /// Opens a context; false when a limit keeps it closed.
    fn push(&mut self, kind: Kind) -> bool {
        let depth = usize::try_from(self.frame.depth).unwrap_or(usize::MAX);
        if self.limits.depth.is_some_and(|max| depth >= max) {
            self.report.record(Limit::Depth);
            return false;
        }
        let stack = match kind.scope() {
            Some(scope) => self.arena.child(self.frame.stack, scope.scope()),
            None => Some(self.frame.stack),
        };
        // An inline table starts where a comment may follow on a later line.
        let payload = u32::from(kind == Kind::Inline);
        let Some(state) =
            stack.and_then(|stack| self.arena.push(self.state, kind.code(), payload, stack))
        else {
            self.report.record(Limit::Nodes);
            return false;
        };
        self.enter(state);
        true
    }

    fn pop(&mut self) {
        self.enter(self.frame.parent);
    }

    fn enter(&mut self, state: u32) {
        self.state = state;
        self.frame = self.arena.frame(state);
    }

    /// The flag of the array or inline table being read: for an array,
    /// whether a value has just ended; for an inline table, whether a
    /// comment may start a line, which it may after `{` and `,` only.
    fn flag(&self) -> bool {
        self.frame.payload != 0
    }

    /// Sets the flag of the array or inline table being read.
    fn set_flag(&mut self, on: bool) {
        let payload = u32::from(on);
        if self.frame.payload == payload {
            return;
        }
        let frame = self.frame;
        match self
            .arena
            .push(frame.parent, frame.kind, payload, frame.stack)
        {
            Some(state) => self.enter(state),
            None => self.report.record(Limit::Nodes),
        }
    }

    /// Notes that a value, key or other text starts at the cursor, for the
    /// array or inline table around it.
    pub(super) fn value_starts(&mut self) {
        match self.top() {
            Kind::Array => self.set_flag(true),
            Kind::Inline => self.set_flag(false),
            Kind::Top | Kind::Basic | Kind::Literal | Kind::BasicMulti | Kind::LiteralMulti => {}
        }
    }

    /// Opens `kind` at `p`, whose begin text ends at `end` and carries
    /// `leaf`; the text stays plain when a limit keeps the context closed.
    pub(super) fn begin(&mut self, p: usize, end: usize, kind: Kind, leaf: S) -> usize {
        self.gap(p);
        if self.push(kind) {
            self.tok(end, &[leaf]);
        }
        end
    }

    /// Emits the end text up to `end` with `leaf` and closes the context.
    pub(super) fn finish(&mut self, p: usize, end: usize, leaf: S) -> usize {
        self.gap(p);
        self.tok(end, &[leaf]);
        self.pop();
        end
    }

    // Lines of a table.

    /// Takes the next match at or after `p` outside every array, inline
    /// table and string.
    fn in_table(&mut self, p: usize) -> usize {
        if self.space_at(p) > 0 {
            return self.space(p);
        }
        match self.s[p] {
            b'#' => return self.comment(p),
            b'[' if p == self.first => {
                if let Some(end) = self.header(p) {
                    return end;
                }
            }
            _ => {}
        }
        if let Some(end) = self.key_value(p) {
            return end;
        }
        self.invalid(p)
    }

    /// White space at `p`: the scope of a table header or comment that
    /// follows it on a line that starts with it, plain otherwise.
    fn space(&mut self, p: usize) -> usize {
        let end = self.skip_space(p);
        if p == 0 && end == self.first && end < self.s.len() {
            match self.s[end] {
                b'#' if self.blanks(end) => self.tok(end, &[S::CommentLeading]),
                b'[' => {
                    if let Some((_, double)) = self.header_end(end) {
                        let group = if double {
                            S::MetaGroupDouble
                        } else {
                            S::MetaGroup
                        };
                        self.tok(end, &[group]);
                    }
                }
                _ => {}
            }
        }
        end
    }

    /// Whether the line up to `end` is only spaces and tabs, as `[ \t]+` reads
    /// it; other white space does not get the leading-space scope.
    fn blanks(&self, end: usize) -> bool {
        self.s[..end].iter().all(|&b| b == b' ' || b == b'\t')
    }

    /// `\s*#.*`: a comment to the end of the line.
    fn comment(&mut self, p: usize) -> usize {
        let len = self.s.len();
        self.tok_at(p, p + 1, &[S::Comment, S::PunctComment]);
        self.tok(len, &[S::Comment]);
        len
    }

    /// A run of text that is none of the rules' matches: `\S+` and the white
    /// space after it, unless that ends the line.
    pub(super) fn invalid(&mut self, p: usize) -> usize {
        let mut end = p;
        while end < self.s.len() && self.space_at(end) == 0 {
            end += self.width_at(end);
        }
        let spaced = self.skip_space(end);
        if spaced < self.s.len() {
            end = spaced;
        }
        self.spend(end - p);
        self.tok_at(p, end, &[S::Invalid]);
        end
    }

    /// Where the table header at `p` ends, and whether it is `[[…]]`.
    ///
    /// `[` or `[[`, text without brackets, then `]` or `]]`.
    fn header_end(&self, p: usize) -> Option<(usize, bool)> {
        let double = self.at(p + 1) == b'[';
        let start = p + if double { 2 } else { 1 };
        let rest = self.s.get(start..)?;
        let offset = memchr2(b'[', b']', rest)?;
        self.spend(offset);
        if rest[offset] != b']' {
            return None;
        }
        let close = start + offset;
        if !double {
            return Some((close + 1, false));
        }
        (self.at(close + 1) == b']').then_some((close + 2, true))
    }

    /// A table header at `p`, where the line's first character is `[`.
    fn header(&mut self, p: usize) -> Option<usize> {
        let (end, double) = self.header_end(p)?;
        let group = if double {
            S::MetaGroupDouble
        } else {
            S::MetaGroup
        };
        let open = p + if double { 2 } else { 1 };
        let close = end - if double { 2 } else { 1 };
        self.tok(open, &[group, S::PunctSection]);
        let mut i = open;
        while i < close {
            let width = self.space_at(i);
            if width > 0 {
                i += width;
                continue;
            }
            if self.s[i] == b'.' {
                i += 1;
                continue;
            }
            let from = i;
            while i < close && self.space_at(i) == 0 && self.s[i] != b'.' {
                i += self.width_at(i);
            }
            self.tok(from, &[group]);
            self.tok(i, &[group, S::SectionName]);
        }
        self.tok(close, &[group]);
        self.tok(end, &[group, S::PunctSection]);
        Some(end)
    }

    // Arrays and inline tables.

    /// Takes the next match at or after `p` in an array.
    fn in_array(&mut self, p: usize) -> usize {
        if self.space_at(p) > 0 {
            return self.leading_space(p);
        }
        match self.s[p] {
            b']' => return self.finish(p, p + 1, S::PunctArrayEnd),
            b'#' => return self.comment(p),
            b',' if self.flag() => {
                self.tok_at(p, p + 1, &[S::PunctArraySep]);
                self.set_flag(false);
                return p + 1;
            }
            _ if !self.flag() => {
                if let Some(end) = self.value(p) {
                    return end;
                }
            }
            _ => {}
        }
        self.invalid(p)
    }

    /// White space in an array or inline table, which is the leading-space
    /// scope when it precedes a comment on a line that starts with it, though
    /// an inline table has no comments.
    fn leading_space(&mut self, p: usize) -> usize {
        let end = self.skip_space(p);
        let comments = self.top() == Kind::Array || self.flag();
        if p == 0 && end == self.first && self.at(end) == b'#' && comments && self.blanks(end) {
            self.tok(end, &[S::CommentLeading]);
        }
        end
    }

    /// Takes the next match at or after `p` in an inline table.
    fn in_inline(&mut self, p: usize) -> usize {
        if self.space_at(p) > 0 {
            return self.leading_space(p);
        }
        match self.s[p] {
            b'}' => return self.finish(p, p + 1, S::PunctInlineEnd),
            b',' => {
                self.tok_at(p, p + 1, &[S::PunctInlineSep]);
                self.set_flag(true);
                return p + 1;
            }
            b'#' if p > 0 && p == self.first && self.flag() && self.blanks(p) => {
                return self.comment(p);
            }
            _ => {}
        }
        self.value_starts();
        if let Some(end) = self.key_value(p) {
            return end;
        }
        p + self.width_at(p)
    }
}
