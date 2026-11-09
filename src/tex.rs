//! TeX, with the scopes of the `tex` grammar that Shiki bundles, from
//! jlelong/vscode-latex-basics through microsoft/vscode.
//!
//! The lexer runs the grammar's begin/end rules on the shared rule engine, in
//! the `engine` module.
//! The tables in `grammar` are generated from the grammar, and `words` holds
//! its long word lists.
//! The `matchers` decide where each regular expression matches.
//! LaTeX's lexer reaches the same matchers for the TeX rules its grammar
//! includes.
//!
//! `\Sexpr{…}` in math hands its text to R.
//! The rules left open at the end of a line are frames of the highlighter's
//! interned state.
//! Shiki's Markdown grammar has no fence of its own for TeX: its `tex` fence
//! embeds LaTeX.

mod engine;
pub(crate) mod grammar;
pub(crate) mod matchers;
mod scope;
mod words;

pub(crate) use engine::Scratch;
use engine::Tex;
pub(crate) use scope::NAMES;

use crate::arena::Arena;
use crate::language::{Context, Line};
use crate::limits::Finished;
use crate::rules;

/// TeX's buffers in the highlighter's scratch.
/// The first TeX line allocates them.
fn buffers(scratch: &mut crate::scratch::Scratch) -> &mut Scratch {
    scratch.tex.get_or_insert_with(Box::default)
}

/// The state at the start of a TeX document whose scopes sit on the stack
/// `base`.
pub(crate) fn start(arena: &mut Arena, base: u32) -> u32 {
    rules::start::<Tex>(arena, base)
}

/// The state at the start of the TeX lines of a fenced code block.
pub(crate) fn start_embedded(arena: &mut Arena, base: u32) -> u32 {
    rules::start_embedded::<Tex>(arena, base)
}

/// Lexes one line under the crate's lexer contract.
/// The text of `\Sexpr{…}` goes to R.
#[inline]
pub(crate) fn lex(cx: &mut Context<'_>, line: Line<'_>, state: u32) -> Finished {
    rules::lex_embedding::<Tex>(cx, line, state, buffers)
}

#[cfg(test)]
mod tests;
