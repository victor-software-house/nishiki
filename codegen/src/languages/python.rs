//! Python's tables.

use nishiki::unstable::codegen::tables::{
    CaptureLists, Identity, LonePattern, Reentry, RuleTables, Split,
};

/// Python's tables, from `tests/engine/grammars/python.json`.
pub(crate) const PYTHON: RuleTables = RuleTables {
    language: "python",
    grammar: "python",
    key_spellings: &[("charecter", "character")],
    capture_lists: CaptureLists::Rules,
    pattern_names: &[],
    split: Split::KeepsGroup,
    lone_pattern: LonePattern::OwnId,
    identity: Identity::Id,
    reentry: Reentry::WithinRule,
    externals: &[],
    injected: None,
};
