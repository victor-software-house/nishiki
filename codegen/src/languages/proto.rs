//! Protocol Buffers's tables.

use nishiki::unstable::codegen::tables::{CaptureLists, RuleTables};

/// Protocol Buffers's tables, from `tests/engine/grammars/proto.json`.
pub(crate) const PROTO: RuleTables = RuleTables {
    language: "proto",
    grammar: "proto",
    key_spellings: &[],
    capture_lists: CaptureLists::Rules,
};
