//! Less's tables.

use nishiki::unstable::codegen::tables::{CaptureLists, LonePattern, Reentry, RuleTables, Split};

/// Less's tables, from `tests/engine/grammars/less.json`.
pub(crate) const LESS: RuleTables = RuleTables {
    language: "less",
    grammar: "less",
    key_spellings: &[],
    capture_lists: CaptureLists::Rules,
    pattern_names: &[],
    split: Split::KeepsGroup,
    lone_pattern: LonePattern::OwnId,
    reentry: Reentry::WithinRule,
    externals: &[],
};
