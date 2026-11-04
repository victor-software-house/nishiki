//! Objective-C, run by nishiki's `TextMate` engine on the `objective-c`
//! grammar that Shiki bundles, which `oracle/engine-tables.ts` compiles into
//! `grammar.rs`.
//!
//! The language has no lexer of its own: its row in `languages!` names the
//! table, and the engine runs the grammar's rules as Shiki does.
//! The grammar includes no other grammar.
//! A Markdown fence named `objc`, `objective-c` or `m` embeds Objective-C.

pub(crate) mod grammar;
