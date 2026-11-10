//! html-derivative, with the scopes of the `html-derivative` grammar that
//! Shiki bundles, which comes from VS Code.
//! It is HTML with a rule of its own for the tags that no other rule takes,
//! such as `<Foo/>` or `<x:y>`.
//! The rule names them `meta.tag.other.unrecognized.html.derivative`.
//! Vue's templates embed it.
//!
//! The HTML lexer runs it, in the grammar's derivative mode.

use crate::arena::Arena;
use crate::language::{Context, Halt, Line, Stopped};
use crate::limits::Finished;

/// The state at the start of an html-derivative document whose scopes sit on
/// the stack `base`.
pub(crate) fn start(arena: &mut Arena, base: u32) -> u32 {
    crate::html::start_derivative(arena, base)
}

/// The state at the start of html-derivative embedded in another language.
/// Its text sits on `base` with no `text.html.derivative` of its own.
/// The grammar's injection does not apply there, as in Shiki.
pub(crate) fn start_embedded(arena: &mut Arena, base: u32) -> u32 {
    crate::html::start_embedded(arena, base)
}

/// Lexes one line under the crate's lexer contract.
#[inline]
pub(crate) fn lex(cx: &mut Context<'_>, line: Line<'_>, state: u32) -> Finished {
    crate::html::lex_derivative(cx, line, state)
}

/// Lexes `line` like [`lex`], for html-derivative that another language
/// embeds from [`start_embedded`].
/// While no rule of html-derivative is open, the line halts at the first of
/// these places:
///
/// - where `end` holds before the root's patterns;
/// - where `after` holds and none of them matches.
///
/// The second value is where.
#[inline]
pub(crate) fn lex_until(
    cx: &mut Context<'_>,
    line: Line<'_>,
    state: u32,
    end: &dyn Halt,
    after: Option<&dyn Halt>,
) -> (Finished, Option<Stopped>) {
    let halts = (end, after);
    let give_back = crate::html::GiveBack::Halts(&halts);
    let (finished, halt) = crate::html::lex_derivative_until(
        cx,
        line,
        state,
        crate::html::Span::Line,
        Some(give_back),
    );
    (finished, halt.map(Stopped::halt))
}

#[cfg(test)]
mod tests;
