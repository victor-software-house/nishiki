//! The word alternations, `(?i:\b(a|b|…)\b)`, read from the generated
//! buckets.

use super::super::engine::Lexer;
use super::super::grammar::{Buckets, CONSTANTS, STORAGE};
use crate::case_fold;
use crate::rules::Chars;

impl Lexer<'_> {
    /// The end of `(?i:\b(a|b|…)\b)` at `p`, for the words of `table`.
    ///
    /// The alternation takes its first word in order that `\b` follows.
    /// A word can match only in the bucket of the letter the text starts
    /// with, and the bucket keeps the grammar's order.
    /// A character above ASCII starts the bucket of the first letter it folds
    /// to, as `ﬁ` starts `f`.
    pub(super) fn ordered(&self, p: usize, table: &Buckets) -> Option<usize> {
        if p >= self.lim() || self.word_before(p) {
            return None;
        }
        let b = self.at(p);
        let first = if b.is_ascii() {
            b.to_ascii_lowercase()
        } else {
            let (c, _) = self.char_at(p)?;
            *case_fold::letters(c)?.first()?
        };
        let bucket = table.get(usize::from(first.checked_sub(b'a')?))?;
        let text = self.text(0, self.lim());
        bucket.iter().find_map(|word| {
            case_fold::literal_end(text, p, word).filter(|&end| self.word_at(end).is_none())
        })
    }

    /// `(?i:\s*\b(words)\b\s*)`: a storage word with the blanks around it.
    /// The blanks before give nothing back, since a word starts with a
    /// letter.
    /// The blanks after can take the line feed.
    pub(super) fn storage(&self, p: usize) -> Option<usize> {
        let word = self.space_end(p);
        let end = self.ordered(word, &STORAGE)?;
        Some(self.space_end(end))
    }

    /// `(?i:\s*\b(words)\b)`: a constant with the blanks before it.
    pub(super) fn constant(&self, p: usize) -> Option<usize> {
        self.ordered(self.space_end(p), &CONSTANTS)
    }
}
