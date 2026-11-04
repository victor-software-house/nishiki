//! The grammar's rules as tables: for each rule the scopes it adds, how it
//! ends and which patterns it tries, in the grammar's order.
//!
//! A [`Pat`] is one regular expression of the grammar, written as a matcher in
//! the sibling modules. The pattern lists below flatten the grammar's
//! `include`s, so the first pattern that matches at the earliest position wins
//! exactly as in a `TextMate` engine.

use std::collections::HashMap;
use std::sync::OnceLock;

use super::matchers::{is_command_name, is_name, is_unquoted};
use super::scope::S;
use super::state::Rule;

/// One pattern of the grammar.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) enum Pat {
    // Matches that name text and push nothing.
    Comment,
    PipeTime,
    PipeOp,
    Sep,
    LineCont,
    VarAt,
    VarDigit,
    VarSpecial,
    VarName,
    HerestringWord,
    RedirectOp,
    RedirectNum,
    Tilde,
    Glob,
    FloatKw,
    Support,
    Escape,
    Boolean,
    Numeric,
    Unquoted,
    DqEscape,
    BtEscape,
    CaseDefault,
    CaseStar,
    CasePlus,
    CaseQuestion,
    CaseAt,
    CasePipe,
    CaseParenOpen,
    CaseChar,
    AnsiEscape,
    AnsiOctal,
    AnsiHex,
    AnsiControl,
    MathOp,
    MathHex,
    MathSemi,
    MathOctal,
    MathOther,
    MathInt,
    MathName,
    SpecialExpansion,
    ArrayAccess,
    ParamDigits,
    ParamName,
    ArgMatch,
    WildcardStar,
    BoolArg,
    CmdKeyword,
    CmdBuiltin,
    CmdName,
    ArrayNameEq,
    ArrayBracketEq,
    ModOption,
    ModMatch,
    LogicalOp,
    LogicalTest,
    RegexCompare,
    FuncBlank,
    RedirectFix,
    // Begins of rules.
    AliasB,
    ArgB,
    ArithDoubleB,
    ArithSingleB,
    ArrayValueB,
    CaseB,
    CasePatternB,
    CaseBodyB,
    CaseGroupB,
    CaseClassB,
    CmdStatementB,
    CmdNameB,
    CmdQuotedB,
    ContinuationDoubleB,
    ContinuationSingleB,
    ForInB,
    ForB,
    FunctionB,
    FuncBraceB,
    FuncParenB,
    HeredocQuotedIndentB,
    HeredocQuotedB,
    HeredocIndentB,
    HeredocB,
    HerestringSingleB,
    HerestringDoubleB,
    BacktickB,
    BacktickCommentB,
    BacktickCommentLineB,
    LogicalDoubleB,
    LogicalSingleB,
    LoopForB,
    LoopWhileB,
    LoopSelectB,
    LoopIfB,
    GroupB,
    ModifiedB,
    NormalAssignB,
    EnvCommandB,
    NormalStatementB,
    OptionB,
    ParenB,
    ExtglobB,
    ProcessSubstitutionB,
    StringSingleB,
    StringDoubleB,
    StringAnsiB,
    SubshellB,
    VarPositionalB,
    VarBraceB,
    WhileB,
    // Ends of rules.
    EndStatement,
    EndOption,
    EndArg,
    EndAlias,
    EndArithDouble,
    EndCase,
    EndCasePattern,
    EndCaseBody,
    EndCmdName,
    EndCmdQuotedDouble,
    EndCmdQuotedSingle,
    EndFunction,
    EndHeredoc,
    EndDone,
    EndFi,
    EndBacktickComment,
    EndBacktickCommentLine,
    /// The rule's own literal, `RuleInfo::end_lit`.
    EndLit,
}

/// What a pattern does when it matches.
#[derive(Clone, Copy, Debug)]
pub(crate) enum Act {
    /// A match rule; `name` goes on all of its text.
    Match(&'static [S]),
    /// A begin rule; it opens the rule.
    Begin(Rule),
    /// A begin rule the matcher chooses.
    BeginChosen,
    /// The end of the innermost rule.
    End,
}

impl Pat {
    #[expect(clippy::too_many_lines, reason = "one arm per pattern, a table")]
    pub(crate) fn act(self) -> Act {
        use Act::{Begin, BeginChosen, End, Match};
        match self {
            Pat::HerestringWord => Match(&[S::MetaHerestring]),
            Pat::RedirectOp => Match(&[S::KeywordOperatorRedirect]),
            Pat::AliasB => Begin(Rule::Alias),
            Pat::ArgB => Begin(Rule::Argument),
            Pat::ArithDoubleB => Begin(Rule::ArithDouble),
            Pat::ArithSingleB => Begin(Rule::ArithSingle),
            Pat::ArrayValueB => Begin(Rule::ArrayValue),
            Pat::CaseB => Begin(Rule::Case),
            Pat::CasePatternB => Begin(Rule::CasePattern),
            Pat::CaseBodyB => Begin(Rule::CaseBody),
            Pat::CaseGroupB => Begin(Rule::CaseGroup),
            Pat::CaseClassB => Begin(Rule::CaseClass),
            Pat::CmdStatementB => Begin(Rule::CommandStatement),
            Pat::CmdNameB => Begin(Rule::CommandName),
            Pat::CmdQuotedB => BeginChosen,
            Pat::ContinuationDoubleB => Begin(Rule::ContinuationDouble),
            Pat::ContinuationSingleB => Begin(Rule::ContinuationSingle),
            Pat::ForInB => Begin(Rule::ForIn),
            Pat::ForB => Begin(Rule::For),
            Pat::FunctionB => Begin(Rule::Function),
            Pat::FuncBraceB => Begin(Rule::FunctionBrace),
            Pat::FuncParenB => Begin(Rule::FunctionParen),
            Pat::HeredocQuotedIndentB => Begin(Rule::HeredocQuotedIndent),
            Pat::HeredocQuotedB => Begin(Rule::HeredocQuoted),
            Pat::HeredocIndentB => Begin(Rule::HeredocIndent),
            Pat::HeredocB => Begin(Rule::Heredoc),
            Pat::HerestringSingleB => Begin(Rule::HerestringSingle),
            Pat::HerestringDoubleB => Begin(Rule::HerestringDouble),
            Pat::BacktickB => Begin(Rule::Backtick),
            Pat::BacktickCommentB => Begin(Rule::BacktickComment),
            Pat::BacktickCommentLineB => Begin(Rule::BacktickCommentLine),
            Pat::LogicalDoubleB => Begin(Rule::LogicalDouble),
            Pat::LogicalSingleB => Begin(Rule::LogicalSingle),
            Pat::LoopForB => Begin(Rule::LoopFor),
            Pat::LoopWhileB => Begin(Rule::LoopWhile),
            Pat::LoopSelectB => Begin(Rule::LoopSelect),
            Pat::LoopIfB => Begin(Rule::LoopIf),
            Pat::GroupB => Begin(Rule::Group),
            Pat::ModifiedB => Begin(Rule::Modified),
            Pat::NormalAssignB => Begin(Rule::NormalAssign),
            Pat::EnvCommandB => Begin(Rule::EnvCommand),
            Pat::NormalStatementB => Begin(Rule::NormalStatement),
            Pat::OptionB => Begin(Rule::Option),
            Pat::ParenB => Begin(Rule::Paren),
            Pat::ExtglobB => Begin(Rule::Extglob),
            Pat::ProcessSubstitutionB => Begin(Rule::ProcessSubstitution),
            Pat::StringSingleB => Begin(Rule::StringSingle),
            Pat::StringDoubleB => Begin(Rule::StringDouble),
            Pat::StringAnsiB => Begin(Rule::StringAnsi),
            Pat::SubshellB => Begin(Rule::Subshell),
            Pat::VarPositionalB => Begin(Rule::VarPositional),
            Pat::VarBraceB => Begin(Rule::VarBrace),
            Pat::WhileB => Begin(Rule::While),
            Pat::EndStatement
            | Pat::EndOption
            | Pat::EndArg
            | Pat::EndAlias
            | Pat::EndArithDouble
            | Pat::EndCase
            | Pat::EndCasePattern
            | Pat::EndCaseBody
            | Pat::EndCmdName
            | Pat::EndCmdQuotedDouble
            | Pat::EndCmdQuotedSingle
            | Pat::EndFunction
            | Pat::EndHeredoc
            | Pat::EndDone
            | Pat::EndFi
            | Pat::EndBacktickComment
            | Pat::EndBacktickCommentLine
            | Pat::EndLit => End,
            Pat::Comment
            | Pat::PipeTime
            | Pat::PipeOp
            | Pat::Sep
            | Pat::LineCont
            | Pat::VarAt
            | Pat::VarDigit
            | Pat::VarSpecial
            | Pat::VarName
            | Pat::RedirectNum
            | Pat::Tilde
            | Pat::Glob
            | Pat::FloatKw
            | Pat::Support
            | Pat::Escape
            | Pat::Boolean
            | Pat::Numeric
            | Pat::Unquoted
            | Pat::DqEscape
            | Pat::BtEscape
            | Pat::CaseDefault
            | Pat::CaseStar
            | Pat::CasePlus
            | Pat::CaseQuestion
            | Pat::CaseAt
            | Pat::CasePipe
            | Pat::CaseParenOpen
            | Pat::CaseChar
            | Pat::AnsiEscape
            | Pat::AnsiOctal
            | Pat::AnsiHex
            | Pat::AnsiControl
            | Pat::MathOp
            | Pat::MathHex
            | Pat::MathSemi
            | Pat::MathOctal
            | Pat::MathOther
            | Pat::MathInt
            | Pat::MathName
            | Pat::SpecialExpansion
            | Pat::ArrayAccess
            | Pat::ParamDigits
            | Pat::ParamName
            | Pat::ArgMatch
            | Pat::WildcardStar
            | Pat::BoolArg
            | Pat::CmdKeyword
            | Pat::CmdBuiltin
            | Pat::CmdName
            | Pat::ArrayNameEq
            | Pat::ArrayBracketEq
            | Pat::ModOption
            | Pat::ModMatch
            | Pat::LogicalOp
            | Pat::LogicalTest
            | Pat::RegexCompare
            | Pat::FuncBlank
            | Pat::RedirectFix => Match(&[]),
        }
    }
}

/// Where a pattern can start to match, for skipping the positions where no
/// pattern of a rule can. Each variant over-approximates: a position it
/// excludes cannot match. The engine always tries the first position of a
/// search, the start of the line and the end of the text, so patterns that
/// need `\\G`, `^` or `$` say [`Start::Never`].
#[derive(Clone, Copy)]
pub(crate) enum Start {
    /// The byte at the position is one of these.
    Bytes(&'static [u8]),
    /// The byte at the position is in this class.
    Class(fn(u8) -> bool),
    /// A zero-width match after one of these bytes.
    After(&'static [u8]),
    /// Anywhere: the rule cannot skip positions.
    Any,
    /// Only where [`Pat::anchored`] or [`Pat::at_line_start`] say.
    Never,
}

fn digit(b: u8) -> bool {
    b.is_ascii_digit()
}

fn name_or_blank(b: u8) -> bool {
    b.is_ascii_alphanumeric() || matches!(b, b'_' | b'-' | b' ' | b'\t')
}

fn redirect_start(b: u8) -> bool {
    b.is_ascii_digit() || matches!(b, b'&' | b'<' | b'>')
}

fn number_start(b: u8) -> bool {
    b.is_ascii_digit() || b == b'-'
}

fn blank_or_unquoted(b: u8) -> bool {
    matches!(b, b' ' | b'\t') || is_unquoted(b)
}

fn case_char(b: u8) -> bool {
    !matches!(
        b,
        b'\t' | b'\n' | b' ' | b')' | b'*' | b'?' | b'@' | b'[' | b'|'
    )
}

fn math_operator(b: u8) -> bool {
    matches!(
        b,
        b'+' | b'-'
            | b'!'
            | b'~'
            | b'*'
            | b'%'
            | b'/'
            | b'<'
            | b'>'
            | b'='
            | b','
            | b':'
            | b'?'
            | b'|'
            | b'&'
            | b'^'
    )
}

fn math_name(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

fn builtin_start(b: u8) -> bool {
    b.is_ascii_alphabetic() || matches!(b, b'.' | b':')
}

fn command_statement_start(b: u8) -> bool {
    !matches!(
        b,
        b'\n' | b'!' | b'#' | b'&' | b'(' | b')' | b'<' | b'>' | b'[' | b'{' | b'|' | b';'
    )
}

fn function_start(b: u8) -> bool {
    !matches!(b, b'\n' | b'\r' | b'"' | b'\'' | b'(' | b')' | b'=')
}

impl Pat {
    /// Whether the pattern can match at the first position of a search without
    /// starting at a byte in its [`Start`]: it is `\\G`.
    pub(crate) fn anchored(self) -> bool {
        matches!(
            self,
            Pat::CmdNameB | Pat::ContinuationDoubleB | Pat::ContinuationSingleB | Pat::FuncBlank
        )
    }

    /// Whether the pattern can match at the start of the line without starting
    /// at a byte in its [`Start`]: it is `^`.
    pub(crate) fn at_line_start(self) -> bool {
        matches!(self, Pat::EndHeredoc | Pat::MathOp | Pat::NormalStatementB)
    }

    #[expect(clippy::too_many_lines, reason = "one arm per pattern, a table")]
    pub(crate) fn start(self) -> Start {
        use Start::{After, Any, Bytes, Class, Never};
        match self {
            Pat::Comment => Bytes(b" \t#"),
            Pat::PipeTime | Pat::Boolean | Pat::BoolArg => Bytes(b"tf"),
            Pat::PipeOp => Bytes(b"!|"),
            Pat::Sep => Bytes(b";&|"),
            Pat::LineCont
            | Pat::Escape
            | Pat::DqEscape
            | Pat::BtEscape
            | Pat::AnsiEscape
            | Pat::AnsiOctal
            | Pat::AnsiHex
            | Pat::AnsiControl => Bytes(b"\\"),
            Pat::VarAt
            | Pat::VarDigit
            | Pat::VarSpecial
            | Pat::VarName
            | Pat::StringAnsiB
            | Pat::SubshellB
            | Pat::VarPositionalB
            | Pat::VarBraceB => Bytes(b"$"),
            Pat::HerestringWord
            | Pat::HeredocQuotedIndentB
            | Pat::HeredocQuotedB
            | Pat::HeredocIndentB
            | Pat::HeredocB
            | Pat::HerestringSingleB
            | Pat::HerestringDoubleB => Bytes(b"<"),
            Pat::RedirectOp => Class(redirect_start),
            Pat::RedirectNum | Pat::ParamDigits | Pat::MathOther | Pat::MathInt => Class(digit),
            Pat::Tilde => Bytes(b"~"),
            Pat::Glob => Bytes(b"*?"),
            Pat::FloatKw => Bytes(b"tedif"),
            Pat::Support => Bytes(b".:"),
            Pat::Numeric => Class(number_start),
            Pat::Unquoted => Class(is_unquoted),
            Pat::CaseDefault => Bytes(b" \t*"),
            Pat::CaseStar | Pat::WildcardStar => Bytes(b"*"),
            Pat::CasePlus => Bytes(b"+"),
            Pat::CaseQuestion => Bytes(b"?"),
            Pat::CaseAt => Bytes(b"@"),
            Pat::CasePipe => Bytes(b"|"),
            Pat::CaseParenOpen
            | Pat::CaseGroupB
            | Pat::ArithDoubleB
            | Pat::ArithSingleB
            | Pat::FuncParenB
            | Pat::ParenB => Bytes(b"("),
            Pat::CaseChar => Class(case_char),
            Pat::MathOp => Class(math_operator),
            Pat::MathHex | Pat::MathOctal => Bytes(b"0"),
            Pat::MathSemi => Bytes(b";"),
            Pat::MathName => Class(math_name),
            Pat::SpecialExpansion => Bytes(b"!:*@#%/"),
            Pat::ArrayAccess
            | Pat::ArrayBracketEq
            | Pat::CaseClassB
            | Pat::LogicalDoubleB
            | Pat::LogicalSingleB => Bytes(b"["),
            Pat::ParamName | Pat::ArrayNameEq | Pat::ModMatch => Class(is_name),
            Pat::ArgMatch => Class(blank_or_unquoted),
            Pat::CmdKeyword => Bytes(b"crb"),
            Pat::CmdBuiltin => Class(builtin_start),
            Pat::CmdName => Class(is_command_name),
            Pat::ModOption | Pat::LogicalTest => Bytes(b"-"),
            Pat::LogicalOp => Bytes(b"=!<>&|"),
            Pat::RegexCompare => Bytes(b"="),
            Pat::RedirectFix => Bytes(b">"),
            Pat::AliasB => Bytes(b" \ta"),
            Pat::ArgB | Pat::OptionB => Bytes(b" \t"),
            Pat::ArrayValueB | Pat::NormalAssignB => Class(name_or_blank),
            Pat::ModifiedB => Bytes(b"rdtel"),
            Pat::CaseB => Bytes(b"c"),
            Pat::CasePatternB | Pat::EndBacktickComment => Any,
            Pat::CaseBodyB => After(b")"),
            Pat::CmdStatementB => Class(command_statement_start),
            Pat::CmdNameB
            | Pat::ContinuationDoubleB
            | Pat::ContinuationSingleB
            | Pat::FuncBlank
            | Pat::EndHeredoc
            | Pat::EndLit => Never,
            Pat::CmdQuotedB => Bytes(b"$\"'"),
            Pat::ForInB | Pat::ForB | Pat::LoopForB | Pat::EndFi => Bytes(b"f"),
            Pat::FunctionB => Class(function_start),
            Pat::FuncBraceB | Pat::GroupB => Bytes(b"{"),
            Pat::BacktickB | Pat::EndBacktickCommentLine => Bytes(b"`"),
            Pat::BacktickCommentB | Pat::BacktickCommentLineB => Bytes(b"#"),
            Pat::LoopWhileB | Pat::WhileB => Bytes(b"wu"),
            Pat::LoopSelectB => Bytes(b"s"),
            Pat::LoopIfB => Bytes(b"i"),
            Pat::EnvCommandB => After(b" \t"),
            Pat::NormalStatementB => After(b"!&(;`{| "),
            Pat::ExtglobB => Bytes(b"!*+?@"),
            Pat::ProcessSubstitutionB => Bytes(b"<>"),
            Pat::StringSingleB => Bytes(b"'"),
            Pat::StringDoubleB => Bytes(b"$\""),
            Pat::EndStatement | Pat::EndOption => Bytes(b"\n&);`{|}] \t#"),
            Pat::EndArg => Bytes(b" \t&;|\n)`"),
            Pat::EndAlias => Bytes(b" \t;&|\n"),
            Pat::EndArithDouble => Bytes(b")"),
            Pat::EndCase => Bytes(b"e"),
            Pat::EndCasePattern => Bytes(b"e)"),
            Pat::EndCaseBody => Bytes(b";e"),
            Pat::EndCmdName => Bytes(b" \t&;|\n)`<"),
            Pat::EndCmdQuotedDouble => After(b"\""),
            Pat::EndCmdQuotedSingle => After(b"'"),
            Pat::EndFunction => After(b")}"),
            Pat::EndDone => Bytes(b"d"),
        }
    }
}

/// What a rule adds and tries.
#[derive(Debug)]
pub(crate) struct RuleInfo {
    /// Scopes of everything in the rule, begin and end included.
    pub(crate) name: &'static [S],
    /// Scopes of what lies between begin and end.
    pub(crate) content: &'static [S],
    /// Prefix of a content scope that ends in the here-document delimiter.
    pub(crate) content_delimiter: Option<&'static str>,
    pub(crate) end: Option<Pat>,
    /// Whether the end pattern comes after the other patterns.
    pub(crate) end_last: bool,
    /// For [`Pat::EndLit`]: the text that ends the rule.
    pub(crate) end_lit: &'static [u8],
    /// For [`Pat::EndLit`]: the scopes of that text.
    pub(crate) end_scopes: &'static [S],
    /// The end pattern and the patterns, in the order they are tried. A
    /// rule's builder leaves the patterns alone here, and [`with_tables`]
    /// places the end among them.
    pub(crate) order: Box<[Pat]>,
    /// For the byte before a position: which zero-width patterns can match
    /// there, as a slot of [`RuleInfo::candidates`].
    after: [u8; 256],
    /// Where the candidates for a position are in `arena`, by [`Self::slot`].
    index: Vec<(usize, usize)>,
    /// The candidate lists, each in the order the patterns are tried.
    arena: Vec<Pat>,
    /// Whether the positions inside a run of blanks that no pattern matched
    /// at the start of can be skipped: no pattern matches anywhere.
    pub(crate) skip_blanks: bool,
}

impl RuleInfo {
    /// The patterns that can match at a position with `byte` and `prev` at and
    /// before it, in order. `first` is whether it is the first position of a
    /// search, and `bol` whether it starts the line.
    pub(crate) fn candidates(&self, first: bool, bol: bool, prev: u8, byte: u8) -> &[Pat] {
        let variant = usize::from(first) | (usize::from(bol) << 1);
        let slot = usize::from(self.after[usize::from(prev)]);
        let (start, len) = self.index[(variant * self.slots() + slot) * 256 + usize::from(byte)];
        &self.arena[start..start + len]
    }

    fn slots(&self) -> usize {
        self.index.len() / (4 * 256)
    }
}

type Group = &'static [Pat];

const COMMENT: Group = &[Pat::Comment];
const PIPELINE: Group = &[Pat::PipeTime, Pat::PipeOp];
const SEP: Group = &[Pat::Sep];
const LOGICAL_DOUBLE: Group = &[Pat::LogicalDoubleB];
const LOGICAL_SINGLE: Group = &[Pat::LogicalSingleB];
const ASSIGNMENT: Group = &[Pat::ArrayValueB, Pat::ModifiedB, Pat::NormalAssignB];
const CASE: Group = &[Pat::CaseB];
const FOR: Group = &[Pat::ForInB, Pat::ForB];
const LOOP: Group = &[
    Pat::LoopForB,
    Pat::LoopWhileB,
    Pat::LoopSelectB,
    Pat::LoopIfB,
];
const FUNCTION: Group = &[Pat::FunctionB];
const LINE_CONTINUATION: Group = &[Pat::LineCont];
const ARITH_DOUBLE: Group = &[Pat::ArithDoubleB];
const MISC_RANGES: Group = &[
    Pat::LogicalSingleB,
    Pat::LogicalDoubleB,
    Pat::SubshellB,
    Pat::GroupB,
];
const VARIABLE: Group = &[
    Pat::VarAt,
    Pat::VarDigit,
    Pat::VarSpecial,
    Pat::VarPositionalB,
    Pat::VarBraceB,
    Pat::VarName,
];
const INTERPOLATION: Group = &[Pat::SubshellB, Pat::BacktickB];
const HEREDOC: Group = &[
    Pat::HeredocQuotedIndentB,
    Pat::HeredocQuotedB,
    Pat::HeredocIndentB,
    Pat::HeredocB,
];
const HERESTRING: Group = &[
    Pat::HerestringSingleB,
    Pat::HerestringDoubleB,
    Pat::HerestringWord,
];
const REDIRECTION: Group = &[Pat::ProcessSubstitutionB, Pat::RedirectOp];
const PATHNAME: Group = &[Pat::Tilde, Pat::Glob, Pat::ExtglobB];
const FLOATING_KEYWORD: Group = &[Pat::FloatKw];
const ALIAS: Group = &[Pat::AliasB];
const NORMAL_STATEMENT: Group = &[Pat::NormalStatementB];
const STRING: Group = &[
    Pat::Escape,
    Pat::StringSingleB,
    Pat::StringDoubleB,
    Pat::StringAnsiB,
];
const SUPPORT: Group = &[Pat::Support];
const BOOLEAN: Group = &[Pat::Boolean];
const REDIRECT_NUMBER: Group = &[Pat::RedirectNum];
const NUMERIC: Group = &[Pat::Numeric];
const PAREN: Group = &[Pat::ParenB];
const UNQUOTED: Group = &[Pat::Unquoted];
const DQ_ESCAPE: Group = &[Pat::DqEscape];
const ARGUMENT_MATCH: Group = &[Pat::ArgMatch];
const MATH: Group = &[
    Pat::MathOp,
    Pat::MathHex,
    Pat::MathSemi,
    Pat::MathOctal,
    Pat::MathOther,
    Pat::MathInt,
    Pat::MathName,
];
const MATH_OPERATORS: Group = &[
    Pat::MathOp,
    Pat::MathHex,
    Pat::MathOctal,
    Pat::MathOther,
    Pat::MathInt,
];
const CASE_CONTEXT: Group = &[
    Pat::CaseStar,
    Pat::CasePlus,
    Pat::CaseQuestion,
    Pat::CaseAt,
    Pat::CasePipe,
    Pat::Escape,
    Pat::CaseParenOpen,
    Pat::CaseGroupB,
    Pat::CaseClassB,
];
const OPTION_B: Group = &[Pat::OptionB];
const ARG_B: Group = &[Pat::ArgB];

const NORMAL_CONTEXT: &[Group] = &[
    COMMENT,
    PIPELINE,
    SEP,
    MISC_RANGES,
    BOOLEAN,
    REDIRECT_NUMBER,
    NUMERIC,
    STRING,
    VARIABLE,
    INTERPOLATION,
    HEREDOC,
    HERESTRING,
    REDIRECTION,
    PATHNAME,
    FLOATING_KEYWORD,
    SUPPORT,
    PAREN,
];

const INITIAL_CONTEXT: &[Group] = &[
    COMMENT,
    PIPELINE,
    SEP,
    LOGICAL_DOUBLE,
    LOGICAL_SINGLE,
    ASSIGNMENT,
    CASE,
    FOR,
    LOOP,
    FUNCTION,
    LINE_CONTINUATION,
    ARITH_DOUBLE,
    MISC_RANGES,
    VARIABLE,
    INTERPOLATION,
    HEREDOC,
    HERESTRING,
    REDIRECTION,
    PATHNAME,
    FLOATING_KEYWORD,
    ALIAS,
    NORMAL_STATEMENT,
    STRING,
    SUPPORT,
];

const TYPICAL_STATEMENTS: &[Group] = &[
    ASSIGNMENT,
    CASE,
    FOR,
    &[Pat::WhileB],
    FUNCTION,
    &[Pat::CmdStatementB],
    LINE_CONTINUATION,
    ARITH_DOUBLE,
    // `normal_context`
    COMMENT,
    PIPELINE,
    SEP,
    MISC_RANGES,
    BOOLEAN,
    REDIRECT_NUMBER,
    NUMERIC,
    STRING,
    VARIABLE,
    INTERPOLATION,
    HEREDOC,
    HERESTRING,
    REDIRECTION,
    PATHNAME,
    FLOATING_KEYWORD,
    SUPPORT,
    PAREN,
];

const LOGICAL_CONTEXT: &[Group] = &[
    &[Pat::RegexCompare],
    &[Pat::ArithSingleB],
    &[Pat::ArithSingleB, Pat::LogicalOp, Pat::LogicalTest],
    LOGICAL_SINGLE,
    LOGICAL_DOUBLE,
    COMMENT,
    BOOLEAN,
    REDIRECT_NUMBER,
    NUMERIC,
    PIPELINE,
    SEP,
    STRING,
    VARIABLE,
    INTERPOLATION,
    HEREDOC,
    HERESTRING,
    PATHNAME,
    FLOATING_KEYWORD,
    SUPPORT,
];

const OPTION_CONTEXT: &[Group] = &[
    MISC_RANGES,
    STRING,
    VARIABLE,
    INTERPOLATION,
    HEREDOC,
    HERESTRING,
    REDIRECTION,
    PATHNAME,
    FLOATING_KEYWORD,
    SUPPORT,
];

fn flatten(groups: &[&[&[Pat]]]) -> Box<[Pat]> {
    groups
        .iter()
        .flat_map(|list| list.iter())
        .flat_map(|group| group.iter().copied())
        .collect()
}

fn info(
    name: &'static [S],
    content: &'static [S],
    end: Option<Pat>,
    groups: &[&[&[Pat]]],
) -> RuleInfo {
    RuleInfo {
        name,
        content,
        content_delimiter: None,
        end,
        end_last: false,
        end_lit: b"",
        end_scopes: &[],
        order: flatten(groups),
        after: [0; 256],
        index: Vec::new(),
        arena: Vec::new(),
        skip_blanks: true,
    }
}

/// A rule that ends at the literal `lit`, whose scopes are `scopes`.
fn lit(
    name: &'static [S],
    content: &'static [S],
    lit: &'static [u8],
    scopes: &'static [S],
    groups: &[&[&[Pat]]],
) -> RuleInfo {
    RuleInfo {
        end_lit: lit,
        end_scopes: scopes,
        ..info(name, content, Some(Pat::EndLit), groups)
    }
}

fn heredoc(prefix: &'static str, groups: &[&[&[Pat]]]) -> RuleInfo {
    RuleInfo {
        content_delimiter: Some(prefix),
        ..info(&[], &[], Some(Pat::EndHeredoc), groups)
    }
}

#[expect(clippy::too_many_lines, reason = "one arm per rule, a table")]
fn build(rule: Rule) -> RuleInfo {
    const NORMAL: &[&[Pat]] = NORMAL_CONTEXT;
    const INITIAL: &[&[Pat]] = INITIAL_CONTEXT;
    const TYPICAL: &[&[Pat]] = TYPICAL_STATEMENTS;
    const LOGICAL: &[&[Pat]] = LOGICAL_CONTEXT;
    const OPTION: &[&[Pat]] = OPTION_CONTEXT;
    match rule {
        Rule::Root | Rule::RetokInitial => info(&[], &[], None, &[INITIAL]),
        Rule::Alias => info(
            &[S::MetaExpressionAssignmentAlias],
            &[],
            Some(Pat::EndAlias),
            &[NORMAL],
        ),
        Rule::Argument => info(
            &[S::MetaArgument],
            &[],
            Some(Pat::EndArg),
            &[&[ARGUMENT_MATCH], NORMAL, &[LINE_CONTINUATION]],
        ),
        Rule::ArithDouble => info(
            &[S::MetaArithmetic],
            &[],
            Some(Pat::EndArithDouble),
            &[&[VARIABLE, MATH, STRING]],
        ),
        Rule::ArithSingle => lit(
            &[S::MetaArithmetic],
            &[],
            b")",
            &[S::PunctuationSectionArithmeticSingle],
            &[&[VARIABLE, MATH, STRING]],
        ),
        Rule::ArrayValue => lit(
            &[],
            &[],
            b")",
            &[S::PunctuationDefinitionArray],
            &[
                &[COMMENT, &[Pat::ArrayNameEq, Pat::ArrayBracketEq]],
                NORMAL,
                &[UNQUOTED],
            ],
        ),
        Rule::Case => info(
            &[S::MetaCase],
            &[],
            Some(Pat::EndCase),
            &[&[
                COMMENT,
                &[Pat::CaseDefault, Pat::CasePatternB, Pat::CaseBodyB],
            ]],
        ),
        Rule::CasePattern => info(
            &[S::MetaCaseEntryPattern],
            &[],
            Some(Pat::EndCasePattern),
            &[&[CASE_CONTEXT, STRING, &[Pat::CaseChar]]],
        ),
        Rule::CaseBody => info(
            &[S::MetaCaseEntryBody],
            &[],
            Some(Pat::EndCaseBody),
            &[TYPICAL, INITIAL],
        ),
        Rule::CaseGroup => lit(
            &[S::MetaParen],
            &[],
            b")",
            &[
                S::PunctuationDefinitionGroup,
                S::PunctuationDefinitionRegexGroup,
            ],
            &[&[CASE_CONTEXT, STRING, &[Pat::CaseChar]]],
        ),
        Rule::CaseClass => lit(
            &[S::StringRegexpCharacterClass],
            &[],
            b"]",
            &[S::PunctuationDefinitionCharacterClass],
            &[&[&[Pat::Escape]]],
        ),
        Rule::CommandStatement => info(
            &[S::MetaStatementCommand],
            &[],
            Some(Pat::EndStatement),
            &[&[
                &[Pat::CmdNameB],
                LINE_CONTINUATION,
                OPTION_B,
                ARG_B,
                STRING,
                HEREDOC,
            ]],
        ),
        Rule::CommandName => info(
            &[S::MetaStatementCommandName],
            &[],
            Some(Pat::EndCmdName),
            &[&[
                &[Pat::CmdKeyword, Pat::CmdBuiltin],
                VARIABLE,
                &[Pat::CmdName, Pat::CmdQuotedB],
                LINE_CONTINUATION,
                UNQUOTED,
            ]],
        ),
        Rule::CommandQuotedDouble => info(
            &[],
            &[],
            Some(Pat::EndCmdQuotedDouble),
            &[&[&[Pat::ContinuationSingleB, Pat::ContinuationDoubleB]]],
        ),
        Rule::CommandQuotedSingle => info(
            &[],
            &[],
            Some(Pat::EndCmdQuotedSingle),
            &[&[&[Pat::ContinuationSingleB, Pat::ContinuationDoubleB]]],
        ),
        Rule::ContinuationDouble => lit(
            &[],
            &[
                S::MetaStatementCommandNameContinuation,
                S::StringQuotedDoubleBare,
                S::EntityNameFunctionCallBare,
                S::EntityNameCommandBare,
            ],
            b"\"",
            &[
                S::StringQuotedDouble,
                S::PunctuationDefinitionStringEnd,
                S::EntityNameFunctionCall,
                S::EntityNameCommand,
            ],
            &[&[DQ_ESCAPE, VARIABLE, INTERPOLATION]],
        ),
        Rule::ContinuationSingle => lit(
            &[],
            &[
                S::MetaStatementCommandNameContinuation,
                S::StringQuotedSingleBare,
                S::EntityNameFunctionCallBare,
                S::EntityNameCommandBare,
            ],
            b"'",
            &[
                S::StringQuotedSingle,
                S::PunctuationDefinitionStringEnd,
                S::EntityNameFunctionCall,
                S::EntityNameCommand,
            ],
            &[],
        ),
        Rule::ForIn => info(
            &[S::MetaForIn],
            &[],
            Some(Pat::EndStatement),
            &[&[STRING, UNQUOTED], NORMAL],
        ),
        Rule::For => info(
            &[S::MetaFor],
            &[],
            Some(Pat::EndStatement),
            &[&[ARITH_DOUBLE], NORMAL],
        ),
        Rule::Function => RuleInfo {
            end_last: true,
            ..info(
                &[S::MetaFunction],
                &[],
                Some(Pat::EndFunction),
                &[
                    &[&[Pat::FuncBlank, Pat::FuncBraceB, Pat::FuncParenB]],
                    INITIAL,
                ],
            )
        },
        Rule::FunctionBrace => lit(
            &[S::MetaFunctionBody],
            &[],
            b"}",
            &[
                S::PunctuationDefinitionGroup,
                S::PunctuationSectionFunctionDefinition,
            ],
            &[INITIAL],
        ),
        Rule::FunctionParen => lit(
            &[S::MetaFunctionBody],
            &[],
            b")",
            &[
                S::PunctuationDefinitionGroup,
                S::PunctuationSectionFunctionDefinition,
            ],
            &[INITIAL],
        ),
        Rule::HeredocQuotedIndent => heredoc("string.quoted.heredoc.indent.", &[]),
        Rule::HeredocQuoted => heredoc("string.quoted.heredoc.no-indent.", &[]),
        Rule::HeredocIndent => heredoc(
            "string.unquoted.heredoc.indent.",
            &[&[DQ_ESCAPE, VARIABLE, INTERPOLATION]],
        ),
        Rule::Heredoc => heredoc(
            "string.unquoted.heredoc.no-indent.",
            &[&[DQ_ESCAPE, VARIABLE, INTERPOLATION]],
        ),
        Rule::HerestringSingle => lit(
            &[S::MetaHerestring],
            &[S::StringQuotedSingle],
            b"'",
            &[S::StringQuotedSingle, S::PunctuationDefinitionStringEnd],
            &[],
        ),
        Rule::HerestringDouble => lit(
            &[S::MetaHerestring],
            &[S::StringQuotedDouble],
            b"\"",
            &[S::StringQuotedDouble, S::PunctuationDefinitionStringEnd],
            &[&[DQ_ESCAPE, VARIABLE, INTERPOLATION]],
        ),
        Rule::Backtick => lit(
            &[S::StringInterpolatedBacktick],
            &[],
            b"`",
            &[S::PunctuationDefinitionEvaluationBackticks],
            &[&[&[Pat::BtEscape, Pat::BacktickCommentB]], INITIAL],
        ),
        Rule::BacktickComment => info(
            &[],
            &[],
            Some(Pat::EndBacktickComment),
            &[&[&[Pat::BacktickCommentLineB]]],
        ),
        Rule::BacktickCommentLine => info(
            &[S::CommentLineNumberSign],
            &[],
            Some(Pat::EndBacktickCommentLine),
            &[],
        ),
        Rule::LogicalDouble => lit(
            &[S::MetaScopeLogicalExpression],
            &[],
            b"]]",
            &[S::PunctuationDefinitionLogicalExpression],
            &[LOGICAL],
        ),
        Rule::LogicalSingle => lit(
            &[S::MetaScopeLogicalExpression],
            &[],
            b"]",
            &[S::PunctuationDefinitionLogicalExpression],
            &[LOGICAL],
        ),
        Rule::LoopFor => info(
            &[S::MetaScopeForInLoop],
            &[],
            Some(Pat::EndDone),
            &[INITIAL],
        ),
        Rule::LoopWhile => info(
            &[S::MetaScopeWhileLoop],
            &[],
            Some(Pat::EndDone),
            &[INITIAL],
        ),
        Rule::LoopSelect => info(
            &[S::MetaScopeSelectBlock],
            &[],
            Some(Pat::EndDone),
            &[INITIAL],
        ),
        Rule::LoopIf => info(&[S::MetaScopeIfBlock], &[], Some(Pat::EndFi), &[INITIAL]),
        Rule::Group => lit(
            &[S::MetaScopeGroup],
            &[],
            b"}",
            &[S::PunctuationDefinitionGroup],
            &[INITIAL],
        ),
        Rule::Modified => info(
            &[S::MetaStatement, S::MetaExpressionAssignmentModified],
            &[],
            Some(Pat::EndStatement),
            &[
                &[&[Pat::ModOption, Pat::ArrayValueB, Pat::ModMatch]],
                NORMAL,
            ],
        ),
        Rule::NormalAssign => info(
            &[S::MetaExpressionAssignment],
            &[],
            Some(Pat::EndStatement),
            &[
                &[
                    COMMENT,
                    STRING,
                    &[Pat::NormalAssignB, Pat::EnvCommandB],
                    UNQUOTED,
                ],
                NORMAL,
            ],
        ),
        Rule::EnvCommand => info(
            &[S::MetaStatementCommandEnv],
            &[],
            Some(Pat::EndStatement),
            &[&[&[Pat::CmdNameB], LINE_CONTINUATION, OPTION_B, ARG_B, STRING]],
        ),
        Rule::NormalStatement => info(
            &[S::MetaStatement],
            &[],
            Some(Pat::EndStatement),
            &[TYPICAL],
        ),
        Rule::Option => info(
            &[],
            &[S::StringUnquotedArgumentBare, S::ConstantOtherOptionBare],
            Some(Pat::EndOption),
            &[OPTION],
        ),
        Rule::Paren => lit(
            &[S::MetaParenGroup],
            &[],
            b")",
            &[S::PunctuationSectionParen],
            &[INITIAL],
        ),
        Rule::Extglob => lit(
            &[S::MetaStructureExtglob],
            &[],
            b")",
            &[S::PunctuationDefinitionExtglob],
            &[INITIAL],
        ),
        Rule::ProcessSubstitution => lit(
            &[S::StringInterpolatedProcessSubstitution],
            &[],
            b")",
            &[S::PunctuationDefinitionStringEnd],
            &[INITIAL],
        ),
        Rule::StringSingle => lit(
            &[S::StringQuotedSingle],
            &[],
            b"'",
            &[S::PunctuationDefinitionStringEnd],
            &[],
        ),
        Rule::StringDouble => lit(
            &[S::StringQuotedDouble],
            &[],
            b"\"",
            &[S::PunctuationDefinitionStringEnd],
            &[&[DQ_ESCAPE, VARIABLE, INTERPOLATION]],
        ),
        Rule::StringAnsi => lit(
            &[S::StringQuotedSingleDollar],
            &[],
            b"'",
            &[S::PunctuationDefinitionStringEnd],
            &[&[&[
                Pat::AnsiEscape,
                Pat::AnsiOctal,
                Pat::AnsiHex,
                Pat::AnsiControl,
            ]]],
        ),
        Rule::Subshell => lit(
            &[S::MetaScopeSubshell],
            &[],
            b")",
            &[S::PunctuationDefinitionSubshellSingle],
            &[&[PAREN], INITIAL],
        ),
        Rule::VarPositional => lit(
            &[],
            &[S::MetaParameterExpansion],
            b"}",
            &[
                S::PunctuationSectionBracketCurlyVariableEnd,
                S::PunctuationDefinitionVariable,
                S::VariableParameterPositional,
            ],
            &[&[
                &[
                    Pat::SpecialExpansion,
                    Pat::ArrayAccess,
                    Pat::ParamDigits,
                    Pat::ParamName,
                ],
                VARIABLE,
                STRING,
            ]],
        ),
        Rule::VarBrace => lit(
            &[],
            &[S::MetaParameterExpansion],
            b"}",
            &[
                S::PunctuationSectionBracketCurlyVariableEnd,
                S::PunctuationDefinitionVariable,
            ],
            &[&[
                &[Pat::SpecialExpansion, Pat::ArrayAccess, Pat::ParamName],
                VARIABLE,
                STRING,
            ]],
        ),
        Rule::While => info(
            &[S::MetaWhile],
            &[],
            Some(Pat::EndStatement),
            &[
                &[LINE_CONTINUATION, MATH_OPERATORS, OPTION_B, UNQUOTED],
                NORMAL,
                &[STRING],
            ],
        ),
        Rule::RetokArgument => info(
            &[],
            &[],
            None,
            &[&[&[Pat::WildcardStar], VARIABLE, NUMERIC, &[Pat::BoolArg]]],
        ),
        Rule::RetokOptions => info(&[], &[], None, &[&[&[Pat::ModOption]]]),
        Rule::RetokString => info(&[], &[], None, &[&[STRING]]),
        Rule::RetokArrayAccess => info(
            &[],
            &[],
            None,
            &[&[&[Pat::SpecialExpansion], STRING, VARIABLE]],
        ),
        Rule::RetokHeredocRest => info(&[], &[], None, &[&[&[Pat::RedirectFix]], TYPICAL]),
    }
}

/// Fills the candidate lists of `info`: for each byte at a position and each
/// byte before it, the patterns that can match there. Positions whose list is
/// empty are skipped without trying anything.
/// Builds the candidate tables of `info`.
///
/// The zero-width patterns that depend on the byte before are listed in
/// order, and each byte before a position activates a set of them, a bit
/// each: at most eight, and at most 256 distinct sets, which
/// `every_rule_fits_its_tables` checks, so the fallbacks never apply. What
/// each pattern needs is computed once, so the lists only look it up.
fn with_tables(mut info: RuleInfo) -> RuleInfo {
    let mut order: Vec<Pat> = Vec::new();
    if let Some(end) = info.end
        && !info.end_last
    {
        order.push(end);
    }
    order.extend(info.order.iter().copied());
    if let Some(end) = info.end
        && info.end_last
    {
        order.push(end);
    }
    let mut zero_width: Vec<(Pat, &'static [u8])> = Vec::new();
    for &pat in &order {
        if let Start::After(set) = pat.start() {
            zero_width.push((pat, set));
        }
    }
    let mut masks: Vec<u8> = vec![0];
    for b in 0..=u8::MAX {
        let mut mask = 0_u8;
        for (index, (_, set)) in zero_width.iter().enumerate() {
            if set.contains(&b) {
                mask |= 1 << index.min(7);
            }
        }
        let slot = if let Some(slot) = masks.iter().position(|&m| m == mask) {
            slot
        } else {
            masks.push(mask);
            masks.len() - 1
        };
        info.after[usize::from(b)] = u8::try_from(slot).unwrap_or_default();
    }
    let classes: Vec<([bool; 256], Option<usize>)> = order
        .iter()
        .map(|&pat| {
            let mut class = [false; 256];
            let mut zero = None;
            match pat.start() {
                Start::Bytes(set) => {
                    for &b in set {
                        class[usize::from(b)] = true;
                    }
                }
                Start::Class(test) => {
                    for b in 0..=u8::MAX {
                        class[usize::from(b)] = test(b);
                    }
                }
                Start::After(_) => zero = zero_width.iter().position(|&(p, _)| p == pat),
                Start::Any => class = [true; 256],
                Start::Never => {
                    if let Some(&b) = info.end_lit.first().filter(|_| pat == Pat::EndLit) {
                        class[usize::from(b)] = true;
                    }
                }
            }
            (class, zero)
        })
        .collect();
    let mut known: HashMap<Vec<Pat>, (usize, usize)> = HashMap::new();
    for variant in 0..4_usize {
        let (first, bol) = (variant & 1 != 0, variant & 2 != 0);
        for &mask in &masks {
            for b in 0..=u8::MAX {
                let list: Vec<Pat> = order
                    .iter()
                    .zip(&classes)
                    .filter(|&(&pat, &(class, zero))| {
                        class[usize::from(b)]
                            || zero.is_some_and(|index| mask & (1 << index.min(7)) != 0)
                            || (first && pat.anchored())
                            || (bol && pat.at_line_start())
                    })
                    .map(|(&pat, _)| pat)
                    .collect();
                let place = if let Some(&place) = known.get(&list) {
                    place
                } else {
                    let place = (info.arena.len(), list.len());
                    info.arena.extend_from_slice(&list);
                    known.insert(list, place);
                    place
                };
                info.index.push(place);
            }
        }
    }
    info.skip_blanks = !order.iter().any(|pat| matches!(pat.start(), Start::Any));
    info.order = order.into_boxed_slice();
    info
}

/// Every rule's table, indexed by [`Rule::index`], built when first needed.
static TABLE: [OnceLock<RuleInfo>; Rule::ALL.len()] = [const { OnceLock::new() }; Rule::ALL.len()];

/// The table of `rule`.
pub(crate) fn rule_info(rule: Rule) -> &'static RuleInfo {
    TABLE[rule.index()].get_or_init(move || with_tables(build(rule)))
}

#[cfg(test)]
mod tests {
    use super::{Start, rule_info};
    use crate::shell::state::Rule;

    #[test]
    fn every_rule_fits_its_tables() {
        for &rule in Rule::ALL {
            let info = rule_info(rule);
            let zero_width = info
                .order
                .iter()
                .filter(|pat| matches!(pat.start(), Start::After(_)))
                .count();
            assert!(
                zero_width <= 8,
                "{rule:?}: {zero_width} zero-width patterns"
            );
            assert!(info.slots() <= 256, "{rule:?}: {} sets", info.slots());
        }
    }
}
