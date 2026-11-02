//! The matchers: for each regular expression of the grammar, the function that
//! decides whether it matches at a position and where. A matcher records the
//! capture groups the grammar names with [`Lexer::cap`]; the engine names them.
//!
//! Each follows its expression as an Oniguruma-style backtracking matcher
//! would. Where a quantifier could give characters back, the doc comment says
//! why it cannot matter.

mod atrules;
mod chars;
mod media;
mod selectors;
mod strings;
mod values;

use super::engine::Lexer;
use super::grammar::Pat;

impl Lexer<'_> {
    /// Whether `pat` matches at `p`, and where its match ends.
    #[expect(
        clippy::too_many_lines,
        reason = "one arm for each expression of the grammar"
    )]
    pub(super) fn matcher(&mut self, pat: Pat, p: usize) -> Option<usize> {
        match pat {
            // begin: /\*
            Pat::CommentBlock => self.comment_block(p),
            // end: \*/
            Pat::CommentBlockEnd => self.comment_block_end(p),
            // match: \\\h{1,6}
            Pat::EscapesAt0 => self.escapes_at0(p),
            // begin: \\$\s*
            Pat::EscapesAt1 => self.escapes_at1(p),
            // end: ^(?<!\G)
            Pat::EscapesAt1End => self.escapes_at1_end(p),
            // match: \\.
            Pat::EscapesAt2 => self.escapes_at2(p),
            // match: /deep/|>>>
            Pat::CombinatorInvalid => self.combinator_invalid(p),
            // begin: (?i)(?<=^|(^|[^:])\s|[;{}]|\*/|\\(?:\h{1,6}|.))(?=[\&*
            // +>A-\[a-z|~[^\x00-\x7F]]|\\(?:\h{1,6}|.)|[#.](
            Pat::Selector => self.selector(p),
            // end: (?=\s*(?:[)/@{]|$))
            Pat::SelectorEnd => self.selector_end(p),
            // match: ,
            Pat::Commas | Pat::LayerNameListAt1 => self.commas(p),
            // match: >>|[+>~]
            Pat::CombinatorsAt1 => self.combinators_at1(p),
            // match: (?:^|(?<=[(,;}\s]))(?![-*\w]+\|(?![-#\&*.:A-\[_a-z[^\x00-\
            // x7F]]))([-A-Z_a-z[^\x00-\x7F]](?:[-0-9A-Z_
            Pat::SelectorInnardsAt4 => self.selector_innards_at4(p),
            // match: (?i)(?<![-:\w])(?:
            // a|abbr|acronym|address|applet|area|article|aside|audio|b|base|basefont|bdi|bdo|bgs
            Pat::TagNames => self.tag_names(p),
            // match: &
            Pat::SelectorInnardsAt6 => self.selector_innards_at6(p),
            // match: \*
            Pat::SelectorInnardsAt7 => self.selector_innards_at7(p),
            // match: (?<![-@\w])([#.])((?:-?[0-9]|-(?=$|[#\&)+,.:>\[{|~\s]|/\*
            // )|(?:[-0-9A-Z_a-z[^\x00-\x7F]]|\\(?:\h{1,6}
            Pat::SelectorInnardsAt8 => self.selector_innards_at8(p),
            // match: (\.)((?:[-0-9A-Z_a-z[^\x00-\x7F]]|\\(?:\h{1,6}|.))+)(?
            // =$|[#\&)+,.:>\[{|~\s]|/\*)
            Pat::SelectorInnardsAt9 => self.selector_innards_at9(p),
            // match: (#)(-?(?![0-9])(?:[-0-9A-Z_a-z[^\x00-\x7F]]|\\(?:\h{1,6}|.
            // ))+)(?=$|[#\&)+,.:>\[{|~\s]|/\*)
            Pat::SelectorInnardsAt10 => self.selector_innards_at10(p),
            // begin: \[
            Pat::SelectorInnardsAt11 => self.selector_innards_at11(p),
            // end: ]
            Pat::SelectorInnardsAt11End => self.selector_innards_at11_end(p),
            // begin: "
            Pat::StringAt0 | Pat::FunctionalPseudoClassesAt1At1 | Pat::AtRulesAt0At2 => {
                self.string_at0(p)
            }
            // end: "|(?<!\\)(?=$|\n)
            Pat::StringAt0End => self.string_at0_end(p),
            // begin: (?:\G|^)(?=(?:[^"\\]|\\.)+$)
            Pat::StringAt0At0 => self.string_at0_at0(p),
            // end: $
            Pat::StringAt0At0End | Pat::StringAt1At0End | Pat::AtRulesAt0At2At0End => {
                self.string_at0_at0_end(p)
            }
            // begin: '
            Pat::StringAt1 | Pat::FunctionalPseudoClassesAt1At2 => self.string_at1(p),
            // end: '|(?<!\\)(?=$|\n)
            Pat::StringAt1End => self.string_at1_end(p),
            // begin: (?:\G|^)(?=(?:[^'\\]|\\.)+$)
            Pat::StringAt1At0 => self.string_at1_at0(p),
            // match: (?<=["'\s]|^|\*/)\s*([Ii])\s*(?=[]\s]|/\*|$)
            Pat::SelectorInnardsAt11At2 => self.selector_innards_at11_at2(p),
            // match: (?<==)\s*((?!/\*)(?:[^]"'\\\s]|\\.)+)
            Pat::SelectorInnardsAt11At3 => self.selector_innards_at11_at3(p),
            // match: [$*^|~]?=
            Pat::SelectorInnardsAt11At5 => self.selector_innards_at11_at5(p),
            // match: \|
            Pat::SelectorInnardsAt11At6 => self.selector_innards_at11_at6(p),
            // match: (-?(?!\d)(?:[-\w[^\x00-\x7F]]|\\(?:\h{1,6}|.))+|\*)(?=\|(?
            // ![=\s]|$|])(?:-?(?!\d)|[-\\\w[^\x00-\x7F]]
            Pat::SelectorInnardsAt11At7 => self.selector_innards_at11_at7(p),
            // match: (-?(?!\d)(?>[-\w[^\x00-\x7F]]|\\(?:\h{1,6}|.))+)\s*(?=[]$*
            // =^|~]|/\*)
            Pat::SelectorInnardsAt11At8 => self.selector_innards_at11_at8(p),
            // match: (?i)(:)(:*)(?:
            // active|any-link|checked|default|disabled|empty|enabled|first|(?:
            // first|last|only)-(?:ch
            Pat::PseudoClasses => self.pseudo_classes(p),
            // match: (?i)(?:(::?)(?:after|before|first-letter|first-line|(?:-(?
            // :ah|apple|atsc|epub|hp|khtml|moz|ms|o|rim|
            Pat::PseudoElements => self.pseudo_elements(p),
            // begin: (?i)((:)dir)(\()
            Pat::FunctionalPseudoClassesAt0 => self.functional_pseudo_classes_at0(p),
            // end: \)
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
            // match: (?i)(?<![-\w])(ltr|rtl)(?![-\w])
            Pat::FunctionalPseudoClassesAt0At2 => self.functional_pseudo_classes_at0_at2(p),
            // begin: \{
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
            // end: }
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
            // begin: (?i)(?<![-\w])(calc)(\()
            Pat::FunctionsAt0 => self.functions_at0(p),
            // match: [*/]|(?<=\s|^)[-+](?=\s|$)
            Pat::ArithmeticOperators => self.arithmetic_operators(p),
            // begin: (?i)(?<![-\w])(rgba?|hsla?
            // |hwb|lab|oklab|lch|oklch|color)(\()
            Pat::FunctionsAt1 => self.functions_at1(p),
            // begin: (?i)(?<![-\w])((?:-(?:webkit-|moz-|o-))?(?:repeating-)?(?:
            // linear|radial|conic)-gradient)(\()
            Pat::FunctionsAt2 => self.functions_at2(p),
            // match: (?i)(?<![-\w])(from|to|at|in|hue)(?![-\w])
            Pat::FunctionsAt2At0 => self.functions_at2_at0(p),
            // begin: (?i)(?<![-\w])(-webkit-gradient)(\()
            Pat::FunctionsAt3 => self.functions_at3(p),
            // begin: (?i)(?<![-\w])(from|to|color-stop)(\()
            Pat::FunctionsAt3At0 => self.functions_at3_at0(p),
            // begin: (?i)(?<![-\
            // w])(annotation|attr|blur|brightness|character-variant|clamp|contrast|counters?
            // |cross-fade
            Pat::FunctionsAt4 => self.functions_at4(p),
            // match: (?i)(?<=[",\s]|\*/|^)\d+x(?=["'),\s]|/\*|$)
            Pat::FunctionsAt4At0 => self.functions_at4_at0(p),
            // begin: (?i)(?<![-\w])(circle|ellipse|inset|polygon|rect)(\()
            Pat::FunctionsAt5 => self.functions_at5(p),
            // match: (?i)(?<=\s|^|\*/)(at|round)(?=\s|/\*|$)
            Pat::FunctionsAt5At0 => self.functions_at5_at0(p),
            // begin: (?i)(?<![-\w])(cubic-bezier|steps)(\()
            Pat::FunctionsAt6 => self.functions_at6(p),
            // match: (?i)(?<![-\w])(start|end)(?=\s*\)|$)
            Pat::FunctionsAt6At0 => self.functions_at6_at0(p),
            // begin: (?i)(?<![-\w])((?:translate|scale|rotate)(?:[XYZ]|3D)?
            // |matrix(?:3D)?|skew[XY]?|perspective)(\()
            Pat::FunctionsAt7 => self.functions_at7(p),
            // begin: (?i)(?<![-@\w])(url)(\()
            Pat::Url => self.url(p),
            // match: [^"')\s]+
            Pat::UrlAt0 | Pat::AtRulesAt6At0At0At3 => self.url_at0(p),
            // begin: (?<![-\w])(--[-A-Z_a-z[^\x00-\x7F]](?:[-0-9A-Z_a-z[^\x00-\
            // x7F]]|\\(?:\h{1,6}|.))*)(\()
            Pat::FunctionsAt9 => self.functions_at9(p),
            // begin: (?i)(?<![-\w])(var)(\()
            Pat::FunctionsAt10 => self.functions_at10(p),
            // match: --[-A-Z_a-z[^\x00-\x7F]](?:[-0-9A-Z_a-z[^\x00-\x7F]]|\\(?:
            // \h{1,6}|.))*
            Pat::FunctionsAt10At0 => self.functions_at10_at0(p),
            // match: (?i)(?<![-\
            // w])(above|absolute|active|add|additive|after-edge|alias|all|all-petite-caps|all-scroll|al
            Pat::PropertyKeywordsAt0 => self.property_keywords_at0(p),
            // match: (?i)(?<![-\
            // w])(arabic-indic|armenian|bengali|cambodian|circle|cjk-decimal|cjk-earthly-branch|cjk-hea
            Pat::PropertyKeywordsAt1 => self.property_keywords_at1(p),
            // match: (?<![-\w])(?i:-(?:
            // ah|apple|atsc|epub|hp|khtml|moz|ms|o|rim|ro|tc|wap|webkit|xv)|(?:
            // mso|prince))-[-A-
            Pat::PropertyKeywordsAt2 | Pat::PropertyNamesAt1 => self.property_keywords_at2(p),
            // match: (?<![-\w])(?i:
            // arial|century|comic|courier|garamond|georgia|helvetica|impact|lucida|symbol|system-ui|
            Pat::PropertyKeywordsAt3 => self.property_keywords_at3(p),
            // match: (?<![-\w])[Uu]\+[?\h]{1,6}(?:(-)\h{1,6})?(?![-\w])
            Pat::UnicodeRange => self.unicode_range(p),
            // match: (#)(?:\h{3,4}|\h{6}|\h{8})\b
            Pat::NumericValuesAt0 => self.numeric_values_at0(p),
            // ?<=[0-9])E[-+]?[0-9]+)?(?:(%))
            Pat::NumericValuesAt1 => self.numeric_percent(p),
            // 0-9])E[-+]?[0-9]+)?(?:(deg)\b)
            Pat::NumericValuesAt2 => self.numeric_unit(p, "deg"),
            // -9])E[-+]?[0-9]+)?(?:(grad)\b)
            Pat::NumericValuesAt3 => self.numeric_unit(p, "grad"),
            // 0-9])E[-+]?[0-9]+)?(?:(rad)\b)
            Pat::NumericValuesAt4 => self.numeric_unit(p, "rad"),
            // -9])E[-+]?[0-9]+)?(?:(turn)\b)
            Pat::NumericValuesAt5 => self.numeric_unit(p, "turn"),
            // [0-9])E[-+]?[0-9]+)?(?:(Hz)\b)
            Pat::NumericValuesAt6 => self.numeric_unit(p, "hz"),
            // 0-9])E[-+]?[0-9]+)?(?:(kHz)\b)
            Pat::NumericValuesAt7 => self.numeric_unit(p, "khz"),
            // [0-9])E[-+]?[0-9]+)?(?:(ch)\b)
            Pat::NumericValuesAt8 => self.numeric_unit(p, "ch"),
            // [0-9])E[-+]?[0-9]+)?(?:(cm)\b)
            Pat::NumericValuesAt9 => self.numeric_unit(p, "cm"),
            // [0-9])E[-+]?[0-9]+)?(?:(em)\b)
            Pat::NumericValuesAt10 => self.numeric_unit(p, "em"),
            // [0-9])E[-+]?[0-9]+)?(?:(ex)\b)
            Pat::NumericValuesAt11 => self.numeric_unit(p, "ex"),
            // [0-9])E[-+]?[0-9]+)?(?:(fr)\b)
            Pat::NumericValuesAt12 => self.numeric_unit(p, "fr"),
            // [0-9])E[-+]?[0-9]+)?(?:(in)\b)
            Pat::NumericValuesAt13 => self.numeric_unit(p, "in"),
            // [0-9])E[-+]?[0-9]+)?(?:(mm)\b)
            Pat::NumericValuesAt14 => self.numeric_unit(p, "mm"),
            // 9])E[-+]?[0-9]+)?(?:(mozmm)\b)
            Pat::NumericValuesAt15 => self.numeric_unit(p, "mozmm"),
            // [0-9])E[-+]?[0-9]+)?(?:(pc)\b)
            Pat::NumericValuesAt16 => self.numeric_unit(p, "pc"),
            // [0-9])E[-+]?[0-9]+)?(?:(pt)\b)
            Pat::NumericValuesAt17 => self.numeric_unit(p, "pt"),
            // [0-9])E[-+]?[0-9]+)?(?:(px)\b)
            Pat::NumericValuesAt18 => self.numeric_unit(p, "px"),
            // =[0-9])E[-+]?[0-9]+)?(?:(q)\b)
            Pat::NumericValuesAt19 => self.numeric_unit(p, "q"),
            // 0-9])E[-+]?[0-9]+)?(?:(rem)\b)
            Pat::NumericValuesAt20 => self.numeric_unit(p, "rem"),
            // 0-9])E[-+]?[0-9]+)?(?:(rch)\b)
            Pat::NumericValuesAt21 => self.numeric_unit(p, "rch"),
            // 0-9])E[-+]?[0-9]+)?(?:(rex)\b)
            Pat::NumericValuesAt22 => self.numeric_unit(p, "rex"),
            // 0-9])E[-+]?[0-9]+)?(?:(rlh)\b)
            Pat::NumericValuesAt23 => self.numeric_unit(p, "rlh"),
            // [0-9])E[-+]?[0-9]+)?(?:(ic)\b)
            Pat::NumericValuesAt24 => self.numeric_unit(p, "ic"),
            // 0-9])E[-+]?[0-9]+)?(?:(ric)\b)
            Pat::NumericValuesAt25 => self.numeric_unit(p, "ric"),
            // -9])E[-+]?[0-9]+)?(?:(rcap)\b)
            Pat::NumericValuesAt26 => self.numeric_unit(p, "rcap"),
            // [0-9])E[-+]?[0-9]+)?(?:(vh)\b)
            Pat::NumericValuesAt27 => self.numeric_unit(p, "vh"),
            // [0-9])E[-+]?[0-9]+)?(?:(vw)\b)
            Pat::NumericValuesAt28 => self.numeric_unit(p, "vw"),
            // [0-9])E[-+]?[0-9]+)?(?:(vb)\b)
            Pat::NumericValuesAt29 => self.numeric_unit(p, "vb"),
            // [0-9])E[-+]?[0-9]+)?(?:(vi)\b)
            Pat::NumericValuesAt30 => self.numeric_unit(p, "vi"),
            // 0-9])E[-+]?[0-9]+)?(?:(svh)\b)
            Pat::NumericValuesAt31 => self.numeric_unit(p, "svh"),
            // 0-9])E[-+]?[0-9]+)?(?:(svw)\b)
            Pat::NumericValuesAt32 => self.numeric_unit(p, "svw"),
            // 0-9])E[-+]?[0-9]+)?(?:(svb)\b)
            Pat::NumericValuesAt33 => self.numeric_unit(p, "svb"),
            // 0-9])E[-+]?[0-9]+)?(?:(svi)\b)
            Pat::NumericValuesAt34 => self.numeric_unit(p, "svi"),
            // 0-9])E[-+]?[0-9]+)?(?:(dvh)\b)
            Pat::NumericValuesAt35 => self.numeric_unit(p, "dvh"),
            // 0-9])E[-+]?[0-9]+)?(?:(dvw)\b)
            Pat::NumericValuesAt36 => self.numeric_unit(p, "dvw"),
            // 0-9])E[-+]?[0-9]+)?(?:(dvb)\b)
            Pat::NumericValuesAt37 => self.numeric_unit(p, "dvb"),
            // 0-9])E[-+]?[0-9]+)?(?:(dvi)\b)
            Pat::NumericValuesAt38 => self.numeric_unit(p, "dvi"),
            // 0-9])E[-+]?[0-9]+)?(?:(lvh)\b)
            Pat::NumericValuesAt39 => self.numeric_unit(p, "lvh"),
            // 0-9])E[-+]?[0-9]+)?(?:(lvw)\b)
            Pat::NumericValuesAt40 => self.numeric_unit(p, "lvw"),
            // 0-9])E[-+]?[0-9]+)?(?:(lvb)\b)
            Pat::NumericValuesAt41 => self.numeric_unit(p, "lvb"),
            // 0-9])E[-+]?[0-9]+)?(?:(lvi)\b)
            Pat::NumericValuesAt42 => self.numeric_unit(p, "lvi"),
            // -9])E[-+]?[0-9]+)?(?:(vmax)\b)
            Pat::NumericValuesAt43 => self.numeric_unit(p, "vmax"),
            // -9])E[-+]?[0-9]+)?(?:(vmin)\b)
            Pat::NumericValuesAt44 => self.numeric_unit(p, "vmin"),
            // 0-9])E[-+]?[0-9]+)?(?:(cqw)\b)
            Pat::NumericValuesAt45 => self.numeric_unit(p, "cqw"),
            // 0-9])E[-+]?[0-9]+)?(?:(cqi)\b)
            Pat::NumericValuesAt46 => self.numeric_unit(p, "cqi"),
            // 0-9])E[-+]?[0-9]+)?(?:(cqh)\b)
            Pat::NumericValuesAt47 => self.numeric_unit(p, "cqh"),
            // 0-9])E[-+]?[0-9]+)?(?:(cqb)\b)
            Pat::NumericValuesAt48 => self.numeric_unit(p, "cqb"),
            // 9])E[-+]?[0-9]+)?(?:(cqmin)\b)
            Pat::NumericValuesAt49 => self.numeric_unit(p, "cqmin"),
            // 9])E[-+]?[0-9]+)?(?:(cqmax)\b)
            Pat::NumericValuesAt50 => self.numeric_unit(p, "cqmax"),
            // 0-9])E[-+]?[0-9]+)?(?:(dpi)\b)
            Pat::NumericValuesAt51 => self.numeric_unit(p, "dpi"),
            // -9])E[-+]?[0-9]+)?(?:(dpcm)\b)
            Pat::NumericValuesAt52 => self.numeric_unit(p, "dpcm"),
            // -9])E[-+]?[0-9]+)?(?:(dppx)\b)
            Pat::NumericValuesAt53 => self.numeric_unit(p, "dppx"),
            // =[0-9])E[-+]?[0-9]+)?(?:(s)\b)
            Pat::NumericValuesAt54 => self.numeric_unit(p, "s"),
            // [0-9])E[-+]?[0-9]+)?(?:(ms)\b)
            Pat::NumericValuesAt55 => self.numeric_unit(p, "ms"),
            // ]+)(?:(?<=[0-9])E[-+]?[0-9]+)?
            Pat::NumericValuesAt56 => self.numeric_bare(p),
            // match: (?i)(?<![-\
            // w])(aqua|black|blue|fuchsia|gray|green|lime|maroon|navy|olive|orange|purple|red|silver|te
            Pat::ColorKeywordsAt0 => self.color_keywords_at0(p),
            // match: (?i)(?<![-\
            // w])(aliceblue|antiquewhite|aquamarine|azure|beige|bisque|blanchedalmond|blueviolet|brown|
            Pat::ColorKeywordsAt1 => self.color_keywords_at1(p),
            // match: (?i)(?<![-\w])currentColor(?![-\w])
            Pat::ColorKeywordsAt2 => self.color_keywords_at2(p),
            // match: (?i)(?<![-\
            // w])(ActiveBorder|ActiveCaption|AppWorkspace|Background|ButtonFace|ButtonHighlight|ButtonS
            Pat::ColorKeywordsAt3 => self.color_keywords_at3(p),
            // match: !\s*important(?![-\w])
            Pat::PropertyValuesAt10 => self.property_values_at10(p),
            // match: [^"'),\s]+
            Pat::FunctionsAt4At3 => self.functions_at4_at3(p),
            // begin: \(
            Pat::FunctionNesting | Pat::MediaQueryAt8 | Pat::FeatureQuery => {
                self.function_nesting(p)
            }
            // begin: (?i)((:)lang)(\()
            Pat::FunctionalPseudoClassesAt1 => self.functional_pseudo_classes_at1(p),
            // match: (?<=[(,\s])[A-Za-z]+(-[0-9A-Za-z]*|\\(?:\h{1,6}|.))*(?=[),
            // \s])
            Pat::FunctionalPseudoClassesAt1At0 => self.functional_pseudo_classes_at1_at0(p),
            // end: "
            Pat::FunctionalPseudoClassesAt1At1End => self.functional_pseudo_classes_at1_at1_end(p),
            // match: (?<=["\s])[*A-Za-z]+(-[*0-9A-Za-z]*)*(?=["\s])
            Pat::FunctionalPseudoClassesAt1At1At1 => self.functional_pseudo_classes_at1_at1_at1(p),
            // end: '
            Pat::FunctionalPseudoClassesAt1At2End => self.functional_pseudo_classes_at1_at2_end(p),
            // match: (?<=['\s])[*A-Za-z]+(-[*0-9A-Za-z]*)*(?=['\s])
            Pat::FunctionalPseudoClassesAt1At2At1 => self.functional_pseudo_classes_at1_at2_at1(p),
            // begin: (?i)((:)(?:not|has|matches|where|is))(\()
            Pat::FunctionalPseudoClassesAt2 => self.functional_pseudo_classes_at2(p),
            // begin: (?i)((:)nth-(?:last-)?(?:child|of-type))(\()
            Pat::FunctionalPseudoClassesAt3 => self.functional_pseudo_classes_at3(p),
            // match: (?i)[-+]?(\d+n?|n)(\s*[-+]\s*\d+)?
            Pat::FunctionalPseudoClassesAt3At0 => self.functional_pseudo_classes_at3_at0(p),
            // match: (?i)even|odd
            Pat::FunctionalPseudoClassesAt3At1 => self.functional_pseudo_classes_at3_at1(p),
            // match: (?<![-@\w])(?=[a-z]\w*-)(?:(?![A-Z])[-\w])+(?![-(\w])
            Pat::SelectorInnardsAt15 => self.selector_innards_at15(p),
            // begin: \A\uFEFF?(?i:(?=\s*@charset\b))
            Pat::AtRulesAt0 => self.at_rules_at0(p),
            // end: ;|(?=$)
            Pat::AtRulesAt0End => self.at_rules_at0_end(p),
            // match: \G((?!@charset)@\w+)|\G(\s+)|(@charset\S[^;]*)|(?<=@
            // charset)( {2,}|\t+)|(?<=@charset )([^";]+)|("[^"
            Pat::AtRulesAt0At0 => self.at_rules_at0_at0(p),
            // match: ((@)charset)(?=\s)
            Pat::AtRulesAt0At1 => self.at_rules_at0_at1(p),
            // end: "|$
            Pat::AtRulesAt0At2End => self.at_rules_at0_at2_end(p),
            // begin: (?:\G|^)(?=[^"]+$)
            Pat::AtRulesAt0At2At0 => self.at_rules_at0_at2_at0(p),
            // begin: (?i)((@)import)(?:\s+|$|(?=["']|/\*))
            Pat::AtRulesAt1 => self.at_rules_at1(p),
            // end: ;
            Pat::AtRulesAt1End | Pat::AtRulesAt14At0End => self.at_rules_at1_end(p),
            // begin: \G\s*(?=/\*)
            Pat::AtRulesAt1At0 => self.at_rules_at1_at0(p),
            // end: (?<=\*/)\s*
            Pat::AtRulesAt1At0End => self.at_rules_at1_at0_end(p),
            // match: \s+
            Pat::AtRulesAt1At3 => self.at_rules_at1_at3(p),
            // begin: (?i)(?<![-\w])(layer)(\()
            Pat::LayerImportAt0 => self.layer_import_at0(p),
            // match: \.
            Pat::LayerNameListAt2 => self.layer_name_list_at2(p),
            // match: [-A-Z_a-z[^\x00-\x7F]](?:[-0-9A-Z_a-z[^\x00-\x7F]]|\\(?:\
            // h{1,6}|.))*
            Pat::LayerNameListAt3
            | Pat::FontFeaturesAt0At0At0
            | Pat::FontFeaturesAt1At0At0
            | Pat::FontFeaturesAt2At0At0
            | Pat::FontFeaturesAt3At0At0
            | Pat::FontFeaturesAt4At0At0
            | Pat::FontFeaturesAt5At0At0
            | Pat::AtRulesAt5At0At2
            | Pat::AtRulesAt7At0At2 => self.layer_name_list_at3(p),
            // match: (?i)(?<![-\w])(layer)(?![-\w])
            Pat::LayerImportAt1 => self.layer_import_at1(p),
            // begin: (?=\s*[^;{])
            Pat::MediaQueryList => self.media_query_list(p),
            // end: (?=\s*[;{])
            Pat::MediaQueryListEnd
            | Pat::MediaQueryEnd
            | Pat::AtRulesAt4At0End
            | Pat::AtRulesAt6At0End
            | Pat::AtRulesAt8At0End
            | Pat::AtRulesAt9At0End
            | Pat::AtRulesAt15At0End => self.media_query_list_end(p),
            // begin: \G
            Pat::MediaQuery => self.media_query(p),
            // match: (?i)(?<=^|[,\s]|\*/)(?:
            // (all|print|screen|speech)|(aural|braille|embossed|handheld|projection|tty|tv)
            Pat::MediaTypes => self.media_types(p),
            // match: (?i)(?<=\s|^|,|\*/)(only)(?=[{\s]|/\*|$)
            Pat::MediaQueryAt3 => self.media_query_at3(p),
            // match: (?i)(?<=\s|^|,|\*/)(not)(?=[{\s]|/\*|$)
            Pat::MediaQueryAt4 => self.media_query_at4(p),
            // match: (?i)(?<=\s|^|\*/|\))and(?=\s|/\*|$)
            Pat::MediaQueryAt5 => self.media_query_at5(p),
            // match: ,(?:(?:\s*,)+|(?=\s*[);{]))
            Pat::MediaQueryAt6 => self.media_query_at6(p),
            // match: (?i)(?<=^|[(\s]|\*/)(?:((?:m(?:in-|ax-))?(?:
            // height|width|aspect-ratio|color|color-index|monochrome|r
            Pat::MediaFeatures => self.media_features(p),
            // match: (?i)(?<=^|[:\s]|\*/)(?:
            // portrait|landscape|progressive|interlace|fullscreen|standalone|minimal-ui|bro
            Pat::MediaFeatureKeywords => self.media_feature_keywords(p),
            // match: :
            Pat::MediaQueryAt8At2 => self.media_query_at8_at2(p),
            // match: >=|<=|[<=>]
            Pat::MediaQueryAt8At3 => self.media_query_at8_at3(p),
            // match: (\d+)\s*(/)\s*(\d+)
            Pat::MediaQueryAt8At4 => self.media_query_at8_at4(p),
            // begin: (?i)((@)font-face)(?=\s*|\{|/\*|$)
            Pat::AtRulesAt2 => self.at_rules_at2(p),
            // end: (?!\G)
            Pat::AtRulesAt2End => self.at_rules_at2_end(p),
            // begin: (?i)((@)(annotation))(?=["';@{\s]|/\*|$)
            Pat::FontFeaturesAt0 => self.font_features_at0(p),
            // end: (?<=})
            Pat::FontFeaturesAt0End
            | Pat::FontFeaturesAt1End
            | Pat::FontFeaturesAt2End
            | Pat::FontFeaturesAt3End
            | Pat::FontFeaturesAt4End
            | Pat::FontFeaturesAt5End => self.font_features_at0_end(p),
            // begin: (?i)((@)(character-variant))(?=["';@{\s]|/\*|$)
            Pat::FontFeaturesAt1 => self.font_features_at1(p),
            // begin: (?i)((@)(ornaments))(?=["';@{\s]|/\*|$)
            Pat::FontFeaturesAt2 => self.font_features_at2(p),
            // begin: (?i)((@)(styleset))(?=["';@{\s]|/\*|$)
            Pat::FontFeaturesAt3 => self.font_features_at3(p),
            // begin: (?i)((@)(stylistic))(?=["';@{\s]|/\*|$)
            Pat::FontFeaturesAt4 => self.font_features_at4(p),
            // begin: (?i)((@)(swash))(?=["';@{\s]|/\*|$)
            Pat::FontFeaturesAt5 => self.font_features_at5(p),
            // match: (?<![-\w])--[-A-Z_a-z[^\x00-\x7F]](?:[-0-9A-Z_a-z[^\x00-\
            // x7F]]|\\(?:\h{1,6}|.))*
            Pat::RuleListInnardsAt3 => self.rule_list_innards_at3(p),
            // begin: (?i)(?<=^|(^|[^:])\s|[{}]|\*/|\\(?:\h{1,6}|.))(?=(?:
            // header|image|label|marquee|mask|nav|ruby|shadow|
            Pat::SharedNamesAt0 => self.shared_names_at0(p),
            // end: (?i)(?=\s|/\*|[,{]|$)
            Pat::SharedNamesAt0End | Pat::SharedNamesAt1End => self.shared_names_at0_end(p),
            // begin: (?i)(?<=^|(^|[^:])\s|[{}]|\*/|\\(?:\h{1,6}|.))(?=(?:
            // content|font|mark|cursor|filter)(?!.*;|[^{]*}|-|
            Pat::SharedNamesAt1 => self.shared_names_at1(p),
            // match: (?i)(?<![-\w])(?:
            // accent-color|additive-symbols|align-content|align-items|align-self|all|animation|an
            Pat::PropertyNamesAt0 => self.property_names_at0(p),
            // begin: (?<![-A-Za-z])(?=[-A-Za-z])
            Pat::RuleListInnardsAt7 => self.rule_list_innards_at7(p),
            // end: $|(?![-A-Za-z])
            Pat::RuleListInnardsAt7End => self.rule_list_innards_at7_end(p),
            // begin: (:)\s*
            Pat::RuleListInnardsAt8 => self.rule_list_innards_at8(p),
            // end: \s*(;)|\s*(?=[)}])
            Pat::RuleListInnardsAt8End => self.rule_list_innards_at8_end(p),
            // match: ;
            Pat::RuleListInnardsAt9 => self.rule_list_innards_at9(p),
            // begin: (?i)(@)page(?=[:{\s]|/\*|$)
            Pat::AtRulesAt3 => self.at_rules_at3(p),
            // end: (?=\s*($|[:;{]))
            Pat::AtRulesAt3End => self.at_rules_at3_end(p),
            // begin: (?i)(?=@media([(\s]|/\*|$))
            Pat::AtRulesAt4 => self.at_rules_at4(p),
            // end: (?<=})(?!\G)
            Pat::AtRulesAt4End
            | Pat::AtRulesAt5End
            | Pat::AtRulesAt6End
            | Pat::AtRulesAt7End
            | Pat::AtRulesAt15End => self.at_rules_at4_end(p),
            // begin: (?i)\G(@)media
            Pat::AtRulesAt4At0 => self.at_rules_at4_at0(p),
            // begin: (?i)(?=@counter-style(["';{\s]|/\*|$))
            Pat::AtRulesAt5 => self.at_rules_at5(p),
            // begin: (?i)\G(@)counter-style
            Pat::AtRulesAt5At0 => self.at_rules_at5_at0(p),
            // end: (?=\s*\{)
            Pat::AtRulesAt5At0End | Pat::AtRulesAt7At0End => self.at_rules_at5_at0_end(p),
            // begin: (?i)(?=@document(["';{\s]|/\*|$))
            Pat::AtRulesAt6 => self.at_rules_at6(p),
            // begin: (?i)\G(@)document
            Pat::AtRulesAt6At0 => self.at_rules_at6_at0(p),
            // begin: (?i)(?<![-\w])(url-prefix|domain|regexp)(\()
            Pat::AtRulesAt6At0At0 => self.at_rules_at6_at0_at0(p),
            // begin: (?i)(?=@(?:-(?:webkit|moz|o|ms)-)?keyframes(["';{\s]|/\*
            // |$))
            Pat::AtRulesAt7 => self.at_rules_at7(p),
            // begin: (?i)\G(@)(?:-(?:webkit|moz|o|ms)-)?keyframes
            Pat::AtRulesAt7At0 => self.at_rules_at7_at0(p),
            // match: (?i)(?<![-\w])(from|to)(?![-\w])|([-+]?(?:\d+(?:\.\d+)?|\.
            // \d+)%)
            Pat::AtRulesAt7At1At2 => self.at_rules_at7_at1_at2(p),
            // begin: (?i)(?=@supports([(\s]|/\*|$))
            Pat::AtRulesAt8 => self.at_rules_at8(p),
            // end: (?<=})(?!\G)|(?=;)
            Pat::AtRulesAt8End => self.at_rules_at8_end(p),
            // begin: (?i)\G(@)supports
            Pat::AtRulesAt8At0 => self.at_rules_at8_at0(p),
            // match: (?i)(?<=[()\s]|^|\*/)(and)(?=[()\s]|/\*|$)
            Pat::FeatureQueryOperatorsAt0 => self.feature_query_operators_at0(p),
            // match: (?i)(?<=[()\s]|^|\*/)(not)(?=[()\s]|/\*|$)
            Pat::FeatureQueryOperatorsAt1 => self.feature_query_operators_at1(p),
            // match: (?i)(?<=[()\s]|^|\*/)(or)(?=[()\s]|/\*|$)
            Pat::FeatureQueryOperatorsAt2 => self.feature_query_operators_at2(p),
            // begin: (?i)(?=@layer(?:[;{\s]|/\*|$))
            Pat::AtRulesAt9 => self.at_rules_at9(p),
            // end: (?<=})(?!\G)|;
            Pat::AtRulesAt9End => self.at_rules_at9_end(p),
            // begin: (?i)\G(@)layer
            Pat::AtRulesAt9At0 => self.at_rules_at9_at0(p),
            // begin: (?i)((@)(-(ms|o)-)?viewport)(?=["';{\s]|/\*|$)
            Pat::AtRulesAt10 => self.at_rules_at10(p),
            // end: (?=\s*[;@{])
            Pat::AtRulesAt10End | Pat::AtRulesAt11End => self.at_rules_at10_end(p),
            // begin: (?i)((@)font-feature-values)(?=["';{\s]|/\*|$)\s*
            Pat::AtRulesAt11 => self.at_rules_at11(p),
            // begin: (?i)((@)namespace)(?=["';\s]|/\*|$)
            Pat::AtRulesAt13 => self.at_rules_at13(p),
            // end: ;|(?=[@{])
            Pat::AtRulesAt13End => self.at_rules_at13_end(p),
            // match: (?i)(?:\G|^|(?<=\s))(?=(?<=\s|^)[-A-Z_a-z[^\x00-\x7F]]|\s*
            // /\*(?:[^*]|\*[^/])*\*/)(.*?)([-A-Z_a-z[^\x
            Pat::AtRulesAt13At1 => self.at_rules_at13_at1(p),
            // begin: (?i)(?=@[-\w]+[^;]+;s*$)
            Pat::AtRulesAt14 => self.at_rules_at14(p),
            // end: (?<=;)(?!\G)
            Pat::AtRulesAt14End => self.at_rules_at14_end(p),
            // begin: (?i)\G(@)[-\w]+
            Pat::AtRulesAt14At0 | Pat::AtRulesAt15At0 => self.at_rules_at14_at0(p),
            // begin: (?i)(?=@[-\w]+([({\s]|/\*|$))
            Pat::AtRulesAt15 => self.at_rules_at15(p),
        }
    }
}
