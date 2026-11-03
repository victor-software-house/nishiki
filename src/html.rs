//! HTML, with the scopes of the `html` grammar that Shiki bundles, which
//! comes from VS Code.
//!
//! The lexer runs the grammar's begin/end rules with hand-written matchers
//! over bytes (see the `engine` module). The rules left open at the end of a
//! line (a tag with its attributes, a comment, a quoted value, an `svg`
//! element, the CSS of a `<style>` element) are frames of the highlighter's
//! interned state; the tag and attribute names that the grammar copies into
//! scope names are made at run time.
//!
//! It reproduces Shiki's output, quirks included: the grammar's injection
//! flags a `<` that no rule claims, even inside a quoted value; an attribute
//! name is a word of the grammar's lists or any other run of characters; the
//! `svg` and `math` elements have rules of their own, and read the
//! attributes of a start tag that the line holds whole on that text alone;
//! and the CSS and JavaScript of `<style>` and `<script>` elements run on the
//! CSS and TypeScript lexers, which halt at the end tag.
//!
//! An empty line leaves the state as it is, because Shiki does not tokenize
//! empty lines.

mod engine;
mod erb;
mod matchers;
mod rules;
mod scope;
mod state;
mod words;

pub(crate) use engine::Scratch;
pub(crate) use scope::NAMES;
use scope::S;
use state::{ROOT_EMBEDDED, Rule};

use crate::arena::{Arena, ROOT};
use crate::language::{Context, Line};

/// The state at the start of an HTML document whose scopes sit on the stack
/// `base`: one rule with `text.html.basic` on top of it.
pub(crate) fn start(arena: &mut Arena, base: u32) -> u32 {
    let stack = arena.child(base, S::TextHtmlBasic.scope()).unwrap_or(base);
    arena
        .push(ROOT, Rule::Root.code(), 0, stack)
        .unwrap_or(ROOT)
}

/// The state at the start of the HTML lines of a fenced code block, whose
/// text sits on the stack `base` with no `text.html.basic` of its own, as in
/// Shiki's embedding. The grammar's injection does not apply there, as it
/// does not when another grammar embeds this one.
pub(crate) fn start_embedded(arena: &mut Arena, base: u32) -> u32 {
    arena
        .push(ROOT, Rule::Root.code(), ROOT_EMBEDDED, base)
        .unwrap_or(ROOT)
}

/// Lexes `line` from `state`, under the crate's one lexer contract
/// ([`Language::lex`](crate::Language)), with token ends counted from the
/// start of the whole line, and returns the state after it.
///
/// An empty line leaves the state as it is: Shiki does not tokenize empty
/// lines.
#[inline]
pub(crate) fn lex(cx: &mut Context<'_>, line: Line<'_>, state: u32) -> u32 {
    if line.text.len() <= line.start {
        return state;
    }
    engine::Lexer::new(cx, line, state, engine::Document::Html).run()
}

/// The state at the start of an ERB document whose scopes sit on the stack
/// `base`: the HTML root rule with `text.html.erb` on top of it.
#[cfg(feature = "lang-erb")]
pub(crate) fn start_erb(arena: &mut Arena, base: u32) -> u32 {
    let stack = arena.child(base, S::TextHtmlErb.scope()).unwrap_or(base);
    arena
        .push(ROOT, Rule::Root.code(), 0, stack)
        .unwrap_or(ROOT)
}

/// Lexes a line of an ERB document like [`lex`], with ERB's injection in
/// place of the grammar's.
#[cfg(feature = "lang-erb")]
pub(crate) fn lex_erb(cx: &mut Context<'_>, line: Line<'_>, state: u32) -> u32 {
    if line.text.len() <= line.start {
        return state;
    }
    engine::Lexer::new(cx, line, state, engine::Document::Erb).run()
}

#[cfg(test)]
mod tests;
