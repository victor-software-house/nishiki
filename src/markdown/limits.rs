//! Limits on the recursion of the matchers.
//! The per-line limits (length, work, depth, nodes) are the highlighter's
//! [`Limits`](crate::Limits).

/// Deepest `[` nesting that a link or image label is scanned through.
/// A deeper `[` ends the label there.
/// It bounds the matcher's recursion, not the work.
pub(crate) const MAX_BRACKET_DEPTH: usize = 32;

/// Deepest `(` nesting that a link destination is scanned through.
pub(crate) const MAX_PAREN_DEPTH: usize = 32;
