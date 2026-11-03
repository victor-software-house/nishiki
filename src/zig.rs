//! Zig, run by nishiki's `TextMate` engine on the `zig` grammar that Shiki
//! bundles, which `oracle/engine-tables.ts` compiles into `grammar.rs`.
//!
//! The language has no lexer of its own: its row in `languages!` names the
//! table, and the engine runs the grammar's rules as Shiki does and names
//! their scopes at run time, so the language has no scope table or frame tag
//! of its own either. Shiki's Markdown grammar has no Zig fence, so no
//! language embeds Zig yet.

pub(crate) mod grammar;
