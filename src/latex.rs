//! LaTeX, with the scopes of the `latex` grammar that Shiki bundles, from
//! jlelong/vscode-latex-basics through microsoft/vscode.
//!
//! The lexer runs the grammar's begin/end rules on the shared rule engine, in
//! the `engine` module.
//! The tables in `grammar` are generated from the grammar, and `words` holds
//! its long word list.
//!
//! The grammar includes the TeX grammar, whose expressions TeX's matchers
//! answer; the `matchers` here decide where the rest match.
//! Code environments hand their text to the language they name.
//! The rules left open at the end of a line are frames of the highlighter's
//! interned state.
//! Shiki's Markdown grammar embeds LaTeX in its `latex` and `tex` fences.

mod engine;
mod grammar;
mod matchers;
mod scope;
mod words;

use engine::Latex;
pub(crate) use engine::Scratch;
pub(crate) use scope::NAMES;

use crate::arena::Arena;
use crate::language::{Context, Halt, Line, Stopped};
use crate::limits::Finished;
use crate::rules;

/// LaTeX's buffers in the highlighter's scratch.
/// The first LaTeX line allocates them.
fn buffers(scratch: &mut crate::scratch::Scratch) -> &mut Scratch {
    scratch.latex.get_or_insert_with(Box::default)
}

/// The state at the start of a LaTeX document whose scopes sit on the stack
/// `base`.
pub(crate) fn start(arena: &mut Arena, base: u32) -> u32 {
    rules::start::<Latex>(arena, base)
}

/// The state at the start of the LaTeX lines of a fenced code block.
pub(crate) fn start_embedded(arena: &mut Arena, base: u32) -> u32 {
    rules::start_embedded::<Latex>(arena, base)
}

/// Lexes one line under the crate's lexer contract.
/// A code environment hands its text to the language it names.
#[inline]
pub(crate) fn lex(cx: &mut Context<'_>, line: Line<'_>, state: u32) -> Finished {
    rules::lex_embedding::<Latex>(cx, line, state, buffers)
}

/// Lexes `line` like [`lex`], for LaTeX that another language embeds.
/// The line halts where `end` holds before LaTeX's patterns are tried.
/// It also halts where `after` holds and the patterns fail.
/// Both apply only while no LaTeX rule is open.
/// The second value holds where the line stopped, and why.
#[inline]
pub(crate) fn lex_until(
    cx: &mut Context<'_>,
    line: Line<'_>,
    state: u32,
    end: &dyn Halt,
    after: Option<&dyn Halt>,
) -> (Finished, Option<Stopped>) {
    rules::lex_embedding_until::<Latex>(cx, line, state, buffers, (end, after))
}

#[cfg(test)]
mod tests;
