//! Protocol Buffers's tables.

use nishiki::unstable::codegen::tables::{
    CaptureLists, CaptureNames, Identity, LonePattern, Reentry, RuleTables, Split, SplitIds,
};

/// Protocol Buffers's tables, from `tests/engine/grammars/proto.json`.
pub(crate) const PROTO: RuleTables = RuleTables {
    language: "proto",
    grammar: "proto",
    key_spellings: &[],
    capture_lists: CaptureLists::Rules,
    pattern_names: &[],
    split: Split::KeepsGroup,
    split_ids: SplitIds::Positions,
    capture_names: CaptureNames::Kept,
    lone_pattern: LonePattern::OwnId,
    identity: Identity::Id,
    reentry: Reentry::WithinRule,
    externals: &[],
    injected: None,
    added_rules: &[],
    merged_words: None,
};
