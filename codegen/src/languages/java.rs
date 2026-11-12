//! Java's tables.

use nishiki::unstable::codegen::tables::{
    CaptureLists, CaptureNames, Identity, LonePattern, Reentry, RuleTables, Split,
};

/// Java's tables, from `tests/engine/grammars/java.json`.
pub(crate) const JAVA: RuleTables = RuleTables {
    language: "java",
    grammar: "java",
    key_spellings: &[("charecter", "character")],
    capture_lists: CaptureLists::ByHand,
    pattern_names: &[],
    split: Split::DropsGroup,
    capture_names: CaptureNames::Kept,
    lone_pattern: LonePattern::OwnId,
    identity: Identity::Id,
    reentry: Reentry::WithinRule,
    externals: &[],
    injected: None,
    added_rules: &[],
};
