//! Objective-C++, run by nishiki's `TextMate` engine on the `objective-cpp`
//! grammar that Shiki bundles.
//! `oracle/engine-tables.ts` compiles that grammar into `grammar.rs`.
//!
//! The language has no lexer of its own.
//! Its row in `languages!` names the table.
//! The engine runs the grammar's rules as Shiki does.
//! Shiki's Markdown grammar has no Objective-C++ fence.

pub(crate) mod grammar;
