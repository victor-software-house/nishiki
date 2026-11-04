//! The case folds that `Onig`'s `(?i)` applies to characters above ASCII whose
//! fold is ASCII letters, for the hand lexers that match keywords without
//! regard to case.
//!
//! A lexer matches a literal with [`literal_end`], whose ASCII loop inlines
//! into the caller and whose cold half, [`literal_at`], reads a character
//! above ASCII. It lowers a word for a list lookup with [`lower_into`]. Each
//! takes the text slice its caller bounds, so a character that runs past the
//! bound does not match.

use crate::utf8::char_at;

/// The ASCII letters that `c`, a character above ASCII, matches under `(?i)`:
/// `s` for `ſ`, `k` for the Kelvin sign, `ss` for `ß` and `ẞ`, and the letters
/// of each Latin ligature from `ﬀ` to `ﬆ`; `None` for any other character.
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
/// `#[cold]`: a caller compares ASCII bytes itself and comes here only at a
/// byte above ASCII, so its own loop stays small enough to inline.
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
/// the letters it folds to, and the length written; `None` when a character
/// above ASCII folds to no ASCII letters or `buf` is too short.
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
}
