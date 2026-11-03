//! Julia, run by nishiki's `TextMate` engine on the `julia` grammar that
//! Shiki bundles, which `oracle/engine-tables.ts` compiles into `grammar.rs`.
//!
//! The language has no lexer of its own: its row in `languages!` names the
//! table, and the engine runs the grammar's rules as Shiki does and names
//! their scopes at run time, so the language has no scope table or frame tag
//! of its own either. Its table leaves out the C++ and R grammars that Julia
//! embeds (`oracle/engine-tables.ts` spec `julia~cpp~r`), so the C++ and R
//! inside its string macros stay plain.

pub(crate) mod grammar;
