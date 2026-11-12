//! CSS's tables.

use nishiki::unstable::codegen::tables::{
    AddedRule, CaptureLists, CaptureNames, Identity, LonePattern, Reentry, RuleTables, Split,
};

/// CSS's tables, from `tests/engine/grammars/css.json`.
///
/// The rule `VueStyleAttribute` lists the patterns of the string in Vue's
/// `style` attribute, from `tests/engine/grammars/vue.json`.
/// A Vue component lexes that string with CSS's tables.
pub(crate) const CSS: RuleTables = RuleTables {
    language: "css",
    grammar: "css",
    key_spellings: &[],
    capture_lists: CaptureLists::ByHand,
    pattern_names: &[],
    split: Split::KeepsGroup,
    capture_names: CaptureNames::SplitsOptionalGroup,
    lone_pattern: LonePattern::OwnId,
    identity: Identity::Id,
    reentry: Reentry::WithinRule,
    externals: &[],
    injected: None,
    added_rules: &[AddedRule {
        id: "VueStyleAttribute",
        grammar: "vue",
        key: "vue-directives-style-attr",
        path: &[0],
    }],
};
