//! PHP, run by nishiki's `TextMate` engine on the `php` grammar that Shiki
//! bundles, which `oracle/engine-tables.ts` compiles into `grammar.rs`.
//!
//! The language has no lexer of its own: its row in `languages!` names the
//! table, and the engine runs the grammar's rules as Shiki does.
//! The grammar includes the whole HTML, SQL, JavaScript, JSON, CSS and XML
//! grammars in strings and heredocs.
//! Each such include hands its region to nishiki's lexer for that language.
//! So the engine runs this grammar with its handing loop.
//!
//! The table has a second root, `FENCE`, for a Markdown `php` fence.
//! That fence's rule includes HTML, then PHP.
//! The root hands the line to the HTML lexer, and tries PHP's rules where
//! HTML matches nothing.
//! The language's row names it as its `embedded` root, so any language that
//! embeds PHP starts it there.

pub(crate) mod grammar;
