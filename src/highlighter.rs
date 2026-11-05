//! The highlighter: one language, its interned stacks and states, and its
//! limits.

use crate::arena::{Arena, ROOT, Scopes};
use crate::language::{Context, Line};
use crate::limits::{Limit, Limits, LineReport};
use crate::scratch::Scratch;
use crate::{Language, Stack, State, Style, Theme, Token};

/// Highlights one language, a line at a time.
///
/// It owns the scope stacks and states its tokens and [`State`]s name.
/// Both therefore belong to the highlighter that made them.
/// Lines are lexed in order from [`State::default`].
/// The state after one line is the input of the next.
/// An editor can therefore keep each line's state and re-highlight from any
/// line.
#[derive(Debug)]
pub struct Highlighter {
    language: Language,
    limits: Limits,
    arena: Arena,
    /// Buffers and caches of the lexers, one field per language.
    scratch: Scratch,
    /// The state at the start of a document.
    /// [`State::default`] stands for it.
    start: u32,
    /// The styles [`Highlighter::style`] computed, for one theme.
    styles: Styles,
}

/// Each stack's style under one theme, indexed by stack id, filled on first
/// use.
#[derive(Debug, Default)]
struct Styles {
    /// The [`Theme`] id the styles belong to; `None` before the first call.
    theme: Option<u64>,
    by_stack: Vec<Option<Style>>,
}

impl Highlighter {
    /// A highlighter for `language` with [`Limits::default`].
    #[must_use]
    pub fn new(language: Language) -> Self {
        Self::new_with_limits(language, Limits::default())
    }

    /// A highlighter for `language` with `limits`.
    #[must_use]
    pub fn new_with_limits(language: Language, limits: Limits) -> Self {
        let mut arena = Arena::new(limits.nodes);
        let start = language.start(&mut arena, ROOT);
        Highlighter {
            language,
            limits,
            arena,
            scratch: Scratch::default(),
            start,
            styles: Styles::default(),
        }
    }

    /// The language this highlighter lexes.
    #[must_use]
    pub fn language(&self) -> Language {
        self.language
    }

    /// The limits this highlighter applies to every line.
    #[must_use]
    pub fn limits(&self) -> Limits {
        self.limits
    }

    /// Forgets every interned state and stack, and keeps the language and
    /// limits.
    ///
    /// It resets the fields in place.
    /// With no language feature on, `Language` has no variants, and neither
    /// has `Highlighter`.
    /// Code after a call that returns either is then unreachable, which
    /// `check:features` denies.
    pub(crate) fn renew(&mut self) {
        self.arena = Arena::new(self.limits.nodes);
        self.start = self.language.start(&mut self.arena, ROOT);
        self.scratch = Scratch::default();
    }

    /// Lexes `line`, which excludes its line ending, starting from `state`.
    ///
    /// Appends tokens that cover every byte of `line` in order.
    /// An empty line appends none.
    /// It leaves in `state` the state for the next line.
    /// It reports the limits the line reached.
    pub fn line(&mut self, line: &str, state: &mut State, out: &mut Vec<Token>) -> LineReport {
        let mut report = LineReport::default();
        let from = if state.0 == ROOT { self.start } else { state.0 };
        if self.limits.line_bytes.is_some_and(|max| line.len() > max) {
            report.record(Limit::LineBytes);
            let stack = Stack(self.arena.frame(from).stack);
            out.push(Token {
                end: line.len(),
                stack,
            });
            return report;
        }
        self.scratch.start_line();
        let first_token = out.len();
        let mut cx = Context {
            scratch: &mut self.scratch,
            arena: &mut self.arena,
            limits: &self.limits,
            out,
            report: &mut report,
            first_token,
        };
        let text = Line {
            text: line,
            start: 0,
            embedded: false,
        };
        let to = self.language.lex(&mut cx, text, from);
        *state = State(if to == self.start { ROOT } else { to });
        report
    }

    /// The scope names of `stack`, innermost first, such as
    /// `"string.quoted.double.rust"` then `"source.rust"`.
    ///
    /// Empty when `stack` is unknown to this highlighter.
    /// A stack from another highlighter resolves to an unrelated stack or to
    /// nothing.
    #[must_use]
    pub fn scopes(&self, stack: Stack) -> Scopes<'_> {
        self.arena.scopes(stack.0)
    }

    /// The style `theme` gives `stack`: the style [`Theme::style`] gives the
    /// stack's [`scopes`](Highlighter::scopes).
    ///
    /// The highlighter keeps each stack's style once computed.
    /// Styling another token with the same stack is then one lookup.
    /// It keeps the styles of one theme, and drops them when called with
    /// another.
    /// It allocates only when a new stack first appears.
    #[must_use]
    pub fn style(&mut self, theme: &Theme, stack: Stack) -> Style {
        if self.styles.theme != Some(theme.id()) {
            self.styles.theme = Some(theme.id());
            self.styles.by_stack.clear();
        }
        let cached = usize::try_from(stack.0)
            .ok()
            .and_then(|index| self.styles.by_stack.get(index))
            .copied()
            .flatten();
        match cached {
            Some(style) => style,
            None => self.style_miss(theme, stack.0),
        }
    }

    /// [`Highlighter::style`] for a stack not styled yet: [`Theme::style`] on
    /// its scopes, kept for the next token with the stack.
    ///
    /// It goes through [`Theme::style`], so the theme's trie keeps one caller.
    /// With a second caller, the trie stopped being inlined into
    /// [`Theme::style`], and every caller of it ran about 4% more
    /// instructions.
    ///
    /// Out of line, so a hit saves no registers for the miss.
    #[inline(never)]
    fn style_miss(&mut self, theme: &Theme, stack: u32) -> Style {
        let count = self.arena.stack_count();
        let style = theme.style(self.arena.scopes(stack));
        if let Some(index) = usize::try_from(stack).ok().filter(|&index| index < count) {
            if self.styles.by_stack.len() < count {
                self.styles.by_stack.resize(count, None);
            }
            if let Some(slot) = self.styles.by_stack.get_mut(index) {
                *slot = Some(style);
            }
        }
        style
    }

    /// The open context of `state`, for the lexers' tests.
    #[cfg(test)]
    pub(crate) fn frame(&self, state: State) -> crate::arena::Frame {
        self.arena
            .frame(if state.0 == ROOT { self.start } else { state.0 })
    }
}
