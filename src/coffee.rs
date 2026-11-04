//! CoffeeScript, run by nishiki's `TextMate` engine on the `coffee` grammar
//! that Shiki bundles, which `oracle/engine-tables.ts` compiles into
//! `grammar.rs`.
//!
//! The language has no lexer of its own: its row in `languages!` names the
//! table, and the engine runs the grammar's rules as Shiki does.
//! The grammar includes the whole JavaScript grammar for embedded JavaScript.
//! That include hands its region to nishiki's JavaScript lexer.
//! So the engine runs this grammar with its handing loop.
//! A Markdown fence named `coffee` embeds CoffeeScript.
//! HAML's `:coffee` filter hands its lines to CoffeeScript.

pub(crate) mod grammar;
