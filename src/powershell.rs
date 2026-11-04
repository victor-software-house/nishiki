//! PowerShell, run by nishiki's `TextMate` engine on the `powershell` grammar
//! that Shiki bundles.
//! `oracle/engine-tables.ts` compiles that grammar into `grammar.rs`.
//!
//! The language has no lexer of its own.
//! Its row in `languages!` names the table.
//! The engine runs the grammar's rules as Shiki does and names their scopes
//! at run time.
//! The language therefore has no scope table or frame tag of its own either.

pub(crate) mod grammar;
