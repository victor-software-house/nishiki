//! The highlighter: what it lexes, its interned stacks and states, and its
//! limits.

use std::fmt::Debug;

use crate::arena::{Arena, ROOT, Scopes};
use crate::language::{Context, Line};
use crate::limits::{Limit, Limits, LineReport};
use crate::scratch::Scratch;
use crate::{Language, Stack, State, Style, Theme, Token};

/// What a [`Highlighter`] lexes, such as a [`Language`].
///
/// The trait is sealed: only nishiki implements it.
#[expect(
    private_bounds,
    reason = "the crate-private supertrait seals the trait and holds what the highlighter calls"
)]
pub trait Syntax: Sealed {}

/// What a [`Syntax`] gives the highlighter.
///
/// It is crate-private, so no caller can implement [`Syntax`].
pub(crate) trait Sealed: Sized {
    /// What the highlighter keeps to lex a line, such as the [`Language`]
    /// itself.
    type Lexer: Copy + Debug;

    /// The state at the start of a document whose scopes sit on `base`.
    fn start(lexer: Self::Lexer, arena: &mut Arena, base: u32) -> u32;

    /// [`Highlighter::line`] for this syntax.
    ///
    /// Each syntax implements it without generics, so nishiki compiles it.
    /// A generic body would be compiled in the caller's crate instead.
    /// That changes what LLVM inlines on the hand lexers' path.
    fn line(
        highlighter: &mut Highlighter<Self>,
        line: &str,
        state: &mut State,
        out: &mut Vec<Token>,
    ) -> LineReport
    where
        Self: Syntax;
}

impl Syntax for Language {}

impl Sealed for Language {
    type Lexer = Language;

    fn start(lexer: Language, arena: &mut Arena, base: u32) -> u32 {
        lexer.start(arena, base)
    }

    fn line(
        highlighter: &mut Highlighter<Language>,
        line: &str,
        state: &mut State,
        out: &mut Vec<Token>,
    ) -> LineReport {
        lex_line(highlighter, line, state, out, |language, cx, text, from| {
            language.lex(cx, text, from)
        })
    }
}

/// Highlights text in one [`Syntax`], `S`, a line at a time.
///
/// `S` is a [`Language`] unless named.
///
/// It owns the scope stacks and states its tokens and [`State`]s name.
/// Both therefore belong to the highlighter that made them.
/// Lines are lexed in order from [`State::default`].
/// The state after one line is the input of the next.
/// An editor can therefore keep each line's state and re-highlight from any
/// line.
#[derive(Debug)]
pub struct Highlighter<S: Syntax = Language> {
    /// What the highlighter lexes with.
    lexer: S::Lexer,
    /// Everything else the highlighter keeps.
    core: Core,
}

/// The limits, stacks, states, buffers and styles of a highlighter.
///
/// They do not depend on its [`Syntax`].
/// Their methods are therefore not generic, so nishiki compiles them once.
#[derive(Debug)]
struct Core {
    limits: Limits,
    arena: Arena,
    /// Buffers and caches of the lexers, one field per language.
    scratch: Scratch,
    /// The state at the start of a document.
    /// [`State::default`] stands for it.
    start: u32,
    /// The styles [`Highlighter::style`] computed, for the last two themes.
    styles: Styles,
}

/// The styles of the theme used last and of the one before it.
///
/// A renderer can style each token with a light and a dark theme in turn.
/// Two slots keep both themes' styles across those calls.
#[derive(Debug, Default)]
struct Styles {
    current: ThemeStyles,
    previous: ThemeStyles,
}

impl Styles {
    /// Makes `theme` the current slot.
    /// The previous slot becomes current when it holds `theme`.
    /// Otherwise the older slot is emptied for `theme`.
    fn switch(&mut self, theme: u64) {
        std::mem::swap(&mut self.current, &mut self.previous);
        if self.current.theme != Some(theme) {
            self.current.theme = Some(theme);
            self.current.by_stack.clear();
        }
    }

    /// Empties both slots, and keeps their themes and their memory.
    fn forget(&mut self) {
        self.current.by_stack.clear();
        self.previous.by_stack.clear();
    }

    /// The current slot's style for `stack`, if it has one.
    fn cached(&self, stack: u32) -> Option<Style> {
        usize::try_from(stack)
            .ok()
            .and_then(|index| self.current.by_stack.get(index))
            .copied()
            .flatten()
    }
}

/// Each stack's style under one theme, indexed by stack id, filled on first
/// use.
#[derive(Debug, Default)]
struct ThemeStyles {
    /// The [`Theme`] id the styles belong to; `None` before its first call.
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
        Self::with_lexer(language, limits)
    }

    /// The language this highlighter lexes.
    #[must_use]
    pub fn language(&self) -> Language {
        self.lexer
    }
}

impl<S: Syntax> Highlighter<S> {
    /// A highlighter that lexes with `lexer` under `limits`.
    pub(crate) fn with_lexer(lexer: S::Lexer, limits: Limits) -> Self {
        let mut arena = Arena::new(limits.nodes);
        let start = S::start(lexer, &mut arena, ROOT);
        Highlighter {
            lexer,
            core: Core {
                limits,
                arena,
                scratch: Scratch::default(),
                start,
                styles: Styles::default(),
            },
        }
    }

    /// What the highlighter lexes with.
    #[cfg(feature = "textmate")]
    pub(crate) fn lexer(&self) -> S::Lexer {
        self.lexer
    }

    /// The limits this highlighter applies to every line.
    #[must_use]
    pub fn limits(&self) -> Limits {
        self.core.limits
    }

    /// Forgets every interned state and stack, and keeps the syntax and
    /// limits.
    ///
    /// It resets the fields in place.
    /// With no language feature on, `Language` has no variants, and neither
    /// has `Highlighter<Language>`.
    /// Code after a call that returns either is then unreachable, which
    /// `check:features` denies.
    ///
    /// It also forgets every cached style.
    /// The new arena gives its ids to other stacks, so a style kept for an
    /// old id would colour an unrelated stack.
    pub(crate) fn renew(&mut self) {
        let core = &mut self.core;
        core.arena = Arena::new(core.limits.nodes);
        core.start = S::start(self.lexer, &mut core.arena, ROOT);
        core.scratch = Scratch::default();
        core.styles.forget();
    }

    /// Lexes `line`, which excludes its line ending, starting from `state`.
    ///
    /// Appends tokens that cover every byte of `line` in order.
    /// An empty line appends none.
    /// It leaves in `state` the state for the next line.
    /// It reports the limits the line reached.
    pub fn line(&mut self, line: &str, state: &mut State, out: &mut Vec<Token>) -> LineReport {
        S::line(self, line, state, out)
    }

    /// The scope names of `stack`, innermost first, such as
    /// `"string.quoted.double.rust"` then `"source.rust"`.
    ///
    /// Empty when `stack` is unknown to this highlighter.
    /// A stack from another highlighter resolves to an unrelated stack or to
    /// nothing.
    #[must_use]
    pub fn scopes(&self, stack: Stack) -> Scopes<'_> {
        self.core.arena.scopes(stack.0)
    }

    /// The style `theme` gives `stack`: the style [`Theme::style`] gives the
    /// stack's [`scopes`](Highlighter::scopes).
    ///
    /// The highlighter keeps each stack's style once computed.
    /// Styling another token with the same stack is then one lookup.
    /// It keeps the styles of the two themes used last.
    /// A call with a third theme drops the styles of the older one.
    /// It allocates only when a new stack first appears.
    #[must_use]
    pub fn style(&mut self, theme: &Theme, stack: Stack) -> Style {
        self.core.style(theme, stack)
    }

    /// The style [`Highlighter::style`] keeps for `stack` under `theme`.
    ///
    /// It reads whichever slot holds `theme`.
    /// It is `None` for a stack not styled yet, and for a theme in neither
    /// slot, so no caller reads a style left from another theme.
    pub(crate) fn cached_style(&self, theme: &Theme, stack: Stack) -> Option<Style> {
        let slot = [&self.core.styles.current, &self.core.styles.previous]
            .into_iter()
            .find(|slot| slot.theme == Some(theme.id()))?;
        usize::try_from(stack.0)
            .ok()
            .and_then(|index| slot.by_stack.get(index))
            .copied()
            .flatten()
    }

    /// The open context of `state`, for the lexers' tests.
    #[cfg(test)]
    #[cfg(feature = "_language")]
    pub(crate) fn frame(&self, state: State) -> crate::arena::Frame {
        self.core.arena.frame(if state.0 == ROOT {
            self.core.start
        } else {
            state.0
        })
    }

    /// The lexers' buffers, for the engine's tests.
    #[cfg(test)]
    #[cfg(feature = "_textmate")]
    pub(crate) fn scratch_mut(&mut self) -> &mut Scratch {
        &mut self.core.scratch
    }
}

/// [`Highlighter::line`] with `lex` as the syntax's lexer.
///
/// Each [`Sealed::line`] calls it with its own `lex`, and is its one caller
/// for that syntax, so LLVM inlines it there.
pub(crate) fn lex_line<S: Syntax>(
    highlighter: &mut Highlighter<S>,
    line: &str,
    state: &mut State,
    out: &mut Vec<Token>,
    lex: impl FnOnce(S::Lexer, &mut Context<'_>, Line<'_>, u32) -> u32,
) -> LineReport {
    let Highlighter { lexer, core } = highlighter;
    let mut report = LineReport::default();
    let from = if state.0 == ROOT { core.start } else { state.0 };
    if core.limits.line_bytes.is_some_and(|max| line.len() > max) {
        report.record(Limit::LineBytes);
        let stack = Stack(core.arena.frame(from).stack);
        out.push(Token {
            end: line.len(),
            stack,
        });
        return report;
    }
    core.scratch.start_line();
    let first_token = out.len();
    let mut cx = Context {
        scratch: &mut core.scratch,
        arena: &mut core.arena,
        limits: &core.limits,
        out,
        report: &mut report,
        first_token,
    };
    let text = Line {
        text: line,
        start: 0,
        embedded: false,
    };
    let to = lex(*lexer, &mut cx, text, from);
    *state = State(if to == core.start { ROOT } else { to });
    report
}

impl Core {
    /// [`Highlighter::style`].
    ///
    /// Not generic, so nishiki compiles it once, whatever the syntax.
    fn style(&mut self, theme: &Theme, stack: Stack) -> Style {
        if self.styles.current.theme != Some(theme.id()) {
            return self.style_switch(theme, stack.0);
        }
        match self.styles.cached(stack.0) {
            Some(style) => style,
            None => self.style_miss(theme, stack.0),
        }
    }

    /// [`Highlighter::style`] with a theme other than the current slot's:
    /// switches slots, then styles `stack`.
    ///
    /// Out of line, so a call with the current theme saves no registers.
    /// With the switch inlined into [`Highlighter::style`], every call saved
    /// four registers, and colouring JSON ran 4% more instructions.
    #[inline(never)]
    fn style_switch(&mut self, theme: &Theme, stack: u32) -> Style {
        self.styles.switch(theme.id());
        match self.styles.cached(stack) {
            Some(style) => style,
            None => self.style_miss(theme, stack),
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
            let by_stack = &mut self.styles.current.by_stack;
            if by_stack.len() < count {
                by_stack.resize(count, None);
            }
            if let Some(slot) = by_stack.get_mut(index) {
                *slot = Some(style);
            }
        }
        style
    }
}

#[cfg(all(test, feature = "lang-python"))]
mod tests {
    use super::*;
    use crate::theme::{RuleScope, ThemeRule, ThemeSource};

    fn rule(scope: &str, foreground: &str) -> ThemeRule {
        ThemeRule {
            scope: RuleScope::Selectors(scope.into()),
            foreground: Some(foreground.into()),
            ..ThemeRule::default()
        }
    }

    fn styles(highlighter: &mut Highlighter, theme: &Theme, line: &str) -> Vec<(Style, Style)> {
        let mut state = State::default();
        let mut tokens = Vec::new();
        let _ = highlighter.line(line, &mut state, &mut tokens);
        tokens
            .iter()
            .map(|token| {
                (
                    highlighter.style(theme, token.stack()),
                    theme.style(highlighter.scopes(token.stack())),
                )
            })
            .collect()
    }

    #[test]
    fn it_forgets_its_cached_styles_when_renewed() {
        let theme = Theme::new(ThemeSource {
            editor_foreground: Some("#000000".into()),
            editor_background: Some("#ffffff".into()),
            rules: vec![rule("string", "#ff0000"), rule("comment", "#00ff00")],
            ..ThemeSource::default()
        });
        let mut highlighter = Highlighter::new(Language::Python);
        let _ = styles(&mut highlighter, &theme, "x = 'a'");
        highlighter.renew();
        for (cached, direct) in styles(&mut highlighter, &theme, "# a comment") {
            assert_eq!(cached, direct);
        }
    }
}
