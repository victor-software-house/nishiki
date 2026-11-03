//! The token and frame bookkeeping the hand-written state machines share.
//!
//! A lexer keeps its open contexts as frames of the arena and appends tokens
//! as it goes. [`Emitter`] holds the arena, the token buffer and the report
//! with the current state and the end of the last token, so each lexer only
//! maps its own scope enum onto [`Scope`] and its context enum onto a code.

use crate::arena::{Arena, Frame};
use crate::language::Context;
use crate::limits::{Limit, Limits, LineReport};
use crate::scope::Scope;
use crate::{Stack, Token};

/// Appends a line's tokens and opens and closes its contexts.
pub(crate) struct Emitter<'a> {
    pub(crate) arena: &'a mut Arena,
    pub(crate) limits: &'a Limits,
    pub(crate) out: &'a mut Vec<Token>,
    pub(crate) report: &'a mut LineReport,
    /// The current state.
    pub(crate) state: u32,
    /// The current state's frame.
    pub(crate) frame: Frame,
    /// The end of the last token.
    pub(crate) pos: usize,
}

impl<'a> Emitter<'a> {
    /// An emitter at `state`, whose next token starts at `pos`.
    pub(crate) fn new(cx: &'a mut Context<'_>, state: u32, pos: usize) -> Self {
        let frame = cx.arena.frame(state);
        Emitter {
            arena: &mut *cx.arena,
            limits: cx.limits,
            out: &mut *cx.out,
            report: &mut *cx.report,
            state,
            frame,
            pos,
        }
    }

    /// Emits the bytes up to `to` with the current context's stack and
    /// `scopes` on top; nothing when `to` is not past the last token.
    #[inline]
    pub(crate) fn tok(&mut self, to: usize, scopes: impl IntoIterator<Item = Scope>) {
        if to <= self.pos {
            return;
        }
        let stack = self.arena.path(self.frame.stack, scopes, self.report);
        self.out.push(Token {
            end: to,
            stack: Stack(stack),
        });
        self.pos = to;
    }

    /// Opens a context of kind `code` with `payload`, whose text carries
    /// `scopes` on top of the current stack; false when the depth or node
    /// limit keeps it closed, which the report records.
    ///
    /// [`Limits::depth_reached`] counts the contexts open above the
    /// language's root, and a frame's depth counts the root too.
    pub(crate) fn push(
        &mut self,
        code: u16,
        payload: u32,
        scopes: impl IntoIterator<Item = Scope>,
    ) -> bool {
        let open = usize::try_from(self.frame.depth.saturating_sub(1)).unwrap_or(usize::MAX);
        if self.limits.depth_reached(open) {
            self.report.record(Limit::Depth);
            return false;
        }
        let mut stack = Some(self.frame.stack);
        for scope in scopes {
            stack = stack.and_then(|stack| self.arena.child(stack, scope));
        }
        let Some(state) = stack.and_then(|stack| self.arena.push(self.state, code, payload, stack))
        else {
            self.report.record(Limit::Nodes);
            return false;
        };
        self.enter(state);
        true
    }

    /// Closes the current context.
    pub(crate) fn pop(&mut self) {
        self.enter(self.frame.parent);
    }

    /// Makes `state` the current state.
    pub(crate) fn enter(&mut self, state: u32) {
        self.state = state;
        self.frame = self.arena.frame(state);
    }
}
