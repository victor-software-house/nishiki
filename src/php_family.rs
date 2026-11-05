//! What PHP's and Hack's lexers share.
//!
//! Hack's grammar descends from PHP's, so many of its expressions are PHP's,
//! word for word.
//! The matcher of each such expression lives here once.
//! So do the lookup of the long word lists that both grammars hold, and the
//! places where SQL in a string hands the line back.

pub(crate) mod halts;
pub(crate) mod matchers;
pub(crate) mod words;

use crate::rules::Grammar;

/// A grammar of the PHP family, PHP's or Hack's.
/// Its lexer gets the shared matchers and word lookups as inherent methods.
///
/// They are not a generic trait impl.
/// rustc counts every item of a trait impl as reachable, and walks the bodies
/// of generic ones.
/// It then exports what they call, such as `case_fold::literal_at`.
/// Every caller in the crate reaches an exported function through the GOT,
/// which cost SQL 0.4% of its instructions at one codegen unit.
pub(crate) trait PhpFamily: Grammar {}
