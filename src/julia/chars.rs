//! The grammar's character classes, for a character or for the bytes of a
//! line.
//! The tables in `classes` are measured on Shiki.

use super::classes;
use crate::classes::{is_space, is_word};
use crate::utf8::{char_at, char_start};

/// Whether `c` is in `table`, whose ASCII characters are the bits of
/// `ascii` and whose other characters are sorted inclusive ranges.
fn contains(ascii: u128, table: &[(u32, u32)], c: char) -> bool {
    let c = u32::from(c);
    if c < 0x80 {
        return (ascii >> c) & 1 == 1;
    }
    table
        .binary_search_by(|&(lo, hi)| {
            if hi < c {
                std::cmp::Ordering::Less
            } else if lo > c {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        })
        .is_ok()
}

/// Whether an identifier can start with `c`.
pub(super) fn is_ident_start(c: char) -> bool {
    contains(classes::IDENT_START_ASCII, classes::IDENT_START, c)
}

/// Whether an identifier can go on with `c`.
pub(super) fn is_ident_rest(c: char) -> bool {
    contains(classes::IDENT_REST_ASCII, classes::IDENT_REST, c)
}

/// Whether `c` is in `[[\p{S}\p{P}]&&[^@\s]]`, the characters of a macro
/// named by symbols, such as `@!`.
pub(super) fn is_macro_operator(c: char) -> bool {
    contains(classes::MACRO_OPERATOR_ASCII, classes::MACRO_OPERATOR, c)
}

/// Whether `c` is one of the arrows `keyword.operator.arrow` lists.
/// None is ASCII.
pub(super) fn is_arrow(c: char) -> bool {
    contains(0, classes::ARROW, c)
}

/// Whether `c` is one of the relations `keyword.operator.relation` lists.
/// None is ASCII.
pub(super) fn is_relation(c: char) -> bool {
    contains(0, classes::RELATION, c)
}

/// Whether `c` is in the first class of `keyword.operator.arithmetic`, which
/// holds `-`, `*`, `+` and `|`.
pub(super) fn is_plus(c: char) -> bool {
    contains(classes::PLUS_ASCII, classes::PLUS, c)
}

/// Whether `c` is in the second class of `keyword.operator.arithmetic`,
/// which holds `%`, `&`, `\` and `^`.
pub(super) fn is_times(c: char) -> bool {
    contains(classes::TIMES_ASCII, classes::TIMES, c)
}

/// The end of the character at `p` of `s` when an interpolated name can
/// start with it: an identifier's first character other than `$`.
pub(super) fn interpolated_start(s: &[u8], p: usize) -> Option<usize> {
    let (c, width) = char_at(s, p)?;
    (c != '$' && is_ident_start(c)).then_some(p + width)
}

/// The start of the run of `[\s\w]` characters that ends at `to`, going back
/// no further than `from`.
pub(super) fn space_or_word_start(s: &[u8], from: usize, mut to: usize) -> usize {
    while to > from {
        let start = char_start(s, to - 1).max(from);
        match char_at(s, start) {
            Some((c, _)) if is_space(c) || is_word(c) => to = start,
            _ => break,
        }
    }
    to
}
