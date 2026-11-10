//! Characters and words at a position.
//! The module covers the character classes the grammar's regular expressions
//! spell, blanks, names and keywords.
//! It also covers the lookbehinds that test the text before a position.

use super::bytes::{ASCII_NAME_CHAR, ASCII_NAME_START, Bytes};
use super::lexer::Lexer;
use crate::classes::is_ascii_space;
use crate::{classes, space};

/// `[$_[:alnum:]]`.
/// Oniguruma's `[[:alnum:]]` is its `[[:alpha:]]` and `\d`.
pub(crate) fn is_alnum_char(c: char) -> bool {
    c == '$' || c == '_' || classes::is_alpha(c) || classes::is_digit(c)
}

/// `[$_[:alpha:]]`.
pub(crate) fn is_alpha_char(c: char) -> bool {
    c == '$' || c == '_' || classes::is_alpha(c)
}

/// What an operator lookbehind accepts before a position.
/// See [`Lexer::operator_before`].
pub(crate) struct Before {
    /// Bytes that may end the text before it.
    pub(crate) bytes: Bytes,
    /// Longer operators that may end it, such as `&&`.
    pub(crate) operators: &'static [&'static str],
    /// Words that may end it, neither inside a longer name nor after a dot.
    pub(crate) words: &'static [&'static str],
    /// What may not end it, though a byte or operator above would match.
    pub(crate) never: &'static [&'static str],
}

impl<'a, const D: u8> Lexer<'a, D> {
    /// The width of the character at `i`, 1 at or past the end.
    pub(crate) fn char_len(&self, i: usize) -> usize {
        self.spend(1);
        self.src
            .get(i..)
            .and_then(|rest| rest.chars().next())
            .map_or(1, char::len_utf8)
    }

    pub(crate) fn char_at(&self, i: usize) -> Option<char> {
        self.src.get(i..).and_then(|rest| rest.chars().next())
    }

    /// The regex `\s` at `i`, the newline that ends the line included.
    pub(crate) fn space_at(&self, i: usize) -> bool {
        is_ascii_space(self.byte(i)) || space::len(self.bytes.get(i..).unwrap_or_default()) > 0
    }

    /// The character before `i`.
    /// The newline that ends the line, when one follows, is the one before
    /// `len + 1`.
    pub(crate) fn prev_char(&self, i: usize) -> Option<char> {
        if i == self.len + 1 {
            return (self.byte(self.len) == b'\n').then_some('\n');
        }
        self.src.get(..i).and_then(|head| head.chars().next_back())
    }

    pub(crate) fn starts(&self, i: usize, lit: &str) -> bool {
        self.bytes
            .get(i..)
            .is_some_and(|rest| crate::bytes::starts_with(rest, lit.as_bytes()))
    }

    /// Whether the line, with its newline, ends at or before `i`.
    pub(crate) fn eol(&self, i: usize) -> bool {
        i >= self.len
    }

    /// Skips `\s*`.
    ///
    /// Rules need the end of a run of blanks at every position inside it.
    /// So the last run found is remembered, and a long run costs one scan.
    pub(crate) fn ws(&self, i: usize) -> usize {
        let (start, end) = self.blanks.get();
        if i >= start && i < end {
            return end;
        }
        let mut j = i;
        loop {
            if is_ascii_space(self.byte(j)) {
                j += 1;
                continue;
            }
            match space::len(self.bytes.get(j..).unwrap_or_default()) {
                0 => break,
                n => j += n,
            }
        }
        self.spend(j - i + 1);
        self.blanks.set((i, j));
        j
    }

    /// `[$_[:alpha:]]` at `i`.
    pub(crate) fn id_start(&self, i: usize) -> bool {
        match self.bytes.get(i) {
            Some(&b) if b < 0x80 => ASCII_NAME_START.has(b),
            Some(_) => self.char_at(i).is_some_and(is_alpha_char),
            None => false,
        }
    }

    /// `[$_[:alnum:]]` at `i`.
    pub(crate) fn alnum_at(&self, i: usize) -> bool {
        match self.bytes.get(i) {
            Some(&b) if b < 0x80 => ASCII_NAME_CHAR.has(b),
            Some(_) => self.char_at(i).is_some_and(is_alnum_char),
            None => false,
        }
    }

    /// `[$_[:alnum:]]` right before `i`.
    pub(crate) fn alnum_before(&self, i: usize) -> bool {
        self.prev_char(i).is_some_and(is_alnum_char)
    }

    /// The end of `[$_[:alnum:]]*` from `i`.
    pub(crate) fn alnum_end(&self, from: usize) -> usize {
        let mut i = from;
        while let Some(&b) = self.bytes.get(i) {
            if b < 0x80 {
                if !ASCII_NAME_CHAR.has(b) {
                    break;
                }
                i += 1;
            } else {
                match self.char_at(i) {
                    Some(c) if is_alnum_char(c) => i += c.len_utf8(),
                    _ => break,
                }
            }
        }
        self.spend(i - from + 1);
        i
    }

    /// `[$_[:alpha:]][$_[:alnum:]]*` at `i`: its end.
    ///
    /// Every rule reads the name at the position.
    /// The last answer is therefore kept.
    pub(crate) fn ident(&self, i: usize) -> Option<usize> {
        let (at, end) = self.last_ident.get();
        if at == i {
            return end;
        }
        let end = self
            .id_start(i)
            .then(|| self.alnum_end(i + self.char_len(i)));
        self.last_ident.set((i, end));
        end
    }

    /// `#?[$_[:alpha:]][$_[:alnum:]]*` at `i`: its end.
    pub(crate) fn hash_ident(&self, i: usize) -> Option<usize> {
        let j = if self.byte(i) == b'#' { i + 1 } else { i };
        self.ident(j)
    }

    /// The regex `\w` before `i`.
    fn word_before(&self, i: usize) -> bool {
        self.prev_char(i).is_some_and(classes::is_word)
    }

    fn word_at(&self, i: usize) -> bool {
        i < self.len && self.char_at(i).is_some_and(classes::is_word)
    }

    /// The regex `\b` at `i`.
    pub(crate) fn boundary(&self, i: usize) -> bool {
        self.word_before(i) != self.word_at(i)
    }

    /// `(?<![$_[:alnum:]])(?:(?<=\.\.\.)|(?<!\.))`, the grammar's guard before
    /// a keyword: not inside a name and not a property after a dot.
    pub(crate) fn keyword_before(&self, i: usize) -> bool {
        !self.alnum_before(i)
            && (self.bytes[..i.min(self.len)].ends_with(b"...") || !self.after_dot(i))
    }

    /// `(?![$_[:alnum:]])(?:(?=\.\.\.)|(?!\.))`, the guard after a keyword.
    pub(crate) fn keyword_after(&self, i: usize) -> bool {
        !self.alnum_at(i) && (self.starts(i, "...") || self.byte(i) != b'.')
    }

    fn after_dot(&self, i: usize) -> bool {
        i > 0 && self.bytes.get(i - 1) == Some(&b'.')
    }

    /// A keyword between the grammar's guards: its end.
    ///
    /// Most calls fail on the first byte.
    /// That byte is therefore checked alone first.
    pub(crate) fn keyword(&self, p: usize, word: &str) -> Option<usize> {
        let word = word.as_bytes();
        let e = p + word.len();
        if self.bytes.get(p) != word.first()
            || e > self.len
            || self
                .bytes
                .get(p..e)
                .is_none_or(|text| !crate::bytes::same(text, word))
        {
            return None;
        }
        (self.keyword_before(p) && self.keyword_after(e)).then_some(e)
    }

    /// The first of `words` that is a keyword at `p`: its end.
    pub(crate) fn keyword_of(&self, p: usize, words: &[&str]) -> Option<usize> {
        words.iter().find_map(|word| self.keyword(p, word))
    }

    /// The source from `a` to `b`, or nothing when that is not a slice of
    /// whole characters.
    pub(crate) fn text(&self, a: usize, b: usize) -> &'a str {
        self.src.get(a..b).unwrap_or("")
    }

    /// `^\s*$`: a blank line, at its start.
    pub(crate) fn blank_line_at(&self, p: usize) -> bool {
        Self::bol(p) && self.eol(self.ws(0))
    }

    /// Whether the text before `p` ends in what `before` accepts.
    /// That is one of its bytes or operators, or one of its words as a whole
    /// word, and none of what it refuses.
    /// The grammar spells this lookbehind, with its own sets, before an object
    /// literal, a cast, a JSX tag and a regular expression.
    pub(crate) fn operator_before(&self, p: usize, before: &Before) -> bool {
        let Some(head) = self.bytes.get(..p) else {
            return false;
        };
        if before
            .never
            .iter()
            .any(|text| crate::bytes::ends_with(head, text.as_bytes()))
        {
            return false;
        }
        head.last().is_some_and(|&b| before.bytes.has(b))
            || before
                .operators
                .iter()
                .any(|text| crate::bytes::ends_with(head, text.as_bytes()))
            || before
                .words
                .iter()
                .any(|word| self.word_before_text(p, word))
    }

    pub(crate) fn word_before_text(&self, p: usize, word: &str) -> bool {
        let Some(head) = self.bytes.get(..p) else {
            return false;
        };
        if !crate::bytes::ends_with(head, word.as_bytes()) {
            return false;
        }
        let start = p - word.len();
        start == 0
            || self
                .prev_char(start)
                .is_some_and(|c| !(c == '.' || is_alnum_char(c)))
    }
}
