//! Elixir, run by nishiki's `TextMate` engine on the `elixir` grammar that
//! Shiki bundles, which `oracle/engine-tables.ts` compiles into `grammar.rs`.
//!
//! The language has no lexer of its own: its row in `languages!` names the
//! table, and the engine runs the grammar's rules as Shiki does.
//! The grammar includes the whole HTML grammar in its `~H` and `~L` sigils.
//! Each such include hands its region to nishiki's HTML lexer.
//! So the engine runs this grammar with its handing loop.

pub(crate) mod grammar;
