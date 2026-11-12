//! Java's tables.

use nishiki::unstable::codegen::tables::{CaptureLists, LonePattern, RuleTables, Split};

/// Java's tables, from `tests/engine/grammars/java.json`.
pub(crate) const JAVA: RuleTables = RuleTables {
    language: "java",
    grammar: "java",
    key_spellings: &[("charecter", "character")],
    capture_lists: CaptureLists::ByHand,
    pattern_names: &[],
    split: Split::DropsGroup,
    lone_pattern: LonePattern::OwnId,
};
