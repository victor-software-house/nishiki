//! The per-line lexer: cursor, token emission, the context stack and the
//! character classes the grammar's regular expressions spell out.

use std::cell::Cell;

use super::kind::Kind;
use super::scope::S;
use crate::arena::{Arena, Frame};
use crate::limits::{Limit, Limits, LineReport};
use crate::{Stack, Token};

/// Lexes one line. `pos` is the end of the last token emitted; the bytes
/// between it and a rule's match are plain text of the current context.
///
/// The grammar was written for `TextMate`, which tokenizes each line with a
/// newline appended. [`Lexer::b`] therefore reads a newline at the end of the
/// line, so `\s` and `$` behave as they do there.
pub(crate) struct Lexer<'a> {
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
    /// Where each open context was entered on this line, by the depth of its
    /// frame; a context opened on an earlier line has no entry.
    enter: Vec<usize>,
    /// The `\G` anchor in force before each open context began, by depth.
    anchors: Vec<usize>,
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
    /// Set when a context was refused for depth; recorded at the line's end.
    depth_hit: Cell<bool>,
    /// The last run of blanks that [`Lexer::ws`] scanned, as start and end.
    blanks: Cell<(usize, usize)>,
    /// The position of the last backtick on the line, if any.
    last_tick: Option<usize>,
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

impl<'a> Lexer<'a> {
    pub(crate) fn new(
        arena: &'a mut Arena,
        limits: &'a Limits,
        src: &'a str,
        out: &'a mut Vec<Token>,
        state: u32,
        report: &'a mut LineReport,
    ) -> Self {
        let frame = arena.frame(state);
        let budget = limits.work.map_or(u64::MAX, |work| work.steps(src.len()));
        let mut lexer = Lexer {
            src,
            s: src.as_bytes(),
            len: src.len(),
            out,
            arena,
            limits,
            report,
            pos: 0,
            state,
            frame,
            steps: Cell::new(0),
            budget,
            enter: Vec::new(),
            anchors: Vec::new(),
            line_pos: 0,
            anchor: usize::MAX,
            first_line: frame.kind == Kind::Top.code() && frame.depth == 1,
            stuck: false,
            depth_hit: Cell::new(false),
            blanks: Cell::new((0, 0)),
            last_tick: memchr::memrchr(b'`', src.as_bytes()),
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
    pub(crate) fn spend(&self, bytes: usize) {
        let bytes = u64::try_from(bytes).unwrap_or(u64::MAX);
        self.steps.set(self.steps.get().saturating_add(bytes));
    }

    /// Whether a backtick lies at or after `p`.
    pub(crate) fn has_backtick_after(&self, p: usize) -> bool {
        self.last_tick.is_some_and(|t| t >= p)
    }

    pub(crate) fn run(&mut self) {
        let mut p = 0;
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
                p += self.char_len(p);
            }
            debug_assert!(
                p > from || self.state != before || self.stuck,
                "the TypeScript lexer made no progress at byte {from}"
            );
        }
        if self.depth_hit.get() {
            self.report.record(Limit::Depth);
        }
        self.flush(self.len);
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
        while is_ws(self.b(j)) {
            j += 1;
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
        match self.b(i) {
            b'$' | b'_' => true,
            b if b.is_ascii_alphabetic() => true,
            b if b >= 0x80 && i < self.len => self.char_at(i).is_some_and(is_alpha_char),
            _ => false,
        }
    }

    /// `[$_[:alnum:]]` at `i`.
    pub(crate) fn alnum_at(&self, i: usize) -> bool {
        match self.b(i) {
            b'$' | b'_' => true,
            b if b.is_ascii_alphanumeric() => true,
            b if b >= 0x80 && i < self.len => self.char_at(i).is_some_and(is_alnum_char),
            _ => false,
        }
    }

    /// `[$_[:alnum:]]` just before `i`.
    pub(crate) fn alnum_before(&self, i: usize) -> bool {
        self.prev_char(i).is_some_and(is_alnum_char)
    }

    /// The end of `[$_[:alnum:]]*` from `i`.
    pub(crate) fn alnum_end(&self, mut i: usize) -> usize {
        while i < self.len && self.alnum_at(i) {
            i += self.char_len(i);
        }
        i
    }

    /// `[$_[:alpha:]][$_[:alnum:]]*` at `i`: its end.
    pub(crate) fn ident(&self, i: usize) -> Option<usize> {
        self.id_start(i)
            .then(|| self.alnum_end(i + self.char_len(i)))
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
        let e = p + word.len();
        (self.starts(p, word) && self.kw_before(p) && self.kw_after(e)).then_some(e)
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
            let Some(child) = self.arena.child(base, scope.scope()) else {
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

    /// Where the context of the frame at `depth` was entered on this line.
    fn entered(&self, depth: u32) -> usize {
        let index = usize::try_from(depth).unwrap_or(usize::MAX);
        self.enter.get(index).copied().unwrap_or(usize::MAX)
    }

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
            let mut frame = self.frame;
            while frame.depth > 0 && self.entered(frame.depth) == self.line_pos {
                if frame.kind == kind.code() {
                    self.stuck = true;
                    return false;
                }
                frame = self.arena.frame(frame.parent);
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
        let depth = usize::try_from(self.frame.depth).unwrap_or(usize::MAX) + 1;
        if self.enter.len() <= depth {
            self.enter.resize(depth + 1, usize::MAX);
            self.anchors.resize(depth + 1, usize::MAX);
        }
        self.enter[depth] = self.line_pos;
        self.anchors[depth] = self.anchor;
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
        let depth = usize::try_from(self.frame.depth).unwrap_or(usize::MAX);
        self.anchor = self.anchors.get(depth).copied().unwrap_or(usize::MAX);
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
        if p <= self.line_pos && self.entered(self.frame.depth) == self.line_pos {
            self.stuck = true;
            return p;
        }
        self.pop();
        p
    }
}

impl Lexer<'_> {
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
        if p != 0 {
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
        p == 0 && self.eol(self.ws(0))
    }
}
