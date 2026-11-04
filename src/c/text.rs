//! The text a pattern is matched against, and the character classes of the
//! grammar's regular expressions: `\w`, `\s`, `\d` and `\b`.

use std::cell::Cell;

use super::engine::{Lexer, Run};
use crate::classes;
use crate::classes::is_ascii_space;
use crate::rules::DynName;
use crate::utf8::char_len;

impl Lexer<'_> {
    /// The number of characters the text has: the line, and its line feed.
    pub(super) fn len_all(&self) -> usize {
        self.s.len() + usize::from(self.nl)
    }

    /// The byte at `i`.
    /// It is the line feed just after the text, and 0 past that.
    #[inline]
    pub(super) fn b(&self, i: usize) -> u8 {
        self.at(i)
    }

    /// The width of the character at `i`, at least 1.
    pub(super) fn width_at(&self, i: usize) -> usize {
        self.s.get(i).map_or(1, |&lead| char_len(lead))
    }

    /// The character that ends at `i`, counting the line feed after the
    /// text as one.
    pub(super) fn char_ending(&self, i: usize) -> Option<char> {
        if i > self.lim() {
            Some('\n')
        } else {
            self.char_before(i)
        }
    }

    /// The character at `i` and its width, for a byte that is not ASCII.
    fn wide_char_at(&self, i: usize) -> Option<(char, usize)> {
        self.char_at(i).map(|(c, end)| (c, end - i))
    }

    /// The width of the `\w` character at `i`, or 0.
    pub(super) fn word_at(&self, i: usize) -> usize {
        let b = self.b(i);
        if b < 0x80 {
            return usize::from(i < self.len_all() && (b.is_ascii_alphanumeric() || b == b'_'));
        }
        match self.wide_char_at(i) {
            Some((c, width)) if classes::is_word(c) => width,
            _ => 0,
        }
    }

    /// Whether the character before `i` is a `\w` character.
    pub(super) fn word_before(&self, i: usize) -> bool {
        match self.prev(i) {
            None => false,
            Some(b) if b < 0x80 => b.is_ascii_alphanumeric() || b == b'_',
            Some(_) => self.char_ending(i).is_some_and(classes::is_word),
        }
    }

    /// The width of the `\s` character at `i`, or 0.
    pub(super) fn space_at(&self, i: usize) -> usize {
        let b = self.b(i);
        if b < 0x80 {
            return usize::from(i < self.len_all() && is_ascii_space(b));
        }
        match self.wide_char_at(i) {
            Some((c, width)) if classes::is_space(c) => width,
            _ => 0,
        }
    }

    /// The width of the `\d` character at `i`, or 0.
    pub(super) fn digit_at(&self, i: usize) -> usize {
        let b = self.b(i);
        if b < 0x80 {
            return usize::from(i < self.len_all() && b.is_ascii_digit());
        }
        match self.wide_char_at(i) {
            Some((c, width)) if classes::is_digit(c) => width,
            _ => 0,
        }
    }

    /// The end of the run that starts at `i` and holds the characters
    /// `width_of` accepts.
    /// Each run is found once.
    /// `cache` keeps the last run.
    /// A search from inside that run ends where the run does.
    fn run_end(&self, i: usize, cache: &Cell<Run>, width_of: impl Fn(usize) -> usize) -> usize {
        let run = cache.get();
        if run.holds(i) {
            self.spend(1);
            return run.end;
        }
        let mut j = i;
        loop {
            let width = width_of(j);
            if width == 0 {
                break;
            }
            j += width;
        }
        self.spend(j - i + 1);
        cache.set(Run { start: i, end: j });
        j
    }

    /// The end of `\s*` from `i`.
    pub(super) fn skip_space(&self, i: usize) -> usize {
        self.run_end(i, &self.x.runs.space, |j| self.space_at(j))
    }

    /// The end of `\w*` from `i`.
    pub(super) fn skip_word(&self, i: usize) -> usize {
        self.run_end(i, &self.x.runs.word, |j| self.word_at(j))
    }

    /// The end of `[A-Za-z_]\w*` at `i`.
    pub(super) fn ident_end(&self, i: usize) -> Option<usize> {
        let b = self.b(i);
        (i < self.len_all() && (b.is_ascii_alphabetic() || b == b'_'))
            .then(|| self.skip_word(i + 1))
    }

    /// The end of `[A-Za-z_][0-9A-Za-z_]*` at `i`, in ASCII only.
    pub(super) fn ascii_ident_end(&self, i: usize) -> Option<usize> {
        let b = self.b(i);
        if i >= self.len_all() || !(b.is_ascii_alphabetic() || b == b'_') {
            return None;
        }
        Some(self.run_end(i + 1, &self.x.runs.ascii_name, |j| {
            let c = self.b(j);
            usize::from(j < self.len_all() && (c.is_ascii_alphanumeric() || c == b'_'))
        }))
    }

    /// The end of the run of colons from `i`.
    pub(super) fn colons_end(&self, i: usize) -> usize {
        self.run_end(i, &self.x.runs.colons, |j| {
            usize::from(self.b(j) == b':' && j < self.len_all())
        })
    }

    /// The regex `\b` at `i`.
    pub(super) fn boundary(&self, i: usize) -> bool {
        self.word_before(i) != (self.word_at(i) > 0)
    }

    // Hits.

    /// A match from `start` to `end`. Its pattern's [`super::lists::P::act`]
    /// says what it does and names it.
    pub(super) fn plain(&mut self, start: usize, end: usize) -> bool {
        self.x.start = start;
        self.x.end = end;
        true
    }

    /// A begin match from `start` to `end`.
    /// Its pattern names the rule it opens.
    pub(super) fn begin(&mut self, start: usize, end: usize) -> bool {
        self.plain(start, end)
    }

    /// A match whose name is `prefix`, the text `text` and `.c`.
    pub(super) fn plain_dyn(
        &mut self,
        start: usize,
        end: usize,
        prefix: &'static str,
        text: (usize, usize),
    ) -> bool {
        self.cap_dyn(start, end, prefix, text);
        self.plain(start, end)
    }

    /// A capture whose name is `prefix`, the text `text` and `.c`.
    pub(super) fn cap_dyn(
        &mut self,
        start: usize,
        end: usize,
        prefix: &'static str,
        text: (usize, usize),
    ) {
        self.span_dyn(
            start,
            end,
            DynName {
                prefix,
                start: text.0,
                end: text.1,
                suffix: ".c",
            },
        );
    }
}
