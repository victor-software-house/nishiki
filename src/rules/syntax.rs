//! A grammar a caller brings to the rule engine, as a [`Syntax`].

use std::marker::PhantomData;

use crate::arena::Arena;
use crate::highlighter::{Highlighter, Sealed, Syntax, lex_line};
#[cfg(nishiki_unstable)]
use crate::limits::Limits;
use crate::limits::LineReport;
use crate::rules::{Grammar, Scratch, lex_with, start};
use crate::{State, Token};

/// The [`Syntax`] of a [`Highlighter`] that runs grammar `G` on the rule
/// engine.
///
/// [`Highlighter::for_rules`] makes such a highlighter.
/// It lexes as a rule-engine language does.
/// Its buffers sit in the highlighter's slot for its syntax, because the
/// highlighter's scratch has no field for `G`.
///
/// Its lexer is generic over `G`, so it compiles in the caller's crate.
/// A [`crate::Language`]'s lexer stays compiled in nishiki.
///
/// A crate outside nishiki cannot write a `Grammar` yet: it has no way to
/// make a `Scope`.
/// `unstable::codegen` brings that, with the tables a grammar needs.
#[cfg_attr(
    not(nishiki_unstable),
    expect(unreachable_pub, reason = "public only through nishiki::unstable")
)]
#[cfg_attr(
    all(not(nishiki_unstable), not(all(test, feature = "lang-toml"))),
    expect(
        dead_code,
        reason = "used only through nishiki::unstable and the tests"
    )
)]
pub struct RuleSyntax<G>(PhantomData<fn() -> G>);

impl<G> Clone for RuleSyntax<G> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<G> Copy for RuleSyntax<G> {}

impl<G> std::fmt::Debug for RuleSyntax<G> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("RuleSyntax")
            .field(&std::any::type_name::<G>())
            .finish()
    }
}

impl<G: Grammar> Syntax for RuleSyntax<G> {}

impl<G: Grammar> Sealed for RuleSyntax<G> {
    type Lexer = RuleSyntax<G>;
    type Buffers = Scratch<G>;

    fn start(_: RuleSyntax<G>, arena: &mut Arena, base: u32) -> u32 {
        start::<G>(arena, base)
    }

    fn line(
        highlighter: &mut Highlighter<RuleSyntax<G>>,
        line: &str,
        state: &mut State,
        out: &mut Vec<Token>,
    ) -> LineReport {
        lex_line(
            highlighter,
            line,
            state,
            out,
            |_, buffers, cx, text, from| lex_with::<G>(cx, text, from, buffers).state(),
        )
    }
}

// The constructors exist only with the cfg.
// Without it, these public methods would make the rule engine's items
// reachable from outside the crate.
#[cfg(nishiki_unstable)]
impl<G: Grammar> Highlighter<RuleSyntax<G>> {
    /// A highlighter for grammar `G` with [`Limits::default`].
    #[must_use]
    pub fn for_rules() -> Self {
        Self::for_rules_with_limits(Limits::default())
    }

    /// A highlighter for grammar `G` with `limits`.
    #[must_use]
    pub fn for_rules_with_limits(limits: Limits) -> Self {
        Self::with_lexer(RuleSyntax(PhantomData), limits)
    }
}

#[cfg(test)]
#[cfg(feature = "lang-toml")]
mod tests;
