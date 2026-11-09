//! Blade, Laravel's templates, with the scopes of the `blade` grammar that
//! Shiki bundles.
//! The grammar comes from laravel/vs-code-extension: HTML whose echoes,
//! directives and `@php` sections hold PHP.
//!
//! The HTML lexer runs it, with Blade's injection in place of the HTML
//! grammar's.
//! Blade's own rules, such as `{{ … }}` and `@if (…)`, are rules of that lexer.
//! The PHP inside them is Blade's own copy of PHP's rules, which the modules
//! here lex on the shared rule engine ([`crate::rules`]).
//! That PHP hands the line back where the Blade rule ends, and also where
//! Blade's injection starts, at any depth of the PHP, as in Shiki.

mod engine;
mod grammar;
mod matchers;
mod nested;
mod scope;
mod words;

use engine::Blade;
pub(crate) use engine::Scratch;
pub(crate) use scope::NAMES;

use crate::arena::Arena;
use crate::language::{Context, Halts, Line, Stopped};
use crate::limits::Finished;
use crate::rules;

/// The buffers of Blade's PHP in the highlighter's scratch.
/// The first line of PHP allocates them.
fn buffers(scratch: &mut crate::scratch::Scratch) -> &mut Scratch {
    scratch.blade.get_or_insert_with(Box::default)
}

/// The state at the start of a Blade document whose scopes sit on the stack
/// `base`.
pub(crate) fn start(arena: &mut Arena, base: u32) -> u32 {
    crate::html::start_blade(arena, base)
}

/// The state at the start of Blade embedded in another language, whose text
/// sits on `base` with no `text.html.php.blade` of its own.
/// Only Blade's own heredocs, `<<<BLADE`, embed it: Shiki's Markdown has no
/// Blade fence, and no other grammar includes Blade.
pub(crate) fn start_embedded(arena: &mut Arena, base: u32) -> u32 {
    crate::html::start_blade_heredoc(arena, base)
}

/// Lexes one line under the crate's lexer contract.
#[inline]
pub(crate) fn lex(cx: &mut Context<'_>, line: Line<'_>, state: u32) -> Finished {
    crate::html::lex_blade(cx, line, state)
}

/// The state at the start of the PHP of a Blade rule, whose text sits on
/// `base`, `source.php` included.
pub(crate) fn php_start(arena: &mut Arena, base: u32) -> u32 {
    rules::start_embedded::<Blade>(arena, base)
}

/// Lexes the PHP of a Blade rule from `line.start` up to one of `halts`.
/// The first holds the rule's end while no PHP rule is open, and gives
/// Blade's injection at any depth ([`Halt::deep`]).
pub(crate) fn lex_php_until(
    cx: &mut Context<'_>,
    line: Line<'_>,
    state: u32,
    halts: Halts<'_>,
) -> (Finished, Option<Stopped>) {
    rules::lex_embedding_until_resuming::<Blade>(cx, line, state, buffers, halts)
}

#[cfg(test)]
mod tests;
