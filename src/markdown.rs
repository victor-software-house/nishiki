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
//! The highlighter's [`Limits`](crate::Limits) bound it.
//! Contexts nested deeper than `Limits::depth` are not opened: containers,
//! emphasis and link text.
//! Each step of the lexer's loop, and each byte a look-ahead reads, counts
//! against `Limits::work`.
//! A line that runs out of work ends in one token from that point.
//! Lines such as `**`, `[` or `![a](` repeated cost work linear in their
//! length, so they stay within the default limit.
//! A label or link destination nested deeper than 32 brackets or parentheses
//! ends there.
//! That bound only limits the matcher's recursion.
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
mod runs;
mod scope;
mod state;
mod stops;
mod text;
mod titles;

pub(crate) use lexer::Scratch;
pub(crate) use scope::NAMES;
use scope::S;
use state::Kind;

use crate::arena::{Arena, ROOT};
use crate::limits::Finished;

/// The scope that a Markdown document's stack starts with.
#[cfg(feature = "lang-html")]
pub(crate) const DOCUMENT: crate::scope::Scope = S::Root.scope();

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

/// Lexes `line` from byte `line.start` like [`line`], for Markdown that
/// another language embeds, as Svelte's `@component` comments do.
/// The line halts where `end` holds at Markdown's root, and the second value
/// is where.
/// `after` never holds: Markdown's patterns match at the start of every
/// line, and nowhere else at the document's level.
///
/// The end can hold in two places:
/// - At the line's start, once the open containers that end there have closed.
///   The halt is tested here, once, and [`line`] reads the answer from the
///   scratch: a second copy of the line's lexer would keep its parts from being
///   inlined into [`line`] (2% more instructions on Markdown documents).
/// - After a block that closed inside the line, such as a raw HTML block at
///   `</style>`. The rest of the line is then at Markdown's root, where `\G` no
///   longer holds, so no block opens there. The rest is one plain token, and
///   the end is tested from its start.
///
/// The character before `line.start` goes to the lexer through the scratch,
/// for the lookbehinds at the start ([`text::LineFacts::before_start`]).
pub(crate) fn lex_until(
    cx: &mut crate::language::Context<'_>,
    line: crate::language::Line<'_>,
    state: u32,
    end: &dyn crate::language::Halt,
    _after: Option<&dyn crate::language::Halt>,
) -> (Finished, Option<usize>) {
    let text = line.text.as_bytes();
    cx.scratch.markdown.start = lexer::Start::Embedded {
        stop: end.holds(text, line.start),
        before: line
            .text
            .get(..line.start)
            .and_then(|before| before.chars().next_back()),
    };
    let first = cx.out.len();
    let finished = cx.lex_from_start(line, |cx, rest| self::line(cx, rest, state));
    cx.scratch.markdown.start = lexer::Start::Document;
    if std::mem::take(&mut cx.scratch.markdown.stopped) {
        return (finished, Some(line.start));
    }
    let halt = halt_after_block(cx, line, end, first, finished.state());
    (finished, halt)
}

/// Where `end` holds in the rest of a line that a block closed inside, now
/// at Markdown's root in the state `state`, and the rest's token cut there.
/// The line's tokens start at `first` in the output.
/// The rest is the line's last token, on the root's stack, after another
/// token of the line.
fn halt_after_block(
    cx: &mut crate::language::Context<'_>,
    line: crate::language::Line<'_>,
    end: &dyn crate::language::Halt,
    first: usize,
    state: u32,
) -> Option<usize> {
    let root = cx.arena.frame(state);
    if Kind::from_code(root.kind) != Kind::Base || cx.out.len() < first + 2 {
        return None;
    }
    let last = cx.out.len() - 1;
    let rest = cx.out[last];
    let from = cx.out[last - 1].end();
    if rest.stack().0 != root.stack {
        return None;
    }
    let halt = end.first(line.text.as_bytes(), from, rest.end())?;
    if halt >= rest.end() {
        return None;
    }
    if halt > from {
        cx.out[last].end = halt;
    } else {
        cx.out.truncate(last);
    }
    Some(halt)
}

/// The tests that name the languages Markdown embeds, so they need them all.
#[cfg(all(test, feature = "all"))]
mod tests;

#[cfg(all(test, not(all(feature = "lang-rust", feature = "lang-yaml"))))]
mod subset_tests;
