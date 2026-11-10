//! The matchers: for each regular expression of the grammar, the function that
//! decides whether it matches at a position and where. A matcher records the
//! capture groups the grammar names with
//! [`Lexer::cap`](crate::rules::Lexer::cap); the engine names them.
//!
//! Each follows its expression as an Oniguruma-style backtracking matcher
//! would. Where a quantifier could give characters back, the doc comment says
//! why it cannot matter, or the matcher tries the shorter matches.

mod atrules;
mod chars;
mod names;
mod selectors;
mod values;

use atrules::Tail;
pub(super) use chars::{Class, Escape};
use values::Spaces;

use super::engine::Lexer;
use super::grammar::Pattern;
use super::words;
use crate::case_fold;
use crate::rules::Chars;

/// The longest word of any table in [`words`], in bytes; `oracle/less-words.ts`
/// fails on a longer one.
const LONGEST_WORD: usize = 64;

/// The word of `table` that `key` is, and whether a longer word of the
/// table starts with `key`. `table` is sorted, so such words sort right
/// after `key`.
fn find(table: &[Word], key: &[u8]) -> (Option<Word>, bool) {
    let (word, next) = match table.binary_search_by(|word| word.text.as_bytes().cmp(key)) {
        Ok(index) => (table.get(index).copied(), index + 1),
        Err(index) => (None, index),
    };
    let goes_on = table
        .get(next)
        .is_some_and(|word| word.text.as_bytes().starts_with(key));
    (word, goes_on)
}

/// A word of one of the grammar's big alternations: its text, its place in
/// the alternation and whether it ignores case.
#[derive(Clone, Copy, Debug)]
pub(super) struct Word {
    pub(super) text: &'static str,
    pub(super) order: u16,
    pub(super) folded: bool,
}

impl Lexer<'_> {
    /// Whether pattern `pattern` matches at `p`, and where its match ends.
    ///
    /// Each arm is a test of the bytes at `p` or a single call.
    /// A single call compiles to a tail jump.
    /// The function then saves no registers on entry.
    /// An arm that needs more work calls an out-of-line helper.
    #[expect(
        clippy::too_many_lines,
        reason = "one arm for each expression of the grammar"
    )]
    #[inline]
    pub(super) fn less_matcher(&mut self, pattern: Pattern, p: usize) -> Option<usize> {
        match pattern {
            Pattern::CommentBlockAt0 => self.lit(p, b"/*"),
            Pattern::CommentBlockAt0End => self.lit(p, b"*/"),
            Pattern::CommentLine => self.comment_line(p),
            Pattern::LessNamespaceAccessorsAt0 => self.when_ahead(p),
            Pattern::LessNamespaceAccessorsAt0End
            | Pattern::MediaQueryEnd
            | Pattern::KeyframeNameEnd => self.comma_or_block(p),
            Pattern::LessNamespaceAccessorsAt0At0 => self.when(p),
            Pattern::LessMixinGuardsAt0 => self.guard_operator(p),
            Pattern::LessMixinGuardsAt0End
            | Pattern::AttrFunctionEnd
            | Pattern::CubicBezierFunctionEnd
            | Pattern::LessBooleanFunctionEnd
            | Pattern::LessLogicalComparisonsAt1End
            | Pattern::CalcFunctionEnd
            | Pattern::VarFunctionEnd
            | Pattern::ColorFunctionsAt0End
            | Pattern::ColorFunctionsAt1End
            | Pattern::ColorFunctionsAt2End
            | Pattern::LessColorBlendFunctionsAt0End
            | Pattern::LessColorChannelFunctionsAt0End
            | Pattern::LessColorDefinitionFunctionsAt0End
            | Pattern::LessColorDefinitionFunctionsAt1End
            | Pattern::LessColorOperationFunctionsAt0End
            | Pattern::LessColorOperationFunctionsAt1End
            | Pattern::LessColorOperationFunctionsAt2End
            | Pattern::LessColorOperationFunctionsAt3End
            | Pattern::LessColorOperationFunctionsAt4End
            | Pattern::LessColorOperationFunctionsAt5End
            | Pattern::LessIfFunctionEnd
            | Pattern::CounterFunctionsAt0End
            | Pattern::CounterFunctionsAt1End
            | Pattern::CrossFadeFunctionAt0End
            | Pattern::LessListFunctionsAt0End
            | Pattern::FilterFunctionEnd
            | Pattern::GradientFunctionsAt0End
            | Pattern::LessListFunctionsAt1End
            | Pattern::FitContentFunctionEnd
            | Pattern::FormatFunctionAt0End
            | Pattern::GradientFunctionsAt1End
            | Pattern::LessListFunctionsAt2End
            | Pattern::GridRepeatFunctionEnd
            | Pattern::MinmaxFunctionEnd
            | Pattern::ImageFunctionEnd
            | Pattern::UrlFunctionEnd
            | Pattern::LessMathFunctionsAt0End
            | Pattern::LessMathFunctionsAt2End
            | Pattern::LessMiscFunctionsAt0End
            | Pattern::LessMiscFunctionsAt1End
            | Pattern::LessMiscFunctionsAt2End
            | Pattern::LessMiscFunctionsAt3End
            | Pattern::LessMiscFunctionsAt5End
            | Pattern::LessMiscFunctionsAt6End
            | Pattern::LessStringFunctionsAt0End
            | Pattern::LessStringFunctionsAt1End
            | Pattern::LessStringFunctionsAt2End
            | Pattern::LessTypeFunctionsAt0End
            | Pattern::LessTypeFunctionsAt1End
            | Pattern::LessTypeFunctionsAt2End
            | Pattern::LocalFunctionEnd
            | Pattern::RegexpFunctionEnd
            | Pattern::ShapeFunctionsAt0End
            | Pattern::ShapeFunctionsAt1End
            | Pattern::ShapeFunctionsAt2End
            | Pattern::ShapeFunctionsAt3End
            | Pattern::StepsFunctionEnd
            | Pattern::SymbolsFunctionEnd
            | Pattern::TransformFunctionsAt0End
            | Pattern::TransformFunctionsAt1End
            | Pattern::TransformFunctionsAt2End
            | Pattern::TransformFunctionsAt3End
            | Pattern::TransformFunctionsAt4End
            | Pattern::TransformFunctionsAt5End
            | Pattern::TransformFunctionsAt6End
            | Pattern::TransformFunctionsAt7End
            | Pattern::LessMathAt1End
            | Pattern::FilterFunctionsAt1End
            | Pattern::FilterFunctionsAt2End
            | Pattern::FilterFunctionsAt3End
            | Pattern::FilterFunctionsAt4End
            | Pattern::PseudoSelectorsAt0End
            | Pattern::PseudoSelectorsAt1End
            | Pattern::PseudoSelectorsAt2End
            | Pattern::PseudoSelectorsAt3End
            | Pattern::PseudoSelectorsAt4End
            | Pattern::PseudoSelectorsAt6End
            | Pattern::PseudoSelectorsAt7End
            | Pattern::LessExtendEnd
            | Pattern::AtContainerAt0At0At1End
            | Pattern::StyleFunctionEnd
            | Pattern::MediaQueryAt2End
            | Pattern::AtImportAt3End
            | Pattern::AtSupportsParensEnd
            | Pattern::ScrollFunctionEnd
            | Pattern::ViewFunctionEnd => self.lit(p, b")"),
            Pattern::LessMixinGuardsAt0At0
            | Pattern::AttrFunctionAt0
            | Pattern::LessBooleanFunctionAt0
            | Pattern::LessLogicalComparisonsAt1
            | Pattern::CalcFunctionAt0
            | Pattern::VarFunctionAt0
            | Pattern::ColorFunctionsAt0At0
            | Pattern::ColorFunctionsAt1At0
            | Pattern::ColorFunctionsAt2At0
            | Pattern::LessColorBlendFunctionsAt0At0
            | Pattern::LessColorChannelFunctionsAt0At0
            | Pattern::LessColorDefinitionFunctionsAt0At0
            | Pattern::LessColorDefinitionFunctionsAt1At0
            | Pattern::LessColorOperationFunctionsAt0At0
            | Pattern::LessColorOperationFunctionsAt1At0
            | Pattern::LessColorOperationFunctionsAt2At0
            | Pattern::LessColorOperationFunctionsAt3At0
            | Pattern::LessColorOperationFunctionsAt4At0
            | Pattern::LessColorOperationFunctionsAt5At0
            | Pattern::LessIfFunctionAt0
            | Pattern::CounterFunctionsAt0At0
            | Pattern::CounterFunctionsAt1At0
            | Pattern::CrossFadeFunctionAt0At0
            | Pattern::LessListFunctionsAt0At0
            | Pattern::FilterFunctionAt0
            | Pattern::GradientFunctionsAt0At0
            | Pattern::LessListFunctionsAt1At0
            | Pattern::FitContentFunctionAt0
            | Pattern::FormatFunctionAt0At0
            | Pattern::GradientFunctionsAt1At0
            | Pattern::LessListFunctionsAt2At0
            | Pattern::GridRepeatFunctionAt0
            | Pattern::MinmaxFunctionAt0
            | Pattern::ImageFunctionAt0
            | Pattern::UrlFunctionAt0
            | Pattern::LessMathFunctionsAt0At0
            | Pattern::LessMathFunctionsAt2At0
            | Pattern::LessMiscFunctionsAt0At0
            | Pattern::LessMiscFunctionsAt1At0
            | Pattern::LessMiscFunctionsAt2At0
            | Pattern::LessMiscFunctionsAt3At0
            | Pattern::LessMiscFunctionsAt5At0
            | Pattern::LessMiscFunctionsAt6At0
            | Pattern::LessStringFunctionsAt0At0
            | Pattern::LessStringFunctionsAt1At0
            | Pattern::LessStringFunctionsAt2At0
            | Pattern::LessTypeFunctionsAt0At0
            | Pattern::LessTypeFunctionsAt1At0
            | Pattern::LessTypeFunctionsAt2At0
            | Pattern::LocalFunctionAt0
            | Pattern::RegexpFunctionAt0
            | Pattern::ShapeFunctionsAt0At0
            | Pattern::ShapeFunctionsAt1At0
            | Pattern::ShapeFunctionsAt2At0
            | Pattern::ShapeFunctionsAt3At0
            | Pattern::SymbolsFunctionAt0
            | Pattern::TransformFunctionsAt0At0
            | Pattern::TransformFunctionsAt1At0
            | Pattern::TransformFunctionsAt2At0
            | Pattern::TransformFunctionsAt3At0
            | Pattern::TransformFunctionsAt4At0
            | Pattern::TransformFunctionsAt5At0
            | Pattern::TransformFunctionsAt6At0
            | Pattern::TransformFunctionsAt7At0
            | Pattern::LessMathAt1
            | Pattern::FilterFunctionsAt1At0
            | Pattern::FilterFunctionsAt2At0
            | Pattern::FilterFunctionsAt3At0
            | Pattern::FilterFunctionsAt4At0
            | Pattern::PseudoSelectorsAt0At0
            | Pattern::PseudoSelectorsAt1At0
            | Pattern::PseudoSelectorsAt2At0
            | Pattern::PseudoSelectorsAt3At0
            | Pattern::PseudoSelectorsAt4At0
            | Pattern::PseudoSelectorsAt6At0
            | Pattern::PseudoSelectorsAt7At0
            | Pattern::LessExtendAt0
            | Pattern::AtContainerAt0At0At1
            | Pattern::StyleFunctionAt0
            | Pattern::AtImportAt3
            | Pattern::AtSupportsParens
            | Pattern::ViewFunctionAt0 => self.lit(p, b"("),
            Pattern::LessMixinGuardsAt0At0End
            | Pattern::AttrFunctionAt0End
            | Pattern::AttrFunctionAt0At2End
            | Pattern::LessBooleanFunctionAt0End
            | Pattern::CalcFunctionAt0End
            | Pattern::VarFunctionAt0End
            | Pattern::ColorFunctionsAt0At0End
            | Pattern::ColorFunctionsAt1At0End
            | Pattern::ColorFunctionsAt2At0End
            | Pattern::LessColorBlendFunctionsAt0At0End
            | Pattern::LessColorChannelFunctionsAt0At0End
            | Pattern::LessColorDefinitionFunctionsAt0At0End
            | Pattern::LessColorDefinitionFunctionsAt1At0End
            | Pattern::LessColorOperationFunctionsAt0At0End
            | Pattern::LessColorOperationFunctionsAt1At0End
            | Pattern::LessColorOperationFunctionsAt2At0End
            | Pattern::LessColorOperationFunctionsAt3At0End
            | Pattern::LessColorOperationFunctionsAt4At0End
            | Pattern::LessColorOperationFunctionsAt5At0End
            | Pattern::LessIfFunctionAt0End
            | Pattern::CounterFunctionsAt0At0End
            | Pattern::CounterFunctionsAt0At0At4End
            | Pattern::CounterFunctionsAt1At0End
            | Pattern::CounterFunctionsAt1At0At1End
            | Pattern::CrossFadeFunctionAt0At0End
            | Pattern::LessListFunctionsAt0At0End
            | Pattern::FilterFunctionAt0End
            | Pattern::GradientFunctionsAt0At0End
            | Pattern::LessListFunctionsAt1At0End
            | Pattern::FitContentFunctionAt0End
            | Pattern::FormatFunctionAt0At0End
            | Pattern::GradientFunctionsAt1At0End
            | Pattern::LessListFunctionsAt2At0End
            | Pattern::GridRepeatFunctionAt0End
            | Pattern::MinmaxFunctionAt0End
            | Pattern::ImageFunctionAt0End
            | Pattern::UrlFunctionAt0End
            | Pattern::LessMathFunctionsAt0At0End
            | Pattern::LessMathFunctionsAt2At0End
            | Pattern::LessMiscFunctionsAt0At0End
            | Pattern::LessMiscFunctionsAt1At0End
            | Pattern::LessMiscFunctionsAt2At0End
            | Pattern::LessMiscFunctionsAt3At0End
            | Pattern::LessMiscFunctionsAt5At0End
            | Pattern::LessMiscFunctionsAt6At0End
            | Pattern::LessStringFunctionsAt0At0End
            | Pattern::LessStringFunctionsAt1At0End
            | Pattern::LessStringFunctionsAt2At0End
            | Pattern::LessTypeFunctionsAt0At0End
            | Pattern::LessTypeFunctionsAt1At0End
            | Pattern::LessTypeFunctionsAt2At0End
            | Pattern::LocalFunctionAt0End
            | Pattern::RegexpFunctionAt0End
            | Pattern::ShapeFunctionsAt0At0End
            | Pattern::ShapeFunctionsAt1At0End
            | Pattern::ShapeFunctionsAt2At0End
            | Pattern::ShapeFunctionsAt3At0End
            | Pattern::SymbolsFunctionAt0End
            | Pattern::TransformFunctionsAt0At0End
            | Pattern::TransformFunctionsAt1At0End
            | Pattern::TransformFunctionsAt2At0End
            | Pattern::TransformFunctionsAt3At0End
            | Pattern::TransformFunctionsAt4At0End
            | Pattern::TransformFunctionsAt5At0End
            | Pattern::TransformFunctionsAt6At0End
            | Pattern::TransformFunctionsAt7At0End
            | Pattern::FilterFunctionsAt1At0End
            | Pattern::FilterFunctionsAt2At0End
            | Pattern::FilterFunctionsAt3At0End
            | Pattern::FilterFunctionsAt4At0End
            | Pattern::PseudoSelectorsAt0At0End
            | Pattern::PseudoSelectorsAt1At0End
            | Pattern::PseudoSelectorsAt2At0End
            | Pattern::PseudoSelectorsAt3At0End
            | Pattern::PseudoSelectorsAt4At0End
            | Pattern::PseudoSelectorsAt6At0End
            | Pattern::PseudoSelectorsAt7At0End
            | Pattern::LessExtendAt0End
            | Pattern::StyleFunctionAt0End
            | Pattern::ViewFunctionAt0End => (self.at(p) == b')').then_some(p),
            Pattern::LessVariableComparisonAt0 => self.variable_reference(p),
            Pattern::LessVariableComparisonAt0End => self.spaces_then(p, b")"),
            Pattern::LessVariableComparisonAt0At0 | Pattern::LessLogicalComparisonsAt0 => {
                self.comparison_operator(p)
            }
            Pattern::LessVariableComparisonAt0At1 => self.keyword_out_of_line(p, b"true"),
            Pattern::AttrFunction => self.func(p, &[b"attr"]),
            Pattern::QualifiedName => self.qualified_name(p),
            Pattern::LiteralStringAt0 => self.lit(p, b"'"),
            Pattern::LiteralStringAt0End => self.string_end(p, b'\''),
            Pattern::LessVariableInterpolation => self.interpolation(p),
            Pattern::StringContentAt1 => self.line_continuation(p),
            Pattern::StringContentAt2 => self.string_escape(p),
            Pattern::LiteralStringAt1 => self.lit(p, b"\""),
            Pattern::LiteralStringAt1End => self.string_end(p, b'"'),
            Pattern::LessStringsAt0 => self.escaped_string(p),
            Pattern::LessStringsAt0End => self.string_end_any(p),
            Pattern::AttrFunctionAt0At2
            | Pattern::CounterFunctionsAt1At0At0
            | Pattern::SelectorsAt10At2
            | Pattern::AtNamespaceAt2 => self.name(p, Class::Alpha, Class::DashWord, Escape::Hex),
            Pattern::AttrFunctionAt0At2At0 | Pattern::LessTypeFunctionsAt1At0At2 => {
                self.unit_name(p)
            }
            Pattern::CommaDelimiter => self.separator(p, b','),
            Pattern::PropertyValueConstantsAt0 => self.alt(
                p,
                &[
                    b"flex-start",
                    b"flex-end",
                    b"start",
                    b"end",
                    b"space-between",
                    b"space-around",
                    b"space-evenly",
                    b"stretch",
                    b"baseline",
                    b"safe",
                    b"unsafe",
                    b"legacy",
                    b"anchor-center",
                    b"first",
                    b"last",
                    b"self-start",
                    b"self-end",
                ],
            ),
            Pattern::PropertyValueConstantsAt1 => self.alt(
                p,
                &[
                    b"text-before-edge",
                    b"before-edge",
                    b"middle",
                    b"central",
                    b"text-after-edge",
                    b"after-edge",
                    b"ideographic",
                    b"alphabetic",
                    b"hanging",
                    b"mathematical",
                    b"top",
                    b"center",
                    b"bottom",
                ],
            ),
            Pattern::GlobalPropertyValues => self.alt(
                p,
                &[b"initial", b"inherit", b"unset", b"revert-layer", b"revert"],
            ),
            Pattern::CubicBezierFunction => self.func_open(p, &[b"cubic-bezier"]),
            Pattern::LessBooleanFunction => self.func(p, &[b"boolean"]),
            Pattern::LessLogicalComparisonsAt2 => self.true_or_false(p),
            Pattern::LessLogicalComparisonsAt3
            | Pattern::LessVariableComparisonAt0At5
            | Pattern::RuleListBodyAt5At5At3 => self.lit(p, b","),
            Pattern::CalcFunction => self.func(p, &[b"calc"]),
            Pattern::VarFunction => self.func(p, &[b"var"]),
            Pattern::CustomPropertyName => self.custom_property(p),
            Pattern::LessVariablesAt0 => self.variable(p),
            Pattern::ColorFunctionsAt0 => self.func(p, &[b"rgba", b"rgb"]),
            Pattern::ValueSeparator => self.separator(p, b'/'),
            Pattern::PercentageType => self.percentage(p),
            Pattern::NumberType => self.number(p),
            Pattern::ColorFunctionsAt1 => self.func(
                p,
                &[b"hsla", b"hsl", b"hwb", b"oklab", b"oklch", b"lab", b"lch"],
            ),
            Pattern::ColorFunctionsAt2 => self.func(p, &[b"light-dark"]),
            Pattern::LessColorBlendFunctionsAt0 => self.func(
                p,
                &[
                    b"multiply",
                    b"screen",
                    b"overlay",
                    b"softlight",
                    b"hardlight",
                    b"difference",
                    b"exclusion",
                    b"negation",
                    b"average",
                ],
            ),
            Pattern::LessColorChannelFunctionsAt0 => self.func(
                p,
                &[
                    b"hue",
                    b"saturation",
                    b"lightness",
                    b"hsvhue",
                    b"hsvsaturation",
                    b"hsvvalue",
                    b"red",
                    b"green",
                    b"blue",
                    b"alpha",
                    b"luma",
                    b"luminance",
                ],
            ),
            Pattern::LessColorDefinitionFunctionsAt0 => self.func(p, &[b"argb"]),
            Pattern::LessColorDefinitionFunctionsAt1 => self.func(p, &[b"hsva", b"hsv"]),
            Pattern::IntegerType => self.integer(p),
            Pattern::LessColorOperationFunctionsAt0 => self.func(p, &[b"fade", b"shade", b"tint"]),
            Pattern::LessColorOperationFunctionsAt1 => self.func(p, &[b"spin"]),
            Pattern::LessColorOperationFunctionsAt2 => self.func(
                p,
                &[
                    b"desaturate",
                    b"saturate",
                    b"lighten",
                    b"darken",
                    b"fadein",
                    b"fadeout",
                ],
            ),
            Pattern::LessColorOperationFunctionsAt3 => self.func(p, &[b"contrast"]),
            Pattern::LessColorOperationFunctionsAt4 => self.func(p, &[b"greyscale"]),
            Pattern::LessColorOperationFunctionsAt5 => self.func(p, &[b"mix"]),
            Pattern::LessIfFunction => self.func(p, &[b"if"]),
            Pattern::CounterFunctionsAt0 => self.func(p, &[b"counter"]),
            Pattern::CounterFunctionsAt0At0At3 => self.counter_name(p),
            Pattern::CounterFunctionsAt0At0At4 | Pattern::CounterFunctionsAt1At0At1 => {
                (self.at(p) == b',').then_some(p)
            }
            Pattern::CounterFunctionsAt0At0At4At1 | Pattern::CounterFunctionsAt1At0At1At5 => {
                self.table_word(p, words::COUNTER_STYLES)
            }
            Pattern::CounterFunctionsAt1 => self.func(p, &[b"counters"]),
            Pattern::CrossFadeFunctionAt0 => self.func(p, &[b"cross-fade"]),
            Pattern::LessListFunctionsAt0 => self.func(p, &[b"length"]),
            Pattern::FilterFunction => self.func(p, &[b"filter"]),
            Pattern::GradientFunctionsAt0 => {
                self.func(p, &[b"repeating-linear-gradient", b"linear-gradient"])
            }
            Pattern::AngleType => self.dimension(p, &[b"deg", b"grad", b"rad", b"turn"]),
            Pattern::LessListFunctionsAt1 => self.func(p, &[b"extract"]),
            Pattern::FitContentFunction => self.func(p, &[b"fit-content"]),
            Pattern::LengthTypeAt0 => self.length(p),
            Pattern::LengthTypeAt1 => self.zero(p),
            Pattern::FormatFunctionAt0 => self.func(p, &[b"format"]),
            Pattern::GradientFunctionsAt1 => {
                self.func(p, &[b"repeating-radial-gradient", b"radial-gradient"])
            }
            Pattern::LessListFunctionsAt2 => self.func(p, &[b"range"]),
            Pattern::GridRepeatFunction => self.func(p, &[b"repeat"]),
            Pattern::MinmaxFunction => self.func(p, &[b"minmax"]),
            Pattern::MinmaxFunctionAt0At4 => self.alt(p, &[b"max-content", b"min-content"]),
            Pattern::GridRepeatFunctionAt0At6 => self.alt(p, &[b"auto-fill", b"auto-fit"]),
            Pattern::GridRepeatFunctionAt0At7 => {
                self.alt(p, &[b"max-content", b"min-content", b"auto"])
            }
            Pattern::ImageFunction => self.func(p, &[b"image"]),
            Pattern::UrlFunction => self.func(p, &[b"url"]),
            Pattern::UnquotedString => self.unquoted_char(p, b""),
            Pattern::LessMathFunctionsAt0 => self.func(
                p,
                &[
                    b"ceil",
                    b"floor",
                    b"percentage",
                    b"round",
                    b"sqrt",
                    b"abs",
                    b"asin",
                    b"acos",
                    b"atan",
                    b"sin",
                    b"cos",
                    b"tan",
                ],
            ),
            Pattern::FrequencyType => self.dimension(p, &[b"Hz", b"kHz"]),
            Pattern::TimeType => self.dimension(p, &[b"s", b"ms"]),
            Pattern::LessMathFunctionsAt1 => self.pi(p),
            Pattern::LessMathFunctionsAt2 => self.func(p, &[b"pow", b"mod", b"min", b"max"]),
            Pattern::LessMiscFunctionsAt0 => self.func(p, &[b"color"]),
            Pattern::LessMiscFunctionsAt1 => {
                self.func(p, &[b"image-size", b"image-width", b"image-height"])
            }
            Pattern::LessMiscFunctionsAt2 => self.func(p, &[b"convert", b"unit"]),
            Pattern::LessMiscFunctionsAt2At0At4 => self.first_of(
                p,
                &[
                    b"cm", b"mm", b"m", b"in", b"pc", b"pt", b"px", b"ms", b"s", b"grad", b"rad",
                    b"deg", b"turn", b"%", b"rem", b"em", b"ex", b"ch",
                ],
            ),
            Pattern::LessMiscFunctionsAt3 => self.func(p, &[b"data-uri"]),
            Pattern::LessMiscFunctionsAt3At0At2 | Pattern::ArbitraryRepetition => {
                self.spaced_lit(p, b',')
            }
            Pattern::LessMiscFunctionsAt4 => self.default_word(p),
            Pattern::LessMiscFunctionsAt5 => self.func(p, &[b"get-unit"]),
            Pattern::LessMiscFunctionsAt6 => self.func(p, &[b"svg-gradient"]),
            Pattern::LessStringFunctionsAt0 => self.func(p, &[b"escape", b"e"]),
            Pattern::LessStringFunctionsAt1 => self.format_function(p),
            Pattern::LessStringFunctionsAt2 => self.func(p, &[b"replace"]),
            Pattern::LessTypeFunctionsAt0 => self.func(
                p,
                &[
                    b"isnumber",
                    b"isstring",
                    b"iscolor",
                    b"iskeyword",
                    b"isurl",
                    b"ispixel",
                    b"isem",
                    b"ispercentage",
                    b"isruleset",
                ],
            ),
            Pattern::LessTypeFunctionsAt1 => self.func(p, &[b"isunit"]),
            Pattern::LessTypeFunctionsAt2 => self.func(p, &[b"isdefined"]),
            Pattern::LocalFunction => self.func(p, &[b"local"]),
            Pattern::RegexpFunction => self.func(p, &[b"regexp"]),
            Pattern::ShapeFunctionsAt0 => self.func(p, &[b"rect"]),
            Pattern::ShapeFunctionsAt0At0At0 => self.keyword_out_of_line(p, b"auto"),
            Pattern::ShapeFunctionsAt1 => self.func(p, &[b"inset"]),
            Pattern::ShapeFunctionsAt1At0At0 => self.keyword_out_of_line(p, b"round"),
            Pattern::ShapeFunctionsAt2 => self.func(p, &[b"circle", b"ellipse"]),
            Pattern::ShapeFunctionsAt2At0At0 => self.keyword_out_of_line(p, b"at"),
            Pattern::ShapeFunctionsAt2At0At1 => self.alt(
                p,
                &[
                    b"top",
                    b"right",
                    b"bottom",
                    b"left",
                    b"center",
                    b"closest-side",
                    b"farthest-side",
                ],
            ),
            Pattern::ShapeFunctionsAt3 => self.func(p, &[b"polygon"]),
            Pattern::ShapeFunctionsAt3At0At0 => self.alt(p, &[b"nonzero", b"evenodd"]),
            Pattern::StepsFunction => self.func_open(p, &[b"steps"]),
            Pattern::StepsFunctionAt0 => self.first_of(
                p,
                &[
                    b"jump-start",
                    b"jump-end",
                    b"jump-none",
                    b"jump-both",
                    b"start",
                    b"end",
                ],
            ),
            Pattern::SymbolsFunction => self.func(p, &[b"symbols"]),
            Pattern::SymbolsFunctionAt0At0 => self.alt(
                p,
                &[b"cyclic", b"numeric", b"alphabetic", b"symbolic", b"fixed"],
            ),
            Pattern::TransformFunctionsAt0 => {
                self.func(p, &[b"matrix3d", b"matrix", b"scale3d", b"scale"])
            }
            Pattern::TransformFunctionsAt1 => self.func(p, &[b"translate3d", b"translate"]),
            Pattern::TransformFunctionsAt2 => self.func(p, &[b"translateX", b"translateY"]),
            Pattern::TransformFunctionsAt3 => self.func(
                p,
                &[
                    b"rotateX", b"rotateY", b"rotateZ", b"rotate", b"skewX", b"skewY",
                ],
            ),
            Pattern::TransformFunctionsAt4 => self.func(p, &[b"skew"]),
            Pattern::TransformFunctionsAt5 => self.func(p, &[b"translateZ", b"perspective"]),
            Pattern::TransformFunctionsAt6 => self.func(p, &[b"rotate3d"]),
            Pattern::TransformFunctionsAt7 => self.func(p, &[b"scaleX", b"scaleY", b"scaleZ"]),
            Pattern::UnicodeRange => self.unicode_range(p),
            Pattern::ColorValuesAt4 => self.alt(
                p,
                &[
                    b"aqua", b"black", b"blue", b"fuchsia", b"gray", b"green", b"lime", b"maroon",
                    b"navy", b"olive", b"orange", b"purple", b"red", b"silver", b"teal", b"white",
                    b"yellow",
                ],
            ),
            Pattern::ColorValuesAt5 => self.table_word(p, words::EXTENDED_COLORS),
            Pattern::ColorValuesAt6 => self.current_color(p),
            Pattern::ColorValuesAt7 => self.hex_color(p),
            Pattern::RelativeColorAt0 => self.lit(p, b"from"),
            Pattern::RelativeColorAt1 => self.channel_letter(p),
            Pattern::PropertyValueConstantsAt5 => self.alt(p, &[b"replace", b"add", b"accumulate"]),
            Pattern::PropertyValueConstantsAt6 => self.alt(
                p,
                &[b"normal", b"alternate-reverse", b"alternate", b"reverse"],
            ),
            Pattern::PropertyValueConstantsAt7 => {
                self.alt(p, &[b"forwards", b"backwards", b"both"])
            }
            Pattern::PropertyValueConstantsAt8 => self.keyword_out_of_line(p, b"infinite"),
            Pattern::PropertyValueConstantsAt9 => self.alt(p, &[b"running", b"paused"]),
            Pattern::PropertyValueConstantsAt10 => {
                self.alt(p, &[b"entry-crossing", b"entry", b"exit-crossing", b"exit"])
            }
            Pattern::PropertyValueConstantsAt11 => self.alt(
                p,
                &[
                    b"linear",
                    b"ease-in-out",
                    b"ease-in",
                    b"ease-out",
                    b"ease",
                    b"step-start",
                    b"step-end",
                ],
            ),
            Pattern::PropertyValueConstantsAt12 => self.table_word(p, words::VALUE_KEYWORDS),
            Pattern::PropertyValueConstantsAt13 => self.generic_family(p),
            Pattern::LessMathAt0 => {
                matches!(self.at(p), b'-' | b'*' | b'+' | b'/').then_some(p + 1)
            }
            Pattern::Important => self.important(p),
            Pattern::LessMiscFunctionsAt6At0At5 | Pattern::GradientFunctionsAt0At0At7 => {
                self.keyword_out_of_line(p, b"to")
            }
            Pattern::LessMiscFunctionsAt6At0At6 => {
                self.alt(p, &[b"top", b"right", b"bottom", b"left", b"center"])
            }
            Pattern::LessMiscFunctionsAt6At0At7 | Pattern::GradientFunctionsAt1At0At6 => {
                self.alt(p, &[b"at", b"circle", b"ellipse"])
            }
            Pattern::GradientFunctionsAt1At0At7 => self.alt(
                p,
                &[
                    b"top",
                    b"right",
                    b"bottom",
                    b"left",
                    b"center",
                    b"farthest-corner",
                    b"farthest-side",
                    b"closest-corner",
                    b"closest-side",
                ],
            ),
            Pattern::GradientFunctionsAt0At0At8 => {
                self.alt(p, &[b"top", b"right", b"bottom", b"left"])
            }
            Pattern::FilterFunctionsAt1 => self.func(p, &[b"blur"]),
            Pattern::FilterFunctionsAt2 => self.func(
                p,
                &[
                    b"brightness",
                    b"contrast",
                    b"grayscale",
                    b"invert",
                    b"opacity",
                    b"saturate",
                    b"sepia",
                ],
            ),
            Pattern::FilterFunctionsAt3 => self.func(p, &[b"drop-shadow"]),
            Pattern::FilterFunctionsAt4 => self.func(p, &[b"hue-rotate"]),
            Pattern::LessColorOperationFunctionsAt2At0At3 => {
                self.keyword_out_of_line(p, b"relative")
            }
            Pattern::SelectorsAt0 => self.custom_element(p),
            Pattern::SelectorsAt1 => self.table_word(p, words::TAG_NAMES),
            Pattern::SelectorsAt2 => self.group_lit(p, b"."),
            Pattern::SelectorsAt2End | Pattern::SelectorsAt3End | Pattern::SelectorsAt4End => {
                self.name_ends(p)
            }
            Pattern::SelectorsAt3 => self.group_lit(p, b"#"),
            Pattern::SelectorsAt4 => self.group_lit(p, b"&"),
            Pattern::PseudoSelectorsAt0 => self.pseudo_func(p, &[b"dir"]),
            Pattern::PseudoSelectorsAt0At0At0 => self.first_of(p, &[b"ltr", b"rtl"]),
            Pattern::PseudoSelectorsAt1 => self.pseudo_func(p, &[b"lang"]),
            Pattern::PseudoSelectorsAt2 => self.pseudo_func(p, &[b"not"]),
            Pattern::PseudoSelectorsAt3 => self.pseudo_func(
                p,
                &[
                    b"nth-last-child",
                    b"nth-last-of-type",
                    b"nth-child",
                    b"nth-of-type",
                ],
            ),
            Pattern::PseudoSelectorsAt3At0At0 => self.alt(p, &[b"even", b"odd"]),
            Pattern::PseudoSelectorsAt3At0At1 => self.an_plus_b(p),
            Pattern::PseudoSelectorsAt4 => self.pseudo_func(
                p,
                &[b"host-context", b"host", b"has", b"is", b"not", b"where"],
            ),
            Pattern::PseudoSelectorsAt5 => self.pseudo_class(p),
            Pattern::PseudoSelectorsAt6 => {
                self.pseudo_element_func(p, &[b"highlight", b"part", b"state"])
            }
            Pattern::PseudoSelectorsAt6At0At0
            | Pattern::RuleListBodyAt5At12At0At3
            | Pattern::AtContainerAt0At0At3 => self.dashes_or_name(p),
            Pattern::PseudoSelectorsAt7 => self.pseudo_element_func(p, &[b"slotted"]),
            Pattern::PseudoSelectorsAt8 => self.pseudo_element(p),
            Pattern::PseudoSelectorsAt9 => self.vendor_pseudo(p),
            Pattern::LessExtend => self.pseudo_func(p, &[b"extend"]),
            Pattern::LessExtendAt0At0 => self.keyword_out_of_line(p, b"all"),
            Pattern::SelectorsAt7 => self.combinator(p),
            Pattern::SelectorsAt8 => self.combinator_run(p),
            Pattern::SelectorsAt9 => self.lit(p, b"/deep/"),
            Pattern::SelectorsAt10 => self.lit(p, b"["),
            Pattern::SelectorsAt10End | Pattern::SelectorsAt10At3At4 => self.lit(p, b"]"),
            Pattern::SelectorsAt10At3 => self.attribute_operator(p),
            Pattern::SelectorsAt10At3End => self.attribute_value_end(p),
            Pattern::SelectorsAt10At3At1 => self.unquoted_char(p, b"[]"),
            Pattern::SelectorsAt10At3At3 => self.attribute_flag(p),
            Pattern::SelectorsAt12 => self.lit(p, b"*"),
            Pattern::LessMixinGuardsAt0At0At1 => self.default_call(p),
            Pattern::LessNamespaceAccessorsAt1 => self.group_lit(p, b"("),
            Pattern::LessNamespaceAccessorsAt1End => self.group_lit(p, b")"),
            Pattern::LessVariableAssignmentAt0 => self.variable_name(p),
            Pattern::LessVariableAssignmentAt0End => self.assignment_end(p),
            Pattern::LessVariableAssignmentAt0At0
            | Pattern::RuleListBodyAt5At5At0
            | Pattern::RuleListBodyAt5At9At0
            | Pattern::RuleListBodyAt5At10At0
            | Pattern::RuleListBodyAt5At11At0 => self.merge_colon_blanks(p),
            Pattern::PropertyListAt0 | Pattern::AtContainerAt0End => {
                (self.at(p) == b'{').then_some(p)
            }
            Pattern::PropertyListAt0End => self.lit(p, b"}"),
            Pattern::RuleListAt0 => self.lit(p, b"{"),
            Pattern::RuleListAt0End => self.rule_list_end(p),
            Pattern::RuleListAt0At0
            | Pattern::RuleListBodyAt5At1End
            | Pattern::RuleListBodyAt5At3End
            | Pattern::RuleListBodyAt5At5End
            | Pattern::RuleListBodyAt5At6End
            | Pattern::RuleListBodyAt5At7End
            | Pattern::RuleListBodyAt5At8End
            | Pattern::RuleListBodyAt5At9End
            | Pattern::RuleListBodyAt5At10End
            | Pattern::RuleListBodyAt5At11End
            | Pattern::RuleListBodyAt5At12End
            | Pattern::RuleListBodyAt6End => self.semicolon_or_close(p, Spaces::Before),
            Pattern::AtCharset => self.at_rule_word(p, b"charset"),
            Pattern::AtCharsetEnd => self.semicolon_or_eol_ahead(p),
            Pattern::AtContainer => self.at_rule_ahead(p, b"container", Tail::Any),
            Pattern::AtContainerEnd
            | Pattern::AtCounterStyleEnd
            | Pattern::AtFontFaceEnd
            | Pattern::AtMediaEnd
            | Pattern::AtKeyframesEnd => self.spaced_lit(p, b'}'),
            Pattern::AtContainerAt0 => self.at_keyword(p, b"container"),
            Pattern::AtContainerAt0At0 | Pattern::MediaQueryList | Pattern::AtKeyframesAt1 => {
                self.spaces_before_other(p)
            }
            Pattern::AtContainerAt0At0End | Pattern::MediaQueryListEnd => {
                self.spaces_then(p, b";{")
            }
            Pattern::AtContainerAt0At0At0 => self.alt(p, &[b"not", b"and", b"or"]),
            Pattern::AtContainerAt0At0At1At0 => self.alt_cap(
                p,
                &[
                    b"aspect-ratio",
                    b"block-size",
                    b"height",
                    b"inline-size",
                    b"orientation",
                    b"width",
                ],
            ),
            Pattern::AtContainerAt0At0At1At1 => self.comparison(p),
            Pattern::AtContainerAt0At0At1At2 => self.lit(p, b":"),
            Pattern::AtContainerAt0At0At1At3 => self.first_of(p, &[b"portrait", b"landscape"]),
            Pattern::AtContainerAt0At0At1At5 => self.lit(p, b"/"),
            Pattern::StyleFunction => self.func(p, &[b"style"]),
            Pattern::AtCounterStyle => self.at_counter_style(p),
            Pattern::AtCustomMedia => self.at_rule_ahead(p, b"custom-media", Tail::Boundary),
            Pattern::AtCustomMediaEnd | Pattern::AtImportAt2End => self.spaces_then(p, b";"),
            Pattern::AtCustomMediaAt0 => self.through_semicolon(p),
            Pattern::AtCustomMediaAt1 => self.spaced_at_keyword(p, b"custom-media"),
            Pattern::MediaQuery => self.media_query(p),
            Pattern::MediaQueryAt2 => self.media_feature_open(p),
            Pattern::MediaQueryAt2At0 => self.media_feature_name(p),
            Pattern::MediaQueryAt2At0End => self.feature_name_end(p),
            Pattern::MediaQueryAt2At1 => self.feature_keyword(p),
            Pattern::MediaQueryAt2At2 => self.ratio(p),
            Pattern::AtFontFace => self.at_font_face(p),
            Pattern::AtMedia => self.at_rule_ahead(p, b"media", Tail::Boundary),
            Pattern::AtMediaAt0 => self.spaced_at_keyword(p, b"media"),
            Pattern::AtMediaAt0End | Pattern::AtSupportsAt0End | Pattern::AtKeyframesAt1End => {
                self.spaces_then(p, b"{")
            }
            Pattern::AtMediaAt1
            | Pattern::AtKeyframesAt0
            | Pattern::AtSupportsAt1
            | Pattern::AtContainerAt1
            | Pattern::LessNamespaceAccessorsAt0At3 => self.spaced_lit(p, b'{'),
            Pattern::AtMediaAt1End
            | Pattern::AtKeyframesAt0End
            | Pattern::AtSupportsAt1End
            | Pattern::AtContainerAt1End
            | Pattern::LessNamespaceAccessorsAt0At3End => (self.at(p) == b'}').then_some(p),
            Pattern::AtImport => self.at_rule_word(p, b"import"),
            Pattern::AtImportEnd | Pattern::AtNamespaceEnd => self.lit(p, b";"),
            Pattern::AtImportAt2 => self.after_import_url(p),
            Pattern::AtImportAt3At0 => self.first_of(
                p,
                &[
                    b"reference",
                    b"inline",
                    b"less",
                    b"css",
                    b"once",
                    b"multiple",
                    b"optional",
                ],
            ),
            Pattern::AtKeyframes => self.at_keyframes(p),
            Pattern::AtKeyframesAt0At0 => self.keyframe_selector(p),
            Pattern::LessNamespaceAccessorsAt2 => self.semicolon_or_close(p, Spaces::None),
            Pattern::AtNamespace => self.at_namespace(p),
            Pattern::AtPage => self.at_page(p),
            Pattern::AtSupports => self.at_rule_ahead(p, b"supports", Tail::Boundary),
            Pattern::AtSupportsEnd => self.group_lit(p, b"}"),
            Pattern::AtSupportsAt0 => self.spaced_at_keyword(p, b"supports"),
            Pattern::AtSupportsOperators => self.alt(p, &[b"and", b"or", b"not"]),
            Pattern::RuleListBodyAt4 => self.interpolated_property_ahead(p),
            Pattern::RuleListBodyAt4End
            | Pattern::RuleListBodyAt4At1End
            | Pattern::RuleListBodyAt5At1At0End
            | Pattern::RuleListBodyAt5At3At0End
            | Pattern::RuleListBodyAt5At6At0End
            | Pattern::RuleListBodyAt5At7At0End
            | Pattern::RuleListBodyAt5At8At0End
            | Pattern::RuleListBodyAt5At12At0End => self.semicolon_or_close_ahead(p),
            Pattern::RuleListBodyAt4At0 => self.not_colon_or_space(p),
            Pattern::RuleListBodyAt4At0End => self.merge_colon_ahead(p),
            Pattern::RuleListBodyAt4At1
            | Pattern::RuleListBodyAt5At1At0
            | Pattern::RuleListBodyAt5At3At0
            | Pattern::RuleListBodyAt5At6At0
            | Pattern::RuleListBodyAt5At7At0
            | Pattern::RuleListBodyAt5At8At0
            | Pattern::RuleListBodyAt5At12At0 => self.merge_colon(p),
            Pattern::RuleListBodyAt5 => matches!(self.at(p), b'-' | b'a'..=b'z').then_some(p),
            Pattern::RuleListBodyAt5End => {
                (self.at_eol(p) || !matches!(self.at(p), b'-' | b'a'..=b'z')).then_some(p)
            }
            Pattern::RuleListBodyAt5At1 => self.vendor_property(p),
            Pattern::RuleListBodyAt5At1At0At1 => self.dash_word_run(p),
            Pattern::RuleListBodyAt5At3 => self.border_radius(p),
            Pattern::RuleListBodyAt5At4 => self.var_property(p),
            Pattern::RuleListBodyAt5At5 => self.font_property(p),
            Pattern::RuleListBodyAt5At5At2 => self.family_name(p),
            Pattern::RuleListBodyAt5At6 => self.keyword_out_of_line(p, b"animation-timeline"),
            Pattern::ScrollFunction => self.func_open(p, &[b"scroll"]),
            Pattern::ScrollFunctionAt0 => self.first_of(p, &[b"root", b"nearest", b"self"]),
            Pattern::ScrollFunctionAt1 => self.first_of(p, &[b"block", b"inline", b"x", b"y"]),
            Pattern::ViewFunction => self.func(p, &[b"view"]),
            Pattern::ViewFunctionAt0At0 => {
                self.first_of(p, &[b"block", b"inline", b"x", b"y", b"auto"])
            }
            Pattern::RuleListBodyAt5At7 => self.animation_property(p),
            Pattern::RuleListBodyAt5At7At0At6 => self.animation_name(p),
            Pattern::RuleListBodyAt5At8 => self.alt_cap(
                p,
                &[
                    b"transition-property",
                    b"transition-duration",
                    b"transition-delay",
                    b"transition-timing-function",
                    b"transition",
                ],
            ),
            Pattern::RuleListBodyAt5At9 => self.alt(p, &[b"backdrop-filter", b"filter"]),
            Pattern::RuleListBodyAt5At9At1 => {
                self.alt(p, &[b"inherit", b"initial", b"unset", b"none"])
            }
            Pattern::RuleListBodyAt5At10 => self.keyword_out_of_line(p, b"will-change"),
            Pattern::RuleListBodyAt5At10At1 => self.first_of(
                p,
                &[
                    b"unset",
                    b"initial",
                    b"inherit",
                    b"will-change",
                    b"auto",
                    b"scroll-position",
                    b"contents",
                ],
            ),
            Pattern::RuleListBodyAt5At10At2 | Pattern::RuleListBodyAt5At11At1 => {
                self.name(p, Class::DashWordWide, Class::AlphaWide, Escape::Hex)
            }
            Pattern::RuleListBodyAt5At11 => {
                self.alt(p, &[b"counter-increment", b"counter-reset", b"counter-set"])
            }
            Pattern::RuleListBodyAt5At11At3 => {
                self.first_of(p, &[b"unset", b"initial", b"inherit", b"auto"])
            }
            Pattern::RuleListBodyAt5At12 => self.container_property(p),
            Pattern::RuleListBodyAt5At12At0At0 => self.keyword_out_of_line(p, b"default"),
            Pattern::RuleListBodyAt5At13 => self.table_word(p, words::PROPERTY_NAMES),
            Pattern::RuleListBodyAt5At14 => self.size_property(p),
            Pattern::SelectorAt0 => self.selector_ahead(p),
            Pattern::SelectorAt0End => self.selector_end(p),
            Pattern::RuleListBodyAt6 => self.merge_colon_word(p),
            Pattern::KeyframeName => self.keyframe_name(p),
        }
    }

    /// `\s*(x)\s*` for the byte `byte`, in group 1.
    #[inline(never)]
    fn separator(&mut self, p: usize, byte: u8) -> Option<usize> {
        let end = self.spaced_lit(p, byte)?;
        Some(self.spaces(end))
    }

    /// `\b(default(\()(\)))\b`.
    #[inline(never)]
    fn default_word(&mut self, p: usize) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        self.default_call(p).filter(|&end| self.boundary(end))
    }

    /// `\b[abchlsw]\b`: a channel of a relative colour.
    #[inline(never)]
    fn channel_letter(&self, p: usize) -> Option<usize> {
        (self.boundary(p)
            && matches!(self.at(p), b'a' | b'b' | b'c' | b'h' | b'l' | b's' | b'w')
            && self.boundary(p + 1))
        .then_some(p + 1)
    }

    /// `--|NAME`: two dashes, or a CSS name.
    #[inline(never)]
    fn dashes_or_name(&mut self, p: usize) -> Option<usize> {
        self.lit(p, b"--")
            .or_else(|| self.name(p, Class::NameStart, Class::NameRest, Escape::Any))
    }

    /// `(?=([]\s]))`: the end of an unquoted attribute value.
    #[inline(never)]
    fn attribute_value_end(&self, p: usize) -> Option<usize> {
        (self.at(p) == b']' || self.space_at(p) > 0).then_some(p)
    }

    /// `(((\+_?)?):)([\t\s]*)`: the colon of [`Lexer::merge_colon`], then
    /// blanks in group 4.
    #[inline(never)]
    fn merge_colon_blanks(&mut self, p: usize) -> Option<usize> {
        let end = self.merge_colon(p)?;
        Some(self.blanks_cap(end, 4))
    }

    /// `(?=\s*})`: the end of a rule list.
    #[inline(never)]
    fn rule_list_end(&self, p: usize) -> Option<usize> {
        self.spaces_then(p, b"}").map(|_| p)
    }

    /// `\s*;`.
    #[inline(never)]
    fn through_semicolon(&self, p: usize) -> Option<usize> {
        self.spaces_then(p, b";").map(|end| end + 1)
    }

    /// `(((\+_?)?):)|(?=\))`: the colon of [`Lexer::merge_colon`], or the place
    /// before a `)`.
    #[inline(never)]
    fn feature_name_end(&mut self, p: usize) -> Option<usize> {
        self.merge_colon(p)
            .or_else(|| (self.at(p) == b')').then_some(p))
    }

    /// `\b(portrait|landscape|progressive|interlace)`.
    #[inline(never)]
    fn feature_keyword(&self, p: usize) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        self.first_of(
            p,
            &[b"portrait", b"landscape", b"progressive", b"interlace"],
        )
    }

    /// `\b(a|b|…)\b` at `p` for a big alternation, as a table: of the words
    /// that start at `p` and end where `\b` holds, the one the alternation
    /// tries first. Every word of a table is made of `[-.0-9:A-Za-z]` and is
    /// at most [`LONGEST_WORD`] bytes long, so the search ends there.
    ///
    /// Each end where `\b` holds is searched as written. When the text holds a
    /// capital, it is also searched lowered, for the words that ignore case
    /// ([`find`]). The search stops at the first end that no longer word of
    /// the table goes on from, in either spelling. No later end can then
    /// match.
    fn table_word(&self, p: usize, table: &[Word]) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        let run = self.bounded(p, LONGEST_WORD, |b| {
            b == b'-' || b == b'.' || b == b':' || b.is_ascii_alphanumeric()
        });
        let mut best: Option<(u16, usize)> = None;
        let mut lower = [0_u8; LONGEST_WORD];
        for end in p + 1..=run {
            if !self.boundary(end) {
                continue;
            }
            let text = self.text(p, end);
            let (mut word, mut goes_on) = find(table, text);
            if text.iter().any(u8::is_ascii_uppercase)
                && let Some(folded) = lower.get_mut(..text.len())
            {
                folded.copy_from_slice(text);
                folded.make_ascii_lowercase();
                let (found, more) = find(table, folded);
                word = word.or(found.filter(|word| word.folded));
                goes_on |= more;
            }
            if let Some(word) = word
                && best.is_none_or(|(order, _)| word.order < order)
            {
                best = Some((word.order, end));
            }
            if !goes_on {
                break;
            }
        }
        self.spend(run - p);
        if !self.at(run).is_ascii()
            && let Some((order, end)) = self.folded_table_word(p, run, table)
            && best.is_none_or(|(best_order, _)| order < best_order)
        {
            best = Some((order, end));
        }
        best.map(|(_, end)| end)
    }

    /// [`Lexer::table_word`] past `run`, where a character above ASCII
    /// stands. It finds, among the words that ignore case, the one the
    /// alternation tries first that the text spells. `(?i)` reads a character
    /// that folds to letters as those letters, `ſ` as `s` and the Kelvin sign
    /// as `k` ([`case_fold::lower_into`]). Any other such character ends the
    /// words. The function is out of line, because such text is rare.
    #[cold]
    fn folded_table_word(&self, p: usize, run: usize, table: &[Word]) -> Option<(u16, usize)> {
        let mut best: Option<(u16, usize)> = None;
        let mut lower = [0_u8; LONGEST_WORD];
        let mut end = run;
        while end < self.lim() {
            let byte = self.at(end);
            end = if byte.is_ascii() {
                if !(byte == b'-' || byte == b'.' || byte == b':' || byte.is_ascii_alphanumeric()) {
                    break;
                }
                end + 1
            } else {
                let Some((c, next)) = self.char_at(end) else {
                    break;
                };
                if case_fold::letters(c).is_none() {
                    break;
                }
                next
            };
            if !self.boundary(end) {
                continue;
            }
            let Some(len) = case_fold::lower_into(self.text(p, end), &mut lower) else {
                break;
            };
            let (word, goes_on) = find(table, lower.get(..len).unwrap_or_default());
            if let Some(word) = word.filter(|word| word.folded)
                && best.is_none_or(|(order, _)| word.order < order)
            {
                best = Some((word.order, end));
            }
            if !goes_on {
                break;
            }
        }
        self.spend(end - run);
        best
    }

    /// `\b(a|b|…)\b` at `p` for a short alternation: the first word, in the
    /// alternation's order, that starts at `p` and ends where `\b` holds.
    fn alt(&self, p: usize, words: &[&[u8]]) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        words
            .iter()
            .map(|word| p + word.len())
            .zip(words)
            .find(|&(end, word)| self.starts(p, word) && end <= self.lim() && self.boundary(end))
            .map(|(end, _)| end)
    }

    /// [`Lexer::alt`], the word in group 1.
    fn alt_cap(&mut self, p: usize, words: &[&[u8]]) -> Option<usize> {
        let end = self.alt(p, words)?;
        self.cap(1, p, end);
        Some(end)
    }

    /// `a|b|…` at `p`: the first word, in the alternation's order, that
    /// starts at `p`.
    fn first_of(&self, p: usize, words: &[&[u8]]) -> Option<usize> {
        words.iter().find_map(|word| self.lit(p, word))
    }

    /// `\b(a|b|…)(?=\()` at `p`, the name in group 1. No word of a list is
    /// another followed by `(`, so the order does not matter.
    ///
    /// Every word is made of `[-\w]`, and `(` is not one of them. A word
    /// followed by `(` is therefore the whole run of `[-\w]` from `p`. The
    /// run is read once for the patterns tried at `p`
    /// ([`Lexer::dash_word_end`]). Only a run that `(` follows is compared
    /// with the words.
    fn func(&mut self, p: usize, words: &[&[u8]]) -> Option<usize> {
        if !self.boundary(p) {
            return None;
        }
        let end = self.dash_word_end(p);
        if end == p || end > self.lim() || self.at(end) != b'(' {
            return None;
        }
        let name = self.text(p, end);
        words.contains(&name).then(|| {
            self.cap(1, p, end);
            end
        })
    }

    /// `\b(a|b|…)(\()` at `p`, the name in group 1 and the `(` in group 2.
    fn func_open(&mut self, p: usize, words: &[&[u8]]) -> Option<usize> {
        let end = self.func(p, words)?;
        self.cap(2, end, end + 1);
        Some(end + 1)
    }

    /// `(x)` for the literal `text`, in group 1.
    fn group_lit(&mut self, p: usize, text: &[u8]) -> Option<usize> {
        let end = self.lit(p, text)?;
        self.cap(1, p, end);
        Some(end)
    }

    /// `\s*(x)` for the byte `byte`, in group 1.
    fn spaced_lit(&mut self, p: usize, byte: u8) -> Option<usize> {
        let at = self.spaces(p);
        (self.at(at) == byte && at < self.lim()).then(|| {
            self.cap(1, at, at + 1);
            at + 1
        })
    }
}
