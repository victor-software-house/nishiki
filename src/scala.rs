//! Scala, with the scopes of the `scala` grammar that Shiki bundles.
//! The grammar comes from scala/vscode-scala-syntax.
//!
//! The lexer runs the grammar's begin/end rules with hand-written matchers over
//! bytes, in the `engine` module.
//! The tables in `grammar` are generated from the grammar.
//! The `matchers` decide where each pattern matches.
//! The rules left open at the end of a line (comments, strings, interpolation,
//! imports, XML literals) are frames of the highlighter's interned state.

mod engine;
mod grammar;
mod matchers;
mod nested;
mod scope;

use engine::Scala;
pub(crate) use engine::Scratch;
pub(crate) use scope::NAMES;

use crate::arena::Arena;
use crate::language::{Context, Line};
use crate::limits::Finished;
use crate::rules;

/// Scala's buffers in the highlighter's scratch.
/// The first Scala line allocates them.
fn buffers(scratch: &mut crate::scratch::Scratch) -> &mut Scratch {
    scratch.scala.get_or_insert_with(Box::default)
}

/// The state at the start of a Scala document whose scopes sit on the stack
/// `base`.
pub(crate) fn start(arena: &mut Arena, base: u32) -> u32 {
    rules::start::<Scala>(arena, base)
}

/// The state at the start of the Scala lines of a fenced code block.
pub(crate) fn start_embedded(arena: &mut Arena, base: u32) -> u32 {
    rules::start_embedded::<Scala>(arena, base)
}

/// Lexes one line under the crate's lexer contract.
#[inline]
pub(crate) fn lex(cx: &mut Context<'_>, line: Line<'_>, state: u32) -> Finished {
    rules::lex::<Scala>(cx, line, state, buffers)
}

#[cfg(test)]
mod tests;
