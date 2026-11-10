//! The grammar's pattern lists: which patterns each context tries, in order,
//! and the bytes each pattern can start at.

use std::collections::HashMap;
use std::sync::OnceLock;

use super::state::Rule;

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
    DoxygenBare,
    DoxygenItalic,
    DoxygenBold,
    DoxygenRaw,
    DoxygenWordA,
    DoxygenWordB,
    DoxygenParameter,
    DoxygenWordC,
    DoxygenWordD,
    Gtkdoc,
    InlineComment,
    InlineEnd,
    InlineStar,
    DoubleQuote,
    SingleQuote,
    EscChar,
    EscBackslash,
    EscInvalid,
    Placeholder,
    PlaceholderBad,
    LineContinuation,
    Number,
    NumberSeparator,
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
    FunctionDefinitionBegin,
    FunctionParametersBegin,
    FunctionParensBegin,
    Probably,
    CallBegin,
    CallParensBegin,
    CFunctionCall,
    MethodAccess,
    MemberAccess,
    PlainAccess,
    InitializationBegin,
    DefineBegin,
    DefineParameterName,
    DefineParameterComma,
    DefineParameterEllipsis,
    DiagnosticBegin,
    IncludeBegin,
    PragmaMark,
    LineDirectiveBegin,
    UndefBegin,
    PragmaDirectiveBegin,
    DefineBlockBegin,
    DefineBlockInnerBegin,
    DefineLParen,
    DefineRParen,
    DefineFunctionBegin,
    DefineCallBegin,
    DefineParensBegin,
    DefineDoubleQuoteBegin,
    DefineSingleQuoteBegin,
    DiagnosticDoubleQuoteBegin,
    DiagnosticSingleQuoteBegin,
    DiagnosticTextBegin,
    IncludeDoubleQuoteBegin,
    IncludeAngleBegin,
    UndefName,
    PragmaName,
    ConditionalIfBegin,
    ConditionalIfBlockBegin,
    EnabledIfBegin,
    EnabledIfBlockBegin,
    DisabledIfBegin,
    DisabledIfBlockBegin,
    Stray,
    HeadContinuationBegin,
    HeadPlainBegin,
    ConditionalElifBegin,
    LiveElifBegin,
    LiveElifBlockBegin,
    LiveElseBegin,
    LiveElseBlockBegin,
    DisabledElifBegin,
    DisabledElifOtherBegin,
    DisabledElifOtherBlockBegin,
    ElseBranchBegin,
    ElseBranchBlockBegin,
    ElifBranchBegin,
    ElifBranchBlockBegin,
    EnabledBodyBegin,
    EnabledBodyBlockBegin,
    DisabledBodyBegin,
    DisabledBodyBlockBegin,
    DisabledElifBodyBegin,
    ElifTailBegin,
    ElifTailBlockBegin,
    ElifElseBegin,
    ElifElseBlockBegin,
    ElifElifBegin,
    NestBegin,
    ConditionalDefinedKeyword,
    ConditionalDefinedInvalid,
    ConditionalTernaryBegin,
    ConditionalNamed,
    ConditionalParensBegin,
    ConditionalConstant,
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
    ParameterDirections,
    /// The end pattern of the innermost rule.
    /// `ends` matches it by rule.
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
pub(super) enum Entry {
    P(P),
    G(&'static [Entry]),
}

use Entry::{G, P as I};

pub(super) const COMMENTS: &[Entry] = &[
    I(P::DocLine),
    I(P::DocBlockOne),
    I(P::DocBlockBegin),
    I(P::BannerBlock),
    I(P::BlockComment),
    I(P::BannerLine),
    I(P::LineLeading),
    I(P::LineComment),
];

pub(super) const STORAGE_TYPES: &[Entry] = &[
    I(P::Primitive),
    I(P::BuiltIn),
    I(P::EnumStructUnion),
    I(P::AsmBegin),
];

pub(super) const OPERATORS: &[Entry] = &[
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

const STRINGS: &[Entry] = &[I(P::DoubleQuote), I(P::SingleQuote)];

pub(super) const DOXY: &[Entry] = &[
    I(P::DoxygenBare),
    I(P::DoxygenItalic),
    I(P::DoxygenBold),
    I(P::DoxygenRaw),
    I(P::DoxygenWordA),
    I(P::DoxygenWordB),
    I(P::DoxygenParameter),
    I(P::DoxygenWordC),
    I(P::DoxygenWordD),
    I(P::Gtkdoc),
];

pub(super) const INLINE: &[Entry] = &[I(P::InlineComment)];
pub(super) const INLINE_CLOSE: &[Entry] = &[I(P::InlineEnd), I(P::InlineStar)];
pub(super) const ACCESS: &[Entry] = &[I(P::MemberAccess), I(P::PlainAccess)];
pub(super) const DEFINE_PARAMS: &[Entry] = &[
    I(P::DefineParameterName),
    I(P::DefineParameterComma),
    I(P::DefineParameterEllipsis),
];
pub(super) const PARAM_DIR: &[Entry] = &[I(P::ParameterDirections)];
pub(super) const NUM_SEP: &[Entry] = &[I(P::NumberSeparator)];
/// The patterns a number-like run is lexed again with.
/// Each of the first six must take the whole run from its start.
/// The last marks what none takes.
const NUMBER: &[Entry] = &[
    I(P::HexFloat),
    I(P::DecFloat),
    I(P::Binary),
    I(P::Octal),
    I(P::HexInt),
    I(P::DecInt),
    I(P::InvalidNumber),
];

const DISABLED_ENTRIES: &[Entry] = &[I(P::NestBegin), I(P::PragmaMark)];

/// The top-level patterns.
/// `$self` includes them.
const SELF: &[Entry] = &[
    I(P::EnabledIfBegin),
    I(P::DisabledIfBegin),
    I(P::ConditionalIfBegin),
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
    I(P::DiagnosticBegin),
    I(P::IncludeBegin),
    I(P::PragmaMark),
    I(P::LineDirectiveBegin),
    I(P::UndefBegin),
    I(P::PragmaDirectiveBegin),
    I(P::SysTypes),
    I(P::Pthread),
    I(P::Stdint),
    I(P::MacConst),
    I(P::MacType),
    I(P::PosixReserved),
    I(P::BlockBegin),
    I(P::ParensBegin),
    I(P::FunctionDefinitionBegin),
    I(P::LineContinuation),
    I(P::BracketBegin),
    I(P::BracketEmpty),
    I(P::Semi),
    I(P::Comma),
];

const BLOCK_INNARDS: &[Entry] = &[
    I(P::EnabledIfBlockBegin),
    I(P::DisabledIfBlockBegin),
    I(P::ConditionalIfBlockBegin),
    I(P::Stray),
    I(P::MethodAccess),
    I(P::MemberAccess),
    I(P::CFunctionCall),
    I(P::InitializationBegin),
    I(P::BlockInnerBegin),
    I(P::ParensBlockBegin),
    G(SELF),
];

const FUNC_INNARDS: &[Entry] = &[
    G(COMMENTS),
    G(STORAGE_TYPES),
    G(OPERATORS),
    I(P::Vararg),
    I(P::FunctionParametersBegin),
    I(P::FunctionParensBegin),
    G(SELF),
];

const CALL_INNARDS: &[Entry] = &[
    G(COMMENTS),
    G(STORAGE_TYPES),
    I(P::MethodAccess),
    I(P::MemberAccess),
    G(OPERATORS),
    I(P::CallBegin),
    I(P::CallParensBegin),
    G(BLOCK_INNARDS),
];

const COND_LINE: &[Entry] = &[
    I(P::ConditionalDefinedKeyword),
    I(P::ConditionalDefinedInvalid),
    G(COMMENTS),
    G(STRINGS),
    I(P::Number),
    I(P::ConditionalTernaryBegin),
    G(OPERATORS),
    I(P::ConditionalConstant),
    I(P::ConditionalNamed),
    I(P::LineContinuation),
    I(P::ConditionalParensBegin),
];

const DEFINE_CONTENTS: &[Entry] = &[
    I(P::Vararg),
    I(P::DefineBlockBegin),
    I(P::DefineLParen),
    I(P::DefineRParen),
    I(P::DefineFunctionBegin),
    I(P::DefineDoubleQuoteBegin),
    I(P::DefineSingleQuoteBegin),
    I(P::MethodAccess),
    I(P::MemberAccess),
    G(SELF),
];

const DEFINE_BLOCKS: &[Entry] = &[I(P::DefineBlockInnerBegin), G(DEFINE_CONTENTS)];

const DEFINE_FUNCTIONS: &[Entry] = &[
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

impl Rule {
    /// The patterns the context tries, in order.
    #[expect(
        clippy::too_many_lines,
        reason = "one arm per context: the table of pattern lists"
    )]
    fn items(self) -> &'static [&'static [Entry]] {
        match self {
            Rule::Top | Rule::Parens | Rule::LiveElse | Rule::EnabledBody | Rule::SwitchTail => {
                &[SELF]
            }
            Rule::Block | Rule::BlockInner | Rule::LiveElseBlock | Rule::EnabledBodyBlock => {
                &[BLOCK_INNARDS]
            }
            Rule::ParensBlock => &[BLOCK_INNARDS, &[I(P::RangeColon)]],
            Rule::FunctionDefinition | Rule::FunctionParens => &[FUNC_INNARDS],
            Rule::FunctionParameters => &[&[I(P::Probably)], FUNC_INNARDS],
            Rule::FunctionCall
            | Rule::CallArguments
            | Rule::CallParens
            | Rule::Method
            | Rule::Initialization
            | Rule::Bracket => &[CALL_INNARDS],
            Rule::Ternary | Rule::AsmInner => &[CALL_INNARDS, SELF],
            Rule::DocLine => &[&[I(P::LineContinuation)], DOXY],
            Rule::DocBlock | Rule::Doxygen => &[DOXY],
            Rule::BlockComment | Rule::IncludeDoubleQuote | Rule::IncludeAngle => &[],
            Rule::LineLeading => &[&[I(P::LineSlash)]],
            Rule::LineSlash
            | Rule::LineComment
            | Rule::DiagnosticDoubleQuote
            | Rule::DiagnosticSingleQuote => &[&[I(P::LineContinuation)]],
            Rule::DoubleQuote | Rule::DefineDoubleQuote => &[&[
                I(P::EscChar),
                I(P::EscInvalid),
                I(P::Placeholder),
                I(P::PlaceholderBad),
                I(P::LineContinuation),
            ]],
            Rule::SingleQuote | Rule::DefineSingleQuote => {
                &[&[I(P::EscChar), I(P::EscInvalid), I(P::LineContinuation)]]
            }
            Rule::Define => &[DEFINE_CONTENTS],
            Rule::DefineBlock => &[DEFINE_BLOCKS],
            Rule::DefineBlockInner => &[DEFINE_BLOCKS, DEFINE_CONTENTS],
            Rule::DefineFunction | Rule::DefineCall | Rule::DefineParens => &[DEFINE_FUNCTIONS],
            Rule::Diagnostic => &[&[
                I(P::DiagnosticDoubleQuoteBegin),
                I(P::DiagnosticSingleQuoteBegin),
                I(P::DiagnosticTextBegin),
            ]],
            Rule::DiagnosticText => &[&[I(P::LineContinuation)], COMMENTS],
            Rule::Include => &[&[
                I(P::LineContinuation),
                I(P::IncludeDoubleQuoteBegin),
                I(P::IncludeAngleBegin),
            ]],
            Rule::LineDirective => &[STRINGS, &[I(P::Number), I(P::LineContinuation)]],
            Rule::Undef => &[&[I(P::UndefName), I(P::LineContinuation)]],
            Rule::PragmaDirective => &[
                STRINGS,
                &[I(P::PragmaName), I(P::Number), I(P::LineContinuation)],
            ],
            Rule::ConditionalIf => &[
                &[
                    I(P::HeadContinuationBegin),
                    I(P::LiveElifBegin),
                    I(P::LiveElseBegin),
                    I(P::DisabledElifBegin),
                    I(P::ConditionalElifBegin),
                ],
                SELF,
            ],
            Rule::ConditionalIfBlock => &[
                &[
                    I(P::HeadContinuationBegin),
                    I(P::LiveElifBlockBegin),
                    I(P::LiveElseBlockBegin),
                    I(P::DisabledElifBegin),
                    I(P::ConditionalElifBegin),
                ],
                BLOCK_INNARDS,
            ],
            Rule::EnabledIf => &[
                &[I(P::HeadPlainBegin)],
                COMMENTS,
                &[
                    I(P::ElseBranchBegin),
                    I(P::ElifBranchBegin),
                    I(P::EnabledBodyBegin),
                ],
            ],
            Rule::EnabledIfBlock => &[
                &[I(P::HeadPlainBegin)],
                COMMENTS,
                &[
                    I(P::ElseBranchBlockBegin),
                    I(P::ElifBranchBlockBegin),
                    I(P::EnabledBodyBlockBegin),
                ],
            ],
            Rule::DisabledIf => &[
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
            Rule::DisabledIfBlock => &[
                &[I(P::HeadPlainBegin)],
                COMMENTS,
                &[
                    I(P::LiveElifBlockBegin),
                    I(P::LiveElseBlockBegin),
                    I(P::DisabledElifBegin),
                    I(P::DisabledElifOtherBlockBegin),
                    I(P::DisabledBodyBlockBegin),
                ],
            ],
            Rule::HeadContinuation
            | Rule::HeadPlain
            | Rule::ConditionalElif
            | Rule::ConditionalTernary
            | Rule::ConditionalParens => &[COND_LINE],
            Rule::LiveElif => &[
                &[I(P::HeadContinuationBegin)],
                COMMENTS,
                &[I(P::ElifTailBegin)],
            ],
            Rule::LiveElifBlock => &[
                &[I(P::HeadContinuationBegin)],
                COMMENTS,
                &[I(P::ElifTailBlockBegin)],
            ],
            Rule::ElifTail => &[&[I(P::ElifElseBegin), I(P::ElifElifBegin)], SELF],
            Rule::ElifTailBlock => &[
                &[I(P::ElifElseBlockBegin), I(P::ElifElifBegin)],
                BLOCK_INNARDS,
            ],
            Rule::ElifElse
            | Rule::ElifElseBlock
            | Rule::ElifElif
            | Rule::DisabledElifBody
            | Rule::DisabledBody
            | Rule::DisabledBodyBlock
            | Rule::ElseBranch
            | Rule::ElseBranchBlock
            | Rule::ElifBranch
            | Rule::ElifBranchBlock
            | Rule::Nest => &[DISABLED_ENTRIES],
            Rule::DisabledElif => &[
                &[I(P::HeadContinuationBegin)],
                COMMENTS,
                &[I(P::DisabledElifBodyBegin)],
            ],
            Rule::DisabledElifOther => &[&[I(P::HeadContinuationBegin)], SELF],
            Rule::DisabledElifOtherBlock => &[&[I(P::HeadContinuationBegin)], BLOCK_INNARDS],
            Rule::Switch => &[&[
                I(P::SwitchHeadBegin),
                I(P::SwitchBodyBegin),
                I(P::SwitchTailBegin),
            ]],
            Rule::SwitchHead => &[&[I(P::SwitchParensBegin)], SELF],
            Rule::SwitchBody => &[&[I(P::DefaultBegin), I(P::CaseBegin)], SELF, BLOCK_INNARDS],
            Rule::SwitchParens | Rule::Case | Rule::Default => {
                &[CALL_INNARDS, SELF, SELF, BLOCK_INNARDS]
            }
            Rule::Asm => &[&[I(P::AsmLineEnd)], COMMENTS, &[I(P::AsmParensBegin)]],
            Rule::AsmParens => &[
                &[
                    I(P::AsmStringBegin),
                    I(P::AsmInnerBegin),
                    I(P::AsmLabel),
                    I(P::AsmColon),
                ],
                COMMENTS,
            ],
            Rule::AsmString => &[&[I(P::EscBackslash), I(P::EscChar), I(P::EscInvalid)]],
            Rule::Inline => &[INLINE],
            Rule::InlineClose => &[INLINE_CLOSE],
            Rule::Access => &[ACCESS],
            Rule::DefineParameters => &[DEFINE_PARAMS],
            Rule::ParameterDirections => &[PARAM_DIR],
            Rule::NumberSeparator => &[NUM_SEP],
            Rule::Number => &[NUMBER],
        }
    }
}

/// The index of the bucket for the byte at `p`.
/// Bucket 256 is the end of the text.
pub(super) const EOT: usize = 256;

fn is_ws(b: u8) -> bool {
    matches!(b, b' ' | b'\t'..=b'\r') || b >= 0x80
}

fn is_ident(b: u8) -> bool {
    b.is_ascii_alphabetic() || b == b'_'
}

/// Whether a match of `p` can start at the byte `b`.
/// The answer is conservative.
/// A pattern may start at more bytes than it should, never at fewer.
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
        P::DoxygenBare
        | P::DoxygenItalic
        | P::DoxygenBold
        | P::DoxygenRaw
        | P::DoxygenWordA
        | P::DoxygenWordB
        | P::DoxygenParameter
        | P::DoxygenWordC
        | P::DoxygenWordD => matches!(b, b'@' | b'\\'),
        P::Gtkdoc => b == b'@' || b.is_ascii_uppercase(),
        P::InlineEnd | P::InlineStar => b == b'*',
        P::DoubleQuote
        | P::DefineDoubleQuoteBegin
        | P::DiagnosticDoubleQuoteBegin
        | P::IncludeDoubleQuoteBegin => b == b'"',
        P::SingleQuote
        | P::DefineSingleQuoteBegin
        | P::DiagnosticSingleQuoteBegin
        | P::NumberSeparator => b == b'\'',
        P::EscChar | P::EscBackslash | P::EscInvalid | P::LineContinuation => b == b'\\',
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
        | P::ConditionalConstant
        | P::FunctionDefinitionBegin => is_ident(b),
        P::Decrement => b == b'-',
        P::Increment => b == b'+',
        P::CompoundAssign | P::Arith => matches!(b, b'-' | b'%' | b'*' | b'+' | b'/'),
        P::BitwiseAssign => matches!(b, b'&' | b'^' | b'<' | b'>' | b'|'),
        P::Shift => matches!(b, b'<' | b'>'),
        P::Compare => matches!(b, b'!' | b'<' | b'>' | b'='),
        P::Logical => matches!(b, b'&' | b'!' | b'|'),
        P::BitOp => matches!(b, b'&' | b'^' | b'|' | b'~'),
        P::Assign => b == b'=',
        P::Ternary | P::ConditionalTernaryBegin => b == b'?',
        P::MacK => b == b'k',
        P::MacG => b == b'g',
        P::MacS => b == b's',
        P::PredefinedProbably => b == b'_',
        P::Semi => b == b';',
        P::Comma | P::DefineParameterComma => b == b',',
        P::BracketBegin => is_ident(b) || b == b'[' || b >= 0x80,
        P::BracketEmpty | P::AsmLabel => b == b'[',
        P::Vararg | P::DefineParameterEllipsis => b == b'.',
        P::BlockBegin | P::BlockInnerBegin | P::DefineBlockBegin | P::DefineBlockInnerBegin => {
            b == b'{'
        }
        P::ParensBegin
        | P::ParensBlockBegin
        | P::FunctionParensBegin
        | P::CallParensBegin
        | P::DefineLParen
        | P::ConditionalParensBegin
        | P::DefineParensBegin
        | P::AsmInnerBegin => b == b'(',
        P::DefineRParen => b == b')',
        P::RangeColon | P::AsmColon => b == b':',
        P::FunctionParametersBegin
        | P::CallBegin
        | P::CFunctionCall
        | P::DefineFunctionBegin
        | P::DefineCallBegin => call,
        P::MethodAccess | P::MemberAccess | P::PlainAccess => {
            is_ident(b) || is_ws(b) || matches!(b, b'.' | b'-')
        }
        P::InitializationBegin => is_ws(b),
        P::Probably | P::DefineParameterName => is_ws(b) || is_ident(b) || b >= 0x80,
        P::DefineBegin => is_ws(b) || matches!(b, b'/' | b'#'),
        P::DiagnosticBegin
        | P::IncludeBegin
        | P::PragmaMark
        | P::LineDirectiveBegin
        | P::UndefBegin
        | P::PragmaDirectiveBegin
        | P::ConditionalIfBegin
        | P::ConditionalIfBlockBegin
        | P::EnabledIfBegin
        | P::EnabledIfBlockBegin
        | P::DisabledIfBegin
        | P::DisabledIfBlockBegin
        | P::Stray
        | P::ConditionalElifBegin
        | P::LiveElifBegin
        | P::LiveElifBlockBegin
        | P::LiveElseBegin
        | P::LiveElseBlockBegin
        | P::DisabledElifBegin
        | P::DisabledElifOtherBegin
        | P::DisabledElifOtherBlockBegin
        | P::ElseBranchBegin
        | P::ElseBranchBlockBegin
        | P::ElifBranchBegin
        | P::ElifBranchBlockBegin
        | P::ElifElseBegin
        | P::ElifElseBlockBegin
        | P::ElifElifBegin
        | P::NestBegin => is_ws(b) || b == b'#',
        P::EnabledBodyBegin
        | P::EnabledBodyBlockBegin
        | P::DisabledBodyBegin
        | P::DisabledBodyBlockBegin
        | P::DisabledElifBodyBegin
        | P::ElifTailBegin
        | P::ElifTailBlockBegin => b == b'\n',
        P::UndefName | P::PragmaName | P::ConditionalNamed => is_ident(b) || b == b'$' || b >= 0x80,
        P::ConditionalDefinedKeyword | P::ConditionalDefinedInvalid => b == b'd',
        P::IncludeAngleBegin => b == b'<',
        P::DiagnosticTextBegin => b != b'"' && b != b'\'',
        P::SwitchBegin => is_ws(b) || matches!(b, b'/' | b's'),
        P::CaseBegin | P::DefaultBegin => is_ws(b) || matches!(b, b'/' | b'c' | b'd'),
        P::SwitchParensBegin | P::AsmParensBegin => is_ws(b) || matches!(b, b'/' | b'('),
        P::AsmStringBegin => matches!(b, b'R' | b'"'),
        P::ParameterDirections => matches!(b, b'i' | b'o'),
        P::HeadContinuationBegin
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
            | P::DiagnosticBegin
            | P::IncludeBegin
            | P::PragmaMark
            | P::LineDirectiveBegin
            | P::UndefBegin
            | P::PragmaDirectiveBegin
            | P::ConditionalIfBegin
            | P::ConditionalIfBlockBegin
            | P::EnabledIfBegin
            | P::EnabledIfBlockBegin
            | P::DisabledIfBegin
            | P::DisabledIfBlockBegin
            | P::Stray
            | P::ConditionalElifBegin
            | P::LiveElifBegin
            | P::LiveElifBlockBegin
            | P::LiveElseBegin
            | P::LiveElseBlockBegin
            | P::DisabledElifBegin
            | P::DisabledElifOtherBegin
            | P::DisabledElifOtherBlockBegin
            | P::ElseBranchBegin
            | P::ElseBranchBlockBegin
            | P::ElifBranchBegin
            | P::ElifBranchBlockBegin
            | P::ElifElseBegin
            | P::ElifElseBlockBegin
            | P::ElifElifBegin
            | P::NestBegin
    )
}

/// Whether `p` can match only where no word character comes before it.
/// Each of these patterns first looks at the character before.
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
            | P::ConditionalConstant
            | P::SysTypes
            | P::Pthread
            | P::Stdint
            | P::MacConst
            | P::MacType
            | P::PosixReserved
            | P::Predefined
            | P::PredefinedProbably
            | P::Number
            | P::FunctionDefinitionBegin
            | P::ConditionalDefinedKeyword
            | P::ConditionalDefinedInvalid
            | P::Sizeof
    )
}

/// Whether `p` reads only white space and comments from its position.
/// Inside a run of white space, such a pattern fails wherever it failed at
/// the first position of the run that the search tried.
fn run_start_only(p: P) -> bool {
    matches!(
        p,
        P::DocBlockBegin
            | P::BlockComment
            | P::LineComment
            | P::LineLeading
            | P::DefineBegin
            | P::InitializationBegin
            | P::SwitchBegin
            | P::SwitchParensBegin
            | P::AsmParensBegin
            | P::CaseBegin
            | P::DefaultBegin
    )
}

/// Whether a pattern of [`run_start_only`] may match in a run of white space
/// that `b` follows.
///
/// `b` must start what the pattern reads after the run:
/// - `/`, a comment;
/// - `#`, a `#define`;
/// - `(`, the parentheses of a `switch` or an `asm`;
/// - `s`, `c` and `d`, the words `switch`, `case` and `default`;
/// - `a`, `b`, `c`, `n`, `o`, `t` and `x`, the operator words of an
///   initialization. They are `and`, `bitor`, `compl`, `not`, `or`, `typeid`,
///   `xor`, `alignof` and the rest.
///
/// A pattern added to [`run_start_only`] adds its bytes here.
pub(super) fn blank_run_live(b: u8) -> bool {
    matches!(
        b,
        b'/' | b'#' | b'(' | b'a' | b'b' | b'c' | b'd' | b'n' | b'o' | b's' | b't' | b'x'
    )
}

/// Whether `p` reads only the chain that follows the word it starts in.
/// Take a word that starts with a letter, where the search tried such a
/// pattern at the word's start.
/// Inside that word, the pattern fails wherever it failed at the start.
fn word_tail_only(p: P) -> bool {
    matches!(
        p,
        P::MethodAccess | P::MemberAccess | P::PlainAccess | P::BracketBegin
    )
}

/// Whether `p` starts only at a name that a `(` follows.
/// The one exception is a list of words it excludes.
/// Those words may match inside a word.
fn call_only(p: P) -> bool {
    matches!(
        p,
        P::FunctionParametersBegin
            | P::CallBegin
            | P::CFunctionCall
            | P::DefineFunctionBegin
            | P::DefineCallBegin
    )
}

/// Whether `p` starts only at a name and then an operator that names an
/// object: after one of `])`, or at a letter.
fn object_only(p: P) -> bool {
    matches!(p, P::MethodAccess | P::MemberAccess | P::PlainAccess)
}

/// The places a position can be, as bits. Each excludes the patterns that
/// cannot match there.
pub(super) mod place {
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
    /// How many sets of these bits there are.
    pub(in crate::c) const COUNT: usize = 64;
}
use place::COUNT as PLACES;

/// Whether the pattern `p` can match at a position whose `place` flags are
/// `flags`.
fn allowed(flags: usize, p: P) -> bool {
    if flags & place::START == 0 && start_only(p) {
        return false;
    }
    if flags & place::AFTER_WORD != 0 && word_start(p) {
        return false;
    }
    if flags & place::BLANK_DEAD != 0 && run_start_only(p) {
        return false;
    }
    if flags & place::IN_WORD != 0 && word_tail_only(p) {
        return false;
    }
    if flags & place::NO_CALL != 0 && call_only(p) {
        return false;
    }
    flags & place::NO_OBJECT == 0 || !object_only(p)
}

/// One context's patterns by the position's `place` and first byte, in the
/// order the context tries them.
pub(super) struct Table {
    /// The class of each byte, and of the end of text.
    class_of: [u8; EOT + 1],
    /// How many classes the context has.
    classes: usize,
    /// For each `place` and class, the list that applies.
    ids: Vec<u16>,
    lists: Vec<Vec<P>>,
}

impl Table {
    /// The table of context `r`, built the first time a line reaches `r`.
    /// A line pays only for the contexts it opens.
    ///
    /// Bytes that start the same patterns share a class.
    /// Each `place` has a column per class.
    /// A class index fits a `u8`, and a list index fits a `u16`.
    /// `tables_fit_their_indices` checks both.
    /// The fallbacks therefore never apply.
    /// It is `None` only for an `r` outside [`Rule::ALL`].
    /// No such `r` exists.
    pub(super) fn of(r: Rule) -> Option<&'static Table> {
        static TABLES: [OnceLock<Table>; Rule::ALL.len()] =
            [const { OnceLock::new() }; Rule::ALL.len()];
        TABLES
            .get(r.index())
            .map(|table| table.get_or_init(move || Table::build(r)))
    }

    /// Builds the table over sets of the context's patterns.
    /// A set has one bit per pattern, in the order the context tries them.
    /// A class is the set of patterns a byte can start.
    /// A `place` is the set of patterns it allows.
    /// A list is made only for a set not seen before.
    fn build(r: Rule) -> Table {
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
        let mut ids = Vec::with_capacity(PLACES * signatures.len());
        let mut key = vec![0_u64; words];
        for flags in 0..PLACES {
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

    pub(super) fn candidates(&self, place: usize, index: usize) -> &[P] {
        let class = self
            .class_of
            .get(index)
            .map_or(0, |&class| usize::from(class));
        self.ids
            .get(place * self.classes + class)
            .and_then(|&id| self.lists.get(usize::from(id)))
            .map_or(&[], Vec::as_slice)
    }
}

fn flatten(list: &[Entry], out: &mut Vec<P>) {
    for item in list {
        match *item {
            Entry::P(p) => out.push(p),
            Entry::G(group) => flatten(group, out),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Rule, Table};

    #[test]
    fn tables_fit_their_indices() {
        for &r in Rule::ALL {
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
