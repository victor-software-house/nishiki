//! Julia, run by nishiki's `TextMate` engine on the `julia` grammar that Shiki
//! bundles.
//! `oracle/engine-tables.ts` compiles that grammar into `grammar.rs`.
//!
//! The language has no lexer of its own.
//! Its row in `languages!` names the table.
//! The engine runs the grammar's rules as Shiki does and names their scopes at
//! run time.
//! The language therefore has no scope table or frame tag of its own either.
//! Its table leaves out the C++ and R grammars that Julia embeds
//! (`oracle/engine-tables.ts` spec `julia~cpp~r`).
//! The C++ and R inside its string macros therefore stay plain.

pub(crate) mod grammar;
