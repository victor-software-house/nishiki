//! HLSL, Microsoft's shading language, with the scopes of the `hlsl` grammar
//! that Shiki bundles.
//! The grammar comes from Visual Studio Code, which converted it from Tim
//! Jones's `shaders-tmLanguage`.
//!
//! The lexer runs the grammar's rules with hand-written matchers over bytes,
//! in the `engine` module.
//! The tables in `grammar` are generated from the grammar.
//! Its `matchers` decide where each pattern matches.
//! The rules left open at the end of a line are frames of the highlighter's
//! interned state.
//! Block comments and strings are such rules.

mod engine;
mod grammar;
mod matchers;
mod scope;
mod words;

use engine::Hlsl;
pub(crate) use engine::Scratch;
pub(crate) use scope::NAMES;

use crate::arena::Arena;
use crate::language::{Context, Halt, Line, Stopped};
use crate::limits::Finished;
use crate::rules;

/// HLSL's buffers in the highlighter's scratch.
/// The first HLSL line allocates them.
fn buffers(scratch: &mut crate::scratch::Scratch) -> &mut Scratch {
    scratch.hlsl.get_or_insert_with(Box::default)
}

/// The state at the start of an HLSL document whose scopes sit on the stack
/// `base`.
pub(crate) fn start(arena: &mut Arena, base: u32) -> u32 {
    rules::start::<Hlsl>(arena, base)
}

/// The state at the start of the HLSL lines of a fenced code block.
pub(crate) fn start_embedded(arena: &mut Arena, base: u32) -> u32 {
    rules::start_embedded::<Hlsl>(arena, base)
}

/// Lexes one line under the crate's lexer contract.
#[inline]
pub(crate) fn lex(cx: &mut Context<'_>, line: Line<'_>, state: u32) -> Finished {
    rules::lex::<Hlsl>(cx, line, state, buffers)
}

/// Lexes `line` like [`lex`], for HLSL that another language embeds, up to
/// where `end` or `after` halts it (see [`rules::lex_until`]).
/// ShaderLab embeds it in a `CGPROGRAM` or `HLSLPROGRAM` block.
#[inline]
pub(crate) fn lex_until(
    cx: &mut Context<'_>,
    line: Line<'_>,
    state: u32,
    end: &dyn Halt,
    after: Option<&dyn Halt>,
) -> (Finished, Option<Stopped>) {
    rules::lex_until::<Hlsl>(cx, line, state, buffers, end, after)
}

#[cfg(test)]
mod tests;
