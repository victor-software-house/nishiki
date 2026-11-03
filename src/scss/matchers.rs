//! The matchers: for each regular expression of the grammar, the function that
//! decides whether it matches at a position and where. A matcher records the
//! capture groups the grammar names with
//! [`Lexer::cap`](crate::rules::Lexer::cap); the engine names them. The
//! patterns that `source.css#…` brings in go to CSS's matchers instead (see the
//! `engine` module).
//!
//! Each follows its expression as an Oniguruma-style backtracking matcher
//! would. Where a quantifier could give characters back, the doc comment says
//! why it cannot matter, or the matcher tries the shorter matches.

mod atrules;
mod chars;
mod selectors;
mod values;

use atrules::Tail;
use selectors::Sigil;

use super::engine::Lexer;
use super::grammar::Pat;
use crate::rules::Chars;

impl Lexer<'_> {
    /// Whether SCSS's own pattern `pat` matches at `p`, and where its match
    /// ends.
    #[expect(
        clippy::too_many_lines,
        reason = "one arm for each expression of the grammar"
    )]
    #[expect(
        clippy::wildcard_enum_match_arm,
        reason = "the patterns `source.css#…` brings in go to CSS's matchers before this match"
    )]
    #[inline]
    pub(super) fn scss_matcher(&mut self, pat: Pat, p: usize) -> Option<usize> {
        match pat {
            Pat::VariableSetting => self.variable_setting(p),
            Pat::VariableSettingEnd | Pat::RootAt20 => self.lit(p, b";"),
            Pat::VariableSettingAt0 => self.variable_setting_at0(p),
            Pat::VariableSettingAt1 | Pat::FunctionAttributesAt0 => self.lit(p, b":"),
            Pat::VariableSettingAt1End => (self.at(p) == b';').then_some(p),
            Pat::CommentBlock => self.lit(p, b"/*"),
            Pat::CommentBlockEnd => self.lit(p, b"*/"),
            Pat::CommentLine => self.lit(p, b"//"),
            Pat::CommentLineEnd => self.comment_line_end(p),
            Pat::Map
            | Pat::ParametersAt1
            | Pat::PropertyValuesAt13
            | Pat::AtRuleUseAt7
            | Pat::AtRuleMediaAt0At4
            | Pat::AtRuleSupportsAt2 => self.lit(p, b"("),
            Pat::MapEnd
            | Pat::ParametersAt1End
            | Pat::PropertyValuesAt13End
            | Pat::AtRuleUseAt7End
            | Pat::AtRuleIncludeAt0End
            | Pat::SelectorPseudoClassAt0End
            | Pat::AtRuleMediaAt0At4End
            | Pat::AtRuleMixinAt0End
            | Pat::AtRuleSupportsAt3 => self.lit(p, b")"),
            Pat::MapAt3 => self.map_at3(p),
            Pat::MapAt4 | Pat::ParametersAt5 | Pat::VariableSettingAt1At6 => self.lit(p, b","),
            Pat::VariablesAt0 => self.variables_at0(p),
            Pat::VariablesAt1 => self.variables_at1(p),
            Pat::Interpolation => self.lit(p, b"#{"),
            Pat::InterpolationEnd | Pat::PropertyListEnd | Pat::AtRuleKeyframesAt3End => {
                self.lit(p, b"}")
            }
            Pat::StringSingle | Pat::StringSingleEnd | Pat::AtRuleKeyframesAt2End => {
                self.lit(p, b"'")
            }
            Pat::StringSingleAt0
            | Pat::StringDoubleAt0
            | Pat::Nested0At1
            | Pat::Nested1At1
            | Pat::AtRuleKeyframesAt1At0
            | Pat::AtRuleKeyframesAt2At0 => self.string_single_at0(p),
            Pat::StringDouble | Pat::StringDoubleEnd | Pat::AtRuleKeyframesAt1End => {
                self.lit(p, b"\"")
            }
            Pat::ConstantFunctions => self.constant_functions(p),
            Pat::ConstantFunctionsEnd | Pat::ConstantSassFunctionsEnd | Pat::FunctionsAt0End => {
                self.constant_functions_end(p)
            }
            Pat::ConstantSassFunctions => self.constant_sass_functions(p),
            Pat::ConstantImportant => self.lit(p, b"!important"),
            Pat::ConstantDefault => self.lit(p, b"!default"),
            Pat::ConstantOptional => self.lit(p, b"!optional"),
            Pat::ConstantMathematicalSymbols => self.constant_mathematical_symbols(p),
            Pat::Operators => self.operators(p),
            Pat::ParametersAt4 => self.parameters_at4(p),
            Pat::FunctionAttributesAt3 => {
                matches!(self.at(p), b';' | b'=' | b'?' | b'@' | b'{' | b'}').then_some(p + 1)
            }
            Pat::AtRuleForward => self.at_keyword(p, b"forward", Tail::Spaces),
            Pat::AtRuleForwardEnd
            | Pat::AtRuleUseEnd
            | Pat::AtRuleWarnEnd
            | Pat::AtRuleExtendEnd
            | Pat::AtRuleReturnEnd
            | Pat::AtRuleContentEnd => self.spaces_before(p, b";"),
            Pat::AtRuleForwardAt0 => self.word_in(p, &[b"as", b"hide", b"show"]),
            Pat::AtRuleForwardAt1 => self.at_rule_forward_at1(p),
            Pat::AtRuleForwardAt2 | Pat::AtRuleUseAt1 => self.at_rule_forward_at2(p),
            Pat::AtRuleUse => self.at_keyword(p, b"use", Tail::Spaces),
            Pat::AtRuleUseAt0 => self.word_in(p, &[b"as", b"with"]),
            Pat::AtRuleUseAt2 | Pat::TagWildcard => self.lit(p, b"*"),
            Pat::AtRuleIncludeAt0 => self.at_rule_include_at0(p),
            Pat::AtRuleIncludeAt1 => self.at_rule_include_at1(p),
            Pat::AtRuleIncludeAt2 => self.at_bare_keyword(p, b"include"),
            Pat::AtRuleImport => self.at_keyword(p, b"import", Tail::Spaces),
            Pat::AtRuleImportEnd => self.spaces_before(p, b";}"),
            Pat::FunctionsAt0 => self.scss_functions_at0(p),
            Pat::FunctionsAt1 => self.dash_word_run(p),
            Pat::AtRuleIf => self.at_keyword(p, b"if", Tail::Spaces),
            Pat::AtRuleIfEnd
            | Pat::AtRuleElseEnd
            | Pat::AtRuleForEnd
            | Pat::AtRuleMediaAt0End
            | Pat::AtRuleFunctionAt0End
            | Pat::AtRuleFontfaceAt0End
            | Pat::AtRulePageAt0End
            | Pat::AtRuleAtRootEnd => self.spaces_before(p, b"{"),
            Pat::ComparisonOperators => self.comparison_operators(p),
            Pat::LogicalOperators => self.word_in(p, &[b"not", b"or", b"and"]),
            Pat::AtRuleElse => self.at_rule_else(p),
            Pat::AtRuleWarn => self.at_rule_warn(p),
            Pat::AtRuleFor => self.at_keyword(p, b"for", Tail::Spaces),
            Pat::AtRuleForAt0 => self.at_rule_for_at0(p),
            Pat::AtRuleWhile => self.at_keyword(p, b"while", Tail::Spaces),
            Pat::AtRuleWhileEnd | Pat::AtRuleEachEnd => self.spaces_before(p, b"}"),
            Pat::AtRuleEach => self.at_keyword(p, b"each", Tail::Spaces),
            Pat::AtRuleEachAt0 => self.at_rule_each_at0(p),
            Pat::AtRuleReturn => self.at_keyword(p, b"return", Tail::None),
            Pat::AtRuleExtend => self.at_keyword(p, b"extend", Tail::Spaces),
            Pat::SelectorCustom => self.selector_custom(p),
            Pat::Nested0At2 | Pat::Nested1At2 => matches!(self.at(p), b'$' | b'}').then_some(p + 1),
            Pat::SelectorClass => self.selector_name(p, Sigil::Class),
            Pat::SelectorId => self.selector_name(p, Sigil::Id),
            Pat::SelectorPseudoClassAt0 => self.selector_pseudo_class_at0(p),
            Pat::SelectorPseudoClassAt0At1 => self.selector_pseudo_class_at0_at1(p),
            Pat::SelectorPseudoClassAt0At2 => self.selector_pseudo_class_at0_at2(p),
            Pat::SelectorPseudoClassAt0At3 => self.word_chars(p),
            Pat::TagParentReference => self.lit(p, b"&"),
            Pat::SelectorAttribute => self.selector_attribute(p),
            Pat::SelectorPlaceholder => self.selector_name(p, Sigil::Placeholder),
            Pat::ParentSelectorSuffix => self.parent_selector_suffix(p),
            Pat::AtRuleContent => self.at_keyword(p, b"content", Tail::Spaces),
            Pat::AtRuleMediaAt0 => self.line_at_keyword(p, b"media", Tail::None),
            Pat::AtRuleMediaAt0At3 => self.keyword(p, b"only"),
            Pat::AtRuleMediaAt0At4At0 | Pat::PropertiesAt0 => self.at_rule_media_at0_at4_at0(p),
            Pat::AtRuleMediaAt0At4At0End | Pat::PropertiesAt0End => {
                self.at_rule_media_at0_at4_at0_end(p)
            }
            Pat::AtRuleMediaAt0At4At1 | Pat::PropertiesAt1 => self.at_rule_media_at0_at4_at1(p),
            Pat::AtRuleMediaAt0At4At1End | Pat::PropertiesAt1End => {
                self.at_rule_media_at0_at4_at1_end(p)
            }
            Pat::PropertyList | Pat::AtRuleKeyframesAt3 => self.lit(p, b"{"),
            Pat::AtRuleMixinAt0 => self.at_rule_mixin_at0(p),
            Pat::AtRuleMixinAt1 => self.at_rule_mixin_at1(p),
            Pat::AtRuleMixinAt2 => self.at_bare_keyword(p, b"mixin"),
            Pat::AtRuleFunctionAt0 | Pat::AtRuleFunctionAt1 => {
                self.at_keyword(p, b"function", Tail::Spaces)
            }
            Pat::AtRuleCharset => self.at_keyword(p, b"charset", Tail::Spaces),
            Pat::AtRuleCharsetEnd => self.spaces_before_or_eol(p, b";"),
            Pat::AtRuleOption => self.line_at_keyword(p, b"option", Tail::Spaces),
            Pat::AtRuleNamespaceAt0 => self.at_rule_namespace_at0(p),
            Pat::AtRuleNamespaceAt0End | Pat::AtRuleNamespaceAt1End => {
                (self.at(p) == b';' || self.at_eol(p)).then_some(p)
            }
            Pat::AtRuleNamespaceAt1 => self.at_rule_namespace_at1(p),
            Pat::AtRuleNamespaceAt2 => self.at_bare_keyword(p, b"namespace"),
            Pat::AtRuleFontfaceAt0 => self.line_at_keyword(p, b"font-face", Tail::None),
            Pat::AtRulePageAt0 => self.at_rule_page_at0(p),
            Pat::AtRuleKeyframes => self.at_rule_keyframes(p),
            Pat::AtRuleKeyframesEnd => self.prev_is(p, b'}').then_some(p),
            Pat::AtRuleKeyframesAt0 => self.at_rule_keyframes_at0(p),
            Pat::AtRuleKeyframesAt1 => self.keyframes_quote(p, b'"'),
            Pat::AtRuleKeyframesAt2 => self.keyframes_quote(p, b'\''),
            Pat::AtRuleKeyframesAt3At0 => self.at_rule_keyframes_at3_at0(p),
            Pat::AtRuleAtRoot => self.at_rule_at_root(p),
            Pat::AtRuleSupports => self.at_rule_supports(p),
            Pat::AtRuleSupportsEnd => (self.at(p) == b'{' || self.at_eol(p)).then_some(p),
            _ => None,
        }
    }
}
