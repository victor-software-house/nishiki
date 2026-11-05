//! Objective-C, with the scopes of the `objective-c` grammar that Shiki
//! bundles.
//! The grammar comes from VS Code, which took it from jeff-hykin's
//! better-objc-syntax.
//!
//! The lexer runs the grammar's begin/end rules with hand-written matchers over
//! bytes, in the `engine` module.
//! The tables in `grammar` are generated from the grammar.
//! The `matchers` decide where each regular expression matches.
//! The rules left open at the end of a line are frames of the highlighter's
//! interned state.
//! Markdown embeds Objective-C in its `objc` fences.

mod c_lang;
mod engine;
mod grammar;
mod matchers;
mod numbers;
mod preproc;
mod scope;
mod words;

use engine::ObjectiveC;
pub(crate) use engine::Scratch;
pub(crate) use scope::NAMES;

use crate::arena::Arena;
use crate::language::{Context, Halt, Line};
use crate::limits::Finished;
use crate::rules;

/// Objective-C's buffers in the highlighter's scratch.
/// The first Objective-C line allocates them.
fn buffers(scratch: &mut crate::scratch::Scratch) -> &mut Scratch {
    scratch.objective_c.get_or_insert_with(Box::default)
}

/// The state at the start of an Objective-C document whose scopes sit on the
/// stack `base`.
pub(crate) fn start(arena: &mut Arena, base: u32) -> u32 {
    rules::start::<ObjectiveC>(arena, base)
}

/// The state at the start of the Objective-C lines of a fenced code block.
pub(crate) fn start_embedded(arena: &mut Arena, base: u32) -> u32 {
    rules::start_embedded::<ObjectiveC>(arena, base)
}

/// Lexes one line under the crate's lexer contract.
#[inline]
pub(crate) fn lex(cx: &mut Context<'_>, line: Line<'_>, state: u32) -> Finished {
    rules::lex::<ObjectiveC>(cx, line, state, buffers)
}

/// Lexes `line` like [`lex`], for Objective-C that another language embeds,
/// up to where `end` or `after` halts it (see [`rules::lex_until`]).
#[inline]
pub(crate) fn lex_until(
    cx: &mut Context<'_>,
    line: Line<'_>,
    state: u32,
    end: &dyn Halt,
    after: Option<&dyn Halt>,
) -> (Finished, Option<usize>) {
    rules::lex_until::<ObjectiveC>(cx, line, state, buffers, end, after)
}

#[cfg(test)]
mod tests;
