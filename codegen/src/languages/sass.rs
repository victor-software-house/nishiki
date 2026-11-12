//! Sass's tables.

use nishiki::unstable::codegen::tables::{CaptureLists, RuleTables};

/// Sass's tables, from `tests/engine/grammars/sass.json`.
pub(crate) const SASS: RuleTables = RuleTables {
    language: "sass",
    grammar: "sass",
    key_spellings: &[],
    capture_lists: CaptureLists::Rules,
};
