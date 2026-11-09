//! Vue, with the scopes of the `vue` grammar that Shiki bundles, which
//! comes from vuejs/language-tools.
//! It covers single-file components whose blocks hold a template, scripts and
//! styles.
//!
//! The HTML lexer runs it, with Vue's rules at the root.
//! A template is html-derivative with Vue's own tag rules.
//! Scripts and styles run on the lexers of the languages they name.

use crate::arena::Arena;
use crate::language::{Context, Line};
use crate::limits::Finished;

/// The state at the start of a Vue component whose scopes sit on the stack
/// `base`.
pub(crate) fn start(arena: &mut Arena, base: u32) -> u32 {
    crate::html::start_vue(arena, base)
}

/// The state at the start of Vue embedded in another language, whose text
/// sits on `base` with no `text.html.vue` of its own.
pub(crate) fn start_embedded(arena: &mut Arena, base: u32) -> u32 {
    crate::html::start_embedded(arena, base)
}

/// Whether `state` is outside every Vue rule.
/// A Markdown `vue` fence closes only there.
#[cfg(feature = "lang-markdown")]
pub(crate) fn at_root(arena: &Arena, state: u32) -> bool {
    crate::html::at_root(arena, state)
}

/// Whether the `while` rule of a Vue `<script>` body in `state` holds at
/// `p` of `s`.
/// Where it fails, a Markdown fence open inside that body closes too.
#[cfg(feature = "lang-markdown")]
pub(crate) fn while_holds(arena: &Arena, state: u32, s: &[u8], p: usize) -> bool {
    crate::html::vue_while_holds(arena, state, s, p)
}

/// The scope stack of the innermost text in `state`, inside the language
/// that a block embeds.
/// A Markdown `vue` fence that opens inside Vue sits on it.
#[cfg(feature = "lang-markdown")]
pub(crate) fn innermost_stack(arena: &Arena, state: u32) -> u32 {
    crate::html::innermost_stack(arena, state)
}

/// Lexes one line under the crate's lexer contract.
#[inline]
pub(crate) fn lex(cx: &mut Context<'_>, line: Line<'_>, state: u32) -> Finished {
    crate::html::lex_vue(cx, line, state)
}
