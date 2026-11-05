//! Reading the line: bytes, regex classes, runs and word boundaries.

use super::lexer::Lexer;
use crate::classes::{is_alnum, is_alpha, space_width, word_before, word_width};
use crate::utf8::{char_len, char_start, multibyte_at};

impl<'a> Lexer<'a> {
    /// The byte at `i`, or 0 past the end.
    pub(super) fn at(&self, i: usize) -> u8 {
        self.s.get(i).copied().unwrap_or(0)
    }

    /// The width of the character at `i`, at least 1.
    pub(super) fn width_at(&self, i: usize) -> usize {
        let Some(&lead) = self.s.get(i) else {
            return 1;
        };
        char_len(lead).min(self.s.len() - i)
    }

    /// The width of the character at `i` when `class` holds for it, and
    /// `ascii` for its byte if it is ASCII; 0 otherwise, past the end too.
    fn width_if(&self, i: usize, ascii: fn(u8) -> bool, class: fn(char) -> bool) -> usize {
        match self.s.get(i) {
            None => 0,
            Some(&b) if b < 0x80 => usize::from(ascii(b)),
            Some(&lead) => multibyte_at(self.s, i, lead)
                .filter(|&(c, _)| class(c))
                .map_or(0, |(_, width)| width),
        }
    }

    /// The width of the `\p{alpha}` character at `i`, or 0.
    pub(super) fn alpha_width(&self, i: usize) -> usize {
        self.width_if(i, |b| b.is_ascii_alphabetic(), is_alpha)
    }

    /// Whether a `\w` character starts at `i`.
    pub(super) fn word_at(&self, i: usize) -> bool {
        word_width(self.s, i) > 0
    }

    /// Whether `\b` holds at `i`.
    pub(super) fn boundary(&self, i: usize) -> bool {
        word_before(self.s, i) != self.word_at(i)
    }

    /// The end of the run that starts at `i` of characters `width` measures.
    fn run_of(&self, i: usize, width: impl Fn(&Self, usize) -> usize) -> usize {
        let mut end = i;
        loop {
            let w = width(self, end);
            if w == 0 {
                self.steps.spend(end - i);
                return end;
            }
            end += w;
        }
    }

    /// The end of the run of white space that starts at `i`: `\s*`.
    pub(super) fn skip_space(&self, i: usize) -> usize {
        self.run_of(i, |lexer, at| space_width(lexer.s, at))
    }

    /// The end of the run of white space other than `\r` that starts at `i`:
    /// `[^\n\r\S]*`.
    /// A line holds no `\n`.
    pub(super) fn skip_blanks(&self, i: usize) -> usize {
        self.run_of(i, |lexer, at| {
            if lexer.at(at) == b'\r' {
                0
            } else {
                space_width(lexer.s, at)
            }
        })
    }

    /// The end of the `\w*` run at `i`.
    pub(super) fn word_end(&self, i: usize) -> usize {
        self.run_of(i, |lexer, at| word_width(lexer.s, at))
    }

    /// The end of the `[-\w]*` run at `i`, which is `[-_\w]*` too.
    pub(super) fn dash_word_end(&self, i: usize) -> usize {
        self.run_of(i, |lexer, at| {
            if lexer.at(at) == b'-' {
                1
            } else {
                word_width(lexer.s, at)
            }
        })
    }

    /// The end of the `[-_[:alnum:]]*` run at `i`.
    pub(super) fn name_end(&self, i: usize) -> usize {
        self.run_of(i, |lexer, at| {
            if matches!(lexer.at(at), b'-' | b'_') {
                1
            } else {
                lexer.width_if(at, |b| b.is_ascii_alphanumeric(), is_alnum)
            }
        })
    }

    /// The end of the `\d*` run at `i`.
    pub(super) fn digits_end(&self, i: usize) -> usize {
        self.run_of(i, |lexer, at| crate::classes::digit_width(lexer.s, at))
    }

    /// The last byte after `start` and up to `end` where `\b` holds: where a
    /// run that a trailing `\b` follows ends once the regex backtracks.
    pub(super) fn trim(&self, start: usize, end: usize) -> Option<usize> {
        let mut e = end;
        while e > start {
            if self.boundary(e) {
                return Some(e);
            }
            self.steps.spend(1);
            e = char_start(self.s, e - 1);
        }
        None
    }

    /// Whether `null`, `false` or `true` starts at `i`, as the lookahead
    /// `(?!null|false|true)` reads it: with no `\b` after the word.
    pub(super) fn keyword_prefix(&self, i: usize) -> bool {
        let rest = self.s.get(i..).unwrap_or_default();
        rest.starts_with(b"null") || rest.starts_with(b"false") || rest.starts_with(b"true")
    }

    /// Whether `word` starts at `i` and `\b` follows it.
    /// Each word this asks for ends with a word character.
    pub(super) fn word_is(&self, i: usize, word: &[u8]) -> bool {
        self.s.get(i..).is_some_and(|rest| rest.starts_with(word)) && !self.word_at(i + word.len())
    }

    /// The `\w` run at `i` when `\b` comes before it.
    pub(super) fn whole_word(&self, i: usize) -> Option<&'a [u8]> {
        if word_before(self.s, i) {
            return None;
        }
        let s = self.s;
        s.get(i..self.word_end(i))
    }
}
