//! Sass's tables.

use nishiki::unstable::codegen::tables::{CaptureLists, LonePattern, Reentry, RuleTables, Split};

/// Sass's tables, from `tests/engine/grammars/sass.json`.
pub(crate) const SASS: RuleTables = RuleTables {
    language: "sass",
    grammar: "sass",
    key_spellings: &[],
    capture_lists: CaptureLists::Rules,
    pattern_names: &[],
    split: Split::KeepsGroup,
    lone_pattern: LonePattern::OwnId,
    reentry: Reentry::WithinRule,
    externals: &[],
};
