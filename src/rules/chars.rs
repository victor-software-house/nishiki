//! The character classes the matchers of the grammars on the rule engine
//! share: `\w`, `\s`, `\d` and `\b` as Oniguruma reads them in UTF-8, and the
//! literal and keyword tests built on them.

use super::{Grammar, Lexer};
use crate::classes::{is_ascii_word, is_digit, is_space, is_word};

/// The character classes and literal tests, for the matchers of every grammar
/// on the rule engine.
///
/// A method of the same name that a language defines on its own lexer takes
/// precedence, so a language moves to these one at a time.
pub(crate) trait Chars {
    /// The width of the `\w` character at `i`, when there is one.
    fn word_at(&self, i: usize) -> Option<usize>;

    /// Whether the character before `i` is a `\w`.
    fn word_before(&self, i: usize) -> bool;

    /// The regex `\b` at `i`.
    fn boundary(&self, i: usize) -> bool;

    /// The width of the `\s` character at `i`, which includes the line feed
    /// that ends the line.
    fn space_at(&self, i: usize) -> usize;

    /// The end of the run of `\w` characters from `i`.
    fn word_end(&self, i: usize) -> usize;

    /// The end of the `\s*` from `i`.
    fn space_end(&self, i: usize) -> usize;

    /// The end of the run of `\d` characters from `i`.
    fn digit_end(&self, i: usize) -> usize;

    /// Whether the character before `i` is `byte`.
    fn prev_is(&self, i: usize, byte: u8) -> bool;

    /// Where the character that ends at `i` starts; the line feed after the
    /// text is one character.
    fn char_start_before(&self, i: usize) -> usize;

    /// The end of `text` at `i` when the line has it there.
    fn lit(&self, i: usize, text: &[u8]) -> Option<usize>;

    /// The end of the word `text` at `i`, with `\b` before and after it.
    fn keyword(&self, i: usize, text: &[u8]) -> Option<usize>;

    /// The end of the word at `i` when `\b` holds before it and the whole word
    /// is one of `words`, which is what `\b(a|b|c)\b` accepts when no word of
    /// the list is a prefix of a word that follows it.
    fn word_in(&self, i: usize, words: &[&[u8]]) -> Option<usize>;
}

/// Every method is `#[inline]`: a language's matchers call them in their own
/// codegen unit, and without it Python lexed about 3% slower than with the
/// inherent copies these replace.
impl<G: Grammar> Chars for Lexer<'_, G> {
    /// The width of the `\w` character at `i`, when there is one.
    #[inline]
    fn word_at(&self, i: usize) -> Option<usize> {
        match self.s.get(i) {
            None => None,
            Some(&b) if b < 0x80 => is_ascii_word(b).then_some(1),
            Some(_) => {
                let (c, end) = self.char_at(i)?;
                is_word(c).then_some(end - i)
            }
        }
    }

    /// Whether the character before `i` is a `\w`.
    #[inline]
    fn word_before(&self, i: usize) -> bool {
        match i.checked_sub(1).and_then(|k| self.s.get(k)) {
            Some(&b) if b < 0x80 => is_ascii_word(b),
            Some(_) => self.char_before(i).is_some_and(is_word),
            None => false,
        }
    }

    /// The regex `\b` at `i`.
    #[inline]
    fn boundary(&self, i: usize) -> bool {
        self.word_before(i) != self.word_at(i).is_some()
    }

    /// The width of the `\s` character at `i`, which includes the line feed
    /// that ends the line.
    #[inline]
    fn space_at(&self, i: usize) -> usize {
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

    /// The end of the run of `\w` characters from `i`.
    #[inline]
    fn word_end(&self, mut i: usize) -> usize {
        let from = i;
        while let Some(width) = self.word_at(i) {
            i += width;
        }
        self.spend(i - from);
        i
    }

    /// The end of the `\s*` from `i`.
    #[inline]
    fn space_end(&self, mut i: usize) -> usize {
        loop {
            let width = self.space_at(i);
            if width == 0 {
                return i;
            }
            i += width;
        }
    }

    /// The end of the run of `\d` characters from `i`.
    #[inline]
    fn digit_end(&self, mut i: usize) -> usize {
        while let Some((c, end)) = self.char_at(i)
            && i < self.lim()
            && is_digit(c)
        {
            i = end;
        }
        i
    }

    /// Whether the character before `i` is `byte`.
    #[inline]
    fn prev_is(&self, i: usize, byte: u8) -> bool {
        i > 0 && self.s.get(i - 1) == Some(&byte)
    }

    /// Where the character that ends at `i` starts; the line feed after the
    /// text is one character.
    #[inline]
    fn char_start_before(&self, i: usize) -> usize {
        if i > self.lim() {
            return self.lim();
        }
        let mut start = i - 1;
        while start > 0 && self.s[start] & 0xc0 == 0x80 {
            start -= 1;
        }
        start
    }

    /// The end of `text` at `i` when the line has it there.
    #[inline]
    fn lit(&self, i: usize, text: &[u8]) -> Option<usize> {
        self.starts(i, text).then(|| i + text.len())
    }

    /// The end of the word `text` at `i`, with `\b` before and after it.
    #[inline]
    fn keyword(&self, i: usize, text: &[u8]) -> Option<usize> {
        let end = self.lit(i, text)?;
        (self.boundary(i) && self.word_at(end).is_none()).then_some(end)
    }

    /// The end of the word at `i` when `\b` holds before it and the whole word
    /// is one of `words`, which is what `\b(a|b|c)\b` accepts when no word of
    /// the list is a prefix of a word that follows it.
    #[inline]
    fn word_in(&self, i: usize, words: &[&[u8]]) -> Option<usize> {
        if !self.boundary(i) {
            return None;
        }
        let end = self.word_end(i);
        words.contains(&self.text(i, end)).then_some(end)
    }
}
