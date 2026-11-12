//! Python's tables.

use nishiki::unstable::codegen::tables::{
    CaptureLists, CaptureNames, Identity, LonePattern, Reentry, RuleTables, Split,
};

/// Python's tables, from `tests/engine/grammars/python.json`.
pub(crate) const PYTHON: RuleTables = RuleTables {
    language: "python",
    grammar: "python",
    key_spellings: &[("charecter", "character")],
    capture_lists: CaptureLists::Rules,
    pattern_names: &[],
    split: Split::KeepsGroup,
    capture_names: CaptureNames::Kept,
    lone_pattern: LonePattern::OwnId,
    identity: Identity::Id,
    reentry: Reentry::WithinRule,
    externals: &[],
    injected: None,
    added_rules: &[],
    merged_words: None,
};
