//! Swift's tables.

use nishiki::unstable::codegen::tables::{CaptureLists, LonePattern, RuleTables, Split};

/// Swift's tables, from `tests/engine/grammars/swift.json`.
pub(crate) const SWIFT: RuleTables = RuleTables {
    language: "swift",
    grammar: "swift",
    key_spellings: &[("charecter", "character")],
    capture_lists: CaptureLists::Rules,
    pattern_names: &[],
    split: Split::KeepsGroup,
    lone_pattern: LonePattern::OwnId,
};
