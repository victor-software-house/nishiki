//! The character classes the grammar's expressions are built from, and the
//! runs of them that its lookaheads and lookbehinds look at.

use super::super::ahead::Find;
use super::super::engine::CssLike;
use crate::rules::{Chars, Lexer};
use crate::{case_fold, classes};

/// The longest word of the generated lists, in bytes.
const LONGEST: usize = 48;

/// Which characters [`Lexer::letters_ci`] runs over.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::css) enum Letters {
    /// `[a-z]`.
    Plain,
    /// `[-a-z]`.
    Dashed,
}

impl<G: CssLike> Lexer<'_, G> {
    /// The end of the run of `letters` from `p`: `[a-z]*` or `[-a-z]*` under
    /// `(?i)`, where a character that folds to one letter, such as the long
    /// s, is a letter too.
    pub(in crate::css) fn letters_ci(&self, mut p: usize, letters: Letters) -> usize {
        while let Some((c, end)) = self.char_at(p) {
            let dash = letters == Letters::Dashed && c == '-';
            let one_letter = || case_fold::letters(c).is_some_and(|fold| fold.len() == 1);
            if !(c.is_ascii_alphabetic() || dash || one_letter()) {
                break;
            }
            p = end;
        }
        p
    }

    /// The first `what` at or after `from`.
    pub(in crate::css) fn find(&self, what: Find, from: usize) -> Option<usize> {
        let (found, read) = G::css_extra(&self.x).ahead.next(self.s, what, from);
        self.spend(read);
        found
    }

    /// The end of the white space from `p`: `\s*`.
    ///
    /// It stays out of line: inlined into the matcher's dispatch, its loop
    /// made every pattern try save and restore six registers.
    #[inline(never)]
    pub(crate) fn spaces(&self, p: usize) -> usize {
        let (from, to) = G::css_extra(&self.x).space_run.get();
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
        G::css_extra(&self.x).space_run.set((p, i));
        i
    }

    /// Whether the character before `p` is white space.
    pub(in crate::css) fn space_before(&self, p: usize) -> bool {
        if p == self.lim() + 1 {
            return true;
        }
        match self.char_before(p) {
            Some(c) if c.is_ascii() => {
                matches!(c, ' ' | '\t' | '\n' | '\u{b}' | '\u{c}' | '\r')
            }
            Some(c) => classes::is_space(c),
            None => false,
        }
    }

    /// Whether the character at `p` is in `[-\w]`.
    pub(crate) fn wordish_at(&self, p: usize) -> bool {
        self.at(p) == b'-' && p < self.lim() || self.word_at(p).is_some()
    }

    /// Whether the character before `p` is in `[-\w]`.
    pub(in crate::css) fn wordish_before(&self, p: usize) -> bool {
        match self.char_before(p) {
            Some('-') => true,
            Some(c) if c.is_ascii() => c.is_ascii_alphanumeric() || c == '_',
            Some(c) => classes::is_word(c),
            None => false,
        }
    }

    /// The end of the run of `[-\w]` characters from `p`.
    pub(crate) fn wordish_run(&self, p: usize) -> usize {
        let (from, to) = G::css_extra(&self.x).word_run.get();
        if from <= p && p < to {
            self.spend(1);
            return to;
        }
        let mut i = p;
        while i < self.lim() && self.wordish_at(i) {
            i = self.char_end(i);
        }
        self.spend(i - p + 1);
        G::css_extra(&self.x).word_run.set((p, i));
        i
    }

    /// The end of the run of `\w` characters from `p`.
    pub(in crate::css) fn word_run(&self, p: usize) -> usize {
        let mut i = p;
        while i < self.lim() && self.word_at(i).is_some() {
            i = self.char_end(i);
        }
        self.spend(i - p + 1);
        i
    }

    /// The end of the character at `p` when it is one of
    /// `[-A-Z_a-z[^\x00-\x7F]]`.
    pub(in crate::css) fn ident_start(&self, p: usize) -> Option<usize> {
        let b = *self.s.get(p)?;
        (b.is_ascii_alphabetic() || b == b'-' || b == b'_' || b >= 0x80).then(|| self.char_end(p))
    }

    /// The end of the character at `p` when it is one of
    /// `[-0-9A-Z_a-z[^\x00-\x7F]]`.
    pub(in crate::css) fn ident_char(&self, p: usize) -> Option<usize> {
        let b = *self.s.get(p)?;
        (b.is_ascii_alphanumeric() || b == b'-' || b == b'_' || b >= 0x80).then(|| self.char_end(p))
    }

    /// The end of `\\\h{1,6}` at `p`: a backslash and up to six hex digits.
    pub(in crate::css) fn hex_escape_end(&self, p: usize) -> Option<usize> {
        if self.s.get(p) != Some(&b'\\') {
            return None;
        }
        let mut end = p + 1;
        while end < self.lim() && end < p + 7 && self.s[end].is_ascii_hexdigit() {
            end += 1;
        }
        (end > p + 1).then_some(end)
    }

    /// The end of `\\(?:\h{1,6}|.)` at `p`.
    pub(in crate::css) fn escape_end(&self, p: usize) -> Option<usize> {
        if self.s.get(p) != Some(&b'\\') {
            return None;
        }
        self.hex_escape_end(p)
            .or_else(|| (p + 1 < self.lim()).then(|| self.char_end(p + 1)))
    }

    /// The end of the run of `(?:[-0-9A-Z_a-z[^\x00-\x7F]]|\\(?:\h{1,6}|.))`
    /// from `p`.
    pub(in crate::css) fn ident_run(&self, p: usize) -> usize {
        let (from, to) = G::css_extra(&self.x).ident_run.get();
        if from <= p && p < to {
            self.spend(1);
            return to;
        }
        let mut i = p;
        let mut escaped = false;
        while let Some(end) = self.ident_char(i).or_else(|| {
            let end = self.escape_end(i);
            escaped |= end.is_some();
            end
        }) {
            i = end;
        }
        self.spend(i - p + 1);
        G::css_extra(&self.x)
            .ident_run
            .set(if escaped { (usize::MAX, 0) } else { (p, i) });
        i
    }

    /// The end of
    /// `[-A-Z_a-z[^\x00-\x7F]](?:[-0-9A-Z_a-z[^\x00-\x7F]]|\\(?:\h{1,6}|.))*`
    /// at `p`: a name, whose first character is not an escape.
    pub(in crate::css) fn name_end(&self, p: usize) -> Option<usize> {
        let first = self.ident_start(p)?;
        Some(self.ident_run(first))
    }

    /// Whether the text `from..to`, folded to lower case as `(?i)` reads it
    /// ([`case_fold::lower_into`]), is one of `words`, which are lower case
    /// and sorted.
    pub(in crate::css) fn in_list(&self, from: usize, to: usize, words: &[&str]) -> bool {
        debug_assert!(words.is_sorted(), "word list not sorted: {words:?}");
        let mut buf = [0; LONGEST];
        self.s
            .get(from..to)
            .and_then(|text| case_fold::lower_into(text, &mut buf))
            .is_some_and(|n| {
                words
                    .binary_search_by(|word| word.as_bytes().cmp(&buf[..n]))
                    .is_ok()
            })
    }

    /// Whether the text `from..to`, folded to lower case, is `word`.
    pub(in crate::css) fn is_word_ci(&self, from: usize, to: usize, word: &str) -> bool {
        let mut buf = [0; LONGEST];
        self.s
            .get(from..to)
            .and_then(|text| case_fold::lower_into(text, &mut buf))
            .is_some_and(|n| &buf[..n] == word.as_bytes())
    }

    /// `word` at `p` without regard to case, a character above ASCII standing
    /// for the letters it folds to, such as `ß` for `ss`, and its end.
    pub(in crate::css) fn lit_ci(&self, p: usize, word: &str) -> Option<usize> {
        case_fold::literal_end(self.s, p, word.as_bytes())
    }

    /// The end of a run of `[-\w]` characters from `p` that is one of `words`
    /// (lower case, sorted) and has no `[-\w]` character around it: the
    /// grammar's `(?<![-\w])(?:word|word|...)(?![-\w])`.
    pub(in crate::css) fn standalone_word(&self, p: usize, words: &[&str]) -> Option<usize> {
        if self.wordish_before(p) {
            return None;
        }
        let end = self.wordish_run(p);
        (end > p && self.in_list(p, end, words)).then_some(end)
    }

    /// `\d` at `p`: the end of the digit.
    pub(in crate::css) fn digit_at(&self, p: usize) -> Option<usize> {
        match self.at(p) {
            0 => None,
            b if b < 0x80 => b.is_ascii_digit().then_some(p + 1),
            _ => self
                .char_at(p)
                .and_then(|(c, end)| classes::is_digit(c).then_some(end)),
        }
    }
}
