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
pub(crate) const fn is_continuation(b: u8) -> bool {
    b & 0xc0 == 0x80
}

/// The start of the character that holds byte `i` of `s`: `i` itself unless
/// it continues a character, and never before 0.
pub(crate) fn char_start(s: &[u8], mut i: usize) -> usize {
    while i > 0 && s.get(i).is_some_and(|&b| is_continuation(b)) {
        i -= 1;
    }
    i
}

/// The character that starts at byte `i` of `s`, and its width; `None` past
/// the end and where no whole character starts.
///
/// An ASCII byte is its own character, so only a multibyte one is validated.
pub(crate) fn char_at(s: &[u8], i: usize) -> Option<(char, usize)> {
    let lead = *s.get(i)?;
    if lead.is_ascii() {
        return Some((char::from(lead), 1));
    }
    let width = char_len(lead);
    let c = std::str::from_utf8(s.get(i..i + width)?)
        .ok()?
        .chars()
        .next()?;
    Some((c, width))
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
}
