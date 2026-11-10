//! The regex parser the engine compiles with, for a caller that reads a
//! grammar's patterns.
//!
//! [`parse`] reads a pattern in Oniguruma's syntax, as `TextMate` grammars
//! write it.
//! It returns a [`ParsedPattern`] whose root is a [`Node`].
//!
//! A code generator walks that tree.
//! It can list the words a long alternation spells out, or decide whether a
//! pattern can match the empty text.
//! The engine compiles the same tree, so a caller reads a pattern as the
//! engine runs it.
//!
//! ```
//! use nishiki::unstable::regex::{Dialect, Node, parse};
//!
//! let pattern = parse("if|else", Dialect::Grammar).expect("the pattern parses");
//! let Node::Alternation(alternatives) = pattern.root else {
//!     panic!("two words make an alternation");
//! };
//! assert_eq!(alternatives.len(), 2);
//! ```

pub use crate::textmate::regex::ast::{
    Anchor, Case, Class, Greediness, Member, Node, Polarity, Property,
};
pub use crate::textmate::regex::parse::{Dialect, Error, ParsedPattern, parse};
