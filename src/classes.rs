//! The regex classes `\d`, `\w`, `\s`, `[[:alpha:]]` and `\p{upper}` for a
//! character, as Shiki's regex engine reads them.
//!
//! Every grammar Shiki runs shares one engine, so every lexer shares these.
//! ASCII is decided here; the rest comes from tables measured on Shiki.

mod table;

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

/// The regex `\s` for an ASCII byte.
///
/// Unlike [`u8::is_ascii_whitespace`] it includes the vertical tab, 0x0b, as
/// Oniguruma does.
#[inline]
pub(crate) fn is_ascii_space(b: u8) -> bool {
    matches!(b, b' ' | b'\t' | b'\n' | 0x0b | 0x0c | b'\r')
}
