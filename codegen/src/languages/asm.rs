//! Assembly's tables.

use nishiki::unstable::codegen::tables::{CaptureLists, LonePattern, RuleTables, Split};

/// Assembly's tables, from `tests/engine/grammars/asm.json`.
pub(crate) const ASM: RuleTables = RuleTables {
    language: "asm",
    grammar: "asm",
    key_spellings: &[],
    capture_lists: CaptureLists::Rules,
    pattern_names: &[],
    split: Split::KeepsGroup,
    lone_pattern: LonePattern::OwnId,
};
