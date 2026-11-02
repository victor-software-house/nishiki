//! The character classes of the grammar's expressions.

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::python::engine::Lexer;
use crate::python::{classes, words};

/// The regex `\w` for a character.
pub(super) fn is_word(c: char) -> bool {
    if c.is_ascii() {
        c.is_ascii_alphanumeric() || c == '_'
    } else {
        classes::is_word(c)
    }
}

/// The regex `[[:alpha:]]` for a character.
pub(super) fn is_alpha(c: char) -> bool {
    if c.is_ascii() {
        c.is_ascii_alphabetic()
    } else {
        classes::is_alpha(c)
    }
}

/// The regex `\s` for a character.
pub(super) fn is_space(c: char) -> bool {
    if c.is_ascii() {
        matches!(c, ' ' | '\t' | '\n' | '\u{0b}' | '\u{0c}' | '\r')
    } else {
        classes::is_space(c)
    }
}

/// The regex `\d` for a character.
pub(super) fn is_digit(c: char) -> bool {
    if c.is_ascii() {
        c.is_ascii_digit()
    } else {
        classes::is_digit(c)
    }
}

/// The regex `\p{upper}` for a character.
pub(super) fn is_upper(c: char) -> bool {
    if c.is_ascii() {
        c.is_ascii_uppercase()
    } else {
        classes::is_upper(c)
    }
}

/// The regex `\p{alnum}` for a character.
pub(super) fn is_alnum(c: char) -> bool {
    is_alpha(c) || is_digit(c)
}

impl Lexer<'_> {
    /// The width of the `\w` character at `i`, when there is one.
    pub(in crate::python) fn word_at(&self, i: usize) -> Option<usize> {
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
    pub(in crate::python) fn word_before(&self, i: usize) -> bool {
        match i.checked_sub(1).and_then(|k| self.s.get(k)) {
            Some(&b) if b < 0x80 => b.is_ascii_alphanumeric() || b == b'_',
            Some(_) => self.char_before(i).is_some_and(is_word),
            None => false,
        }
    }

    /// The regex `\b` at `i`.
    pub(in crate::python) fn boundary(&self, i: usize) -> bool {
        self.word_before(i) != self.word_at(i).is_some()
    }

    /// The end of the run of `\w` characters from `i`. Patterns ask about
    /// the same word one after another, so the last run is kept.
    pub(in crate::python) fn word_end(&self, i: usize) -> usize {
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

    /// The width of the `\s` character at `i`, which includes the line feed
    /// that ends the line.
    pub(in crate::python) fn space_at(&self, i: usize) -> usize {
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

    /// The end of the run of `\d` characters from `i`.
    pub(in crate::python) fn digit_end(&self, mut i: usize) -> usize {
        while let Some((c, end)) = self.char_at(i)
            && i < self.lim()
            && is_digit(c)
        {
            i = end;
        }
        i
    }

    /// The end of the `\s*` from `i`.
    pub(in crate::python) fn space_end(&self, mut i: usize) -> usize {
        loop {
            let width = self.space_at(i);
            if width == 0 {
                return i;
            }
            i += width;
        }
    }

    /// The end of the identifier `[_[:alpha:]]\w*` at `i`, when one starts.
    pub(in crate::python) fn ident_end(&self, i: usize) -> Option<usize> {
        let (c, first) = self.char_at(i)?;
        if c != '_' && !is_alpha(c) {
            return None;
        }
        Some(self.word_end(first))
    }

    /// The end of the word at `i` when `\b` holds before it and the word is
    /// one of `list`, a sorted list of words.
    pub(in crate::python) fn word_in(&self, i: usize, list: &[&[u8]]) -> Option<usize> {
        if !self.boundary(i) {
            return None;
        }
        let end = self.word_end(i);
        list.binary_search(&self.text(i, end)).ok().map(|_| end)
    }

    /// The end of the word at `i` when `\b` holds before it and one of the
    /// word lists in `bits` has the word.
    pub(in crate::python) fn word_listed(&self, i: usize, bits: u8) -> Option<usize> {
        if !self.boundary(i) {
            return None;
        }
        let end = self.word_end(i);
        let (at, mask) = self.x.word_mask.get();
        let mask = if at == i {
            mask
        } else {
            let mask = word_masks().get(self.text(i, end)).copied().unwrap_or(0);
            self.x.word_mask.set((i, mask));
            mask
        };
        (mask & bits != 0).then_some(end)
    }

    /// Whether the character before `i` is `byte`.
    pub(in crate::python) fn prev_is(&self, i: usize, byte: u8) -> bool {
        i > 0 && self.s.get(i - 1) == Some(&byte)
    }
}

/// The word lists a word can be in.
pub(in crate::python) const FUNCTION: u8 = 1;
pub(in crate::python) const LEGACY: u8 = 2;
pub(in crate::python) const TYPE: u8 = 4;
pub(in crate::python) const EXCEPTION: u8 = 8;
pub(in crate::python) const MAGIC_FUNCTION: u8 = 16;
pub(in crate::python) const MAGIC_VARIABLE: u8 = 32;

/// Which of the big word lists have each word, built on first use.
fn word_masks() -> &'static HashMap<&'static [u8], u8> {
    static MASKS: OnceLock<HashMap<&'static [u8], u8>> = OnceLock::new();
    MASKS.get_or_init(|| {
        let mut masks: HashMap<&'static [u8], u8> = HashMap::new();
        for (list, bit) in [
            (words::BUILTIN_FUNCTIONS, FUNCTION),
            (words::LEGACY_BUILTINS, LEGACY),
            (words::BUILTIN_TYPES, TYPE),
            (words::BUILTIN_EXCEPTIONS, EXCEPTION),
            (words::MAGIC_FUNCTIONS, MAGIC_FUNCTION),
            (words::MAGIC_VARIABLES, MAGIC_VARIABLE),
        ] {
            for word in list {
                *masks.entry(word).or_insert(0) |= bit;
            }
        }
        masks
    })
}
