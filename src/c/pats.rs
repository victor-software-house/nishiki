//! The patterns of the grammar, by name: what each one does when it matches,
//! and what decides where it matches.

use super::access::CallKind;
use super::engine::{C, Lexer};
use super::lists::P;
use super::preproc::{Look, Meta};
use super::scope::S;
use super::state::R;
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
            P::DocLine => Act::Begin(R::DocLine),
            P::DocBlockBegin => Act::Begin(R::DocBlock),
            P::BlockComment => Act::Begin(R::BlockComment),
            P::LineLeading => Act::Begin(R::LineLeading),
            P::LineSlash => Act::Begin(R::LineSlash),
            P::LineComment => Act::Begin(R::LineComment),
            P::DQuote => Act::Begin(R::DQuote),
            P::SQuote => Act::Begin(R::SQuote),
            P::Ternary => Act::Begin(R::Ternary),
            P::AsmBegin => Act::Begin(R::Asm),
            P::BracketBegin => Act::Begin(R::Bracket),
            P::BlockBegin => Act::Begin(R::Block),
            P::BlockInnerBegin => Act::Begin(R::BlockInner),
            P::ParensBegin => Act::Begin(R::Parens),
            P::ParensBlockBegin => Act::Begin(R::ParensBlock),
            P::FuncDefBegin => Act::Begin(R::FuncDef),
            P::FuncParamsBegin => Act::Begin(R::FuncParams),
            P::FuncParensBegin => Act::Begin(R::FuncParens),
            P::CallBegin => Act::Begin(R::CallArgs),
            P::CallParensBegin => Act::Begin(R::CallParens),
            P::CFunctionCall => Act::Begin(R::FuncCall),
            P::MethodAccess => Act::Begin(R::Method),
            P::InitBegin => Act::Begin(R::Init),
            P::DefineBegin => Act::Begin(R::Define),
            P::DiagBegin => Act::Begin(R::Diag),
            P::IncludeBegin => Act::Begin(R::Include),
            P::LineDirBegin => Act::Begin(R::LineDir),
            P::UndefBegin => Act::Begin(R::Undef),
            P::PragmaDirBegin => Act::Begin(R::PragmaDir),
            P::DefineBlockBegin => Act::Begin(R::DefineBlock),
            P::DefineBlockInnerBegin => Act::Begin(R::DefineBlockInner),
            P::DefineFuncBegin => Act::Begin(R::DefineFunc),
            P::DefineCallBegin => Act::Begin(R::DefineCall),
            P::DefineParensBegin => Act::Begin(R::DefineParens),
            P::DefineDQuoteBegin => Act::Begin(R::DefineDQuote),
            P::DefineSQuoteBegin => Act::Begin(R::DefineSQuote),
            P::DiagDQuoteBegin => Act::Begin(R::DiagDQuote),
            P::DiagSQuoteBegin => Act::Begin(R::DiagSQuote),
            P::DiagTextBegin => Act::Begin(R::DiagText),
            P::IncludeDQBegin => Act::Begin(R::IncludeDQ),
            P::IncludeLtBegin => Act::Begin(R::IncludeLt),
            P::CondIfBegin => Act::Begin(R::CondIf),
            P::CondIfBBegin => Act::Begin(R::CondIfB),
            P::EnabledIfBegin => Act::Begin(R::EnabledIf),
            P::EnabledIfBBegin => Act::Begin(R::EnabledIfB),
            P::DisabledIfBegin => Act::Begin(R::DisabledIf),
            P::DisabledIfBBegin => Act::Begin(R::DisabledIfB),
            P::HeadContBegin => Act::Begin(R::HeadCont),
            P::HeadPlainBegin => Act::Begin(R::HeadPlain),
            P::CondElifBegin => Act::Begin(R::CondElif),
            P::LiveElifBegin => Act::Begin(R::LiveElif),
            P::LiveElifBBegin => Act::Begin(R::LiveElifB),
            P::LiveElseBegin => Act::Begin(R::LiveElse),
            P::LiveElseBBegin => Act::Begin(R::LiveElseB),
            P::DisabledElifBegin => Act::Begin(R::DisabledElif),
            P::DisabledElifOtherBegin => Act::Begin(R::DisabledElifOther),
            P::DisabledElifOtherBBegin => Act::Begin(R::DisabledElifOtherB),
            P::ElseBranchBegin => Act::Begin(R::ElseBranch),
            P::ElseBranchBBegin => Act::Begin(R::ElseBranchB),
            P::ElifBranchBegin => Act::Begin(R::ElifBranch),
            P::ElifBranchBBegin => Act::Begin(R::ElifBranchB),
            P::EnabledBodyBegin => Act::Begin(R::EnabledBody),
            P::EnabledBodyBBegin => Act::Begin(R::EnabledBodyB),
            P::DisabledBodyBegin => Act::Begin(R::DisabledBody),
            P::DisabledBodyBBegin => Act::Begin(R::DisabledBodyB),
            P::DisabledElifBodyBegin => Act::Begin(R::DisabledElifBody),
            P::ElifTailBegin => Act::Begin(R::ElifTail),
            P::ElifTailBBegin => Act::Begin(R::ElifTailB),
            P::ElifElseBegin => Act::Begin(R::ElifElse),
            P::ElifElseBBegin => Act::Begin(R::ElifElseB),
            P::ElifElifBegin => Act::Begin(R::ElifElif),
            P::NestBegin => Act::Begin(R::Nest),
            P::CondTernaryBegin => Act::Begin(R::CondTernary),
            P::CondParensBegin => Act::Begin(R::CondParens),
            P::SwitchBegin => Act::Begin(R::Switch),
            P::SwitchHeadBegin => Act::Begin(R::SwitchHead),
            P::SwitchBodyBegin => Act::Begin(R::SwitchBody),
            P::SwitchTailBegin => Act::Begin(R::SwitchTail),
            P::SwitchParensBegin => Act::Begin(R::SwitchParens),
            P::CaseBegin => Act::Begin(R::Case),
            P::DefaultBegin => Act::Begin(R::Default),
            P::AsmParensBegin => Act::Begin(R::AsmParens),
            P::AsmStringBegin => Act::Begin(R::AsmString),
            P::AsmInnerBegin => Act::Begin(R::AsmInner),
            P::DocBlockOne => Act::Match(scopes![S::CommentBlockDoc]),
            P::BannerBlock => Act::Match(scopes![S::CommentBlockBanner]),
            P::BannerLine => Act::Match(scopes![S::CommentLineBanner]),
            P::DoxyBare | P::DoxyWordA | P::DoxyWordB | P::DoxyWordC | P::DoxyWordD => {
                Act::Match(scopes![S::StorageDoxygen])
            }
            P::Gtkdoc => Act::Match(scopes![S::StorageGtkdoc]),
            P::InlineEnd => Act::Match(scopes![S::CommentBlock, S::PunctCommentEnd]),
            P::InlineStar => Act::Match(scopes![S::CommentBlock]),
            P::EscChar | P::EscBackslash => Act::Match(scopes![S::EscapeChar]),
            P::EscInvalid => Act::Match(scopes![S::InvalidEscape]),
            P::Placeholder => Act::Match(scopes![S::Placeholder]),
            P::NumSep => Act::Match(scopes![S::PunctSeparatorNumeric]),
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
            P::ConstLang | P::CondConstant => Act::Match(scopes![S::ConstantLanguage]),
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
            P::DefineParamComma => Act::Match(scopes![S::PunctParameters]),
            P::DefineParamEllipsis => Act::Match(scopes![S::Ellipses, S::PunctVarargParam]),
            P::PragmaMark => Act::Match(scopes![S::MetaSection]),
            P::DefineLParen => Act::Match(scopes![S::PunctParensBegin]),
            P::DefineRParen => Act::Match(scopes![S::PunctParensEnd]),
            P::UndefName | P::CondNamed => Act::Match(scopes![S::EntityFunctionPre]),
            P::PragmaName => Act::Match(scopes![S::EntityAttrPragma]),
            P::CondDefinedKeyword => Act::Match(scopes![S::DirectiveConditional]),
            P::CondDefinedInvalid => Act::Match(scopes![S::InvalidMacroName]),
            P::AsmColon => Act::Match(scopes![S::PunctColonAsm]),
            P::InvalidNumber => Act::Match(scopes![S::InvalidNumeric]),
            P::DoxyItalic
            | P::DoxyBold
            | P::DoxyRaw
            | P::DoxyParam
            | P::InlineComment
            | P::PlaceholderBad
            | P::LineCont
            | P::Number
            | P::EnumStructUnion
            | P::Predefined
            | P::PredefinedProbably
            | P::Probably
            | P::MemberAccess
            | P::PlainAccess
            | P::DefineParamName
            | P::Stray
            | P::AsmLineEnd
            | P::AsmLabel
            | P::ParamDir
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
    /// Whether `pat` matches at `p`.
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
    pub(super) fn match_pat(&mut self, pat: P, p: usize) -> bool {
        match pat {
            P::DocLine => self.doc_line(p),
            P::DocBlockOne => self.doc_block_one(p),
            P::DocBlockBegin => self.doc_block_begin(p),
            P::BannerBlock => self.banner_block(p),
            P::BlockComment => self.block_comment(p),
            P::BannerLine => self.banner_line(p),
            P::LineLeading => self.line_leading(p),
            P::LineSlash => self.line_slash(p),
            P::LineComment => self.line_comment(p),
            P::DoxyBare => self.doxy_plain(p, comments::DOXY_BARE),
            P::DoxyItalic => self.doxy_arg(p, &["a", "em", "e"], scopes![S::MarkupItalic]),
            P::DoxyBold => self.doxy_arg(p, &["b"], scopes![S::MarkupBold]),
            P::DoxyRaw => self.doxy_arg(p, &["c", "p"], scopes![S::MarkupRaw]),
            P::DoxyWordA => self.doxy_plain(p, comments::DOXY_A),
            P::DoxyWordB => self.doxy_plain(p, comments::DOXY_B),
            P::DoxyParam => self.doxy_param(p),
            P::DoxyWordC => self.doxy_plain(p, comments::DOXY_C),
            P::DoxyWordD => self.doxy_plain(p, comments::DOXY_D),
            P::Gtkdoc => self.gtkdoc(p),
            P::InlineComment => self.inline_comment(p),
            P::InlineEnd => self.inline_end(p),
            P::InlineStar => self.byte_rule(p, b'*'),
            P::DQuote | P::DefineDQuoteBegin | P::DiagDQuoteBegin | P::IncludeDQBegin => {
                self.quote(p, b'"')
            }
            P::SQuote | P::DefineSQuoteBegin | P::DiagSQuoteBegin => self.quote(p, b'\''),
            P::EscChar => self.esc_char(p),
            P::EscBackslash => self.esc_backslash(p),
            P::EscInvalid => self.esc_invalid(p),
            P::Placeholder => self.placeholder(p),
            P::PlaceholderBad => self.placeholder_bad(p),
            P::LineCont => self.line_cont(p),
            P::Number => self.number_match(p),
            P::NumSep => self.digit_sep(p) && self.plain(p, p + 1),
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
            P::Ternary | P::CondTernaryBegin => self.ternary_begin(p),
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
            P::ConstLang | P::CondConstant => self.word_rule(p, wordset::CONSTANT),
            P::SysTypes => self.word_rule(p, wordset::SYS_TYPES),
            P::Pthread => self.word_rule(p, wordset::PTHREAD),
            P::Stdint => self.word_rule(p, wordset::STDINT),
            P::MacConst => self.word_rule(p, wordset::MAC_CONSTANT),
            P::MacType => self.word_rule(p, wordset::MAC_TYPE),
            P::PosixReserved => self.posix_reserved(p),
            P::Predefined => self.predefined(p),
            P::PredefinedProbably => self.predefined_probably(p),
            P::Semi => self.byte_rule(p, b';'),
            P::Comma | P::DefineParamComma => self.byte_rule(p, b','),
            P::BracketBegin => self.bracket_begin(p),
            P::BracketEmpty => self.bracket_empty(p),
            P::Vararg => self.vararg(p),
            P::BlockBegin | P::BlockInnerBegin | P::DefineBlockBegin | P::DefineBlockInnerBegin => {
                self.brace(p)
            }
            P::ParensBegin
            | P::ParensBlockBegin
            | P::FuncParensBegin
            | P::CallParensBegin
            | P::DefineParensBegin
            | P::CondParensBegin => self.paren(p),
            P::RangeColon => self.range_colon(p),
            P::FuncDefBegin => self.func_def(p),
            P::FuncParamsBegin => self.call_like(p, CallKind::Params),
            P::Probably => self.probably(p),
            P::CallBegin | P::DefineCallBegin => self.call_like(p, CallKind::Args),
            P::CFunctionCall => self.function_call(p),
            P::MethodAccess => self.method_access(p),
            P::MemberAccess => self.member_access(p),
            P::PlainAccess => self.plain_access(p),
            P::InitBegin => self.init_begin(p),
            P::DefineBegin => self.define_begin(p),
            P::DefineParamName => self.define_param(p),
            P::DefineParamEllipsis => self.starts(p, b"...") && self.plain(p, p + 3),
            P::DiagBegin => self.named_directive(
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
            P::LineDirBegin => self.line_directive(p, "line", scopes![S::DirectiveLine]),
            P::UndefBegin => self.line_directive(p, "undef", scopes![S::DirectiveUndef]),
            P::PragmaDirBegin => self.line_directive(p, "pragma", scopes![S::DirectivePragma]),
            P::DefineLParen => self.byte_rule(p, b'('),
            P::DefineRParen => self.byte_rule(p, b')'),
            P::DefineFuncBegin => self.define_func(p),
            P::DiagTextBegin => self.any_but_quote(p),
            P::IncludeLtBegin => self.angle_begin(p),
            P::UndefName | P::CondNamed => self.macro_name(p, b'$'),
            P::PragmaName => self.macro_name(p, b'-'),
            P::CondIfBegin | P::CondIfBBegin => self.cond_if(p),
            P::EnabledIfBegin | P::EnabledIfBBegin => self.cond_if_look(p, Look::Enabled),
            P::DisabledIfBegin | P::DisabledIfBBegin => self.cond_if_look(p, Look::Disabled),
            P::Stray => self.stray(p),
            P::HeadContBegin | P::HeadPlainBegin => self.head_begin(p),
            P::CondElifBegin => self.cond_begin(p, &["elif"], Look::No, Meta::Omitted),
            P::LiveElifBegin | P::LiveElifBBegin => {
                self.cond_begin(p, &["elif"], Look::Enabled, Meta::Whole)
            }
            P::DisabledElifBegin => self.cond_begin(p, &["elif"], Look::Disabled, Meta::Whole),
            P::DisabledElifOtherBegin
            | P::DisabledElifOtherBBegin
            | P::ElifBranchBegin
            | P::ElifBranchBBegin
            | P::ElifElifBegin => self.cond_begin(p, &["elif"], Look::No, Meta::Whole),
            P::LiveElseBegin
            | P::LiveElseBBegin
            | P::ElseBranchBegin
            | P::ElseBranchBBegin
            | P::ElifElseBegin
            | P::ElifElseBBegin => self.cond_begin(p, &["else"], Look::No, Meta::Whole),
            P::EnabledBodyBegin
            | P::EnabledBodyBBegin
            | P::DisabledBodyBegin
            | P::DisabledBodyBBegin
            | P::DisabledElifBodyBegin
            | P::ElifTailBegin
            | P::ElifTailBBegin => self.newline_begin(p),
            P::NestBegin => self.nest_begin(p),
            P::CondDefinedKeyword => self.cond_defined(p),
            P::CondDefinedInvalid => self.cond_defined_invalid(p),
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
            P::ParamDir => self.param_dir(p),
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
