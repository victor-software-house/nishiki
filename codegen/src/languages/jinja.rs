//! Jinja's tables.

use nishiki::unstable::codegen::tables::{
    CaptureLists, CaptureNames, Identity, LonePattern, Reentry, RuleTables, Split, SplitIds,
};

/// Jinja's tables, from `tests/engine/grammars/jinja.json`.
pub(crate) const JINJA: RuleTables = RuleTables {
    language: "jinja",
    grammar: "jinja",
    key_spellings: &[],
    capture_lists: CaptureLists::Rules,
    pattern_names: &[
        ("RootAt0", "Raw"),
        ("RootAt2", "Output"),
        ("RootAt3", "Statement"),
        ("ExpressionAt0", "Block"),
        ("ExpressionAt1", "Filter"),
        ("ExpressionAt2", "Is"),
        ("ExpressionAt3", "TagName"),
        ("ExpressionAt4", "Keywords"),
        ("ExpressionAt5", "Constants"),
        ("ExpressionAt6", "Builtins"),
        ("ExpressionAt7", "Name"),
        ("ExpressionAt8", "Arithmetic"),
        ("ExpressionAt9", "FilterCall"),
        ("ExpressionAt10", "Member"),
        ("ExpressionAt11", "Brackets"),
        ("ExpressionAt12", "Parens"),
        ("ExpressionAt13", "Braces"),
        ("ExpressionAt14", "Punctuation"),
        ("ExpressionAt15", "Comparison"),
        ("ExpressionAt16", "Assignment"),
        ("ExpressionAt17", "DoubleString"),
        ("ExpressionAt18", "SingleString"),
        ("ExpressionAt19", "Regex"),
    ],
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
