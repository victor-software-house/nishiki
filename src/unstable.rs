//! The rule engine's API, for a caller that runs a grammar of their own.
//!
//! This module is exempt from semver.
//! Any release can change it, a patch included.
//!
//! It compiles only with `--cfg nishiki_unstable`, which a caller sets in
//! `RUSTFLAGS` or in `.cargo/config.toml`:
//!
//! ```toml
//! [build]
//! rustflags = ["--cfg", "nishiki_unstable"]
//! ```
//!
//! It also needs a language on the rule engine, such as Python or Java.
//! The default features turn several on.
//!
//! A cfg is the opt-in. No Cargo feature turns the API on.
//! Features unify across a dependency graph, so a crate that depends on
//! nishiki could turn the API on for every other crate that does.
//!
//! [`Grammar`] is the trait each rule-engine language implements: its rules,
//! patterns and scopes, and the matchers that decide where a pattern
//! matches.
//! The other items are the types its signatures name.
//! [`Scopes`] here is a static scope list.
//! It differs from [`crate::Scopes`], which walks a token's scope stack.
//!
//! [`grammar`] holds a grammar's rules as its JSON reads.
//! [`regex`] holds the parser the engine compiles patterns with.
//! [`is_space`] and [`is_word`] are the classes `\s` and `\w` as Shiki reads
//! them.
//!
//! A caller outside the crate names these items:
//!
//! ```
//! use nishiki::unstable::{CarriedState, Grammar};
//!
//! /// The state a grammar's root rule carries from line to line.
//! fn root_state<G: Grammar>() -> CarriedState {
//!     G::rule_carried_state(G::ROOT)
//! }
//!
//! assert!(size_of::<CarriedState>() <= 2);
//! ```

// The items here carry the crate's internal features.
// Those mean nothing to a reader of the docs.
#![cfg_attr(
    docsrs,
    doc(auto_cfg(hide(
        feature,
        values("_embedding", "_language", "_rule-engine", "_rule-tables", "_textmate")
    )))
)]

pub use crate::arena::Arena;
pub use crate::classes::{is_space, is_word};
#[cfg(feature = "_embedding")]
pub use crate::rules::Deferrals;
pub use crate::rules::{
    Act, BlankRuns, Captures, CarriedState, Grammar, Lexer, NameCase, RuleSyntax, ScopeName,
};
pub use crate::scope::Scope;
#[cfg(feature = "_language")]
pub use crate::scope::{Keyed, Scopes};

#[cfg(feature = "textmate")]
pub mod grammar;
#[cfg(feature = "_textmate")]
pub mod regex;
