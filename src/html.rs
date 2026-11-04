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
mod vue;
mod words;

#[cfg(all(feature = "lang-html-derivative", feature = "lang-markdown"))]
pub(crate) use engine::GiveBack;
pub(crate) use engine::Scratch;
pub(crate) use scope::NAMES;
use scope::S;
use state::{ROOT_EMBEDDED, Rule};

use crate::arena::{Arena, ROOT};
use crate::language::{Context, Halt, Line};
use crate::limits::Finished;

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
pub(crate) fn lex(cx: &mut Context<'_>, line: Line<'_>, state: u32) -> Finished {
    if line.text.len() <= line.start {
        return Finished::unspent(state);
    }
    engine::Lexer::new(cx, line, state, engine::Document::Html).run()
}

/// Lexes `line` like [`lex`], for HTML that another language embeds from
/// [`start_embedded`]: the line halts at the first place where `end` holds
/// before the root's patterns, or `after` holds where none of them matches,
/// while no rule of HTML is open, and the second value is where.
#[inline]
pub(crate) fn lex_until(
    cx: &mut Context<'_>,
    line: Line<'_>,
    state: u32,
    end: &dyn Halt,
    after: Option<&dyn Halt>,
) -> (Finished, Option<usize>) {
    if line.text.len() <= line.start {
        return (Finished::unspent(state), None);
    }
    let halts = (end, after);
    let mut lexer = engine::Lexer::new(cx, line, state, engine::Document::Html);
    lexer.give_back = Some(engine::GiveBack::Halts(&halts));
    let (finished, halt) = lexer.run_until();
    (finished, halt)
}

/// The state at the start of an html-derivative document whose scopes sit on
/// the stack `base`: the HTML root rule with `text.html.derivative` on top of
/// it.
#[cfg(feature = "lang-html-derivative")]
pub(crate) fn start_derivative(arena: &mut Arena, base: u32) -> u32 {
    let stack = arena
        .child(base, S::TextHtmlDerivative.scope())
        .unwrap_or(base);
    arena
        .push(ROOT, Rule::Root.code(), 0, stack)
        .unwrap_or(ROOT)
}

/// Lexes a line of an html-derivative document like [`lex`], with the
/// grammar's own rule for the tags no other rule takes.
#[cfg(feature = "lang-html-derivative")]
pub(crate) fn lex_derivative(cx: &mut Context<'_>, line: Line<'_>, state: u32) -> Finished {
    if line.text.len() <= line.start {
        return Finished::unspent(state);
    }
    engine::Lexer::new(cx, line, state, engine::Document::Derivative).run()
}

/// Whether a line feed follows the text that html-derivative is given.
#[cfg(feature = "lang-html-derivative")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Span {
    /// A line, which a line feed ends.
    Line,
    /// A capture group, which `TextMate` lexes again on its text alone.
    Capture,
}

/// Lexes html-derivative in a rule of another grammar that includes the
/// grammar's root patterns or hands it a capture group: from `line.start`
/// and `state` up to where `give_back` returns the line, or to the end of the
/// text. Returns the state there and, when the line went back to the including
/// grammar, where.
#[cfg(feature = "lang-html-derivative")]
pub(crate) fn lex_derivative_until(
    cx: &mut Context<'_>,
    line: Line<'_>,
    state: u32,
    span: Span,
    give_back: Option<engine::GiveBack<'_>>,
) -> (Finished, Option<usize>) {
    if line.text.len() <= line.start {
        return (Finished::unspent(state), None);
    }
    let mut lexer = engine::Lexer::new(cx, line, state, engine::Document::Derivative);
    lexer.lf = span == Span::Line;
    lexer.give_back = give_back;
    lexer.run_until()
}

/// Whether one of html-derivative's root patterns matches at `line.start`,
/// with the steps the test spent. It tests as it would from `state`, an
/// embedded start. It lexes nothing.
#[cfg(feature = "lang-html-derivative")]
pub(crate) fn derivative_opens_at(
    cx: &mut Context<'_>,
    line: Line<'_>,
    state: u32,
    span: Span,
) -> (bool, u64) {
    let start = line.start;
    let mut lexer = engine::Lexer::new(cx, line, state, engine::Document::Derivative);
    lexer.lf = span == Span::Line;
    lexer.opens_at(start)
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
pub(crate) fn lex_erb(cx: &mut Context<'_>, line: Line<'_>, state: u32) -> Finished {
    if line.text.len() <= line.start {
        return Finished::unspent(state);
    }
    engine::Lexer::new(cx, line, state, engine::Document::Erb).run()
}

/// The state at the start of a Vue component whose scopes sit on the stack
/// `base`: the HTML root rule with `text.html.vue` on top of it.
#[cfg(feature = "lang-vue")]
pub(crate) fn start_vue(arena: &mut Arena, base: u32) -> u32 {
    let stack = arena.child(base, S::TextHtmlVue.scope()).unwrap_or(base);
    arena
        .push(ROOT, Rule::Root.code(), 0, stack)
        .unwrap_or(ROOT)
}

/// Whether `state` is outside every rule.
#[cfg(all(feature = "lang-vue", feature = "lang-markdown"))]
pub(crate) fn at_root(arena: &Arena, state: u32) -> bool {
    arena.frame(state).kind == Rule::Root.code()
}

/// Whether the `while` rule of a Vue body on top of `state` holds at `p` of
/// `s`: `^(?!\s*</script[>\s])`.
/// A state with no such body on top has no `while` rule to fail.
#[cfg(all(feature = "lang-vue", feature = "lang-markdown"))]
pub(crate) fn vue_while_holds(arena: &Arena, state: u32, s: &[u8], p: usize) -> bool {
    Rule::from_code(arena.frame(state).kind) != Rule::VueBodyWhile
        || (p == 0 && !vue::script_end(s, vue::blanks_end(s, 0)))
}

/// The scope stack of the innermost text in `state`.
/// Where the innermost rule embeds another language, that language's state
/// holds it.
#[cfg(all(feature = "lang-vue", feature = "lang-markdown"))]
pub(crate) fn innermost_stack(arena: &Arena, state: u32) -> u32 {
    let frame = arena.frame(state);
    let embeds = matches!(
        Rule::from_code(frame.kind),
        Rule::BodyCss | Rule::BodyJs | Rule::VueBody | Rule::VueBodyWhile | Rule::VueExpr
    );
    if embeds && frame.payload != 0 {
        arena.frame(frame.payload).stack
    } else {
        frame.stack
    }
}

/// Lexes a line of a Vue component like [`lex`], with Vue's rules at the
/// root.
#[cfg(feature = "lang-vue")]
pub(crate) fn lex_vue(cx: &mut Context<'_>, line: Line<'_>, state: u32) -> Finished {
    if line.text.len() <= line.start {
        return Finished::unspent(state);
    }
    engine::Lexer::new(cx, line, state, engine::Document::Vue).run()
}

#[cfg(test)]
mod tests;
