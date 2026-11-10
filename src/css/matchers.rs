//! The matchers: for each regular expression of the grammar, the function that
//! decides whether it matches at a position and where.
//! A matcher records the capture groups the grammar names with
//! [`Lexer::capture`](crate::rules::Lexer::capture).
//! The engine names them.
//!
//! Each follows its expression as an Oniguruma-style backtracking matcher
//! would.
//! Where a quantifier could give characters back, the doc comment says why it
//! cannot matter.

mod atrules;
mod chars;
mod media;
mod selectors;
mod strings;
mod values;

use super::engine::CssLike;
use super::grammar::Pattern;
use crate::rules::Lexer;

impl<G: CssLike> Lexer<'_, G> {
    /// Whether CSS pattern `pattern` matches at `p`, and where its match ends.
    /// It serves CSS, and SCSS, which includes CSS's rules.
    #[expect(
        clippy::too_many_lines,
        reason = "one arm for each expression of the grammar"
    )]
    #[inline]
    pub(crate) fn css_matcher(&mut self, pattern: Pattern, p: usize) -> Option<usize> {
        match pattern {
            Pattern::CommentBlock => self.comment_block(p),
            Pattern::CommentBlockEnd => self.comment_block_end(p),
            Pattern::EscapesAt0 => self.escapes_at0(p),
            Pattern::EscapesAt1 => self.escapes_at1(p),
            Pattern::EscapesAt1End => self.escapes_at1_end(p),
            Pattern::EscapesAt2 => self.escapes_at2(p),
            Pattern::CombinatorInvalid => self.combinator_invalid(p),
            Pattern::Selector => self.selector(p),
            Pattern::SelectorEnd => self.selector_end(p),
            Pattern::Commas | Pattern::LayerNameListAt1 => self.commas(p),
            Pattern::CombinatorsAt1 => self.combinators_at1(p),
            Pattern::SelectorInnardsAt4 => self.selector_innards_at4(p),
            Pattern::TagNames => self.tag_names(p),
            Pattern::SelectorInnardsAt6 => self.selector_innards_at6(p),
            Pattern::SelectorInnardsAt7 => self.selector_innards_at7(p),
            Pattern::SelectorInnardsAt8 => self.selector_innards_at8(p),
            Pattern::SelectorInnardsAt9 => self.selector_innards_at9(p),
            Pattern::SelectorInnardsAt10 => self.selector_innards_at10(p),
            Pattern::SelectorInnardsAt11 => self.selector_innards_at11(p),
            Pattern::SelectorInnardsAt11End => self.selector_innards_at11_end(p),
            Pattern::StringAt0
            | Pattern::FunctionalPseudoClassesAt1At1
            | Pattern::AtRulesAt0At2 => self.string_at0(p),
            Pattern::StringAt0End => self.string_at0_end(p),
            Pattern::StringAt0At0 => self.string_at0_at0(p),
            Pattern::StringAt0At0End | Pattern::StringAt1At0End | Pattern::AtRulesAt0At2At0End => {
                self.string_at0_at0_end(p)
            }
            Pattern::StringAt1 | Pattern::FunctionalPseudoClassesAt1At2 => self.string_at1(p),
            Pattern::StringAt1End => self.string_at1_end(p),
            Pattern::StringAt1At0 => self.string_at1_at0(p),
            Pattern::SelectorInnardsAt11At2 => self.selector_innards_at11_at2(p),
            Pattern::SelectorInnardsAt11At3 => self.selector_innards_at11_at3(p),
            Pattern::SelectorInnardsAt11At5 => self.selector_innards_at11_at5(p),
            Pattern::SelectorInnardsAt11At6 => self.selector_innards_at11_at6(p),
            Pattern::SelectorInnardsAt11At7 => self.selector_innards_at11_at7(p),
            Pattern::SelectorInnardsAt11At8 => self.selector_innards_at11_at8(p),
            Pattern::PseudoClasses => self.pseudo_classes(p),
            Pattern::PseudoElements => self.pseudo_elements(p),
            Pattern::FunctionalPseudoClassesAt0 => self.functional_pseudo_classes_at0(p),
            Pattern::FunctionalPseudoClassesAt0End
            | Pattern::FunctionsAt0End
            | Pattern::FunctionsAt1End
            | Pattern::FunctionsAt2End
            | Pattern::FunctionsAt3End
            | Pattern::FunctionsAt3At0End
            | Pattern::FunctionsAt4End
            | Pattern::FunctionsAt5End
            | Pattern::FunctionsAt6End
            | Pattern::FunctionsAt7End
            | Pattern::UrlEnd
            | Pattern::FunctionsAt9End
            | Pattern::FunctionsAt10End
            | Pattern::FunctionNestingEnd
            | Pattern::FunctionalPseudoClassesAt1End
            | Pattern::FunctionalPseudoClassesAt2End
            | Pattern::FunctionalPseudoClassesAt3End
            | Pattern::LayerImportAt0End
            | Pattern::MediaQueryAt8End
            | Pattern::AtRulesAt6At0At0End
            | Pattern::FeatureQueryEnd => self.functional_pseudo_classes_at0_end(p),
            Pattern::FunctionalPseudoClassesAt0At2 => self.functional_pseudo_classes_at0_at2(p),
            Pattern::PropertyValuesAt0
            | Pattern::RuleList
            | Pattern::FontFeaturesAt0At0
            | Pattern::FontFeaturesAt1At0
            | Pattern::FontFeaturesAt2At0
            | Pattern::FontFeaturesAt3At0
            | Pattern::FontFeaturesAt4At0
            | Pattern::FontFeaturesAt5At0
            | Pattern::AtRulesAt4At1
            | Pattern::AtRulesAt5At1
            | Pattern::AtRulesAt6At1
            | Pattern::AtRulesAt7At1
            | Pattern::AtRulesAt8At1
            | Pattern::AtRulesAt9At1
            | Pattern::AtRulesAt15At1 => self.property_values_at0(p),
            Pattern::PropertyValuesAt0End
            | Pattern::RuleListEnd
            | Pattern::FontFeaturesAt0At0End
            | Pattern::FontFeaturesAt1At0End
            | Pattern::FontFeaturesAt2At0End
            | Pattern::FontFeaturesAt3At0End
            | Pattern::FontFeaturesAt4At0End
            | Pattern::FontFeaturesAt5At0End
            | Pattern::AtRulesAt4At1End
            | Pattern::AtRulesAt5At1End
            | Pattern::AtRulesAt6At1End
            | Pattern::AtRulesAt7At1End
            | Pattern::AtRulesAt8At1End
            | Pattern::AtRulesAt9At1End
            | Pattern::AtRulesAt15At1End => self.property_values_at0_end(p),
            Pattern::FunctionsAt0 => self.functions_at0(p),
            Pattern::ArithmeticOperators => self.arithmetic_operators(p),
            Pattern::FunctionsAt1 => self.functions_at1(p),
            Pattern::FunctionsAt2 => self.functions_at2(p),
            Pattern::FunctionsAt2At0 => self.functions_at2_at0(p),
            Pattern::FunctionsAt3 => self.functions_at3(p),
            Pattern::FunctionsAt3At0 => self.functions_at3_at0(p),
            Pattern::FunctionsAt4 => self.functions_at4(p),
            Pattern::FunctionsAt4At0 => self.functions_at4_at0(p),
            Pattern::FunctionsAt5 => self.functions_at5(p),
            Pattern::FunctionsAt5At0 => self.functions_at5_at0(p),
            Pattern::FunctionsAt6 => self.functions_at6(p),
            Pattern::FunctionsAt6At0 => self.functions_at6_at0(p),
            Pattern::FunctionsAt7 => self.functions_at7(p),
            Pattern::Url => self.url(p),
            Pattern::UrlAt0 | Pattern::AtRulesAt6At0At0At3 => self.url_at0(p),
            Pattern::FunctionsAt9 => self.functions_at9(p),
            Pattern::FunctionsAt10 => self.functions_at10(p),
            Pattern::FunctionsAt10At0 => self.functions_at10_at0(p),
            Pattern::PropertyKeywordsAt0 => self.property_keywords_at0(p),
            Pattern::PropertyKeywordsAt1 => self.property_keywords_at1(p),
            Pattern::PropertyKeywordsAt2 | Pattern::PropertyNamesAt1 => {
                self.property_keywords_at2(p)
            }
            Pattern::PropertyKeywordsAt3 => self.property_keywords_at3(p),
            Pattern::UnicodeRange => self.unicode_range(p),
            Pattern::NumericValuesAt0 => self.numeric_values_at0(p),
            Pattern::NumericValuesAt1 => self.numeric_percent(p),
            Pattern::NumericValuesAt2 => self.numeric_unit(p),
            Pattern::NumericValuesAt3 => self.numeric_bare(p),
            Pattern::ColorKeywordsAt0 => self.color_keywords_at0(p),
            Pattern::ColorKeywordsAt1 => self.color_keywords_at1(p),
            Pattern::ColorKeywordsAt2 => self.color_keywords_at2(p),
            Pattern::ColorKeywordsAt3 => self.color_keywords_at3(p),
            Pattern::PropertyValuesAt10 => self.property_values_at10(p),
            Pattern::FunctionsAt4At3 => self.functions_at4_at3(p),
            Pattern::FunctionNesting | Pattern::MediaQueryAt8 | Pattern::FeatureQuery => {
                self.function_nesting(p)
            }
            Pattern::FunctionalPseudoClassesAt1 => self.functional_pseudo_classes_at1(p),
            Pattern::FunctionalPseudoClassesAt1At0 => self.functional_pseudo_classes_at1_at0(p),
            Pattern::FunctionalPseudoClassesAt1At1End => {
                self.functional_pseudo_classes_at1_at1_end(p)
            }
            Pattern::FunctionalPseudoClassesAt1At1At1 => {
                self.functional_pseudo_classes_at1_at1_at1(p)
            }
            Pattern::FunctionalPseudoClassesAt1At2End => {
                self.functional_pseudo_classes_at1_at2_end(p)
            }
            Pattern::FunctionalPseudoClassesAt1At2At1 => {
                self.functional_pseudo_classes_at1_at2_at1(p)
            }
            Pattern::FunctionalPseudoClassesAt2 => self.functional_pseudo_classes_at2(p),
            Pattern::FunctionalPseudoClassesAt3 => self.functional_pseudo_classes_at3(p),
            Pattern::FunctionalPseudoClassesAt3At0 => self.functional_pseudo_classes_at3_at0(p),
            Pattern::FunctionalPseudoClassesAt3At1 => self.functional_pseudo_classes_at3_at1(p),
            Pattern::SelectorInnardsAt15 => self.selector_innards_at15(p),
            Pattern::AtRulesAt0 => self.at_rules_at0(p),
            Pattern::AtRulesAt0End => self.at_rules_at0_end(p),
            Pattern::AtRulesAt0At0 => self.at_rules_at0_at0(p),
            Pattern::AtRulesAt0At1 => self.at_rules_at0_at1(p),
            Pattern::AtRulesAt0At2End => self.at_rules_at0_at2_end(p),
            Pattern::AtRulesAt0At2At0 => self.at_rules_at0_at2_at0(p),
            Pattern::AtRulesAt1 => self.at_rules_at1(p),
            Pattern::AtRulesAt1End | Pattern::AtRulesAt14At0End => self.at_rules_at1_end(p),
            Pattern::AtRulesAt1At0 => self.at_rules_at1_at0(p),
            Pattern::AtRulesAt1At0End => self.at_rules_at1_at0_end(p),
            Pattern::AtRulesAt1At3 => self.at_rules_at1_at3(p),
            Pattern::LayerImportAt0 => self.layer_import_at0(p),
            Pattern::LayerNameListAt2 => self.layer_name_list_at2(p),
            Pattern::LayerNameListAt3
            | Pattern::FontFeaturesAt0At0At0
            | Pattern::FontFeaturesAt1At0At0
            | Pattern::FontFeaturesAt2At0At0
            | Pattern::FontFeaturesAt3At0At0
            | Pattern::FontFeaturesAt4At0At0
            | Pattern::FontFeaturesAt5At0At0
            | Pattern::AtRulesAt5At0At2
            | Pattern::AtRulesAt7At0At2 => self.layer_name_list_at3(p),
            Pattern::LayerImportAt1 => self.layer_import_at1(p),
            Pattern::MediaQueryList => self.media_query_list(p),
            Pattern::MediaQueryListEnd
            | Pattern::MediaQueryEnd
            | Pattern::AtRulesAt4At0End
            | Pattern::AtRulesAt6At0End
            | Pattern::AtRulesAt8At0End
            | Pattern::AtRulesAt9At0End
            | Pattern::AtRulesAt15At0End => self.media_query_list_end(p),
            Pattern::MediaQuery => self.media_query(p),
            Pattern::MediaTypes => self.media_types(p),
            Pattern::MediaQueryAt3 => self.media_query_at3(p),
            Pattern::MediaQueryAt4 => self.media_query_at4(p),
            Pattern::MediaQueryAt5 => self.media_query_at5(p),
            Pattern::MediaQueryAt6 => self.media_query_at6(p),
            Pattern::MediaFeatures => self.media_features(p),
            Pattern::MediaFeatureKeywords => self.media_feature_keywords(p),
            Pattern::MediaQueryAt8At2 => self.media_query_at8_at2(p),
            Pattern::MediaQueryAt8At3 => self.media_query_at8_at3(p),
            Pattern::MediaQueryAt8At4 => self.media_query_at8_at4(p),
            Pattern::AtRulesAt2 => self.at_rules_at2(p),
            Pattern::AtRulesAt2End => self.at_rules_at2_end(p),
            Pattern::FontFeaturesAt0 => self.font_features_at0(p),
            Pattern::FontFeaturesAt0End
            | Pattern::FontFeaturesAt1End
            | Pattern::FontFeaturesAt2End
            | Pattern::FontFeaturesAt3End
            | Pattern::FontFeaturesAt4End
            | Pattern::FontFeaturesAt5End => self.font_features_at0_end(p),
            Pattern::FontFeaturesAt1 => self.font_features_at1(p),
            Pattern::FontFeaturesAt2 => self.font_features_at2(p),
            Pattern::FontFeaturesAt3 => self.font_features_at3(p),
            Pattern::FontFeaturesAt4 => self.font_features_at4(p),
            Pattern::FontFeaturesAt5 => self.font_features_at5(p),
            Pattern::RuleListInnardsAt3 => self.rule_list_innards_at3(p),
            Pattern::SharedNamesAt0 => self.shared_names_at0(p),
            Pattern::SharedNamesAt0End | Pattern::SharedNamesAt1End => self.shared_names_at0_end(p),
            Pattern::SharedNamesAt1 => self.shared_names_at1(p),
            Pattern::PropertyNamesAt0 => self.property_names_at0(p),
            Pattern::RuleListInnardsAt7 => self.rule_list_innards_at7(p),
            Pattern::RuleListInnardsAt7End => self.rule_list_innards_at7_end(p),
            Pattern::RuleListInnardsAt8 => self.rule_list_innards_at8(p),
            Pattern::RuleListInnardsAt8End => self.rule_list_innards_at8_end(p),
            Pattern::RuleListInnardsAt9 => self.rule_list_innards_at9(p),
            Pattern::AtRulesAt3 => self.at_rules_at3(p),
            Pattern::AtRulesAt3End => self.at_rules_at3_end(p),
            Pattern::AtRulesAt4 => self.at_rules_at4(p),
            Pattern::AtRulesAt4End
            | Pattern::AtRulesAt5End
            | Pattern::AtRulesAt6End
            | Pattern::AtRulesAt7End
            | Pattern::AtRulesAt15End => self.at_rules_at4_end(p),
            Pattern::AtRulesAt4At0 => self.at_rules_at4_at0(p),
            Pattern::AtRulesAt5 => self.at_rules_at5(p),
            Pattern::AtRulesAt5At0 => self.at_rules_at5_at0(p),
            Pattern::AtRulesAt5At0End | Pattern::AtRulesAt7At0End => self.at_rules_at5_at0_end(p),
            Pattern::AtRulesAt6 => self.at_rules_at6(p),
            Pattern::AtRulesAt6At0 => self.at_rules_at6_at0(p),
            Pattern::AtRulesAt6At0At0 => self.at_rules_at6_at0_at0(p),
            Pattern::AtRulesAt7 => self.at_rules_at7(p),
            Pattern::AtRulesAt7At0 => self.at_rules_at7_at0(p),
            Pattern::AtRulesAt7At1At2 => self.at_rules_at7_at1_at2(p),
            Pattern::AtRulesAt8 => self.at_rules_at8(p),
            Pattern::AtRulesAt8End => self.at_rules_at8_end(p),
            Pattern::AtRulesAt8At0 => self.at_rules_at8_at0(p),
            Pattern::FeatureQueryOperatorsAt0 => self.feature_query_operators_at0(p),
            Pattern::FeatureQueryOperatorsAt1 => self.feature_query_operators_at1(p),
            Pattern::FeatureQueryOperatorsAt2 => self.feature_query_operators_at2(p),
            Pattern::AtRulesAt9 => self.at_rules_at9(p),
            Pattern::AtRulesAt9End => self.at_rules_at9_end(p),
            Pattern::AtRulesAt9At0 => self.at_rules_at9_at0(p),
            Pattern::AtRulesAt10 => self.at_rules_at10(p),
            Pattern::AtRulesAt10End | Pattern::AtRulesAt11End => self.at_rules_at10_end(p),
            Pattern::AtRulesAt11 => self.at_rules_at11(p),
            Pattern::AtRulesAt13 => self.at_rules_at13(p),
            Pattern::AtRulesAt13End => self.at_rules_at13_end(p),
            Pattern::AtRulesAt13At1 => self.at_rules_at13_at1(p),
            Pattern::AtRulesAt14 => self.at_rules_at14(p),
            Pattern::AtRulesAt14End => self.at_rules_at14_end(p),
            Pattern::AtRulesAt14At0 | Pattern::AtRulesAt15At0 => self.at_rules_at14_at0(p),
            Pattern::AtRulesAt15 => self.at_rules_at15(p),
        }
    }
}
