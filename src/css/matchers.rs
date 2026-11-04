//! The matchers: for each regular expression of the grammar, the function that
//! decides whether it matches at a position and where.
//! A matcher records the capture groups the grammar names with
//! [`Lexer::cap`](crate::rules::Lexer::cap).
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
use super::grammar::Pat;
use crate::rules::Lexer;

impl<G: CssLike> Lexer<'_, G> {
    /// Whether CSS pattern `pat` matches at `p`, and where its match ends.
    /// It serves CSS, and SCSS, which includes CSS's rules.
    #[expect(
        clippy::too_many_lines,
        reason = "one arm for each expression of the grammar"
    )]
    #[inline]
    pub(crate) fn css_matcher(&mut self, pat: Pat, p: usize) -> Option<usize> {
        match pat {
            Pat::CommentBlock => self.comment_block(p),
            Pat::CommentBlockEnd => self.comment_block_end(p),
            Pat::EscapesAt0 => self.escapes_at0(p),
            Pat::EscapesAt1 => self.escapes_at1(p),
            Pat::EscapesAt1End => self.escapes_at1_end(p),
            Pat::EscapesAt2 => self.escapes_at2(p),
            Pat::CombinatorInvalid => self.combinator_invalid(p),
            Pat::Selector => self.selector(p),
            Pat::SelectorEnd => self.selector_end(p),
            Pat::Commas | Pat::LayerNameListAt1 => self.commas(p),
            Pat::CombinatorsAt1 => self.combinators_at1(p),
            Pat::SelectorInnardsAt4 => self.selector_innards_at4(p),
            Pat::TagNames => self.tag_names(p),
            Pat::SelectorInnardsAt6 => self.selector_innards_at6(p),
            Pat::SelectorInnardsAt7 => self.selector_innards_at7(p),
            Pat::SelectorInnardsAt8 => self.selector_innards_at8(p),
            Pat::SelectorInnardsAt9 => self.selector_innards_at9(p),
            Pat::SelectorInnardsAt10 => self.selector_innards_at10(p),
            Pat::SelectorInnardsAt11 => self.selector_innards_at11(p),
            Pat::SelectorInnardsAt11End => self.selector_innards_at11_end(p),
            Pat::StringAt0 | Pat::FunctionalPseudoClassesAt1At1 | Pat::AtRulesAt0At2 => {
                self.string_at0(p)
            }
            Pat::StringAt0End => self.string_at0_end(p),
            Pat::StringAt0At0 => self.string_at0_at0(p),
            Pat::StringAt0At0End | Pat::StringAt1At0End | Pat::AtRulesAt0At2At0End => {
                self.string_at0_at0_end(p)
            }
            Pat::StringAt1 | Pat::FunctionalPseudoClassesAt1At2 => self.string_at1(p),
            Pat::StringAt1End => self.string_at1_end(p),
            Pat::StringAt1At0 => self.string_at1_at0(p),
            Pat::SelectorInnardsAt11At2 => self.selector_innards_at11_at2(p),
            Pat::SelectorInnardsAt11At3 => self.selector_innards_at11_at3(p),
            Pat::SelectorInnardsAt11At5 => self.selector_innards_at11_at5(p),
            Pat::SelectorInnardsAt11At6 => self.selector_innards_at11_at6(p),
            Pat::SelectorInnardsAt11At7 => self.selector_innards_at11_at7(p),
            Pat::SelectorInnardsAt11At8 => self.selector_innards_at11_at8(p),
            Pat::PseudoClasses => self.pseudo_classes(p),
            Pat::PseudoElements => self.pseudo_elements(p),
            Pat::FunctionalPseudoClassesAt0 => self.functional_pseudo_classes_at0(p),
            Pat::FunctionalPseudoClassesAt0End
            | Pat::FunctionsAt0End
            | Pat::FunctionsAt1End
            | Pat::FunctionsAt2End
            | Pat::FunctionsAt3End
            | Pat::FunctionsAt3At0End
            | Pat::FunctionsAt4End
            | Pat::FunctionsAt5End
            | Pat::FunctionsAt6End
            | Pat::FunctionsAt7End
            | Pat::UrlEnd
            | Pat::FunctionsAt9End
            | Pat::FunctionsAt10End
            | Pat::FunctionNestingEnd
            | Pat::FunctionalPseudoClassesAt1End
            | Pat::FunctionalPseudoClassesAt2End
            | Pat::FunctionalPseudoClassesAt3End
            | Pat::LayerImportAt0End
            | Pat::MediaQueryAt8End
            | Pat::AtRulesAt6At0At0End
            | Pat::FeatureQueryEnd => self.functional_pseudo_classes_at0_end(p),
            Pat::FunctionalPseudoClassesAt0At2 => self.functional_pseudo_classes_at0_at2(p),
            Pat::PropertyValuesAt0
            | Pat::RuleList
            | Pat::FontFeaturesAt0At0
            | Pat::FontFeaturesAt1At0
            | Pat::FontFeaturesAt2At0
            | Pat::FontFeaturesAt3At0
            | Pat::FontFeaturesAt4At0
            | Pat::FontFeaturesAt5At0
            | Pat::AtRulesAt4At1
            | Pat::AtRulesAt5At1
            | Pat::AtRulesAt6At1
            | Pat::AtRulesAt7At1
            | Pat::AtRulesAt8At1
            | Pat::AtRulesAt9At1
            | Pat::AtRulesAt15At1 => self.property_values_at0(p),
            Pat::PropertyValuesAt0End
            | Pat::RuleListEnd
            | Pat::FontFeaturesAt0At0End
            | Pat::FontFeaturesAt1At0End
            | Pat::FontFeaturesAt2At0End
            | Pat::FontFeaturesAt3At0End
            | Pat::FontFeaturesAt4At0End
            | Pat::FontFeaturesAt5At0End
            | Pat::AtRulesAt4At1End
            | Pat::AtRulesAt5At1End
            | Pat::AtRulesAt6At1End
            | Pat::AtRulesAt7At1End
            | Pat::AtRulesAt8At1End
            | Pat::AtRulesAt9At1End
            | Pat::AtRulesAt15At1End => self.property_values_at0_end(p),
            Pat::FunctionsAt0 => self.functions_at0(p),
            Pat::ArithmeticOperators => self.arithmetic_operators(p),
            Pat::FunctionsAt1 => self.functions_at1(p),
            Pat::FunctionsAt2 => self.functions_at2(p),
            Pat::FunctionsAt2At0 => self.functions_at2_at0(p),
            Pat::FunctionsAt3 => self.functions_at3(p),
            Pat::FunctionsAt3At0 => self.functions_at3_at0(p),
            Pat::FunctionsAt4 => self.functions_at4(p),
            Pat::FunctionsAt4At0 => self.functions_at4_at0(p),
            Pat::FunctionsAt5 => self.functions_at5(p),
            Pat::FunctionsAt5At0 => self.functions_at5_at0(p),
            Pat::FunctionsAt6 => self.functions_at6(p),
            Pat::FunctionsAt6At0 => self.functions_at6_at0(p),
            Pat::FunctionsAt7 => self.functions_at7(p),
            Pat::Url => self.url(p),
            Pat::UrlAt0 | Pat::AtRulesAt6At0At0At3 => self.url_at0(p),
            Pat::FunctionsAt9 => self.functions_at9(p),
            Pat::FunctionsAt10 => self.functions_at10(p),
            Pat::FunctionsAt10At0 => self.functions_at10_at0(p),
            Pat::PropertyKeywordsAt0 => self.property_keywords_at0(p),
            Pat::PropertyKeywordsAt1 => self.property_keywords_at1(p),
            Pat::PropertyKeywordsAt2 | Pat::PropertyNamesAt1 => self.property_keywords_at2(p),
            Pat::PropertyKeywordsAt3 => self.property_keywords_at3(p),
            Pat::UnicodeRange => self.unicode_range(p),
            Pat::NumericValuesAt0 => self.numeric_values_at0(p),
            Pat::NumericValuesAt1 => self.numeric_percent(p),
            Pat::NumericValuesAt2 => self.numeric_unit(p),
            Pat::NumericValuesAt3 => self.numeric_bare(p),
            Pat::ColorKeywordsAt0 => self.color_keywords_at0(p),
            Pat::ColorKeywordsAt1 => self.color_keywords_at1(p),
            Pat::ColorKeywordsAt2 => self.color_keywords_at2(p),
            Pat::ColorKeywordsAt3 => self.color_keywords_at3(p),
            Pat::PropertyValuesAt10 => self.property_values_at10(p),
            Pat::FunctionsAt4At3 => self.functions_at4_at3(p),
            Pat::FunctionNesting | Pat::MediaQueryAt8 | Pat::FeatureQuery => {
                self.function_nesting(p)
            }
            Pat::FunctionalPseudoClassesAt1 => self.functional_pseudo_classes_at1(p),
            Pat::FunctionalPseudoClassesAt1At0 => self.functional_pseudo_classes_at1_at0(p),
            Pat::FunctionalPseudoClassesAt1At1End => self.functional_pseudo_classes_at1_at1_end(p),
            Pat::FunctionalPseudoClassesAt1At1At1 => self.functional_pseudo_classes_at1_at1_at1(p),
            Pat::FunctionalPseudoClassesAt1At2End => self.functional_pseudo_classes_at1_at2_end(p),
            Pat::FunctionalPseudoClassesAt1At2At1 => self.functional_pseudo_classes_at1_at2_at1(p),
            Pat::FunctionalPseudoClassesAt2 => self.functional_pseudo_classes_at2(p),
            Pat::FunctionalPseudoClassesAt3 => self.functional_pseudo_classes_at3(p),
            Pat::FunctionalPseudoClassesAt3At0 => self.functional_pseudo_classes_at3_at0(p),
            Pat::FunctionalPseudoClassesAt3At1 => self.functional_pseudo_classes_at3_at1(p),
            Pat::SelectorInnardsAt15 => self.selector_innards_at15(p),
            Pat::AtRulesAt0 => self.at_rules_at0(p),
            Pat::AtRulesAt0End => self.at_rules_at0_end(p),
            Pat::AtRulesAt0At0 => self.at_rules_at0_at0(p),
            Pat::AtRulesAt0At1 => self.at_rules_at0_at1(p),
            Pat::AtRulesAt0At2End => self.at_rules_at0_at2_end(p),
            Pat::AtRulesAt0At2At0 => self.at_rules_at0_at2_at0(p),
            Pat::AtRulesAt1 => self.at_rules_at1(p),
            Pat::AtRulesAt1End | Pat::AtRulesAt14At0End => self.at_rules_at1_end(p),
            Pat::AtRulesAt1At0 => self.at_rules_at1_at0(p),
            Pat::AtRulesAt1At0End => self.at_rules_at1_at0_end(p),
            Pat::AtRulesAt1At3 => self.at_rules_at1_at3(p),
            Pat::LayerImportAt0 => self.layer_import_at0(p),
            Pat::LayerNameListAt2 => self.layer_name_list_at2(p),
            Pat::LayerNameListAt3
            | Pat::FontFeaturesAt0At0At0
            | Pat::FontFeaturesAt1At0At0
            | Pat::FontFeaturesAt2At0At0
            | Pat::FontFeaturesAt3At0At0
            | Pat::FontFeaturesAt4At0At0
            | Pat::FontFeaturesAt5At0At0
            | Pat::AtRulesAt5At0At2
            | Pat::AtRulesAt7At0At2 => self.layer_name_list_at3(p),
            Pat::LayerImportAt1 => self.layer_import_at1(p),
            Pat::MediaQueryList => self.media_query_list(p),
            Pat::MediaQueryListEnd
            | Pat::MediaQueryEnd
            | Pat::AtRulesAt4At0End
            | Pat::AtRulesAt6At0End
            | Pat::AtRulesAt8At0End
            | Pat::AtRulesAt9At0End
            | Pat::AtRulesAt15At0End => self.media_query_list_end(p),
            Pat::MediaQuery => self.media_query(p),
            Pat::MediaTypes => self.media_types(p),
            Pat::MediaQueryAt3 => self.media_query_at3(p),
            Pat::MediaQueryAt4 => self.media_query_at4(p),
            Pat::MediaQueryAt5 => self.media_query_at5(p),
            Pat::MediaQueryAt6 => self.media_query_at6(p),
            Pat::MediaFeatures => self.media_features(p),
            Pat::MediaFeatureKeywords => self.media_feature_keywords(p),
            Pat::MediaQueryAt8At2 => self.media_query_at8_at2(p),
            Pat::MediaQueryAt8At3 => self.media_query_at8_at3(p),
            Pat::MediaQueryAt8At4 => self.media_query_at8_at4(p),
            Pat::AtRulesAt2 => self.at_rules_at2(p),
            Pat::AtRulesAt2End => self.at_rules_at2_end(p),
            Pat::FontFeaturesAt0 => self.font_features_at0(p),
            Pat::FontFeaturesAt0End
            | Pat::FontFeaturesAt1End
            | Pat::FontFeaturesAt2End
            | Pat::FontFeaturesAt3End
            | Pat::FontFeaturesAt4End
            | Pat::FontFeaturesAt5End => self.font_features_at0_end(p),
            Pat::FontFeaturesAt1 => self.font_features_at1(p),
            Pat::FontFeaturesAt2 => self.font_features_at2(p),
            Pat::FontFeaturesAt3 => self.font_features_at3(p),
            Pat::FontFeaturesAt4 => self.font_features_at4(p),
            Pat::FontFeaturesAt5 => self.font_features_at5(p),
            Pat::RuleListInnardsAt3 => self.rule_list_innards_at3(p),
            Pat::SharedNamesAt0 => self.shared_names_at0(p),
            Pat::SharedNamesAt0End | Pat::SharedNamesAt1End => self.shared_names_at0_end(p),
            Pat::SharedNamesAt1 => self.shared_names_at1(p),
            Pat::PropertyNamesAt0 => self.property_names_at0(p),
            Pat::RuleListInnardsAt7 => self.rule_list_innards_at7(p),
            Pat::RuleListInnardsAt7End => self.rule_list_innards_at7_end(p),
            Pat::RuleListInnardsAt8 => self.rule_list_innards_at8(p),
            Pat::RuleListInnardsAt8End => self.rule_list_innards_at8_end(p),
            Pat::RuleListInnardsAt9 => self.rule_list_innards_at9(p),
            Pat::AtRulesAt3 => self.at_rules_at3(p),
            Pat::AtRulesAt3End => self.at_rules_at3_end(p),
            Pat::AtRulesAt4 => self.at_rules_at4(p),
            Pat::AtRulesAt4End
            | Pat::AtRulesAt5End
            | Pat::AtRulesAt6End
            | Pat::AtRulesAt7End
            | Pat::AtRulesAt15End => self.at_rules_at4_end(p),
            Pat::AtRulesAt4At0 => self.at_rules_at4_at0(p),
            Pat::AtRulesAt5 => self.at_rules_at5(p),
            Pat::AtRulesAt5At0 => self.at_rules_at5_at0(p),
            Pat::AtRulesAt5At0End | Pat::AtRulesAt7At0End => self.at_rules_at5_at0_end(p),
            Pat::AtRulesAt6 => self.at_rules_at6(p),
            Pat::AtRulesAt6At0 => self.at_rules_at6_at0(p),
            Pat::AtRulesAt6At0At0 => self.at_rules_at6_at0_at0(p),
            Pat::AtRulesAt7 => self.at_rules_at7(p),
            Pat::AtRulesAt7At0 => self.at_rules_at7_at0(p),
            Pat::AtRulesAt7At1At2 => self.at_rules_at7_at1_at2(p),
            Pat::AtRulesAt8 => self.at_rules_at8(p),
            Pat::AtRulesAt8End => self.at_rules_at8_end(p),
            Pat::AtRulesAt8At0 => self.at_rules_at8_at0(p),
            Pat::FeatureQueryOperatorsAt0 => self.feature_query_operators_at0(p),
            Pat::FeatureQueryOperatorsAt1 => self.feature_query_operators_at1(p),
            Pat::FeatureQueryOperatorsAt2 => self.feature_query_operators_at2(p),
            Pat::AtRulesAt9 => self.at_rules_at9(p),
            Pat::AtRulesAt9End => self.at_rules_at9_end(p),
            Pat::AtRulesAt9At0 => self.at_rules_at9_at0(p),
            Pat::AtRulesAt10 => self.at_rules_at10(p),
            Pat::AtRulesAt10End | Pat::AtRulesAt11End => self.at_rules_at10_end(p),
            Pat::AtRulesAt11 => self.at_rules_at11(p),
            Pat::AtRulesAt13 => self.at_rules_at13(p),
            Pat::AtRulesAt13End => self.at_rules_at13_end(p),
            Pat::AtRulesAt13At1 => self.at_rules_at13_at1(p),
            Pat::AtRulesAt14 => self.at_rules_at14(p),
            Pat::AtRulesAt14End => self.at_rules_at14_end(p),
            Pat::AtRulesAt14At0 | Pat::AtRulesAt15At0 => self.at_rules_at14_at0(p),
            Pat::AtRulesAt15 => self.at_rules_at15(p),
        }
    }
}
