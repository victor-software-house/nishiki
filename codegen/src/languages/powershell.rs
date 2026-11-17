//! PowerShell's tables.

use nishiki::unstable::codegen::tables::{
    CaptureLists, CaptureNames, Identity, LonePattern, Reentry, RuleTables, Split,
};

/// PowerShell's tables, from `tests/engine/grammars/powershell.json`.
pub(crate) const POWERSHELL: RuleTables = RuleTables {
    language: "powershell",
    grammar: "powershell",
    key_spellings: &[],
    capture_lists: CaptureLists::Rules,
    pattern_names: &[
        ("RootAt0", "BlockComment"),
        ("RootAt14", "SingleQuoted"),
        ("RootAt15", "HereDouble"),
        ("RootAt16", "HereSingle"),
        ("RootAt18", "ArrayExpression"),
        ("RootAt19", "ComplexSubexpression"),
        ("RootAt23", "StopParsing"),
    ],
    split: Split::KeepsGroup,
    capture_names: CaptureNames::Kept,
    lone_pattern: LonePattern::ListId,
    identity: Identity::Id,
    reentry: Reentry::WithinRule,
    externals: &[],
    injected: None,
};
