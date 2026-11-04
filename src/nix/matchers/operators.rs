//! The binary operators.

use super::Lexer;
use crate::rules::Chars;

/// The two-byte operators the alternation tries before any one-byte one.
const PAIRS: &[&[u8]] = &[b"|>", b"<|", b"==", b"&&", b"||", b"->", b"//", b"++"];

impl Lexer<'_> {
    /// `(\bor\b|\.|\|>|<\||==|!=?|<=?|>=?|&&|\|\||->|//|\?|\+\+|[-*]|/(?=([^*
    /// ]|$))|\+)`.
    ///
    /// The alternation takes its first branch that matches.
    /// Each two-byte branch comes before every one-byte branch its first byte
    /// starts, so the longer one wins.
    /// A lone `/` is an operator unless a `*` follows it.
    pub(super) fn binary_operator(&self, p: usize) -> Option<usize> {
        if let Some(end) = self.keyword(p, b"or") {
            return Some(end);
        }
        if PAIRS.iter().any(|pair| self.starts(p, pair)) {
            return Some(p + 2);
        }
        match self.at(p) {
            b'!' | b'<' | b'>' => Some(if self.at(p + 1) == b'=' { p + 2 } else { p + 1 }),
            b'.' | b'?' | b'-' | b'*' | b'+' => Some(p + 1),
            b'/' => (self.at(p + 1) != b'*').then_some(p + 1),
            _ => None,
        }
    }
}
