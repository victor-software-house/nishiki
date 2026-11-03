//! ERB, with the scopes of the `erb` grammar that Shiki bundles, which comes
//! from `TextMate`'s Ruby bundle: HTML whose `<% %>` tags hold Ruby.
//!
//! The HTML lexer runs it, with ERB's injection in place of the HTML
//! grammar's: wherever no HTML rule matches, outside comments, a `<%` opens
//! a comment or Ruby code, which the Ruby lexer reads up to the `%>` that
//! ends it.

use crate::arena::Arena;
use crate::language::{Context, Line};

/// The state at the start of an ERB document whose scopes sit on the stack
/// `base`.
pub(crate) fn start(arena: &mut Arena, base: u32) -> u32 {
    crate::html::start_erb(arena, base)
}

/// The state at the start of ERB embedded in another language, whose text
/// sits on `base` with no `text.html.erb` of its own; Shiki's Markdown has
/// no ERB fence, so no Markdown block opens it.
pub(crate) fn start_embedded(arena: &mut Arena, base: u32) -> u32 {
    crate::html::start_embedded(arena, base)
}

/// Lexes one line under the crate's lexer contract.
#[inline]
pub(crate) fn lex(cx: &mut Context<'_>, line: Line<'_>, state: u32) -> u32 {
    crate::html::lex_erb(cx, line, state)
}

#[cfg(test)]
mod tests;
