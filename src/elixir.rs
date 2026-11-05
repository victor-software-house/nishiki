//! Elixir, with the scopes of the `elixir` grammar that Shiki bundles.
//! The grammar comes from elixir-editors/elixir-tmbundle.
//!
//! The lexer runs the grammar's begin/end rules with hand-written matchers
//! over bytes, in the `engine` module.
//! The tables in `grammar` are generated from the grammar.
//! The `matchers` decide where each pattern matches.
//! The rules left open at the end of a line, such as heredocs, sigils and
//! `def` heads, are frames of the highlighter's interned state.
//!
//! The `~H"""` and `~L"""` sigils hold HTML, which HTML's lexer lexes up to
//! the sigil's end (see [`rules::lex_embedding`]).
//! Most lines hold no such sigil, and they are lexed without HTML
//! (see [`rules::lex_on_demand`]).

mod engine;
mod grammar;
mod matchers;
mod scope;

use engine::Elixir;
pub(crate) use engine::Scratch;
pub(crate) use scope::NAMES;

use crate::arena::Arena;
use crate::language::{Context, Halt, Line};
use crate::limits::Finished;
use crate::rules;

/// Elixir's buffers in the highlighter's scratch.
/// The first Elixir line allocates them.
fn buffers(scratch: &mut crate::scratch::Scratch) -> &mut Scratch {
    scratch.elixir.get_or_insert_with(Box::default)
}

/// The state at the start of an Elixir document whose scopes sit on the
/// stack `base`.
pub(crate) fn start(arena: &mut Arena, base: u32) -> u32 {
    rules::start::<Elixir>(arena, base)
}

/// The state at the start of the Elixir lines of a fenced code block.
pub(crate) fn start_embedded(arena: &mut Arena, base: u32) -> u32 {
    rules::start_embedded::<Elixir>(arena, base)
}

/// Lexes one line under the crate's lexer contract.
#[inline]
pub(crate) fn lex(cx: &mut Context<'_>, line: Line<'_>, state: u32) -> Finished {
    rules::lex_on_demand::<Elixir>(cx, line, state, buffers)
}

/// Lexes `line` like [`lex`], for Elixir that another language embeds, up to
/// where `end` or `after` halts it (see [`rules::lex_embedding_until`]).
#[inline]
pub(crate) fn lex_until(
    cx: &mut Context<'_>,
    line: Line<'_>,
    state: u32,
    end: &dyn Halt,
    after: Option<&dyn Halt>,
) -> (Finished, Option<usize>) {
    rules::lex_embedding_until::<Elixir>(cx, line, state, buffers, (end, after))
}

#[cfg(test)]
mod tests;
