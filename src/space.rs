//! The Unicode spaces that `\s` reads in the grammars Shiki runs.

/// The length in bytes of the Unicode space that starts `bytes`, or 0 when
/// none does.
///
/// Shiki's regex engine reads `\s` as the Unicode `White_Space` characters.
/// They are the ASCII blanks and these:
/// - U+0085, U+00A0 and U+1680;
/// - U+2000 to U+200A;
/// - U+2028, U+2029, U+202F, U+205F and U+3000.
///
/// U+FEFF and U+200B are not among them.
pub(crate) fn len(bytes: &[u8]) -> usize {
    match bytes {
        [0xc2, 0x85 | 0xa0, ..] => 2,
        [0xe1, 0x9a, 0x80, ..]
        | [0xe2, 0x80, 0x80..=0x8a | 0xa8 | 0xa9 | 0xaf, ..]
        | [0xe2, 0x81, 0x9f, ..]
        | [0xe3, 0x80, 0x80, ..] => 3,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::len;

    #[test]
    fn it_matches_the_unicode_white_space_characters() {
        for c in (0x80..0x11_0000).filter_map(char::from_u32) {
            let mut buffer = [0; 4];
            let bytes = c.encode_utf8(&mut buffer).as_bytes();
            let ascii_blank = c.is_ascii();
            let expected = if c.is_whitespace() && !ascii_blank {
                c.len_utf8()
            } else {
                0
            };
            assert_eq!(len(bytes), expected, "{c:?}");
        }
    }

    /// This byte match and the measured `\s` table in `classes` spell the
    /// same set; the match stays because the lexers call it per byte.
    #[cfg(feature = "_classes")]
    #[test]
    fn it_agrees_with_the_measured_space_table() {
        for c in (0x80..0x11_0000).filter_map(char::from_u32) {
            let mut buffer = [0; 4];
            let bytes = c.encode_utf8(&mut buffer).as_bytes();
            assert_eq!(len(bytes) > 0, crate::classes::is_space(c), "{c:?}");
        }
    }
}
