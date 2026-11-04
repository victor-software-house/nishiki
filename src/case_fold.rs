//! The case folds that `Onig`'s `(?i)` applies to characters above ASCII whose
//! fold is ASCII letters.
//! The hand lexers use them to match keywords without regard to case.
//!
//! A lexer matches a literal with [`literal_end`].
//! Its ASCII loop inlines into the caller.
//! Its cold half, [`literal_at`], reads a character above ASCII.
//! A lexer lowers a word for a list lookup with [`lower_into`].
//! Each takes the text slice its caller bounds.
//! A character that runs past the bound therefore does not match.

use crate::utf8::char_at;

/// The ASCII letters that `c`, a character above ASCII, matches under `(?i)`:
/// `s` for `ſ`, `k` for the Kelvin sign, `ss` for `ß` and `ẞ`, and the letters
/// of each Latin ligature from `ﬀ` to `ﬆ`.
/// It is `None` for any other character.
pub(crate) fn letters(c: char) -> Option<&'static [u8]> {
    match c {
        'ſ' => Some(b"s"),
        '\u{212a}' => Some(b"k"),
        'ß' | '\u{1e9e}' => Some(b"ss"),
        '\u{fb00}' => Some(b"ff"),
        '\u{fb01}' => Some(b"fi"),
        '\u{fb02}' => Some(b"fl"),
        '\u{fb03}' => Some(b"ffi"),
        '\u{fb04}' => Some(b"ffl"),
        '\u{fb05}' | '\u{fb06}' => Some(b"st"),
        _ => None,
    }
}

/// The width of the character at `at` of `text` and how many bytes of `word`
/// it stands for, when its fold starts `word` without regard to ASCII case.
///
/// It is `#[cold]`.
/// A caller compares ASCII bytes itself and comes here only at a byte above
/// ASCII.
/// The caller's own loop then stays small enough to inline.
#[cold]
pub(crate) fn literal_at(text: &[u8], at: usize, word: &[u8]) -> Option<(usize, usize)> {
    let (c, width) = char_at(text, at)?;
    let folded = letters(c)?;
    word.get(..folded.len())?
        .eq_ignore_ascii_case(folded)
        .then_some((width, folded.len()))
}

/// The end of `word` at `at` of `text`, without regard to ASCII case, a
/// character above ASCII standing for the letters it folds to.
///
/// `#[inline]`, so that the ASCII loop sits in each caller and only
/// [`literal_at`] stays out of line.
#[inline]
pub(crate) fn literal_end(text: &[u8], at: usize, word: &[u8]) -> Option<usize> {
    let mut at = at;
    let mut rest = word;
    while let [expected, tail @ ..] = rest {
        let &byte = text.get(at)?;
        if byte.is_ascii() {
            if !byte.eq_ignore_ascii_case(expected) {
                return None;
            }
            at += 1;
            rest = tail;
        } else {
            let (width, read) = literal_at(text, at, rest)?;
            at += width;
            rest = rest.get(read..)?;
        }
    }
    Some(at)
}

/// `text` lowered into `buf` as `(?i)` reads it, a character above ASCII as
/// the letters it folds to, and the length written.
/// It is `None` when a character above ASCII folds to no ASCII letters, or
/// when `buf` is too short.
#[inline]
pub(crate) fn lower_into(text: &[u8], buf: &mut [u8]) -> Option<usize> {
    let mut len = 0;
    let mut at = 0;
    while let Some(&byte) = text.get(at) {
        if byte.is_ascii() {
            *buf.get_mut(len)? = byte.to_ascii_lowercase();
            len += 1;
            at += 1;
        } else {
            let (c, width) = char_at(text, at)?;
            let folded = letters(c)?;
            buf.get_mut(len..len + folded.len())?
                .copy_from_slice(folded);
            len += folded.len();
            at += width;
        }
    }
    Some(len)
}

/// The end of the text at `at` of `text` that `word` matches as a
/// back-reference does under `(?i)`.
/// `word` is a text matched before.
///
/// - An ASCII letter matches in either case.
/// - A character above ASCII matches as the letters it folds to.
/// - Any other character matches itself.
///
/// So `ﬁlter` and `FILTER` match each other.
///
/// Nearly every end tag equals `word` but for ASCII case.
/// One slice comparison answers that case.
/// Only text or a word with a byte above ASCII is read character by
/// character, in [`folded_reference_end`].
#[inline]
pub(crate) fn reference_end(text: &[u8], at: usize, word: &[u8]) -> Option<usize> {
    let same = text.get(at..at + word.len());
    if same.is_some_and(|same| same.eq_ignore_ascii_case(word)) {
        return Some(at + word.len());
    }
    if word.is_ascii() && same.is_none_or(<[u8]>::is_ascii) {
        return None;
    }
    folded_reference_end(text, at, word)
}

/// [`reference_end`] read character by character, through the folds.
#[cold]
fn folded_reference_end(text: &[u8], at: usize, word: &[u8]) -> Option<usize> {
    let mut have = Folded {
        text: text.get(at..)?,
        at: 0,
        pending: &[],
    };
    let want = Folded {
        text: word,
        at: 0,
        pending: &[],
    };
    for byte in want {
        if have.next()? != byte {
            return None;
        }
    }
    have.pending.is_empty().then_some(at + have.at)
}

/// The bytes a text compares by under `(?i)`, character by character: an
/// ASCII letter lowered, the letters a character above ASCII folds to, and
/// the bytes of any other character.
struct Folded<'t> {
    text: &'t [u8],
    /// Where the next character starts.
    at: usize,
    /// The rest of the bytes of the last character read.
    pending: &'t [u8],
}

impl Iterator for Folded<'_> {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        if let [first, rest @ ..] = self.pending {
            self.pending = rest;
            return Some(*first);
        }
        let &byte = self.text.get(self.at)?;
        if byte.is_ascii() {
            self.at += 1;
            return Some(byte.to_ascii_lowercase());
        }
        let (c, width) = char_at(self.text, self.at)?;
        let own = self.text.get(self.at..self.at + width)?;
        self.at += width;
        let [first, rest @ ..] = letters(c).unwrap_or(own) else {
            return None;
        };
        self.pending = rest;
        Some(*first)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_fold_stands_for_its_letters_in_either_case() {
        let text = "\u{df}\u{fb03}\u{17f}".as_bytes();
        assert_eq!(literal_at(text, 0, b"SSe"), Some((2, 2)));
        assert_eq!(literal_at(text, 2, b"ffix"), Some((3, 3)));
        assert_eq!(literal_at(text, 5, b"s"), Some((2, 1)));
        assert_eq!(literal_at(text, 0, b"s"), None, "half of `ss` is no match");
        assert_eq!(literal_at(text, 2, b"ffl"), None);
        assert_eq!(literal_at("\u{e9}".as_bytes(), 0, b"e"), None);
        assert_eq!(letters('a'), None, "ASCII is the caller's");
    }

    #[test]
    fn a_literal_ends_after_the_characters_that_fold_to_it() {
        let text = "X\u{fb01}eld\u{df}e\u{17f}".as_bytes();
        assert_eq!(literal_end(text, 1, b"field"), Some(7));
        assert_eq!(literal_end(text, 1, b"FIELDSSES"), Some(text.len()));
        assert_eq!(literal_end(text, 1, b"fieldse"), None, "half of `ss`");
        assert_eq!(literal_end(text, 0, b"xfi"), Some(4));
        assert_eq!(literal_end(text, 0, b"xfie"), Some(5));
        assert_eq!(literal_end(text, 1, b"f"), None, "half of `fi`");
        assert_eq!(literal_end(text, 1, b""), Some(1));
        assert_eq!(literal_end(b"ab", 0, b"abc"), None, "past the end");
    }

    #[test]
    fn a_word_lowers_each_fold_to_its_letters() {
        let mut buf = [0; 8];
        let text = "A\u{fb03}X\u{212a}".as_bytes();
        assert_eq!(lower_into(text, &mut buf), Some(6));
        assert_eq!(&buf[..6], b"affixk");
        assert_eq!(lower_into("\u{e9}".as_bytes(), &mut buf), None);
        assert_eq!(lower_into(b"abcdefghi", &mut buf), None, "too long");
        assert_eq!(lower_into(b"", &mut buf), Some(0));
    }

    #[test]
    fn a_reference_matches_a_text_with_the_same_folds() {
        let fi = "\u{fb01}lter>".as_bytes();
        assert_eq!(
            reference_end(b"</FILTER>", 2, "\u{fb01}lter".as_bytes()),
            Some(8)
        );
        assert_eq!(reference_end(fi, 0, b"filter"), Some(7));
        assert_eq!(reference_end(fi, 0, b"f"), None, "half of `fi`");
        assert_eq!(reference_end(fi, 0, b"fil"), Some(4));
        let e = "\u{e9}t\u{e9}".as_bytes();
        assert_eq!(reference_end(e, 0, e), Some(e.len()));
        assert_eq!(reference_end(e, 0, b"ete"), None, "no fold to ASCII");
        assert_eq!(reference_end(b"ab", 0, b"abc"), None, "past the end");
    }
}
