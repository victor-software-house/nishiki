//! The long word lists, looked up in the tables of [`super::super::words`].
//!
//! A list is `\b(…)\b`, `(?i)\b(…)\b` or `(\\)?\b(…)\b`.
//! Its words are runs of `\w`, so a match is the whole run of `\w` at its
//! start.
//! A few class names join two runs with `\` or `-`, as `XMLDiff\DOM` does.
//! No word of a table is the first run of such a name, so the run alone is
//! looked up first.

use std::cmp::Ordering;

use super::super::engine::Lexer;
use super::super::words::{Backslash, Case, Table, Words};
use crate::case_fold;
use crate::rules::Chars;

/// The longest word of any table, in bytes, with room for a fold that
/// lengthens it.
const LONGEST: usize = 96;

impl Words {
    /// The first list that holds `word`, which the table spells: lowercase
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

impl Lexer<'_> {
    /// The list of `table` that holds the word at `p` first, and the end of
    /// its match.
    /// A table whose lists take a backslash reads one at `p` before the word.
    pub(in crate::php) fn table_word(&self, table: Table, p: usize) -> Option<(u8, usize)> {
        let words = table.words();
        let start = match words.backslash {
            Backslash::Optional if self.at(p) == b'\\' => p + 1,
            Backslash::Optional | Backslash::Never => p,
        };
        if !self.boundary(start) || self.word_at(start).is_none() {
            return None;
        }
        let first = self.word_end(start);
        if let Some(list) = self.lookup(words, start, first) {
            return Some((list, first));
        }
        let joined = match self.at(first) {
            b'\\' | b'-' if self.word_at(first + 1).is_some() => self.word_end(first + 1),
            _ => return None,
        };
        self.lookup(words, start, joined).map(|list| (list, joined))
    }

    /// The list of `words` that holds the text `start..end` first.
    fn lookup(&self, words: &Words, start: usize, end: usize) -> Option<u8> {
        let text = self.text(start, end);
        match words.case {
            Case::Exact => words.list(text),
            Case::Folded => {
                let mut buf = [0; LONGEST];
                let len = case_fold::lower_into(text, &mut buf)?;
                words.list(buf.get(..len)?)
            }
        }
    }

    /// The list of `table` whose match starts at `p`, after the scan found the
    /// table's head there.
    pub(in crate::php) fn word_list(&self, table: Table, p: usize) -> Option<u8> {
        self.table_word(table, p).map(|(list, _)| list)
    }

    /// The matcher of list `list` of `table`.
    /// The head, list 0, stands for the whole table, so any list's word
    /// matches it.
    /// Another list matches only its own words.
    /// The backslash before the word is capture group 1.
    pub(super) fn table_list(&mut self, table: Table, list: u8, p: usize) -> Option<usize> {
        let (found, end) = self.table_word(table, p)?;
        if list != 0 && found != list {
            return None;
        }
        if self.at(p) == b'\\' {
            self.cap(1, p, p + 1);
        }
        Some(end)
    }
}
