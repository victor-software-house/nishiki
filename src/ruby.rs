//! Ruby, with the scopes of the `ruby` grammar that Shiki bundles.
//! The grammar comes from Shopify's ruby-lsp through VS Code.
//!
//! The lexer runs the grammar's begin/end rules with hand-written matchers over
//! bytes, in the `engine` module.
//! The tables in `grammar` are generated from the grammar.
//! Its `matchers` decide where each pattern matches.
//! The rules left open at the end of a line (strings, heredocs, `=begin`
//! comments, blocks) are frames of the highlighter's interned state.

pub(crate) mod engine;
mod grammar;
mod matchers;
mod nested;
mod scope;

use engine::Ruby;
pub(crate) use engine::Scratch;
pub(crate) use scope::NAMES;

use crate::arena::Arena;
use crate::language::{Context, Line, Stopped};
use crate::limits::Finished;
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

/// Lexes one line under the crate's lexer contract.
/// A heredoc named for a language hands its text to that language up to where
/// Ruby's own patterns take it back ([`interpolation`] and [`escape`]).
/// See [`rules::lex_embedding`].
#[inline]
pub(crate) fn lex(cx: &mut Context<'_>, line: Line<'_>, state: u32) -> Finished {
    rules::lex_embedding::<Ruby>(cx, line, state, |scratch| &mut scratch.ruby)
}

/// A heredoc named for a language takes the line back where Ruby's own patterns
/// begin. [`interpolation`] is tried before the embedded grammar.
/// [`escape`] is tried after it.
impl rules::Embeds for Ruby {
    type End = fn(&[u8], usize) -> bool;

    fn embed_halts(_: &rules::Embedding, _: &Arena) -> (Self::End, Option<rules::FnHalt>) {
        (interpolation, Some(escape))
    }
}

/// Where Ruby's interpolation or a heredoc at the start of a line begins:
/// `#{`, `#@name`, `#@@name`, `#$name` or `^<<[-~]?name`.
/// These are the patterns a language-named heredoc lists before the embedded
/// grammar.
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

/// Where Ruby's escape `\\.` begins.
/// A language-named heredoc lists it after the embedded grammar.
/// A match of that grammar there therefore wins.
fn escape(s: &[u8], p: usize) -> bool {
    s.get(p) == Some(&b'\\') && p + 1 < s.len()
}

/// Lexes `line` like [`lex`], for Ruby that another language embeds up to a
/// halt (see [`rules::lex_until`]). ERB embeds Ruby up to the end of its code.
/// HAML embeds a line of Ruby up to the rules HAML lists around it.
#[inline]
pub(crate) fn lex_until(
    cx: &mut Context<'_>,
    line: Line<'_>,
    state: u32,
    end: &dyn crate::language::Halt,
    after: Option<&dyn crate::language::Halt>,
) -> (Finished, Option<Stopped>) {
    rules::lex_until::<Ruby>(cx, line, state, |scratch| &mut scratch.ruby, end, after)
}

#[cfg(test)]
mod tests;
