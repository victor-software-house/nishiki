//! The patterns of the grammar, by name: what each one matches at a byte.

use super::access::CallKind;
use super::engine::Lexer;
use super::lists::P;
use super::preproc::Look;
use super::scope::S;
use super::state::R;
use super::wordset;

impl Lexer<'_> {
    /// Whether `pat` matches at `p`; the match is in `hit` and `sp`.
    #[expect(
        clippy::too_many_lines,
        reason = "one arm per pattern: the table of what each pattern matches"
    )]
    pub(super) fn try_pat(&mut self, pat: P, p: usize) -> bool {
        self.sp.truncate(self.span_base());
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
            P::DoxyBare => self.doxy_plain(p, 0),
            P::DoxyItalic => self.doxy_arg(p, &["a", "em", "e"], &[S::MarkupItalic]),
            P::DoxyBold => self.doxy_arg(p, &["b"], &[S::MarkupBold]),
            P::DoxyRaw => self.doxy_arg(p, &["c", "p"], &[S::MarkupRaw]),
            P::DoxyWordA => self.doxy_plain(p, 1),
            P::DoxyWordB => self.doxy_plain(p, 2),
            P::DoxyParam => self.doxy_param(p),
            P::DoxyWordC => self.doxy_plain(p, 3),
            P::DoxyWordD => self.doxy_plain(p, 4),
            P::Gtkdoc => self.gtkdoc(p),
            P::InlineComment => self.inline_comment(p),
            P::InlineEnd => self.inline_end(p),
            P::InlineStar => self.inline_star(p),
            P::DQuote => self.quote(p, b'"', R::DQuote),
            P::SQuote => self.quote(p, b'\'', R::SQuote),
            P::EscChar => self.esc_char(p),
            P::EscBackslash => self.esc_backslash(p),
            P::EscInvalid => self.esc_invalid(p),
            P::Placeholder => self.placeholder(p),
            P::PlaceholderBad => self.placeholder_bad(p),
            P::LineCont => self.line_cont(p),
            P::Number => self.number_match(p),
            P::NumSep => self.digit_sep(p) && self.plain(p, p + 1, &[S::PunctSeparatorNumeric]),
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
            P::Ternary => self.ternary_begin(p, R::Ternary),
            P::KeywordControl => self.word_rule(p, wordset::CONTROL, &[S::KeywordControl]),
            P::Primitive => self.word_rule(p, wordset::PRIMITIVE, &[S::StoragePrimitive]),
            P::BuiltIn => self.word_rule(p, wordset::BUILTIN, &[S::StorageBuiltin]),
            P::EnumStructUnion => self.enum_struct_union(p),
            P::AsmBegin => self.asm_begin(p),
            P::Typedef => self.typedef(p),
            P::Modifier => self.word_rule(p, wordset::MODIFIER, &[S::StorageModifier]),
            P::MacK => self.mac_classic(p, b'k', &[S::ConstantMacClassic]),
            P::MacG => self.mac_classic(p, b'g', &[S::VariableMacGlobal]),
            P::MacS => self.mac_classic(p, b's', &[S::VariableMacStatic]),
            P::ConstLang | P::CondConstant => {
                self.word_rule(p, wordset::CONSTANT, &[S::ConstantLanguage])
            }
            P::SysTypes => self.word_rule(p, wordset::SYS_TYPES, &[S::SupportSysTypes]),
            P::Pthread => self.word_rule(p, wordset::PTHREAD, &[S::SupportPthread]),
            P::Stdint => self.word_rule(p, wordset::STDINT, &[S::SupportStdint]),
            P::MacConst => self.word_rule(p, wordset::MAC_CONSTANT, &[S::SupportMacConstant]),
            P::MacType => self.word_rule(p, wordset::MAC_TYPE, &[S::SupportMacType]),
            P::PosixReserved => self.posix_reserved(p),
            P::Predefined => self.predefined(p),
            P::PredefinedProbably => self.predefined_probably(p),
            P::Semi => self.byte_rule(p, b';', &[S::PunctTerminator]),
            P::Comma => self.byte_rule(p, b',', &[S::PunctDelimiter]),
            P::BracketBegin => self.bracket_begin(p),
            P::BracketEmpty => self.bracket_empty(p),
            P::Vararg => self.vararg(p),
            P::BlockBegin => self.brace(p, R::Block),
            P::BlockInnerBegin => self.brace(p, R::BlockInner),
            P::ParensBegin => self.paren(p, R::Parens),
            P::ParensBlockBegin => self.paren(p, R::ParensBlock),
            P::RangeColon => self.range_colon(p),
            P::FuncDefBegin => self.func_def(p),
            P::FuncParamsBegin => self.call_like(p, R::FuncParams, CallKind::Params),
            P::FuncParensBegin => self.paren(p, R::FuncParens),
            P::Probably => self.probably(p),
            P::CallBegin => self.call_like(p, R::CallArgs, CallKind::Args),
            P::CallParensBegin => self.paren(p, R::CallParens),
            P::CFunctionCall => self.function_call(p),
            P::MethodAccess => self.method_access(p),
            P::MemberAccess => self.member_access(p),
            P::PlainAccess => self.plain_access(p),
            P::InitBegin => self.init_begin(p),
            P::DefineBegin => self.define_begin(p),
            P::DefineParamName => self.define_param(p),
            P::DefineParamComma => self.byte_rule(p, b',', &[S::PunctParameters]),
            P::DefineParamEllipsis => {
                self.starts(p, b"...") && self.plain(p, p + 3, &[S::Ellipses, S::PunctVarargParam])
            }
            P::DiagBegin => self.named_directive(
                p,
                &["error", "warning"],
                "keyword.control.directive.diagnostic.",
                R::Diag,
            ),
            P::IncludeBegin => self.named_directive(
                p,
                &["include_next", "include", "import"],
                "keyword.control.directive.",
                R::Include,
            ),
            P::PragmaMark => self.pragma_mark(p),
            P::LineDirBegin => self.line_directive(p, "line", &[S::DirectiveLine], R::LineDir),
            P::UndefBegin => self.line_directive(p, "undef", &[S::DirectiveUndef], R::Undef),
            P::PragmaDirBegin => {
                self.line_directive(p, "pragma", &[S::DirectivePragma], R::PragmaDir)
            }
            P::DefineBlockBegin => self.brace(p, R::DefineBlock),
            P::DefineBlockInnerBegin => self.brace(p, R::DefineBlockInner),
            P::DefineLParen => self.byte_rule(p, b'(', &[S::PunctParensBegin]),
            P::DefineRParen => self.byte_rule(p, b')', &[S::PunctParensEnd]),
            P::DefineFuncBegin => self.define_func(p),
            P::DefineCallBegin => self.call_like(p, R::DefineCall, CallKind::Args),
            P::DefineParensBegin => self.paren(p, R::DefineParens),
            P::DefineDQuoteBegin => self.quote(p, b'"', R::DefineDQuote),
            P::DefineSQuoteBegin => self.quote(p, b'\'', R::DefineSQuote),
            P::DiagDQuoteBegin => self.quote(p, b'"', R::DiagDQuote),
            P::DiagSQuoteBegin => self.quote(p, b'\'', R::DiagSQuote),
            P::DiagTextBegin => self.any_but_quote(p, R::DiagText),
            P::IncludeDQBegin => self.quote(p, b'"', R::IncludeDQ),
            P::IncludeLtBegin => self.angle_begin(p),
            P::UndefName | P::CondNamed => self.macro_name(p, b'$', &[S::EntityFunctionPre]),
            P::PragmaName => self.macro_name(p, b'-', &[S::EntityAttrPragma]),
            P::CondIfBegin => self.cond_if(p, R::CondIf),
            P::CondIfBBegin => self.cond_if(p, R::CondIfB),
            P::EnabledIfBegin => self.cond_if_look(p, Look::Enabled, R::EnabledIf),
            P::EnabledIfBBegin => self.cond_if_look(p, Look::Enabled, R::EnabledIfB),
            P::DisabledIfBegin => self.cond_if_look(p, Look::Disabled, R::DisabledIf),
            P::DisabledIfBBegin => self.cond_if_look(p, Look::Disabled, R::DisabledIfB),
            P::Stray => self.stray(p),
            P::HeadContBegin => self.head_begin(p, R::HeadCont),
            P::HeadPlainBegin => self.head_begin(p, R::HeadPlain),
            P::CondElifBegin => self.cond_begin(p, &["elif"], Look::No, false, R::CondElif),
            P::LiveElifBegin => self.cond_begin(p, &["elif"], Look::Enabled, true, R::LiveElif),
            P::LiveElifBBegin => self.cond_begin(p, &["elif"], Look::Enabled, true, R::LiveElifB),
            P::LiveElseBegin => self.cond_begin(p, &["else"], Look::No, true, R::LiveElse),
            P::LiveElseBBegin => self.cond_begin(p, &["else"], Look::No, true, R::LiveElseB),
            P::DisabledElifBegin => {
                self.cond_begin(p, &["elif"], Look::Disabled, true, R::DisabledElif)
            }
            P::DisabledElifOtherBegin => {
                self.cond_begin(p, &["elif"], Look::No, true, R::DisabledElifOther)
            }
            P::DisabledElifOtherBBegin => {
                self.cond_begin(p, &["elif"], Look::No, true, R::DisabledElifOtherB)
            }
            P::ElseBranchBegin => self.cond_begin(p, &["else"], Look::No, true, R::ElseBranch),
            P::ElseBranchBBegin => self.cond_begin(p, &["else"], Look::No, true, R::ElseBranchB),
            P::ElifBranchBegin => self.cond_begin(p, &["elif"], Look::No, true, R::ElifBranch),
            P::ElifBranchBBegin => self.cond_begin(p, &["elif"], Look::No, true, R::ElifBranchB),
            P::EnabledBodyBegin => self.newline_begin(p, R::EnabledBody),
            P::EnabledBodyBBegin => self.newline_begin(p, R::EnabledBodyB),
            P::DisabledBodyBegin => self.newline_begin(p, R::DisabledBody),
            P::DisabledBodyBBegin => self.newline_begin(p, R::DisabledBodyB),
            P::DisabledElifBodyBegin => self.newline_begin(p, R::DisabledElifBody),
            P::ElifTailBegin => self.newline_begin(p, R::ElifTail),
            P::ElifTailBBegin => self.newline_begin(p, R::ElifTailB),
            P::ElifElseBegin => self.cond_begin(p, &["else"], Look::No, true, R::ElifElse),
            P::ElifElseBBegin => self.cond_begin(p, &["else"], Look::No, true, R::ElifElseB),
            P::ElifElifBegin => self.cond_begin(p, &["elif"], Look::No, true, R::ElifElif),
            P::NestBegin => self.nest_begin(p),
            P::CondDefinedKeyword => self.cond_defined(p),
            P::CondDefinedInvalid => self.cond_defined_invalid(p),
            P::CondTernaryBegin => self.ternary_begin(p, R::CondTernary),
            P::CondParensBegin => self.paren(p, R::CondParens),
            P::SwitchBegin => self.switch_begin(p),
            P::SwitchHeadBegin => self.switch_head(p),
            P::SwitchBodyBegin => self.switch_body(p),
            P::SwitchTailBegin => self.switch_tail(p),
            P::SwitchParensBegin => self.switch_parens(p),
            P::CaseBegin => self.label_begin(p, "case", &[S::KeywordCase], R::Case),
            P::DefaultBegin => self.label_begin(p, "default", &[S::KeywordDefault], R::Default),
            P::AsmLineEnd => self.asm_line_end(p),
            P::AsmParensBegin => self.asm_parens(p),
            P::AsmStringBegin => self.asm_string(p),
            P::AsmInnerBegin => self.asm_inner(p),
            P::AsmLabel => self.asm_label(p),
            P::AsmColon => self.byte_rule(p, b':', &[S::PunctColonAsm]),
            P::ParamDir => self.param_dir(p),
        }
    }
}
