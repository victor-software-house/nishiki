//! nishiki, a syntax highlighting engine written from scratch.
//!
//! nishiki splits each line of source into tokens and gives every token the
//! scope stack Shiki gives it, so a theme in Shiki's format colours the result
//! as Shiki would. Lexers resume line by line: the state after one line is the
//! input of the next, so an editor can re-highlight from any line, and a
//! streamed reply can be highlighted as it arrives.
//!
//! [`Language::ALL`] lists the languages. Markdown ([`Language::Markdown`])
//! hands the code in its fences to the other lexers.
//!
//! ```
//! use nishiki::{Highlighter, Language, State};
//!
//! let mut highlighter = Highlighter::new(Language::Rust);
//! let mut state = State::default();
//! let mut tokens = Vec::new();
//! let line = "let x = 42;";
//! let report = highlighter.line(line, &mut state, &mut tokens);
//! assert!(report.is_complete());
//!
//! let mut start = 0;
//! let mut innermost = Vec::new();
//! for token in &tokens {
//!     let scope = highlighter.scopes(token.stack()).next();
//!     innermost.push((&line[start..token.end()], scope.unwrap_or_default()));
//!     start = token.end();
//! }
//! assert_eq!(
//!     innermost,
//!     [
//!         ("let", "storage.type.rust"),
//!         (" ", "source.rust"),
//!         ("x", "variable.other.rust"),
//!         (" ", "source.rust"),
//!         ("=", "keyword.operator.assignment.equal.rust"),
//!         (" ", "source.rust"),
//!         ("42", "constant.numeric.decimal.rust"),
//!         (";", "punctuation.semi.rust"),
//!     ]
//! );
//! ```
//!
//! Every line is bounded by [`Limits`], which the caller can set, raise,
//! lower or turn off; each [`LineReport`] says which limits a line reached.
//!
//! A [`Theme`] in Shiki's format turns a token's scopes into a [`Style`], as
//! Shiki would colour it; the `json` feature reads theme files.

// docs.rs labels feature-gated items, such as `Theme::from_json`, with their
// feature.
#![cfg_attr(docsrs, feature(doc_cfg))]

mod arena;
#[cfg(feature = "lang-c")]
mod c;
#[cfg(feature = "_classes")]
mod classes;
#[cfg(feature = "lang-cpp")]
mod cpp;
#[cfg(feature = "lang-csharp")]
mod csharp;
#[cfg(feature = "lang-css")]
mod css;
#[cfg(feature = "lang-diff")]
mod diff;
#[cfg(feature = "lang-dockerfile")]
mod dockerfile;
#[cfg(any(feature = "lang-json", feature = "lang-toml", feature = "lang-yaml"))]
mod emit;
#[cfg(any(feature = "lang-c", feature = "lang-python"))]
mod fnv;
#[cfg(feature = "lang-go")]
mod go;
#[cfg(feature = "lang-graphql")]
mod graphql;
mod highlighter;
#[cfg(feature = "lang-html")]
mod html;
#[cfg(feature = "lang-ini")]
mod ini;
#[cfg(feature = "lang-java")]
mod java;
#[cfg(feature = "lang-json")]
mod json;
#[cfg(feature = "lang-kotlin")]
mod kotlin;
mod language;
mod limits;
#[cfg(feature = "lang-make")]
mod make;
#[cfg(feature = "lang-markdown")]
mod markdown;
#[cfg(feature = "lang-powershell")]
mod powershell;
#[cfg(feature = "lang-python")]
mod python;
#[cfg(feature = "lang-ruby")]
mod ruby;
#[cfg(feature = "_rule-engine")]
mod rules;
#[cfg(feature = "lang-rust")]
mod rust;
#[cfg(feature = "lang-scala")]
mod scala;
mod scope;
mod scratch;
#[cfg(feature = "lang-shell")]
mod shell;
#[cfg(feature = "_space")]
mod space;
#[cfg(feature = "lang-sql")]
mod sql;
#[cfg(feature = "lang-swift")]
mod swift;
mod tag;
#[cfg(test)]
mod test_support;
#[cfg(feature = "_textmate")]
mod textmate;
mod theme;
#[cfg(feature = "lang-toml")]
mod toml;
#[cfg(any(
    feature = "lang-typescript",
    feature = "lang-javascript",
    feature = "lang-jsx",
    feature = "lang-tsx"
))]
mod typescript;
#[cfg(feature = "_utf8")]
mod utf8;
#[cfg(feature = "lang-xml")]
mod xml;
#[cfg(feature = "lang-yaml")]
mod yaml;

pub use arena::Scopes;
pub use highlighter::Highlighter;
pub use language::Language;
pub use limits::{Budget, Limit, Limits, LineReport, Work};
#[cfg(feature = "json")]
pub use theme::ThemeError;
pub use theme::{Color, FontStyle, RuleScope, Style, Theme, ThemeKind, ThemeRule, ThemeSource};

/// The README's examples, compiled and run as doctests.
#[cfg(doctest)]
#[doc = include_str!("../README.md")]
struct ReadmeDoctests;

/// A run of bytes in one line that shares one scope stack.
///
/// A token starts where the previous token of its line ended, or at byte 0,
/// and ends at [`Token::end`]. The tokens of a line cover it without gaps.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Token {
    end: usize,
    stack: Stack,
}

impl Token {
    /// The byte offset in the line where the token ends, exclusive.
    #[must_use]
    pub fn end(self) -> usize {
        self.end
    }

    /// The token's scope stack, which [`Highlighter::scopes`] resolves to
    /// scope names.
    #[must_use]
    pub fn stack(self) -> Stack {
        self.stack
    }
}

/// An interned scope stack, which only the highlighter that produced it can
/// resolve, with [`Highlighter::scopes`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Stack(u32);

/// A lexer state between two lines: the contexts left open, such as a
/// string or a comment.
///
/// A small `Copy` handle into the state its [`Highlighter`] interned, so
/// equal states compare equal in one step. [`State::default`] starts a
/// document. A state belongs to the highlighter that produced it.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct State(u32);
