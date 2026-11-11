//! UTF-8 helpers the lexers share.

/// How many bytes the character that starts with `lead` takes.
///
/// A continuation byte counts as one, so a scan over bytes that are not valid
/// UTF-8 still advances.
#[inline]
pub(crate) fn char_len(lead: u8) -> usize {
    match lead {
        0xf0.. => 4,
        0xe0.. => 3,
        0xc0.. => 2,
        _ => 1,
    }
}

/// Whether `b` continues a multibyte character rather than starting one.
#[inline]
#[cfg_attr(
    all(not(feature = "_hand-lexers"), not(test)),
    expect(
        dead_code,
        reason = "only hand-written lexers and the engine tests use it"
    )
)]
pub(crate) const fn is_continuation(b: u8) -> bool {
    b & 0xc0 == 0x80
}

/// The start of the character that holds byte `i` of `s`: `i` itself unless
/// it continues a character, and never before 0.
#[cfg_attr(
    all(not(feature = "_hand-lexers"), not(test)),
    expect(
        dead_code,
        reason = "only hand-written lexers and the engine tests use it"
    )
)]
pub(crate) fn char_start(s: &[u8], mut i: usize) -> usize {
    while i > 0 && s.get(i).is_some_and(|&b| is_continuation(b)) {
        i -= 1;
    }
    i
}

/// The character that starts at byte `i` of `s`, and its width.
/// It is `None` past the end and where no whole character starts.
///
/// The decoder builds the character from the bits of its bytes.
/// It runs no separate validation pass.
/// It returns `None` for a byte that cannot lead a character, and when a
/// continuation byte is missing or out of place.
/// [`char::from_u32`] rejects a surrogate and a value past U+10FFFF.
/// The width check rejects an overlong form.
///
/// The tests check every character against [`str::from_utf8`].
///
/// The ASCII test is inline in the caller.
/// A multibyte character goes to [`multibyte_at`], which is out of line.
/// A caller that has already tested for ASCII calls [`multibyte_at`] directly.
#[inline]
#[cfg_attr(
    all(not(feature = "_hand-lexers"), not(test)),
    expect(
        dead_code,
        reason = "only hand-written lexers and the engine tests use it"
    )
)]
pub(crate) fn char_at(s: &[u8], i: usize) -> Option<(char, usize)> {
    let lead = *s.get(i)?;
    if lead.is_ascii() {
        return Some((char::from(lead), 1));
    }
    multibyte_at(s, i, lead)
}

/// [`char_at`] for the character at `i` whose `lead` is above ASCII.
///
/// It is `#[cold]`.
/// The caller's ASCII path then saves no registers for this call.
#[cold]
#[cfg_attr(
    all(not(feature = "_hand-lexers"), not(test)),
    expect(
        dead_code,
        reason = "only hand-written lexers and the engine tests use it"
    )
)]
pub(crate) fn multibyte_at(s: &[u8], i: usize, lead: u8) -> Option<(char, usize)> {
    let (width, bits) = match lead {
        0xc2..=0xdf => (2, lead & 0x1f),
        0xe0..=0xef => (3, lead & 0x0f),
        0xf0..=0xf4 => (4, lead & 0x07),
        _ => return None,
    };
    let mut code = u32::from(bits);
    for k in 1..width {
        let byte = *s.get(i + k)?;
        if !is_continuation(byte) {
            return None;
        }
        code = (code << 6) | u32::from(byte & 0x3f);
    }
    let c = char::from_u32(code)?;
    (c.len_utf8() == width).then_some((c, width))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn characters_are_found_from_any_of_their_bytes() {
        let s = "a\u{e9}\u{20ac}".as_bytes();
        assert_eq!(char_at(s, 0), Some(('a', 1)));
        assert_eq!(char_at(s, 1), Some(('\u{e9}', 2)));
        assert_eq!(char_at(s, 2), None);
        assert_eq!(char_at(s, 3), Some(('\u{20ac}', 3)));
        assert_eq!(char_at(s, 6), None);
        assert_eq!(char_start(s, 2), 1);
        assert_eq!(char_start(s, 5), 3);
        assert_eq!(char_start(s, 0), 0);
        assert!(is_continuation(s[2]));
        assert!(!is_continuation(s[3]));
    }

    #[test]
    fn char_at_reads_ascii_multibyte_and_continuation_leads() {
        assert_eq!(char_at(b"\x00", 0), Some(('\0', 1)));
        assert_eq!(char_at(b"z", 0), Some(('z', 1)));
        assert_eq!(char_at(b"\x7f", 0), Some(('\x7f', 1)));
        let s = "\u{17f}\u{212a}\u{1f600}".as_bytes();
        assert_eq!(char_at(s, 0), Some(('\u{17f}', 2)));
        assert_eq!(char_at(s, 2), Some(('\u{212a}', 3)));
        assert_eq!(char_at(s, 5), Some(('\u{1f600}', 4)));
        assert_eq!(char_at(s, 1), None, "a continuation byte starts nothing");
        assert_eq!(char_at(s, 6), None, "nor does a later one");
        assert_eq!(char_at(b"\xc3", 0), None, "a lead cut off at the end");
        assert_eq!(char_at(b"\xc3a", 0), None, "a lead before ASCII");
        assert_eq!(char_at(b"\xc0\xaf", 0), None, "an overlong form");
        assert_eq!(char_at(b"\xed\xa0\x80", 0), None, "a surrogate");
        assert_eq!(char_at(b"\xff", 0), None, "a byte no character starts with");
    }

    /// `char_at` reads every encoded character back at its width.
    /// It finds no character at a continuation byte.
    #[test]
    fn char_at_reads_every_character_as_str_chars_does() {
        let mut buffer = [0; 4];
        for code in 0..=0x10_ffff {
            let Some(c) = char::from_u32(code) else {
                continue;
            };
            let bytes = c.encode_utf8(&mut buffer).as_bytes();
            let expected = std::str::from_utf8(bytes)
                .ok()
                .and_then(|text| text.chars().next());
            assert_eq!(expected, Some(c));
            assert_eq!(char_at(bytes, 0), Some((c, bytes.len())), "{code:#x}");
            for i in 1..bytes.len() {
                assert_eq!(char_at(bytes, i), None, "{code:#x} at {i}");
            }
        }
    }

    /// The test covers every input of one or two bytes.
    /// `char_at` finds a character exactly where `str::from_utf8` finds one.
    #[test]
    fn char_at_agrees_with_from_utf8_on_every_short_input() {
        let reference = |s: &[u8]| {
            let width = s.first().map_or(0, |&lead| char_len(lead));
            s.get(..width)
                .and_then(|bytes| std::str::from_utf8(bytes).ok())
                .and_then(|text| text.chars().next())
                .map(|c| (c, width))
        };
        for first in 0..=u8::MAX {
            assert_eq!(char_at(&[first], 0), reference(&[first]), "{first:#x}");
            for second in 0..=u8::MAX {
                let s = [first, second];
                assert_eq!(char_at(&s, 0), reference(&s), "{first:#x} {second:#x}");
            }
        }
    }
}
