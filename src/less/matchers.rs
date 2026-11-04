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
use super::grammar::Pat;
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
    /// Whether pattern `pat` matches at `p`, and where its match ends.
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
    pub(super) fn less_matcher(&mut self, pat: Pat, p: usize) -> Option<usize> {
        match pat {
            Pat::CommentBlockAt0 => self.lit(p, b"/*"),
            Pat::CommentBlockAt0End => self.lit(p, b"*/"),
            Pat::CommentLine => self.comment_line(p),
            Pat::LessNamespaceAccessorsAt0 => self.when_ahead(p),
            Pat::LessNamespaceAccessorsAt0End | Pat::MediaQueryEnd | Pat::KeyframeNameEnd => {
                self.comma_or_block(p)
            }
            Pat::LessNamespaceAccessorsAt0At0 => self.when(p),
            Pat::LessMixinGuardsAt0 => self.guard_operator(p),
            Pat::LessMixinGuardsAt0End
            | Pat::AttrFunctionEnd
            | Pat::CubicBezierFunctionEnd
            | Pat::LessBooleanFunctionEnd
            | Pat::LessLogicalComparisonsAt1End
            | Pat::CalcFunctionEnd
            | Pat::VarFunctionEnd
            | Pat::ColorFunctionsAt0End
            | Pat::ColorFunctionsAt1End
            | Pat::ColorFunctionsAt2End
            | Pat::LessColorBlendFunctionsAt0End
            | Pat::LessColorChannelFunctionsAt0End
            | Pat::LessColorDefinitionFunctionsAt0End
            | Pat::LessColorDefinitionFunctionsAt1End
            | Pat::LessColorOperationFunctionsAt0End
            | Pat::LessColorOperationFunctionsAt1End
            | Pat::LessColorOperationFunctionsAt2End
            | Pat::LessColorOperationFunctionsAt3End
            | Pat::LessColorOperationFunctionsAt4End
            | Pat::LessColorOperationFunctionsAt5End
            | Pat::LessIfFunctionEnd
            | Pat::CounterFunctionsAt0End
            | Pat::CounterFunctionsAt1End
            | Pat::CrossFadeFunctionAt0End
            | Pat::LessListFunctionsAt0End
            | Pat::FilterFunctionEnd
            | Pat::GradientFunctionsAt0End
            | Pat::LessListFunctionsAt1End
            | Pat::FitContentFunctionEnd
            | Pat::FormatFunctionAt0End
            | Pat::GradientFunctionsAt1End
            | Pat::LessListFunctionsAt2End
            | Pat::GridRepeatFunctionEnd
            | Pat::MinmaxFunctionEnd
            | Pat::ImageFunctionEnd
            | Pat::UrlFunctionEnd
            | Pat::LessMathFunctionsAt0End
            | Pat::LessMathFunctionsAt2End
            | Pat::LessMiscFunctionsAt0End
            | Pat::LessMiscFunctionsAt1End
            | Pat::LessMiscFunctionsAt2End
            | Pat::LessMiscFunctionsAt3End
            | Pat::LessMiscFunctionsAt5End
            | Pat::LessMiscFunctionsAt6End
            | Pat::LessStringFunctionsAt0End
            | Pat::LessStringFunctionsAt1End
            | Pat::LessStringFunctionsAt2End
            | Pat::LessTypeFunctionsAt0End
            | Pat::LessTypeFunctionsAt1End
            | Pat::LessTypeFunctionsAt2End
            | Pat::LocalFunctionEnd
            | Pat::RegexpFunctionEnd
            | Pat::ShapeFunctionsAt0End
            | Pat::ShapeFunctionsAt1End
            | Pat::ShapeFunctionsAt2End
            | Pat::ShapeFunctionsAt3End
            | Pat::StepsFunctionEnd
            | Pat::SymbolsFunctionEnd
            | Pat::TransformFunctionsAt0End
            | Pat::TransformFunctionsAt1End
            | Pat::TransformFunctionsAt2End
            | Pat::TransformFunctionsAt3End
            | Pat::TransformFunctionsAt4End
            | Pat::TransformFunctionsAt5End
            | Pat::TransformFunctionsAt6End
            | Pat::TransformFunctionsAt7End
            | Pat::LessMathAt1End
            | Pat::FilterFunctionsAt1End
            | Pat::FilterFunctionsAt2End
            | Pat::FilterFunctionsAt3End
            | Pat::FilterFunctionsAt4End
            | Pat::PseudoSelectorsAt0End
            | Pat::PseudoSelectorsAt1End
            | Pat::PseudoSelectorsAt2End
            | Pat::PseudoSelectorsAt3End
            | Pat::PseudoSelectorsAt4End
            | Pat::PseudoSelectorsAt6End
            | Pat::PseudoSelectorsAt7End
            | Pat::LessExtendEnd
            | Pat::AtContainerAt0At0At1End
            | Pat::StyleFunctionEnd
            | Pat::MediaQueryAt2End
            | Pat::AtImportAt3End
            | Pat::AtSupportsParensEnd
            | Pat::ScrollFunctionEnd
            | Pat::ViewFunctionEnd => self.lit(p, b")"),
            Pat::LessMixinGuardsAt0At0
            | Pat::AttrFunctionAt0
            | Pat::LessBooleanFunctionAt0
            | Pat::LessLogicalComparisonsAt1
            | Pat::CalcFunctionAt0
            | Pat::VarFunctionAt0
            | Pat::ColorFunctionsAt0At0
            | Pat::ColorFunctionsAt1At0
            | Pat::ColorFunctionsAt2At0
            | Pat::LessColorBlendFunctionsAt0At0
            | Pat::LessColorChannelFunctionsAt0At0
            | Pat::LessColorDefinitionFunctionsAt0At0
            | Pat::LessColorDefinitionFunctionsAt1At0
            | Pat::LessColorOperationFunctionsAt0At0
            | Pat::LessColorOperationFunctionsAt1At0
            | Pat::LessColorOperationFunctionsAt2At0
            | Pat::LessColorOperationFunctionsAt3At0
            | Pat::LessColorOperationFunctionsAt4At0
            | Pat::LessColorOperationFunctionsAt5At0
            | Pat::LessIfFunctionAt0
            | Pat::CounterFunctionsAt0At0
            | Pat::CounterFunctionsAt1At0
            | Pat::CrossFadeFunctionAt0At0
            | Pat::LessListFunctionsAt0At0
            | Pat::FilterFunctionAt0
            | Pat::GradientFunctionsAt0At0
            | Pat::LessListFunctionsAt1At0
            | Pat::FitContentFunctionAt0
            | Pat::FormatFunctionAt0At0
            | Pat::GradientFunctionsAt1At0
            | Pat::LessListFunctionsAt2At0
            | Pat::GridRepeatFunctionAt0
            | Pat::MinmaxFunctionAt0
            | Pat::ImageFunctionAt0
            | Pat::UrlFunctionAt0
            | Pat::LessMathFunctionsAt0At0
            | Pat::LessMathFunctionsAt2At0
            | Pat::LessMiscFunctionsAt0At0
            | Pat::LessMiscFunctionsAt1At0
            | Pat::LessMiscFunctionsAt2At0
            | Pat::LessMiscFunctionsAt3At0
            | Pat::LessMiscFunctionsAt5At0
            | Pat::LessMiscFunctionsAt6At0
            | Pat::LessStringFunctionsAt0At0
            | Pat::LessStringFunctionsAt1At0
            | Pat::LessStringFunctionsAt2At0
            | Pat::LessTypeFunctionsAt0At0
            | Pat::LessTypeFunctionsAt1At0
            | Pat::LessTypeFunctionsAt2At0
            | Pat::LocalFunctionAt0
            | Pat::RegexpFunctionAt0
            | Pat::ShapeFunctionsAt0At0
            | Pat::ShapeFunctionsAt1At0
            | Pat::ShapeFunctionsAt2At0
            | Pat::ShapeFunctionsAt3At0
            | Pat::SymbolsFunctionAt0
            | Pat::TransformFunctionsAt0At0
            | Pat::TransformFunctionsAt1At0
            | Pat::TransformFunctionsAt2At0
            | Pat::TransformFunctionsAt3At0
            | Pat::TransformFunctionsAt4At0
            | Pat::TransformFunctionsAt5At0
            | Pat::TransformFunctionsAt6At0
            | Pat::TransformFunctionsAt7At0
            | Pat::LessMathAt1
            | Pat::FilterFunctionsAt1At0
            | Pat::FilterFunctionsAt2At0
            | Pat::FilterFunctionsAt3At0
            | Pat::FilterFunctionsAt4At0
            | Pat::PseudoSelectorsAt0At0
            | Pat::PseudoSelectorsAt1At0
            | Pat::PseudoSelectorsAt2At0
            | Pat::PseudoSelectorsAt3At0
            | Pat::PseudoSelectorsAt4At0
            | Pat::PseudoSelectorsAt6At0
            | Pat::PseudoSelectorsAt7At0
            | Pat::LessExtendAt0
            | Pat::AtContainerAt0At0At1
            | Pat::StyleFunctionAt0
            | Pat::AtImportAt3
            | Pat::AtSupportsParens
            | Pat::ViewFunctionAt0 => self.lit(p, b"("),
            Pat::LessMixinGuardsAt0At0End
            | Pat::AttrFunctionAt0End
            | Pat::AttrFunctionAt0At2End
            | Pat::LessBooleanFunctionAt0End
            | Pat::CalcFunctionAt0End
            | Pat::VarFunctionAt0End
            | Pat::ColorFunctionsAt0At0End
            | Pat::ColorFunctionsAt1At0End
            | Pat::ColorFunctionsAt2At0End
            | Pat::LessColorBlendFunctionsAt0At0End
            | Pat::LessColorChannelFunctionsAt0At0End
            | Pat::LessColorDefinitionFunctionsAt0At0End
            | Pat::LessColorDefinitionFunctionsAt1At0End
            | Pat::LessColorOperationFunctionsAt0At0End
            | Pat::LessColorOperationFunctionsAt1At0End
            | Pat::LessColorOperationFunctionsAt2At0End
            | Pat::LessColorOperationFunctionsAt3At0End
            | Pat::LessColorOperationFunctionsAt4At0End
            | Pat::LessColorOperationFunctionsAt5At0End
            | Pat::LessIfFunctionAt0End
            | Pat::CounterFunctionsAt0At0End
            | Pat::CounterFunctionsAt0At0At4End
            | Pat::CounterFunctionsAt1At0End
            | Pat::CounterFunctionsAt1At0At1End
            | Pat::CrossFadeFunctionAt0At0End
            | Pat::LessListFunctionsAt0At0End
            | Pat::FilterFunctionAt0End
            | Pat::GradientFunctionsAt0At0End
            | Pat::LessListFunctionsAt1At0End
            | Pat::FitContentFunctionAt0End
            | Pat::FormatFunctionAt0At0End
            | Pat::GradientFunctionsAt1At0End
            | Pat::LessListFunctionsAt2At0End
            | Pat::GridRepeatFunctionAt0End
            | Pat::MinmaxFunctionAt0End
            | Pat::ImageFunctionAt0End
            | Pat::UrlFunctionAt0End
            | Pat::LessMathFunctionsAt0At0End
            | Pat::LessMathFunctionsAt2At0End
            | Pat::LessMiscFunctionsAt0At0End
            | Pat::LessMiscFunctionsAt1At0End
            | Pat::LessMiscFunctionsAt2At0End
            | Pat::LessMiscFunctionsAt3At0End
            | Pat::LessMiscFunctionsAt5At0End
            | Pat::LessMiscFunctionsAt6At0End
            | Pat::LessStringFunctionsAt0At0End
            | Pat::LessStringFunctionsAt1At0End
            | Pat::LessStringFunctionsAt2At0End
            | Pat::LessTypeFunctionsAt0At0End
            | Pat::LessTypeFunctionsAt1At0End
            | Pat::LessTypeFunctionsAt2At0End
            | Pat::LocalFunctionAt0End
            | Pat::RegexpFunctionAt0End
            | Pat::ShapeFunctionsAt0At0End
            | Pat::ShapeFunctionsAt1At0End
            | Pat::ShapeFunctionsAt2At0End
            | Pat::ShapeFunctionsAt3At0End
            | Pat::SymbolsFunctionAt0End
            | Pat::TransformFunctionsAt0At0End
            | Pat::TransformFunctionsAt1At0End
            | Pat::TransformFunctionsAt2At0End
            | Pat::TransformFunctionsAt3At0End
            | Pat::TransformFunctionsAt4At0End
            | Pat::TransformFunctionsAt5At0End
            | Pat::TransformFunctionsAt6At0End
            | Pat::TransformFunctionsAt7At0End
            | Pat::FilterFunctionsAt1At0End
            | Pat::FilterFunctionsAt2At0End
            | Pat::FilterFunctionsAt3At0End
            | Pat::FilterFunctionsAt4At0End
            | Pat::PseudoSelectorsAt0At0End
            | Pat::PseudoSelectorsAt1At0End
            | Pat::PseudoSelectorsAt2At0End
            | Pat::PseudoSelectorsAt3At0End
            | Pat::PseudoSelectorsAt4At0End
            | Pat::PseudoSelectorsAt6At0End
            | Pat::PseudoSelectorsAt7At0End
            | Pat::LessExtendAt0End
            | Pat::StyleFunctionAt0End
            | Pat::ViewFunctionAt0End => (self.at(p) == b')').then_some(p),
            Pat::LessVariableComparisonAt0 => self.variable_reference(p),
            Pat::LessVariableComparisonAt0End => self.spaces_then(p, b")"),
            Pat::LessVariableComparisonAt0At0 | Pat::LessLogicalComparisonsAt0 => {
                self.comparison_operator(p)
            }
            Pat::LessVariableComparisonAt0At1 => self.keyword_out_of_line(p, b"true"),
            Pat::AttrFunction => self.func(p, &[b"attr"]),
            Pat::QualifiedName => self.qualified_name(p),
            Pat::LiteralStringAt0 => self.lit(p, b"'"),
            Pat::LiteralStringAt0End => self.string_end(p, b'\''),
            Pat::LessVariableInterpolation => self.interpolation(p),
            Pat::StringContentAt1 => self.line_continuation(p),
            Pat::StringContentAt2 => self.string_escape(p),
            Pat::LiteralStringAt1 => self.lit(p, b"\""),
            Pat::LiteralStringAt1End => self.string_end(p, b'"'),
            Pat::LessStringsAt0 => self.escaped_string(p),
            Pat::LessStringsAt0End => self.string_end_any(p),
            Pat::AttrFunctionAt0At2
            | Pat::CounterFunctionsAt1At0At0
            | Pat::SelectorsAt10At2
            | Pat::AtNamespaceAt2 => self.name(p, Class::Alpha, Class::DashWord, Escape::Hex),
            Pat::AttrFunctionAt0At2At0 | Pat::LessTypeFunctionsAt1At0At2 => self.unit_name(p),
            Pat::CommaDelimiter => self.separator(p, b','),
            Pat::PropertyValueConstantsAt0 => self.alt(
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
            Pat::PropertyValueConstantsAt1 => self.alt(
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
            Pat::GlobalPropertyValues => self.alt(
                p,
                &[b"initial", b"inherit", b"unset", b"revert-layer", b"revert"],
            ),
            Pat::CubicBezierFunction => self.func_open(p, &[b"cubic-bezier"]),
            Pat::LessBooleanFunction => self.func(p, &[b"boolean"]),
            Pat::LessLogicalComparisonsAt2 => self.true_or_false(p),
            Pat::LessLogicalComparisonsAt3
            | Pat::LessVariableComparisonAt0At5
            | Pat::RuleListBodyAt5At5At3 => self.lit(p, b","),
            Pat::CalcFunction => self.func(p, &[b"calc"]),
            Pat::VarFunction => self.func(p, &[b"var"]),
            Pat::CustomPropertyName => self.custom_property(p),
            Pat::LessVariablesAt0 => self.variable(p),
            Pat::ColorFunctionsAt0 => self.func(p, &[b"rgba", b"rgb"]),
            Pat::ValueSeparator => self.separator(p, b'/'),
            Pat::PercentageType => self.percentage(p),
            Pat::NumberType => self.number(p),
            Pat::ColorFunctionsAt1 => self.func(
                p,
                &[b"hsla", b"hsl", b"hwb", b"oklab", b"oklch", b"lab", b"lch"],
            ),
            Pat::ColorFunctionsAt2 => self.func(p, &[b"light-dark"]),
            Pat::LessColorBlendFunctionsAt0 => self.func(
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
            Pat::LessColorChannelFunctionsAt0 => self.func(
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
            Pat::LessColorDefinitionFunctionsAt0 => self.func(p, &[b"argb"]),
            Pat::LessColorDefinitionFunctionsAt1 => self.func(p, &[b"hsva", b"hsv"]),
            Pat::IntegerType => self.integer(p),
            Pat::LessColorOperationFunctionsAt0 => self.func(p, &[b"fade", b"shade", b"tint"]),
            Pat::LessColorOperationFunctionsAt1 => self.func(p, &[b"spin"]),
            Pat::LessColorOperationFunctionsAt2 => self.func(
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
            Pat::LessColorOperationFunctionsAt3 => self.func(p, &[b"contrast"]),
            Pat::LessColorOperationFunctionsAt4 => self.func(p, &[b"greyscale"]),
            Pat::LessColorOperationFunctionsAt5 => self.func(p, &[b"mix"]),
            Pat::LessIfFunction => self.func(p, &[b"if"]),
            Pat::CounterFunctionsAt0 => self.func(p, &[b"counter"]),
            Pat::CounterFunctionsAt0At0At3 => self.counter_name(p),
            Pat::CounterFunctionsAt0At0At4 | Pat::CounterFunctionsAt1At0At1 => {
                (self.at(p) == b',').then_some(p)
            }
            Pat::CounterFunctionsAt0At0At4At1 | Pat::CounterFunctionsAt1At0At1At5 => {
                self.table_word(p, words::COUNTER_STYLES)
            }
            Pat::CounterFunctionsAt1 => self.func(p, &[b"counters"]),
            Pat::CrossFadeFunctionAt0 => self.func(p, &[b"cross-fade"]),
            Pat::LessListFunctionsAt0 => self.func(p, &[b"length"]),
            Pat::FilterFunction => self.func(p, &[b"filter"]),
            Pat::GradientFunctionsAt0 => {
                self.func(p, &[b"repeating-linear-gradient", b"linear-gradient"])
            }
            Pat::AngleType => self.dimension(p, &[b"deg", b"grad", b"rad", b"turn"]),
            Pat::LessListFunctionsAt1 => self.func(p, &[b"extract"]),
            Pat::FitContentFunction => self.func(p, &[b"fit-content"]),
            Pat::LengthTypeAt0 => self.length(p),
            Pat::LengthTypeAt1 => self.zero(p),
            Pat::FormatFunctionAt0 => self.func(p, &[b"format"]),
            Pat::GradientFunctionsAt1 => {
                self.func(p, &[b"repeating-radial-gradient", b"radial-gradient"])
            }
            Pat::LessListFunctionsAt2 => self.func(p, &[b"range"]),
            Pat::GridRepeatFunction => self.func(p, &[b"repeat"]),
            Pat::MinmaxFunction => self.func(p, &[b"minmax"]),
            Pat::MinmaxFunctionAt0At4 => self.alt(p, &[b"max-content", b"min-content"]),
            Pat::GridRepeatFunctionAt0At6 => self.alt(p, &[b"auto-fill", b"auto-fit"]),
            Pat::GridRepeatFunctionAt0At7 => {
                self.alt(p, &[b"max-content", b"min-content", b"auto"])
            }
            Pat::ImageFunction => self.func(p, &[b"image"]),
            Pat::UrlFunction => self.func(p, &[b"url"]),
            Pat::UnquotedString => self.unquoted_char(p, b""),
            Pat::LessMathFunctionsAt0 => self.func(
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
            Pat::FrequencyType => self.dimension(p, &[b"Hz", b"kHz"]),
            Pat::TimeType => self.dimension(p, &[b"s", b"ms"]),
            Pat::LessMathFunctionsAt1 => self.pi(p),
            Pat::LessMathFunctionsAt2 => self.func(p, &[b"pow", b"mod", b"min", b"max"]),
            Pat::LessMiscFunctionsAt0 => self.func(p, &[b"color"]),
            Pat::LessMiscFunctionsAt1 => {
                self.func(p, &[b"image-size", b"image-width", b"image-height"])
            }
            Pat::LessMiscFunctionsAt2 => self.func(p, &[b"convert", b"unit"]),
            Pat::LessMiscFunctionsAt2At0At4 => self.first_of(
                p,
                &[
                    b"cm", b"mm", b"m", b"in", b"pc", b"pt", b"px", b"ms", b"s", b"grad", b"rad",
                    b"deg", b"turn", b"%", b"rem", b"em", b"ex", b"ch",
                ],
            ),
            Pat::LessMiscFunctionsAt3 => self.func(p, &[b"data-uri"]),
            Pat::LessMiscFunctionsAt3At0At2 | Pat::ArbitraryRepetition => self.spaced_lit(p, b','),
            Pat::LessMiscFunctionsAt4 => self.default_word(p),
            Pat::LessMiscFunctionsAt5 => self.func(p, &[b"get-unit"]),
            Pat::LessMiscFunctionsAt6 => self.func(p, &[b"svg-gradient"]),
            Pat::LessStringFunctionsAt0 => self.func(p, &[b"escape", b"e"]),
            Pat::LessStringFunctionsAt1 => self.format_function(p),
            Pat::LessStringFunctionsAt2 => self.func(p, &[b"replace"]),
            Pat::LessTypeFunctionsAt0 => self.func(
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
            Pat::LessTypeFunctionsAt1 => self.func(p, &[b"isunit"]),
            Pat::LessTypeFunctionsAt2 => self.func(p, &[b"isdefined"]),
            Pat::LocalFunction => self.func(p, &[b"local"]),
            Pat::RegexpFunction => self.func(p, &[b"regexp"]),
            Pat::ShapeFunctionsAt0 => self.func(p, &[b"rect"]),
            Pat::ShapeFunctionsAt0At0At0 => self.keyword_out_of_line(p, b"auto"),
            Pat::ShapeFunctionsAt1 => self.func(p, &[b"inset"]),
            Pat::ShapeFunctionsAt1At0At0 => self.keyword_out_of_line(p, b"round"),
            Pat::ShapeFunctionsAt2 => self.func(p, &[b"circle", b"ellipse"]),
            Pat::ShapeFunctionsAt2At0At0 => self.keyword_out_of_line(p, b"at"),
            Pat::ShapeFunctionsAt2At0At1 => self.alt(
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
            Pat::ShapeFunctionsAt3 => self.func(p, &[b"polygon"]),
            Pat::ShapeFunctionsAt3At0At0 => self.alt(p, &[b"nonzero", b"evenodd"]),
            Pat::StepsFunction => self.func_open(p, &[b"steps"]),
            Pat::StepsFunctionAt0 => self.first_of(
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
            Pat::SymbolsFunction => self.func(p, &[b"symbols"]),
            Pat::SymbolsFunctionAt0At0 => self.alt(
                p,
                &[b"cyclic", b"numeric", b"alphabetic", b"symbolic", b"fixed"],
            ),
            Pat::TransformFunctionsAt0 => {
                self.func(p, &[b"matrix3d", b"matrix", b"scale3d", b"scale"])
            }
            Pat::TransformFunctionsAt1 => self.func(p, &[b"translate3d", b"translate"]),
            Pat::TransformFunctionsAt2 => self.func(p, &[b"translateX", b"translateY"]),
            Pat::TransformFunctionsAt3 => self.func(
                p,
                &[
                    b"rotateX", b"rotateY", b"rotateZ", b"rotate", b"skewX", b"skewY",
                ],
            ),
            Pat::TransformFunctionsAt4 => self.func(p, &[b"skew"]),
            Pat::TransformFunctionsAt5 => self.func(p, &[b"translateZ", b"perspective"]),
            Pat::TransformFunctionsAt6 => self.func(p, &[b"rotate3d"]),
            Pat::TransformFunctionsAt7 => self.func(p, &[b"scaleX", b"scaleY", b"scaleZ"]),
            Pat::UnicodeRange => self.unicode_range(p),
            Pat::ColorValuesAt4 => self.alt(
                p,
                &[
                    b"aqua", b"black", b"blue", b"fuchsia", b"gray", b"green", b"lime", b"maroon",
                    b"navy", b"olive", b"orange", b"purple", b"red", b"silver", b"teal", b"white",
                    b"yellow",
                ],
            ),
            Pat::ColorValuesAt5 => self.table_word(p, words::EXTENDED_COLORS),
            Pat::ColorValuesAt6 => self.current_color(p),
            Pat::ColorValuesAt7 => self.hex_color(p),
            Pat::RelativeColorAt0 => self.lit(p, b"from"),
            Pat::RelativeColorAt1 => self.channel_letter(p),
            Pat::PropertyValueConstantsAt5 => self.alt(p, &[b"replace", b"add", b"accumulate"]),
            Pat::PropertyValueConstantsAt6 => self.alt(
                p,
                &[b"normal", b"alternate-reverse", b"alternate", b"reverse"],
            ),
            Pat::PropertyValueConstantsAt7 => self.alt(p, &[b"forwards", b"backwards", b"both"]),
            Pat::PropertyValueConstantsAt8 => self.keyword_out_of_line(p, b"infinite"),
            Pat::PropertyValueConstantsAt9 => self.alt(p, &[b"running", b"paused"]),
            Pat::PropertyValueConstantsAt10 => {
                self.alt(p, &[b"entry-crossing", b"entry", b"exit-crossing", b"exit"])
            }
            Pat::PropertyValueConstantsAt11 => self.alt(
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
            Pat::PropertyValueConstantsAt12 => self.table_word(p, words::VALUE_KEYWORDS),
            Pat::PropertyValueConstantsAt13 => self.generic_family(p),
            Pat::LessMathAt0 => matches!(self.at(p), b'-' | b'*' | b'+' | b'/').then_some(p + 1),
            Pat::Important => self.important(p),
            Pat::LessMiscFunctionsAt6At0At5 | Pat::GradientFunctionsAt0At0At7 => {
                self.keyword_out_of_line(p, b"to")
            }
            Pat::LessMiscFunctionsAt6At0At6 => {
                self.alt(p, &[b"top", b"right", b"bottom", b"left", b"center"])
            }
            Pat::LessMiscFunctionsAt6At0At7 | Pat::GradientFunctionsAt1At0At6 => {
                self.alt(p, &[b"at", b"circle", b"ellipse"])
            }
            Pat::GradientFunctionsAt1At0At7 => self.alt(
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
            Pat::GradientFunctionsAt0At0At8 => self.alt(p, &[b"top", b"right", b"bottom", b"left"]),
            Pat::FilterFunctionsAt1 => self.func(p, &[b"blur"]),
            Pat::FilterFunctionsAt2 => self.func(
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
            Pat::FilterFunctionsAt3 => self.func(p, &[b"drop-shadow"]),
            Pat::FilterFunctionsAt4 => self.func(p, &[b"hue-rotate"]),
            Pat::LessColorOperationFunctionsAt2At0At3 => self.keyword_out_of_line(p, b"relative"),
            Pat::SelectorsAt0 => self.custom_element(p),
            Pat::SelectorsAt1 => self.table_word(p, words::TAG_NAMES),
            Pat::SelectorsAt2 => self.group_lit(p, b"."),
            Pat::SelectorsAt2End | Pat::SelectorsAt3End | Pat::SelectorsAt4End => self.name_ends(p),
            Pat::SelectorsAt3 => self.group_lit(p, b"#"),
            Pat::SelectorsAt4 => self.group_lit(p, b"&"),
            Pat::PseudoSelectorsAt0 => self.pseudo_func(p, &[b"dir"]),
            Pat::PseudoSelectorsAt0At0At0 => self.first_of(p, &[b"ltr", b"rtl"]),
            Pat::PseudoSelectorsAt1 => self.pseudo_func(p, &[b"lang"]),
            Pat::PseudoSelectorsAt2 => self.pseudo_func(p, &[b"not"]),
            Pat::PseudoSelectorsAt3 => self.pseudo_func(
                p,
                &[
                    b"nth-last-child",
                    b"nth-last-of-type",
                    b"nth-child",
                    b"nth-of-type",
                ],
            ),
            Pat::PseudoSelectorsAt3At0At0 => self.alt(p, &[b"even", b"odd"]),
            Pat::PseudoSelectorsAt3At0At1 => self.an_plus_b(p),
            Pat::PseudoSelectorsAt4 => self.pseudo_func(
                p,
                &[b"host-context", b"host", b"has", b"is", b"not", b"where"],
            ),
            Pat::PseudoSelectorsAt5 => self.pseudo_class(p),
            Pat::PseudoSelectorsAt6 => {
                self.pseudo_element_func(p, &[b"highlight", b"part", b"state"])
            }
            Pat::PseudoSelectorsAt6At0At0
            | Pat::RuleListBodyAt5At12At0At3
            | Pat::AtContainerAt0At0At3 => self.dashes_or_name(p),
            Pat::PseudoSelectorsAt7 => self.pseudo_element_func(p, &[b"slotted"]),
            Pat::PseudoSelectorsAt8 => self.pseudo_element(p),
            Pat::PseudoSelectorsAt9 => self.vendor_pseudo(p),
            Pat::LessExtend => self.pseudo_func(p, &[b"extend"]),
            Pat::LessExtendAt0At0 => self.keyword_out_of_line(p, b"all"),
            Pat::SelectorsAt7 => self.combinator(p),
            Pat::SelectorsAt8 => self.combinator_run(p),
            Pat::SelectorsAt9 => self.lit(p, b"/deep/"),
            Pat::SelectorsAt10 => self.lit(p, b"["),
            Pat::SelectorsAt10End | Pat::SelectorsAt10At3At4 => self.lit(p, b"]"),
            Pat::SelectorsAt10At3 => self.attribute_operator(p),
            Pat::SelectorsAt10At3End => self.attribute_value_end(p),
            Pat::SelectorsAt10At3At1 => self.unquoted_char(p, b"[]"),
            Pat::SelectorsAt10At3At3 => self.attribute_flag(p),
            Pat::SelectorsAt12 => self.lit(p, b"*"),
            Pat::LessMixinGuardsAt0At0At1 => self.default_call(p),
            Pat::LessNamespaceAccessorsAt1 => self.group_lit(p, b"("),
            Pat::LessNamespaceAccessorsAt1End => self.group_lit(p, b")"),
            Pat::LessVariableAssignmentAt0 => self.variable_name(p),
            Pat::LessVariableAssignmentAt0End => self.assignment_end(p),
            Pat::LessVariableAssignmentAt0At0
            | Pat::RuleListBodyAt5At5At0
            | Pat::RuleListBodyAt5At9At0
            | Pat::RuleListBodyAt5At10At0
            | Pat::RuleListBodyAt5At11At0 => self.merge_colon_blanks(p),
            Pat::PropertyListAt0 | Pat::AtContainerAt0End => (self.at(p) == b'{').then_some(p),
            Pat::PropertyListAt0End => self.lit(p, b"}"),
            Pat::RuleListAt0 => self.lit(p, b"{"),
            Pat::RuleListAt0End => self.rule_list_end(p),
            Pat::RuleListAt0At0
            | Pat::RuleListBodyAt5At1End
            | Pat::RuleListBodyAt5At3End
            | Pat::RuleListBodyAt5At5End
            | Pat::RuleListBodyAt5At6End
            | Pat::RuleListBodyAt5At7End
            | Pat::RuleListBodyAt5At8End
            | Pat::RuleListBodyAt5At9End
            | Pat::RuleListBodyAt5At10End
            | Pat::RuleListBodyAt5At11End
            | Pat::RuleListBodyAt5At12End
            | Pat::RuleListBodyAt6End => self.semicolon_or_close(p, Spaces::Before),
            Pat::AtCharset => self.at_rule_word(p, b"charset"),
            Pat::AtCharsetEnd => self.semicolon_or_eol_ahead(p),
            Pat::AtContainer => self.at_rule_ahead(p, b"container", Tail::Any),
            Pat::AtContainerEnd
            | Pat::AtCounterStyleEnd
            | Pat::AtFontFaceEnd
            | Pat::AtMediaEnd
            | Pat::AtKeyframesEnd => self.spaced_lit(p, b'}'),
            Pat::AtContainerAt0 => self.at_keyword(p, b"container"),
            Pat::AtContainerAt0At0 | Pat::MediaQueryList | Pat::AtKeyframesAt1 => {
                self.spaces_before_other(p)
            }
            Pat::AtContainerAt0At0End | Pat::MediaQueryListEnd => self.spaces_then(p, b";{"),
            Pat::AtContainerAt0At0At0 => self.alt(p, &[b"not", b"and", b"or"]),
            Pat::AtContainerAt0At0At1At0 => self.alt_cap(
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
            Pat::AtContainerAt0At0At1At1 => self.comparison(p),
            Pat::AtContainerAt0At0At1At2 => self.lit(p, b":"),
            Pat::AtContainerAt0At0At1At3 => self.first_of(p, &[b"portrait", b"landscape"]),
            Pat::AtContainerAt0At0At1At5 => self.lit(p, b"/"),
            Pat::StyleFunction => self.func(p, &[b"style"]),
            Pat::AtCounterStyle => self.at_counter_style(p),
            Pat::AtCustomMedia => self.at_rule_ahead(p, b"custom-media", Tail::Boundary),
            Pat::AtCustomMediaEnd | Pat::AtImportAt2End => self.spaces_then(p, b";"),
            Pat::AtCustomMediaAt0 => self.through_semicolon(p),
            Pat::AtCustomMediaAt1 => self.spaced_at_keyword(p, b"custom-media"),
            Pat::MediaQuery => self.media_query(p),
            Pat::MediaQueryAt2 => self.media_feature_open(p),
            Pat::MediaQueryAt2At0 => self.media_feature_name(p),
            Pat::MediaQueryAt2At0End => self.feature_name_end(p),
            Pat::MediaQueryAt2At1 => self.feature_keyword(p),
            Pat::MediaQueryAt2At2 => self.ratio(p),
            Pat::AtFontFace => self.at_font_face(p),
            Pat::AtMedia => self.at_rule_ahead(p, b"media", Tail::Boundary),
            Pat::AtMediaAt0 => self.spaced_at_keyword(p, b"media"),
            Pat::AtMediaAt0End | Pat::AtSupportsAt0End | Pat::AtKeyframesAt1End => {
                self.spaces_then(p, b"{")
            }
            Pat::AtMediaAt1
            | Pat::AtKeyframesAt0
            | Pat::AtSupportsAt1
            | Pat::AtContainerAt1
            | Pat::LessNamespaceAccessorsAt0At3 => self.spaced_lit(p, b'{'),
            Pat::AtMediaAt1End
            | Pat::AtKeyframesAt0End
            | Pat::AtSupportsAt1End
            | Pat::AtContainerAt1End
            | Pat::LessNamespaceAccessorsAt0At3End => (self.at(p) == b'}').then_some(p),
            Pat::AtImport => self.at_rule_word(p, b"import"),
            Pat::AtImportEnd | Pat::AtNamespaceEnd => self.lit(p, b";"),
            Pat::AtImportAt2 => self.after_import_url(p),
            Pat::AtImportAt3At0 => self.first_of(
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
            Pat::AtKeyframes => self.at_keyframes(p),
            Pat::AtKeyframesAt0At0 => self.keyframe_selector(p),
            Pat::LessNamespaceAccessorsAt2 => self.semicolon_or_close(p, Spaces::None),
            Pat::AtNamespace => self.at_namespace(p),
            Pat::AtPage => self.at_page(p),
            Pat::AtSupports => self.at_rule_ahead(p, b"supports", Tail::Boundary),
            Pat::AtSupportsEnd => self.group_lit(p, b"}"),
            Pat::AtSupportsAt0 => self.spaced_at_keyword(p, b"supports"),
            Pat::AtSupportsOperators => self.alt(p, &[b"and", b"or", b"not"]),
            Pat::RuleListBodyAt4 => self.interpolated_property_ahead(p),
            Pat::RuleListBodyAt4End
            | Pat::RuleListBodyAt4At1End
            | Pat::RuleListBodyAt5At1At0End
            | Pat::RuleListBodyAt5At3At0End
            | Pat::RuleListBodyAt5At6At0End
            | Pat::RuleListBodyAt5At7At0End
            | Pat::RuleListBodyAt5At8At0End
            | Pat::RuleListBodyAt5At12At0End => self.semicolon_or_close_ahead(p),
            Pat::RuleListBodyAt4At0 => self.not_colon_or_space(p),
            Pat::RuleListBodyAt4At0End => self.merge_colon_ahead(p),
            Pat::RuleListBodyAt4At1
            | Pat::RuleListBodyAt5At1At0
            | Pat::RuleListBodyAt5At3At0
            | Pat::RuleListBodyAt5At6At0
            | Pat::RuleListBodyAt5At7At0
            | Pat::RuleListBodyAt5At8At0
            | Pat::RuleListBodyAt5At12At0 => self.merge_colon(p),
            Pat::RuleListBodyAt5 => matches!(self.at(p), b'-' | b'a'..=b'z').then_some(p),
            Pat::RuleListBodyAt5End => {
                (self.at_eol(p) || !matches!(self.at(p), b'-' | b'a'..=b'z')).then_some(p)
            }
            Pat::RuleListBodyAt5At1 => self.vendor_property(p),
            Pat::RuleListBodyAt5At1At0At1 => self.dash_word_run(p),
            Pat::RuleListBodyAt5At3 => self.border_radius(p),
            Pat::RuleListBodyAt5At4 => self.var_property(p),
            Pat::RuleListBodyAt5At5 => self.font_property(p),
            Pat::RuleListBodyAt5At5At2 => self.family_name(p),
            Pat::RuleListBodyAt5At6 => self.keyword_out_of_line(p, b"animation-timeline"),
            Pat::ScrollFunction => self.func_open(p, &[b"scroll"]),
            Pat::ScrollFunctionAt0 => self.first_of(p, &[b"root", b"nearest", b"self"]),
            Pat::ScrollFunctionAt1 => self.first_of(p, &[b"block", b"inline", b"x", b"y"]),
            Pat::ViewFunction => self.func(p, &[b"view"]),
            Pat::ViewFunctionAt0At0 => {
                self.first_of(p, &[b"block", b"inline", b"x", b"y", b"auto"])
            }
            Pat::RuleListBodyAt5At7 => self.animation_property(p),
            Pat::RuleListBodyAt5At7At0At6 => self.animation_name(p),
            Pat::RuleListBodyAt5At8 => self.alt_cap(
                p,
                &[
                    b"transition-property",
                    b"transition-duration",
                    b"transition-delay",
                    b"transition-timing-function",
                    b"transition",
                ],
            ),
            Pat::RuleListBodyAt5At9 => self.alt(p, &[b"backdrop-filter", b"filter"]),
            Pat::RuleListBodyAt5At9At1 => self.alt(p, &[b"inherit", b"initial", b"unset", b"none"]),
            Pat::RuleListBodyAt5At10 => self.keyword_out_of_line(p, b"will-change"),
            Pat::RuleListBodyAt5At10At1 => self.first_of(
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
            Pat::RuleListBodyAt5At10At2 | Pat::RuleListBodyAt5At11At1 => {
                self.name(p, Class::DashWordWide, Class::AlphaWide, Escape::Hex)
            }
            Pat::RuleListBodyAt5At11 => {
                self.alt(p, &[b"counter-increment", b"counter-reset", b"counter-set"])
            }
            Pat::RuleListBodyAt5At11At3 => {
                self.first_of(p, &[b"unset", b"initial", b"inherit", b"auto"])
            }
            Pat::RuleListBodyAt5At12 => self.container_property(p),
            Pat::RuleListBodyAt5At12At0At0 => self.keyword_out_of_line(p, b"default"),
            Pat::RuleListBodyAt5At13 => self.table_word(p, words::PROPERTY_NAMES),
            Pat::RuleListBodyAt5At14 => self.size_property(p),
            Pat::SelectorAt0 => self.selector_ahead(p),
            Pat::SelectorAt0End => self.selector_end(p),
            Pat::RuleListBodyAt6 => self.merge_colon_word(p),
            Pat::KeyframeName => self.keyframe_name(p),
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
