//! Sass, the indented syntax, with the scopes of the `sass` grammar that Shiki
//! bundles.
//! The grammar comes from `vscode-sass-indented`, by `TheRealSyler`.
//!
//! The lexer runs the grammar's begin/end rules with hand-written matchers over
//! bytes, in the `engine` module.
//! The tables in `grammar` are generated from the grammar.
//! The `matchers` decide where each pattern matches.
//! The rules left open at the end of a line are frames of the highlighter's
//! interned state.
//! A block comment is such a rule: it goes on while the next lines are
//! indented deeper than its `/*`.
//! Shiki's Markdown grammar has no Sass fence.
//! Vue's `<style lang="sass">` and HAML's `:sass` filter embed Sass through
//! [`lex_until`].
mod engine;
mod grammar;
mod matchers;
mod scope;

use engine::Sass;
pub(crate) use engine::Scratch;
pub(crate) use scope::NAMES;

use crate::arena::Arena;
use crate::language::{Context, Halt, Line};
use crate::limits::Finished;
use crate::rules;

/// Sass's buffers in the highlighter's scratch.
/// The first Sass line allocates them.
fn buffers(scratch: &mut crate::scratch::Scratch) -> &mut Scratch {
    scratch.sass.get_or_insert_with(Box::default)
}

/// The state at the start of a Sass document whose scopes sit on the stack
/// `base`.
pub(crate) fn start(arena: &mut Arena, base: u32) -> u32 {
    rules::start::<Sass>(arena, base)
}

/// The state at the start of the Sass lines of a fenced code block.
pub(crate) fn start_embedded(arena: &mut Arena, base: u32) -> u32 {
    rules::start_embedded::<Sass>(arena, base)
}

/// Lexes one line under the crate's lexer contract.
#[inline]
pub(crate) fn lex(cx: &mut Context<'_>, line: Line<'_>, state: u32) -> Finished {
    rules::lex::<Sass>(cx, line, state, buffers)
}

/// Lexes `line` like [`lex`], for Sass that another language embeds, up to
/// where `end` or `after` halts it (see [`rules::lex_until`]).
/// Vue's `<style lang="sass">` and HAML's `:sass` filter embed it.
pub(crate) fn lex_until(
    cx: &mut Context<'_>,
    line: Line<'_>,
    state: u32,
    end: &dyn Halt,
    after: Option<&dyn Halt>,
) -> (Finished, Option<usize>) {
    rules::lex_until::<Sass>(cx, line, state, buffers, end, after)
}

#[cfg(test)]
mod tests;
