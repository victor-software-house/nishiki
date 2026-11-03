//! The per-line lexer: cursor, token emission, the context stack and the
//! character classes the grammar's regular expressions spell out.

use std::cell::Cell;

use super::Dialect;
use super::bytes::Bytes;
use super::kind::Kind;
use super::scope::S;
use crate::arena::{Arena, Frame};
use crate::language::Context;
use crate::limits::{Limit, Limits, LineReport};
use crate::{Stack, Token};

/// Lexes one line. `pos` is the end of the last token emitted; the bytes
/// between it and a rule's match are plain text of the current context.
///
/// The grammar was written for `TextMate`, which tokenizes each line with a
/// newline appended. [`Lexer::b`] therefore reads a newline at the end of the
/// line, so `\s` and `$` behave as they do there.
pub(crate) struct Lexer<'a, const D: u8> {
    pub(crate) src: &'a str,
    pub(crate) s: &'a [u8],
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
    steps: Cell<u64>,
    /// Steps this line may spend; `u64::MAX` for no limit.
    budget: u64,
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
    /// Where a `}` ended the interpolation that the line began inside.
    halt: Option<usize>,
    /// Whether the text starts the line, where `^` matches: not when a
    /// Markdown container took the start of the line.
    bol: bool,
    /// Set when a context was refused for depth; recorded at the line's end.
    depth_hit: Cell<bool>,
    /// The last run of blanks that [`Lexer::ws`] scanned, as start and end.
    blanks: Cell<(usize, usize)>,
    /// The last name [`Lexer::ident`] scanned, as its start and its end.
    last_ident: Cell<(usize, Option<usize>)>,
    /// The position of the last backtick on the line, if any.
    last_tick: Option<usize>,
    /// The position of the last `(` or `<` on the line, if any.
    last_open: Option<usize>,
}

/// For each ASCII byte: bit 0 is `[$_[:alnum:]]`, bit 1 is `[$_[:alpha:]]`.
const CLASS: [u8; 128] = {
    let mut table = [0; 128];
    let mut b = 0;
    while b < 128 {
        // `$`, `_`, `A`-`Z` and `a`-`z` are letters; `0`-`9` are digits.
        let letter = b == 36 || b == 95 || (b >= 65 && b <= 90) || (b >= 97 && b <= 122);
        if letter {
            table[b] = 3;
        } else if b >= 48 && b <= 57 {
            table[b] = 1;
        }
        b += 1;
    }
    table
};

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

pub(crate) fn is_ws(b: u8) -> bool {
    matches!(b, b' ' | b'\t' | b'\n' | b'\r' | 0x0b | 0x0c)
}

/// `[$_[:alnum:]]`.
pub(crate) fn is_alnum_char(c: char) -> bool {
    c == '$' || c == '_' || c.is_alphanumeric()
}

/// `[$_[:alpha:]]`.
pub(crate) fn is_alpha_char(c: char) -> bool {
    c == '$' || c == '_' || c.is_alphabetic()
}

impl<'a, const D: u8> Lexer<'a, D> {
    /// The grammar followed: TypeScript, or one that reads JSX. A parameter of
    /// the type, so the TypeScript lexer compiles with no JSX branches.
    pub(crate) const DIALECT: Dialect = Dialect::of(D);

    pub(crate) fn new(cx: &'a mut Context<'_>, src: &'a str, bol: bool, state: u32) -> Self {
        let frame = cx.arena.frame(state);
        let limits = cx.limits;
        let budget = limits.work.map_or(u64::MAX, |work| work.steps(src.len()));
        let opened = &mut cx.scratch.typescript;
        opened.clear();
        let mut lexer = Lexer {
            src,
            s: src.as_bytes(),
            len: src.len(),
            out: &mut *cx.out,
            arena: &mut *cx.arena,
            limits,
            report: &mut *cx.report,
            pos: 0,
            state,
            frame,
            steps: Cell::new(0),
            budget,
            opened,
            line_pos: 0,
            anchor: usize::MAX,
            first_line: frame.kind == Kind::Top.code() && frame.depth == 1,
            stuck: false,
            halt: None,
            bol,
            depth_hit: Cell::new(false),
            blanks: Cell::new((0, 0)),
            last_ident: Cell::new((usize::MAX, None)),
            last_tick: memchr::memrchr(b'`', src.as_bytes()),
            last_open: memchr::memrchr2(b'(', b'<', src.as_bytes()),
        };
        lexer.spend(src.len());
        if lexer.first_line {
            // Everything on the first line opens above this marker, so the
            // document start is never reached again and `\A` matches once.
            lexer.push(Kind::Started);
        }
        lexer
    }

    /// The state after the line.
    pub(crate) fn into_state(self) -> u32 {
        self.state
    }

    /// Counts `bytes` read ahead of the cursor against the line's work.
    /// `^` at `p`.
    pub(crate) fn bol(&self, p: usize) -> bool {
        p == 0 && self.bol
    }

    pub(crate) fn spend(&self, bytes: usize) {
        let bytes = u64::try_from(bytes).unwrap_or(u64::MAX);
        self.steps.set(self.steps.get().saturating_add(bytes));
    }

    /// Whether a `(` or `<` lies at or after `p`, which every call lookahead
    /// needs after the name.
    pub(crate) fn has_open_after(&self, p: usize) -> bool {
        self.last_open.is_some_and(|at| at >= p)
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
    pub(crate) fn run(&mut self) {
        let mut p = 0;
        #[cfg(debug_assertions)]
        let (mut still, mut bound) = (0usize, 0usize);
        // `TextMate` keeps matching after a rule has consumed the newline, at
        // the end of the line, where a rule that tests for `$` can still end.
        while p <= self.len + 1 && !self.stuck {
            self.spend(1);
            if self.steps.get() > self.budget {
                self.report.record(Limit::Work);
                break;
            }
            let (from, before) = (p, self.state);
            if let Some(next) = self.step(p) {
                self.line_pos = next;
                p = next;
            } else {
                p = self.skip_unmatched(p);
            }
            debug_assert!(
                p > from || self.state != before || self.stuck,
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

    /// The end of an interpolation, which the lexer that embeds this one
    /// consumes: a `}` that no context of the line is waiting for stops the
    /// line before it.
    pub(crate) fn halt_at_brace(&mut self, p: usize) -> Option<usize> {
        if p < self.len && self.b(p) == b'}' {
            self.halt = Some(p);
            self.stuck = true;
            return Some(p);
        }
        None
    }

    /// Where an unbalanced `}` ended an interpolation, if the line has one.
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
            #[cfg(debug_assertions)]
            self.check_skipped(step, target);
            return target;
        }
        let next = if p >= self.len || p == 0 {
            return step;
        } else if is_ws(self.b(p)) {
            if !is_ws(self.b(p - 1)) {
                return step;
            }
            self.ws(p)
        } else if self.alnum_at(p) && self.second_of_letter_name(p) {
            let mut q = step;
            while q < self.len && self.b(q) != b'$' && self.alnum_at(q) {
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
    /// ends at `*/`, and a doc comment also reads `@`, `[` and `{`. A string
    /// ends at its quote or the line's last character, and reads `\`; a
    /// template ends at its backtick and reads `\` and `${`.
    fn body_end(&self, step: usize) -> Option<usize> {
        let rest = self.s.get(step..)?;
        let kind = self.top();
        let end = rest.len();
        let found = if matches!(kind, Kind::CommentLine | Kind::CommentLineNl) {
            end
        } else if kind == Kind::CommentBlock {
            memchr::memchr(b'*', rest).unwrap_or(end)
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
        let byte = self.b(p);
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

    // Bytes and characters.

    /// The byte at `i`: a newline at the end of the line, 0 beyond.
    pub(crate) fn b(&self, i: usize) -> u8 {
        match i.cmp(&self.len) {
            std::cmp::Ordering::Less => self.s[i],
            std::cmp::Ordering::Equal => b'\n',
            std::cmp::Ordering::Greater => 0,
        }
    }

    /// The width of the character at `i`, 1 at or past the end.
    pub(crate) fn char_len(&self, i: usize) -> usize {
        self.spend(1);
        self.src
            .get(i..)
            .and_then(|rest| rest.chars().next())
            .map_or(1, char::len_utf8)
    }

    pub(crate) fn char_at(&self, i: usize) -> Option<char> {
        self.src.get(i..).and_then(|rest| rest.chars().next())
    }

    /// The character before `i`; the newline that ends the line is the one
    /// before `len + 1`.
    pub(crate) fn prev_char(&self, i: usize) -> Option<char> {
        if i == self.len + 1 {
            return Some('\n');
        }
        self.src.get(..i).and_then(|head| head.chars().next_back())
    }

    pub(crate) fn starts(&self, i: usize, lit: &str) -> bool {
        self.s
            .get(i..)
            .is_some_and(|rest| rest.starts_with(lit.as_bytes()))
    }

    /// Whether the line, with its newline, ends at or before `i`.
    pub(crate) fn eol(&self, i: usize) -> bool {
        i >= self.len
    }

    /// Skips `\s*`.
    ///
    /// Rules ask for the end of a run of blanks at every position inside it,
    /// so the last run found is remembered and a long run costs one scan.
    pub(crate) fn ws(&self, i: usize) -> usize {
        let (start, end) = self.blanks.get();
        if i >= start && i < end {
            return end;
        }
        let mut j = i;
        loop {
            let b = self.b(j);
            if is_ws(b) {
                j += 1;
            } else if b >= 0xc2
                && let Some(c) = self.src.get(j..).and_then(|rest| rest.chars().next())
                && c.is_whitespace()
            {
                j += c.len_utf8();
            } else {
                break;
            }
        }
        self.spend(j - i + 1);
        self.blanks.set((i, j));
        j
    }

    /// The end of the scan window that starts at `i`: the line, with its
    /// newline. Every step of a scan is counted against the line's work.
    pub(crate) fn window(&self, _i: usize) -> usize {
        self.len + 1
    }

    /// `[$_[:alpha:]]` at `i`.
    pub(crate) fn id_start(&self, i: usize) -> bool {
        match self.s.get(i) {
            Some(&b) if b < 0x80 => CLASS[usize::from(b)] & 2 != 0,
            Some(_) => self.char_at(i).is_some_and(is_alpha_char),
            None => false,
        }
    }

    /// `[$_[:alnum:]]` at `i`.
    pub(crate) fn alnum_at(&self, i: usize) -> bool {
        match self.s.get(i) {
            Some(&b) if b < 0x80 => CLASS[usize::from(b)] & 1 != 0,
            Some(_) => self.char_at(i).is_some_and(is_alnum_char),
            None => false,
        }
    }

    /// `[$_[:alnum:]]` just before `i`.
    pub(crate) fn alnum_before(&self, i: usize) -> bool {
        self.prev_char(i).is_some_and(is_alnum_char)
    }

    /// The end of `[$_[:alnum:]]*` from `i`.
    pub(crate) fn alnum_end(&self, from: usize) -> usize {
        let mut i = from;
        while let Some(&b) = self.s.get(i) {
            if b < 0x80 {
                if CLASS[usize::from(b)] & 1 == 0 {
                    break;
                }
                i += 1;
            } else {
                match self.char_at(i) {
                    Some(c) if is_alnum_char(c) => i += c.len_utf8(),
                    _ => break,
                }
            }
        }
        self.spend(i - from + 1);
        i
    }

    /// `[$_[:alpha:]][$_[:alnum:]]*` at `i`: its end.
    pub(crate) fn ident(&self, i: usize) -> Option<usize> {
        // Every rule asks for the name at the position, so the last answer is
        // kept.
        let (at, end) = self.last_ident.get();
        if at == i {
            return end;
        }
        let end = self
            .id_start(i)
            .then(|| self.alnum_end(i + self.char_len(i)));
        self.last_ident.set((i, end));
        end
    }

    /// `#?[$_[:alpha:]][$_[:alnum:]]*` at `i`: its end.
    pub(crate) fn hash_ident(&self, i: usize) -> Option<usize> {
        let j = if self.b(i) == b'#' { i + 1 } else { i };
        self.ident(j)
    }

    /// The regex `\w` before `i`.
    fn word_before(&self, i: usize) -> bool {
        self.prev_char(i)
            .is_some_and(|c| c == '_' || c.is_alphanumeric())
    }

    fn word_at(&self, i: usize) -> bool {
        i < self.len
            && self
                .char_at(i)
                .is_some_and(|c| c == '_' || c.is_alphanumeric())
    }

    /// The regex `\b` at `i`.
    pub(crate) fn boundary(&self, i: usize) -> bool {
        self.word_before(i) != self.word_at(i)
    }

    /// `(?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))`, the grammar's guard before
    /// a keyword: not inside a name and not a property after a dot.
    pub(crate) fn kw_before(&self, i: usize) -> bool {
        !self.alnum_before(i) && (self.s[..i.min(self.len)].ends_with(b"...") || !self.after_dot(i))
    }

    /// `(?![$_[:alnum:]])(?:(?=\.\.\.)|(?!\.))`, the guard after a keyword.
    pub(crate) fn kw_after(&self, i: usize) -> bool {
        !self.alnum_at(i) && (self.starts(i, "...") || self.b(i) != b'.')
    }

    fn after_dot(&self, i: usize) -> bool {
        i > 0 && self.s.get(i - 1) == Some(&b'.')
    }

    /// A keyword between the grammar's guards: its end.
    pub(crate) fn kw(&self, p: usize, word: &str) -> Option<usize> {
        let word = word.as_bytes();
        let e = p + word.len();
        // Most calls fail on the first byte, so it is checked alone first.
        if self.s.get(p) != word.first() || e > self.len || self.s[p..e] != *word {
            return None;
        }
        (self.kw_before(p) && self.kw_after(e)).then_some(e)
    }

    /// The first of `words` that is a keyword at `p`: its end.
    pub(crate) fn kw_of(&self, p: usize, words: &[&str]) -> Option<usize> {
        words.iter().find_map(|word| self.kw(p, word))
    }

    /// The identifier at `p` as text.
    pub(crate) fn text(&self, a: usize, b: usize) -> &'a str {
        self.src.get(a..b).unwrap_or("")
    }

    // Emission.

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
    pub(crate) fn cap(&mut self, kind: Kind, a: usize, b: usize, leaf: &[S]) {
        let base = self.node(self.frame.stack, kind.names());
        self.emit(a, base);
        let stack = self.node(base, leaf);
        self.emit(b, stack);
    }

    /// Emits what is left of the match up to `to` with `leaf` inside the rule
    /// `kind`: the uncaptured part of a match that the rule scopes as a whole.
    pub(crate) fn fill(&mut self, kind: Kind, to: usize, leaf: &[S]) {
        self.cap(kind, self.pos, to, leaf);
    }

    /// Emits the rest of a match of the rule `kind` up to `to`.
    pub(crate) fn fin(&mut self, kind: Kind, to: usize) {
        self.cap(kind, to, to, &[]);
    }

    /// A match of a rule without scopes of its own: plain text up to `a`, then
    /// `a..b` with `leaf`.
    pub(crate) fn one(&mut self, a: usize, b: usize, leaf: &[S]) {
        self.flush(a);
        self.cap(Kind::Top, a, b, leaf);
    }

    // Contexts.

    /// Opens `kind` at the end of its begin tokens; false when a limit keeps
    /// it closed. Callers emit the begin tokens first.
    pub(crate) fn push(&mut self, kind: Kind) -> bool {
        self.push_with(kind, 0)
    }

    /// Opens `kind` carrying `payload`.
    pub(crate) fn push_with(&mut self, kind: Kind, payload: u32) -> bool {
        // TextMate refuses to open a context a second time at a position the
        // line has not advanced past, and gives the rest of the line to the
        // open one.
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
        self.cap(kind, a, b, leaf);
        b
    }

    /// Ends the innermost context with a zero-width match at `p`.
    pub(crate) fn close_here(&mut self, p: usize) -> usize {
        self.flush(p);
        // A context closed where it opened, without the line advancing, is a
        // loop in the grammar; TextMate leaves it open (see `stuck`).
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
    pub(crate) fn open1(&mut self, kind: Kind, a: usize, b: usize, leaf: &[S]) -> Option<usize> {
        if self.full() {
            return None;
        }
        self.flush(a);
        self.cap(kind, a, b, leaf);
        self.push(kind);
        Some(b)
    }

    /// A zero-width begin at `p`: opens `kind` without emitting a token.
    pub(crate) fn open0(&mut self, kind: Kind, p: usize) -> Option<usize> {
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
    pub(crate) fn tok(&mut self, a: usize, b: usize, leaf: S) -> Option<usize> {
        self.one(a, b, &[leaf]);
        Some(b)
    }

    /// `.` or `?.` (not before a digit) at `i`: the end of the accessor.
    pub(crate) fn accessor_at(&self, i: usize) -> Option<usize> {
        match self.b(i) {
            b'.' => Some(i + 1),
            b'?' if self.b(i + 1) == b'.' && !self.digit_after_ws(i + 2) => Some(i + 2),
            _ => None,
        }
    }

    /// The `(?!\s*\d)` that follows `?.`, as a positive test.
    fn digit_after_ws(&self, i: usize) -> bool {
        self.b(self.ws(i)).is_ascii_digit()
    }

    /// The scope of an accessor token of width `end - at`.
    pub(crate) fn accessor_scope(at: usize, end: usize) -> S {
        if end - at == 1 {
            S::PunctuationAccessor
        } else {
            S::PunctuationAccessorOptional
        }
    }

    /// `\p{upper}[$_\d[:upper:]]*` not followed by a name character, after an
    /// optional `#`: its end.
    pub(crate) fn constant_name(&self, i: usize, allow_hash: bool) -> Option<usize> {
        let mut j = i;
        if allow_hash && self.b(j) == b'#' {
            j += 1;
        }
        let first = self.char_at(j)?;
        if !first.is_uppercase() {
            return None;
        }
        j += first.len_utf8();
        while let Some(c) = self.char_at(j) {
            if c == '$' || c == '_' || c.is_ascii_digit() || c.is_uppercase() {
                j += c.len_utf8();
            } else {
                break;
            }
        }
        (!self.alnum_at(j)).then_some(j)
    }

    /// `\s*\??\.\s*#?ID` at `i`.
    pub(crate) fn member_ahead(&self, i: usize) -> bool {
        let mut j = self.ws(i);
        if self.b(j) == b'?' {
            j += 1;
        }
        self.b(j) == b'.' && self.hash_ident(self.ws(j + 1)).is_some()
    }

    /// `^\s*` followed by one of the statement keywords the grammar treats as
    /// the start of a new statement, as the end rules spell it.
    pub(crate) fn statement_start_at(&self, p: usize) -> bool {
        if !self.bol(p) {
            return false;
        }
        let j = self.ws(0);
        self.statement_keyword(j).is_some()
    }

    /// The end of a statement keyword at `j`, with its trailing `\b`.
    pub(crate) fn statement_keyword(&self, j: usize) -> Option<usize> {
        const WORDS: [&str; 28] = [
            "abstract",
            "async",
            "break",
            "case",
            "catch",
            "class",
            "const",
            "continue",
            "declare",
            "do",
            "else",
            "enum",
            "export",
            "finally",
            "function",
            "for",
            "goto",
            "if",
            "import",
            "interface",
            "let",
            "module",
            "namespace",
            "switch",
            "return",
            "throw",
            "try",
            "type",
        ];
        if let Some(e) = self.await_using_end(j) {
            return Some(e);
        }
        if let Some(e) = self.using_end(j) {
            return Some(e);
        }
        for word in WORDS.iter().chain(["var", "while"].iter()) {
            if self.starts(j, word) && self.boundary(j + word.len()) {
                return Some(j + word.len());
            }
        }
        None
    }

    /// `\busing(?=\s+(?!in\b|of\b(?!\s*(?:of\b|=)))[$_[:alpha:]])\b`: its end.
    pub(crate) fn using_end(&self, j: usize) -> Option<usize> {
        if !self.starts(j, "using") || !self.boundary(j) {
            return None;
        }
        let e = j + 5;
        let k = self.ws(e);
        if k == e || !self.id_start(k) {
            return None;
        }
        let word_end = self.alnum_end(k);
        let word = self.text(k, word_end);
        if word == "in" {
            return None;
        }
        if word == "of" {
            let m = self.ws(word_end);
            if !(self.starts(m, "of") && self.boundary(m + 2) || self.b(m) == b'=') {
                return None;
            }
        }
        self.boundary(e).then_some(e)
    }

    /// `\bawait\s+\busing(…)\b`: its end.
    pub(crate) fn await_using_end(&self, j: usize) -> Option<usize> {
        if !self.starts(j, "await") || !self.boundary(j) {
            return None;
        }
        let k = self.ws(j + 5);
        if k == j + 5 {
            return None;
        }
        self.using_end(k)
    }

    /// `^\s*$`: a blank line, at its start.
    pub(crate) fn blank_line_at(&self, p: usize) -> bool {
        self.bol(p) && self.eol(self.ws(0))
    }
}
