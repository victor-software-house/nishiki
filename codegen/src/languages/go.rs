//! Go's tables.

use nishiki::unstable::codegen::tables::{CaptureLists, RuleTables};

/// Go's tables, from `tests/engine/grammars/go.json`.
pub(crate) const GO: RuleTables = RuleTables {
    language: "go",
    grammar: "go",
    key_spellings: &[],
    capture_lists: CaptureLists::Rules,
};
