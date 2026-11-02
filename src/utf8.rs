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
