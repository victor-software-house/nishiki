//! The per-line lexer: cursor, token emission and the context stack.

use std::cell::{Cell, OnceCell};

use memchr::memmem;

use super::scope::S;
use super::state::Kind;
use crate::arena::{Arena, Frame};
use crate::limits::{Limit, Limits, LineReport};
use crate::{Stack, Token};

/// Lexes one line. `pos` is the end of the last token; `s` is scanned ahead
/// of it, and the bytes in between are plain text of the current context.
pub(crate) struct Lexer<'a> {
    pub(crate) s: &'a [u8],
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
    /// Where the line's last `>(` starts, once [`Lexer::call_ahead`] has
    /// searched for it.
    last_call: OnceCell<Option<usize>>,
    /// End of the ASCII identifier run at the cursor, and of the word run
    /// (which also covers non-ASCII bytes). Set by [`Lexer::scan_word`].
    pub(crate) ident_end: usize,
    pub(crate) word_end: usize,
}

pub(crate) fn is_ws(b: u8) -> bool {
    matches!(b, b' ' | b'\t' | b'\r' | 0x0b | 0x0c)
}

/// ASCII letters, digits and `_`: the character class the grammar spells out.
pub(crate) fn is_ident(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

/// The regex `\w`, with non-ASCII bytes counted as word characters. A
/// Unicode space is not one: [`Lexer::word_at`] and [`Lexer::word_before`]
/// tell it apart.
pub(crate) fn is_word(b: u8) -> bool {
    is_ident(b) || b >= 0x80
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
        Lexer {
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
            last_call: OnceCell::new(),
            ident_end: 0,
            word_end: 0,
        }
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
                Kind::Str => self.in_string(p),
                Kind::Raw => self.in_raw_string(p),
                Kind::Char => self.in_char(p),
                Kind::Block | Kind::Doc => self.in_block_comment(p),
                Kind::Top
                | Kind::FnDef
                | Kind::Call
                | Kind::Generic
                | Kind::AngleSq
                | Kind::Attr
                | Kind::Use
                | Kind::Import => self.in_code(p),
            };
            debug_assert!(p > from, "the Rust lexer made no progress at byte {from}");
        }
        self.gap(len);
    }

    // Bytes and boundaries.

    /// The byte at `i`, or 0 past the end.
    pub(crate) fn at(&self, i: usize) -> u8 {
        self.s.get(i).copied().unwrap_or(0)
    }

    pub(crate) fn starts_with(&self, i: usize, prefix: &[u8]) -> bool {
        self.s.get(i..).is_some_and(|rest| rest.starts_with(prefix))
    }

    /// The end of the blanks at `i`: the ASCII ones and the Unicode spaces that
    /// the grammar's `\s` reads.
    pub(crate) fn blanks(&self, mut i: usize) -> usize {
        let from = i;
        while i < self.s.len() {
            if is_ws(self.s[i]) {
                i += 1;
            } else if self.s[i] >= 0xc2 && crate::space::len(&self.s[i..]) > 0 {
                i += crate::space::len(&self.s[i..]);
            } else {
                break;
            }
        }
        self.spend(i - from);
        i
    }

    pub(crate) fn skip(&self, mut i: usize, class: impl Fn(u8) -> bool) -> usize {
        let from = i;
        while i < self.s.len() && class(self.s[i]) {
            i += 1;
        }
        self.spend(i - from);
        i
    }

    /// Whether `>(` occurs at or after `from`: the lookahead of the turbofish
    /// call rule. The line is searched once, from its end, so each check is
    /// one comparison and a line of repeated `a::<` stays linear.
    pub(crate) fn call_ahead(&self, from: usize) -> bool {
        let last = self.last_call.get_or_init(|| {
            self.spend(self.s.len());
            memmem::rfind(self.s, b">(")
        });
        last.is_some_and(|at| at >= from)
    }

    /// Counts `bytes` read ahead of the cursor against the line's work.
    pub(crate) fn spend(&self, bytes: usize) {
        let bytes = u64::try_from(bytes).unwrap_or(u64::MAX);
        self.steps.set(self.steps.get().saturating_add(bytes));
    }

    /// The regex `\b` at `i`.
    #[inline]
    pub(crate) fn boundary(&self, i: usize) -> bool {
        self.word_before(i) != self.word_at(i)
    }

    /// Whether the character at `i` is a word character.
    #[inline]
    pub(crate) fn word_at(&self, i: usize) -> bool {
        match self.s.get(i) {
            None => false,
            Some(&b) if b < 0xc2 => is_word(b),
            Some(_) => crate::space::len(&self.s[i..]) == 0,
        }
    }

    /// Whether the character that ends at `i` is a word character.
    #[inline]
    pub(crate) fn word_before(&self, i: usize) -> bool {
        match i.checked_sub(1).map(|j| self.s[j]) {
            None => false,
            Some(b) if b < 0x80 => is_ident(b),
            Some(_) => !self.space_before(i),
        }
    }

    /// Whether a Unicode space ends at `i`.
    fn space_before(&self, i: usize) -> bool {
        (i >= 2 && crate::space::len(&self.s[i - 2..i]) == 2)
            || (i >= 3 && crate::space::len(&self.s[i - 3..i]) == 3)
    }

    /// The end of the word characters at `i`.
    pub(crate) fn word_run(&self, mut i: usize) -> usize {
        let from = i;
        while self.word_at(i) {
            i += 1;
        }
        self.spend(i - from);
        i
    }

    /// Whether a word starts at `i`, given that the byte there is a word byte.
    pub(crate) fn word_start(&self, i: usize) -> bool {
        !self.word_before(i)
    }

    /// Records the identifier and word runs starting at `p`.
    pub(crate) fn scan_word(&mut self, p: usize) {
        self.ident_end = self.skip(p, is_ident);
        self.word_end = if self.at(self.ident_end) >= 0x80 {
            self.word_run(self.ident_end)
        } else {
            self.ident_end
        };
    }

    /// Whether the word at the cursor is exactly `word`.
    pub(crate) fn word_is(&self, p: usize, word: &[u8]) -> bool {
        self.word_end == p + word.len() && self.s[p..self.word_end] == *word
    }

    // Emission.

    /// Emits plain text of the current context up to `to`.
    pub(crate) fn gap(&mut self, to: usize) {
        self.tok(to, &[]);
    }

    /// Emits the bytes up to `to` with the current context plus `leaf`.
    pub(crate) fn tok(&mut self, to: usize, leaf: &[S]) {
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

    // Contexts.

    /// The innermost open context.
    pub(crate) fn top(&self) -> Kind {
        Kind::from_code(self.frame.kind)
    }

    /// The data the innermost context carries.
    pub(crate) fn payload(&self) -> u32 {
        self.frame.payload
    }

    /// Opens a context; false when a limit keeps it closed. Call
    /// [`Lexer::gap`] first.
    pub(crate) fn push(&mut self, kind: Kind) -> bool {
        self.push_with(kind, 0)
    }

    /// Opens a context that carries `payload`.
    pub(crate) fn push_with(&mut self, kind: Kind, payload: u32) -> bool {
        let open = usize::try_from(self.frame.depth.saturating_sub(1)).unwrap_or(usize::MAX);
        if self.limits.depth_reached(open) {
            self.report.record(Limit::Depth);
            return false;
        }
        let stack = match kind.scope() {
            Some(scope) => self.arena.child(self.frame.stack, scope.scope()),
            None => Some(self.frame.stack),
        };
        let Some(state) =
            stack.and_then(|stack| self.arena.push(self.state, kind.code(), payload, stack))
        else {
            self.report.record(Limit::Nodes);
            return false;
        };
        self.enter(state);
        true
    }

    /// Replaces the innermost context's payload.
    pub(crate) fn set_payload(&mut self, payload: u32) {
        let Frame {
            parent,
            kind,
            stack,
            ..
        } = self.frame;
        match self.arena.push(parent, kind, payload, stack) {
            Some(state) => self.enter(state),
            None => self.report.record(Limit::Nodes),
        }
    }

    pub(crate) fn pop(&mut self) {
        self.enter(self.frame.parent);
    }

    fn enter(&mut self, state: u32) {
        self.state = state;
        self.frame = self.arena.frame(state);
    }

    // Code.

    fn in_code(&mut self, p: usize) -> usize {
        let b = self.s[p];
        if is_ws(b) || (b >= 0xc2 && crate::space::len(&self.s[p..]) > 0) {
            return self.whitespace(p);
        }
        if let Some(end) = self.end_of_context(p, b) {
            return end;
        }
        if is_ident(b) {
            self.scan_word(p);
        }
        let kind = self.top();
        if kind == Kind::Top
            && let Some(end) = self.top_rules(p, b)
        {
            return end;
        }
        for &group in super::code::groups(kind) {
            if !group.can_start(b) {
                continue;
            }
            if let Some(end) = self.group(group, p, b) {
                return end;
            }
        }
        p + 1
    }

    /// Skips blanks. A blank run after a word may start the comparison rule.
    fn whitespace(&mut self, p: usize) -> usize {
        if p > 0
            && self.word_before(p)
            && super::code::groups(self.top()).contains(&super::code::Group::Keywords)
            && let Some(end) = self.comparison(p)
        {
            return end;
        }
        self.blanks(p)
    }

    /// Closes the innermost code context when its end rule matches at `p`.
    fn end_of_context(&mut self, p: usize, b: u8) -> Option<usize> {
        if !matches!(b, b'{' | b';' | b')' | b'>' | b']') {
            return None;
        }
        let leaf = match (self.top(), b) {
            (Kind::FnDef, b'{') => S::Curly,
            (Kind::FnDef | Kind::Use | Kind::Import, b';') => S::Semi,
            (Kind::Call, b')') => S::Round,
            (Kind::Generic | Kind::AngleSq, b'>') => S::Angle,
            (Kind::Attr, b']') => S::AttrBracket,
            _ => return None,
        };
        self.gap(p);
        self.tok(p + 1, &[leaf]);
        self.pop();
        Some(p + 1)
    }
}
