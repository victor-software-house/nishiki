//! The regex classes `\d`, `\w`, `\s`, `[[:alpha:]]`, `\p{upper}`, `\p{L}`,
//! `\p{N}` and `\p{M}` for a character, as Shiki's regex engine reads them.
//!
//! Every grammar Shiki runs shares one engine, so every lexer shares these.
//! ASCII is decided here; the rest comes from tables measured on Shiki.

mod table;

use crate::utf8::{char_at, char_start};

/// The regex `\s` for a character: the table measured on Shiki.
pub(crate) fn is_space(c: char) -> bool {
    if c.is_ascii() {
        matches!(c, ' ' | '\t' | '\n' | '\u{b}' | '\u{c}' | '\r')
    } else {
        table::is_space(c)
    }
}

/// The regex `\w` for a character, from the same measured table.
pub(crate) fn is_word(c: char) -> bool {
    if c.is_ascii() {
        c.is_ascii_alphanumeric() || c == '_'
    } else {
        table::is_word(c)
    }
}

/// The regex `[[:alpha:]]` for a character, from the same measured table.
pub(crate) fn is_alpha(c: char) -> bool {
    if c.is_ascii() {
        c.is_ascii_alphabetic()
    } else {
        table::is_alpha(c)
    }
}

/// The regex `\d` for a character, from the same measured table.
pub(crate) fn is_digit(c: char) -> bool {
    if c.is_ascii() {
        c.is_ascii_digit()
    } else {
        table::is_digit(c)
    }
}

/// The regex `\p{upper}` for a character, from the same measured tables.
pub(crate) fn is_upper(c: char) -> bool {
    if c.is_ascii() {
        c.is_ascii_uppercase()
    } else {
        table::is_upper(c)
    }
}

/// The regex `\p{alnum}` for a character: `[[:alpha:]]` or `\d`.
pub(crate) fn is_alnum(c: char) -> bool {
    is_alpha(c) || is_digit(c)
}

/// The regex `\w` for a byte: an ASCII letter, a digit or `_`, and false for
/// every byte of a multibyte character.
///
/// One load from a table, which measured faster than the range tests of
/// [`u8::is_ascii_alphanumeric`] in the rule engine's matchers.
#[inline]
pub(crate) fn is_ascii_word(b: u8) -> bool {
    WORD_BYTES[usize::from(b)]
}

/// Which bytes are `\w`: the ASCII digits, upper-case letters, `_` and
/// lower-case letters.
const WORD_BYTES: [bool; 256] = {
    let mut table = [false; 256];
    let mut i = 0;
    while i < table.len() {
        table[i] = matches!(i, 0x30..=0x39 | 0x41..=0x5a | 0x5f | 0x61..=0x7a);
        i += 1;
    }
    table
};

/// The regex `\p{L}` for a character: a letter.
pub(crate) fn is_letter(c: char) -> bool {
    if c.is_ascii() {
        c.is_ascii_alphabetic()
    } else {
        table::is_letter(c)
    }
}

/// The regex `\p{N}` for a character: a number.
pub(crate) fn is_number(c: char) -> bool {
    if c.is_ascii() {
        c.is_ascii_digit()
    } else {
        table::is_number(c)
    }
}

/// The regex `\p{M}` for a character: a combining mark, which no ASCII
/// character is.
pub(crate) fn is_mark(c: char) -> bool {
    !c.is_ascii() && table::is_mark(c)
}

/// The regex `\s` for an ASCII byte.
///
/// Unlike [`u8::is_ascii_whitespace`] it includes the vertical tab, 0x0b, as
/// Oniguruma does.
#[inline]
pub(crate) fn is_ascii_space(b: u8) -> bool {
    matches!(b, b' ' | b'\t' | b'\n' | 0x0b | 0x0c | b'\r')
}

/// The width of the character at byte `i` of `s` when `class` holds for it
/// and `ascii` for its byte if it is ASCII; 0 otherwise, past the end too.
#[inline]
fn width_if(s: &[u8], i: usize, ascii: fn(u8) -> bool, class: fn(char) -> bool) -> usize {
    match s.get(i) {
        None => 0,
        Some(&b) if b < 0x80 => usize::from(ascii(b)),
        Some(_) => char_at(s, i)
            .filter(|&(c, _)| class(c))
            .map_or(0, |(_, width)| width),
    }
}

/// The width of the `\s` character at byte `i` of `s`, 0 when none starts
/// there. Past the end is 0: a lexer that counts the line feed after the line
/// as white space asks for it itself.
#[inline]
pub(crate) fn space_width(s: &[u8], i: usize) -> usize {
    width_if(s, i, is_ascii_space, is_space)
}

/// The width of the `\w` character at byte `i` of `s`, 0 when none starts
/// there.
#[inline]
pub(crate) fn word_width(s: &[u8], i: usize) -> usize {
    width_if(s, i, is_ascii_word, is_word)
}

/// The width of the `\d` character at byte `i` of `s`, 0 when none starts
/// there.
#[inline]
pub(crate) fn digit_width(s: &[u8], i: usize) -> usize {
    width_if(s, i, |b| b.is_ascii_digit(), is_digit)
}

/// Whether the character that ends at byte `i` of `s` is a `\w`.
#[inline]
pub(crate) fn word_before(s: &[u8], i: usize) -> bool {
    i.checked_sub(1)
        .is_some_and(|last| word_width(s, char_start(s, last)) > 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn widths_follow_the_classes() {
        let s = "a_1 \u{3000}\u{663}\u{e9}".as_bytes();
        assert_eq!(word_width(s, 0), 1);
        assert_eq!(word_width(s, 1), 1);
        assert_eq!(digit_width(s, 2), 1);
        assert_eq!(space_width(s, 3), 1);
        assert_eq!(space_width(s, 4), 3);
        assert_eq!(digit_width(s, 7), 2);
        assert_eq!(word_width(s, 9), 2);
        assert_eq!(space_width(s, s.len()), 0);
        assert!(word_before(s, 1));
        assert!(!word_before(s, 4));
        assert!(word_before(s, s.len()));
        assert!(!word_before(s, 0));
    }
}
