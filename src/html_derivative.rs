//! html-derivative, with the scopes of the `html-derivative` grammar that
//! Shiki bundles, which comes from VS Code: HTML with a rule of its own for
//! the tags that no other rule takes, such as `<Foo/>` or `<x:y>`, which it
//! names `meta.tag.other.unrecognized.html.derivative`. Vue's templates
//! embed it.
//!
//! The HTML lexer runs it, in the grammar's derivative mode.

use crate::arena::Arena;
use crate::language::{Context, Line};
use crate::limits::Finished;

/// The state at the start of an html-derivative document whose scopes sit on
/// the stack `base`.
pub(crate) fn start(arena: &mut Arena, base: u32) -> u32 {
    crate::html::start_derivative(arena, base)
}

/// The state at the start of html-derivative embedded in another language,
/// whose text sits on `base` with no `text.html.derivative` of its own, and
/// where the grammar's injection does not apply, as in Shiki.
pub(crate) fn start_embedded(arena: &mut Arena, base: u32) -> u32 {
    crate::html::start_embedded(arena, base)
}

/// Lexes one line under the crate's lexer contract.
#[inline]
pub(crate) fn lex(cx: &mut Context<'_>, line: Line<'_>, state: u32) -> Finished {
    crate::html::lex_derivative(cx, line, state)
}

#[cfg(test)]
mod tests;
