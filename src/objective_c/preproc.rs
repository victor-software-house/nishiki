//! The preprocessor: `#if`, `#else`, `#endif` and their kin, `#pragma mark`,
//! `defined`, and the ends that stop at a comment or the line feed.
//!
//! Most directives start `^\s*#\s*` and then name a word with `\b` after it.
//! The word is then the whole run of word characters there.

use super::engine::Lexer;
use crate::rules::Chars;

/// The words `if(?:n?def)?` accepts before `\b`.
pub(super) const IF_WORDS: &[&[u8]] = &[b"if", b"ifdef", b"ifndef"];
/// The words `e(?:lse|ndif)` accepts.
pub(super) const ELSE_ENDIF: &[&[u8]] = &[b"else", b"endif"];
/// The words `e(?:lse|lif|ndif)` accepts.
pub(super) const ELSE_ELIF_ENDIF: &[&[u8]] = &[b"else", b"elif", b"endif"];

/// Whether a run of zeros ends in a one (`0*1`) or holds only zeros (`0+`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Truth {
    /// `0*1`.
    One,
    /// `0+`.
    Zero,
}

impl Lexer<'_> {
    /// The end of `\s*` from `i`, short of the line feed.
    pub(super) fn blank_end(&self, i: usize) -> usize {
        self.space_end(i).min(self.lim())
    }

    /// `^\s*#\s*` and then a word of `words` followed by `\b`: where the `#`
    /// stands, and where the word starts and ends.
    pub(super) fn directive(&self, p: usize, words: &[&[u8]]) -> Option<(usize, usize, usize)> {
        if p != 0 {
            return None;
        }
        let hash = self.blank_end(0);
        if self.at(hash) != b'#' {
            return None;
        }
        let start = self.blank_end(hash + 1);
        let end = self.word_end(start);
        let word = self.text(start, end);
        words.contains(&word).then_some((hash, start, end))
    }

    /// [`Lexer::directive`] for one word.
    pub(super) fn directive_one(&self, p: usize, word: &[u8]) -> Option<(usize, usize, usize)> {
        self.directive(p, &[word])
    }

    /// `^\s*(#(if)\s+(…))\b`, with no blank between `#` and `if`, and then
    /// `0*1` or `0`: where the `#` stands and where the number starts and
    /// ends.
    pub(super) fn interface_if(&self, p: usize, truth: Truth) -> Option<(usize, usize, usize)> {
        if p != 0 {
            return None;
        }
        let hash = self.blank_end(0);
        if !self.starts(hash, b"#if") {
            return None;
        }
        let digits = self.space_end(hash + 3);
        if digits == hash + 3 {
            return None;
        }
        let end = match truth {
            Truth::One => {
                let ones = self.skip(digits, |b| b == b'0');
                (self.at(ones) == b'1' && ones < self.lim()).then_some(ones + 1)?
            }
            Truth::Zero => {
                (self.at(digits) == b'0' && digits < self.lim()).then_some(digits + 1)?
            }
        };
        self.boundary(end).then_some((hash, digits, end))
    }

    /// The lazy `.*?(?:(?=/[*/])|$)` from `i`: the first comment start, or
    /// the end of the line.
    pub(super) fn to_comment(&self, i: usize) -> usize {
        let rest = self.text(i, self.lim());
        let mut at = 0;
        while let Some(slash) = memchr::memchr(b'/', &rest[at..]) {
            let k = at + slash;
            if matches!(rest.get(k + 1), Some(b'*' | b'/')) {
                self.spend(k);
                return i + k;
            }
            at = k + 1;
        }
        self.spend(rest.len());
        self.lim()
    }

    /// `(?=.*\\\s*\n)` at `i`: a backslash at or after `i` that only blanks
    /// follow up to the line feed.
    pub(super) fn continued_from(&self, i: usize) -> bool {
        if self.at(self.lim()) != b'\n' {
            return false;
        }
        let text = self.text(0, self.lim());
        let mut end = text.len();
        while end > i {
            let start = self.char_start_before(end);
            match self.char_at(start) {
                Some((c, _)) if crate::classes::is_space(c) => end = start,
                _ => break,
            }
        }
        end > i && text.get(end - 1) == Some(&b'\\')
    }

    /// `(?=/\*(?!.*\\\s*\n))` at `i`.
    pub(super) fn block_comment_ahead(&self, i: usize) -> bool {
        self.starts(i, b"/*") && !self.continued_from(i + 2)
    }

    /// `(?<!\\)(?=\s*\n)` at `i`: blanks and then the line feed, after no
    /// backslash.
    pub(super) fn blank_to_line_feed(&self, i: usize) -> bool {
        i <= self.lim()
            && !self.prev_is(i, b'\\')
            && self.blank_end(i) == self.lim()
            && self.at(self.lim()) == b'\n'
    }

    /// `(?<!\\)(?=\n)` at `i`.
    pub(super) fn line_feed_here(&self, i: usize) -> bool {
        !self.prev_is(i, b'\\') && i == self.lim() && self.at(i) == b'\n'
    }

    /// The lookahead `(?=\s*\(*\b0*1\b\)*\s*(?:$|//|/\*))` at `i`, with `0+`
    /// for [`Truth::Zero`].
    pub(super) fn condition_is(&self, i: usize, truth: Truth) -> bool {
        let mut q = self.blank_end(i);
        q = self.skip(q, |b| b == b'(');
        if !self.boundary(q) {
            return false;
        }
        let zeros = self.skip(q, |b| b == b'0');
        let end = match truth {
            Truth::One if self.at(zeros) == b'1' && zeros < self.lim() => zeros + 1,
            Truth::Zero if zeros > q => zeros,
            Truth::One | Truth::Zero => return false,
        };
        if !self.boundary(end) {
            return false;
        }
        let r = self.space_end(self.skip(end, |b| b == b')'));
        self.at_eol(r) || self.starts(r, b"//") || self.starts(r, b"/*")
    }

    /// `^\s*((#)\s*word)\b`, recording groups 1 and 2: the end of the word.
    pub(super) fn hash_word(&mut self, p: usize, words: &[&[u8]]) -> Option<(usize, usize)> {
        let (hash, start, end) = self.directive(p, words)?;
        self.cap(1, hash, end);
        self.cap(2, hash, hash + 1);
        Some((start, end))
    }

    /// `^\s*((#)\s*word)\b(?=condition)`.
    pub(super) fn hash_word_if(&mut self, p: usize, word: &[u8], truth: Truth) -> Option<usize> {
        let (hash, _, end) = self.directive_one(p, word)?;
        if !self.condition_is(end, truth) {
            return None;
        }
        self.cap(1, hash, end);
        self.cap(2, hash, hash + 1);
        Some(end)
    }

    /// `^\s*(#\s*(word))\b`, recording group 1 from the `#` and group 2 the
    /// word: the end of the word.
    pub(super) fn hash_word_inner(&mut self, p: usize, words: &[&[u8]]) -> Option<usize> {
        let (hash, start, end) = self.directive(p, words)?;
        self.cap(1, hash, end);
        self.cap(2, start, end);
        Some(end)
    }

    /// `^\s*(#\s*(pragma\s+mark)\s+(.*))`.
    /// With `nested`, C's `^\s*(((#)\s*pragma\s+mark)\s+(.*))`, whose groups
    /// nest one more level and give `#` a group of its own.
    pub(super) fn pragma_mark(&mut self, p: usize, nested: bool) -> Option<usize> {
        if p != 0 {
            return None;
        }
        let hash = self.blank_end(0);
        if self.at(hash) != b'#' {
            return None;
        }
        let pragma = self.blank_end(hash + 1);
        if !self.starts(pragma, b"pragma") {
            return None;
        }
        let mark = self.space_end(pragma + 6);
        if mark == pragma + 6 || !self.starts(mark, b"mark") {
            return None;
        }
        let rest = self.space_end(mark + 4);
        if rest == mark + 4 {
            return None;
        }
        let text_end = rest.max(self.lim());
        if nested {
            self.cap(1, hash, text_end);
            self.cap(2, hash, mark + 4);
            self.cap(3, hash, hash + 1);
            self.cap(4, rest, text_end);
        } else {
            self.cap(1, hash, text_end);
            self.cap(2, pragma, mark + 4);
            self.cap(3, rest, text_end);
        }
        Some(text_end)
    }

    /// `\bdefined\b(?:\s*$|(?=\s*\(*\s*(?!defined\b)[$A-Z_a-z][$\w]*\b\s*\)*\s*
    /// (?:\n|//|/\*|[:?]|&&|\|\||\\\s*\n)))`.
    pub(super) fn defined_test(&self, p: usize) -> Option<usize> {
        let end = self.keyword(p, b"defined")?;
        let after = self.space_end(end);
        if self.at_eol(after) {
            return Some(after);
        }
        let mut q = self.blank_end(end);
        q = self.skip(q, |b| b == b'(');
        q = self.blank_end(q);
        if self.keyword(q, b"defined").is_some() {
            return None;
        }
        let first = self.at(q);
        if !(first == b'$' || first == b'_' || first.is_ascii_alphabetic()) || q >= self.lim() {
            return None;
        }
        let mut i = q + 1;
        loop {
            if self.at(i) == b'$' && i < self.lim() {
                i += 1;
            } else if let Some(width) = self.word_at(i) {
                i += width;
            } else {
                break;
            }
        }
        self.spend(i - q);
        let mut e = i;
        while e > q {
            if self.boundary(e) && self.condition_end(e) {
                return Some(end);
            }
            e = self.char_start_before(e);
        }
        None
    }

    /// `\s*\)*\s*(?:\n|//|/\*|[:?]|&&|\|\||\\\s*\n)` at `i`.
    fn condition_end(&self, i: usize) -> bool {
        let q = self.blank_end(self.skip(self.blank_end(i), |b| b == b')'));
        match self.at(q) {
            b'\n' => q == self.lim(),
            b'/' => matches!(self.at(q + 1), b'/' | b'*'),
            b':' | b'?' => true,
            b'&' => self.at(q + 1) == b'&',
            b'|' => self.at(q + 1) == b'|',
            b'\\' => self.blank_end(q + 1) == self.lim() && self.at(self.lim()) == b'\n',
            _ => false,
        }
    }
}
