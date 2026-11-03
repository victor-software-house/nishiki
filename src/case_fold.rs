//! The case folds that `Onig`'s `(?i)` applies to characters above ASCII whose
//! fold is ASCII letters, for the hand lexers that match keywords without
//! regard to case.

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
}
