//! Common Lisp's tables.

use nishiki::unstable::codegen::tables::{
    CaptureLists, Identity, LonePattern, Reentry, RuleTables, Split,
};

/// Common Lisp's tables, from `tests/engine/grammars/common-lisp.json`.
pub(crate) const COMMON_LISP: RuleTables = RuleTables {
    language: "common_lisp",
    grammar: "common-lisp",
    key_spellings: &[],
    capture_lists: CaptureLists::Rules,
    pattern_names: &[],
    split: Split::KeepsGroup,
    lone_pattern: LonePattern::OwnId,
    identity: Identity::Id,
    reentry: Reentry::WithinWalk,
    externals: &[],
    injected: None,
};
