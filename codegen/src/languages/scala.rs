//! Scala's tables.

use nishiki::unstable::codegen::tables::{
    CaptureLists, CaptureNames, Identity, LonePattern, Reentry, RuleTables, Split, SplitIds,
};

/// Scala's tables, from `tests/engine/grammars/scala.json`.
pub(crate) const SCALA: RuleTables = RuleTables {
    language: "scala",
    grammar: "scala",
    key_spellings: &[],
    capture_lists: CaptureLists::ByHand,
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
