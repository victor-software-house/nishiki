//! Zig, run by nishiki's `TextMate` engine on the `zig` grammar
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

/// The state at the start of a Zig document whose scopes sit on the
/// stack `base`, with `source.zig` on top.
pub(crate) fn start(arena: &mut Arena, base: u32) -> u32 {
    textmate::start(arena, &GRAMMAR, base)
}

/// The state at the start of Zig lines embedded in another language, whose
/// text sits on the stack `base` with no `source.zig` of its own, as Shiki
/// embeds a grammar. Shiki's Markdown grammar has no Zig fence, so no
/// language embeds Zig yet; every language provides this
/// (`Language::start_embedded`).
pub(crate) fn start_embedded(arena: &mut Arena, base: u32) -> u32 {
    textmate::start_embedded(arena, &GRAMMAR, base)
}

/// Lexes `line` from `state` under the crate's one lexer contract
/// (`Language::lex`).
pub(crate) fn lex(cx: &mut Context<'_>, line: Line<'_>, state: u32) -> u32 {
    textmate::lex(cx, &GRAMMAR, line, state)
}
