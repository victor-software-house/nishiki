//! The per-line lexer: cursor, token emission, the context stack and the
//! character classes the grammar's regular expressions spell out.

use std::cell::Cell;

use super::Dialect;
use super::bytes::Bytes;
use super::chars::is_alpha_char;
use super::kind::Kind;
use super::scope::S;
use crate::arena::{Arena, Frame};
use crate::classes::is_ascii_space;
use crate::language::{Context, Halt};
use crate::limits::{Finished, Limit, Limits, LineReport, Steps};
use crate::{Stack, Token, classes};

/// Lexes one line. `pos` is the end of the last token emitted; the bytes
/// between it and a rule's match are plain text of the current context.
///
/// The grammar was written for `TextMate`, which tokenizes each line with a
/// newline appended. [`Lexer::byte`] therefore reads a newline at the end of
/// the line, so `\s` and `$` behave as they do there.
pub(crate) struct Lexer<'a, const D: u8> {
    pub(crate) src: &'a str,
    pub(crate) bytes: &'a [u8],
    pub(crate) len: usize,
    out: &'a mut Vec<Token>,
    arena: &'a mut Arena,
    limits: &'a Limits,
    report: &'a mut LineReport,
    pos: usize,
    /// The current state, and its frame.
    state: u32,
    frame: Frame,
    /// Steps spent on this line, counted in `&self` scans too.
    steps: Steps,
    /// The contexts opened on this line and still open, innermost last: the
    /// top frames of the stack.
    opened: &'a mut Vec<Opened>,
    /// The end of the last rule match, which `TextMate` calls the line
    /// position.
    pub(crate) line_pos: usize,
    /// The end of the innermost begin match, where `\G` matches.
    pub(crate) anchor: usize,
    /// Whether this is the first line of the document, where `\A` matches.
    pub(crate) first_line: bool,
    /// Set when the grammar loops: a context was opened and closed without the
    /// line advancing. `TextMate` then gives the rest of the line to the
    /// context that is open.
    stuck: bool,
    /// Where the language that embeds this one takes the line back, if it
    /// does.
    pub(crate) halts: Option<Halts<'a>>,
    /// Where `halts` stopped the line.
    halt: Option<usize>,
    /// Set when a context was refused for depth; recorded at the line's end.
    depth_hit: Cell<bool>,
    /// The last run of blanks that [`Lexer::ws`] scanned, as start and end.
    pub(crate) blanks: Cell<(usize, usize)>,
    /// The last name [`Lexer::ident`] scanned, as its start and its end.
    pub(crate) last_ident: Cell<(usize, Option<usize>)>,
    /// The position of the last backtick in the text, if any.
    last_tick: Option<usize>,
    /// The position of the last `(` or `<` in the text, if any.
    last_open: Option<usize>,
    /// Where the `=>` that ends the line, before trailing blanks, begins.
    arrow_at_end: Option<usize>,
    /// The last search for a `]`: where it started and the `]` it found.
    next_bracket: Cell<(usize, Option<usize>)>,
}

/// The tests of the language that embeds this one, on the line: `end`
/// before this grammar's rules and `after` where none matches (see
/// [`lex_until`](super::lex_until)).
#[derive(Clone, Copy)]
pub(crate) struct Halts<'a> {
    pub(crate) end: &'a dyn Halt,
    pub(crate) after: Option<&'a dyn Halt>,
}

/// A context opened on the current line, which `TextMate` remembers to
/// catch a grammar that loops.
///
/// Only the contexts opened on the line need this, and they are always the
/// top of the stack: a frame from an earlier line is popped only when nothing
/// opened on the line sits above it. The highlighter keeps the stack across
/// lines, so a line clears it rather than filling a table per depth.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Opened {
    /// The context's kind code.
    kind: u16,
    /// The line position where it was entered.
    enter: usize,
    /// The `\G` anchor in force before it began.
    anchor: usize,
}

impl<'a, const D: u8> Lexer<'a, D> {
    /// The grammar followed: TypeScript, or one that reads JSX. A parameter of
    /// the type, so the TypeScript lexer compiles with no JSX branches.
    pub(crate) const DIALECT: Dialect = Dialect::of(D);

    /// Whether the text is a capture group that another grammar lexes again,
    /// which `TextMate` cuts from the line without the line feed it appends
    /// to every line. A const, so the lexers of whole lines pay nothing for
    /// it.
    const BARE: bool = D & Dialect::BARE != 0;

    /// The lexer of the line `src` from byte `from` and `state`, which `halts`
    /// may stop. The bytes before `from` belong to the language that embeds
    /// this one: the scan never starts there, but lookbehinds read them, and
    /// `^` matches only at byte 0, as in `TextMate`, where an embedded grammar
    /// sees the whole line.
    ///
    /// Everything on the first line opens above a `Started` marker, so the
    /// document start is never reached again and `\A` matches once.
    pub(crate) fn new(
        cx: &'a mut Context<'_>,
        src: &'a str,
        from: usize,
        state: u32,
        halts: Option<Halts<'a>>,
    ) -> Self {
        let frame = cx.arena.frame(state);
        let limits = cx.limits;
        let text = src.len().saturating_sub(from);
        let steps = Steps::new(limits, text);
        let line = cx.scratch.line();
        let scratch = &mut cx.scratch.typescript;
        scratch.opened.clear();
        let (last_tick, last_open, read) = scratch.searches.last(line, src.as_bytes(), from);
        let mut lexer = Lexer {
            src,
            bytes: src.as_bytes(),
            len: src.len(),
            out: &mut *cx.out,
            arena: &mut *cx.arena,
            limits,
            report: &mut *cx.report,
            pos: from,
            state,
            frame,
            steps,
            opened: &mut scratch.opened,
            line_pos: from,
            anchor: usize::MAX,
            first_line: frame.kind == Kind::Top.code() && frame.depth == 1,
            stuck: false,
            halts,
            halt: None,
            depth_hit: Cell::new(false),
            blanks: Cell::new((0, 0)),
            last_ident: Cell::new((usize::MAX, None)),
            last_tick,
            last_open,
            next_bracket: Cell::new((usize::MAX, None)),
            arrow_at_end: {
                let text = src.trim_end_matches(classes::is_space);
                text.ends_with("=>").then(|| text.len() - 2)
            },
        };
        lexer.spend(read);
        if lexer.first_line {
            lexer.push(Kind::Started);
        }
        lexer
    }

    /// The state after the line, with its count given back to the lender.
    pub(crate) fn finish_line(self) -> Finished {
        self.steps.finish(self.state)
    }

    /// `^` at `p`: only the line's first byte, which an embedded text that
    /// starts later never reaches.
    pub(crate) fn bol(p: usize) -> bool {
        p == 0
    }

    /// Counts `bytes` read ahead of the cursor against the line's work.
    pub(crate) fn spend(&self, bytes: usize) {
        self.steps.spend(bytes);
    }

    /// Whether a `(` or `<` lies at or after `p`, which every call lookahead
    /// needs after the name.
    pub(crate) fn has_open_after(&self, p: usize) -> bool {
        self.last_open.is_some_and(|at| at >= p)
    }

    /// The first `]` at or after `i` on the line. A run of `[` asks at each
    /// one, so the answer is kept while it still holds, and the line is
    /// scanned once.
    pub(crate) fn bracket_from(&self, i: usize) -> Option<usize> {
        let (from, found) = self.next_bracket.get();
        if from <= i && found.is_none_or(|at| at >= i) {
            return found;
        }
        let rest = self.bytes.get(i..).unwrap_or_default();
        let found = memchr::memchr(b']', rest).map(|at| i + at);
        self.spend(found.map_or(rest.len(), |at| at - i + 1));
        self.next_bracket.set((i, found));
        found
    }

    /// Whether the line ends, before trailing blanks, in a `=>` that begins at
    /// or after `p`: `\s*.*=>\s*$` from `p`.
    pub(crate) fn arrow_ends_line_after(&self, p: usize) -> bool {
        self.arrow_at_end.is_some_and(|at| at >= p)
    }

    /// Whether a backtick lies at or after `p`.
    pub(crate) fn has_backtick_after(&self, p: usize) -> bool {
        self.last_tick.is_some_and(|t| t >= p)
    }

    /// Lexes the line.
    ///
    /// A step that matches without advancing either opens a context, which
    /// cannot open twice at one position (see [`Lexer::push_with`]), or closes
    /// one opened before that position (see [`Lexer::close_here`]). So a run of
    /// steps at one position is at most the depth when the run began plus the
    /// number of kinds; debug builds assert that bound.
    ///
    /// `TextMate` keeps matching after a rule has consumed the newline, at the
    /// end of the line, where a rule that tests for `$` can still end.
    pub(crate) fn run(&mut self) {
        let mut p = self.line_pos;
        #[cfg(debug_assertions)]
        let (mut still, mut bound) = (0usize, 0usize);
        while p <= self.window() && !self.stuck && self.halt.is_none() {
            self.spend(1);
            if self.steps.exhausted() {
                self.report.record(Limit::Work);
                break;
            }
            let (from, before) = (p, self.state);
            if let Some(next) = self.step(p) {
                self.line_pos = next;
                p = next;
            } else if !self.halt_if(p, self.halts.and_then(|halts| halts.after)) {
                p = self.skip_unmatched(p);
            }
            debug_assert!(
                p > from || self.state != before || self.stuck || self.halt.is_some(),
                "the TypeScript lexer made no progress at byte {from}"
            );
            #[cfg(debug_assertions)]
            if p > from {
                still = 0;
            } else {
                if still == 0 {
                    let depth = usize::try_from(self.frame.depth).unwrap_or(usize::MAX);
                    bound = depth.saturating_add(Kind::COUNT);
                }
                still += 1;
                debug_assert!(
                    still <= bound,
                    "the TypeScript lexer took {still} steps at byte {from} without advancing"
                );
            }
        }
        if self.depth_hit.get() {
            self.report.record(Limit::Depth);
        }
        self.flush(self.halt.unwrap_or(self.len));
    }

    /// Stops the line at `p` when `test`, one of the embedding language's,
    /// holds there while no context of the code is open beyond the one the
    /// embedding started.
    pub(crate) fn halt_if(&mut self, p: usize, test: Option<&dyn Halt>) -> bool {
        let Some(test) = test else {
            return false;
        };
        if p > self.len
            || !matches!(
                self.top(),
                Kind::Top
                    | Kind::Started
                    | Kind::Script
                    | Kind::Expression
                    | Kind::EntryObjLit
                    | Kind::EntryObjBind
                    | Kind::EntryObjBindConst
                    | Kind::EntryArrBind
                    | Kind::EntryArrBindConst
                    | Kind::EntryTypeParams
                    | Kind::EntryTemplate
            )
            || !test.holds(self.bytes, p)
        {
            return false;
        }
        self.halt = Some(p);
        true
    }

    /// Where the embedding language took the line back, if it did.
    pub(crate) fn halted_at(&self) -> Option<usize> {
        self.halt
    }

    /// Where the scan resumes after no rule matched at `p`: the next position
    /// a rule can match at.
    ///
    /// Inside a run of blanks, or of the name characters after a name's first
    /// letter and the character after it, the next positions all have the
    /// same kind of character before them as `p`, so the same lookbehinds
    /// give the same answers, and a rule without a lookbehind would have
    /// matched at the run's or the name's start. The first position of a run
    /// and the second of a name are not skipped, because a lookbehind that
    /// failed at the first (`(?<![$_[:alnum:]])` after a name) can pass at the
    /// next. The scan still reaches `${`, which starts with a name character,
    /// and the last character of the line, which the end rules that test
    /// `[^\\\n]$` match. The oracle fixtures and the differential tests hold
    /// that nothing else matches in what is skipped.
    fn skip_unmatched(&mut self, p: usize) -> usize {
        let step = p + self.char_len(p);
        if p < self.len
            && let Some(target) = self.body_end(step)
        {
            #[cfg(feature = "lang-svelte")]
            let target = self.svelte_body_end(step, target);
            #[cfg(debug_assertions)]
            self.check_skipped(step, target);
            return target;
        }
        let next = if p >= self.len || p == 0 || self.halts.is_some() {
            return step;
        } else if is_ascii_space(self.byte(p)) {
            if !is_ascii_space(self.byte(p - 1)) {
                return step;
            }
            self.ws(p)
        } else if self.alnum_at(p) && self.second_of_letter_name(p) {
            let mut q = step;
            while q < self.len && self.byte(q) != b'$' && self.alnum_at(q) {
                q += self.char_len(q);
            }
            q
        } else {
            return step;
        };
        let last = self.src.char_indices().next_back().map_or(0, |(i, _)| i);
        if p < last { next.min(last) } else { step }
    }

    /// Where the plain text of a comment or string that holds `p` ends: the
    /// next position a rule of that context can match at, after `step`.
    ///
    /// A line comment has no rule before the line's end. A block comment
    /// ends at `*/`, and a doc comment also reads `@`, `[` and `{`. The
    /// comments of a script also end before `</script`. A string
    /// ends at its quote or the line's last character, and reads `\`; a
    /// template ends at its backtick and reads `\` and `${`.
    fn body_end(&self, step: usize) -> Option<usize> {
        let rest = self.bytes.get(step..)?;
        let kind = self.top();
        let end = rest.len();
        let found = if matches!(kind, Kind::CommentLine | Kind::CommentLineNl) {
            end
        } else if kind == Kind::CommentBlock {
            memchr::memchr(b'*', rest).unwrap_or(end)
        } else if kind == Kind::ScriptLineComment {
            memchr::memchr(b'<', rest).unwrap_or(end)
        } else if kind == Kind::ScriptBlockComment {
            memchr::memchr2(b'*', b'<', rest).unwrap_or(end)
        } else if kind == Kind::CommentDoc {
            let first = memchr::memchr3(b'*', b'@', b'[', rest).unwrap_or(end);
            memchr::memchr(b'{', &rest[..first]).unwrap_or(first)
        } else if kind == Kind::StrSingle {
            memchr::memchr2(b'\'', b'\\', rest).unwrap_or(end)
        } else if kind == Kind::StrDouble {
            memchr::memchr2(b'"', b'\\', rest).unwrap_or(end)
        } else if matches!(kind, Kind::Template | Kind::TemplateType) {
            memchr::memchr3(b'`', b'$', b'\\', rest).unwrap_or(end)
        } else {
            return None;
        };
        let mut target = step + found;
        if matches!(kind, Kind::StrSingle | Kind::StrDouble) {
            let last = self.src.char_indices().next_back().map_or(0, |(i, _)| i);
            target = target.min(last);
        }
        (target > step).then_some(target)
    }

    /// Debug builds confirm that no rule matches at a skipped position.
    #[cfg(debug_assertions)]
    fn check_skipped(&mut self, from: usize, to: usize) {
        let mut q = from;
        while q < to {
            debug_assert!(
                self.step(q).is_none(),
                "a rule matched at byte {q}, which the skip passed over"
            );
            q += self.char_len(q);
        }
    }

    /// Whether the character before `p` is the first of a name that starts
    /// with a letter, `$` or `_`, and is alone before `p`.
    fn second_of_letter_name(&self, p: usize) -> bool {
        let Some(first) = self.src.get(..p).and_then(|head| head.chars().next_back()) else {
            return false;
        };
        is_alpha_char(first) && !self.alnum_before(p - first.len_utf8())
    }

    /// Runs `rule` at `p` when the byte there is in `starts`, the bytes the
    /// rule can match at; a rule that cannot start there is not called.
    ///
    /// A debug build calls the rule anyway and fails when it matches at a byte
    /// outside its set, so a set that is too small is found by the tests.
    pub(crate) fn only(
        &mut self,
        name: &'static str,
        starts: Bytes,
        p: usize,
        rule: impl FnOnce(&mut Self, usize) -> Option<usize>,
    ) -> Option<usize> {
        let byte = self.byte(p);
        if cfg!(debug_assertions) {
            let result = rule(self, p);
            debug_assert!(
                result.is_none() || starts.has(byte),
                "rule {name} matched at byte {byte:#04x}, outside its start set"
            );
            result
        } else if starts.has(byte) {
            rule(self, p)
        } else {
            None
        }
    }

    /// The byte at `i`: the newline at the end of the line when one follows
    /// (see [`Lexer::BARE`]), 0 beyond.
    pub(crate) fn byte(&self, i: usize) -> u8 {
        match i.cmp(&self.len) {
            std::cmp::Ordering::Less => self.bytes[i],
            std::cmp::Ordering::Equal if Self::BARE => 0,
            std::cmp::Ordering::Equal => b'\n',
            std::cmp::Ordering::Greater => 0,
        }
    }

    /// The end of every scan ahead: the line, with its newline when one
    /// follows. Every step of a scan is counted against the line's work.
    pub(crate) fn window(&self) -> usize {
        self.len + usize::from(!Self::BARE)
    }

    /// `base` with `scopes` pushed; stops short when the node limit is
    /// reached.
    fn node(&mut self, mut base: u32, scopes: &[S]) -> u32 {
        for scope in scopes {
            let Some(child) = self.arena.child(base, scope.scope(Self::DIALECT)) else {
                self.report.record(Limit::Nodes);
                break;
            };
            base = child;
        }
        base
    }

    fn emit(&mut self, to: usize, stack: u32) {
        let to = to.min(self.len);
        if to > self.pos {
            self.out.push(Token {
                end: to,
                stack: Stack(stack),
            });
            self.pos = to;
        }
    }

    /// Emits plain text of the current context up to `to`.
    pub(crate) fn flush(&mut self, to: usize) {
        self.emit(to, self.frame.stack);
    }

    /// Emits `a..b` as a capture with `leaf` scopes inside the match of the
    /// rule `kind`; the bytes before `a` carry the rule's own scopes.
    pub(crate) fn capture(&mut self, kind: Kind, a: usize, b: usize, leaf: &[S]) {
        let base = self.node(self.frame.stack, kind.names());
        self.emit(a, base);
        let stack = self.node(base, leaf);
        self.emit(b, stack);
    }

    /// Emits what is left of the match up to `to` with `leaf` inside the rule
    /// `kind`: the uncaptured part of a match that the rule scopes as a whole.
    pub(crate) fn fill(&mut self, kind: Kind, to: usize, leaf: &[S]) {
        self.capture(kind, self.pos, to, leaf);
    }

    /// Emits the rest of a match of the rule `kind` up to `to`.
    pub(crate) fn finish(&mut self, kind: Kind, to: usize) {
        self.capture(kind, to, to, &[]);
    }

    /// A match of a rule without scopes of its own: plain text up to `a`, then
    /// `a..b` with `leaf`.
    pub(crate) fn token(&mut self, a: usize, b: usize, leaf: &[S]) {
        self.flush(a);
        self.capture(Kind::Top, a, b, leaf);
    }

    /// Opens `kind` at the end of its begin tokens; false when a limit keeps
    /// it closed. Callers emit the begin tokens first.
    pub(crate) fn push(&mut self, kind: Kind) -> bool {
        self.push_with(kind, 0)
    }

    /// Opens `kind` carrying `payload`.
    ///
    /// `TextMate` refuses to open a context a second time at a position the
    /// line has not advanced past, and gives the rest of the line to the open
    /// one.
    pub(crate) fn push_with(&mut self, kind: Kind, payload: u32) -> bool {
        if self.pos <= self.line_pos {
            let here = self.opened.iter().rev();
            let mut here = here.take_while(|opened| opened.enter == self.line_pos);
            if here.any(|opened| opened.kind == kind.code()) {
                self.stuck = true;
                return false;
            }
        }
        if self.full() {
            return false;
        }
        let outer = self.node(self.frame.stack, kind.names());
        let stack = self.node(outer, kind.content());
        let Some(state) = self.arena.push(self.state, kind.code(), payload, stack) else {
            self.report.record(Limit::Nodes);
            return false;
        };
        self.opened.push(Opened {
            kind: kind.code(),
            enter: self.line_pos,
            anchor: self.anchor,
        });
        self.anchor = self.pos;
        self.set_state(state);
        true
    }

    fn set_state(&mut self, state: u32) {
        self.state = state;
        self.frame = self.arena.frame(state);
    }

    /// The innermost open context.
    pub(crate) fn top(&self) -> Kind {
        let kind = Kind::from_code(self.frame.kind);
        if kind == Kind::Started {
            Kind::Top
        } else {
            kind
        }
    }

    /// The data the innermost context carries.
    pub(crate) fn payload(&self) -> u32 {
        self.frame.payload
    }

    pub(crate) fn pop(&mut self) {
        self.anchor = self.opened.pop().map_or(usize::MAX, |opened| opened.anchor);
        self.set_state(self.frame.parent);
    }

    /// Ends the innermost context where `end` matches: `a..b` carries the
    /// context's own scopes and `leaf`. Plain text before `a` stays inside.
    pub(crate) fn close(&mut self, a: usize, b: usize, leaf: &[S]) -> usize {
        let kind = self.top();
        self.flush(a);
        self.pop();
        self.capture(kind, a, b, leaf);
        b
    }

    /// Ends the innermost context with a zero-width match at `p`.
    ///
    /// A context closed where it opened, without the line advancing, is a loop
    /// in the grammar; `TextMate` leaves it open (see `stuck`).
    pub(crate) fn close_here(&mut self, p: usize) -> usize {
        self.flush(p);
        let here = self.opened.last().map(|opened| opened.enter);
        if p <= self.line_pos && here == Some(self.line_pos) {
            self.stuck = true;
            return p;
        }
        self.pop();
        p
    }
}

impl<const D: u8> Lexer<'_, D> {
    /// Whether a context cannot open because the stack is full.
    pub(crate) fn full(&self) -> bool {
        let depth = usize::try_from(self.frame.depth).unwrap_or(usize::MAX);
        let full = self.limits.depth.is_some_and(|max| depth >= max);
        if full {
            self.depth_hit.set(true);
        }
        full
    }

    /// A rule whose begin is the single token `a..b` with `leaf`: emits it and
    /// opens `kind`.
    pub(crate) fn open_token(
        &mut self,
        kind: Kind,
        a: usize,
        b: usize,
        leaf: &[S],
    ) -> Option<usize> {
        if self.full() {
            return None;
        }
        self.flush(a);
        self.capture(kind, a, b, leaf);
        self.push(kind);
        Some(b)
    }

    /// A zero-width begin at `p`: opens `kind` without emitting a token.
    pub(crate) fn open_empty(&mut self, kind: Kind, p: usize) -> Option<usize> {
        if self.full() {
            return None;
        }
        self.flush(p);
        self.push(kind);
        Some(p)
    }

    /// A match of a rule with no scopes of its own and no context: the token
    /// `a..b` with `leaf`.
    #[expect(
        clippy::unnecessary_wraps,
        reason = "a rule function returns `Option<usize>`, and this is its tail"
    )]
    pub(crate) fn token_rule(&mut self, a: usize, b: usize, leaf: S) -> Option<usize> {
        self.token(a, b, &[leaf]);
        Some(b)
    }
}
