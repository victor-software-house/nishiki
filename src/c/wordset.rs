//! One lookup that says which of the grammar's word lists a word is in, so
//! the dozen patterns that test a word against a list do not each search.

use std::collections::HashMap;
use std::hash::{BuildHasherDefault, Hasher};
use std::sync::OnceLock;

use super::words;

pub(super) const CONTROL: u32 = 1;
pub(super) const MODIFIER: u32 = 1 << 1;
pub(super) const CONSTANT: u32 = 1 << 2;
pub(super) const SYS_TYPES: u32 = 1 << 3;
pub(super) const MAC_CONSTANT: u32 = 1 << 4;
pub(super) const MAC_TYPE: u32 = 1 << 5;
pub(super) const PRIMITIVE: u32 = 1 << 6;
pub(super) const BUILTIN: u32 = 1 << 7;
pub(super) const STDINT: u32 = 1 << 8;
pub(super) const PTHREAD: u32 = 1 << 9;
pub(super) const PREDEF: u32 = 1 << 10;
pub(super) const TAG_TYPE: u32 = 1 << 11;
pub(super) const ASM: u32 = 1 << 12;

/// The words and the lists they belong to, besides the generated ones.
const TAG_TYPES: &[&str] = &["enum", "struct", "union"];
const ASMS: &[&str] = &["__asm__", "asm"];

/// FNV-1a over the bytes of a word.
#[derive(Default)]
struct Fnv(u64);

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

type Map = HashMap<&'static [u8], u32, BuildHasherDefault<Fnv>>;

fn map() -> &'static Map {
    static MAP: OnceLock<Map> = OnceLock::new();
    MAP.get_or_init(|| {
        let mut map = Map::default();
        let lists: [(&[&'static str], u32); 13] = [
            (words::CONTROL, CONTROL),
            (words::MODIFIER, MODIFIER),
            (words::CONSTANT, CONSTANT),
            (words::SYS_TYPES, SYS_TYPES),
            (words::MAC_CONSTANT, MAC_CONSTANT),
            (words::MAC_TYPE, MAC_TYPE),
            (words::PRIMITIVE, PRIMITIVE),
            (words::BUILTIN, BUILTIN),
            (words::STDINT, STDINT),
            (words::PTHREAD, PTHREAD),
            (words::PREDEF, PREDEF),
            (TAG_TYPES, TAG_TYPE),
            (ASMS, ASM),
        ];
        for (list, bit) in lists {
            for word in list {
                *map.entry(word.as_bytes()).or_insert(0) |= bit;
            }
        }
        map
    })
}

/// The lists `word` is in, as a set of the bits above.
pub(super) fn of(word: &[u8]) -> u32 {
    map().get(word).copied().unwrap_or(0)
}
