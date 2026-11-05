//! nishiki, a syntax highlighting engine written from scratch.
//!
//! nishiki splits each line of source into tokens.
//! Every token gets the scope stack Shiki gives it,
//! so a theme in Shiki's format colours the result as Shiki would.
//!
//! # Quick start
//!
//! ```sh
//! cargo add nishiki
//! ```
//!
//! A [`Highlighter`] lexes one [`Language`], one line at a time.
//! For each line, [`Highlighter::line`] gives:
//!
//! - [`Token`]s, each with the byte offset where it ends and its scope stack;
//! - a [`State`]: where the lexer stands after the line, which the next line
//!   starts from.
//!
//! Because the state passes from line to line:
//!
//! - an editor can re-highlight from any line;
//! - a streamed reply can be highlighted as it arrives.
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
//! [`Highlighter::scopes`] lists a stack innermost first,
//! the order theme rules match it in.
//!
//! # Themes
//!
//! A [`Theme`] is a `TextMate` theme in Shiki's format.
//! [`Theme::style`] gives a scope stack the [`Style`] Shiki would:
//! its colours and font style.
//!
//! | Constructor | Feature | Takes |
//! |:--|:--|:--|
//! | [`Theme::from_json`] | `json` | A theme file, as `@shikijs/themes` ships it. |
//! | [`Theme::new`] | none | The rules as values, in a [`ThemeSource`]. |
//!
//! Without `json` and the `textmate` tier, nishiki depends on `memchr` alone.
//!
//! ```
//! use nishiki::{Highlighter, Language, RuleScope, State, Theme, ThemeRule, ThemeSource};
//!
//! let theme = Theme::new(ThemeSource {
//!     rules: vec![ThemeRule {
//!         scope: RuleScope::Selectors("storage.type".into()),
//!         foreground: Some("#f97583".into()),
//!         ..ThemeRule::default()
//!     }],
//!     ..ThemeSource::default()
//! });
//! let mut highlighter = Highlighter::new(Language::Rust);
//! let mut tokens = Vec::new();
//! let _ = highlighter.line("let x = 42;", &mut State::default(), &mut tokens);
//! let style = highlighter.style(&theme, tokens[0].stack());
//! assert_eq!(style.foreground.to_string(), "#F97583");
//! ```
//!
//! A theme colour that is not hex, such as a CSS variable, sets no colour.
//! Shiki passes it through.
//!
//! # Streaming
//!
//! A [`Document`] highlights text as it arrives, such as a model's reply or a
//! file a tool writes.
//!
//! - [`Document::push`] and [`Document::push_str`] append a delta, which may
//!   end mid-line or inside a UTF-8 character.
//! - [`Document::lex`] lexes what changed, through a line or to the end.
//! - [`Document::take_changes`] gives the lines to draw again.
//! - A completed line is lexed once. Only the last line, still being written,
//!   is lexed again on each delta.
//!
//! A later line never changes an earlier one, so a completed line is final.
//! The last line is [`LineKind::Provisional`] until [`Document::finish`].
//! Its last token can still change, as `in` becomes `int`. Showing the bytes
//! from its `stable` offset in the default colour avoids most flicker.
//!
//! ```
//! use nishiki::{Document, Language, LineKind, Theme, ThemeSource, Until};
//!
//! let mut document = Document::new(Language::Rust, Theme::new(ThemeSource::default()));
//! let mut redrawn = Vec::new();
//! for delta in ["let x", " = 4", "2;\nfn m", "ain() {}"] {
//!     document.push_str(delta);
//!     document.lex(Until::End);
//!     if let Some(changes) = document.take_changes() {
//!         redrawn.extend(changes.start..changes.new_end);
//!     }
//! }
//! assert_eq!(redrawn, [0, 0, 0, 1, 1]);
//! assert_eq!(
//!     document.line(0).map(|line| line.kind()),
//!     Some(LineKind::Exact)
//! );
//! assert!(matches!(
//!     document.line(1).map(|line| line.kind()),
//!     Some(LineKind::Provisional { .. })
//! ));
//!
//! document.finish();
//! document.lex(Until::End);
//! assert_eq!(
//!     document.line(1).map(|line| line.kind()),
//!     Some(LineKind::Exact)
//! );
//! ```
//!
//! # Limits
//!
//! [`Limits`] bounds every line:
//!
//! - its length in bytes;
//! - the lexer steps it may take;
//! - the contexts open at once;
//! - the scope stacks and states a highlighter interns.
//!
//! How they behave:
//!
//! - Each limit can be set, raised, lowered or turned off.
//! - [`Limits::for_budget`] derives them from a memory budget and a per-line
//!   time budget.
//! - Limits count bytes and steps, never time. The same input and limits give
//!   the same tokens on every machine.
//! - Each line's [`LineReport`] names the limits it reached.
//!
//! ```
//! use nishiki::{Highlighter, Language, Limits};
//!
//! let limits = Limits {
//!     depth: None,
//!     ..Limits::default()
//! };
//! let highlighter = Highlighter::new_with_limits(Language::Rust, limits);
//! assert_eq!(highlighter.limits().depth, None);
//! ```
//!
//! nishiki's own `TextMate` engine adds fixed bounds:
//!
//! | Constant | Bounds | Past it |
//! |:--|:--|:--|
//! | `MAX_END` (32,767) | The back-referenced `end` and `while` patterns a highlighter fills in from their `begin` matches. | The rule closes on its grammar's own pattern, and the report names the node limit. |
//! | `MAX_INSTS` (40,000) | The instructions one pattern compiles to. | The pattern never matches. |
//! | `MAX_CALLS` (256) | The subroutine calls nested in one match. | The match fails. |
//!
//! A language's own bounds are listed on its [`Language`] variant.
//!
//! # Cargo features
//!
//! Every language with a lexer of its own is on by default.
//! To build only some, turn off the default features
//! and name what you highlight:
//!
//! ```toml
//! nishiki = { version = "0.1", default-features = false, features = ["lang-rust", "lang-markdown"] }
//! ```
//!
//! - Each language is a `lang-*` feature, named on its [`Language`] variant.
//! - One module lexes TypeScript, JavaScript, JSX and TSX. Each of the four is
//!   its own feature, and a build pays for the copies it turns on.
//! - A language that embeds another turns it on:
//!   - `lang-graphql` turns on `lang-javascript`;
//!   - `lang-xml` turns on `lang-java`;
//!   - `lang-html` turns on `lang-css` and `lang-javascript`;
//!   - `lang-scss` turns on `lang-css`.
//! - [`Language::ALL`] and [`Language::from_name`] know only the languages that
//!   are built.
//! - Nothing is downloaded at run time. Every language you turn on is in the
//!   binary.
//! - `json` reads theme files with [`Theme::from_json`]. It is not the JSON
//!   language, which is `lang-json`.
//!
//! ## Tiers
//!
//! A tier turns on several languages at once.
//! Tiers overlap.
//!
//! | Feature | Languages |
//! |:--|:--|
//! | `all` (the default) | Every language |
//! | `config` | JSON, TOML, YAML, INI, Diff, Dockerfile, Makefile, XML |
//! | `web` | TypeScript, JavaScript, JSX, TSX, CSS, JSON, GraphQL, HTML, ERB, HTML (Derivative), SCSS, Less, PostCSS, Vue, Svelte |
//! | `systems` | Rust, C, C++, Objective-C, Objective-C++, Shell, Java, Kotlin, Go, Swift |
//! | `docs` | Markdown, LaTeX, TeX and `config` |
//!
//! The `textmate` feature builds nishiki's own `TextMate` engine, which runs
//! Shiki's grammars from tables compiled in.
//! No language uses it now.
//! It is off by default and not in `all`,
//! so the default build carries no grammar tables.
//!
//! # Markdown fences
//!
//! [`Language::Markdown`] hands the code in its fences to the other lexers.
//!
//! - A fence is lexed as a supported language when its info string names it the
//!   way Shiki's Markdown grammar does.
//! - Any other fence is plain text, as in Shiki when only these languages are
//!   loaded. That includes `toml` and `kotlin`: Shiki's Markdown grammar has no
//!   rule that embeds them.
//! - A fence of a language whose feature is off is plain text, as in Shiki when
//!   that grammar is not loaded.
//! - YAML front matter is lexed as YAML.
//!
//! # Fidelity
//!
//! The scopes match Shiki's, quirks included.
//! Each [`Language`] variant lists the quirks of its language.

// docs.rs labels feature-gated items, such as `Theme::from_json`, with their
// feature.
#![cfg_attr(docsrs, feature(doc_cfg))]

/// Counts the allocations of the library's own tests, per thread. Some tests
/// check that lexing a line again allocates nothing. Only the library reaches
/// some of that code, such as a grammar that hands off but is not yet a
/// language.
#[cfg(test)]
#[global_allocator]
static ALLOCATOR: nishiki_alloc_count::CountingAllocator = nishiki_alloc_count::CountingAllocator;

mod arena;
#[cfg(feature = "lang-asm")]
mod asm;
#[cfg(feature = "lang-bat")]
mod bat;
#[cfg(any(
    feature = "lang-common-lisp",
    feature = "lang-pascal",
    feature = "lang-tex"
))]
mod bytes;
#[cfg(feature = "lang-c")]
mod c;
#[cfg(any(
    feature = "lang-asm",
    feature = "lang-bat",
    feature = "lang-cmake",
    feature = "lang-common-lisp",
    feature = "lang-css",
    feature = "lang-haskell",
    feature = "lang-latex",
    feature = "lang-less",
    feature = "lang-markdown",
    feature = "lang-mermaid",
    feature = "lang-pascal",
    feature = "lang-php",
    feature = "lang-powershell",
    feature = "lang-sql",
    feature = "lang-vb",
    feature = "_php-family"
))]
mod case_fold;
#[cfg(feature = "_classes")]
mod classes;
#[cfg(feature = "lang-cmake")]
mod cmake;
#[cfg(feature = "lang-coffee")]
mod coffee;
#[cfg(feature = "lang-common-lisp")]
mod common_lisp;
#[cfg(feature = "lang-cpp")]
mod cpp;
#[cfg(feature = "lang-csharp")]
mod csharp;
#[cfg(feature = "lang-css")]
mod css;
#[cfg(feature = "lang-dart")]
mod dart;
#[cfg(feature = "lang-diff")]
mod diff;
#[cfg(feature = "lang-dockerfile")]
mod dockerfile;
mod document;
#[cfg(feature = "lang-elixir")]
mod elixir;
#[cfg(any(
    feature = "lang-hcl",
    feature = "lang-json",
    feature = "lang-toml",
    feature = "lang-yaml"
))]
mod emit;
#[cfg(feature = "lang-erb")]
mod erb;
mod fnv;
#[cfg(feature = "lang-gdscript")]
mod gdscript;
#[cfg(feature = "lang-go")]
mod go;
#[cfg(feature = "lang-graphql")]
mod graphql;
#[cfg(feature = "lang-hack")]
mod hack;
#[cfg(feature = "lang-haml")]
mod haml;
#[cfg(feature = "lang-haskell")]
mod haskell;
#[cfg(feature = "lang-hcl")]
mod hcl;
mod highlighter;
#[cfg(feature = "lang-html")]
mod html;
#[cfg(feature = "lang-html-derivative")]
mod html_derivative;
#[cfg(feature = "lang-ini")]
mod ini;
#[cfg(feature = "lang-java")]
mod java;
#[cfg(feature = "lang-jinja")]
mod jinja;
#[cfg(feature = "lang-json")]
mod json;
#[cfg(feature = "lang-julia")]
mod julia;
#[cfg(feature = "lang-kotlin")]
mod kotlin;
mod language;
#[cfg(feature = "lang-latex")]
mod latex;
#[cfg(feature = "lang-less")]
mod less;
mod limits;
#[cfg(feature = "lang-lua")]
mod lua;
#[cfg(feature = "lang-make")]
mod make;
#[cfg(feature = "lang-markdown")]
mod markdown;
#[cfg(feature = "lang-matlab")]
mod matlab;
#[cfg(feature = "lang-mermaid")]
mod mermaid;
#[cfg(feature = "lang-nix")]
mod nix;
#[cfg(feature = "lang-objective-c")]
mod objective_c;
#[cfg(feature = "lang-objective-cpp")]
mod objective_cpp;
#[cfg(feature = "lang-pascal")]
mod pascal;
#[cfg(feature = "lang-perl")]
mod perl;
#[cfg(feature = "lang-php")]
mod php;
#[cfg(feature = "_php-family")]
mod php_family;
#[cfg(feature = "lang-postcss")]
mod postcss;
#[cfg(feature = "lang-powershell")]
mod powershell;
#[cfg(feature = "lang-proto")]
mod proto;
#[cfg(feature = "lang-python")]
mod python;
#[cfg(feature = "lang-r")]
mod r;
#[cfg(test)]
#[cfg(any(feature = "_rule-engine", feature = "lang-shell"))]
mod resume_check;
#[cfg(feature = "lang-ruby")]
mod ruby;
#[cfg(feature = "_rule-engine")]
mod rules;
#[cfg(feature = "lang-rust")]
mod rust;
#[cfg(feature = "lang-sass")]
mod sass;
#[cfg(feature = "lang-scala")]
mod scala;
mod scope;
mod scratch;
#[cfg(feature = "lang-scss")]
mod scss;
#[cfg(feature = "lang-shell")]
mod shell;
#[cfg(feature = "_space")]
mod space;
#[cfg(feature = "lang-sql")]
mod sql;
#[cfg(feature = "lang-svelte")]
mod svelte;
#[cfg(feature = "lang-swift")]
mod swift;
mod tag;
#[cfg(test)]
mod test_support;
#[cfg(feature = "lang-tex")]
mod tex;
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
#[cfg(feature = "lang-vb")]
mod vb;
#[cfg(feature = "lang-vue")]
mod vue;
#[cfg(feature = "lang-xml")]
mod xml;
#[cfg(feature = "lang-yaml")]
mod yaml;
#[cfg(feature = "lang-zig")]
mod zig;

pub use arena::Scopes;
pub use document::{Changes, Document, DocumentLine, EditError, LineKind, Tail, Until};
pub use highlighter::Highlighter;
pub use language::Language;
pub use limits::{Budget, Limit, Limits, LineReport, Work};
#[cfg(feature = "json")]
pub use theme::ThemeError;
pub use theme::{Color, FontStyle, RuleScope, Style, Theme, ThemeKind, ThemeRule, ThemeSource};

/// The README's example, compiled and run as a doctest.
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
