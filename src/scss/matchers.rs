//! The matchers: for each regular expression of the grammar, the function that
//! decides whether it matches at a position and where.
//! A matcher records the capture groups the grammar names with
//! [`Lexer::capture`](crate::rules::Lexer::capture).
//! The engine names them.
//! The patterns that `source.css#…` brings in go to CSS's matchers instead.
//! How that works is in the `engine` module.
//!
//! Each follows its expression as an Oniguruma-style backtracking matcher
//! would.
//! Where a quantifier could give characters back, the doc comment shows why it
//! cannot matter, or the matcher tries the shorter matches.

mod atrules;
mod chars;
mod selectors;
mod values;

use atrules::Tail;
use selectors::Sigil;

use super::engine::Lexer;
use super::grammar::Pattern;
use crate::rules::Chars;

impl Lexer<'_> {
    /// Whether SCSS's own pattern `pattern` matches at `p`, and where its match
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
    pub(super) fn scss_matcher(&mut self, pattern: Pattern, p: usize) -> Option<usize> {
        match pattern {
            Pattern::VariableSetting => self.variable_setting(p),
            Pattern::VariableSettingEnd | Pattern::RootAt20 => self.lit(p, b";"),
            Pattern::VariableSettingAt0 => self.variable_setting_at0(p),
            Pattern::VariableSettingAt1 | Pattern::FunctionAttributesAt0 => self.lit(p, b":"),
            Pattern::VariableSettingAt1End => (self.at(p) == b';').then_some(p),
            Pattern::CommentBlock => self.lit(p, b"/*"),
            Pattern::CommentBlockEnd => self.lit(p, b"*/"),
            Pattern::CommentLine => self.lit(p, b"//"),
            Pattern::CommentLineEnd => self.comment_line_end(p),
            Pattern::Map
            | Pattern::ParametersAt1
            | Pattern::PropertyValuesAt13
            | Pattern::AtRuleUseAt7
            | Pattern::AtRuleMediaAt0At4
            | Pattern::AtRuleSupportsAt2 => self.lit(p, b"("),
            Pattern::MapEnd
            | Pattern::ParametersAt1End
            | Pattern::PropertyValuesAt13End
            | Pattern::AtRuleUseAt7End
            | Pattern::AtRuleIncludeAt0End
            | Pattern::SelectorPseudoClassAt0End
            | Pattern::AtRuleMediaAt0At4End
            | Pattern::AtRuleMixinAt0End
            | Pattern::AtRuleSupportsAt3 => self.lit(p, b")"),
            Pattern::MapAt3 => self.map_at3(p),
            Pattern::MapAt4 | Pattern::ParametersAt5 | Pattern::VariableSettingAt1At6 => {
                self.lit(p, b",")
            }
            Pattern::VariablesAt0 => self.variables_at0(p),
            Pattern::VariablesAt1 => self.variables_at1(p),
            Pattern::Interpolation => self.lit(p, b"#{"),
            Pattern::InterpolationEnd
            | Pattern::PropertyListEnd
            | Pattern::AtRuleKeyframesAt3End => self.lit(p, b"}"),
            Pattern::StringSingle | Pattern::StringSingleEnd | Pattern::AtRuleKeyframesAt2End => {
                self.lit(p, b"'")
            }
            Pattern::StringSingleAt0
            | Pattern::StringDoubleAt0
            | Pattern::Nested0At1
            | Pattern::Nested1At1
            | Pattern::AtRuleKeyframesAt1At0
            | Pattern::AtRuleKeyframesAt2At0 => self.string_single_at0(p),
            Pattern::StringDouble | Pattern::StringDoubleEnd | Pattern::AtRuleKeyframesAt1End => {
                self.lit(p, b"\"")
            }
            Pattern::ConstantFunctions => self.constant_functions(p),
            Pattern::ConstantFunctionsEnd
            | Pattern::ConstantSassFunctionsEnd
            | Pattern::FunctionsAt0End => self.constant_functions_end(p),
            Pattern::ConstantSassFunctions => self.constant_sass_functions(p),
            Pattern::ConstantImportant => self.lit(p, b"!important"),
            Pattern::ConstantDefault => self.lit(p, b"!default"),
            Pattern::ConstantOptional => self.lit(p, b"!optional"),
            Pattern::ConstantMathematicalSymbols => self.constant_mathematical_symbols(p),
            Pattern::Operators => self.operators(p),
            Pattern::ParametersAt4 => self.parameters_at4(p),
            Pattern::FunctionAttributesAt3 => {
                matches!(self.at(p), b';' | b'=' | b'?' | b'@' | b'{' | b'}').then_some(p + 1)
            }
            Pattern::AtRuleForward => self.at_keyword(p, b"forward", Tail::Spaces),
            Pattern::AtRuleForwardEnd
            | Pattern::AtRuleUseEnd
            | Pattern::AtRuleWarnEnd
            | Pattern::AtRuleExtendEnd
            | Pattern::AtRuleReturnEnd
            | Pattern::AtRuleContentEnd => self.spaces_before(p, b";"),
            Pattern::AtRuleForwardAt0 => self.word_in(p, &[b"as", b"hide", b"show"]),
            Pattern::AtRuleForwardAt1 => self.at_rule_forward_at1(p),
            Pattern::AtRuleForwardAt2 | Pattern::AtRuleUseAt1 => self.at_rule_forward_at2(p),
            Pattern::AtRuleUse => self.at_keyword(p, b"use", Tail::Spaces),
            Pattern::AtRuleUseAt0 => self.word_in(p, &[b"as", b"with"]),
            Pattern::AtRuleUseAt2 | Pattern::TagWildcard => self.lit(p, b"*"),
            Pattern::AtRuleIncludeAt0 => self.at_rule_include_at0(p),
            Pattern::AtRuleIncludeAt1 => self.at_rule_include_at1(p),
            Pattern::AtRuleIncludeAt2 => self.at_bare_keyword(p, b"include"),
            Pattern::AtRuleImport => self.at_keyword(p, b"import", Tail::Spaces),
            Pattern::AtRuleImportEnd => self.spaces_before(p, b";}"),
            Pattern::FunctionsAt0 => self.scss_functions_at0(p),
            Pattern::FunctionsAt1 => self.dash_word_run(p),
            Pattern::AtRuleIf => self.at_keyword(p, b"if", Tail::Spaces),
            Pattern::AtRuleIfEnd
            | Pattern::AtRuleElseEnd
            | Pattern::AtRuleForEnd
            | Pattern::AtRuleMediaAt0End
            | Pattern::AtRuleFunctionAt0End
            | Pattern::AtRuleFontfaceAt0End
            | Pattern::AtRulePageAt0End
            | Pattern::AtRuleAtRootEnd => self.spaces_before(p, b"{"),
            Pattern::ComparisonOperators => self.comparison_operators(p),
            Pattern::LogicalOperators => self.word_in(p, &[b"not", b"or", b"and"]),
            Pattern::AtRuleElse => self.at_rule_else(p),
            Pattern::AtRuleWarn => self.at_rule_warn(p),
            Pattern::AtRuleFor => self.at_keyword(p, b"for", Tail::Spaces),
            Pattern::AtRuleForAt0 => self.at_rule_for_at0(p),
            Pattern::AtRuleWhile => self.at_keyword(p, b"while", Tail::Spaces),
            Pattern::AtRuleWhileEnd | Pattern::AtRuleEachEnd => self.spaces_before(p, b"}"),
            Pattern::AtRuleEach => self.at_keyword(p, b"each", Tail::Spaces),
            Pattern::AtRuleEachAt0 => self.at_rule_each_at0(p),
            Pattern::AtRuleReturn => self.at_keyword(p, b"return", Tail::None),
            Pattern::AtRuleExtend => self.at_keyword(p, b"extend", Tail::Spaces),
            Pattern::SelectorCustom => self.selector_custom(p),
            Pattern::Nested0At2 | Pattern::Nested1At2 => {
                matches!(self.at(p), b'$' | b'}').then_some(p + 1)
            }
            Pattern::SelectorClass => self.selector_name(p, Sigil::Class),
            Pattern::SelectorId => self.selector_name(p, Sigil::Id),
            Pattern::SelectorPseudoClassAt0 => self.selector_pseudo_class_at0(p),
            Pattern::SelectorPseudoClassAt0At1 => self.selector_pseudo_class_at0_at1(p),
            Pattern::SelectorPseudoClassAt0At2 => self.selector_pseudo_class_at0_at2(p),
            Pattern::SelectorPseudoClassAt0At3 => self.word_chars(p),
            Pattern::TagParentReference => self.lit(p, b"&"),
            Pattern::SelectorAttribute => self.selector_attribute(p),
            Pattern::SelectorPlaceholder => self.selector_name(p, Sigil::Placeholder),
            Pattern::ParentSelectorSuffix => self.parent_selector_suffix(p),
            Pattern::AtRuleContent => self.at_keyword(p, b"content", Tail::Spaces),
            Pattern::AtRuleMediaAt0 => self.line_at_keyword(p, b"media", Tail::None),
            Pattern::AtRuleMediaAt0At3 => self.keyword_out_of_line(p, b"only"),
            Pattern::AtRuleMediaAt0At4At0 | Pattern::PropertiesAt0 => {
                self.at_rule_media_at0_at4_at0(p)
            }
            Pattern::AtRuleMediaAt0At4At0End | Pattern::PropertiesAt0End => {
                self.at_rule_media_at0_at4_at0_end(p)
            }
            Pattern::AtRuleMediaAt0At4At1 | Pattern::PropertiesAt1 => {
                self.at_rule_media_at0_at4_at1(p)
            }
            Pattern::AtRuleMediaAt0At4At1End | Pattern::PropertiesAt1End => {
                self.at_rule_media_at0_at4_at1_end(p)
            }
            Pattern::PropertyList | Pattern::AtRuleKeyframesAt3 => self.lit(p, b"{"),
            Pattern::AtRuleMixinAt0 => self.at_rule_mixin_at0(p),
            Pattern::AtRuleMixinAt1 => self.at_rule_mixin_at1(p),
            Pattern::AtRuleMixinAt2 => self.at_bare_keyword(p, b"mixin"),
            Pattern::AtRuleFunctionAt0 | Pattern::AtRuleFunctionAt1 => {
                self.at_keyword(p, b"function", Tail::Spaces)
            }
            Pattern::AtRuleCharset => self.at_keyword(p, b"charset", Tail::Spaces),
            Pattern::AtRuleCharsetEnd => self.spaces_before_or_eol(p, b";"),
            Pattern::AtRuleOption => self.line_at_keyword(p, b"option", Tail::Spaces),
            Pattern::AtRuleNamespaceAt0 => self.at_rule_namespace_at0(p),
            Pattern::AtRuleNamespaceAt0End | Pattern::AtRuleNamespaceAt1End => {
                (self.at(p) == b';' || self.at_eol(p)).then_some(p)
            }
            Pattern::AtRuleNamespaceAt1 => self.at_rule_namespace_at1(p),
            Pattern::AtRuleNamespaceAt2 => self.at_bare_keyword(p, b"namespace"),
            Pattern::AtRuleFontfaceAt0 => self.line_at_keyword(p, b"font-face", Tail::None),
            Pattern::AtRulePageAt0 => self.at_rule_page_at0(p),
            Pattern::AtRuleKeyframes => self.at_rule_keyframes(p),
            Pattern::AtRuleKeyframesEnd => self.prev_is(p, b'}').then_some(p),
            Pattern::AtRuleKeyframesAt0 => self.at_rule_keyframes_at0(p),
            Pattern::AtRuleKeyframesAt1 => self.keyframes_quote(p, b'"'),
            Pattern::AtRuleKeyframesAt2 => self.keyframes_quote(p, b'\''),
            Pattern::AtRuleKeyframesAt3At0 => self.at_rule_keyframes_at3_at0(p),
            Pattern::AtRuleAtRoot => self.at_rule_at_root(p),
            Pattern::AtRuleSupports => self.at_rule_supports(p),
            Pattern::AtRuleSupportsEnd => (self.at(p) == b'{' || self.at_eol(p)).then_some(p),
            _ => None,
        }
    }
}
