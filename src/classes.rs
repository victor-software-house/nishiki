//! The regex classes `\d`, `\w`, `\s`, `[[:alpha:]]`, `\p{upper}`, `\p{L}`,
//! `\p{N}` and `\p{M}` for a character, as Shiki's regex engine reads them.
//! Scala adds three unions of categories: `[\p{Lt}\p{Lu}]`,
//! `[\p{Lo}\p{Nl}\p{Ll}]` and `[\p{Sm}\p{So}]`.
//! Identifiers add `\p{Nl}`, `[\p{Mn}\p{Mc}]` and `\p{Pc}`.
//!
//! Every grammar Shiki runs shares one engine, so every lexer shares these.
//! ASCII is decided here; the rest comes from tables measured on Shiki.

mod table;

use crate::utf8::{char_start, multibyte_at};

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

/// The regex `[\p{Lt}\p{Lu}]` for a character: a title-case or upper-case
/// letter.
/// In ASCII, `A` to `Z`.
#[cfg(any(feature = "lang-scala", feature = "lang-haskell"))]
pub(crate) fn is_title_or_upper(c: char) -> bool {
    if c.is_ascii() {
        c.is_ascii_uppercase()
    } else {
        table::is_title_or_upper(c)
    }
}

/// The regex `[\p{Lo}\p{Nl}\p{Ll}]` for a character: a lower-case letter, an
/// other letter or a letter number.
/// In ASCII, `a` to `z`.
#[cfg(feature = "lang-scala")]
pub(crate) fn is_lower_or_other_letter(c: char) -> bool {
    if c.is_ascii() {
        c.is_ascii_lowercase()
    } else {
        table::is_lower_or_other_letter(c)
    }
}

/// The regex `[\p{Sm}\p{So}]` for a character: a math or other symbol.
/// In ASCII, `+`, `<`, `=`, `>`, `|` and `~`.
#[cfg(feature = "lang-scala")]
pub(crate) fn is_symbol(c: char) -> bool {
    if c.is_ascii() {
        matches!(c, '+' | '<' | '=' | '>' | '|' | '~')
    } else {
        table::is_symbol(c)
    }
}

/// The regex `\p{Ll}` for a character: a lower-case letter.
/// In ASCII, `a` to `z`, as measured on Shiki's engine.
#[cfg(feature = "lang-haskell")]
pub(crate) fn is_lower(c: char) -> bool {
    if c.is_ascii() {
        c.is_ascii_lowercase()
    } else {
        table::is_lower(c)
    }
}

/// The regex `\p{Lt}` for a character: a title-case letter.
/// ASCII has none.
#[cfg(feature = "lang-haskell")]
pub(crate) fn is_title(c: char) -> bool {
    !c.is_ascii() && table::is_title(c)
}

/// The regex `[\p{S}\p{P}]` for a character: a symbol or a punctuation
/// mark.
/// In ASCII, every printable character that is not a letter, a digit or a
/// space, as measured on Shiki's engine.
#[cfg(feature = "lang-haskell")]
pub(crate) fn is_symbol_or_punctuation(c: char) -> bool {
    if c.is_ascii() {
        c.is_ascii_punctuation()
    } else {
        table::is_symbol_or_punctuation(c)
    }
}

/// The regex `\p{Nl}` for a character: a letter number, such as the Roman
/// numeral U+2160.
/// ASCII has none.
#[cfg(feature = "_identifier-classes")]
pub(crate) fn is_letter_number(c: char) -> bool {
    !c.is_ascii() && table::is_letter_number(c)
}

/// The regex `[\p{Mn}\p{Mc}]` for a character: a combining mark that does
/// not enclose its base, such as U+0300.
/// ASCII has none.
#[cfg(feature = "_identifier-classes")]
pub(crate) fn is_nonenclosing_mark(c: char) -> bool {
    !c.is_ascii() && table::is_nonenclosing_mark(c)
}

/// The regex `\p{Pc}` for a character: a connector punctuation mark.
/// In ASCII, `_`.
#[cfg(feature = "_identifier-classes")]
pub(crate) fn is_connector(c: char) -> bool {
    if c.is_ascii() {
        c == '_'
    } else {
        table::is_connector(c)
    }
}

/// The regex `\s` for an ASCII byte.
///
/// Unlike [`u8::is_ascii_whitespace`] it includes the vertical tab, 0x0b, as
/// Oniguruma does.
#[inline]
pub(crate) fn is_ascii_space(b: u8) -> bool {
    matches!(b, b' ' | b'\t' | b'\n' | 0x0b | 0x0c | b'\r')
}

/// The width of the character at byte `i` of `s` when `class` holds for it.
/// For an ASCII character, `ascii` must hold for its byte instead.
/// The width is 0 otherwise, past the end too.
#[inline]
fn width_if(s: &[u8], i: usize, ascii: fn(u8) -> bool, class: fn(char) -> bool) -> usize {
    match s.get(i) {
        None => 0,
        Some(&b) if b < 0x80 => usize::from(ascii(b)),
        Some(&lead) => multibyte_at(s, i, lead)
            .filter(|&(c, _)| class(c))
            .map_or(0, |(_, width)| width),
    }
}

/// The width of the `\s` character at byte `i` of `s`, 0 when none starts
/// there. Past the end is 0: a lexer that counts the line feed after the line
/// as white space reads it itself.
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

    /// Each case is Oniguruma's answer in Shiki 4.6.0.
    /// U+20DD and U+0488 are enclosing marks, which `[\p{Mn}\p{Mc}]` leaves
    /// out.
    #[cfg(feature = "_identifier-classes")]
    #[test]
    fn identifier_classes_follow_oniguruma() {
        let cases = [
            ('a', false, false, false),
            ('_', false, false, true),
            ('0', false, false, false),
            ('\u{16ee}', true, false, false),
            ('\u{2160}', true, false, false),
            ('\u{3007}', true, false, false),
            ('\u{10140}', true, false, false),
            ('\u{300}', false, true, false),
            ('\u{903}', false, true, false),
            ('\u{20dd}', false, false, false),
            ('\u{488}', false, false, false),
            ('\u{203f}', false, false, true),
            ('\u{fe33}', false, false, true),
            ('\u{ff3f}', false, false, true),
            ('\u{e9}', false, false, false),
            ('\u{663}', false, false, false),
            ('\u{1f600}', false, false, false),
        ];
        for (c, letter_number, mark, connector) in cases {
            assert_eq!(is_letter_number(c), letter_number, "{c:?} \\p{{Nl}}");
            assert_eq!(is_nonenclosing_mark(c), mark, "{c:?} [\\p{{Mn}}\\p{{Mc}}]");
            assert_eq!(is_connector(c), connector, "{c:?} \\p{{Pc}}");
        }
    }
}
