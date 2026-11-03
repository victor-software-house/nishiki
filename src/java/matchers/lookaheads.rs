//! The lookaheads that end a clause or begin a declaration.

use crate::java::engine::Lexer;
use crate::rules::Chars;

/// What a member variable's lookahead starts with.
const MEMBER_STARTS: &[&[u8]] = &[
    b"private",
    b"protected",
    b"public",
    b"native",
    b"synchronized",
    b"abstract",
    b"threadsafe",
    b"transient",
    b"static",
    b"final",
];

impl Lexer<'_> {
    /// The lookahead `(?=X)` where `X` is one character of `set`.
    pub(super) fn ahead_of(&self, p: usize, set: &[u8]) -> Option<usize> {
        self.one_of(p, set).map(|_| p)
    }

    /// The lookahead `(?=a|b|...)` of literal alternatives.
    pub(super) fn ahead_of_any(&self, p: usize, texts: &[&[u8]]) -> Option<usize> {
        texts.iter().any(|text| self.starts(p, text)).then_some(p)
    }

    /// `(?={|\bword\b)`.
    pub(super) fn before_brace_or(&self, p: usize, word: &[u8]) -> Option<usize> {
        (self.at(p) == b'{' && p < self.lim() || self.keyword(p, word).is_some()).then_some(p)
    }

    /// `(?=\s*extends|permits|\{)` and its kin: `first` after blanks, or
    /// `second`, or a `{`.
    pub(super) fn class_clause_end(&self, p: usize, first: &[u8], second: &[u8]) -> Option<usize> {
        (self.starts(self.space_end(p), first)
            || self.starts(p, second)
            || (self.at(p) == b'{' && p < self.lim()))
        .then_some(p)
    }

    /// `(?=\b(var)\b\s+[$A-Z_a-z][$\w]*\s*([:;=]))`.
    pub(super) fn local_variable_ahead(&self, p: usize) -> Option<usize> {
        let end = self.keyword(p, b"var")?;
        let name = self.space_end(end);
        if name == end {
            return None;
        }
        let name_end = self.ident_end(name)?;
        self.one_of(self.space_end(name_end), b":;=").map(|_| p)
    }

    /// `(?=private|protected|...|final)`: one of the modifiers' text.
    pub(super) fn member_variable_ahead(&self, p: usize) -> Option<usize> {
        self.ahead_of_any(p, MEMBER_STARTS)
    }
}
