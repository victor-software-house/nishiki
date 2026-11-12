//! Elixir's tables.

use nishiki::unstable::codegen::tables::{CaptureLists, LonePattern, Reentry, RuleTables, Split};

/// Elixir's tables, from `tests/engine/grammars/elixir.json`.
pub(crate) const ELIXIR: RuleTables = RuleTables {
    language: "elixir",
    grammar: "elixir",
    key_spellings: &[],
    capture_lists: CaptureLists::Rules,
    pattern_names: &[],
    split: Split::KeepsGroup,
    lone_pattern: LonePattern::OwnId,
    reentry: Reentry::WithinWalk,
};
