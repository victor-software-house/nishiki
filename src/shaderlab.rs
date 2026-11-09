//! ShaderLab, Unity's shader language, with the scopes of the `shaderlab`
//! grammar that Shiki bundles.
//! The grammar comes from Visual Studio Code, which converted it from Tim
//! Jones's `shaders-tmLanguage`.
//!
//! The lexer runs the grammar's rules with hand-written matchers over bytes,
//! in the `engine` module.
//! The tables in `grammar` are generated from the grammar.
//! The `matchers` decide where each pattern matches.
//!
//! A `CGPROGRAM` or `HLSLPROGRAM` block hands its text to the HLSL lexer.
//! That lexer halts at the block's end.
//! It also halts where one of Unity's own patterns starts and none of HLSL's
//! matches.
//!
//! The rules left open at the end of a line are frames of the highlighter's
//! interned state.
//! A block's frame keeps HLSL's state.

mod engine;
mod grammar;
mod matchers;
mod scope;
mod words;

pub(crate) use engine::Scratch;
use engine::ShaderLab;
pub(crate) use scope::NAMES;

use crate::arena::Arena;
use crate::language::{Context, Line};
use crate::limits::Finished;
use crate::rules;

/// ShaderLab's buffers in the highlighter's scratch.
/// The first ShaderLab line allocates them.
fn buffers(scratch: &mut crate::scratch::Scratch) -> &mut Scratch {
    scratch.shaderlab.get_or_insert_with(Box::default)
}

/// The state at the start of a ShaderLab document whose scopes sit on the
/// stack `base`.
pub(crate) fn start(arena: &mut Arena, base: u32) -> u32 {
    rules::start::<ShaderLab>(arena, base)
}

/// The state at the start of ShaderLab whose text sits on the stack `base`,
/// with no `source.shaderlab` of its own.
/// No Markdown fence embeds ShaderLab, because Shiki's Markdown grammar has
/// no rule for it.
/// The function completes the contract every language keeps.
pub(crate) fn start_embedded(arena: &mut Arena, base: u32) -> u32 {
    rules::start_embedded::<ShaderLab>(arena, base)
}

/// Lexes one line under the crate's lexer contract.
/// A block's HLSL goes to the HLSL lexer, up to where the block takes the
/// line back ([`rules::lex_embedding`]).
pub(crate) fn lex(cx: &mut Context<'_>, line: Line<'_>, state: u32) -> Finished {
    rules::lex_embedding::<ShaderLab>(cx, line, state, buffers)
}

#[cfg(test)]
mod tests;
