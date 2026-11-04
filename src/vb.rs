//! Visual Basic .NET, run by nishiki's `TextMate` engine on the `vb` grammar
//! that Shiki bundles.
//! `oracle/engine-tables.ts` compiles that grammar into `grammar.rs`.
//!
//! The language has no lexer of its own.
//! Its row in `languages!` names the table.
//! The engine runs the grammar's rules as Shiki does.
//! Markdown's `vb` fence embeds it.

pub(crate) mod grammar;
