//! The runs of characters the grammar's expressions are built from, which
//! its lookaheads and lookbehinds look at, on the shared character classes of
//! [`Chars`].

use super::super::engine::Lexer;
use crate::rules::Chars;

impl Lexer<'_> {
    /// Forgets the remembered runs when the end of the text has moved since
    /// they were found.
    pub(in crate::go) fn sync_runs(&self) {
        let end = self.lim() * 2 + usize::from(self.nl);
        if self.x.runs_end.get() != end {
            self.x.runs_end.set(end);
            self.x.word_run.set((usize::MAX, 0));
            self.x.space_run.set((usize::MAX, 0));
            for run in &self.x.runs {
                run.set((usize::MAX, 0));
            }
        }
    }

    /// The end of the white space from `p`: `\s*`.
    pub(in crate::go) fn spaces(&self, p: usize) -> usize {
        self.sync_runs();
        let (from, to) = self.x.space_run.get();
        if from <= p && p < to {
            self.spend(1);
            return to;
        }
        let mut i = p;
        loop {
            let width = self.space_at(i);
            if width == 0 {
                break;
            }
            i += width;
        }
        self.spend(i - p + 1);
        self.x.space_run.set((p, i));
        i
    }

    /// The end of the run of `\w` characters from `p`.
    pub(in crate::go) fn word_run(&self, p: usize) -> usize {
        self.sync_runs();
        let (from, to) = self.x.word_run.get();
        if from <= p && p < to {
            self.spend(1);
            return to;
        }
        let mut i = p;
        while let Some(width) = self.word_at(i) {
            i += width;
        }
        self.spend(i - p + 1);
        self.x.word_run.set((p, i));
        i
    }

    /// Whether the character at `p` is a hex digit, `\h`.
    pub(in crate::go) fn hex_at(&self, p: usize) -> bool {
        p < self.lim() && self.at(p).is_ascii_hexdigit()
    }

    /// The end of the run from `p` of `\w` characters and the ASCII bytes
    /// `extra` accepts: a class such as `[*.\w]`.
    pub(in crate::go) fn words_and(&self, p: usize, extra: impl Fn(u8) -> bool) -> usize {
        let mut i = p;
        while i < self.lim() {
            let b = self.at(i);
            if b < 0x80 {
                if !(extra(b) || b.is_ascii_alphanumeric() || b == b'_') {
                    break;
                }
                i += 1;
            } else if self.word_at(i).is_some() {
                i = self.char_end(i);
            } else {
                break;
            }
        }
        self.spend(i - p + 1);
        i
    }

    /// The end of `[.\w]*` from `p`.
    pub(in crate::go) fn dot_words(&self, p: usize) -> usize {
        self.words_and(p, |b| b == b'.')
    }

    /// The end of `[*.\w]*` from `p`.
    pub(in crate::go) fn star_dot_words(&self, p: usize) -> usize {
        self.words_and(p, |b| matches!(b, b'*' | b'.'))
    }

    /// The end of `[]*.\[\w]*` from `p`: a type with its slice and pointer
    /// marks.
    pub(in crate::go) fn type_run(&self, p: usize) -> usize {
        self.words_and(p, |b| matches!(b, b']' | b'*' | b'.' | b'['))
    }

    /// The end of `[]*\[]+{0,1}` from `p`: slice and pointer marks.
    pub(in crate::go) fn marks(&self, p: usize) -> usize {
        self.skip(p, |b| matches!(b, b']' | b'*' | b'['))
    }

    /// Whether a character stands at `p`: one of the text, or its line feed.
    pub(in crate::go) fn char_stands(&self, p: usize) -> bool {
        p < self.lim() || (p == self.lim() && self.nl)
    }

    /// Where the character before `p` starts; the line feed is one.
    pub(in crate::go) fn char_start_before(&self, p: usize) -> usize {
        if p > self.lim() {
            return self.lim();
        }
        p - self.char_before(p).map_or(1, char::len_utf8)
    }

    /// The end of the text with its line feed: where `[\s\S]+` stops.
    pub(in crate::go) fn text_end(&self) -> usize {
        self.lim() + usize::from(self.nl)
    }

    /// `/[*/].*` at `p`: the end of a comment that runs to the end of the
    /// line.
    pub(in crate::go) fn trailing_comment(&self, p: usize) -> Option<usize> {
        (self.at(p) == b'/' && p + 1 < self.lim() && matches!(self.at(p + 1), b'*' | b'/'))
            .then(|| self.lim())
    }

    /// The first `close` at or after `p` in the text: where `[^close]*close`
    /// ends its run.
    pub(in crate::go) fn find_byte(&self, p: usize, close: u8) -> Option<usize> {
        let found = self
            .s
            .get(p..self.lim())
            .and_then(|rest| memchr::memchr(close, rest))
            .map(|at| p + at);
        self.spend(found.map_or(self.lim().saturating_sub(p), |at| at - p) + 1);
        found
    }
}
