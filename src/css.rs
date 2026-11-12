//! CSS, with the scopes of the `css` grammar that Shiki bundles.
//! The grammar comes from VS Code.
//!
//! The lexer runs the grammar's begin/end rules with hand-written matchers over
//! bytes, in the `engine` module.
//! The tables in `grammar` are generated from the grammar.
//! Its `matchers` decide where each pattern matches.
//! The rules left open at the end of a line (rule lists, at-rules, comments,
//! strings, selectors) are frames of the highlighter's interned state.

mod ahead;
pub(crate) mod engine;
mod grammar;
mod matchers;
mod nested;
mod scope;
#[rustfmt::skip]
mod words;

use engine::Css;
pub(crate) use engine::Scratch;
#[cfg(feature = "lang-scss")]
pub(crate) use engine::{CssLike, Extra, unit_name_case};
#[cfg(all(test, feature = "lang-scss"))]
pub(crate) use grammar::UNITS;
#[cfg(feature = "lang-scss")]
pub(crate) use grammar::{Nested, Pattern};
pub(crate) use scope::NAMES;
#[cfg(feature = "lang-scss")]
pub(crate) use scope::S;

use crate::arena::Arena;
use crate::language::{Context, Halt, Line, Stopped};
use crate::limits::Finished;
use crate::rules;

/// The state at the start of a CSS document whose scopes sit on the stack
/// `base`.
pub(crate) fn start(arena: &mut Arena, base: u32) -> u32 {
    rules::start::<Css>(arena, base)
}

/// The state at the start of the CSS lines of a fenced code block.
#[cfg(any(feature = "_start-embedded", feature = "lang-html"))]
pub(crate) fn start_embedded(arena: &mut Arena, base: u32) -> u32 {
    rules::start_embedded::<Css>(arena, base)
}

/// The state at the start of a Vue `style` attribute's string, whose
/// text sits on the stack `base`.
/// Its rule holds the string's patterns, which include CSS's.
#[cfg(feature = "lang-vue")]
pub(crate) fn start_vue_style_attribute(arena: &mut Arena, base: u32) -> u32 {
    let code = grammar::Rule::VueStyleAttribute.code();
    arena
        .push(crate::arena::ROOT, code, 0, base)
        .unwrap_or(crate::arena::ROOT)
}

/// Lexes one line under the crate's lexer contract.
#[inline]
pub(crate) fn lex(cx: &mut Context<'_>, line: Line<'_>, state: u32) -> Finished {
    rules::lex::<Css>(cx, line, state, |scratch| &mut scratch.css)
}

/// Lexes `line` like [`lex`], for CSS that another language embeds, up to
/// where `end` or `after` halts it (see [`rules::lex_until`]).
#[inline]
pub(crate) fn lex_until(
    cx: &mut Context<'_>,
    line: Line<'_>,
    state: u32,
    end: &dyn Halt,
    after: Option<&dyn Halt>,
) -> (Finished, Option<Stopped>) {
    rules::lex_until::<Css>(cx, line, state, |scratch| &mut scratch.css, end, after)
}

#[cfg(test)]
mod tests;
