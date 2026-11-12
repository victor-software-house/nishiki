//! Swift's tables.

use nishiki::unstable::codegen::tables::{
    CaptureLists, CaptureNames, Identity, LonePattern, Reentry, RuleTables, Split, SplitIds,
};

/// Swift's tables, from `tests/engine/grammars/swift.json`.
pub(crate) const SWIFT: RuleTables = RuleTables {
    language: "swift",
    grammar: "swift",
    key_spellings: &[],
    capture_lists: CaptureLists::Rules,
    pattern_names: &[],
    split: Split::KeepsGroup,
    split_ids: SplitIds::Words,
    capture_names: CaptureNames::SplitsGroup,
    lone_pattern: LonePattern::OwnId,
    identity: Identity::Id,
    reentry: Reentry::WithinRule,
    externals: &[],
    injected: None,
    added_rules: &[],
    merged_words: None,
};
