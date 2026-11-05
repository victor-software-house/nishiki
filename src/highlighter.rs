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
    ///
    /// It also forgets every cached style.
    /// The new arena gives its ids to other stacks, so a style kept for an
    /// old id would colour an unrelated stack.
    pub(crate) fn renew(&mut self) {
        self.arena = Arena::new(self.limits.nodes);
        self.start = self.language.start(&mut self.arena, ROOT);
        self.scratch = Scratch::default();
        self.styles.forget();
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
    /// It keeps the styles of the two themes used last.
    /// A call with a third theme drops the styles of the older one.
    /// It allocates only when a new stack first appears.
    #[must_use]
    pub fn style(&mut self, theme: &Theme, stack: Stack) -> Style {
        if self.styles.current.theme != Some(theme.id()) {
            return self.style_switch(theme, stack.0);
        }
        match self.styles.cached(stack.0) {
            Some(style) => style,
            None => self.style_miss(theme, stack.0),
        }
    }

    /// The style [`Highlighter::style`] keeps for `stack` under `theme`.
    ///
    /// It reads whichever slot holds `theme`.
    /// It is `None` for a stack not styled yet, and for a theme in neither
    /// slot, so no caller reads a style left from another theme.
    pub(crate) fn cached_style(&self, theme: &Theme, stack: Stack) -> Option<Style> {
        let slot = [&self.styles.current, &self.styles.previous]
            .into_iter()
            .find(|slot| slot.theme == Some(theme.id()))?;
        usize::try_from(stack.0)
            .ok()
            .and_then(|index| slot.by_stack.get(index))
            .copied()
            .flatten()
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

    /// The open context of `state`, for the lexers' tests.
    #[cfg(test)]
    #[cfg(feature = "_language")]
    pub(crate) fn frame(&self, state: State) -> crate::arena::Frame {
        self.arena
            .frame(if state.0 == ROOT { self.start } else { state.0 })
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
