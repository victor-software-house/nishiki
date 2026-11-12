//! GDScript's tables.

use nishiki::unstable::codegen::tables::{
    CaptureLists, CaptureNames, Identity, LonePattern, Reentry, RuleTables, Split, SplitIds,
};

/// GDScript's tables, from `tests/engine/grammars/gdscript.json`.
pub(crate) const GDSCRIPT: RuleTables = RuleTables {
    language: "gdscript",
    grammar: "gdscript",
    key_spellings: &[],
    capture_lists: CaptureLists::Rules,
    pattern_names: &[],
    split: Split::KeepsGroup,
    split_ids: SplitIds::Positions,
    capture_names: CaptureNames::Kept,
    lone_pattern: LonePattern::OwnId,
    identity: Identity::Id,
    reentry: Reentry::WithinWalk,
    externals: &[],
    injected: None,
    added_rules: &[],
    merged_words: None,
};
