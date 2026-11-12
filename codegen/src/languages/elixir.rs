//! Elixir's tables.

use nishiki::unstable::codegen::tables::{
    CaptureLists, External, Identity, LonePattern, Reentry, RuleTables, Split,
};

/// Elixir's tables, from `tests/engine/grammars/elixir.json`.
pub(crate) const ELIXIR: RuleTables = RuleTables {
    language: "elixir",
    grammar: "elixir",
    key_spellings: &[],
    capture_lists: CaptureLists::Rules,
    pattern_names: &[],
    split: Split::KeepsGroup,
    lone_pattern: LonePattern::OwnId,
    identity: Identity::Id,
    reentry: Reentry::WithinWalk,
    externals: &[
        ("text.elixir", External::AddsNothing),
        ("text.html.basic", External::AddsNothing),
    ],
};
