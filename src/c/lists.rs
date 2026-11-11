//! The grammar's pattern lists: which patterns each context tries, in order,
//! and the bytes each pattern can start at.

use std::collections::HashMap;
use std::sync::OnceLock;

use super::state::Rule;

/// A pattern of the grammar: a `match` rule or the begin of a begin/end rule.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub(crate) enum Pattern {
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
    BitOperator,
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
    DefineLeftParen,
    DefineRightParen,
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
    Pattern(Pattern),
    Group(&'static [Entry]),
}

pub(super) const COMMENTS: &[Entry] = &[
    Entry::Pattern(Pattern::DocLine),
    Entry::Pattern(Pattern::DocBlockOne),
    Entry::Pattern(Pattern::DocBlockBegin),
    Entry::Pattern(Pattern::BannerBlock),
    Entry::Pattern(Pattern::BlockComment),
    Entry::Pattern(Pattern::BannerLine),
    Entry::Pattern(Pattern::LineLeading),
    Entry::Pattern(Pattern::LineComment),
];

pub(super) const STORAGE_TYPES: &[Entry] = &[
    Entry::Pattern(Pattern::Primitive),
    Entry::Pattern(Pattern::BuiltIn),
    Entry::Pattern(Pattern::EnumStructUnion),
    Entry::Pattern(Pattern::AsmBegin),
];

pub(super) const OPERATORS: &[Entry] = &[
    Entry::Pattern(Pattern::Sizeof),
    Entry::Pattern(Pattern::Decrement),
    Entry::Pattern(Pattern::Increment),
    Entry::Pattern(Pattern::CompoundAssign),
    Entry::Pattern(Pattern::BitwiseAssign),
    Entry::Pattern(Pattern::Shift),
    Entry::Pattern(Pattern::Compare),
    Entry::Pattern(Pattern::Logical),
    Entry::Pattern(Pattern::BitOperator),
    Entry::Pattern(Pattern::Assign),
    Entry::Pattern(Pattern::Arith),
    Entry::Pattern(Pattern::Ternary),
];

const STRINGS: &[Entry] = &[
    Entry::Pattern(Pattern::DoubleQuote),
    Entry::Pattern(Pattern::SingleQuote),
];

pub(super) const DOXYGEN: &[Entry] = &[
    Entry::Pattern(Pattern::DoxygenBare),
    Entry::Pattern(Pattern::DoxygenItalic),
    Entry::Pattern(Pattern::DoxygenBold),
    Entry::Pattern(Pattern::DoxygenRaw),
    Entry::Pattern(Pattern::DoxygenWordA),
    Entry::Pattern(Pattern::DoxygenWordB),
    Entry::Pattern(Pattern::DoxygenParameter),
    Entry::Pattern(Pattern::DoxygenWordC),
    Entry::Pattern(Pattern::DoxygenWordD),
    Entry::Pattern(Pattern::Gtkdoc),
];

pub(super) const INLINE: &[Entry] = &[Entry::Pattern(Pattern::InlineComment)];
pub(super) const INLINE_CLOSE: &[Entry] = &[
    Entry::Pattern(Pattern::InlineEnd),
    Entry::Pattern(Pattern::InlineStar),
];
pub(super) const ACCESS: &[Entry] = &[
    Entry::Pattern(Pattern::MemberAccess),
    Entry::Pattern(Pattern::PlainAccess),
];
pub(super) const DEFINE_PARAMETERS: &[Entry] = &[
    Entry::Pattern(Pattern::DefineParameterName),
    Entry::Pattern(Pattern::DefineParameterComma),
    Entry::Pattern(Pattern::DefineParameterEllipsis),
];
pub(super) const PARAMETER_DIRECTIONS: &[Entry] = &[Entry::Pattern(Pattern::ParameterDirections)];
pub(super) const NUMBER_SEPARATOR: &[Entry] = &[Entry::Pattern(Pattern::NumberSeparator)];
/// The patterns a number-like run is lexed again with.
/// Each of the first six must take the whole run from its start.
/// The last marks what none takes.
const NUMBER: &[Entry] = &[
    Entry::Pattern(Pattern::HexFloat),
    Entry::Pattern(Pattern::DecFloat),
    Entry::Pattern(Pattern::Binary),
    Entry::Pattern(Pattern::Octal),
    Entry::Pattern(Pattern::HexInt),
    Entry::Pattern(Pattern::DecInt),
    Entry::Pattern(Pattern::InvalidNumber),
];

const DISABLED_ENTRIES: &[Entry] = &[
    Entry::Pattern(Pattern::NestBegin),
    Entry::Pattern(Pattern::PragmaMark),
];

/// The top-level patterns.
/// `$self` includes them.
const SELF: &[Entry] = &[
    Entry::Pattern(Pattern::EnabledIfBegin),
    Entry::Pattern(Pattern::DisabledIfBegin),
    Entry::Pattern(Pattern::ConditionalIfBegin),
    Entry::Pattern(Pattern::Stray),
    Entry::Pattern(Pattern::Predefined),
    Entry::Pattern(Pattern::PredefinedProbably),
    Entry::Group(COMMENTS),
    Entry::Pattern(Pattern::SwitchBegin),
    Entry::Pattern(Pattern::KeywordControl),
    Entry::Group(STORAGE_TYPES),
    Entry::Pattern(Pattern::Typedef),
    Entry::Pattern(Pattern::Modifier),
    Entry::Pattern(Pattern::MacK),
    Entry::Pattern(Pattern::MacG),
    Entry::Pattern(Pattern::MacS),
    Entry::Pattern(Pattern::ConstLang),
    Entry::Group(OPERATORS),
    Entry::Pattern(Pattern::Number),
    Entry::Group(STRINGS),
    Entry::Pattern(Pattern::DefineBegin),
    Entry::Pattern(Pattern::DiagnosticBegin),
    Entry::Pattern(Pattern::IncludeBegin),
    Entry::Pattern(Pattern::PragmaMark),
    Entry::Pattern(Pattern::LineDirectiveBegin),
    Entry::Pattern(Pattern::UndefBegin),
    Entry::Pattern(Pattern::PragmaDirectiveBegin),
    Entry::Pattern(Pattern::SysTypes),
    Entry::Pattern(Pattern::Pthread),
    Entry::Pattern(Pattern::Stdint),
    Entry::Pattern(Pattern::MacConst),
    Entry::Pattern(Pattern::MacType),
    Entry::Pattern(Pattern::PosixReserved),
    Entry::Pattern(Pattern::BlockBegin),
    Entry::Pattern(Pattern::ParensBegin),
    Entry::Pattern(Pattern::FunctionDefinitionBegin),
    Entry::Pattern(Pattern::LineContinuation),
    Entry::Pattern(Pattern::BracketBegin),
    Entry::Pattern(Pattern::BracketEmpty),
    Entry::Pattern(Pattern::Semi),
    Entry::Pattern(Pattern::Comma),
];

const BLOCK_INNARDS: &[Entry] = &[
    Entry::Pattern(Pattern::EnabledIfBlockBegin),
    Entry::Pattern(Pattern::DisabledIfBlockBegin),
    Entry::Pattern(Pattern::ConditionalIfBlockBegin),
    Entry::Pattern(Pattern::Stray),
    Entry::Pattern(Pattern::MethodAccess),
    Entry::Pattern(Pattern::MemberAccess),
    Entry::Pattern(Pattern::CFunctionCall),
    Entry::Pattern(Pattern::InitializationBegin),
    Entry::Pattern(Pattern::BlockInnerBegin),
    Entry::Pattern(Pattern::ParensBlockBegin),
    Entry::Group(SELF),
];

const FUNCTION_INNARDS: &[Entry] = &[
    Entry::Group(COMMENTS),
    Entry::Group(STORAGE_TYPES),
    Entry::Group(OPERATORS),
    Entry::Pattern(Pattern::Vararg),
    Entry::Pattern(Pattern::FunctionParametersBegin),
    Entry::Pattern(Pattern::FunctionParensBegin),
    Entry::Group(SELF),
];

const CALL_INNARDS: &[Entry] = &[
    Entry::Group(COMMENTS),
    Entry::Group(STORAGE_TYPES),
    Entry::Pattern(Pattern::MethodAccess),
    Entry::Pattern(Pattern::MemberAccess),
    Entry::Group(OPERATORS),
    Entry::Pattern(Pattern::CallBegin),
    Entry::Pattern(Pattern::CallParensBegin),
    Entry::Group(BLOCK_INNARDS),
];

const CONDITIONAL_LINE: &[Entry] = &[
    Entry::Pattern(Pattern::ConditionalDefinedKeyword),
    Entry::Pattern(Pattern::ConditionalDefinedInvalid),
    Entry::Group(COMMENTS),
    Entry::Group(STRINGS),
    Entry::Pattern(Pattern::Number),
    Entry::Pattern(Pattern::ConditionalTernaryBegin),
    Entry::Group(OPERATORS),
    Entry::Pattern(Pattern::ConditionalConstant),
    Entry::Pattern(Pattern::ConditionalNamed),
    Entry::Pattern(Pattern::LineContinuation),
    Entry::Pattern(Pattern::ConditionalParensBegin),
];

const DEFINE_CONTENTS: &[Entry] = &[
    Entry::Pattern(Pattern::Vararg),
    Entry::Pattern(Pattern::DefineBlockBegin),
    Entry::Pattern(Pattern::DefineLeftParen),
    Entry::Pattern(Pattern::DefineRightParen),
    Entry::Pattern(Pattern::DefineFunctionBegin),
    Entry::Pattern(Pattern::DefineDoubleQuoteBegin),
    Entry::Pattern(Pattern::DefineSingleQuoteBegin),
    Entry::Pattern(Pattern::MethodAccess),
    Entry::Pattern(Pattern::MemberAccess),
    Entry::Group(SELF),
];

const DEFINE_BLOCKS: &[Entry] = &[
    Entry::Pattern(Pattern::DefineBlockInnerBegin),
    Entry::Group(DEFINE_CONTENTS),
];

const DEFINE_FUNCTIONS: &[Entry] = &[
    Entry::Group(COMMENTS),
    Entry::Group(STORAGE_TYPES),
    Entry::Pattern(Pattern::Vararg),
    Entry::Pattern(Pattern::MethodAccess),
    Entry::Pattern(Pattern::MemberAccess),
    Entry::Group(OPERATORS),
    Entry::Pattern(Pattern::DefineCallBegin),
    Entry::Pattern(Pattern::DefineParensBegin),
    Entry::Group(DEFINE_CONTENTS),
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
            Rule::ParensBlock => &[BLOCK_INNARDS, &[Entry::Pattern(Pattern::RangeColon)]],
            Rule::FunctionDefinition | Rule::FunctionParens => &[FUNCTION_INNARDS],
            Rule::FunctionParameters => &[&[Entry::Pattern(Pattern::Probably)], FUNCTION_INNARDS],
            Rule::FunctionCall
            | Rule::CallArguments
            | Rule::CallParens
            | Rule::Method
            | Rule::Initialization
            | Rule::Bracket => &[CALL_INNARDS],
            Rule::Ternary | Rule::AsmInner => &[CALL_INNARDS, SELF],
            Rule::DocLine => &[&[Entry::Pattern(Pattern::LineContinuation)], DOXYGEN],
            Rule::DocBlock | Rule::Doxygen => &[DOXYGEN],
            Rule::BlockComment | Rule::IncludeDoubleQuote | Rule::IncludeAngle => &[],
            Rule::LineLeading => &[&[Entry::Pattern(Pattern::LineSlash)]],
            Rule::LineSlash
            | Rule::LineComment
            | Rule::DiagnosticDoubleQuote
            | Rule::DiagnosticSingleQuote => &[&[Entry::Pattern(Pattern::LineContinuation)]],
            Rule::DoubleQuote | Rule::DefineDoubleQuote => &[&[
                Entry::Pattern(Pattern::EscChar),
                Entry::Pattern(Pattern::EscInvalid),
                Entry::Pattern(Pattern::Placeholder),
                Entry::Pattern(Pattern::PlaceholderBad),
                Entry::Pattern(Pattern::LineContinuation),
            ]],
            Rule::SingleQuote | Rule::DefineSingleQuote => &[&[
                Entry::Pattern(Pattern::EscChar),
                Entry::Pattern(Pattern::EscInvalid),
                Entry::Pattern(Pattern::LineContinuation),
            ]],
            Rule::Define => &[DEFINE_CONTENTS],
            Rule::DefineBlock => &[DEFINE_BLOCKS],
            Rule::DefineBlockInner => &[DEFINE_BLOCKS, DEFINE_CONTENTS],
            Rule::DefineFunction | Rule::DefineCall | Rule::DefineParens => &[DEFINE_FUNCTIONS],
            Rule::Diagnostic => &[&[
                Entry::Pattern(Pattern::DiagnosticDoubleQuoteBegin),
                Entry::Pattern(Pattern::DiagnosticSingleQuoteBegin),
                Entry::Pattern(Pattern::DiagnosticTextBegin),
            ]],
            Rule::DiagnosticText => &[&[Entry::Pattern(Pattern::LineContinuation)], COMMENTS],
            Rule::Include => &[&[
                Entry::Pattern(Pattern::LineContinuation),
                Entry::Pattern(Pattern::IncludeDoubleQuoteBegin),
                Entry::Pattern(Pattern::IncludeAngleBegin),
            ]],
            Rule::LineDirective => &[
                STRINGS,
                &[
                    Entry::Pattern(Pattern::Number),
                    Entry::Pattern(Pattern::LineContinuation),
                ],
            ],
            Rule::Undef => &[&[
                Entry::Pattern(Pattern::UndefName),
                Entry::Pattern(Pattern::LineContinuation),
            ]],
            Rule::PragmaDirective => &[
                STRINGS,
                &[
                    Entry::Pattern(Pattern::PragmaName),
                    Entry::Pattern(Pattern::Number),
                    Entry::Pattern(Pattern::LineContinuation),
                ],
            ],
            Rule::ConditionalIf => &[
                &[
                    Entry::Pattern(Pattern::HeadContinuationBegin),
                    Entry::Pattern(Pattern::LiveElifBegin),
                    Entry::Pattern(Pattern::LiveElseBegin),
                    Entry::Pattern(Pattern::DisabledElifBegin),
                    Entry::Pattern(Pattern::ConditionalElifBegin),
                ],
                SELF,
            ],
            Rule::ConditionalIfBlock => &[
                &[
                    Entry::Pattern(Pattern::HeadContinuationBegin),
                    Entry::Pattern(Pattern::LiveElifBlockBegin),
                    Entry::Pattern(Pattern::LiveElseBlockBegin),
                    Entry::Pattern(Pattern::DisabledElifBegin),
                    Entry::Pattern(Pattern::ConditionalElifBegin),
                ],
                BLOCK_INNARDS,
            ],
            Rule::EnabledIf => &[
                &[Entry::Pattern(Pattern::HeadPlainBegin)],
                COMMENTS,
                &[
                    Entry::Pattern(Pattern::ElseBranchBegin),
                    Entry::Pattern(Pattern::ElifBranchBegin),
                    Entry::Pattern(Pattern::EnabledBodyBegin),
                ],
            ],
            Rule::EnabledIfBlock => &[
                &[Entry::Pattern(Pattern::HeadPlainBegin)],
                COMMENTS,
                &[
                    Entry::Pattern(Pattern::ElseBranchBlockBegin),
                    Entry::Pattern(Pattern::ElifBranchBlockBegin),
                    Entry::Pattern(Pattern::EnabledBodyBlockBegin),
                ],
            ],
            Rule::DisabledIf => &[
                &[Entry::Pattern(Pattern::HeadPlainBegin)],
                COMMENTS,
                &[
                    Entry::Pattern(Pattern::LiveElifBegin),
                    Entry::Pattern(Pattern::LiveElseBegin),
                    Entry::Pattern(Pattern::DisabledElifBegin),
                    Entry::Pattern(Pattern::DisabledElifOtherBegin),
                    Entry::Pattern(Pattern::DisabledBodyBegin),
                ],
            ],
            Rule::DisabledIfBlock => &[
                &[Entry::Pattern(Pattern::HeadPlainBegin)],
                COMMENTS,
                &[
                    Entry::Pattern(Pattern::LiveElifBlockBegin),
                    Entry::Pattern(Pattern::LiveElseBlockBegin),
                    Entry::Pattern(Pattern::DisabledElifBegin),
                    Entry::Pattern(Pattern::DisabledElifOtherBlockBegin),
                    Entry::Pattern(Pattern::DisabledBodyBlockBegin),
                ],
            ],
            Rule::HeadContinuation
            | Rule::HeadPlain
            | Rule::ConditionalElif
            | Rule::ConditionalTernary
            | Rule::ConditionalParens => &[CONDITIONAL_LINE],
            Rule::LiveElif => &[
                &[Entry::Pattern(Pattern::HeadContinuationBegin)],
                COMMENTS,
                &[Entry::Pattern(Pattern::ElifTailBegin)],
            ],
            Rule::LiveElifBlock => &[
                &[Entry::Pattern(Pattern::HeadContinuationBegin)],
                COMMENTS,
                &[Entry::Pattern(Pattern::ElifTailBlockBegin)],
            ],
            Rule::ElifTail => &[
                &[
                    Entry::Pattern(Pattern::ElifElseBegin),
                    Entry::Pattern(Pattern::ElifElifBegin),
                ],
                SELF,
            ],
            Rule::ElifTailBlock => &[
                &[
                    Entry::Pattern(Pattern::ElifElseBlockBegin),
                    Entry::Pattern(Pattern::ElifElifBegin),
                ],
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
                &[Entry::Pattern(Pattern::HeadContinuationBegin)],
                COMMENTS,
                &[Entry::Pattern(Pattern::DisabledElifBodyBegin)],
            ],
            Rule::DisabledElifOther => &[&[Entry::Pattern(Pattern::HeadContinuationBegin)], SELF],
            Rule::DisabledElifOtherBlock => &[
                &[Entry::Pattern(Pattern::HeadContinuationBegin)],
                BLOCK_INNARDS,
            ],
            Rule::Switch => &[&[
                Entry::Pattern(Pattern::SwitchHeadBegin),
                Entry::Pattern(Pattern::SwitchBodyBegin),
                Entry::Pattern(Pattern::SwitchTailBegin),
            ]],
            Rule::SwitchHead => &[&[Entry::Pattern(Pattern::SwitchParensBegin)], SELF],
            Rule::SwitchBody => &[
                &[
                    Entry::Pattern(Pattern::DefaultBegin),
                    Entry::Pattern(Pattern::CaseBegin),
                ],
                SELF,
                BLOCK_INNARDS,
            ],
            Rule::SwitchParens | Rule::Case | Rule::Default => {
                &[CALL_INNARDS, SELF, SELF, BLOCK_INNARDS]
            }
            Rule::Asm => &[
                &[Entry::Pattern(Pattern::AsmLineEnd)],
                COMMENTS,
                &[Entry::Pattern(Pattern::AsmParensBegin)],
            ],
            Rule::AsmParens => &[
                &[
                    Entry::Pattern(Pattern::AsmStringBegin),
                    Entry::Pattern(Pattern::AsmInnerBegin),
                    Entry::Pattern(Pattern::AsmLabel),
                    Entry::Pattern(Pattern::AsmColon),
                ],
                COMMENTS,
            ],
            Rule::AsmString => &[&[
                Entry::Pattern(Pattern::EscBackslash),
                Entry::Pattern(Pattern::EscChar),
                Entry::Pattern(Pattern::EscInvalid),
            ]],
            Rule::Inline => &[INLINE],
            Rule::InlineClose => &[INLINE_CLOSE],
            Rule::Access => &[ACCESS],
            Rule::DefineParameters => &[DEFINE_PARAMETERS],
            Rule::ParameterDirections => &[PARAMETER_DIRECTIONS],
            Rule::NumberSeparator => &[NUMBER_SEPARATOR],
            Rule::Number => &[NUMBER],
        }
    }
}

/// The bucket index for the byte at `p`.
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
fn starts(p: Pattern, b: u8) -> bool {
    let call = is_ident(b)
        || matches!(
            b,
            b':' | b'-' | b'!' | b'&' | b'*' | b'+' | b'<' | b'=' | b'>' | b'(' | b'['
        );
    match p {
        Pattern::DocLine
        | Pattern::DocBlockBegin
        | Pattern::BlockComment
        | Pattern::LineComment => is_ws(b) || b == b'/',
        Pattern::DocBlockOne
        | Pattern::BannerBlock
        | Pattern::BannerLine
        | Pattern::InlineComment
        | Pattern::LineSlash => b == b'/',
        Pattern::LineLeading => matches!(b, b' ' | b'\t' | b'/'),
        Pattern::DoxygenBare
        | Pattern::DoxygenItalic
        | Pattern::DoxygenBold
        | Pattern::DoxygenRaw
        | Pattern::DoxygenWordA
        | Pattern::DoxygenWordB
        | Pattern::DoxygenParameter
        | Pattern::DoxygenWordC
        | Pattern::DoxygenWordD => matches!(b, b'@' | b'\\'),
        Pattern::Gtkdoc => b == b'@' || b.is_ascii_uppercase(),
        Pattern::InlineEnd | Pattern::InlineStar => b == b'*',
        Pattern::DoubleQuote
        | Pattern::DefineDoubleQuoteBegin
        | Pattern::DiagnosticDoubleQuoteBegin
        | Pattern::IncludeDoubleQuoteBegin => b == b'"',
        Pattern::SingleQuote
        | Pattern::DefineSingleQuoteBegin
        | Pattern::DiagnosticSingleQuoteBegin
        | Pattern::NumberSeparator => b == b'\'',
        Pattern::EscChar
        | Pattern::EscBackslash
        | Pattern::EscInvalid
        | Pattern::LineContinuation => b == b'\\',
        Pattern::Placeholder | Pattern::PlaceholderBad => b == b'%',
        Pattern::Number => b == b'.' || b.is_ascii_digit() || b >= 0x80,
        Pattern::Typedef => b == b't',
        Pattern::Sizeof
        | Pattern::Primitive
        | Pattern::BuiltIn
        | Pattern::EnumStructUnion
        | Pattern::AsmBegin
        | Pattern::Modifier
        | Pattern::ConstLang
        | Pattern::SysTypes
        | Pattern::Pthread
        | Pattern::Stdint
        | Pattern::MacConst
        | Pattern::MacType
        | Pattern::PosixReserved
        | Pattern::Predefined
        | Pattern::KeywordControl
        | Pattern::ConditionalConstant
        | Pattern::FunctionDefinitionBegin => is_ident(b),
        Pattern::Decrement => b == b'-',
        Pattern::Increment => b == b'+',
        Pattern::CompoundAssign | Pattern::Arith => matches!(b, b'-' | b'%' | b'*' | b'+' | b'/'),
        Pattern::BitwiseAssign => matches!(b, b'&' | b'^' | b'<' | b'>' | b'|'),
        Pattern::Shift => matches!(b, b'<' | b'>'),
        Pattern::Compare => matches!(b, b'!' | b'<' | b'>' | b'='),
        Pattern::Logical => matches!(b, b'&' | b'!' | b'|'),
        Pattern::BitOperator => matches!(b, b'&' | b'^' | b'|' | b'~'),
        Pattern::Assign => b == b'=',
        Pattern::Ternary | Pattern::ConditionalTernaryBegin => b == b'?',
        Pattern::MacK => b == b'k',
        Pattern::MacG => b == b'g',
        Pattern::MacS => b == b's',
        Pattern::PredefinedProbably => b == b'_',
        Pattern::Semi => b == b';',
        Pattern::Comma | Pattern::DefineParameterComma => b == b',',
        Pattern::BracketBegin => is_ident(b) || b == b'[' || b >= 0x80,
        Pattern::BracketEmpty | Pattern::AsmLabel => b == b'[',
        Pattern::Vararg | Pattern::DefineParameterEllipsis => b == b'.',
        Pattern::BlockBegin
        | Pattern::BlockInnerBegin
        | Pattern::DefineBlockBegin
        | Pattern::DefineBlockInnerBegin => b == b'{',
        Pattern::ParensBegin
        | Pattern::ParensBlockBegin
        | Pattern::FunctionParensBegin
        | Pattern::CallParensBegin
        | Pattern::DefineLeftParen
        | Pattern::ConditionalParensBegin
        | Pattern::DefineParensBegin
        | Pattern::AsmInnerBegin => b == b'(',
        Pattern::DefineRightParen => b == b')',
        Pattern::RangeColon | Pattern::AsmColon => b == b':',
        Pattern::FunctionParametersBegin
        | Pattern::CallBegin
        | Pattern::CFunctionCall
        | Pattern::DefineFunctionBegin
        | Pattern::DefineCallBegin => call,
        Pattern::MethodAccess | Pattern::MemberAccess | Pattern::PlainAccess => {
            is_ident(b) || is_ws(b) || matches!(b, b'.' | b'-')
        }
        Pattern::InitializationBegin => is_ws(b),
        Pattern::Probably | Pattern::DefineParameterName => is_ws(b) || is_ident(b) || b >= 0x80,
        Pattern::DefineBegin => is_ws(b) || matches!(b, b'/' | b'#'),
        Pattern::DiagnosticBegin
        | Pattern::IncludeBegin
        | Pattern::PragmaMark
        | Pattern::LineDirectiveBegin
        | Pattern::UndefBegin
        | Pattern::PragmaDirectiveBegin
        | Pattern::ConditionalIfBegin
        | Pattern::ConditionalIfBlockBegin
        | Pattern::EnabledIfBegin
        | Pattern::EnabledIfBlockBegin
        | Pattern::DisabledIfBegin
        | Pattern::DisabledIfBlockBegin
        | Pattern::Stray
        | Pattern::ConditionalElifBegin
        | Pattern::LiveElifBegin
        | Pattern::LiveElifBlockBegin
        | Pattern::LiveElseBegin
        | Pattern::LiveElseBlockBegin
        | Pattern::DisabledElifBegin
        | Pattern::DisabledElifOtherBegin
        | Pattern::DisabledElifOtherBlockBegin
        | Pattern::ElseBranchBegin
        | Pattern::ElseBranchBlockBegin
        | Pattern::ElifBranchBegin
        | Pattern::ElifBranchBlockBegin
        | Pattern::ElifElseBegin
        | Pattern::ElifElseBlockBegin
        | Pattern::ElifElifBegin
        | Pattern::NestBegin => is_ws(b) || b == b'#',
        Pattern::EnabledBodyBegin
        | Pattern::EnabledBodyBlockBegin
        | Pattern::DisabledBodyBegin
        | Pattern::DisabledBodyBlockBegin
        | Pattern::DisabledElifBodyBegin
        | Pattern::ElifTailBegin
        | Pattern::ElifTailBlockBegin => b == b'\n',
        Pattern::UndefName | Pattern::PragmaName | Pattern::ConditionalNamed => {
            is_ident(b) || b == b'$' || b >= 0x80
        }
        Pattern::ConditionalDefinedKeyword | Pattern::ConditionalDefinedInvalid => b == b'd',
        Pattern::IncludeAngleBegin => b == b'<',
        Pattern::DiagnosticTextBegin => b != b'"' && b != b'\'',
        Pattern::SwitchBegin => is_ws(b) || matches!(b, b'/' | b's'),
        Pattern::CaseBegin | Pattern::DefaultBegin => is_ws(b) || matches!(b, b'/' | b'c' | b'd'),
        Pattern::SwitchParensBegin | Pattern::AsmParensBegin => {
            is_ws(b) || matches!(b, b'/' | b'(')
        }
        Pattern::AsmStringBegin => matches!(b, b'R' | b'"'),
        Pattern::ParameterDirections => matches!(b, b'i' | b'o'),
        Pattern::HeadContinuationBegin
        | Pattern::HeadPlainBegin
        | Pattern::SwitchHeadBegin
        | Pattern::SwitchBodyBegin
        | Pattern::SwitchTailBegin
        | Pattern::AsmLineEnd
        | Pattern::End => true,
        Pattern::HexFloat | Pattern::Binary | Pattern::Octal | Pattern::HexInt => b == b'0',
        Pattern::DecFloat => b == b'.' || b.is_ascii_digit(),
        Pattern::DecInt => b.is_ascii_digit(),
        Pattern::InvalidNumber => {
            b.is_ascii_alphanumeric() || matches!(b, b'\'' | b'.' | b'_' | b'-' | b'+')
        }
    }
}

/// Whether `p` can match only at the start of the line, where `^` does.
fn start_only(p: Pattern) -> bool {
    matches!(
        p,
        Pattern::DocLine
            | Pattern::BannerBlock
            | Pattern::BannerLine
            | Pattern::AsmLineEnd
            | Pattern::DiagnosticBegin
            | Pattern::IncludeBegin
            | Pattern::PragmaMark
            | Pattern::LineDirectiveBegin
            | Pattern::UndefBegin
            | Pattern::PragmaDirectiveBegin
            | Pattern::ConditionalIfBegin
            | Pattern::ConditionalIfBlockBegin
            | Pattern::EnabledIfBegin
            | Pattern::EnabledIfBlockBegin
            | Pattern::DisabledIfBegin
            | Pattern::DisabledIfBlockBegin
            | Pattern::Stray
            | Pattern::ConditionalElifBegin
            | Pattern::LiveElifBegin
            | Pattern::LiveElifBlockBegin
            | Pattern::LiveElseBegin
            | Pattern::LiveElseBlockBegin
            | Pattern::DisabledElifBegin
            | Pattern::DisabledElifOtherBegin
            | Pattern::DisabledElifOtherBlockBegin
            | Pattern::ElseBranchBegin
            | Pattern::ElseBranchBlockBegin
            | Pattern::ElifBranchBegin
            | Pattern::ElifBranchBlockBegin
            | Pattern::ElifElseBegin
            | Pattern::ElifElseBlockBegin
            | Pattern::ElifElifBegin
            | Pattern::NestBegin
    )
}

/// Whether `p` can match only where no word character comes before it.
/// Each of these patterns first looks at the character before.
fn word_start(p: Pattern) -> bool {
    matches!(
        p,
        Pattern::KeywordControl
            | Pattern::Primitive
            | Pattern::BuiltIn
            | Pattern::EnumStructUnion
            | Pattern::AsmBegin
            | Pattern::Modifier
            | Pattern::MacK
            | Pattern::MacG
            | Pattern::MacS
            | Pattern::ConstLang
            | Pattern::ConditionalConstant
            | Pattern::SysTypes
            | Pattern::Pthread
            | Pattern::Stdint
            | Pattern::MacConst
            | Pattern::MacType
            | Pattern::PosixReserved
            | Pattern::Predefined
            | Pattern::PredefinedProbably
            | Pattern::Number
            | Pattern::FunctionDefinitionBegin
            | Pattern::ConditionalDefinedKeyword
            | Pattern::ConditionalDefinedInvalid
            | Pattern::Sizeof
    )
}

/// Whether `p` reads only white space and comments from its position.
/// Inside a run of white space, such a pattern fails wherever it failed at
/// the first position of the run that the search tried.
fn run_start_only(p: Pattern) -> bool {
    matches!(
        p,
        Pattern::DocBlockBegin
            | Pattern::BlockComment
            | Pattern::LineComment
            | Pattern::LineLeading
            | Pattern::DefineBegin
            | Pattern::InitializationBegin
            | Pattern::SwitchBegin
            | Pattern::SwitchParensBegin
            | Pattern::AsmParensBegin
            | Pattern::CaseBegin
            | Pattern::DefaultBegin
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
fn word_tail_only(p: Pattern) -> bool {
    matches!(
        p,
        Pattern::MethodAccess
            | Pattern::MemberAccess
            | Pattern::PlainAccess
            | Pattern::BracketBegin
    )
}

/// Whether `p` starts only at a name that a `(` follows.
/// The one exception is a list of words it excludes.
/// Those words may match inside a word.
fn call_only(p: Pattern) -> bool {
    matches!(
        p,
        Pattern::FunctionParametersBegin
            | Pattern::CallBegin
            | Pattern::CFunctionCall
            | Pattern::DefineFunctionBegin
            | Pattern::DefineCallBegin
    )
}

/// Whether `p` starts only at a name and then an operator that names an
/// object: after one of `])`, or at a letter.
fn object_only(p: Pattern) -> bool {
    matches!(
        p,
        Pattern::MethodAccess | Pattern::MemberAccess | Pattern::PlainAccess
    )
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
fn allowed(flags: usize, p: Pattern) -> bool {
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
    lists: Vec<Vec<Pattern>>,
}

impl Table {
    /// The table of context `r`, built the first time a line reaches `r`.
    /// A line pays only for the contexts it opens.
    ///
    /// Bytes that start the same patterns share a class, and each `place` has a
    /// column per class.
    /// A class index fits a `u8`, and a list index fits a `u16`.
    /// `tables_fit_their_indices` checks both, so the fallbacks never apply.
    /// It is `None` only for an `r` outside [`Rule::ALL`], and no such `r`
    /// exists.
    pub(super) fn of(r: Rule) -> Option<&'static Table> {
        static TABLES: [OnceLock<Table>; Rule::ALL.len()] =
            [const { OnceLock::new() }; Rule::ALL.len()];
        TABLES
            .get(r.index())
            .map(|table| table.get_or_init(move || Table::build(r)))
    }

    /// Builds the table over sets of the context's patterns.
    /// A set has one bit per pattern, in the order the context tries them.
    /// A class is the set of patterns a byte can start, and a `place` is the
    /// set of patterns it allows.
    /// Only a set not seen before gets a list.
    fn build(r: Rule) -> Table {
        let mut flat = Vec::new();
        for list in r.items() {
            flatten(list, &mut flat);
        }
        let words = flat.len().div_ceil(64).max(1);
        let set = |keep: &dyn Fn(Pattern) -> bool| {
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
        let mut lists: Vec<Vec<Pattern>> = Vec::new();
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

    pub(super) fn candidates(&self, place: usize, index: usize) -> &[Pattern] {
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

fn flatten(list: &[Entry], out: &mut Vec<Pattern>) {
    for item in list {
        match *item {
            Entry::Pattern(p) => out.push(p),
            Entry::Group(group) => flatten(group, out),
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
