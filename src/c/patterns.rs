//! The patterns of the grammar, by name: what each one does when it matches,
//! and what decides where it matches.

use super::access::List;
use super::engine::{C, Lexer};
use super::lists::Pattern;
use super::preproc::{Look, Meta};
use super::scope::S;
use super::state::Rule;
use super::{comments, wordset};
use crate::rules::Act;
use crate::scope::scopes;

impl Pattern {
    /// What the pattern does when it matches: the scopes it names its text
    /// with, the rule it opens, or the end of the innermost rule.
    /// The matchers record only where a match lies and the scopes of its
    /// parts.
    #[expect(
        clippy::too_many_lines,
        reason = "one arm per pattern: the table of what each pattern does"
    )]
    pub(super) fn act(self) -> Act<C> {
        match self {
            Pattern::End => Act::End,
            Pattern::DocLine => Act::Begin(Rule::DocLine),
            Pattern::DocBlockBegin => Act::Begin(Rule::DocBlock),
            Pattern::BlockComment => Act::Begin(Rule::BlockComment),
            Pattern::LineLeading => Act::Begin(Rule::LineLeading),
            Pattern::LineSlash => Act::Begin(Rule::LineSlash),
            Pattern::LineComment => Act::Begin(Rule::LineComment),
            Pattern::DoubleQuote => Act::Begin(Rule::DoubleQuote),
            Pattern::SingleQuote => Act::Begin(Rule::SingleQuote),
            Pattern::Ternary => Act::Begin(Rule::Ternary),
            Pattern::AsmBegin => Act::Begin(Rule::Asm),
            Pattern::BracketBegin => Act::Begin(Rule::Bracket),
            Pattern::BlockBegin => Act::Begin(Rule::Block),
            Pattern::BlockInnerBegin => Act::Begin(Rule::BlockInner),
            Pattern::ParensBegin => Act::Begin(Rule::Parens),
            Pattern::ParensBlockBegin => Act::Begin(Rule::ParensBlock),
            Pattern::FunctionDefinitionBegin => Act::Begin(Rule::FunctionDefinition),
            Pattern::FunctionParametersBegin => Act::Begin(Rule::FunctionParameters),
            Pattern::FunctionParensBegin => Act::Begin(Rule::FunctionParens),
            Pattern::CallBegin => Act::Begin(Rule::CallArguments),
            Pattern::CallParensBegin => Act::Begin(Rule::CallParens),
            Pattern::CFunctionCall => Act::Begin(Rule::FunctionCall),
            Pattern::MethodAccess => Act::Begin(Rule::Method),
            Pattern::InitializationBegin => Act::Begin(Rule::Initialization),
            Pattern::DefineBegin => Act::Begin(Rule::Define),
            Pattern::DiagnosticBegin => Act::Begin(Rule::Diagnostic),
            Pattern::IncludeBegin => Act::Begin(Rule::Include),
            Pattern::LineDirectiveBegin => Act::Begin(Rule::LineDirective),
            Pattern::UndefBegin => Act::Begin(Rule::Undef),
            Pattern::PragmaDirectiveBegin => Act::Begin(Rule::PragmaDirective),
            Pattern::DefineBlockBegin => Act::Begin(Rule::DefineBlock),
            Pattern::DefineBlockInnerBegin => Act::Begin(Rule::DefineBlockInner),
            Pattern::DefineFunctionBegin => Act::Begin(Rule::DefineFunction),
            Pattern::DefineCallBegin => Act::Begin(Rule::DefineCall),
            Pattern::DefineParensBegin => Act::Begin(Rule::DefineParens),
            Pattern::DefineDoubleQuoteBegin => Act::Begin(Rule::DefineDoubleQuote),
            Pattern::DefineSingleQuoteBegin => Act::Begin(Rule::DefineSingleQuote),
            Pattern::DiagnosticDoubleQuoteBegin => Act::Begin(Rule::DiagnosticDoubleQuote),
            Pattern::DiagnosticSingleQuoteBegin => Act::Begin(Rule::DiagnosticSingleQuote),
            Pattern::DiagnosticTextBegin => Act::Begin(Rule::DiagnosticText),
            Pattern::IncludeDoubleQuoteBegin => Act::Begin(Rule::IncludeDoubleQuote),
            Pattern::IncludeAngleBegin => Act::Begin(Rule::IncludeAngle),
            Pattern::ConditionalIfBegin => Act::Begin(Rule::ConditionalIf),
            Pattern::ConditionalIfBlockBegin => Act::Begin(Rule::ConditionalIfBlock),
            Pattern::EnabledIfBegin => Act::Begin(Rule::EnabledIf),
            Pattern::EnabledIfBlockBegin => Act::Begin(Rule::EnabledIfBlock),
            Pattern::DisabledIfBegin => Act::Begin(Rule::DisabledIf),
            Pattern::DisabledIfBlockBegin => Act::Begin(Rule::DisabledIfBlock),
            Pattern::HeadContinuationBegin => Act::Begin(Rule::HeadContinuation),
            Pattern::HeadPlainBegin => Act::Begin(Rule::HeadPlain),
            Pattern::ConditionalElifBegin => Act::Begin(Rule::ConditionalElif),
            Pattern::LiveElifBegin => Act::Begin(Rule::LiveElif),
            Pattern::LiveElifBlockBegin => Act::Begin(Rule::LiveElifBlock),
            Pattern::LiveElseBegin => Act::Begin(Rule::LiveElse),
            Pattern::LiveElseBlockBegin => Act::Begin(Rule::LiveElseBlock),
            Pattern::DisabledElifBegin => Act::Begin(Rule::DisabledElif),
            Pattern::DisabledElifOtherBegin => Act::Begin(Rule::DisabledElifOther),
            Pattern::DisabledElifOtherBlockBegin => Act::Begin(Rule::DisabledElifOtherBlock),
            Pattern::ElseBranchBegin => Act::Begin(Rule::ElseBranch),
            Pattern::ElseBranchBlockBegin => Act::Begin(Rule::ElseBranchBlock),
            Pattern::ElifBranchBegin => Act::Begin(Rule::ElifBranch),
            Pattern::ElifBranchBlockBegin => Act::Begin(Rule::ElifBranchBlock),
            Pattern::EnabledBodyBegin => Act::Begin(Rule::EnabledBody),
            Pattern::EnabledBodyBlockBegin => Act::Begin(Rule::EnabledBodyBlock),
            Pattern::DisabledBodyBegin => Act::Begin(Rule::DisabledBody),
            Pattern::DisabledBodyBlockBegin => Act::Begin(Rule::DisabledBodyBlock),
            Pattern::DisabledElifBodyBegin => Act::Begin(Rule::DisabledElifBody),
            Pattern::ElifTailBegin => Act::Begin(Rule::ElifTail),
            Pattern::ElifTailBlockBegin => Act::Begin(Rule::ElifTailBlock),
            Pattern::ElifElseBegin => Act::Begin(Rule::ElifElse),
            Pattern::ElifElseBlockBegin => Act::Begin(Rule::ElifElseBlock),
            Pattern::ElifElifBegin => Act::Begin(Rule::ElifElif),
            Pattern::NestBegin => Act::Begin(Rule::Nest),
            Pattern::ConditionalTernaryBegin => Act::Begin(Rule::ConditionalTernary),
            Pattern::ConditionalParensBegin => Act::Begin(Rule::ConditionalParens),
            Pattern::SwitchBegin => Act::Begin(Rule::Switch),
            Pattern::SwitchHeadBegin => Act::Begin(Rule::SwitchHead),
            Pattern::SwitchBodyBegin => Act::Begin(Rule::SwitchBody),
            Pattern::SwitchTailBegin => Act::Begin(Rule::SwitchTail),
            Pattern::SwitchParensBegin => Act::Begin(Rule::SwitchParens),
            Pattern::CaseBegin => Act::Begin(Rule::Case),
            Pattern::DefaultBegin => Act::Begin(Rule::Default),
            Pattern::AsmParensBegin => Act::Begin(Rule::AsmParens),
            Pattern::AsmStringBegin => Act::Begin(Rule::AsmString),
            Pattern::AsmInnerBegin => Act::Begin(Rule::AsmInner),
            Pattern::DocBlockOne => Act::Match(scopes![S::CommentBlockDoc]),
            Pattern::BannerBlock => Act::Match(scopes![S::CommentBlockBanner]),
            Pattern::BannerLine => Act::Match(scopes![S::CommentLineBanner]),
            Pattern::DoxygenBare
            | Pattern::DoxygenWordA
            | Pattern::DoxygenWordB
            | Pattern::DoxygenWordC
            | Pattern::DoxygenWordD => Act::Match(scopes![S::StorageDoxygen]),
            Pattern::Gtkdoc => Act::Match(scopes![S::StorageGtkdoc]),
            Pattern::InlineEnd => Act::Match(scopes![S::CommentBlock, S::PunctCommentEnd]),
            Pattern::InlineStar => Act::Match(scopes![S::CommentBlock]),
            Pattern::EscChar | Pattern::EscBackslash => Act::Match(scopes![S::EscapeChar]),
            Pattern::EscInvalid => Act::Match(scopes![S::InvalidEscape]),
            Pattern::Placeholder => Act::Match(scopes![S::Placeholder]),
            Pattern::NumberSeparator => Act::Match(scopes![S::PunctSeparatorNumeric]),
            Pattern::Sizeof => Act::Match(scopes![S::OpSizeof]),
            Pattern::Decrement => Act::Match(scopes![S::OpDecrement]),
            Pattern::Increment => Act::Match(scopes![S::OpIncrement]),
            Pattern::CompoundAssign => Act::Match(scopes![S::OpAssignCompound]),
            Pattern::BitwiseAssign => Act::Match(scopes![S::OpAssignBitwise]),
            Pattern::Shift => Act::Match(scopes![S::OpShift]),
            Pattern::Compare => Act::Match(scopes![S::OpCompare]),
            Pattern::Logical => Act::Match(scopes![S::OpLogical]),
            Pattern::BitOperator | Pattern::Arith => Act::Match(scopes![S::Operator]),
            Pattern::Assign => Act::Match(scopes![S::OpAssign]),
            Pattern::KeywordControl => Act::Match(scopes![S::KeywordControl]),
            Pattern::Primitive => Act::Match(scopes![S::StoragePrimitive]),
            Pattern::BuiltIn => Act::Match(scopes![S::StorageBuiltin]),
            Pattern::Typedef => Act::Match(scopes![S::KeywordTypedef]),
            Pattern::Modifier => Act::Match(scopes![S::StorageModifier]),
            Pattern::MacK => Act::Match(scopes![S::ConstantMacClassic]),
            Pattern::MacG => Act::Match(scopes![S::VariableMacGlobal]),
            Pattern::MacS => Act::Match(scopes![S::VariableMacStatic]),
            Pattern::ConstLang | Pattern::ConditionalConstant => {
                Act::Match(scopes![S::ConstantLanguage])
            }
            Pattern::SysTypes => Act::Match(scopes![S::SupportSysTypes]),
            Pattern::Pthread => Act::Match(scopes![S::SupportPthread]),
            Pattern::Stdint => Act::Match(scopes![S::SupportStdint]),
            Pattern::MacConst => Act::Match(scopes![S::SupportMacConstant]),
            Pattern::MacType => Act::Match(scopes![S::SupportMacType]),
            Pattern::PosixReserved => Act::Match(scopes![S::SupportPosix]),
            Pattern::Semi => Act::Match(scopes![S::PunctTerminator]),
            Pattern::Comma => Act::Match(scopes![S::PunctDelimiter]),
            Pattern::BracketEmpty => Act::Match(scopes![S::StorageArray]),
            Pattern::Vararg => Act::Match(scopes![S::PunctVararg]),
            Pattern::RangeColon => Act::Match(scopes![S::PunctRangeBased]),
            Pattern::DefineParameterComma => Act::Match(scopes![S::PunctParameters]),
            Pattern::DefineParameterEllipsis => {
                Act::Match(scopes![S::Ellipses, S::PunctVarargParam])
            }
            Pattern::PragmaMark => Act::Match(scopes![S::MetaSection]),
            Pattern::DefineLeftParen => Act::Match(scopes![S::PunctParensBegin]),
            Pattern::DefineRightParen => Act::Match(scopes![S::PunctParensEnd]),
            Pattern::UndefName | Pattern::ConditionalNamed => {
                Act::Match(scopes![S::EntityFunctionPre])
            }
            Pattern::PragmaName => Act::Match(scopes![S::EntityAttrPragma]),
            Pattern::ConditionalDefinedKeyword => Act::Match(scopes![S::DirectiveConditional]),
            Pattern::ConditionalDefinedInvalid => Act::Match(scopes![S::InvalidMacroName]),
            Pattern::AsmColon => Act::Match(scopes![S::PunctColonAsm]),
            Pattern::InvalidNumber => Act::Match(scopes![S::InvalidNumeric]),
            Pattern::DoxygenItalic
            | Pattern::DoxygenBold
            | Pattern::DoxygenRaw
            | Pattern::DoxygenParameter
            | Pattern::InlineComment
            | Pattern::PlaceholderBad
            | Pattern::LineContinuation
            | Pattern::Number
            | Pattern::EnumStructUnion
            | Pattern::Predefined
            | Pattern::PredefinedProbably
            | Pattern::Probably
            | Pattern::MemberAccess
            | Pattern::PlainAccess
            | Pattern::DefineParameterName
            | Pattern::Stray
            | Pattern::AsmLineEnd
            | Pattern::AsmLabel
            | Pattern::ParameterDirections
            | Pattern::HexFloat
            | Pattern::DecFloat
            | Pattern::Binary
            | Pattern::Octal
            | Pattern::HexInt
            | Pattern::DecInt => Act::Match(S::NO_SCOPES),
        }
    }
}

impl Lexer<'_> {
    /// Whether `pattern` matches at `p`.
    /// The match's start and end are in `x`, and its parts are in the spans.
    ///
    /// It is `#[inline]` because `find`, in another module, calls it for every
    /// candidate pattern.
    /// At the default sixteen codegen units, the two land in different units.
    /// A generic method that is not `#[inline]` cannot be inlined across them
    /// (`bench/AGENTS.md` rule 13).
    #[inline]
    #[expect(
        clippy::too_many_lines,
        reason = "one arm per pattern: the table of what each pattern matches"
    )]
    pub(super) fn match_pattern(&mut self, pattern: Pattern, p: usize) -> bool {
        match pattern {
            Pattern::DocLine => self.doc_line(p),
            Pattern::DocBlockOne => self.doc_block_one(p),
            Pattern::DocBlockBegin => self.doc_block_begin(p),
            Pattern::BannerBlock => self.banner_block(p),
            Pattern::BlockComment => self.block_comment(p),
            Pattern::BannerLine => self.banner_line(p),
            Pattern::LineLeading => self.line_leading(p),
            Pattern::LineSlash => self.line_slash(p),
            Pattern::LineComment => self.line_comment(p),
            Pattern::DoxygenBare => self.doxygen_plain(p, comments::DOXYGEN_BARE),
            Pattern::DoxygenItalic => {
                self.doxygen_argument(p, &["a", "em", "e"], scopes![S::MarkupItalic])
            }
            Pattern::DoxygenBold => self.doxygen_argument(p, &["b"], scopes![S::MarkupBold]),
            Pattern::DoxygenRaw => self.doxygen_argument(p, &["c", "p"], scopes![S::MarkupRaw]),
            Pattern::DoxygenWordA => self.doxygen_plain(p, comments::DOXYGEN_A),
            Pattern::DoxygenWordB => self.doxygen_plain(p, comments::DOXYGEN_B),
            Pattern::DoxygenParameter => self.doxygen_parameter(p),
            Pattern::DoxygenWordC => self.doxygen_plain(p, comments::DOXYGEN_C),
            Pattern::DoxygenWordD => self.doxygen_plain(p, comments::DOXYGEN_D),
            Pattern::Gtkdoc => self.gtkdoc(p),
            Pattern::InlineComment => self.inline_comment(p),
            Pattern::InlineEnd => self.inline_end(p),
            Pattern::InlineStar => self.byte_rule(p, b'*'),
            Pattern::DoubleQuote
            | Pattern::DefineDoubleQuoteBegin
            | Pattern::DiagnosticDoubleQuoteBegin
            | Pattern::IncludeDoubleQuoteBegin => self.quote(p, b'"'),
            Pattern::SingleQuote
            | Pattern::DefineSingleQuoteBegin
            | Pattern::DiagnosticSingleQuoteBegin => self.quote(p, b'\''),
            Pattern::EscChar => self.esc_char(p),
            Pattern::EscBackslash => self.esc_backslash(p),
            Pattern::EscInvalid => self.esc_invalid(p),
            Pattern::Placeholder => self.placeholder(p),
            Pattern::PlaceholderBad => self.placeholder_bad(p),
            Pattern::LineContinuation => self.line_cont(p),
            Pattern::Number => self.number_match(p),
            Pattern::NumberSeparator => self.digit_separator(p) && self.plain(p, p + 1),
            Pattern::Sizeof => self.sizeof(p),
            Pattern::Decrement => self.operator_decrement(p),
            Pattern::Increment => self.operator_increment(p),
            Pattern::CompoundAssign => self.operator_compound(p),
            Pattern::BitwiseAssign => self.operator_bitwise_assignment(p),
            Pattern::Shift => self.operator_shift(p),
            Pattern::Compare => self.operator_comparison(p),
            Pattern::Logical => self.operator_logical(p),
            Pattern::BitOperator => self.operator_bitwise(p),
            Pattern::Assign => self.operator_assignment(p),
            Pattern::Arith => self.operator_arithmetic(p),
            Pattern::Ternary | Pattern::ConditionalTernaryBegin => self.ternary_begin(p),
            Pattern::KeywordControl => self.word_rule(p, wordset::CONTROL),
            Pattern::Primitive => self.word_rule(p, wordset::PRIMITIVE),
            Pattern::BuiltIn => self.word_rule(p, wordset::BUILTIN),
            Pattern::EnumStructUnion => self.enum_struct_union(p),
            Pattern::AsmBegin => self.asm_begin(p),
            Pattern::Typedef => self.typedef(p),
            Pattern::Modifier => self.word_rule(p, wordset::MODIFIER),
            Pattern::MacK => self.mac_classic(p, b'k'),
            Pattern::MacG => self.mac_classic(p, b'g'),
            Pattern::MacS => self.mac_classic(p, b's'),
            Pattern::ConstLang | Pattern::ConditionalConstant => {
                self.word_rule(p, wordset::CONSTANT)
            }
            Pattern::SysTypes => self.word_rule(p, wordset::SYS_TYPES),
            Pattern::Pthread => self.word_rule(p, wordset::PTHREAD),
            Pattern::Stdint => self.word_rule(p, wordset::STDINT),
            Pattern::MacConst => self.word_rule(p, wordset::MAC_CONSTANT),
            Pattern::MacType => self.word_rule(p, wordset::MAC_TYPE),
            Pattern::PosixReserved => self.posix_reserved(p),
            Pattern::Predefined => self.predefined(p),
            Pattern::PredefinedProbably => self.predefined_probably(p),
            Pattern::Semi => self.byte_rule(p, b';'),
            Pattern::Comma | Pattern::DefineParameterComma => self.byte_rule(p, b','),
            Pattern::BracketBegin => self.bracket_begin(p),
            Pattern::BracketEmpty => self.bracket_empty(p),
            Pattern::Vararg => self.vararg(p),
            Pattern::BlockBegin
            | Pattern::BlockInnerBegin
            | Pattern::DefineBlockBegin
            | Pattern::DefineBlockInnerBegin => self.brace(p),
            Pattern::ParensBegin
            | Pattern::ParensBlockBegin
            | Pattern::FunctionParensBegin
            | Pattern::CallParensBegin
            | Pattern::DefineParensBegin
            | Pattern::ConditionalParensBegin => self.paren(p),
            Pattern::RangeColon => self.range_colon(p),
            Pattern::FunctionDefinitionBegin => self.function_definition(p),
            Pattern::FunctionParametersBegin => self.call_like(p, List::Parameters),
            Pattern::Probably => self.probably(p),
            Pattern::CallBegin | Pattern::DefineCallBegin => self.call_like(p, List::Arguments),
            Pattern::CFunctionCall => self.function_call(p),
            Pattern::MethodAccess => self.method_access(p),
            Pattern::MemberAccess => self.member_access(p),
            Pattern::PlainAccess => self.plain_access(p),
            Pattern::InitializationBegin => self.initialization_begin(p),
            Pattern::DefineBegin => self.define_begin(p),
            Pattern::DefineParameterName => self.define_parameter(p),
            Pattern::DefineParameterEllipsis => self.starts(p, b"...") && self.plain(p, p + 3),
            Pattern::DiagnosticBegin => self.named_directive(
                p,
                &["error", "warning"],
                "keyword.control.directive.diagnostic.",
            ),
            Pattern::IncludeBegin => self.named_directive(
                p,
                &["include_next", "include", "import"],
                "keyword.control.directive.",
            ),
            Pattern::PragmaMark => self.pragma_mark(p),
            Pattern::LineDirectiveBegin => {
                self.line_directive(p, "line", scopes![S::DirectiveLine])
            }
            Pattern::UndefBegin => self.line_directive(p, "undef", scopes![S::DirectiveUndef]),
            Pattern::PragmaDirectiveBegin => {
                self.line_directive(p, "pragma", scopes![S::DirectivePragma])
            }
            Pattern::DefineLeftParen => self.byte_rule(p, b'('),
            Pattern::DefineRightParen => self.byte_rule(p, b')'),
            Pattern::DefineFunctionBegin => self.define_function(p),
            Pattern::DiagnosticTextBegin => self.any_but_quote(p),
            Pattern::IncludeAngleBegin => self.angle_begin(p),
            Pattern::UndefName | Pattern::ConditionalNamed => self.macro_name(p, b'$'),
            Pattern::PragmaName => self.macro_name(p, b'-'),
            Pattern::ConditionalIfBegin | Pattern::ConditionalIfBlockBegin => {
                self.conditional_if(p)
            }
            Pattern::EnabledIfBegin | Pattern::EnabledIfBlockBegin => {
                self.conditional_if_look(p, Look::Enabled)
            }
            Pattern::DisabledIfBegin | Pattern::DisabledIfBlockBegin => {
                self.conditional_if_look(p, Look::Disabled)
            }
            Pattern::Stray => self.stray(p),
            Pattern::HeadContinuationBegin | Pattern::HeadPlainBegin => self.head_begin(p),
            Pattern::ConditionalElifBegin => {
                self.conditional_begin(p, &["elif"], Look::No, Meta::Omitted)
            }
            Pattern::LiveElifBegin | Pattern::LiveElifBlockBegin => {
                self.conditional_begin(p, &["elif"], Look::Enabled, Meta::Whole)
            }
            Pattern::DisabledElifBegin => {
                self.conditional_begin(p, &["elif"], Look::Disabled, Meta::Whole)
            }
            Pattern::DisabledElifOtherBegin
            | Pattern::DisabledElifOtherBlockBegin
            | Pattern::ElifBranchBegin
            | Pattern::ElifBranchBlockBegin
            | Pattern::ElifElifBegin => self.conditional_begin(p, &["elif"], Look::No, Meta::Whole),
            Pattern::LiveElseBegin
            | Pattern::LiveElseBlockBegin
            | Pattern::ElseBranchBegin
            | Pattern::ElseBranchBlockBegin
            | Pattern::ElifElseBegin
            | Pattern::ElifElseBlockBegin => {
                self.conditional_begin(p, &["else"], Look::No, Meta::Whole)
            }
            Pattern::EnabledBodyBegin
            | Pattern::EnabledBodyBlockBegin
            | Pattern::DisabledBodyBegin
            | Pattern::DisabledBodyBlockBegin
            | Pattern::DisabledElifBodyBegin
            | Pattern::ElifTailBegin
            | Pattern::ElifTailBlockBegin => self.newline_begin(p),
            Pattern::NestBegin => self.nest_begin(p),
            Pattern::ConditionalDefinedKeyword => self.conditional_defined(p),
            Pattern::ConditionalDefinedInvalid => self.conditional_defined_invalid(p),
            Pattern::SwitchBegin => self.switch_begin(p),
            Pattern::SwitchHeadBegin => self.switch_head(p),
            Pattern::SwitchBodyBegin => self.switch_body(p),
            Pattern::SwitchTailBegin => self.switch_tail(p),
            Pattern::SwitchParensBegin => self.switch_parens(p),
            Pattern::CaseBegin => self.label_begin(p, "case", scopes![S::KeywordCase]),
            Pattern::DefaultBegin => self.label_begin(p, "default", scopes![S::KeywordDefault]),
            Pattern::AsmLineEnd => self.asm_line_end(p),
            Pattern::AsmParensBegin => self.asm_parens(p),
            Pattern::AsmStringBegin => self.asm_string(p),
            Pattern::AsmInnerBegin => self.asm_inner(p),
            Pattern::AsmLabel => self.asm_label(p),
            Pattern::AsmColon => self.byte_rule(p, b':'),
            Pattern::ParameterDirections => self.parameter_direction(p),
            Pattern::End => self.try_end(self.top_rule(), p),
            Pattern::HexFloat => self.number_pattern(p, Self::hex_float),
            Pattern::DecFloat => self.number_pattern(p, Self::dec_float),
            Pattern::Binary => self.number_pattern(p, Self::binary),
            Pattern::Octal => self.number_pattern(p, Self::octal),
            Pattern::HexInt => self.number_pattern(p, Self::hex_int),
            Pattern::DecInt => self.number_pattern(p, Self::dec_int),
            Pattern::InvalidNumber => self.invalid_number(p),
        }
    }
}
