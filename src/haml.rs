//! HAML, run by nishiki's `TextMate` engine on the `haml` grammar that Shiki
//! bundles, which `oracle/engine-tables.ts` compiles into `grammar.rs`.
//!
//! The language has no lexer of its own: its row in `languages!` names the
//! table, and the engine runs the grammar's rules as Shiki does.
//! The grammar includes the whole Ruby, JavaScript, CSS, Markdown, Sass and
//! CoffeeScript grammars.
//! Each such include hands its region to nishiki's lexer for that language.
//! So the engine runs this grammar with its handing loop.
//! Ruby's `<<~HAML` heredoc embeds HAML.
//! Markdown cannot embed it, because Shiki's Markdown grammar has no HAML
//! fence.

pub(crate) mod grammar;
