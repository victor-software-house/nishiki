//! Protocol Buffers's tables.

use nishiki::unstable::codegen::tables::{CaptureLists, LonePattern, Reentry, RuleTables, Split};

/// Protocol Buffers's tables, from `tests/engine/grammars/proto.json`.
pub(crate) const PROTO: RuleTables = RuleTables {
    language: "proto",
    grammar: "proto",
    key_spellings: &[],
    capture_lists: CaptureLists::Rules,
    pattern_names: &[],
    split: Split::KeepsGroup,
    lone_pattern: LonePattern::OwnId,
    reentry: Reentry::WithinRule,
    externals: &[],
};
