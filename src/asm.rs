//! x86 and x86-64 assembly, run by nishiki's `TextMate` engine on the `asm`
//! grammar that Shiki bundles.
//! `oracle/engine-tables.ts` compiles that grammar into `grammar.rs`.
//!
//! The language has no lexer of its own.
//! Its row in `languages!` names the table.
//! The engine runs the grammar's rules as Shiki does.
//! Shiki's Markdown grammar has no assembly fence.

pub(crate) mod grammar;
