//! Markdown, with the scopes of the Markdown grammar that Shiki bundles.
//!
//! The lexer is a hand-written port of the grammar's rules to byte matchers,
//! run by the loop of the `TextMate` engine. Its contexts, the containers left
//! open at the end of a line (quotes, lists, fences, tables, paragraphs), are
//! frames of the highlighter's interned state.
//!
//! A fenced code block in a language nishiki lexes is handed to that lexer
//! line by line, and a `markdown` fence to Markdown's own rules. Its scopes
//! sit inside the fence's embedded scope, such as `meta.embedded.block.rust`,
//! without the language's own root scope, as in Shiki. Any other fence is
//! plain text with `fenced_code.block.language` on its info string: Shiki
//! drops the grammar's rule for a language whose grammar is not loaded, and
//! that is what its output shows. Adding a fence language adds an entry to
//! the table in `fence.rs` and its embedded scope in `rules::content_name`.
//!
//! ```
//! use nishiki::{Highlighter, Language, State};
//!
//! let mut highlighter = Highlighter::new(Language::Markdown);
//! let mut state = State::default();
//! let mut tokens = Vec::new();
//! for line in ["```rust", "let x = 1;"] {
//!     tokens.clear();
//!     let report = highlighter.line(line, &mut state, &mut tokens);
//!     assert!(report.is_complete());
//! }
//! let mut scopes: Vec<&str> = highlighter.scopes(tokens[0].stack()).collect();
//! scopes.reverse();
//! assert_eq!(
//!     scopes,
//!     [
//!         "text.html.markdown",
//!         "markup.fenced_code.block.markdown",
//!         "meta.embedded.block.rust",
//!         "keyword.other.rust",
//!         "storage.type.rust",
//!     ]
//! );
//! ```
//!
//! It reproduces Shiki's output, quirks included:
//!
//! 1. A line holding only whitespace is lexed but an empty line is not, so the
//!    state passes over it unchanged.
//! 2. A fence indented by four spaces or more is still a fence, and a bare
//!    fence line inside an embedded fence that is not its closing fence ends
//!    the embedded lines and starts them again on the next line.
//! 3. HTML blocks other than comments, front matter, and the tags inside
//!    emphasis are plain text, since the grammar hands them to grammars that
//!    Shiki does not load.
//! 4. The `>` of an email autolink has no scope of its own.
//!
//! The highlighter's [`Limits`](crate::Limits) bound it. Contexts nested deeper
//! than `Limits::depth` (containers, emphasis, link text) are not opened. Each
//! step of the lexer's loop, and each byte a look-ahead reads, counts against
//! `Limits::work`, so a line of `**` or `[` repeated ends at the limit instead
//! of costing time quadratic in its length; the rest of that line is one
//! token. A label or link destination nested deeper than 32 brackets or
//! parentheses ends there, which only bounds the matcher's recursion.
//!
//! Where it differs from the grammar's regular expressions otherwise:
//!
//! 1. The regular expressions backtrack in ways that rarely matter, and the
//!    lexer follows the alternatives that do: a code span gives back backticks,
//!    a heading's closing `#` run, a table cell's trailing blanks and a
//!    reference definition's destination and title. It does not follow every
//!    one, such as an escaped `>` that ends an angle-bracket destination.
//! 2. A fence of more than 255 characters, or indented with a tab by more than
//!    three characters, is never closed by a fence of the same kind.

mod block;
mod caps;
mod fence;
#[cfg(feature = "lang-html-derivative")]
mod html;
mod inline;
mod lexer;
mod limits;
mod rules;
mod scope;
mod state;
mod text;
mod ticks;

pub(crate) use lexer::Scratch;
pub(crate) use scope::NAMES;
use scope::S;
use state::Kind;

use crate::arena::{Arena, ROOT};
use crate::limits::Finished;

/// The state at the start of a Markdown document whose scopes sit on the
/// stack `base`: one context with `text.html.markdown` on top of it.
pub(crate) fn start(arena: &mut Arena, base: u32) -> u32 {
    let stack = arena.child(base, S::Root.scope()).unwrap_or(base);
    arena
        .push(ROOT, Kind::Base.code(), 1, stack)
        .unwrap_or(ROOT)
}

/// The state at the start of a Markdown body whose text sits on the stack
/// `base` with no root scope of its own.
///
/// No Markdown fence embeds Markdown through this: a Markdown fence is lexed
/// by the Markdown lexer's own rules. It completes the contract every
/// language keeps.
pub(crate) fn start_embedded(arena: &mut Arena, base: u32) -> u32 {
    arena.push(ROOT, Kind::Base.code(), 1, base).unwrap_or(ROOT)
}

/// Lexes one line from `state`, appending its tokens, and returns the state
/// after it. An empty line appends none and leaves the state as it was, as it
/// does in Shiki.
pub(crate) fn line(cx: &mut crate::language::Context<'_>, line: &str, state: u32) -> Finished {
    if line.is_empty() {
        return Finished::unspent(state);
    }
    let (parts, scratch) = cx.parts();
    let mut lexer = lexer::Lexer::new(parts, scratch, line, state);
    lexer.run();
    lexer.finish()
}

/// The tests that name the languages Markdown embeds, so they need them all.
#[cfg(all(test, feature = "all"))]
mod tests;

#[cfg(all(test, not(all(feature = "lang-rust", feature = "lang-yaml"))))]
mod subset_tests;

/// Lexes `line` from `state` under the crate's one lexer contract
/// ([`Language::lex`](crate::Language)), with token ends counted from the
/// start of the whole line, and returns the state after it.
#[inline]
pub(crate) fn lex(
    cx: &mut crate::language::Context<'_>,
    line: crate::language::Line<'_>,
    state: u32,
) -> Finished {
    cx.lex_from_start(line, |cx, rest| self::line(cx, rest, state))
}
