//! The per-line lexer: cursor, token emission, the context stack and the
//! rules that read a table's lines.

use std::cell::{Cell, OnceCell};

use memchr::{memchr2, memrchr};

use super::scope::S;
use super::state::{ArrayNext, InlineComments, Rule};
use crate::classes::space_width;
use crate::emit::Emitter;
use crate::language::Context;
use crate::limits::{Finished, Limit, Steps};
use crate::utf8::char_len;

/// Lexes one line.
/// `pos` is the end of the last token.
/// The bytes between it and the cursor are plain text of the current context.
pub(crate) struct Lexer<'a> {
    pub(super) s: &'a [u8],
    /// The tokens, the contexts and the end of the last token.
    pub(super) em: Emitter<'a>,
    /// The work this line has spent and may spend.
    pub(super) steps: Steps,
    /// The first byte that is not white space.
    /// A table header, or a comment that gets the leading-space scope, starts
    /// here at the earliest.
    first: usize,
    /// Where the table header at `first` ends, and whether it is `[[…]]`.
    /// It is scanned once.
    /// The white space before the header and the header itself ask in turn.
    header: OnceCell<Option<(usize, bool)>>,
    /// The last `=` of the line.
    /// A key must come before it.
    pub(super) last_eq: Option<usize>,
    /// The last answer of a search for the first `"` that `\s*=` follows
    /// after a byte: the byte searched from, and the answer.
    /// The answer is `usize::MAX` for none.
    /// It holds for every byte between the two.
    pub(super) quote_eq: Cell<(usize, usize)>,
    /// No key that is only bare names, dots and white space starts before
    /// this byte, because one that started earlier failed where this ends.
    pub(super) no_key_before: usize,
    /// A key made of names with a quoted one among them failed to end in an
    /// `=`.
    /// This holds the start of the next of its names and the byte where it
    /// failed.
    /// A key that starts at that name fails the same way.
    pub(super) failed_chain: Cell<(usize, usize)>,
}

impl<'a> Lexer<'a> {
    /// The lexer of one line, from `state`.
    ///
    /// It is `#[inline]`, as `run` is.
    /// `toml::lex`, in another module, calls both on every line.
    /// At the default sixteen codegen units they were called out of line.
    #[inline]
    pub(crate) fn new(cx: &'a mut Context<'_>, s: &'a [u8], state: u32) -> Self {
        let steps = Steps::new(cx.limits, s.len());
        let mut lexer = Lexer {
            s,
            em: Emitter::new(cx, state, 0),
            steps,
            first: 0,
            header: OnceCell::new(),
            last_eq: memrchr(b'=', s),
            quote_eq: Cell::new((0, 0)),
            no_key_before: 0,
            failed_chain: Cell::new((usize::MAX, 0)),
        };
        lexer.first = lexer.skip_space(0);
        lexer
    }

    /// The state after the line, with its count given back to the lender.
    pub(crate) fn finish_line(self) -> Finished {
        self.steps.finish(self.em.state)
    }

    /// Lexes the line.
    ///
    /// The `in_*` steps it calls are `#[inline]` too.
    /// Once `run` is inlined into `toml::lex`, that module calls them.
    #[inline]
    pub(crate) fn run(&mut self) {
        let len = self.s.len();
        let mut p = 0;
        while p < len {
            self.steps.spend(1);
            if self.steps.exhausted() {
                self.em.report.record(Limit::Work);
                break;
            }
            let from = p;
            p = match self.top() {
                Rule::Basic | Rule::BasicMulti => self.in_basic(p),
                Rule::Literal | Rule::LiteralMulti => self.in_literal(p),
                Rule::Top => self.in_table(p),
                Rule::Array => self.in_array(p),
                Rule::Inline => self.in_inline(p),
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

    /// The end of the run of white space that starts at `i`.
    pub(super) fn skip_space(&self, mut i: usize) -> usize {
        let from = i;
        loop {
            let width = space_width(self.s, i);
            if width == 0 {
                self.steps.spend(i - from);
                return i;
            }
            i += width;
        }
    }

    // Emission.

    /// Emits plain text of the current context up to `to`.
    pub(super) fn gap(&mut self, to: usize) {
        self.tok(to, &[]);
    }

    /// Emits the bytes up to `to` with the current context plus `leaf`.
    pub(super) fn tok(&mut self, to: usize, leaf: &[S]) {
        self.em.tok(to, leaf.iter().map(|scope| scope.scope()));
    }

    /// Emits the bytes from `from` to `to` with `leaf`, after plain text up
    /// to `from`.
    pub(super) fn tok_at(&mut self, from: usize, to: usize, leaf: &[S]) {
        self.gap(from);
        self.tok(to, leaf);
    }

    // Contexts.

    pub(super) fn top(&self) -> Rule {
        Rule::from_code(self.em.frame.rule)
    }

    /// Opens a context.
    /// It returns false when a limit keeps the context closed.
    fn push(&mut self, rule: Rule) -> bool {
        let payload = if rule == Rule::Inline {
            InlineComments::Allowed.payload()
        } else {
            ArrayNext::Value.payload()
        };
        self.em
            .push(rule.code(), payload, rule.scope().map(S::scope))
    }

    /// What the array being read expects next.
    fn array_next(&self) -> ArrayNext {
        ArrayNext::from_payload(self.em.frame.payload)
    }

    /// Whether a comment may start a line of the inline table being read.
    fn inline_comments(&self) -> InlineComments {
        InlineComments::from_payload(self.em.frame.payload)
    }

    /// Sets the payload of the array or inline table being read.
    fn set_payload(&mut self, payload: u32) {
        if self.em.frame.payload == payload {
            return;
        }
        let frame = self.em.frame;
        match self
            .em
            .arena
            .push(frame.parent, frame.rule, payload, frame.stack)
        {
            Some(state) => self.em.enter(state),
            None => self.em.report.record(Limit::Nodes),
        }
    }

    /// Notes that a value, key or other text starts at the cursor, for the
    /// array or inline table around it.
    pub(super) fn value_starts(&mut self) {
        match self.top() {
            Rule::Array => self.set_payload(ArrayNext::Comma.payload()),
            Rule::Inline => self.set_payload(InlineComments::Barred.payload()),
            Rule::Top | Rule::Basic | Rule::Literal | Rule::BasicMulti | Rule::LiteralMulti => {}
        }
    }

    /// Opens `rule` at `p`.
    /// Its begin text ends at `end` and carries `leaf`.
    /// The text stays plain when a limit keeps the context closed.
    pub(super) fn begin(&mut self, p: usize, end: usize, rule: Rule, leaf: S) -> usize {
        self.gap(p);
        if self.push(rule) {
            self.tok(end, &[leaf]);
        }
        end
    }

    /// Emits the end text up to `end` with `leaf` and closes the context.
    pub(super) fn finish(&mut self, p: usize, end: usize, leaf: S) -> usize {
        self.gap(p);
        self.tok(end, &[leaf]);
        self.em.pop();
        end
    }

    // Lines of a table.

    /// Takes the next match at or after `p` outside every array, inline
    /// table and string.
    #[inline]
    fn in_table(&mut self, p: usize) -> usize {
        if space_width(self.s, p) > 0 {
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

    /// White space at `p`.
    /// On a line that starts with it, it takes the scope of the table header
    /// or comment that follows.
    /// Elsewhere it is plain.
    fn space(&mut self, p: usize) -> usize {
        let end = self.skip_space(p);
        if p == 0 && end == self.first && end < self.s.len() {
            match self.s[end] {
                b'#' if self.blanks(end) => self.tok(end, &[S::CommentLeading]),
                b'[' => {
                    if let Some((_, double)) = self.header_end() {
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
    /// it.
    /// Other white space does not get the leading-space scope.
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
        while end < self.s.len() && space_width(self.s, end) == 0 {
            end += self.width_at(end);
        }
        let spaced = self.skip_space(end);
        if spaced < self.s.len() {
            end = spaced;
        }
        self.steps.spend(end - p);
        self.tok_at(p, end, &[S::Invalid]);
        end
    }

    /// Where the table header at the line's first character ends, and
    /// whether it is `[[…]]`.
    ///
    /// `[` or `[[`, text without brackets, then `]` or `]]`.
    fn header_end(&self) -> Option<(usize, bool)> {
        *self.header.get_or_init(move || {
            let p = self.first;
            let double = self.at(p + 1) == b'[';
            let start = p + if double { 2 } else { 1 };
            let rest = self.s.get(start..)?;
            let offset = memchr2(b'[', b']', rest)?;
            self.steps.spend(offset);
            if rest[offset] != b']' {
                return None;
            }
            let close = start + offset;
            if !double {
                return Some((close + 1, false));
            }
            (self.at(close + 1) == b']').then_some((close + 2, true))
        })
    }

    /// A table header at `p`, the line's first character, which is `[`.
    fn header(&mut self, p: usize) -> Option<usize> {
        let (end, double) = self.header_end()?;
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
            let width = space_width(self.s, i);
            if width > 0 {
                i += width;
                continue;
            }
            if self.s[i] == b'.' {
                i += 1;
                continue;
            }
            let from = i;
            while i < close && space_width(self.s, i) == 0 && self.s[i] != b'.' {
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
    #[inline]
    fn in_array(&mut self, p: usize) -> usize {
        if space_width(self.s, p) > 0 {
            return self.leading_space(p);
        }
        match self.s[p] {
            b']' => return self.finish(p, p + 1, S::PunctArrayEnd),
            b'#' => return self.comment(p),
            b',' if self.array_next() == ArrayNext::Comma => {
                self.tok_at(p, p + 1, &[S::PunctArraySep]);
                self.set_payload(ArrayNext::Value.payload());
                return p + 1;
            }
            _ if self.array_next() == ArrayNext::Value => {
                if let Some(end) = self.value(p) {
                    return end;
                }
            }
            _ => {}
        }
        self.invalid(p)
    }

    /// White space in an array or inline table.
    /// It takes the leading-space scope when it precedes a comment on a line
    /// that starts with it.
    /// An inline table has no comments.
    fn leading_space(&mut self, p: usize) -> usize {
        let end = self.skip_space(p);
        let comments =
            self.top() == Rule::Array || self.inline_comments() == InlineComments::Allowed;
        if p == 0 && end == self.first && self.at(end) == b'#' && comments && self.blanks(end) {
            self.tok(end, &[S::CommentLeading]);
        }
        end
    }

    /// Takes the next match at or after `p` in an inline table.
    #[inline]
    fn in_inline(&mut self, p: usize) -> usize {
        if space_width(self.s, p) > 0 {
            return self.leading_space(p);
        }
        match self.s[p] {
            b'}' => return self.finish(p, p + 1, S::PunctInlineEnd),
            b',' => {
                self.tok_at(p, p + 1, &[S::PunctInlineSep]);
                self.set_payload(InlineComments::Allowed.payload());
                return p + 1;
            }
            b'#' if p > 0
                && p == self.first
                && self.inline_comments() == InlineComments::Allowed
                && self.blanks(p) =>
            {
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
