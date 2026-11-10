//! A grammar's rules as its JSON holds them, for a caller that reads a
//! grammar rather than runs it.
//!
//! [`Grammar::from_json`] reads a grammar with the reader the engine uses.
//! [`unlinked`] then hands back that reading: every rule object once, with
//! its `match`, `begin`, `name`, captures and patterns.
//! Its includes stay names, so it depends on no other grammar.
//! The reader keeps the text's key order, which a grammar's `injections`
//! depend on.
//!
//! ```
//! use nishiki::Grammar;
//! use nishiki::unstable::grammar::unlinked;
//!
//! let json = r#"{"scopeName": "source.demo", "patterns": [{"match": "if|else"}]}"#;
//! let grammar = Grammar::from_json(json).expect("the grammar reads");
//! let rules = unlinked(grammar);
//! let first = &rules.rules[rules.patterns[0]];
//! assert_eq!(first.matched.as_deref(), Some("if|else"));
//! ```

use crate::Grammar;
pub use crate::textmate::compile::{
    List, Repository, RuleId, Text, UnlinkedCapture, UnlinkedGrammar, UnlinkedRule,
};

/// The rules `grammar` was read from, as its JSON holds them.
#[must_use]
pub fn unlinked(grammar: Grammar) -> &'static UnlinkedGrammar {
    grammar.unlinked()
}
