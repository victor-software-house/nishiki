//! Scala's tables.

use nishiki::unstable::codegen::tables::{
    CaptureLists, CaptureNames, Identity, LonePattern, Reentry, RuleTables, Split,
};

/// Scala's tables, from `tests/engine/grammars/scala.json`.
pub(crate) const SCALA: RuleTables = RuleTables {
    language: "scala",
    grammar: "scala",
    key_spellings: &[],
    capture_lists: CaptureLists::ByHand,
    pattern_names: &[],
    split: Split::KeepsGroup,
    capture_names: CaptureNames::Kept,
    lone_pattern: LonePattern::OwnId,
    identity: Identity::Id,
    reentry: Reentry::WithinRule,
    externals: &[],
    injected: None,
    added_rules: &[],
};
