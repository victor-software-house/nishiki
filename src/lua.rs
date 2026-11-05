//! Lua, with the scopes of the `lua` grammar that Shiki bundles.
//! That grammar comes from sumneko's lua.tmbundle through VS Code.
//!
//! The lexer runs the grammar's begin/end rules with hand-written matchers
//! over bytes (see the `engine` module).
//! The tables in `grammar` are generated from the grammar.
//! The `matchers` decide where each pattern matches.
//! The rules left open at the end of a line are frames of the highlighter's
//! interned state: long strings and comments, and function heads.

mod engine;
mod grammar;
mod matchers;
mod scope;

use engine::Lua;
pub(crate) use engine::Scratch;
pub(crate) use scope::NAMES;

use crate::arena::Arena;
use crate::language::{Context, Halt, Line, Stopped};
use crate::limits::Finished;
use crate::rules;

/// The state at the start of a Lua document whose scopes sit on the stack
/// `base`.
pub(crate) fn start(arena: &mut Arena, base: u32) -> u32 {
    rules::start::<Lua>(arena, base)
}

/// The state at the start of the Lua lines of a fenced code block.
pub(crate) fn start_embedded(arena: &mut Arena, base: u32) -> u32 {
    rules::start_embedded::<Lua>(arena, base)
}

/// Lexes one line under the crate's lexer contract.
/// The body of an `ffi.cdef[[...]]` is C (see [`rules::lex_embedding`]).
/// C lexes it up to the bracket that closes it.
/// Most lines hold no such body, and they are lexed without C
/// (see [`rules::lex_on_demand`]).
#[inline]
pub(crate) fn lex(cx: &mut Context<'_>, line: Line<'_>, state: u32) -> Finished {
    rules::lex_on_demand::<Lua>(cx, line, state, |scratch| &mut scratch.lua)
}

/// The body of an `ffi.cdef` long bracket goes back to Lua at its closing
/// bracket. The cdef rule's frame carries the bracket's `=` signs.
impl rules::Embeds for Lua {
    type End = LongBracketClose;

    fn embed_halts(
        embedding: &rules::Embedding,
        arena: &Arena,
    ) -> (Self::End, Option<rules::FnHalt>) {
        let level = arena.string(embedding.parent).len();
        (LongBracketClose { level }, None)
    }
}

/// The close of a long bracket with `level` `=` signs, as `]==]` is.
/// The body of an `ffi.cdef[==[` ends there.
#[derive(Clone, Copy, Debug)]
pub(crate) struct LongBracketClose {
    level: usize,
}

impl Halt for LongBracketClose {
    fn first(&self, s: &[u8], from: usize, to: usize) -> Option<usize> {
        let end = to.saturating_add(1).min(s.len());
        let mut p = from;
        while p < end {
            let at = p + memchr::memchr(b']', s.get(p..end)?)?;
            if self.holds(s, at) {
                return Some(at);
            }
            p = at + 1;
        }
        None
    }

    fn holds(&self, s: &[u8], p: usize) -> bool {
        let close = p + 1 + self.level;
        s.get(p) == Some(&b']')
            && s.get(p + 1..close)
                .is_some_and(|run| run.iter().all(|&b| b == b'='))
            && s.get(close) == Some(&b']')
    }
}

/// Lexes `line` like [`lex`], for Lua that another language embeds, up to
/// where `end` or `after` halts it (see [`rules::lex_until`]).
#[inline]
pub(crate) fn lex_until(
    cx: &mut Context<'_>,
    line: Line<'_>,
    state: u32,
    end: &dyn Halt,
    after: Option<&dyn Halt>,
) -> (Finished, Option<Stopped>) {
    rules::lex_until::<Lua>(cx, line, state, |scratch| &mut scratch.lua, end, after)
}

#[cfg(test)]
mod tests;
