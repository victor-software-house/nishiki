//! Characters, runs and the zero-width assertions the expressions use.

use super::super::engine::Lexer;
use super::super::limits::MAX_LOOPS;
pub(crate) use crate::classes::{is_digit, is_space, is_word};

/// Marks a matcher as one level deeper into such a loop until it is dropped.
pub(crate) struct LoopGuard<'a>(&'a std::cell::Cell<u32>);

impl Drop for LoopGuard<'_> {
    fn drop(&mut self) {
        self.0.set(self.0.get() - 1);
    }
}

impl Lexer<'_> {
    /// One level deeper into a loop written as recursion, or `None` where the
    /// loop may not go on.
    pub(crate) fn deeper(&self) -> Option<LoopGuard<'_>> {
        let depth = self.x.loops.get();
        if depth >= MAX_LOOPS {
            return None;
        }
        self.x.loops.set(depth + 1);
        Some(LoopGuard(&self.x.loops))
    }

    /// The character at `i` and its end. The line feed that follows the text
    /// counts as a character, as it does to the expressions.
    pub(crate) fn ch(&self, i: usize) -> Option<(char, usize)> {
        if i == self.s.len() {
            return self.nl.then_some(('\n', i + 1));
        }
        let b = *self.s.get(i)?;
        if b < 0x80 {
            return Some((char::from(b), i + 1));
        }
        let end = self.char_end(i);
        let text = std::str::from_utf8(self.s.get(i..end)?).ok()?;
        Some((text.chars().next()?, end))
    }

    /// The character that ends at `i`; the line feed after the text counts.
    pub(crate) fn ch_before(&self, i: usize) -> Option<char> {
        if self.nl && i == self.s.len() + 1 {
            return Some('\n');
        }
        self.char_before(i.min(self.s.len()))
    }

    /// `text` at `p`: where it ends.
    pub(crate) fn lit(&self, p: usize, text: &[u8]) -> Option<usize> {
        self.starts(p, text).then(|| p + text.len())
    }

    /// The first of `texts` at `p`, in order: where it ends.
    pub(crate) fn lit_any(&self, p: usize, texts: &[&[u8]]) -> Option<usize> {
        texts.iter().find_map(|text| self.lit(p, text))
    }

    /// The end of the character at `p` when `accept` takes it.
    pub(crate) fn one(&self, p: usize, accept: impl Fn(char) -> bool) -> Option<usize> {
        let (c, end) = self.ch(p)?;
        accept(c).then_some(end)
    }

    /// The end of the run of characters from `p` that `accept` takes.
    pub(crate) fn span_of(&self, mut p: usize, accept: impl Fn(char) -> bool) -> usize {
        let from = p;
        while let Some((c, end)) = self.ch(p) {
            if !accept(c) {
                break;
            }
            p = end;
        }
        self.spend(p - from + 1);
        p
    }

    /// The width of the `\s` character at `p`, or 0.
    pub(crate) fn space_at(&self, p: usize) -> usize {
        match self.ch(p) {
            Some((c, end)) if is_space(c) => end - p,
            _ => 0,
        }
    }

    /// The end of `\s*` from `p`.
    pub(crate) fn skip_space(&self, p: usize) -> usize {
        let (start, end, len, nl) = self.x.blank_run.get();
        if start <= p && p < end && len == self.s.len() && nl == self.nl {
            self.spend(1);
            return end;
        }
        let end = self.span_of(p, is_space);
        if end > p {
            self.x.blank_run.set((p, end, self.s.len(), self.nl));
        }
        end
    }

    /// The width of the `\w` character at `p`, or 0.
    pub(crate) fn word_at(&self, p: usize) -> usize {
        match self.ch(p) {
            Some((c, end)) if is_word(c) => end - p,
            _ => 0,
        }
    }

    /// Whether a `\w` character ends at `p`.
    pub(crate) fn word_before(&self, p: usize) -> bool {
        self.ch_before(p).is_some_and(is_word)
    }

    /// The regex `\b` at `p`.
    pub(crate) fn boundary(&self, p: usize) -> bool {
        self.word_before(p) != (self.word_at(p) > 0)
    }

    /// The end of `\w*` from `p`.
    pub(crate) fn skip_word(&self, p: usize) -> usize {
        self.span_of(p, is_word)
    }

    /// Whether the text before `p` ends with `text`.
    pub(crate) fn ends_with_at(&self, p: usize, text: &[u8]) -> bool {
        p >= text.len() && self.text(p - text.len(), p) == text
    }
}
