//! PHP, with the scopes of the `php` grammar that Shiki bundles.
//! The grammar comes from `KapitanOczywisty`'s language-php through VS Code.
//!
//! The lexer runs the grammar's begin/end rules with hand-written matchers over
//! bytes, in the `engine` module.
//! The tables in `grammar` and `words` are generated from the grammar.
//! The `matchers` decide where each pattern matches.
//! The rules left open at the end of a line are frames of the highlighter's
//! interned state.
//!
//! Heredocs and nowdocs named for a language, and strings that start with a
//! SQL keyword, hand their text to that language's lexer.
//! It halts where the rule's end or one of its own patterns takes the line
//! back ([`engine::End`]).
//!
//! A Markdown `php` fence starts in the `Fence` rule, which embeds HTML and
//! lists PHP's rules after it.
//! HTML lexes the line, and halts where one of PHP's patterns starts and none
//! of HTML's own match.

mod engine;
mod grammar;
mod matchers;
mod nested;
mod scope;
mod words;

use engine::Php;
pub(crate) use engine::Scratch;
use grammar::Rule;
pub(crate) use scope::NAMES;

use crate::arena::{Arena, ROOT};
use crate::language::{Context, Halt, Line};
use crate::limits::Finished;
use crate::rules;

/// PHP's buffers in the highlighter's scratch.
/// The first PHP line allocates them.
fn buffers(scratch: &mut crate::scratch::Scratch) -> &mut Scratch {
    scratch.php.get_or_insert_with(Box::default)
}

/// The state at the start of a PHP document whose scopes sit on the stack
/// `base`.
pub(crate) fn start(arena: &mut Arena, base: u32) -> u32 {
    rules::start::<Php>(arena, base)
}

/// The state at the start of PHP that another language embeds, such as the
/// lines of a Markdown fence: the root rule, with the `Fence` rule above it.
/// HTML is not started until the first line hands it the text.
pub(crate) fn start_embedded(arena: &mut Arena, base: u32) -> u32 {
    let root = rules::start_embedded::<Php>(arena, base);
    arena
        .push(root, Rule::Fence.code(), 0, base)
        .unwrap_or(ROOT)
}

/// Lexes one line under the crate's lexer contract.
/// A rule that embeds a language hands its text to that language up to where
/// PHP takes it back. See [`rules::lex_embedding`].
#[inline]
pub(crate) fn lex(cx: &mut Context<'_>, line: Line<'_>, state: u32) -> Finished {
    rules::lex_embedding::<Php>(cx, line, state, buffers)
}

/// Lexes `line` like [`lex`], for PHP that another language embeds up to a
/// halt (see [`rules::lex_until`]).
#[inline]
pub(crate) fn lex_until(
    cx: &mut Context<'_>,
    line: Line<'_>,
    state: u32,
    end: &dyn Halt,
    after: Option<&dyn Halt>,
) -> (Finished, Option<usize>) {
    rules::lex_embedding_until::<Php>(cx, line, state, buffers, (end, after))
}

#[cfg(test)]
mod tests;
