//! Perl, run by nishiki's `TextMate` engine on the `perl` grammar that Shiki
//! bundles, which `oracle/engine-tables.ts` compiles into `grammar.rs`.
//!
//! The language has no lexer of its own: its row in `languages!` names the
//! table, and the engine runs the grammar's rules as Shiki does.
//! The grammar includes the whole HTML, XML, CSS, JavaScript and SQL grammars
//! in heredocs.
//! Each such include hands its region to nishiki's lexer for that language.
//! So the engine runs this grammar with its handing loop.

pub(crate) mod grammar;
