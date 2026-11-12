//! Less's tables.

use nishiki::unstable::codegen::tables::{
    CaptureLists, CaptureNames, Identity, LonePattern, Reentry, RuleTables, Split, SplitIds,
};

/// Less's tables, from `tests/engine/grammars/less.json`.
pub(crate) const LESS: RuleTables = RuleTables {
    language: "less",
    grammar: "less",
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
