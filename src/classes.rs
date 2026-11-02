//! The regex classes `\d`, `\w` and `\s` for a character, as Shiki's
//! regex engine reads them.
//!
//! Every grammar Shiki runs shares one engine, so every lexer shares these.
//! ASCII is decided here; the rest comes from a table measured on Shiki.

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

/// The regex `\d` for a character, from the same measured table.
pub(crate) fn is_digit(c: char) -> bool {
    if c.is_ascii() {
        c.is_ascii_digit()
    } else {
        table::is_digit(c)
    }
}
