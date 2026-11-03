//! One lookup that says which of the grammar's word lists a word is in, so
//! the dozen patterns that test a word against a list do not each search.
//! Every test of a word against a list goes through it.

use std::sync::OnceLock;

use super::words;
use crate::fnv::FnvMap;

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
pub(super) const FUNCDEF_EXCL: u32 = 1 << 13;
pub(super) const MEMBER_EXCL: u32 = 1 << 14;
pub(super) const CALL_EXCL: u32 = 1 << 15;
pub(super) const DEFINE_EXCL: u32 = 1 << 16;

/// The words and the lists they belong to, besides the generated ones.
const TAG_TYPES: &[&str] = &["enum", "struct", "union"];
const ASMS: &[&str] = &["__asm__", "asm"];

type Map = FnvMap<&'static [u8], u32>;

fn map() -> &'static Map {
    static MAP: OnceLock<Map> = OnceLock::new();
    MAP.get_or_init(|| {
        let mut map = Map::default();
        let lists: [(&[&'static str], u32); 17] = [
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
            (words::FUNCDEF_EXCL, FUNCDEF_EXCL),
            (words::MEMBER_EXCL, MEMBER_EXCL),
            (words::CALL_EXCL, CALL_EXCL),
            (words::DEFINE_EXCL, DEFINE_EXCL),
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
