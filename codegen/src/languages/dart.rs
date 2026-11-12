//! Dart's tables.

use nishiki::unstable::codegen::tables::{
    CaptureLists, CaptureNames, Identity, LonePattern, Reentry, RuleTables, Split,
};

/// Dart's tables, from `tests/engine/grammars/dart.json`.
pub(crate) const DART: RuleTables = RuleTables {
    language: "dart",
    grammar: "dart",
    key_spellings: &[],
    capture_lists: CaptureLists::Rules,
    pattern_names: &[("RootAt0", "Shebang"), ("RootAt1", "Directive")],
    split: Split::KeepsGroup,
    capture_names: CaptureNames::Kept,
    lone_pattern: LonePattern::ListId,
    identity: Identity::Id,
    reentry: Reentry::WithinRule,
    externals: &[],
    injected: None,
    added_rules: &[],
    merged_words: None,
};
