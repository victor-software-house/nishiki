//! The character classes and runs the grammar's expressions are built from.

use super::super::engine::Lexer;
use crate::rules::Chars;

impl Lexer<'_> {
    /// The end of the run of `[-\w]` from `p`, where `\w` is Oniguruma's;
    /// CSS's cache answers a search from inside the last run.
    pub(super) fn dash_word_end(&self, p: usize) -> usize {
        self.wordish_run(p)
    }

    /// `[-\w]+` at `p`.
    pub(super) fn dash_word_run(&self, p: usize) -> Option<usize> {
        let end = self.dash_word_end(p);
        (end > p).then_some(end)
    }

    /// `\w+` at `p`.
    pub(super) fn word_chars(&self, p: usize) -> Option<usize> {
        let end = self.word_end(p);
        (end > p).then_some(end)
    }

    /// `\s+` at `p`.
    pub(super) fn spaces_run(&self, p: usize) -> Option<usize> {
        let end = self.spaces(p);
        (end > p).then_some(end)
    }

    /// The last `k` in `from + 1..=to` where `\b` holds: where a run of
    /// `from..to` that a `\b` follows ends, the run giving characters back
    /// until one does.
    pub(super) fn boundary_back(&self, from: usize, to: usize) -> Option<usize> {
        let mut k = to;
        while k > from {
            if self.boundary(k) {
                self.spend(to - k);
                return Some(k);
            }
            k = self.char_start_before(k);
        }
        None
    }

    /// The lookbehind `(?<=text)` at `p`.
    pub(super) fn after(&self, p: usize, text: &[u8]) -> bool {
        p >= text.len() && self.s.get(p - text.len()..p) == Some(text)
    }

    /// `\s*(?=[set])`: the white space from `p` when one of the bytes of
    /// `set` follows it. The white space cannot give back a character that
    /// would be one of them.
    pub(super) fn spaces_before(&self, p: usize, set: &[u8]) -> Option<usize> {
        let end = self.spaces(p);
        set.contains(&self.at(end)).then_some(end)
    }

    /// `\s*((?=[set]|$))`: [`Lexer::spaces_before`], or the end of the line.
    pub(super) fn spaces_before_or_eol(&self, p: usize, set: &[u8]) -> Option<usize> {
        let end = self.spaces(p);
        (set.contains(&self.at(end)) || self.at_eol(end)).then_some(end)
    }
}
