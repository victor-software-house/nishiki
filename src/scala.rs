//! Scala, run by nishiki's `TextMate` engine on the `scala` grammar
//! that Shiki bundles, which `oracle/engine-tables.ts` compiles into
//! `grammar.rs`.
//!
//! The language has no lexer of its own: the engine runs the grammar's rules
//! as Shiki does, and names their scopes at run time, so the language has no
//! scope table or frame tag of its own either.

mod grammar;

use grammar::GRAMMAR;

use crate::arena::Arena;
use crate::language::{Context, Line};
use crate::textmate;

/// The state at the start of a Scala document whose scopes sit on the
/// stack `base`, with `source.scala` on top.
pub(crate) fn start(arena: &mut Arena, base: u32) -> u32 {
    textmate::start(arena, &GRAMMAR, base)
}

/// The state at the start of the Scala lines of a Markdown fence, whose
/// text sits on the stack `base` with no `source.scala` of its own, as in
/// Shiki's embedding.
pub(crate) fn start_embedded(arena: &mut Arena, base: u32) -> u32 {
    textmate::start_embedded(arena, &GRAMMAR, base)
}

/// Lexes `line` from `state` under the crate's one lexer contract
/// (`Language::lex`).
pub(crate) fn lex(cx: &mut Context<'_>, line: Line<'_>, state: u32) -> u32 {
    textmate::lex(cx, &GRAMMAR, line, state)
}
