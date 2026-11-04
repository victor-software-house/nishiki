//! The matchers: for each regular expression of the grammar, the function that
//! decides whether it matches at a position and where. A matcher records the
//! capture groups the grammar names with [`Lexer::cap`]; the engine names them.
//!
//! Each follows its expression as an Oniguruma-style backtracking matcher
//! would. Where a quantifier could give characters back, the doc comment says
//! why it cannot matter.

mod access;
mod basic;
mod casing;
mod chars;
mod decls;
mod directives;
mod doc;
mod forms;
mod functions;
mod gaps;
mod heads;
mod idents;
mod numbers;
mod params;
mod placeholder;
mod qualified;
mod scopes;
mod spaced;
mod templates;
mod variables;
mod words;

use access::OperatorAt;
use numbers::NumberIn;
use scopes::Callee;
use spaced::Paren;
use variables::VariableForm;

use super::engine::Lexer;
use super::grammar::Rx;
use crate::rules::Chars;

/// Texts that some expressions always contain after where they start.
/// A line without one of them after `p` cannot match the expression, which
/// saves running its matcher at every word of the line.
pub(super) const NEEDS: &[&[u8]] = &[
    b"[",
    b"(",
    b"=",
    b"~",
    b"::",
    b"operator",
    b"{",
    b"]]",
    b")",
    b"(*",
    b".",
    b"->",
    b":",
    b"goto",
    b"tatic_assert",
    b"new",
    b"delete",
    b"pragma",
    b"typename",
    b"#",
];

/// The index in [`NEEDS`] of `(`, which a builtin type's initializer,
/// `GAP(int)GAP(\()`, ends with.
const NEED_OPEN_PAREN: usize = 1;
/// The index in [`NEEDS`] of `:`, which an access specifier,
/// `GAP(public)\s*(:)`, and a label, `GAP(ID)GAP(:)(?!:)`, end with.
const NEED_COLON: usize = 12;
/// The index in [`NEEDS`] of `goto`, the keyword of `GAP(goto)GAP(ID)`.
const NEED_GOTO: usize = 13;
/// The index in [`NEEDS`] of `tatic_assert`, which both of
/// `(?<!\w)static_assert|_Static_assert(?!\w)` contain.
const NEED_STATIC_ASSERT: usize = 14;
/// The index in [`NEEDS`] of `new`, one of the two words of
/// `GAP((?:delete\s*\[]|delete)|new)(?!\w)`.
const NEED_NEW: usize = 15;
/// The index in [`NEEDS`] of `delete`, the other word of the memory
/// operators.
const NEED_DELETE: usize = 16;
/// The index in [`NEEDS`] of `pragma`, which `^GAP(#)\s*pragma\s+mark`
/// spells out.
const NEED_PRAGMA: usize = 17;
/// The index in [`NEEDS`] of `typename`, the keyword the dependent type
/// name `GAP(typename)GAP…` starts with.
const NEED_TYPENAME: usize = 18;
/// The index in [`NEEDS`] of `#`, which the directive heads spell out after
/// their gap: `^GAP(#)\s*(if(?:n?def|))`, `^GAP(#)\s*e(?:ndif|lse|lif…)`,
/// `^GAP(#)\s*(error|warning)`, `^GAP(#)\s*define`, `^GAP(#)\s*undef` and
/// `^GAP(#)\s*include(?:|_next)`.
const NEED_HASH: usize = 19;

/// The capture group, past the groups of the raw string's begin pattern,
/// that holds its delimiter whichever of groups 2 and 3 took it: the rule's
/// payload, which its end pattern `(\)(\2)(\3)")` repeats.
pub(super) const RAW_DELIMITER: u8 = 4;
/// The longest delimiter group 2 of the raw string's begin takes; a longer
/// one is group 3, and the name `string.quoted.double.raw.$2` leaves it out.
pub(super) const RAW_DELIMITER_MAX: usize = 16;

/// The index in [`NEEDS`] of `]]`, which closes an attribute.
pub(super) const NEED_BRACKETS: usize = 7;
/// The index in [`NEEDS`] of `)`.
pub(super) const NEED_PAREN: usize = 8;

/// The texts of [`NEEDS`], by index, of which `rx` always contains one.
#[expect(
    clippy::wildcard_enum_match_arm,
    reason = "few of the expressions have one"
)]
fn need(rx: Rx) -> Option<&'static [usize]> {
    Some(match rx {
        Rx::SimpleArrayAssignment => &[0],
        Rx::FunctionDefinition | Rx::FunctionCallAt0 | Rx::FunctionCallAt1 => &[1],
        Rx::FunctionPointerParameter => &[9],
        Rx::MethodAccess | Rx::MemberAccess => &[10, 11],
        Rx::SimpleConstructorCall => &[1, 6],
        Rx::VariableAssignment | Rx::NormalVariableAssignment => &[2],
        Rx::DestructorRoot => &[3],
        Rx::ConstructorRoot | Rx::ScopeResolutionInnerGenerated => &[4],
        Rx::OperatorOverload => &[5],
        Rx::ConstructorBracketCall | Rx::CurlyInitializer => &[6],
        Rx::BuiltinStorageTypeInitializer => &[NEED_OPEN_PAREN],
        Rx::AccessControlKeywords | Rx::Label => &[NEED_COLON],
        Rx::GotoStatement => &[NEED_GOTO],
        Rx::StaticAssert => &[NEED_STATIC_ASSERT],
        Rx::MemoryOperators => &[NEED_NEW, NEED_DELETE],
        Rx::PragmaMark => &[NEED_PRAGMA],
        Rx::Typename => &[NEED_TYPENAME],
        Rx::PreprocessorConditionalRange
        | Rx::PreprocessorConditionalStandalone
        | Rx::Diagnostic
        | Rx::Macro
        | Rx::Undef
        | Rx::Include => &[NEED_HASH],
        _ => return None,
    })
}

impl Lexer<'_> {
    /// Whether `rx` matches at `p`, and where its match ends.
    #[expect(
        clippy::wildcard_enum_match_arm,
        reason = "the expressions that no hand matcher answers match nowhere"
    )]
    pub(super) fn matcher(&mut self, rx: Rx, p: usize) -> Option<usize> {
        if let Some(base) = rx.macro_base() {
            return self
                .matcher(base, p)
                .or_else(|| self.macro_line_end(p).then_some(p));
        }
        if let Some(needs) = need(rx)
            && !needs.iter().any(|&need| self.has_ahead(need, p))
        {
            return None;
        }
        match rx {
            Rx::CommentsAt0At7Cap3At1 => self.lit(p, b","),
            Rx::DiagnosticAt1 => self.lit(p, b"\""),
            Rx::DiagnosticAt2 => self.lit(p, b"'"),
            Rx::CppAttributesAt4 => self.lit(p, b":"),
            Rx::TemplateCallRange => self.lit(p, b"<"),
            Rx::TemplateCallRangeEnd => self.lit(p, b">"),
            Rx::SimpleArrayAssignmentCap13At2 => self.lit(p, b"&"),
            Rx::ParameterAt10 => self.lit(p, b"="),
            Rx::ParameterAt13End => self.lit(p, b"]"),
            Rx::LambdasAt3End => self.lit(p, b"}"),
            Rx::UsingNamespaceEnd => self.lit(p, b";"),
            Rx::OperatorsAt0End => self.lit(p, b")"),
            Rx::CppAttributesAt1 => self.lit(p, b"("),
            Rx::ConstructorBracketCallCap1At0 => self.lit(p, b"::"),
            Rx::CppAttributesEnd => self.lit(p, b"]]"),
            Rx::SimpleArrayAssignmentCap13At0 => self.lit(p, b"*"),
            Rx::ParameterAt13 => self.lit(p, b"["),
            Rx::LambdasAt2 => self.lit(p, b"->"),
            Rx::LambdasAt3 => self.lit(p, b"{"),
            Rx::OperatorsAt6 => self.lit(p, b"--"),
            Rx::TernaryOperator => self.lit(p, b"?"),
            Rx::BlockCommentEnd => self.lit(p, b"*/"),
            Rx::StringContextAt4End => self.lit(p, b")\""),
            Rx::CppAttributes => self.lit(p, b"[["),
            Rx::OperatorsAt7 => self.lit(p, b"++"),
            Rx::OperatorsAt8 => self.lit_any(p, &[b"<<", b">>"]),
            Rx::MacroAt0Cap2At2 => self.lit(p, b"..."),
            Rx::AlignasAttribute => self.lit(p, b"alignas("),
            Rx::MsAttributes => self.lit(p, b"__declspec("),
            Rx::TypedefClassAt0Cap11At2 => self.lit(p, b"DLLEXPORT"),
            Rx::CommentsAt0At7Cap2At0 => self.lit_any(p, &[b"in", b"out"]),
            Rx::OperatorsAt9 => self.compare_operator(p),
            Rx::OperatorsAt10 => self.lit_any(p, &[b"&&", b"!", b"||"]),
            Rx::OperatorsAt11 => self.one_of(p, b"&^|~"),
            Rx::OperatorsAt13 => self.one_of(p, b"-%*+/"),
            Rx::TypedefClassAt0At1End => self.lit_any(p, &[b"}", b"%>", b"??>"]),
            Rx::TypedefClassAt0At0End => self
                .lit_any(p, &[b"{", b"<%", b"??<"])
                .or_else(|| (self.at(p) == b';').then_some(p)),
            Rx::StringContextAt3End => self.lit_any(p, &[b")glsl\"", b")GLSL\""]),
            Rx::StringContextAt2End => self.lit_any(p, &[b")_r\"", b")re\"", b")regex\""]),

            Rx::PreprocessorNumberLiteralCap0At0End => self.at_eol(p).then_some(p),
            Rx::PreprocessorNumberLiteralCap0At0 => (p < self.lim()).then_some(p),
            Rx::NormalVariableAssignmentEnd => self.ahead(p, b';').then_some(p),
            Rx::LambdasAt2End => self.ahead(p, b'{').then_some(p),
            Rx::StaticAssertAt0End => self.ahead(p, b')').then_some(p),
            Rx::ParameterAt9 => (self.prev(p) == Some(b'=')).then_some(p),
            Rx::TypedefClassEnd => (self.prev(p) == Some(b';')).then_some(p),
            Rx::AssemblyEnd => (!self.at_g(p)).then_some(p),
            Rx::LambdasEnd => matches!(self.prev(p), Some(b';' | b'}')).then_some(p),
            Rx::ParenthesesAt2 => self.single_colon(p),
            Rx::TypedefClassAt0At1 => self.after_open(p).then_some(p),
            Rx::CommentsAt0End => self.after_line(p).then_some(p),
            Rx::ParameterEnd => self
                .ahead(p, b')')
                .then_some(p)
                .or_else(|| self.group_lit(p, 1, b",")),
            Rx::CurlyInitializerAt0End => self
                .group_lit(p, 1, b",")
                .or_else(|| self.ahead(p, b'}').then_some(p)),
            Rx::TypedefClassAt0At0 => self.anchor_space(p),

            Rx::StringEscapesContextCAt1 => self.backslash_any(p),
            Rx::LineContinuationCharacter => self.line_continuation(p),
            Rx::StringContextAt0At2 => self.octal_escape(p),
            Rx::StringContextAt0At1 => self.simple_escape(p),

            Rx::LambdasAt2At1 => self.non_space_run(p),
            Rx::DiagnosticAt3 => self.one(p, |c| c != '"' && c != '\''),
            Rx::BlockCommentAt0 => self.rest_without_star(p),
            Rx::GccAttributesEnd => self.close_close(p),
            Rx::AssemblyAt2At0 => self.raw_prefix_quote(p),
            Rx::CommentsAt2End => self.doc_end(p),
            Rx::LineComment => self.lead_slashes(p, b"//"),
            Rx::BlockComment => self.lead_slashes(p, b"/*"),
            Rx::ConstructorRootAt2End => self.spaces_then(p, b';'),
            Rx::ConstructorRootAt2 => self.after_close_spaces(p),
            Rx::PreprocessorNumberLiteralCap0At0At0Cap2At0 => self.digit_separator(p),
            Rx::TemplateDefinitionAt0 => self.word_spaces_angle(p),
            Rx::StringContextAt0 => self.string_prefix_quote(p, b'"'),
            Rx::PragmaAt2 => self.pragma_name(p),
            Rx::PredefinedMacrosAt1 => self.predefined_probably(p),
            Rx::CommentsAt0At10 => self.todo_label(p),
            Rx::CommentsAt0 => self.doc_line_lead(p),
            Rx::StringContextAt4 => self.raw_open(p),

            Rx::StringContextAt0At0 => self.ucn(p),
            Rx::VarargEllipses => self.ellipsis(p),
            Rx::InheritanceContextAt3 => self.word_lit(p, b"virtual"),
            Rx::RangeForInner => self.after_for_paren(p).then_some(p),
            Rx::StringContextAt1 => {
                if self.prev(p).is_some_and(|b| b.is_ascii_hexdigit()) {
                    None
                } else {
                    self.string_prefix_quote(p, b'\'')
                }
            }
            Rx::NamespaceBlock => self.word_group(p, b"namespace"),
            Rx::UsingName => self.using_name(p),
            Rx::MacroAt0 => self.macro_parameters(p),
            Rx::PreprocessorConditionalDefined => self.defined_paren(p),
            Rx::StringContextAt0At3 => self.hex_escape(p),
            Rx::CommentsAt0At3 => self.doc_tag(p, &[b"b"]),
            Rx::CommentsAt0At4 => self.doc_tag(p, &[b"c", b"p"]),
            Rx::CommentsAt0At2 => self.doc_tag(p, &[b"a", b"em", b"e"]),
            Rx::GccAttributes => self.gcc_attribute(p),
            Rx::ConstructorInlineEnd => (self.after_close(p)
                || (p < self.lim() && b"];=>[".contains(&self.s[p])))
            .then_some(p),
            Rx::TemplateDefinition => self.template_angle(p),
            Rx::CommentsAt1 => self.doc_block_one(p),
            Rx::StringContextAt3 => self.raw_tagged(p, &[b"glsl", b"GLSL"]),
            Rx::StringContextAt2 => self.raw_tagged(p, &[b"_r", b"re", b"regex"]),
            Rx::InlineComment => self.inline_comment(p),
            Rx::CommentsAt2 => self.doc_block_begin(p),
            Rx::PreprocessorNumberLiteralCap0At0At6 => self.number_tail(p),
            Rx::StaticAssertAt0 => self.static_assert_comma(p),
            Rx::TemplateExplicitInstantiation => self.explicit_instantiation(p),
            Rx::InheritanceContextAt2 => self.word_in(p, &[b"protected", b"private", b"public"]),
            Rx::LanguageConstants => self.word_in(p, &[b"nullptr", b"false", b"NULL", b"true"]),
            Rx::ConstructorBracketCallCap1At1 => {
                self.word_in(p, &[b"struct", b"class", b"union", b"enum"])
            }
            Rx::LambdasAt1 => self.word_in(p, &[b"constexpr", b"consteval", b"mutable"]),
            Rx::Assembly => self.asm_begin(p),
            Rx::PragmaMarkCap3At0 => self.spaced_comment(p),
            Rx::PragmaEnd => self.logical_line_end(p).then_some(p),
            Rx::PreprocessorConditionalDefinedEnd => self
                .lit(p, b")")
                .or_else(|| self.logical_line_end(p).then_some(p)),
            Rx::TemplateIsolatedDefinition => self.template_isolated(p),
            Rx::SquareBrackets => self.square_brackets(p),
            Rx::DiagnosticAt1End => self
                .group_lit(p, 1, b"\"")
                .or_else(|| self.logical_line_end(p).then_some(p)),
            Rx::DiagnosticAt2End => self
                .group_lit(p, 1, b"'")
                .or_else(|| self.logical_line_end(p).then_some(p)),
            Rx::PreprocessorNumberLiteral => self.number_literal(p),
            Rx::TypedefClassAt0End => self.typedef_end(p),
            Rx::OperatorsAt12 => self.assign_operators(p),
            Rx::TypedefClass => self.typedef_before(p, b"class"),
            Rx::TypedefUnion => self.typedef_before(p, b"union"),
            Rx::TypedefStruct => self.typedef_before(p, b"struct"),
            Rx::StringContextAt5 => self.raw_delimited(p),
            Rx::QualifiedTypeCap0At9 => self.ident_ucn(p),
            Rx::StringEscapesContextCAt0 => self.escaped_char(p),

            Rx::AssemblyAt2 => self.gap_paren(p, Paren::Plain),
            Rx::SwitchConditionalParentheses => self.gap_paren(p, Paren::Group),
            Rx::Parameter => self.gap_ahead_word(p),
            Rx::TheThisKeyword => self.gap_word(p, &[b"this"], 3),
            Rx::ExternBlock => self.extern_block(p),
            Rx::CaseStatement => self.gap4_word(p, &[b"case"]),
            Rx::SwitchStatement => self.gap4_word(p, &[b"switch"]),
            Rx::DefaultStatement => self.gap4_word(p, &[b"default"]),
            Rx::QualifiersAndSpecifiersPostParametersAt1Cap1At0 => self.gap4_word(
                p,
                &[b"override", b"volatile", b"noexcept", b"final", b"const"],
            ),
            Rx::AssemblyAt0 => self.gap_line(p),
            Rx::Line => self.directive_head(p, b"line"),
            Rx::Pragma => self.directive_head(p, b"pragma"),
            Rx::PreprocessorConditionalRange => self.conditional_head(p),
            Rx::TypedefClassAt0Cap11At0 => self.final_gap(p),
            Rx::OperatorsAt5 => self.sizeof_pack(p),
            Rx::OperatorsAt0 => self.word_gap_paren(p, b"sizeof"),
            Rx::OperatorsAt1 => self.word_gap_paren(p, b"alignof"),
            Rx::OperatorsAt2 => self.word_gap_paren(p, b"alignas"),
            Rx::OperatorsAt3 => self.word_gap_paren(p, b"typeid"),
            Rx::OperatorsAt4 => self.word_gap_paren(p, b"noexcept"),
            Rx::Decltype => self.word_gap_paren(p, b"decltype"),
            Rx::QualifiersAndSpecifiersPostParametersAt0 => self.word_gap_paren(p, b"requires"),
            Rx::ConstructorInlineAt0At1 => self.equals_default(p),
            Rx::StaticAssert => self.static_assert(p),
            Rx::ExceptionKeywords => self.gap_word(p, &[b"throw", b"catch", b"try"], 3),
            Rx::TypeCastingOperators => self.gap_word(
                p,
                &[
                    b"reinterpret_cast",
                    b"dynamic_cast",
                    b"static_cast",
                    b"const_cast",
                ],
                3,
            ),
            Rx::MiscKeywords => self.gap_word(
                p,
                &[b"constinit", b"typedef", b"concept", b"export", b"module"],
                3,
            ),
            Rx::StorageSpecifiers => self.gap_word(
                p,
                &[
                    b"thread_local",
                    b"volatile",
                    b"register",
                    b"restrict",
                    b"static",
                    b"extern",
                    b"const",
                ],
                3,
            ),
            Rx::ControlFlowKeywords => self.gap_word(
                p,
                &[
                    b"co_return",
                    b"co_yield",
                    b"co_await",
                    b"continue",
                    b"default",
                    b"switch",
                    b"return",
                    b"catch",
                    b"while",
                    b"throw",
                    b"break",
                    b"case",
                    b"goto",
                    b"else",
                    b"for",
                    b"try",
                    b"if",
                    b"do",
                ],
                3,
            ),

            Rx::FunctionalSpecifiersPreParameters => self.word_in(
                p,
                &[
                    b"constexpr",
                    b"consteval",
                    b"explicit",
                    b"mutable",
                    b"virtual",
                    b"inline",
                    b"friend",
                ],
            ),
            Rx::ConstructorRootCap9At0 => self.ident_ahead_colon(p),
            Rx::ConstructorRootCap9At1 => self.after_colon_ident(p),
            Rx::DestructorRootCap13At1 => self.after_colon_destructor(p),
            Rx::CurlyInitializerAt0 => self.dot_ident(p),
            Rx::MacroArgument => self.macro_argument(p),
            Rx::CommentsAt0At7Cap3At0 => self.word_ident(p),
            Rx::CppAttributesAt5 => self.ident_ahead_scope(p),
            Rx::ConstructorInlineAt0At3At1 => self.ident_brace(p),
            Rx::WordlikeOperatorsAt0 => self.word_in(
                p,
                &[
                    b"noexcept",
                    b"xor_eq",
                    b"and_eq",
                    b"delete",
                    b"not_eq",
                    b"bitand",
                    b"bitor",
                    b"compl",
                    b"or_eq",
                    b"not",
                    b"xor",
                    b"new",
                    b"and",
                    b"or",
                ],
            ),
            Rx::CppAttributesAt2 => self.using_alias(p),
            Rx::MacroAt0Cap2At0 => self.macro_parameter(p),
            Rx::ParameterAt14 => self.parameter_name(p),
            Rx::StringContextAt0End => self.literal_end(p, b'"'),
            Rx::StringContextAt1End => self.literal_end(p, b'\''),
            Rx::LambdasCap2At1 => self.lambda_capture(p),
            Rx::TemplateArgumentDefaulted => self.template_default(p),
            Rx::AssemblyAt2At2 => self.bracketed_name(p),
            Rx::ParameterAt11 => self.parameter_name_gap(p, Self::ahead_parameter_end),
            Rx::ParameterOrMaybeValueAt13 => {
                self.parameter_name_gap(p, Self::ahead_parameter_value_end)
            }
            Rx::Undef => self.undef(p),
            Rx::Macro => self.macro_define(p),
            Rx::GotoStatement => self.goto_statement(p),
            Rx::Label => self.label(p),
            Rx::TypedefClassAt0Cap11At1 => self.class_name_final(p),
            Rx::FunctionDefinitionCap7At0 => self.specifier_gap(p),
            Rx::PragmaMark => self.pragma_mark(p),
            Rx::AccessControlKeywords => self.access_label(p),
            Rx::Diagnostic => self.diagnostic(p),
            Rx::MemoryOperators => self.memory_operator(p),
            Rx::PreprocessorConditionalStandalone => self.conditional_standalone(p),
            Rx::TemplateCallRangeHelperAt0 => self.number_angle(p, b"<<"),
            Rx::TemplateCallRangeHelperAt1 => self.number_angle(p, b"<"),
            Rx::QualifiersAndSpecifiersPostParametersAt1 => self.trailing_qualifiers(p),
            Rx::ParameterAt16 => self.reference_chain(p),
            Rx::SimpleArrayAssignmentCap13At1 => self.ampersand_run(p),
            Rx::UnknownVariable => self.unknown_variable(p),
            Rx::TemplateCallInnards => self.template_call_innards(p),
            Rx::StringEscapesContextCAt2 => self.placeholder(p),
            Rx::PreprocessorNumberLiteralCap0At0At0 => {
                self.number_shape(p, 0, NumberIn::Preprocessor)
            }
            Rx::NumberLiteralCap0At0At0 => self.number_shape(p, 0, NumberIn::Code),
            Rx::PreprocessorNumberLiteralCap0At0At1 => {
                self.number_shape(p, 1, NumberIn::Preprocessor)
            }
            Rx::NumberLiteralCap0At0At1 => self.number_shape(p, 1, NumberIn::Code),
            Rx::PreprocessorNumberLiteralCap0At0At2 => {
                self.number_shape(p, 2, NumberIn::Preprocessor)
            }
            Rx::NumberLiteralCap0At0At2 => self.number_shape(p, 2, NumberIn::Code),
            Rx::PreprocessorNumberLiteralCap0At0At3 => {
                self.number_shape(p, 3, NumberIn::Preprocessor)
            }
            Rx::NumberLiteralCap0At0At3 => self.number_shape(p, 3, NumberIn::Code),
            Rx::PreprocessorNumberLiteralCap0At0At4 => {
                self.number_shape(p, 4, NumberIn::Preprocessor)
            }
            Rx::NumberLiteralCap0At0At4 => self.number_shape(p, 4, NumberIn::Code),
            Rx::PreprocessorNumberLiteralCap0At0At5 => {
                self.number_shape(p, 5, NumberIn::Preprocessor)
            }
            Rx::NumberLiteralCap0At0At5 => self.number_shape(p, 5, NumberIn::Code),
            Rx::CommentsAt0At1 => self.doc_plain(p),
            Rx::CommentsAt0At5 => self.doc_word(p),
            Rx::CommentsAt0At6 => self.doc_decl(p),
            Rx::CommentsAt0At8 => self.doc_text(p),
            Rx::CommentsAt0At9 => self.doc_block(p),
            Rx::CommentsAt0At7 => self.doc_param(p),
            Rx::MethodAccessCap13At1 => self.object_operator(p, OperatorAt::AfterObject),
            Rx::MethodAccessCap13At0 => self.object_operator(p, OperatorAt::Before),
            Rx::MethodAccess => self.method_access(p),
            Rx::MemberAccess => self.member_access(p),
            Rx::TypedefClassAt0 => self.typedef_tag(p, b"class"),
            Rx::TypedefUnionAt0 => self.typedef_tag(p, b"union"),
            Rx::TypedefStructAt0 => self.typedef_tag(p, b"struct"),
            Rx::StandardDeclaresAt0 => self.standard_declare(p, b"struct"),
            Rx::StandardDeclaresAt1 => self.standard_declare(p, b"union"),
            Rx::StandardDeclaresAt2 => self.standard_declare(p, b"enum"),
            Rx::StandardDeclaresAt3 => self.standard_declare(p, b"class"),
            Rx::ConstructorInline => self.constructor_inline(p),
            Rx::DestructorInline => self.destructor_inline(p),
            Rx::FunctionPointerParameterEnd => self.function_pointer_end(p),
            Rx::Include => self.include(p),
            Rx::TemplateDefinitionArgument => self.template_parameter(p),
            Rx::OverQualifiedTypesAt0 => self.over_qualified(p, b"struct"),
            Rx::OverQualifiedTypesAt1 => self.over_qualified(p, b"enum"),
            Rx::OverQualifiedTypesAt2 => self.over_qualified(p, b"union"),
            Rx::OverQualifiedTypesAt3 => self.over_qualified(p, b"class"),
            Rx::InlineBuiltinStorageType => self.inline_builtin_type(p),
            Rx::BuiltinStorageTypeInitializer => self.builtin_type_call(p),
            Rx::TypedefStructAt0At2At0 => self.typedef_declarator(p),
            Rx::ConstructorInlineAt0At3At0 => self.call_with_template(p),
            Rx::Lambdas => self.lambda_introducer(p),
            Rx::EnumeratorList => self.enumerator(p),
            Rx::EmacsFileBanner => self.emacs_banner(p),
            Rx::QualifiedType => self.qualified_type(p),
            Rx::Typename => self.typename_type(p),
            Rx::InheritanceContextAt4 => self.base_class(p),
            Rx::CurlyInitializer => self.type_brace(p),
            Rx::SimpleConstructorCall => self.simple_constructor_call(p),
            Rx::ConstructorBracketCall => self.constructor_bracket_call(p),
            Rx::UsingNamespace => self.using_namespace(p),
            Rx::NamespaceBlockAt0At4 => self.namespace_name(p),
            Rx::NamespaceAlias => self.namespace_alias(p),
            Rx::ConstructorRoot => self.constructor_root(p),
            Rx::DestructorRoot => self.destructor_root(p),
            Rx::FunctionCallAt0 => self.function_call_head(p, Callee::UpperCase),
            Rx::FunctionCallAt1 => self.function_call_head(p, Callee::Any),
            Rx::ScopeResolutionInnerGenerated => self.scope_resolution_inner(p),
            Rx::ParameterAt5 => self.parameter_storage(p),
            Rx::FunctionPointerParameter => self.function_pointer_parameter(p),
            Rx::StringContextAt5End => self.raw_close(p),
            Rx::NormalVariableDeclaration => self.normal_variable(p, VariableForm::Declaration),
            Rx::NormalVariableAssignment => self.normal_variable(p, VariableForm::Assignment),
            Rx::VariableAssignment => self.variable_assignment(p),
            Rx::SimpleArrayAssignment => self.simple_array_assignment(p),
            Rx::RangeForInnerAt0 => self.range_for_variable(p),
            Rx::RangeForInnerAt1 => self.range_for_binding(p),
            Rx::TypeAlias => self.type_alias(p),
            Rx::FunctionDefinitionAt0At2 => self.trailing_return(p),
            Rx::OperatorOverload => self.operator_overload(p),
            Rx::FunctionDefinition => self.function_definition(p),
            Rx::EnumBlock => self.enum_block(p),
            Rx::PredefinedMacrosAt0 => self.predefined_macro(p),
            _ => None,
        }
    }
}
