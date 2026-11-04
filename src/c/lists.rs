//! The grammar's pattern lists: which patterns each context tries, in order,
//! and the bytes each pattern can start at.

use std::collections::HashMap;
use std::sync::OnceLock;

use super::state::R;

/// A pattern of the grammar: a `match` rule or the begin of a begin/end rule.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub(crate) enum P {
    DocLine,
    DocBlockOne,
    DocBlockBegin,
    BannerBlock,
    BlockComment,
    BannerLine,
    LineLeading,
    LineSlash,
    LineComment,
    DoxyBare,
    DoxyItalic,
    DoxyBold,
    DoxyRaw,
    DoxyWordA,
    DoxyWordB,
    DoxyParam,
    DoxyWordC,
    DoxyWordD,
    Gtkdoc,
    InlineComment,
    InlineEnd,
    InlineStar,
    DQuote,
    SQuote,
    EscChar,
    EscBackslash,
    EscInvalid,
    Placeholder,
    PlaceholderBad,
    LineCont,
    Number,
    NumSep,
    Sizeof,
    Decrement,
    Increment,
    CompoundAssign,
    BitwiseAssign,
    Shift,
    Compare,
    Logical,
    BitOp,
    Assign,
    Arith,
    Ternary,
    KeywordControl,
    Primitive,
    BuiltIn,
    EnumStructUnion,
    AsmBegin,
    Typedef,
    Modifier,
    MacK,
    MacG,
    MacS,
    ConstLang,
    SysTypes,
    Pthread,
    Stdint,
    MacConst,
    MacType,
    PosixReserved,
    Predefined,
    PredefinedProbably,
    Semi,
    Comma,
    BracketBegin,
    BracketEmpty,
    Vararg,
    BlockBegin,
    BlockInnerBegin,
    ParensBegin,
    ParensBlockBegin,
    RangeColon,
    FuncDefBegin,
    FuncParamsBegin,
    FuncParensBegin,
    Probably,
    CallBegin,
    CallParensBegin,
    CFunctionCall,
    MethodAccess,
    MemberAccess,
    PlainAccess,
    InitBegin,
    DefineBegin,
    DefineParamName,
    DefineParamComma,
    DefineParamEllipsis,
    DiagBegin,
    IncludeBegin,
    PragmaMark,
    LineDirBegin,
    UndefBegin,
    PragmaDirBegin,
    DefineBlockBegin,
    DefineBlockInnerBegin,
    DefineLParen,
    DefineRParen,
    DefineFuncBegin,
    DefineCallBegin,
    DefineParensBegin,
    DefineDQuoteBegin,
    DefineSQuoteBegin,
    DiagDQuoteBegin,
    DiagSQuoteBegin,
    DiagTextBegin,
    IncludeDQBegin,
    IncludeLtBegin,
    UndefName,
    PragmaName,
    CondIfBegin,
    CondIfBBegin,
    EnabledIfBegin,
    EnabledIfBBegin,
    DisabledIfBegin,
    DisabledIfBBegin,
    Stray,
    HeadContBegin,
    HeadPlainBegin,
    CondElifBegin,
    LiveElifBegin,
    LiveElifBBegin,
    LiveElseBegin,
    LiveElseBBegin,
    DisabledElifBegin,
    DisabledElifOtherBegin,
    DisabledElifOtherBBegin,
    ElseBranchBegin,
    ElseBranchBBegin,
    ElifBranchBegin,
    ElifBranchBBegin,
    EnabledBodyBegin,
    EnabledBodyBBegin,
    DisabledBodyBegin,
    DisabledBodyBBegin,
    DisabledElifBodyBegin,
    ElifTailBegin,
    ElifTailBBegin,
    ElifElseBegin,
    ElifElseBBegin,
    ElifElifBegin,
    NestBegin,
    CondDefinedKeyword,
    CondDefinedInvalid,
    CondTernaryBegin,
    CondNamed,
    CondParensBegin,
    CondConstant,
    SwitchBegin,
    SwitchHeadBegin,
    SwitchBodyBegin,
    SwitchTailBegin,
    SwitchParensBegin,
    CaseBegin,
    DefaultBegin,
    AsmLineEnd,
    AsmParensBegin,
    AsmStringBegin,
    AsmInnerBegin,
    AsmLabel,
    AsmColon,
    ParamDir,
    /// The end pattern of the innermost rule, which `ends` matches by rule.
    End,
    HexFloat,
    DecFloat,
    Binary,
    Octal,
    HexInt,
    DecInt,
    InvalidNumber,
}

/// One entry of a pattern list: a pattern, or a group of entries a rule
/// includes.
#[derive(Clone, Copy, Debug)]
pub(super) enum Item {
    P(P),
    G(&'static [Item]),
}

use Item::{G, P as I};

pub(super) const COMMENTS: &[Item] = &[
    I(P::DocLine),
    I(P::DocBlockOne),
    I(P::DocBlockBegin),
    I(P::BannerBlock),
    I(P::BlockComment),
    I(P::BannerLine),
    I(P::LineLeading),
    I(P::LineComment),
];

pub(super) const STORAGE_TYPES: &[Item] = &[
    I(P::Primitive),
    I(P::BuiltIn),
    I(P::EnumStructUnion),
    I(P::AsmBegin),
];

pub(super) const OPERATORS: &[Item] = &[
    I(P::Sizeof),
    I(P::Decrement),
    I(P::Increment),
    I(P::CompoundAssign),
    I(P::BitwiseAssign),
    I(P::Shift),
    I(P::Compare),
    I(P::Logical),
    I(P::BitOp),
    I(P::Assign),
    I(P::Arith),
    I(P::Ternary),
];

const STRINGS: &[Item] = &[I(P::DQuote), I(P::SQuote)];

pub(super) const DOXY: &[Item] = &[
    I(P::DoxyBare),
    I(P::DoxyItalic),
    I(P::DoxyBold),
    I(P::DoxyRaw),
    I(P::DoxyWordA),
    I(P::DoxyWordB),
    I(P::DoxyParam),
    I(P::DoxyWordC),
    I(P::DoxyWordD),
    I(P::Gtkdoc),
];

pub(super) const INLINE: &[Item] = &[I(P::InlineComment)];
pub(super) const INLINE_CLOSE: &[Item] = &[I(P::InlineEnd), I(P::InlineStar)];
pub(super) const ACCESS: &[Item] = &[I(P::MemberAccess), I(P::PlainAccess)];
pub(super) const DEFINE_PARAMS: &[Item] = &[
    I(P::DefineParamName),
    I(P::DefineParamComma),
    I(P::DefineParamEllipsis),
];
pub(super) const PARAM_DIR: &[Item] = &[I(P::ParamDir)];
pub(super) const NUM_SEP: &[Item] = &[I(P::NumSep)];
/// The patterns a number-like run is lexed again with: each of the first six
/// must take the whole run from its start, and the last marks what none takes.
const NUMBER: &[Item] = &[
    I(P::HexFloat),
    I(P::DecFloat),
    I(P::Binary),
    I(P::Octal),
    I(P::HexInt),
    I(P::DecInt),
    I(P::InvalidNumber),
];

const DISABLED_ITEMS: &[Item] = &[I(P::NestBegin), I(P::PragmaMark)];

/// The top-level patterns, which `$self` includes.
const SELF: &[Item] = &[
    I(P::EnabledIfBegin),
    I(P::DisabledIfBegin),
    I(P::CondIfBegin),
    I(P::Stray),
    I(P::Predefined),
    I(P::PredefinedProbably),
    G(COMMENTS),
    I(P::SwitchBegin),
    I(P::KeywordControl),
    G(STORAGE_TYPES),
    I(P::Typedef),
    I(P::Modifier),
    I(P::MacK),
    I(P::MacG),
    I(P::MacS),
    I(P::ConstLang),
    G(OPERATORS),
    I(P::Number),
    G(STRINGS),
    I(P::DefineBegin),
    I(P::DiagBegin),
    I(P::IncludeBegin),
    I(P::PragmaMark),
    I(P::LineDirBegin),
    I(P::UndefBegin),
    I(P::PragmaDirBegin),
    I(P::SysTypes),
    I(P::Pthread),
    I(P::Stdint),
    I(P::MacConst),
    I(P::MacType),
    I(P::PosixReserved),
    I(P::BlockBegin),
    I(P::ParensBegin),
    I(P::FuncDefBegin),
    I(P::LineCont),
    I(P::BracketBegin),
    I(P::BracketEmpty),
    I(P::Semi),
    I(P::Comma),
];

const BLOCK_INNARDS: &[Item] = &[
    I(P::EnabledIfBBegin),
    I(P::DisabledIfBBegin),
    I(P::CondIfBBegin),
    I(P::Stray),
    I(P::MethodAccess),
    I(P::MemberAccess),
    I(P::CFunctionCall),
    I(P::InitBegin),
    I(P::BlockInnerBegin),
    I(P::ParensBlockBegin),
    G(SELF),
];

const FUNC_INNARDS: &[Item] = &[
    G(COMMENTS),
    G(STORAGE_TYPES),
    G(OPERATORS),
    I(P::Vararg),
    I(P::FuncParamsBegin),
    I(P::FuncParensBegin),
    G(SELF),
];

const CALL_INNARDS: &[Item] = &[
    G(COMMENTS),
    G(STORAGE_TYPES),
    I(P::MethodAccess),
    I(P::MemberAccess),
    G(OPERATORS),
    I(P::CallBegin),
    I(P::CallParensBegin),
    G(BLOCK_INNARDS),
];

const COND_LINE: &[Item] = &[
    I(P::CondDefinedKeyword),
    I(P::CondDefinedInvalid),
    G(COMMENTS),
    G(STRINGS),
    I(P::Number),
    I(P::CondTernaryBegin),
    G(OPERATORS),
    I(P::CondConstant),
    I(P::CondNamed),
    I(P::LineCont),
    I(P::CondParensBegin),
];

const DEFINE_CONTENTS: &[Item] = &[
    I(P::Vararg),
    I(P::DefineBlockBegin),
    I(P::DefineLParen),
    I(P::DefineRParen),
    I(P::DefineFuncBegin),
    I(P::DefineDQuoteBegin),
    I(P::DefineSQuoteBegin),
    I(P::MethodAccess),
    I(P::MemberAccess),
    G(SELF),
];

const DEFINE_BLOCKS: &[Item] = &[I(P::DefineBlockInnerBegin), G(DEFINE_CONTENTS)];

const DEFINE_FUNCTIONS: &[Item] = &[
    G(COMMENTS),
    G(STORAGE_TYPES),
    I(P::Vararg),
    I(P::MethodAccess),
    I(P::MemberAccess),
    G(OPERATORS),
    I(P::DefineCallBegin),
    I(P::DefineParensBegin),
    G(DEFINE_CONTENTS),
];

impl R {
    /// The patterns the context tries, in order.
    #[expect(
        clippy::too_many_lines,
        reason = "one arm per context: the table of pattern lists"
    )]
    fn items(self) -> &'static [&'static [Item]] {
        match self {
            R::Top | R::Parens | R::LiveElse | R::EnabledBody | R::SwitchTail => &[SELF],
            R::Block | R::BlockInner | R::LiveElseB | R::EnabledBodyB => &[BLOCK_INNARDS],
            R::ParensBlock => &[BLOCK_INNARDS, &[I(P::RangeColon)]],
            R::FuncDef | R::FuncParens => &[FUNC_INNARDS],
            R::FuncParams => &[&[I(P::Probably)], FUNC_INNARDS],
            R::FuncCall | R::CallArgs | R::CallParens | R::Method | R::Init | R::Bracket => {
                &[CALL_INNARDS]
            }
            R::Ternary | R::AsmInner => &[CALL_INNARDS, SELF],
            R::DocLine => &[&[I(P::LineCont)], DOXY],
            R::DocBlock | R::Doxy => &[DOXY],
            R::BlockComment | R::IncludeDQ | R::IncludeLt => &[],
            R::LineLeading => &[&[I(P::LineSlash)]],
            R::LineSlash | R::LineComment | R::DiagDQuote | R::DiagSQuote => &[&[I(P::LineCont)]],
            R::DQuote | R::DefineDQuote => &[&[
                I(P::EscChar),
                I(P::EscInvalid),
                I(P::Placeholder),
                I(P::PlaceholderBad),
                I(P::LineCont),
            ]],
            R::SQuote | R::DefineSQuote => &[&[I(P::EscChar), I(P::EscInvalid), I(P::LineCont)]],
            R::Define => &[DEFINE_CONTENTS],
            R::DefineBlock => &[DEFINE_BLOCKS],
            R::DefineBlockInner => &[DEFINE_BLOCKS, DEFINE_CONTENTS],
            R::DefineFunc | R::DefineCall | R::DefineParens => &[DEFINE_FUNCTIONS],
            R::Diag => &[&[
                I(P::DiagDQuoteBegin),
                I(P::DiagSQuoteBegin),
                I(P::DiagTextBegin),
            ]],
            R::DiagText => &[&[I(P::LineCont)], COMMENTS],
            R::Include => &[&[I(P::LineCont), I(P::IncludeDQBegin), I(P::IncludeLtBegin)]],
            R::LineDir => &[STRINGS, &[I(P::Number), I(P::LineCont)]],
            R::Undef => &[&[I(P::UndefName), I(P::LineCont)]],
            R::PragmaDir => &[STRINGS, &[I(P::PragmaName), I(P::Number), I(P::LineCont)]],
            R::CondIf => &[
                &[
                    I(P::HeadContBegin),
                    I(P::LiveElifBegin),
                    I(P::LiveElseBegin),
                    I(P::DisabledElifBegin),
                    I(P::CondElifBegin),
                ],
                SELF,
            ],
            R::CondIfB => &[
                &[
                    I(P::HeadContBegin),
                    I(P::LiveElifBBegin),
                    I(P::LiveElseBBegin),
                    I(P::DisabledElifBegin),
                    I(P::CondElifBegin),
                ],
                BLOCK_INNARDS,
            ],
            R::EnabledIf => &[
                &[I(P::HeadPlainBegin)],
                COMMENTS,
                &[
                    I(P::ElseBranchBegin),
                    I(P::ElifBranchBegin),
                    I(P::EnabledBodyBegin),
                ],
            ],
            R::EnabledIfB => &[
                &[I(P::HeadPlainBegin)],
                COMMENTS,
                &[
                    I(P::ElseBranchBBegin),
                    I(P::ElifBranchBBegin),
                    I(P::EnabledBodyBBegin),
                ],
            ],
            R::DisabledIf => &[
                &[I(P::HeadPlainBegin)],
                COMMENTS,
                &[
                    I(P::LiveElifBegin),
                    I(P::LiveElseBegin),
                    I(P::DisabledElifBegin),
                    I(P::DisabledElifOtherBegin),
                    I(P::DisabledBodyBegin),
                ],
            ],
            R::DisabledIfB => &[
                &[I(P::HeadPlainBegin)],
                COMMENTS,
                &[
                    I(P::LiveElifBBegin),
                    I(P::LiveElseBBegin),
                    I(P::DisabledElifBegin),
                    I(P::DisabledElifOtherBBegin),
                    I(P::DisabledBodyBBegin),
                ],
            ],
            R::HeadCont | R::HeadPlain | R::CondElif | R::CondTernary | R::CondParens => {
                &[COND_LINE]
            }
            R::LiveElif => &[&[I(P::HeadContBegin)], COMMENTS, &[I(P::ElifTailBegin)]],
            R::LiveElifB => &[&[I(P::HeadContBegin)], COMMENTS, &[I(P::ElifTailBBegin)]],
            R::ElifTail => &[&[I(P::ElifElseBegin), I(P::ElifElifBegin)], SELF],
            R::ElifTailB => &[&[I(P::ElifElseBBegin), I(P::ElifElifBegin)], BLOCK_INNARDS],
            R::ElifElse
            | R::ElifElseB
            | R::ElifElif
            | R::DisabledElifBody
            | R::DisabledBody
            | R::DisabledBodyB
            | R::ElseBranch
            | R::ElseBranchB
            | R::ElifBranch
            | R::ElifBranchB
            | R::Nest => &[DISABLED_ITEMS],
            R::DisabledElif => &[
                &[I(P::HeadContBegin)],
                COMMENTS,
                &[I(P::DisabledElifBodyBegin)],
            ],
            R::DisabledElifOther => &[&[I(P::HeadContBegin)], SELF],
            R::DisabledElifOtherB => &[&[I(P::HeadContBegin)], BLOCK_INNARDS],
            R::Switch => &[&[
                I(P::SwitchHeadBegin),
                I(P::SwitchBodyBegin),
                I(P::SwitchTailBegin),
            ]],
            R::SwitchHead => &[&[I(P::SwitchParensBegin)], SELF],
            R::SwitchBody => &[&[I(P::DefaultBegin), I(P::CaseBegin)], SELF, BLOCK_INNARDS],
            R::SwitchParens | R::Case | R::Default => &[CALL_INNARDS, SELF, SELF, BLOCK_INNARDS],
            R::Asm => &[&[I(P::AsmLineEnd)], COMMENTS, &[I(P::AsmParensBegin)]],
            R::AsmParens => &[
                &[
                    I(P::AsmStringBegin),
                    I(P::AsmInnerBegin),
                    I(P::AsmLabel),
                    I(P::AsmColon),
                ],
                COMMENTS,
            ],
            R::AsmString => &[&[I(P::EscBackslash), I(P::EscChar), I(P::EscInvalid)]],
            R::Inline => &[INLINE],
            R::InlineClose => &[INLINE_CLOSE],
            R::Access => &[ACCESS],
            R::DefineParams => &[DEFINE_PARAMS],
            R::ParamDir => &[PARAM_DIR],
            R::NumSep => &[NUM_SEP],
            R::Number => &[NUMBER],
        }
    }
}

/// The index of the bucket for the byte at `p`; 256 is the end of the text.
pub(super) const EOT: usize = 256;

fn is_ws(b: u8) -> bool {
    matches!(b, b' ' | b'\t'..=b'\r') || b >= 0x80
}

fn is_ident(b: u8) -> bool {
    b.is_ascii_alphabetic() || b == b'_'
}

/// Whether a match of `p` can start at the byte `b`, conservatively: a
/// pattern may start at more bytes than it should, never at fewer.
#[expect(
    clippy::too_many_lines,
    reason = "one arm per pattern, the table of first bytes"
)]
fn starts(p: P, b: u8) -> bool {
    let call = is_ident(b)
        || matches!(
            b,
            b':' | b'-' | b'!' | b'&' | b'*' | b'+' | b'<' | b'=' | b'>' | b'(' | b'['
        );
    match p {
        P::DocLine | P::DocBlockBegin | P::BlockComment | P::LineComment => is_ws(b) || b == b'/',
        P::DocBlockOne | P::BannerBlock | P::BannerLine | P::InlineComment | P::LineSlash => {
            b == b'/'
        }
        P::LineLeading => matches!(b, b' ' | b'\t' | b'/'),
        P::DoxyBare
        | P::DoxyItalic
        | P::DoxyBold
        | P::DoxyRaw
        | P::DoxyWordA
        | P::DoxyWordB
        | P::DoxyParam
        | P::DoxyWordC
        | P::DoxyWordD => matches!(b, b'@' | b'\\'),
        P::Gtkdoc => b == b'@' || b.is_ascii_uppercase(),
        P::InlineEnd | P::InlineStar => b == b'*',
        P::DQuote | P::DefineDQuoteBegin | P::DiagDQuoteBegin | P::IncludeDQBegin => b == b'"',
        P::SQuote | P::DefineSQuoteBegin | P::DiagSQuoteBegin | P::NumSep => b == b'\'',
        P::EscChar | P::EscBackslash | P::EscInvalid | P::LineCont => b == b'\\',
        P::Placeholder | P::PlaceholderBad => b == b'%',
        P::Number => b == b'.' || b.is_ascii_digit() || b >= 0x80,
        P::Typedef => b == b't',
        P::Sizeof
        | P::Primitive
        | P::BuiltIn
        | P::EnumStructUnion
        | P::AsmBegin
        | P::Modifier
        | P::ConstLang
        | P::SysTypes
        | P::Pthread
        | P::Stdint
        | P::MacConst
        | P::MacType
        | P::PosixReserved
        | P::Predefined
        | P::KeywordControl
        | P::CondConstant
        | P::FuncDefBegin => is_ident(b),
        P::Decrement => b == b'-',
        P::Increment => b == b'+',
        P::CompoundAssign | P::Arith => matches!(b, b'-' | b'%' | b'*' | b'+' | b'/'),
        P::BitwiseAssign => matches!(b, b'&' | b'^' | b'<' | b'>' | b'|'),
        P::Shift => matches!(b, b'<' | b'>'),
        P::Compare => matches!(b, b'!' | b'<' | b'>' | b'='),
        P::Logical => matches!(b, b'&' | b'!' | b'|'),
        P::BitOp => matches!(b, b'&' | b'^' | b'|' | b'~'),
        P::Assign => b == b'=',
        P::Ternary | P::CondTernaryBegin => b == b'?',
        P::MacK => b == b'k',
        P::MacG => b == b'g',
        P::MacS => b == b's',
        P::PredefinedProbably => b == b'_',
        P::Semi => b == b';',
        P::Comma | P::DefineParamComma => b == b',',
        P::BracketBegin => is_ident(b) || b == b'[' || b >= 0x80,
        P::BracketEmpty | P::AsmLabel => b == b'[',
        P::Vararg | P::DefineParamEllipsis => b == b'.',
        P::BlockBegin | P::BlockInnerBegin | P::DefineBlockBegin | P::DefineBlockInnerBegin => {
            b == b'{'
        }
        P::ParensBegin
        | P::ParensBlockBegin
        | P::FuncParensBegin
        | P::CallParensBegin
        | P::DefineLParen
        | P::CondParensBegin
        | P::DefineParensBegin
        | P::AsmInnerBegin => b == b'(',
        P::DefineRParen => b == b')',
        P::RangeColon | P::AsmColon => b == b':',
        P::FuncParamsBegin
        | P::CallBegin
        | P::CFunctionCall
        | P::DefineFuncBegin
        | P::DefineCallBegin => call,
        P::MethodAccess | P::MemberAccess | P::PlainAccess => {
            is_ident(b) || is_ws(b) || matches!(b, b'.' | b'-')
        }
        P::InitBegin => is_ws(b),
        P::Probably | P::DefineParamName => is_ws(b) || is_ident(b) || b >= 0x80,
        P::DefineBegin => is_ws(b) || matches!(b, b'/' | b'#'),
        P::DiagBegin
        | P::IncludeBegin
        | P::PragmaMark
        | P::LineDirBegin
        | P::UndefBegin
        | P::PragmaDirBegin
        | P::CondIfBegin
        | P::CondIfBBegin
        | P::EnabledIfBegin
        | P::EnabledIfBBegin
        | P::DisabledIfBegin
        | P::DisabledIfBBegin
        | P::Stray
        | P::CondElifBegin
        | P::LiveElifBegin
        | P::LiveElifBBegin
        | P::LiveElseBegin
        | P::LiveElseBBegin
        | P::DisabledElifBegin
        | P::DisabledElifOtherBegin
        | P::DisabledElifOtherBBegin
        | P::ElseBranchBegin
        | P::ElseBranchBBegin
        | P::ElifBranchBegin
        | P::ElifBranchBBegin
        | P::ElifElseBegin
        | P::ElifElseBBegin
        | P::ElifElifBegin
        | P::NestBegin => is_ws(b) || b == b'#',
        P::EnabledBodyBegin
        | P::EnabledBodyBBegin
        | P::DisabledBodyBegin
        | P::DisabledBodyBBegin
        | P::DisabledElifBodyBegin
        | P::ElifTailBegin
        | P::ElifTailBBegin => b == b'\n',
        P::UndefName | P::PragmaName | P::CondNamed => is_ident(b) || b == b'$' || b >= 0x80,
        P::CondDefinedKeyword | P::CondDefinedInvalid => b == b'd',
        P::IncludeLtBegin => b == b'<',
        P::DiagTextBegin => b != b'"' && b != b'\'',
        P::SwitchBegin => is_ws(b) || matches!(b, b'/' | b's'),
        P::CaseBegin | P::DefaultBegin => is_ws(b) || matches!(b, b'/' | b'c' | b'd'),
        P::SwitchParensBegin | P::AsmParensBegin => is_ws(b) || matches!(b, b'/' | b'('),
        P::AsmStringBegin => matches!(b, b'R' | b'"'),
        P::ParamDir => matches!(b, b'i' | b'o'),
        P::HeadContBegin
        | P::HeadPlainBegin
        | P::SwitchHeadBegin
        | P::SwitchBodyBegin
        | P::SwitchTailBegin
        | P::AsmLineEnd
        | P::End => true,
        P::HexFloat | P::Binary | P::Octal | P::HexInt => b == b'0',
        P::DecFloat => b == b'.' || b.is_ascii_digit(),
        P::DecInt => b.is_ascii_digit(),
        P::InvalidNumber => {
            b.is_ascii_alphanumeric() || matches!(b, b'\'' | b'.' | b'_' | b'-' | b'+')
        }
    }
}

/// Whether `p` can match only at the start of the line, where `^` does.
fn start_only(p: P) -> bool {
    matches!(
        p,
        P::DocLine
            | P::BannerBlock
            | P::BannerLine
            | P::AsmLineEnd
            | P::DiagBegin
            | P::IncludeBegin
            | P::PragmaMark
            | P::LineDirBegin
            | P::UndefBegin
            | P::PragmaDirBegin
            | P::CondIfBegin
            | P::CondIfBBegin
            | P::EnabledIfBegin
            | P::EnabledIfBBegin
            | P::DisabledIfBegin
            | P::DisabledIfBBegin
            | P::Stray
            | P::CondElifBegin
            | P::LiveElifBegin
            | P::LiveElifBBegin
            | P::LiveElseBegin
            | P::LiveElseBBegin
            | P::DisabledElifBegin
            | P::DisabledElifOtherBegin
            | P::DisabledElifOtherBBegin
            | P::ElseBranchBegin
            | P::ElseBranchBBegin
            | P::ElifBranchBegin
            | P::ElifBranchBBegin
            | P::ElifElseBegin
            | P::ElifElseBBegin
            | P::ElifElifBegin
            | P::NestBegin
    )
}

/// Whether `p` can match only where no word character comes before it:
/// each of these looks at the character before first.
fn word_start(p: P) -> bool {
    matches!(
        p,
        P::KeywordControl
            | P::Primitive
            | P::BuiltIn
            | P::EnumStructUnion
            | P::AsmBegin
            | P::Modifier
            | P::MacK
            | P::MacG
            | P::MacS
            | P::ConstLang
            | P::CondConstant
            | P::SysTypes
            | P::Pthread
            | P::Stdint
            | P::MacConst
            | P::MacType
            | P::PosixReserved
            | P::Predefined
            | P::PredefinedProbably
            | P::Number
            | P::FuncDefBegin
            | P::CondDefinedKeyword
            | P::CondDefinedInvalid
            | P::Sizeof
    )
}

/// Whether `p` reads only white space and comments from its position, so
/// that inside a run of white space it fails wherever it failed at the first
/// position of the run that the search tried.
fn run_start_only(p: P) -> bool {
    matches!(
        p,
        P::DocBlockBegin
            | P::BlockComment
            | P::LineComment
            | P::LineLeading
            | P::DefineBegin
            | P::InitBegin
            | P::SwitchBegin
            | P::SwitchParensBegin
            | P::AsmParensBegin
            | P::CaseBegin
            | P::DefaultBegin
    )
}

/// Whether a pattern of [`run_start_only`] may match in a run of white space
/// that `b` follows. `b` must start what the pattern reads after the run:
/// `/` a comment, `#` a `#define`, `(` the parentheses of a `switch` or an
/// `asm`, `s`, `c` and `d` the words `switch`, `case` and `default`, and `a`,
/// `b`, `c`, `n`, `o`, `t` and `x` the operator words of an initialization
/// (`and`, `bitor`, `compl`, `not`, `or`, `typeid`, `xor`, `alignof` and the
/// rest). A pattern added to [`run_start_only`] adds its bytes here.
pub(super) fn blank_run_live(b: u8) -> bool {
    matches!(
        b,
        b'/' | b'#' | b'(' | b'a' | b'b' | b'c' | b'd' | b'n' | b'o' | b's' | b't' | b'x'
    )
}

/// Whether `p` reads only the chain that follows the word it starts in, so
/// that inside a word that starts with a letter it fails wherever it failed
/// at the start of the word, if the search tried that.
fn word_tail_only(p: P) -> bool {
    matches!(
        p,
        P::MethodAccess | P::MemberAccess | P::PlainAccess | P::BracketBegin
    )
}

/// Whether `p` starts only at a name that a `(` follows, except that a list
/// of words it excludes may match inside a word.
fn call_only(p: P) -> bool {
    matches!(
        p,
        P::FuncParamsBegin
            | P::CallBegin
            | P::CFunctionCall
            | P::DefineFuncBegin
            | P::DefineCallBegin
    )
}

/// Whether `p` starts only at a name and then an operator that names an
/// object: after one of `])`, or at a letter.
fn object_only(p: P) -> bool {
    matches!(p, P::MethodAccess | P::MemberAccess | P::PlainAccess)
}

/// The kinds of position, as bits. Each excludes the patterns that cannot
/// match there.
pub(super) mod kind {
    /// The start of the line.
    pub(in crate::c) const START: usize = 1;
    /// After a word character.
    pub(in crate::c) const AFTER_WORD: usize = 2;
    /// Inside a run of white space that the search has tried.
    pub(in crate::c) const BLANK_DEAD: usize = 4;
    /// Inside an ASCII word that the search has tried at its start.
    pub(in crate::c) const IN_WORD: usize = 8;
    /// Like `IN_WORD`, and no `(` follows the word.
    pub(in crate::c) const NO_CALL: usize = 16;
    /// Neither a letter nor after `]` or `)`.
    pub(in crate::c) const NO_OBJECT: usize = 32;
    /// How many kinds there are.
    pub(in crate::c) const COUNT: usize = 64;
}
use kind::COUNT as KINDS;

/// Whether the pattern `p` can match at a position of the kinds `flags`.
fn allowed(flags: usize, p: P) -> bool {
    if flags & kind::START == 0 && start_only(p) {
        return false;
    }
    if flags & kind::AFTER_WORD != 0 && word_start(p) {
        return false;
    }
    if flags & kind::BLANK_DEAD != 0 && run_start_only(p) {
        return false;
    }
    if flags & kind::IN_WORD != 0 && word_tail_only(p) {
        return false;
    }
    if flags & kind::NO_CALL != 0 && call_only(p) {
        return false;
    }
    flags & kind::NO_OBJECT == 0 || !object_only(p)
}

/// One context's patterns by kind of position and first byte, in the order
/// the context tries them.
pub(super) struct Table {
    /// The class of each byte, and of the end of text.
    class_of: [u8; EOT + 1],
    /// How many classes the context has.
    classes: usize,
    /// For each kind and class, the list that applies.
    ids: Vec<u16>,
    lists: Vec<Vec<P>>,
}

impl Table {
    /// The table of context `r`, built the first time a line reaches `r`,
    /// so a line pays only for the contexts it opens. Bytes that start the
    /// same patterns share a class, so each kind has a column per class. A
    /// class index fits a `u8` and a list index a `u16`, which
    /// `tables_fit_their_indices` checks, so the fallbacks never apply.
    /// `None` only for an `r` outside [`R::ALL`], which has no such value.
    pub(super) fn of(r: R) -> Option<&'static Table> {
        static TABLES: [OnceLock<Table>; R::ALL.len()] = [const { OnceLock::new() }; R::ALL.len()];
        TABLES
            .get(r.index())
            .map(|table| table.get_or_init(move || Table::build(r)))
    }

    /// Builds the table over sets of the context's patterns, one bit per
    /// pattern in the order the context tries them: a class is the set of
    /// patterns a byte can start, a kind the set it allows, and a list is
    /// made only for a set not seen before.
    fn build(r: R) -> Table {
        let mut flat = Vec::new();
        for list in r.items() {
            flatten(list, &mut flat);
        }
        let words = flat.len().div_ceil(64).max(1);
        let set = |keep: &dyn Fn(P) -> bool| {
            let mut bits = vec![0_u64; words];
            for (i, &p) in flat.iter().enumerate() {
                if keep(p) {
                    bits[i / 64] |= 1 << (i % 64);
                }
            }
            bits
        };
        let mut signatures: Vec<Vec<u64>> = Vec::new();
        let mut class_of = [0; EOT + 1];
        for (byte, class) in class_of.iter_mut().enumerate() {
            let signature = set(&|p| u8::try_from(byte).map_or(true, |b| starts(p, b)));
            let index = signatures
                .iter()
                .position(|known| *known == signature)
                .unwrap_or_else(|| {
                    signatures.push(signature);
                    signatures.len() - 1
                });
            *class = u8::try_from(index).unwrap_or(u8::MAX);
        }
        let mut lists: Vec<Vec<P>> = Vec::new();
        let mut seen: HashMap<Vec<u64>, u16> = HashMap::new();
        let mut ids = Vec::with_capacity(KINDS * signatures.len());
        let mut key = vec![0_u64; words];
        for flags in 0..KINDS {
            let allows = set(&|p| allowed(flags, p));
            for signature in &signatures {
                for ((bits, starts), allows) in key.iter_mut().zip(signature).zip(&allows) {
                    *bits = starts & allows;
                }
                let id = if let Some(&id) = seen.get(key.as_slice()) {
                    id
                } else {
                    let list = flat
                        .iter()
                        .enumerate()
                        .filter(|&(i, _)| key[i / 64] & (1 << (i % 64)) != 0)
                        .map(|(_, &p)| p)
                        .collect();
                    lists.push(list);
                    let id = u16::try_from(lists.len() - 1).unwrap_or(0);
                    seen.insert(key.clone(), id);
                    id
                };
                ids.push(id);
            }
        }
        Table {
            class_of,
            classes: signatures.len(),
            ids,
            lists,
        }
    }

    pub(super) fn candidates(&self, kind: usize, index: usize) -> &[P] {
        let class = self
            .class_of
            .get(index)
            .map_or(0, |&class| usize::from(class));
        self.ids
            .get(kind * self.classes + class)
            .and_then(|&id| self.lists.get(usize::from(id)))
            .map_or(&[], Vec::as_slice)
    }
}

fn flatten(list: &[Item], out: &mut Vec<P>) {
    for item in list {
        match *item {
            Item::P(p) => out.push(p),
            Item::G(group) => flatten(group, out),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{R, Table};

    #[test]
    fn tables_fit_their_indices() {
        for &r in R::ALL {
            let table = Table::of(r).unwrap_or_else(|| panic!("no table for {r:?}"));
            assert!(table.classes <= 1 << 8, "{r:?}: {} classes", table.classes);
            assert!(
                table.lists.len() <= 1 << 16,
                "{r:?}: {} lists",
                table.lists.len()
            );
        }
    }
}
