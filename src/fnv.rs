//! A hash map with a fixed hasher, for word tables built from the crate's own
//! lists: its probes are the same in every process, so instruction counts of
//! lexing are too, where std's randomly seeded hasher moved them by about
//! 0.3% from run to run. Keys that come from the text being lexed keep std's
//! seeded hasher, which resists inputs crafted to collide.
//!
//! A theme's trie orders its children by the hash of the parts of the theme's
//! own selectors.
//! Text only looks parts up there, so it cannot add children that collide,
//! and the fixed hasher is safe for it too.

#[cfg(any(feature = "lang-c", feature = "lang-python"))]
use std::collections::HashMap;
#[cfg(any(feature = "lang-c", feature = "lang-python"))]
use std::hash::BuildHasherDefault;
use std::hash::Hasher;

/// A map keyed through [`Fnv`].
#[cfg(any(feature = "lang-c", feature = "lang-python"))]
pub(crate) type FnvMap<K, V> = HashMap<K, V, BuildHasherDefault<Fnv>>;

/// FNV-1a over the bytes written.
#[derive(Default)]
pub(crate) struct Fnv(u64);

impl Hasher for Fnv {
    fn finish(&self) -> u64 {
        self.0
    }

    fn write(&mut self, bytes: &[u8]) {
        let mut hash = if self.0 == 0 {
            0xcbf2_9ce4_8422_2325
        } else {
            self.0
        };
        for &byte in bytes {
            hash = (hash ^ u64::from(byte)).wrapping_mul(0x0100_0000_01b3);
        }
        self.0 = hash;
    }
}
