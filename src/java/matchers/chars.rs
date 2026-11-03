//! The character classes of the grammar's expressions.

use crate::java::engine::Lexer;
use crate::rules::Chars;

impl Lexer<'_> {
    /// The end of the run of `\w` characters from `i`. Patterns ask about
    /// the same word one after another, so the last run is kept.
    pub(in crate::java) fn word_end(&self, i: usize) -> usize {
        let (start, end) = self.x.word_run.get();
        if start <= i && i < end {
            return end;
        }
        let mut e = i;
        while let Some(width) = self.word_at(e) {
            e += width;
        }
        self.spend(e - i);
        if e > i {
            self.x.word_run.set((i, e));
        }
        e
    }

    /// The end of the run of `[$\w]` characters from `i`, kept the same way.
    pub(in crate::java) fn name_end(&self, i: usize) -> usize {
        let (start, end) = self.x.name_run.get();
        if start <= i && i < end {
            return end;
        }
        let mut e = i;
        loop {
            if self.s.get(e) == Some(&b'$') {
                e += 1;
            } else if let Some(width) = self.word_at(e) {
                e += width;
            } else {
                break;
            }
        }
        self.spend(e - i);
        if e > i {
            self.x.name_run.set((i, e));
        }
        e
    }

    /// The end of the identifier `[$A-Z_a-z][$\w]*` at `i`, when one starts.
    pub(in crate::java) fn ident_end(&self, i: usize) -> Option<usize> {
        let b = *self.s.get(i)?;
        (b.is_ascii_alphabetic() || b == b'_' || b == b'$').then(|| self.name_end(i + 1))
    }

    /// The end of the `[ \t]*` from `i`.
    pub(in crate::java) fn blank_end(&self, i: usize) -> usize {
        self.skip(i, |b| matches!(b, b' ' | b'\t'))
    }

    /// The end of the first of `words` at `i`, with `\b` before and after it.
    pub(in crate::java) fn keyword_of(&self, i: usize, words: &[&[u8]]) -> Option<usize> {
        if !self.boundary(i) {
            return None;
        }
        words
            .iter()
            .find_map(|word| self.lit(i, word).filter(|&end| self.word_at(end).is_none()))
    }
}
