//! A theme trie's children: a table from a parent node and a scope part to
//! the child node.
//!
//! It uses open addressing, and no entry sits more than [`MAX_PROBE`] slots
//! from its home slot.
//! A lookup therefore reads at most that many slots, whatever the theme.
//!
//! The table starts with fixed hash keys, so a theme's lookups run the same
//! instructions in every process.
//!
//! A theme can come from an untrusted source, and with known keys it could
//! pick parts whose slots crowd together.
//! When an entry would sit past the bound, the table is built again with
//! random keys, which leave no way to pick such parts.

use std::hash::{BuildHasher, RandomState};

use super::Node;
use crate::bytes;

/// How far from its home slot an entry may sit.
const MAX_PROBE: usize = 32;

/// The child of an empty slot.
const EMPTY: usize = usize::MAX;

/// The keys [`Keys::hash`] multiplies by.
#[derive(Clone, Copy, Debug)]
struct Keys {
    start: u64,
    word: u64,
    end: u64,
}

impl Keys {
    /// The keys every table starts with: hexadecimal digits of pi, the
    /// multipliers made odd.
    const FIXED: Keys = Keys {
        start: 0x243f_6a88_85a3_08d3,
        word: 0x1319_8a2e_0370_7345 | 1,
        end: 0xa409_3822_299f_31d1 | 1,
    };

    /// Keys from std's random hasher state; the multipliers are odd.
    fn random() -> Keys {
        let state = RandomState::new();
        Keys {
            start: state.hash_one(0_u8),
            word: state.hash_one(1_u8) | 1,
            end: state.hash_one(2_u8) | 1,
        }
    }

    /// The hash of `part` under `parent`.
    ///
    /// It folds the part in eight bytes at a time: each step multiplies by a
    /// key into 128 bits and XORs the halves.
    ///
    /// A whole word loads as one integer.
    /// The last few bytes go in through a loop, because a copy into a buffer
    /// called libc's `memcpy` for every part.
    /// The length goes in last, so a part and the same part with zero bytes
    /// after it differ.
    fn hash(self, parent: usize, part: &str) -> u64 {
        let mut hash = self.start ^ u64::try_from(parent).unwrap_or(u64::MAX);
        let (words, tail) = part.as_bytes().as_chunks::<8>();
        for &word in words {
            hash = fold(hash ^ u64::from_le_bytes(word), self.word);
        }
        if !tail.is_empty() {
            let word = tail
                .iter()
                .rev()
                .fold(0, |word, &byte| (word << 8) | u64::from(byte));
            hash = fold(hash ^ word, self.word);
        }
        fold(
            hash ^ u64::try_from(part.len()).unwrap_or(u64::MAX),
            self.end,
        )
    }
}

/// `value` times `key` as a 128-bit product, then the XOR of its two halves.
fn fold(value: u64, key: u64) -> u64 {
    let product = u128::from(value) * u128::from(key);
    let high = u64::try_from(product >> 64).unwrap_or(0);
    let low = u64::try_from(product & u128::from(u64::MAX)).unwrap_or(0);
    high ^ low
}

/// One slot: the hash of its parent and part, and the two nodes.
#[derive(Clone, Copy, Debug)]
struct Slot {
    hash: u64,
    parent: usize,
    child: usize,
}

impl Slot {
    const EMPTY: Slot = Slot {
        hash: 0,
        parent: 0,
        child: EMPTY,
    };
}

#[derive(Clone, Debug)]
pub(super) struct Children {
    /// A power of two of slots.
    slots: Box<[Slot]>,
    keys: Keys,
}

impl Children {
    /// The table of `entries`, each a parent, a part and the child.
    ///
    /// The entries come in the order of their children, so the same theme
    /// lays out the same table in every process.
    pub(super) fn build(entries: &[(usize, &str, usize)]) -> Children {
        let mut size = entries.len().saturating_mul(2).next_power_of_two().max(8);
        if let Some(table) = Children::fill(entries, Keys::FIXED, size) {
            return table;
        }
        loop {
            if let Some(table) = Children::fill(entries, Keys::random(), size) {
                return table;
            }
            size = size.saturating_mul(2);
        }
    }

    /// The table of `entries` under `keys` with `size` slots, if every entry
    /// sits within [`MAX_PROBE`] slots of its home.
    fn fill(entries: &[(usize, &str, usize)], keys: Keys, size: usize) -> Option<Children> {
        let mut slots = vec![Slot::EMPTY; size].into_boxed_slice();
        let mask = size - 1;
        for &(parent, part, child) in entries {
            let hash = keys.hash(parent, part);
            let home = home(hash, mask);
            let at = (0..MAX_PROBE)
                .map(|probe| (home + probe) & mask)
                .find(|&at| slots.get(at).is_some_and(|slot| slot.child == EMPTY))?;
            *slots.get_mut(at)? = Slot {
                hash,
                parent,
                child,
            };
        }
        Some(Children { slots, keys })
    }

    /// The child of `parent` for `part`, if the trie has one.
    ///
    /// It checks the parent and compares the part byte by byte, since two
    /// entries can share a hash.
    ///
    /// At one codegen unit, the map it replaced inlined into `Trie::apply`.
    #[expect(
        clippy::inline_always,
        reason = "Trie::find calls it once per scope part"
    )]
    #[inline(always)]
    pub(super) fn find(&self, nodes: &[Node], parent: usize, part: &str) -> Option<usize> {
        let hash = self.keys.hash(parent, part);
        let mask = self.slots.len() - 1;
        let home = home(hash, mask);
        for probe in 0..MAX_PROBE {
            let slot = self.slots.get((home + probe) & mask)?;
            if slot.child == EMPTY {
                return None;
            }
            if slot.hash == hash
                && slot.parent == parent
                && nodes
                    .get(slot.child)
                    .is_some_and(|node| bytes::same(node.part.as_bytes(), part.as_bytes()))
            {
                return Some(slot.child);
            }
        }
        None
    }
}

/// The home slot of `hash` in a table of `mask + 1` slots.
fn home(hash: u64, mask: usize) -> usize {
    usize::try_from(hash).unwrap_or(0) & mask
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_hashes_a_part_by_its_bytes_its_length_and_its_parent() {
        let keys = Keys::FIXED;
        assert_eq!(keys.hash(2, "string"), keys.hash(2, "string"));
        assert_ne!(keys.hash(2, "ab"), keys.hash(2, "ab\0"));
        assert_ne!(keys.hash(1, "string"), keys.hash(2, "string"));
        assert_ne!(keys.hash(2, "punctuation"), keys.hash(2, "punctuatioN"));
    }

    #[test]
    fn it_keeps_every_entry_within_the_probe_bound() {
        let parts: Vec<String> = (0..2_000).map(|n| format!("part{n}")).collect();
        let entries: Vec<(usize, &str, usize)> = parts
            .iter()
            .enumerate()
            .map(|(child, part)| (0, part.as_str(), child + 1))
            .collect();
        let table = Children::build(&entries);
        let mask = table.slots.len() - 1;
        for (at, slot) in table.slots.iter().enumerate() {
            if slot.child != EMPTY {
                let home = home(slot.hash, mask);
                assert!((at + table.slots.len() - home) & mask < MAX_PROBE);
            }
        }
    }
}
