//! The sorted word lists of [`super::types`] as integer keys, so that a
//! lookup compares integers instead of calling `memcmp` at every step of its
//! binary search.
//!
//! A key is a word's bytes packed big-endian into three `u64`s and padded
//! with zeros. Packed that way, keys sort as their words do, because no word
//! holds a zero byte. The tables are built from the generated lists at
//! compile time, which fails when a word is longer than a key or a list is
//! out of order.

use super::types::{BEHIND_4D9B, NOT_0C8F, NOT_C3E4, PRIMITIVE_TYPES, STDINT_TYPES, TYPE_WORDS};

/// A word packed into three big-endian `u64`s.
type Key = [u64; 3];

/// The longest word a key holds.
const KEY_BYTES: usize = 24;

/// A sorted word list as keys.
pub(super) struct Words(&'static [Key]);

impl Words {
    /// Whether `word` is in the list.
    pub(super) fn contains(&self, word: &[u8]) -> bool {
        word.len() <= KEY_BYTES && self.0.binary_search(&key(word)).is_ok()
    }
}

/// The key of `word`, which is at most [`KEY_BYTES`] long.
const fn key(word: &[u8]) -> Key {
    let mut bytes = [0; KEY_BYTES];
    let mut i = 0;
    while i < word.len() {
        bytes[i] = word[i];
        i += 1;
    }
    let mut out = [0; 3];
    let mut k = 0;
    while k < 3 {
        let mut chunk = [0; 8];
        let mut j = 0;
        while j < 8 {
            chunk[j] = bytes[8 * k + j];
            j += 1;
        }
        out[k] = u64::from_be_bytes(chunk);
        k += 1;
    }
    out
}

/// Whether key `a` sorts before key `b`.
const fn before(a: &Key, b: &Key) -> bool {
    let mut i = 0;
    while i < 3 {
        if a[i] != b[i] {
            return a[i] < b[i];
        }
        i += 1;
    }
    false
}

/// The keys of `words`, which must number `N`, each fit in a key, and be
/// sorted without repeats.
const fn keys<const N: usize>(words: &[&[u8]]) -> [Key; N] {
    assert!(words.len() == N, "the list has N words");
    let mut out = [[0; 3]; N];
    let mut i = 0;
    while i < N {
        assert!(words[i].len() <= KEY_BYTES, "a listed word fits in a key");
        out[i] = key(words[i]);
        assert!(i == 0 || before(&out[i - 1], &out[i]), "the list is sorted");
        i += 1;
    }
    out
}

/// [`PRIMITIVE_TYPES`] as keys.
pub(super) const PRIMITIVE_KEYS: Words = Words(&keys::<{ PRIMITIVE_TYPES.len() }>(PRIMITIVE_TYPES));
/// [`STDINT_TYPES`] as keys.
pub(super) const STDINT_KEYS: Words = Words(&keys::<{ STDINT_TYPES.len() }>(STDINT_TYPES));
/// [`TYPE_WORDS`] as keys.
pub(super) const TYPE_KEYS: Words = Words(&keys::<{ TYPE_WORDS.len() }>(TYPE_WORDS));
/// [`BEHIND_4D9B`] as keys.
pub(super) const BEHIND_KEYS: Words = Words(&keys::<{ BEHIND_4D9B.len() }>(BEHIND_4D9B));
/// [`NOT_0C8F`] as keys.
pub(super) const NOT_0C8F_KEYS: Words = Words(&keys::<{ NOT_0C8F.len() }>(NOT_0C8F));
/// [`NOT_C3E4`] as keys.
pub(super) const NOT_C3E4_KEYS: Words = Words(&keys::<{ NOT_C3E4.len() }>(NOT_C3E4));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keys_find_exactly_the_listed_words() {
        for word in TYPE_WORDS {
            assert!(TYPE_KEYS.contains(word));
        }
        for word in [
            &b""[..],
            b"in",
            b"intx",
            b"int8",
            b"uint_least8_t_",
            b"\xc3\xa9",
        ] {
            assert_eq!(
                TYPE_KEYS.contains(word),
                TYPE_WORDS.binary_search(&word).is_ok()
            );
        }
        assert!(!NOT_0C8F_KEYS.contains(&[b'a'; KEY_BYTES + 1]));
    }
}
