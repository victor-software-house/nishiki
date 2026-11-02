//! The character classes of the grammar's expressions.

use crate::classes::{is_digit, is_space, is_word};
use crate::java::engine::Lexer;

impl Lexer<'_> {
    /// The width of the `\w` character at `i`, when there is one.
    pub(in crate::java) fn word_at(&self, i: usize) -> Option<usize> {
        match self.s.get(i) {
            None => None,
            Some(&b) if b < 0x80 => (b.is_ascii_alphanumeric() || b == b'_').then_some(1),
            Some(_) => {
                let (c, end) = self.char_at(i)?;
                is_word(c).then_some(end - i)
            }
        }
    }

    /// Whether the character before `i` is a `\w`.
    pub(in crate::java) fn word_before(&self, i: usize) -> bool {
        match i.checked_sub(1).and_then(|k| self.s.get(k)) {
            Some(&b) if b < 0x80 => b.is_ascii_alphanumeric() || b == b'_',
            Some(_) => self.char_before(i).is_some_and(is_word),
            None => false,
        }
    }

    /// The regex `\b` at `i`.
    pub(in crate::java) fn boundary(&self, i: usize) -> bool {
        self.word_before(i) != self.word_at(i).is_some()
    }

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

    /// The width of the `\s` character at `i`, which includes the line feed
    /// that ends the line.
    pub(in crate::java) fn space_at(&self, i: usize) -> usize {
        if i > self.lim() {
            return 0;
        }
        let b = self.at(i);
        if b < 0x80 {
            return usize::from(matches!(b, b' ' | b'\t' | b'\n' | 0x0b | 0x0c | b'\r'));
        }
        match self.char_at(i) {
            Some((c, end)) if is_space(c) => end - i,
            _ => 0,
        }
    }

    /// The end of the `\s*` from `i`.
    pub(in crate::java) fn space_end(&self, mut i: usize) -> usize {
        loop {
            let width = self.space_at(i);
            if width == 0 {
                return i;
            }
            i += width;
        }
    }

    /// The end of the `[ \t]*` from `i`.
    pub(in crate::java) fn blank_end(&self, i: usize) -> usize {
        self.skip(i, |b| matches!(b, b' ' | b'\t'))
    }

    /// The end of the run of `\d` characters from `i`.
    pub(in crate::java) fn digit_end(&self, mut i: usize) -> usize {
        while let Some((c, end)) = self.char_at(i)
            && i < self.lim()
            && is_digit(c)
        {
            i = end;
        }
        i
    }

    /// Whether the character before `i` is `byte`.
    pub(in crate::java) fn prev_is(&self, i: usize, byte: u8) -> bool {
        i > 0 && self.s.get(i - 1) == Some(&byte)
    }

    /// The end of `text` at `i` when the line has it there.
    pub(in crate::java) fn lit(&self, i: usize, text: &[u8]) -> Option<usize> {
        self.starts(i, text).then(|| i + text.len())
    }

    /// The end of the word `text` at `i`, with `\b` before and after it.
    pub(in crate::java) fn keyword(&self, i: usize, text: &[u8]) -> Option<usize> {
        let end = self.lit(i, text)?;
        (self.boundary(i) && self.word_at(end).is_none()).then_some(end)
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
