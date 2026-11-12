//! Ruby's tables.

use nishiki::unstable::codegen::tables::{
    CaptureLists, CaptureNames, External, Identity, LonePattern, Reentry, RuleTables, Split,
    SplitIds,
};

/// Ruby's tables, from `tests/engine/grammars/ruby.json`.
///
/// The heredocs include other grammars.
/// Their lists add nothing, since the language's code hands each heredoc's
/// text to the language it names.
pub(crate) const RUBY: RuleTables = RuleTables {
    language: "ruby",
    grammar: "ruby",
    key_spellings: &[],
    capture_lists: CaptureLists::Rules,
    pattern_names: &[],
    split: Split::KeepsGroup,
    split_ids: SplitIds::Positions,
    capture_names: CaptureNames::Kept,
    lone_pattern: LonePattern::OwnId,
    identity: Identity::Id,
    reentry: Reentry::WithinRule,
    externals: &[
        ("source.c", External::AddsNothing),
        ("source.cpp", External::AddsNothing),
        ("source.css", External::AddsNothing),
        ("source.graphql", External::AddsNothing),
        ("source.js", External::AddsNothing),
        ("source.js.jquery", External::AddsNothing),
        ("source.lua", External::AddsNothing),
        ("source.shell", External::AddsNothing),
        ("source.sql", External::AddsNothing),
        ("source.yaml", External::AddsNothing),
        ("text.haml", External::AddsNothing),
        ("text.html.basic", External::AddsNothing),
        ("text.slim", External::AddsNothing),
        ("text.xml", External::AddsNothing),
    ],
    injected: None,
    added_rules: &[],
    merged_words: None,
};
