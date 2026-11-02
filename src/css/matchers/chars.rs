//! The character classes the grammar's expressions are built from, and the
//! runs of them that its lookaheads and lookbehinds look at.

use super::super::ahead::{Ahead, Find};
use super::super::engine::Lexer;
use crate::classes;

/// The longest word of the generated lists, in bytes.
const LONGEST: usize = 48;

impl Lexer<'_> {
    /// The end of the run of letters from `p`, with `-` when `dash`: `[a-z]*`
    /// or `[-a-z]*` under `(?i)`, where the long s and the Kelvin sign are
    /// letters too.
    pub(in crate::css) fn letters_ci(&self, mut p: usize, dash: bool) -> usize {
        while let Some((c, end)) = self.char_at(p) {
            if !(c.is_ascii_alphabetic() || c == '\u{17f}' || c == '\u{212a}' || dash && c == '-') {
                break;
            }
            p = end;
        }
        p
    }

    /// The first `what` at or after `from`.
    pub(in crate::css) fn find(&self, what: Find, from: usize) -> Option<usize> {
        self.x
            .ahead
            .get_or_init(|| Ahead::new(self.s))
            .next(what, from)
    }

    /// `text` at `p`: the end of it.
    pub(in crate::css) fn lit(&self, p: usize, text: &[u8]) -> Option<usize> {
        self.starts(p, text).then_some(p + text.len())
    }

    /// The width of the `\s` character at `p`, 0 when there is none. The line
    /// feed after the line is one.
    pub(in crate::css) fn space_width(&self, p: usize) -> usize {
        match self.at(p) {
            0 => 0,
            b if b < 0x80 => usize::from(matches!(b, b' ' | b'\t' | b'\n' | 0x0b | 0x0c | b'\r')),
            _ => match self.char_at(p) {
                Some((c, end)) if classes::is_space(c) => end - p,
                _ => 0,
            },
        }
    }

    /// The end of the white space from `p`: `\s*`.
    pub(in crate::css) fn spaces(&self, p: usize) -> usize {
        let (from, to) = self.x.space_run.get();
        if from <= p && p < to {
            self.spend(1);
            return to;
        }
        let mut i = p;
        loop {
            let width = self.space_width(i);
            if width == 0 {
                break;
            }
            i += width;
        }
        self.spend(i - p + 1);
        self.x.space_run.set((p, i));
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

    /// Whether the character at `p` is a `\w`.
    pub(in crate::css) fn word_at(&self, p: usize) -> bool {
        match self.at(p) {
            0 => false,
            b if b < 0x80 => b.is_ascii_alphanumeric() || b == b'_',
            _ => self.char_at(p).is_some_and(|(c, _)| classes::is_word(c)),
        }
    }

    /// Whether the character at `p` is in `[-\w]`.
    pub(in crate::css) fn wordish_at(&self, p: usize) -> bool {
        self.at(p) == b'-' && p < self.lim() || self.word_at(p)
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

    /// Whether the character before `p` is a `\w`.
    pub(in crate::css) fn word_before(&self, p: usize) -> bool {
        match self.char_before(p) {
            Some(c) if c.is_ascii() => c.is_ascii_alphanumeric() || c == '_',
            Some(c) => classes::is_word(c),
            None => false,
        }
    }

    /// The regex `\b` at `p`.
    pub(in crate::css) fn boundary(&self, p: usize) -> bool {
        self.word_before(p) != self.word_at(p)
    }

    /// The end of the run of `[-\w]` characters from `p`.
    pub(in crate::css) fn wordish_run(&self, p: usize) -> usize {
        let (from, to) = self.x.word_run.get();
        if from <= p && p < to {
            self.spend(1);
            return to;
        }
        let mut i = p;
        while i < self.lim() && self.wordish_at(i) {
            i = self.char_end(i);
        }
        self.spend(i - p + 1);
        self.x.word_run.set((p, i));
        i
    }

    /// The end of the run of `\w` characters from `p`.
    pub(in crate::css) fn word_run(&self, p: usize) -> usize {
        let mut i = p;
        while i < self.lim() && self.word_at(i) {
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

    /// The end of `\\(?:\h{1,6}|.)` at `p`.
    pub(in crate::css) fn escape_end(&self, p: usize) -> Option<usize> {
        if self.s.get(p) != Some(&b'\\') {
            return None;
        }
        let mut i = p + 1;
        while i < self.lim() && i < p + 7 && self.s[i].is_ascii_hexdigit() {
            i += 1;
        }
        if i > p + 1 {
            return Some(i);
        }
        (i < self.lim()).then(|| self.char_end(i))
    }

    /// The end of the run of `(?:[-0-9A-Z_a-z[^\x00-\x7F]]|\\(?:\h{1,6}|.))`
    /// from `p`.
    pub(in crate::css) fn ident_run(&self, p: usize) -> usize {
        let (from, to) = self.x.ident_run.get();
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
        self.x
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

    /// The text `from..to` folded to lower case as `(?i)` reads it, into
    /// `buf`: the long s is an `s` and the Kelvin sign a `k`. `None` when a
    /// character cannot be in a word of the lists, or the text is too long.
    fn fold(&self, from: usize, to: usize, buf: &mut [u8; LONGEST]) -> Option<usize> {
        let mut n = 0;
        let mut i = from;
        while i < to {
            let (c, end) = self.char_at(i)?;
            let folded = match c {
                '\u{17f}' => b's',
                '\u{212a}' => b'k',
                c if c.is_ascii() => u8::try_from(c).ok()?.to_ascii_lowercase(),
                _ => return None,
            };
            *buf.get_mut(n)? = folded;
            n += 1;
            i = end;
        }
        Some(n)
    }

    /// Whether the text `from..to`, folded to lower case, is one of `words`,
    /// which are lower case and sorted.
    pub(in crate::css) fn in_list(&self, from: usize, to: usize, words: &[&str]) -> bool {
        let mut buf = [0; LONGEST];
        self.fold(from, to, &mut buf).is_some_and(|n| {
            words
                .binary_search_by(|word| word.as_bytes().cmp(&buf[..n]))
                .is_ok()
        })
    }

    /// Whether the text `from..to`, folded to lower case, is `word`.
    pub(in crate::css) fn is_word_ci(&self, from: usize, to: usize, word: &str) -> bool {
        let mut buf = [0; LONGEST];
        self.fold(from, to, &mut buf)
            .is_some_and(|n| &buf[..n] == word.as_bytes())
    }

    /// `word` at `p`, without regard to case, and its end.
    pub(in crate::css) fn lit_ci(&self, p: usize, word: &str) -> Option<usize> {
        let mut i = p;
        for &want in word.as_bytes() {
            let (c, end) = self.char_at(i)?;
            let same = match c {
                '\u{17f}' => want == b's',
                '\u{212a}' => want == b'k',
                c if c.is_ascii() => u8::try_from(c).ok()?.to_ascii_lowercase() == want,
                _ => false,
            };
            if !same {
                return None;
            }
            i = end;
        }
        Some(i)
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
    pub(in crate::css) fn digit_end(&self, p: usize) -> Option<usize> {
        match self.at(p) {
            0 => None,
            b if b < 0x80 => b.is_ascii_digit().then_some(p + 1),
            _ => self
                .char_at(p)
                .and_then(|(c, end)| classes::is_digit(c).then_some(end)),
        }
    }

    /// The end of the run of `\d` characters from `p`.
    pub(in crate::css) fn digits(&self, mut p: usize) -> usize {
        while let Some(end) = self.digit_end(p) {
            p = end;
        }
        p
    }

    /// Whether `$` matches at `p`: before the line feed or at the end.
    pub(in crate::css) fn at_end(&self, p: usize) -> bool {
        self.at_eol(p)
    }
}
