//! Keywords and the words the grammar lists.

use super::super::engine::Lexer;
use crate::rules::Chars;

/// `\b(abstract|final|…|value)\b`: the grammar's `prefix-modifiers`.
pub(super) const PREFIX_MODIFIERS: &[&[u8]] = &[
    b"abstract",
    b"final",
    b"enum",
    b"open",
    b"annotation",
    b"sealed",
    b"data",
    b"override",
    b"lateinit",
    b"private",
    b"protected",
    b"public",
    b"internal",
    b"inner",
    b"companion",
    b"noinline",
    b"crossinline",
    b"vararg",
    b"reified",
    b"tailrec",
    b"operator",
    b"infix",
    b"inline",
    b"external",
    b"const",
    b"suspend",
    b"value",
];

/// `\b(where|by|get|set)\b`.
pub(super) const POSTFIX_MODIFIERS: &[&[u8]] = &[b"where", b"by", b"get", b"set"];

/// `\b(init|catch|finally|field)\b`.
pub(super) const SOFT_KEYWORDS: &[&[u8]] = &[b"init", b"catch", b"finally", b"field"];

/// `\b(as|typeof|is|in)\b`.
pub(super) const HARD_KEYWORDS: &[&[u8]] = &[b"as", b"typeof", b"is", b"in"];

/// `\b(if|else|while|do|when|try|throw|break|continue|return|for)\b`.
pub(super) const CONTROL_KEYWORDS: &[&[u8]] = &[
    b"if",
    b"else",
    b"while",
    b"do",
    b"when",
    b"try",
    b"throw",
    b"break",
    b"continue",
    b"return",
    b"for",
];

impl Lexer<'_> {
    /// `\b(this|super)(@\w+)?\b`: when the label is there, its word ends the
    /// match at a boundary; without it, the keyword's own end must be one.
    pub(super) fn self_reference(&self, p: usize) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        let end = self.lit(p, b"this").or_else(|| self.lit(p, b"super"))?;
        if self.at(end) == b'@' {
            let label = self.word_end(end + 1);
            if label > end + 1 {
                return Some(label);
            }
        }
        self.boundary(end).then_some(end)
    }

    /// `\b(import)\b\s*` and `\b(package)\b\s*`: the keyword as group 1, and
    /// the blanks after it, which can include the line feed.
    pub(super) fn directive(&mut self, p: usize, word: &[u8]) -> Option<usize> {
        let end = self.keyword(p, word)?;
        self.cap(1, p, end);
        Some(self.space_end(end))
    }

    /// `;|$`.
    pub(super) fn semicolon_or_end(&self, p: usize) -> Option<usize> {
        if self.at(p) == b';' {
            return Some(p + 1);
        }
        self.at_eol(p).then_some(p)
    }
}
