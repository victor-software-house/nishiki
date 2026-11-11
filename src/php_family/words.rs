//! The long word lists of PHP's and Hack's grammars, looked up in the tables
//! each grammar's `words.rs` holds.
//!
//! A list is `\b(…)\b`, `(?i)\b(…)\b` or `(\\)?\b(…)\b`.
//! Its words are runs of `\w`, so a match is the whole run of `\w` at its
//! start.
//! A few class names join two runs with `\` or `-`, as `XMLDiff\DOM` does.
//! No word of a table is the first run of such a name, so the run alone is
//! looked up first.

use std::cmp::Ordering;

use super::PhpFamily;
use crate::case_fold;
use crate::rules::{Chars, Grammar, Lexer};

/// The longest word of any table, in bytes, with room for a fold that
/// lengthens it.
const LONGEST: usize = 96;

/// How a table compares a word.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Case {
    /// `(?i)`: the words are lowercase, and the text is folded to compare.
    Folded,
    /// The text compares as written.
    Exact,
}

/// Whether a match takes a backslash before the word.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Backslash {
    /// `(\\)?\b(…)\b`: a backslash right before the word joins the match.
    Optional,
    /// `\b(…)\b`: the match is the word alone.
    Never,
}

/// A table of words: their text, each word's end in it, and the list that
/// holds the word first.
/// A word is a run of word bytes, or two runs joined by `\` or `-`.
pub(crate) struct Words {
    /// The words, one after another, sorted.
    pub(crate) text: &'static str,
    /// Where each word ends in `text`.
    pub(crate) ends: &'static [u16],
    /// The list that holds each word first.
    pub(crate) lists: &'static [u8],
    /// How the table compares a word.
    pub(crate) case: Case,
    /// Whether a match takes a backslash before the word.
    pub(crate) backslash: Backslash,
}

impl Words {
    /// The first list that holds `word` as the table spells it: lowercase
    /// for a folded table.
    fn list(&self, word: &[u8]) -> Option<u8> {
        let (mut low, mut high) = (0, self.ends.len());
        while low < high {
            let mid = low + (high - low) / 2;
            match self.word(mid).cmp(word) {
                Ordering::Less => low = mid + 1,
                Ordering::Greater => high = mid,
                Ordering::Equal => return self.lists.get(mid).copied(),
            }
        }
        None
    }

    /// Word `at` of the table.
    fn word(&self, at: usize) -> &'static [u8] {
        let start = at
            .checked_sub(1)
            .and_then(|before| self.ends.get(before))
            .map_or(0, |&end| usize::from(end));
        let end = self.ends.get(at).map_or(start, |&end| usize::from(end));
        self.text.as_bytes().get(start..end).unwrap_or(&[])
    }
}

/// The lookups of a word table, on the rule engine's lexer of either grammar.
#[cfg_attr(
    nishiki_unstable,
    expect(private_bounds, reason = "the bound and the methods are crate-private")
)]
impl<G: PhpFamily> Lexer<'_, G> {
    /// The list of `words` that holds the word at `p` first, and the end of
    /// its match.
    /// A table whose lists take a backslash reads one at `p` before the word.
    pub(crate) fn table_word(&self, words: &Words, p: usize) -> Option<(u8, usize)> {
        let start = match words.backslash {
            Backslash::Optional if self.at(p) == b'\\' => p + 1,
            Backslash::Optional | Backslash::Never => p,
        };
        if !self.boundary(start) || self.word_at(start).is_none() {
            return None;
        }
        let first = self.word_end(start);
        if let Some(list) = lookup(self, words, start, first) {
            return Some((list, first));
        }
        let joined = match self.at(first) {
            b'\\' | b'-' if self.word_at(first + 1).is_some() => self.word_end(first + 1),
            _ => return None,
        };
        lookup(self, words, start, joined).map(|list| (list, joined))
    }

    /// The list of `words` whose match starts at `p`, after the scan found
    /// the table's head there.
    pub(crate) fn word_list(&self, words: &Words, p: usize) -> Option<u8> {
        self.table_word(words, p).map(|(list, _)| list)
    }

    /// The matcher of list `list` of `words`.
    /// The head, list 0, stands for the whole table, so any list's word
    /// matches it.
    /// Another list matches only its own words.
    /// The backslash before the word is capture group 1.
    pub(crate) fn table_list(&mut self, words: &Words, list: u8, p: usize) -> Option<usize> {
        let (found, end) = self.table_word(words, p)?;
        if list != 0 && found != list {
            return None;
        }
        if self.at(p) == b'\\' {
            self.capture(1, p, p + 1);
        }
        Some(end)
    }
}

/// The list of `words` that holds the text `start..end` first.
fn lookup<G: Grammar>(lexer: &Lexer<'_, G>, words: &Words, start: usize, end: usize) -> Option<u8> {
    let text = lexer.text(start, end);
    match words.case {
        Case::Exact => words.list(text),
        Case::Folded => {
            let mut buf = [0; LONGEST];
            let len = case_fold::lower_into(text, &mut buf)?;
            words.list(buf.get(..len)?)
        }
    }
}
