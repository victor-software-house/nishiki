//! The patterns of the grammar, by name: what each one does when it matches,
//! and what decides where it matches.

use super::access::List;
use super::engine::{C, Lexer};
use super::lists::P;
use super::preproc::{Look, Meta};
use super::scope::S;
use super::state::Rule;
use super::{comments, wordset};
use crate::rules::Act;
use crate::scope::scopes;

impl P {
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
            P::End => Act::End,
            P::DocLine => Act::Begin(Rule::DocLine),
            P::DocBlockBegin => Act::Begin(Rule::DocBlock),
            P::BlockComment => Act::Begin(Rule::BlockComment),
            P::LineLeading => Act::Begin(Rule::LineLeading),
            P::LineSlash => Act::Begin(Rule::LineSlash),
            P::LineComment => Act::Begin(Rule::LineComment),
            P::DoubleQuote => Act::Begin(Rule::DoubleQuote),
            P::SingleQuote => Act::Begin(Rule::SingleQuote),
            P::Ternary => Act::Begin(Rule::Ternary),
            P::AsmBegin => Act::Begin(Rule::Asm),
            P::BracketBegin => Act::Begin(Rule::Bracket),
            P::BlockBegin => Act::Begin(Rule::Block),
            P::BlockInnerBegin => Act::Begin(Rule::BlockInner),
            P::ParensBegin => Act::Begin(Rule::Parens),
            P::ParensBlockBegin => Act::Begin(Rule::ParensBlock),
            P::FunctionDefinitionBegin => Act::Begin(Rule::FunctionDefinition),
            P::FunctionParametersBegin => Act::Begin(Rule::FunctionParameters),
            P::FunctionParensBegin => Act::Begin(Rule::FunctionParens),
            P::CallBegin => Act::Begin(Rule::CallArguments),
            P::CallParensBegin => Act::Begin(Rule::CallParens),
            P::CFunctionCall => Act::Begin(Rule::FunctionCall),
            P::MethodAccess => Act::Begin(Rule::Method),
            P::InitializationBegin => Act::Begin(Rule::Initialization),
            P::DefineBegin => Act::Begin(Rule::Define),
            P::DiagnosticBegin => Act::Begin(Rule::Diagnostic),
            P::IncludeBegin => Act::Begin(Rule::Include),
            P::LineDirectiveBegin => Act::Begin(Rule::LineDirective),
            P::UndefBegin => Act::Begin(Rule::Undef),
            P::PragmaDirectiveBegin => Act::Begin(Rule::PragmaDirective),
            P::DefineBlockBegin => Act::Begin(Rule::DefineBlock),
            P::DefineBlockInnerBegin => Act::Begin(Rule::DefineBlockInner),
            P::DefineFunctionBegin => Act::Begin(Rule::DefineFunction),
            P::DefineCallBegin => Act::Begin(Rule::DefineCall),
            P::DefineParensBegin => Act::Begin(Rule::DefineParens),
            P::DefineDoubleQuoteBegin => Act::Begin(Rule::DefineDoubleQuote),
            P::DefineSingleQuoteBegin => Act::Begin(Rule::DefineSingleQuote),
            P::DiagnosticDoubleQuoteBegin => Act::Begin(Rule::DiagnosticDoubleQuote),
            P::DiagnosticSingleQuoteBegin => Act::Begin(Rule::DiagnosticSingleQuote),
            P::DiagnosticTextBegin => Act::Begin(Rule::DiagnosticText),
            P::IncludeDoubleQuoteBegin => Act::Begin(Rule::IncludeDoubleQuote),
            P::IncludeAngleBegin => Act::Begin(Rule::IncludeAngle),
            P::ConditionalIfBegin => Act::Begin(Rule::ConditionalIf),
            P::ConditionalIfBlockBegin => Act::Begin(Rule::ConditionalIfBlock),
            P::EnabledIfBegin => Act::Begin(Rule::EnabledIf),
            P::EnabledIfBlockBegin => Act::Begin(Rule::EnabledIfBlock),
            P::DisabledIfBegin => Act::Begin(Rule::DisabledIf),
            P::DisabledIfBlockBegin => Act::Begin(Rule::DisabledIfBlock),
            P::HeadContinuationBegin => Act::Begin(Rule::HeadContinuation),
            P::HeadPlainBegin => Act::Begin(Rule::HeadPlain),
            P::ConditionalElifBegin => Act::Begin(Rule::ConditionalElif),
            P::LiveElifBegin => Act::Begin(Rule::LiveElif),
            P::LiveElifBlockBegin => Act::Begin(Rule::LiveElifBlock),
            P::LiveElseBegin => Act::Begin(Rule::LiveElse),
            P::LiveElseBlockBegin => Act::Begin(Rule::LiveElseBlock),
            P::DisabledElifBegin => Act::Begin(Rule::DisabledElif),
            P::DisabledElifOtherBegin => Act::Begin(Rule::DisabledElifOther),
            P::DisabledElifOtherBlockBegin => Act::Begin(Rule::DisabledElifOtherBlock),
            P::ElseBranchBegin => Act::Begin(Rule::ElseBranch),
            P::ElseBranchBlockBegin => Act::Begin(Rule::ElseBranchBlock),
            P::ElifBranchBegin => Act::Begin(Rule::ElifBranch),
            P::ElifBranchBlockBegin => Act::Begin(Rule::ElifBranchBlock),
            P::EnabledBodyBegin => Act::Begin(Rule::EnabledBody),
            P::EnabledBodyBlockBegin => Act::Begin(Rule::EnabledBodyBlock),
            P::DisabledBodyBegin => Act::Begin(Rule::DisabledBody),
            P::DisabledBodyBlockBegin => Act::Begin(Rule::DisabledBodyBlock),
            P::DisabledElifBodyBegin => Act::Begin(Rule::DisabledElifBody),
            P::ElifTailBegin => Act::Begin(Rule::ElifTail),
            P::ElifTailBlockBegin => Act::Begin(Rule::ElifTailBlock),
            P::ElifElseBegin => Act::Begin(Rule::ElifElse),
            P::ElifElseBlockBegin => Act::Begin(Rule::ElifElseBlock),
            P::ElifElifBegin => Act::Begin(Rule::ElifElif),
            P::NestBegin => Act::Begin(Rule::Nest),
            P::ConditionalTernaryBegin => Act::Begin(Rule::ConditionalTernary),
            P::ConditionalParensBegin => Act::Begin(Rule::ConditionalParens),
            P::SwitchBegin => Act::Begin(Rule::Switch),
            P::SwitchHeadBegin => Act::Begin(Rule::SwitchHead),
            P::SwitchBodyBegin => Act::Begin(Rule::SwitchBody),
            P::SwitchTailBegin => Act::Begin(Rule::SwitchTail),
            P::SwitchParensBegin => Act::Begin(Rule::SwitchParens),
            P::CaseBegin => Act::Begin(Rule::Case),
            P::DefaultBegin => Act::Begin(Rule::Default),
            P::AsmParensBegin => Act::Begin(Rule::AsmParens),
            P::AsmStringBegin => Act::Begin(Rule::AsmString),
            P::AsmInnerBegin => Act::Begin(Rule::AsmInner),
            P::DocBlockOne => Act::Match(scopes![S::CommentBlockDoc]),
            P::BannerBlock => Act::Match(scopes![S::CommentBlockBanner]),
            P::BannerLine => Act::Match(scopes![S::CommentLineBanner]),
            P::DoxygenBare
            | P::DoxygenWordA
            | P::DoxygenWordB
            | P::DoxygenWordC
            | P::DoxygenWordD => Act::Match(scopes![S::StorageDoxygen]),
            P::Gtkdoc => Act::Match(scopes![S::StorageGtkdoc]),
            P::InlineEnd => Act::Match(scopes![S::CommentBlock, S::PunctCommentEnd]),
            P::InlineStar => Act::Match(scopes![S::CommentBlock]),
            P::EscChar | P::EscBackslash => Act::Match(scopes![S::EscapeChar]),
            P::EscInvalid => Act::Match(scopes![S::InvalidEscape]),
            P::Placeholder => Act::Match(scopes![S::Placeholder]),
            P::NumberSeparator => Act::Match(scopes![S::PunctSeparatorNumeric]),
            P::Sizeof => Act::Match(scopes![S::OpSizeof]),
            P::Decrement => Act::Match(scopes![S::OpDecrement]),
            P::Increment => Act::Match(scopes![S::OpIncrement]),
            P::CompoundAssign => Act::Match(scopes![S::OpAssignCompound]),
            P::BitwiseAssign => Act::Match(scopes![S::OpAssignBitwise]),
            P::Shift => Act::Match(scopes![S::OpShift]),
            P::Compare => Act::Match(scopes![S::OpCompare]),
            P::Logical => Act::Match(scopes![S::OpLogical]),
            P::BitOp | P::Arith => Act::Match(scopes![S::Operator]),
            P::Assign => Act::Match(scopes![S::OpAssign]),
            P::KeywordControl => Act::Match(scopes![S::KeywordControl]),
            P::Primitive => Act::Match(scopes![S::StoragePrimitive]),
            P::BuiltIn => Act::Match(scopes![S::StorageBuiltin]),
            P::Typedef => Act::Match(scopes![S::KeywordTypedef]),
            P::Modifier => Act::Match(scopes![S::StorageModifier]),
            P::MacK => Act::Match(scopes![S::ConstantMacClassic]),
            P::MacG => Act::Match(scopes![S::VariableMacGlobal]),
            P::MacS => Act::Match(scopes![S::VariableMacStatic]),
            P::ConstLang | P::ConditionalConstant => Act::Match(scopes![S::ConstantLanguage]),
            P::SysTypes => Act::Match(scopes![S::SupportSysTypes]),
            P::Pthread => Act::Match(scopes![S::SupportPthread]),
            P::Stdint => Act::Match(scopes![S::SupportStdint]),
            P::MacConst => Act::Match(scopes![S::SupportMacConstant]),
            P::MacType => Act::Match(scopes![S::SupportMacType]),
            P::PosixReserved => Act::Match(scopes![S::SupportPosix]),
            P::Semi => Act::Match(scopes![S::PunctTerminator]),
            P::Comma => Act::Match(scopes![S::PunctDelimiter]),
            P::BracketEmpty => Act::Match(scopes![S::StorageArray]),
            P::Vararg => Act::Match(scopes![S::PunctVararg]),
            P::RangeColon => Act::Match(scopes![S::PunctRangeBased]),
            P::DefineParameterComma => Act::Match(scopes![S::PunctParameters]),
            P::DefineParameterEllipsis => Act::Match(scopes![S::Ellipses, S::PunctVarargParam]),
            P::PragmaMark => Act::Match(scopes![S::MetaSection]),
            P::DefineLParen => Act::Match(scopes![S::PunctParensBegin]),
            P::DefineRParen => Act::Match(scopes![S::PunctParensEnd]),
            P::UndefName | P::ConditionalNamed => Act::Match(scopes![S::EntityFunctionPre]),
            P::PragmaName => Act::Match(scopes![S::EntityAttrPragma]),
            P::ConditionalDefinedKeyword => Act::Match(scopes![S::DirectiveConditional]),
            P::ConditionalDefinedInvalid => Act::Match(scopes![S::InvalidMacroName]),
            P::AsmColon => Act::Match(scopes![S::PunctColonAsm]),
            P::InvalidNumber => Act::Match(scopes![S::InvalidNumeric]),
            P::DoxygenItalic
            | P::DoxygenBold
            | P::DoxygenRaw
            | P::DoxygenParameter
            | P::InlineComment
            | P::PlaceholderBad
            | P::LineContinuation
            | P::Number
            | P::EnumStructUnion
            | P::Predefined
            | P::PredefinedProbably
            | P::Probably
            | P::MemberAccess
            | P::PlainAccess
            | P::DefineParameterName
            | P::Stray
            | P::AsmLineEnd
            | P::AsmLabel
            | P::ParameterDirections
            | P::HexFloat
            | P::DecFloat
            | P::Binary
            | P::Octal
            | P::HexInt
            | P::DecInt => Act::Match(S::NO_SCOPES),
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
    pub(super) fn match_pattern(&mut self, pattern: P, p: usize) -> bool {
        match pattern {
            P::DocLine => self.doc_line(p),
            P::DocBlockOne => self.doc_block_one(p),
            P::DocBlockBegin => self.doc_block_begin(p),
            P::BannerBlock => self.banner_block(p),
            P::BlockComment => self.block_comment(p),
            P::BannerLine => self.banner_line(p),
            P::LineLeading => self.line_leading(p),
            P::LineSlash => self.line_slash(p),
            P::LineComment => self.line_comment(p),
            P::DoxygenBare => self.doxy_plain(p, comments::DOXY_BARE),
            P::DoxygenItalic => self.doxy_arg(p, &["a", "em", "e"], scopes![S::MarkupItalic]),
            P::DoxygenBold => self.doxy_arg(p, &["b"], scopes![S::MarkupBold]),
            P::DoxygenRaw => self.doxy_arg(p, &["c", "p"], scopes![S::MarkupRaw]),
            P::DoxygenWordA => self.doxy_plain(p, comments::DOXY_A),
            P::DoxygenWordB => self.doxy_plain(p, comments::DOXY_B),
            P::DoxygenParameter => self.doxy_param(p),
            P::DoxygenWordC => self.doxy_plain(p, comments::DOXY_C),
            P::DoxygenWordD => self.doxy_plain(p, comments::DOXY_D),
            P::Gtkdoc => self.gtkdoc(p),
            P::InlineComment => self.inline_comment(p),
            P::InlineEnd => self.inline_end(p),
            P::InlineStar => self.byte_rule(p, b'*'),
            P::DoubleQuote
            | P::DefineDoubleQuoteBegin
            | P::DiagnosticDoubleQuoteBegin
            | P::IncludeDoubleQuoteBegin => self.quote(p, b'"'),
            P::SingleQuote | P::DefineSingleQuoteBegin | P::DiagnosticSingleQuoteBegin => {
                self.quote(p, b'\'')
            }
            P::EscChar => self.esc_char(p),
            P::EscBackslash => self.esc_backslash(p),
            P::EscInvalid => self.esc_invalid(p),
            P::Placeholder => self.placeholder(p),
            P::PlaceholderBad => self.placeholder_bad(p),
            P::LineContinuation => self.line_cont(p),
            P::Number => self.number_match(p),
            P::NumberSeparator => self.digit_sep(p) && self.plain(p, p + 1),
            P::Sizeof => self.sizeof(p),
            P::Decrement => self.op_dec(p),
            P::Increment => self.op_inc(p),
            P::CompoundAssign => self.op_compound(p),
            P::BitwiseAssign => self.op_bitwise_assign(p),
            P::Shift => self.op_shift(p),
            P::Compare => self.op_compare(p),
            P::Logical => self.op_logical(p),
            P::BitOp => self.op_bit(p),
            P::Assign => self.op_assign(p),
            P::Arith => self.op_arith(p),
            P::Ternary | P::ConditionalTernaryBegin => self.ternary_begin(p),
            P::KeywordControl => self.word_rule(p, wordset::CONTROL),
            P::Primitive => self.word_rule(p, wordset::PRIMITIVE),
            P::BuiltIn => self.word_rule(p, wordset::BUILTIN),
            P::EnumStructUnion => self.enum_struct_union(p),
            P::AsmBegin => self.asm_begin(p),
            P::Typedef => self.typedef(p),
            P::Modifier => self.word_rule(p, wordset::MODIFIER),
            P::MacK => self.mac_classic(p, b'k'),
            P::MacG => self.mac_classic(p, b'g'),
            P::MacS => self.mac_classic(p, b's'),
            P::ConstLang | P::ConditionalConstant => self.word_rule(p, wordset::CONSTANT),
            P::SysTypes => self.word_rule(p, wordset::SYS_TYPES),
            P::Pthread => self.word_rule(p, wordset::PTHREAD),
            P::Stdint => self.word_rule(p, wordset::STDINT),
            P::MacConst => self.word_rule(p, wordset::MAC_CONSTANT),
            P::MacType => self.word_rule(p, wordset::MAC_TYPE),
            P::PosixReserved => self.posix_reserved(p),
            P::Predefined => self.predefined(p),
            P::PredefinedProbably => self.predefined_probably(p),
            P::Semi => self.byte_rule(p, b';'),
            P::Comma | P::DefineParameterComma => self.byte_rule(p, b','),
            P::BracketBegin => self.bracket_begin(p),
            P::BracketEmpty => self.bracket_empty(p),
            P::Vararg => self.vararg(p),
            P::BlockBegin | P::BlockInnerBegin | P::DefineBlockBegin | P::DefineBlockInnerBegin => {
                self.brace(p)
            }
            P::ParensBegin
            | P::ParensBlockBegin
            | P::FunctionParensBegin
            | P::CallParensBegin
            | P::DefineParensBegin
            | P::ConditionalParensBegin => self.paren(p),
            P::RangeColon => self.range_colon(p),
            P::FunctionDefinitionBegin => self.func_def(p),
            P::FunctionParametersBegin => self.call_like(p, List::Parameters),
            P::Probably => self.probably(p),
            P::CallBegin | P::DefineCallBegin => self.call_like(p, List::Arguments),
            P::CFunctionCall => self.function_call(p),
            P::MethodAccess => self.method_access(p),
            P::MemberAccess => self.member_access(p),
            P::PlainAccess => self.plain_access(p),
            P::InitializationBegin => self.init_begin(p),
            P::DefineBegin => self.define_begin(p),
            P::DefineParameterName => self.define_param(p),
            P::DefineParameterEllipsis => self.starts(p, b"...") && self.plain(p, p + 3),
            P::DiagnosticBegin => self.named_directive(
                p,
                &["error", "warning"],
                "keyword.control.directive.diagnostic.",
            ),
            P::IncludeBegin => self.named_directive(
                p,
                &["include_next", "include", "import"],
                "keyword.control.directive.",
            ),
            P::PragmaMark => self.pragma_mark(p),
            P::LineDirectiveBegin => self.line_directive(p, "line", scopes![S::DirectiveLine]),
            P::UndefBegin => self.line_directive(p, "undef", scopes![S::DirectiveUndef]),
            P::PragmaDirectiveBegin => {
                self.line_directive(p, "pragma", scopes![S::DirectivePragma])
            }
            P::DefineLParen => self.byte_rule(p, b'('),
            P::DefineRParen => self.byte_rule(p, b')'),
            P::DefineFunctionBegin => self.define_func(p),
            P::DiagnosticTextBegin => self.any_but_quote(p),
            P::IncludeAngleBegin => self.angle_begin(p),
            P::UndefName | P::ConditionalNamed => self.macro_name(p, b'$'),
            P::PragmaName => self.macro_name(p, b'-'),
            P::ConditionalIfBegin | P::ConditionalIfBlockBegin => self.cond_if(p),
            P::EnabledIfBegin | P::EnabledIfBlockBegin => self.cond_if_look(p, Look::Enabled),
            P::DisabledIfBegin | P::DisabledIfBlockBegin => self.cond_if_look(p, Look::Disabled),
            P::Stray => self.stray(p),
            P::HeadContinuationBegin | P::HeadPlainBegin => self.head_begin(p),
            P::ConditionalElifBegin => self.cond_begin(p, &["elif"], Look::No, Meta::Omitted),
            P::LiveElifBegin | P::LiveElifBlockBegin => {
                self.cond_begin(p, &["elif"], Look::Enabled, Meta::Whole)
            }
            P::DisabledElifBegin => self.cond_begin(p, &["elif"], Look::Disabled, Meta::Whole),
            P::DisabledElifOtherBegin
            | P::DisabledElifOtherBlockBegin
            | P::ElifBranchBegin
            | P::ElifBranchBlockBegin
            | P::ElifElifBegin => self.cond_begin(p, &["elif"], Look::No, Meta::Whole),
            P::LiveElseBegin
            | P::LiveElseBlockBegin
            | P::ElseBranchBegin
            | P::ElseBranchBlockBegin
            | P::ElifElseBegin
            | P::ElifElseBlockBegin => self.cond_begin(p, &["else"], Look::No, Meta::Whole),
            P::EnabledBodyBegin
            | P::EnabledBodyBlockBegin
            | P::DisabledBodyBegin
            | P::DisabledBodyBlockBegin
            | P::DisabledElifBodyBegin
            | P::ElifTailBegin
            | P::ElifTailBlockBegin => self.newline_begin(p),
            P::NestBegin => self.nest_begin(p),
            P::ConditionalDefinedKeyword => self.cond_defined(p),
            P::ConditionalDefinedInvalid => self.cond_defined_invalid(p),
            P::SwitchBegin => self.switch_begin(p),
            P::SwitchHeadBegin => self.switch_head(p),
            P::SwitchBodyBegin => self.switch_body(p),
            P::SwitchTailBegin => self.switch_tail(p),
            P::SwitchParensBegin => self.switch_parens(p),
            P::CaseBegin => self.label_begin(p, "case", scopes![S::KeywordCase]),
            P::DefaultBegin => self.label_begin(p, "default", scopes![S::KeywordDefault]),
            P::AsmLineEnd => self.asm_line_end(p),
            P::AsmParensBegin => self.asm_parens(p),
            P::AsmStringBegin => self.asm_string(p),
            P::AsmInnerBegin => self.asm_inner(p),
            P::AsmLabel => self.asm_label(p),
            P::AsmColon => self.byte_rule(p, b':'),
            P::ParameterDirections => self.param_dir(p),
            P::End => self.try_end(self.top_rule(), p),
            P::HexFloat => self.number_pattern(p, Self::hex_float),
            P::DecFloat => self.number_pattern(p, Self::dec_float),
            P::Binary => self.number_pattern(p, Self::binary),
            P::Octal => self.number_pattern(p, Self::octal),
            P::HexInt => self.number_pattern(p, Self::hex_int),
            P::DecInt => self.number_pattern(p, Self::dec_int),
            P::InvalidNumber => self.invalid_number(p),
        }
    }
}
