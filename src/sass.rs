//! Sass, the indented syntax, run by nishiki's `TextMate` engine on the `sass`
//! grammar that Shiki bundles, which `oracle/engine-tables.ts` compiles into
//! `grammar.rs`.
//!
//! The language has no lexer of its own: its row in `languages!` names the
//! table, and the engine runs the grammar's rules as Shiki does.
//! Shiki's Markdown grammar has no Sass fence.
//! HAML's `:sass` filter hands its lines to Sass.

pub(crate) mod grammar;
