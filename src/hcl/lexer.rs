//! The per-line lexer: the line's state, its main loop, token emission
//! and the context stack.

use super::scope::S;
use super::state::Kind;
use crate::emit::Emitter;
use crate::language::Context;
use crate::limits::{Finished, Limit, Steps};

/// What becomes of the line feed after the line.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Feed {
    /// The rules still see it after the last byte.
    Ahead,
    /// An inline `for` took the rest of the line and its line feed.
    /// The rest is lexed in the `for`'s own context, with no line feed after
    /// it.
    /// `outer` is the state to go back to at the line's end.
    Capture { outer: u32 },
    /// A line comment took it.
    Taken,
}

/// Whether a skipped word may hold an inline `for` or `if` at its end.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Tail {
    /// It may not: the context has neither rule.
    Plain,
    /// It may: the context is an object or a bracket.
    ForIf,
}

/// Lexes one line.
/// `em.pos` is the end of the last token.
/// The bytes between it and the cursor are plain text of the current context.
pub(crate) struct Lexer<'a> {
    pub(super) s: &'a [u8],
    /// The tokens, the contexts and the end of the last token.
    pub(super) em: Emitter<'a>,
    /// The work this line has spent and may spend.
    pub(super) steps: Steps,
    /// What becomes of the line feed.
    pub(super) feed: Feed,
    /// No block header starts before this byte.
    /// One that started earlier ran to it without its `{`.
    /// A header that starts later in the same run of labels ends there too.
    pub(super) no_block_before: usize,
    /// No function call starts before this byte.
    /// The name of one that started earlier ran to it without its `(`.
    pub(super) no_call_before: usize,
    /// No attribute definition starts before this byte.
    /// The name of one that started earlier ran to it without its `=`.
    pub(super) no_attribute_before: usize,
    /// No object key starts before this byte, for the same reason.
    pub(super) no_key_before: usize,
}

impl<'a> Lexer<'a> {
    /// The lexer of one line, from `state`.
    #[inline]
    pub(crate) fn new(cx: &'a mut Context<'_>, s: &'a [u8], state: u32) -> Self {
        let steps = Steps::new(cx.limits, s.len());
        Lexer {
            s,
            em: Emitter::new(cx, state, 0),
            steps,
            feed: Feed::Ahead,
            no_block_before: 0,
            no_call_before: 0,
            no_attribute_before: 0,
            no_key_before: 0,
        }
    }

    /// The state after the line, with its count given back to the lender.
    pub(crate) fn finish_line(self) -> Finished {
        self.steps.finish(self.em.state)
    }

    /// Lexes the line, then its line feed.
    #[inline]
    pub(crate) fn run(&mut self) {
        let len = self.s.len();
        let mut p = 0;
        while p < len {
            if self.steps.step() {
                self.em.report.record(Limit::Work);
                break;
            }
            let from = p;
            p = match self.top() {
                Kind::Top => self.in_top(p),
                Kind::Block => self.in_block(p),
                Kind::Objects => self.in_objects(p),
                Kind::ObjectKey => self.in_object_key(p),
                Kind::Brackets => self.in_brackets(p),
                Kind::TupleFor | Kind::ObjectFor | Kind::InlineFor => self.in_for(p),
                Kind::Functions => self.in_call(p),
                Kind::Parens => self.in_parens(p),
                Kind::InlineIf => self.in_inline_if(p),
                Kind::String => self.in_string(p),
                Kind::Heredoc => self.in_heredoc(p),
                Kind::Interpolation => self.in_interpolation(p),
                Kind::BlockComment => self.in_block_comment(p),
            };
            debug_assert!(p > from, "the HCL lexer made no progress at byte {from}");
        }
        self.gap(len);
        match self.feed {
            Feed::Ahead => {
                if self.top() == Kind::InlineIf {
                    self.em.pop();
                }
            }
            Feed::Capture { outer } => self.em.enter(outer),
            Feed::Taken => {}
        }
    }

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

    pub(super) fn top(&self) -> Kind {
        Kind::from_code(self.em.frame.kind)
    }

    /// Opens a context with `payload`.
    /// It returns false when a limit keeps the context closed.
    pub(super) fn push(&mut self, kind: Kind, payload: u32) -> bool {
        self.em
            .push(kind.code(), payload, kind.scope().map(S::scope))
    }

    /// Opens `kind` at `p`.
    /// Its begin text ends at `end` and carries `leaf`.
    /// The text stays plain when a limit keeps the context closed.
    pub(super) fn begin(&mut self, p: usize, end: usize, kind: Kind, leaf: S) -> usize {
        self.gap(p);
        if self.push(kind, 0) {
            self.tok(end, &[leaf]);
        }
        end
    }

    /// Emits the end text up to `end` with `leaf` and closes the context.
    pub(super) fn finish(&mut self, p: usize, end: usize, leaf: S) -> usize {
        self.tok_at(p, end, &[leaf]);
        self.em.pop();
        end
    }

    /// Emits the single character at `p` with `leaf`.
    pub(super) fn mark(&mut self, p: usize, leaf: S) -> usize {
        self.tok_at(p, p + 1, &[leaf]);
        p + 1
    }
}
