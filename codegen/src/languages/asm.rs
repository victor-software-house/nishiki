//! Assembly's tables.

use nishiki::unstable::codegen::tables::{
    CaptureLists, CaptureNames, External, Identity, LonePattern, MergedWords, Reentry, RuleTables,
    Split,
};

/// Assembly's tables, from `tests/engine/grammars/asm.json`.
///
/// The grammar includes two repository keys it never defines.
/// vscode-textmate skips such an include, and drops a rule that lists
/// nothing else, so a quote starts no string.
///
/// The registers and mnemonics, `Root`'s first two entries, merge into the
/// match `Word`.
pub(crate) const ASM: RuleTables = RuleTables {
    language: "asm",
    grammar: "asm",
    key_spellings: &[],
    capture_lists: CaptureLists::Rules,
    pattern_names: &[],
    split: Split::KeepsGroup,
    capture_names: CaptureNames::Kept,
    lone_pattern: LonePattern::OwnId,
    identity: Identity::Id,
    reentry: Reentry::WithinWalk,
    externals: &[
        ("#string_escaped_char", External::Missing),
        ("#string_placeholder", External::Missing),
    ],
    injected: None,
    added_rules: &[],
    merged_words: Some(MergedWords {
        id: "Word",
        entries: 2,
    }),
};
