//! Visual Basic's tables.

use nishiki::unstable::codegen::tables::{
    CaptureLists, Identity, LonePattern, Reentry, RuleTables, Split,
};

/// Visual Basic's tables, from `tests/engine/grammars/vb.json`.
pub(crate) const VB: RuleTables = RuleTables {
    language: "vb",
    grammar: "vb",
    key_spellings: &[],
    capture_lists: CaptureLists::Rules,
    pattern_names: &[
        ("RootAt0", "LineFeed"),
        ("RootAt2", "Tabs"),
        ("RootAt3", "Spaces"),
        ("RootAt4", "FunctionHead"),
        ("RootAt5", "Comment"),
        ("RootAt6", "Control"),
        ("RootAt7", "Operators"),
        ("RootAt8", "DimName"),
        ("RootAt9", "Storage"),
        ("RootAt10", "Modifiers"),
        ("RootAt11", "Constants"),
        ("RootAt12", "String"),
        ("RootAt13", "DollarVariable"),
        ("RootAt14", "AspClasses"),
        ("RootAt15", "AspCollections"),
        ("RootAt16", "AspConstants"),
        ("RootAt17", "AspFunctions"),
        ("RootAt18", "AspEvents"),
        ("RootAt19", "AsType"),
        ("RootAt20", "Functions"),
        ("RootAt21", "Numbers"),
        ("RootAt22", "VbConstants"),
        ("RootAt23", "FunctionCall"),
        ("RootAt24", "Variable"),
        ("RootAt25", "JsOperators"),
    ],
    split: Split::KeepsGroup,
    lone_pattern: LonePattern::OwnId,
    identity: Identity::Id,
    reentry: Reentry::WithinWalk,
    externals: &[],
    injected: None,
};
