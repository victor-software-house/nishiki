//! Ruby, with the scopes of the `ruby` grammar that Shiki bundles, which
//! comes from Shopify's ruby-lsp through VS Code.
//!
//! The lexer runs the grammar's begin/end rules with hand-written matchers
//! over bytes (see the `engine` module): the tables in `grammar` are
//! generated from the grammar, and the `matchers` decide where each pattern
//! matches. The rules left open at the end of a line (strings, heredocs,
//! `=begin` comments, blocks) are frames of the highlighter's interned state.

mod engine;
mod grammar;
mod matchers;
mod nested;
mod scope;

use engine::Ruby;
pub(crate) use engine::Scratch;
pub(crate) use scope::NAMES;

use crate::arena::Arena;
use crate::language::{Context, Line};
use crate::rules;

/// The state at the start of a Ruby document whose scopes sit on the stack
/// `base`.
pub(crate) fn start(arena: &mut Arena, base: u32) -> u32 {
    rules::start::<Ruby>(arena, base)
}

/// The state at the start of the Ruby lines of a fenced code block.
pub(crate) fn start_embedded(arena: &mut Arena, base: u32) -> u32 {
    rules::start_embedded::<Ruby>(arena, base)
}

/// Lexes one line under the crate's lexer contract. A heredoc named for a
/// language hands its text to that language up to where Ruby's own patterns
/// take it back ([`interpolation`] and [`escape`]); see
/// [`rules::lex_embedding`].
#[inline]
pub(crate) fn lex(cx: &mut Context<'_>, line: Line<'_>, state: u32) -> u32 {
    rules::lex_embedding::<Ruby>(
        cx,
        line,
        state,
        |scratch| &mut scratch.ruby,
        interpolation,
        Some(escape),
    )
}

/// Where Ruby's interpolation or a heredoc at the start of a line begins:
/// `#{`, `#@name`, `#@@name`, `#$name` or `^<<[-~]?name`, the patterns a
/// language-named heredoc lists before the embedded grammar.
fn interpolation(s: &[u8], p: usize) -> bool {
    let name = |at: usize| {
        s.get(at)
            .is_some_and(|&b| b.is_ascii_alphabetic() || b == b'_')
    };
    match s.get(p) {
        Some(b'#') => match s.get(p + 1) {
            Some(b'{') => true,
            Some(b'@') => name(p + 2) || (s.get(p + 2) == Some(&b'@') && name(p + 3)),
            Some(b'$') => name(p + 2),
            _ => false,
        },
        Some(b'<') if p == 0 && s.get(1) == Some(&b'<') => {
            let at = 2 + usize::from(matches!(s.get(2), Some(b'-' | b'~')));
            name(at)
        }
        _ => false,
    }
}

/// Where Ruby's escape `\\.` begins, which a language-named heredoc lists
/// after the embedded grammar, so a match of that grammar there wins.
fn escape(s: &[u8], p: usize) -> bool {
    s.get(p) == Some(&b'\\') && p + 1 < s.len()
}

/// Lexes `line` like [`lex`], for Ruby that ERB embeds up to the end of its
/// code (see [`rules::lex_until`]).
#[cfg(feature = "lang-erb")]
#[inline]
pub(crate) fn lex_until(
    cx: &mut Context<'_>,
    line: Line<'_>,
    state: u32,
    end: crate::language::Halt,
) -> (u32, Option<usize>) {
    rules::lex_until::<Ruby>(cx, line, state, |scratch| &mut scratch.ruby, end, None)
}

#[cfg(test)]
mod tests;
