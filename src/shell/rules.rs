//! The grammar's rules as tables.
//! For each rule they hold the scopes it adds, how it ends and which patterns
//! it tries, in the grammar's order.
//!
//! A [`Pattern`] is one regular expression of the grammar, written as a matcher
//! in the sibling modules.
//! The pattern lists below flatten the grammar's `include`s.
//! The first pattern that matches at the earliest position therefore wins
//! exactly as in a `TextMate` engine.

use std::collections::HashMap;
use std::sync::OnceLock;

use super::matchers::{is_command_name, is_name, is_unquoted};
use super::scope::S;
use super::state::Rule;
use crate::scope::{Scopes, scopes};

/// One pattern of the grammar.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[expect(
    clippy::enum_variant_names,
    reason = "the end of a case item's pattern is EndCasePattern"
)]
pub(crate) enum Pattern {
    // Matches that name text and push nothing.
    Comment,
    PipeTime,
    PipeOp,
    Sep,
    LineCont,
    VariableAt,
    VariableDigit,
    VariableSpecial,
    VariableName,
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
    ArithmeticDoubleB,
    ArithmeticSingleB,
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
    VariablePositionalB,
    VariableBraceB,
    WhileB,
    // Ends of rules.
    EndStatement,
    EndOption,
    EndArg,
    EndAlias,
    EndArithmeticDouble,
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
    /// The rule's own literal, `RuleDefinition::end_lit`.
    EndLit,
}

/// What a pattern does when it matches.
#[derive(Clone, Copy, Debug)]
pub(crate) enum Act {
    /// A match rule.
    /// `name` goes on all of its text.
    Match(Scopes<S>),
    /// A begin rule.
    /// It opens the rule.
    Begin(Rule),
    /// A begin rule the matcher chooses.
    BeginChosen,
    /// The end of the innermost rule.
    End,
}

impl Pattern {
    #[expect(clippy::too_many_lines, reason = "one arm per pattern, a table")]
    pub(crate) fn act(self) -> Act {
        use Act::{Begin, BeginChosen, End, Match};
        match self {
            Pattern::HerestringWord => Match(scopes![S::MetaHerestring]),
            Pattern::RedirectOp => Match(scopes![S::KeywordOperatorRedirect]),
            Pattern::AliasB => Begin(Rule::Alias),
            Pattern::ArgB => Begin(Rule::Argument),
            Pattern::ArithmeticDoubleB => Begin(Rule::ArithmeticDouble),
            Pattern::ArithmeticSingleB => Begin(Rule::ArithmeticSingle),
            Pattern::ArrayValueB => Begin(Rule::ArrayValue),
            Pattern::CaseB => Begin(Rule::Case),
            Pattern::CasePatternB => Begin(Rule::CasePattern),
            Pattern::CaseBodyB => Begin(Rule::CaseBody),
            Pattern::CaseGroupB => Begin(Rule::CaseGroup),
            Pattern::CaseClassB => Begin(Rule::CaseClass),
            Pattern::CmdStatementB => Begin(Rule::CommandStatement),
            Pattern::CmdNameB => Begin(Rule::CommandName),
            Pattern::CmdQuotedB => BeginChosen,
            Pattern::ContinuationDoubleB => Begin(Rule::ContinuationDouble),
            Pattern::ContinuationSingleB => Begin(Rule::ContinuationSingle),
            Pattern::ForInB => Begin(Rule::ForIn),
            Pattern::ForB => Begin(Rule::For),
            Pattern::FunctionB => Begin(Rule::Function),
            Pattern::FuncBraceB => Begin(Rule::FunctionBrace),
            Pattern::FuncParenB => Begin(Rule::FunctionParen),
            Pattern::HeredocQuotedIndentB => Begin(Rule::HeredocQuotedIndent),
            Pattern::HeredocQuotedB => Begin(Rule::HeredocQuoted),
            Pattern::HeredocIndentB => Begin(Rule::HeredocIndent),
            Pattern::HeredocB => Begin(Rule::Heredoc),
            Pattern::HerestringSingleB => Begin(Rule::HerestringSingle),
            Pattern::HerestringDoubleB => Begin(Rule::HerestringDouble),
            Pattern::BacktickB => Begin(Rule::Backtick),
            Pattern::BacktickCommentB => Begin(Rule::BacktickComment),
            Pattern::BacktickCommentLineB => Begin(Rule::BacktickCommentLine),
            Pattern::LogicalDoubleB => Begin(Rule::LogicalDouble),
            Pattern::LogicalSingleB => Begin(Rule::LogicalSingle),
            Pattern::LoopForB => Begin(Rule::LoopFor),
            Pattern::LoopWhileB => Begin(Rule::LoopWhile),
            Pattern::LoopSelectB => Begin(Rule::LoopSelect),
            Pattern::LoopIfB => Begin(Rule::LoopIf),
            Pattern::GroupB => Begin(Rule::Group),
            Pattern::ModifiedB => Begin(Rule::Modified),
            Pattern::NormalAssignB => Begin(Rule::NormalAssign),
            Pattern::EnvCommandB => Begin(Rule::EnvCommand),
            Pattern::NormalStatementB => Begin(Rule::NormalStatement),
            Pattern::OptionB => Begin(Rule::Option),
            Pattern::ParenB => Begin(Rule::Paren),
            Pattern::ExtglobB => Begin(Rule::Extglob),
            Pattern::ProcessSubstitutionB => Begin(Rule::ProcessSubstitution),
            Pattern::StringSingleB => Begin(Rule::StringSingle),
            Pattern::StringDoubleB => Begin(Rule::StringDouble),
            Pattern::StringAnsiB => Begin(Rule::StringAnsi),
            Pattern::SubshellB => Begin(Rule::Subshell),
            Pattern::VariablePositionalB => Begin(Rule::VariablePositional),
            Pattern::VariableBraceB => Begin(Rule::VariableBrace),
            Pattern::WhileB => Begin(Rule::While),
            Pattern::EndStatement
            | Pattern::EndOption
            | Pattern::EndArg
            | Pattern::EndAlias
            | Pattern::EndArithmeticDouble
            | Pattern::EndCase
            | Pattern::EndCasePattern
            | Pattern::EndCaseBody
            | Pattern::EndCmdName
            | Pattern::EndCmdQuotedDouble
            | Pattern::EndCmdQuotedSingle
            | Pattern::EndFunction
            | Pattern::EndHeredoc
            | Pattern::EndDone
            | Pattern::EndFi
            | Pattern::EndBacktickComment
            | Pattern::EndBacktickCommentLine
            | Pattern::EndLit => End,
            Pattern::Comment
            | Pattern::PipeTime
            | Pattern::PipeOp
            | Pattern::Sep
            | Pattern::LineCont
            | Pattern::VariableAt
            | Pattern::VariableDigit
            | Pattern::VariableSpecial
            | Pattern::VariableName
            | Pattern::RedirectNum
            | Pattern::Tilde
            | Pattern::Glob
            | Pattern::FloatKw
            | Pattern::Support
            | Pattern::Escape
            | Pattern::Boolean
            | Pattern::Numeric
            | Pattern::Unquoted
            | Pattern::DqEscape
            | Pattern::BtEscape
            | Pattern::CaseDefault
            | Pattern::CaseStar
            | Pattern::CasePlus
            | Pattern::CaseQuestion
            | Pattern::CaseAt
            | Pattern::CasePipe
            | Pattern::CaseParenOpen
            | Pattern::CaseChar
            | Pattern::AnsiEscape
            | Pattern::AnsiOctal
            | Pattern::AnsiHex
            | Pattern::AnsiControl
            | Pattern::MathOp
            | Pattern::MathHex
            | Pattern::MathSemi
            | Pattern::MathOctal
            | Pattern::MathOther
            | Pattern::MathInt
            | Pattern::MathName
            | Pattern::SpecialExpansion
            | Pattern::ArrayAccess
            | Pattern::ParamDigits
            | Pattern::ParamName
            | Pattern::ArgMatch
            | Pattern::WildcardStar
            | Pattern::BoolArg
            | Pattern::CmdKeyword
            | Pattern::CmdBuiltin
            | Pattern::CmdName
            | Pattern::ArrayNameEq
            | Pattern::ArrayBracketEq
            | Pattern::ModOption
            | Pattern::ModMatch
            | Pattern::LogicalOp
            | Pattern::LogicalTest
            | Pattern::RegexCompare
            | Pattern::FuncBlank
            | Pattern::RedirectFix => Match(S::NO_SCOPES),
        }
    }
}

/// Where a pattern can start to match, for skipping the positions where no
/// pattern of a rule can.
/// Each variant over-approximates: a position it excludes cannot match.
/// The engine always tries the first position of a search, the start of the
/// line and the end of the text.
/// Patterns that need `\\G`, `^` or `$` therefore say [`Start::Never`].
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
    /// Only where [`Pattern::anchored`] or [`Pattern::at_line_start`] say.
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

impl Pattern {
    /// Whether the pattern can match at the first position of a search without
    /// starting at a byte in its [`Start`]: it is `\\G`.
    pub(crate) fn anchored(self) -> bool {
        matches!(
            self,
            Pattern::CmdNameB
                | Pattern::ContinuationDoubleB
                | Pattern::ContinuationSingleB
                | Pattern::FuncBlank
        )
    }

    /// Whether the pattern can match at the start of the line without starting
    /// at a byte in its [`Start`]: it is `^`.
    pub(crate) fn at_line_start(self) -> bool {
        matches!(
            self,
            Pattern::EndHeredoc | Pattern::MathOp | Pattern::NormalStatementB
        )
    }

    #[expect(clippy::too_many_lines, reason = "one arm per pattern, a table")]
    pub(crate) fn start(self) -> Start {
        use Start::{After, Any, Bytes, Class, Never};
        match self {
            Pattern::Comment => Bytes(b" \t#"),
            Pattern::PipeTime | Pattern::Boolean | Pattern::BoolArg => Bytes(b"tf"),
            Pattern::PipeOp => Bytes(b"!|"),
            Pattern::Sep => Bytes(b";&|"),
            Pattern::LineCont
            | Pattern::Escape
            | Pattern::DqEscape
            | Pattern::BtEscape
            | Pattern::AnsiEscape
            | Pattern::AnsiOctal
            | Pattern::AnsiHex
            | Pattern::AnsiControl => Bytes(b"\\"),
            Pattern::VariableAt
            | Pattern::VariableDigit
            | Pattern::VariableSpecial
            | Pattern::VariableName
            | Pattern::StringAnsiB
            | Pattern::SubshellB
            | Pattern::VariablePositionalB
            | Pattern::VariableBraceB => Bytes(b"$"),
            Pattern::HerestringWord
            | Pattern::HeredocQuotedIndentB
            | Pattern::HeredocQuotedB
            | Pattern::HeredocIndentB
            | Pattern::HeredocB
            | Pattern::HerestringSingleB
            | Pattern::HerestringDoubleB => Bytes(b"<"),
            Pattern::RedirectOp => Class(redirect_start),
            Pattern::RedirectNum | Pattern::ParamDigits | Pattern::MathOther | Pattern::MathInt => {
                Class(digit)
            }
            Pattern::Tilde => Bytes(b"~"),
            Pattern::Glob => Bytes(b"*?"),
            Pattern::FloatKw => Bytes(b"tedif"),
            Pattern::Support => Bytes(b".:"),
            Pattern::Numeric => Class(number_start),
            Pattern::Unquoted => Class(is_unquoted),
            Pattern::CaseDefault => Bytes(b" \t*"),
            Pattern::CaseStar | Pattern::WildcardStar => Bytes(b"*"),
            Pattern::CasePlus => Bytes(b"+"),
            Pattern::CaseQuestion => Bytes(b"?"),
            Pattern::CaseAt => Bytes(b"@"),
            Pattern::CasePipe => Bytes(b"|"),
            Pattern::CaseParenOpen
            | Pattern::CaseGroupB
            | Pattern::ArithmeticDoubleB
            | Pattern::ArithmeticSingleB
            | Pattern::FuncParenB
            | Pattern::ParenB => Bytes(b"("),
            Pattern::CaseChar => Class(case_char),
            Pattern::MathOp => Class(math_operator),
            Pattern::MathHex | Pattern::MathOctal => Bytes(b"0"),
            Pattern::MathSemi => Bytes(b";"),
            Pattern::MathName => Class(math_name),
            Pattern::SpecialExpansion => Bytes(b"!:*@#%/"),
            Pattern::ArrayAccess
            | Pattern::ArrayBracketEq
            | Pattern::CaseClassB
            | Pattern::LogicalDoubleB
            | Pattern::LogicalSingleB => Bytes(b"["),
            Pattern::ParamName | Pattern::ArrayNameEq | Pattern::ModMatch => Class(is_name),
            Pattern::ArgMatch => Class(blank_or_unquoted),
            Pattern::CmdKeyword => Bytes(b"crb"),
            Pattern::CmdBuiltin => Class(builtin_start),
            Pattern::CmdName => Class(is_command_name),
            Pattern::ModOption | Pattern::LogicalTest => Bytes(b"-"),
            Pattern::LogicalOp => Bytes(b"=!<>&|"),
            Pattern::RegexCompare => Bytes(b"="),
            Pattern::RedirectFix => Bytes(b">"),
            Pattern::AliasB => Bytes(b" \ta"),
            Pattern::ArgB | Pattern::OptionB => Bytes(b" \t"),
            Pattern::ArrayValueB | Pattern::NormalAssignB => Class(name_or_blank),
            Pattern::ModifiedB => Bytes(b"rdtel"),
            Pattern::CaseB => Bytes(b"c"),
            Pattern::CasePatternB | Pattern::EndBacktickComment => Any,
            Pattern::CaseBodyB => After(b")"),
            Pattern::CmdStatementB => Class(command_statement_start),
            Pattern::CmdNameB
            | Pattern::ContinuationDoubleB
            | Pattern::ContinuationSingleB
            | Pattern::FuncBlank
            | Pattern::EndHeredoc
            | Pattern::EndLit => Never,
            Pattern::CmdQuotedB => Bytes(b"$\"'"),
            Pattern::ForInB | Pattern::ForB | Pattern::LoopForB | Pattern::EndFi => Bytes(b"f"),
            Pattern::FunctionB => Class(function_start),
            Pattern::FuncBraceB | Pattern::GroupB => Bytes(b"{"),
            Pattern::BacktickB | Pattern::EndBacktickCommentLine => Bytes(b"`"),
            Pattern::BacktickCommentB | Pattern::BacktickCommentLineB => Bytes(b"#"),
            Pattern::LoopWhileB | Pattern::WhileB => Bytes(b"wu"),
            Pattern::LoopSelectB => Bytes(b"s"),
            Pattern::LoopIfB => Bytes(b"i"),
            Pattern::EnvCommandB => After(b" \t"),
            Pattern::NormalStatementB => After(b"!&(;`{| "),
            Pattern::ExtglobB => Bytes(b"!*+?@"),
            Pattern::ProcessSubstitutionB => Bytes(b"<>"),
            Pattern::StringSingleB => Bytes(b"'"),
            Pattern::StringDoubleB => Bytes(b"$\""),
            Pattern::EndStatement | Pattern::EndOption => Bytes(b"\n&);`{|}] \t#"),
            Pattern::EndArg => Bytes(b" \t&;|\n)`"),
            Pattern::EndAlias => Bytes(b" \t;&|\n"),
            Pattern::EndArithmeticDouble => Bytes(b")"),
            Pattern::EndCase => Bytes(b"e"),
            Pattern::EndCasePattern => Bytes(b"e)"),
            Pattern::EndCaseBody => Bytes(b";e"),
            Pattern::EndCmdName => Bytes(b" \t&;|\n)`<"),
            Pattern::EndCmdQuotedDouble => After(b"\""),
            Pattern::EndCmdQuotedSingle => After(b"'"),
            Pattern::EndFunction => After(b")}"),
            Pattern::EndDone => Bytes(b"d"),
        }
    }
}

/// What a rule adds and tries: its definition of the shell's rule table.
#[derive(Debug)]
pub(crate) struct RuleDefinition {
    /// Scopes of everything in the rule, begin and end included.
    pub(crate) name: Scopes<S>,
    /// Scopes of what lies between begin and end.
    pub(crate) content: Scopes<S>,
    /// Prefix of a content scope that ends in the here-document delimiter.
    pub(crate) content_delimiter: Option<&'static str>,
    pub(crate) end: Option<Pattern>,
    /// Whether the end pattern comes after the other patterns.
    pub(crate) end_last: bool,
    /// For [`Pattern::EndLit`]: the text that ends the rule.
    pub(crate) end_lit: &'static [u8],
    /// For [`Pattern::EndLit`]: the scopes of that text.
    pub(crate) end_scopes: Scopes<S>,
    /// The end pattern and the patterns, in the order they are tried.
    /// A rule's builder leaves the patterns alone here.
    /// [`with_tables`] places the end among them.
    pub(crate) order: Box<[Pattern]>,
    /// For the byte before a position: which zero-width patterns can match
    /// there, as a slot of [`RuleDefinition::candidates`].
    after: [u8; 256],
    /// Where the candidates for a position are in `arena`, by [`Self::slot`].
    index: Vec<(usize, usize)>,
    /// The candidate lists, each in the order the patterns are tried.
    arena: Vec<Pattern>,
    /// Whether the positions inside a run of blanks that no pattern matched
    /// at the start of can be skipped: no pattern matches anywhere.
    pub(crate) skip_blanks: bool,
}

impl RuleDefinition {
    /// The patterns that can match at a position with `byte` and `prev` at and
    /// before it, in order.
    /// `first` is whether it is the first position of a search, and `bol`
    /// whether it starts the line.
    pub(crate) fn candidates(&self, first: bool, bol: bool, prev: u8, byte: u8) -> &[Pattern] {
        let variant = usize::from(first) | (usize::from(bol) << 1);
        let slot = usize::from(self.after[usize::from(prev)]);
        let (start, len) = self.index[(variant * self.slots() + slot) * 256 + usize::from(byte)];
        &self.arena[start..start + len]
    }

    fn slots(&self) -> usize {
        self.index.len() / (4 * 256)
    }
}

type Group = &'static [Pattern];

const COMMENT: Group = &[Pattern::Comment];
const PIPELINE: Group = &[Pattern::PipeTime, Pattern::PipeOp];
const SEP: Group = &[Pattern::Sep];
const LOGICAL_DOUBLE: Group = &[Pattern::LogicalDoubleB];
const LOGICAL_SINGLE: Group = &[Pattern::LogicalSingleB];
const ASSIGNMENT: Group = &[
    Pattern::ArrayValueB,
    Pattern::ModifiedB,
    Pattern::NormalAssignB,
];
const CASE: Group = &[Pattern::CaseB];
const FOR: Group = &[Pattern::ForInB, Pattern::ForB];
const LOOP: Group = &[
    Pattern::LoopForB,
    Pattern::LoopWhileB,
    Pattern::LoopSelectB,
    Pattern::LoopIfB,
];
const FUNCTION: Group = &[Pattern::FunctionB];
const LINE_CONTINUATION: Group = &[Pattern::LineCont];
const ARITH_DOUBLE: Group = &[Pattern::ArithmeticDoubleB];
const MISC_RANGES: Group = &[
    Pattern::LogicalSingleB,
    Pattern::LogicalDoubleB,
    Pattern::SubshellB,
    Pattern::GroupB,
];
const VARIABLE: Group = &[
    Pattern::VariableAt,
    Pattern::VariableDigit,
    Pattern::VariableSpecial,
    Pattern::VariablePositionalB,
    Pattern::VariableBraceB,
    Pattern::VariableName,
];
const INTERPOLATION: Group = &[Pattern::SubshellB, Pattern::BacktickB];
const HEREDOC: Group = &[
    Pattern::HeredocQuotedIndentB,
    Pattern::HeredocQuotedB,
    Pattern::HeredocIndentB,
    Pattern::HeredocB,
];
const HERESTRING: Group = &[
    Pattern::HerestringSingleB,
    Pattern::HerestringDoubleB,
    Pattern::HerestringWord,
];
const REDIRECTION: Group = &[Pattern::ProcessSubstitutionB, Pattern::RedirectOp];
const PATHNAME: Group = &[Pattern::Tilde, Pattern::Glob, Pattern::ExtglobB];
const FLOATING_KEYWORD: Group = &[Pattern::FloatKw];
const ALIAS: Group = &[Pattern::AliasB];
const NORMAL_STATEMENT: Group = &[Pattern::NormalStatementB];
const STRING: Group = &[
    Pattern::Escape,
    Pattern::StringSingleB,
    Pattern::StringDoubleB,
    Pattern::StringAnsiB,
];
const SUPPORT: Group = &[Pattern::Support];
const BOOLEAN: Group = &[Pattern::Boolean];
const REDIRECT_NUMBER: Group = &[Pattern::RedirectNum];
const NUMERIC: Group = &[Pattern::Numeric];
const PAREN: Group = &[Pattern::ParenB];
const UNQUOTED: Group = &[Pattern::Unquoted];
const DQ_ESCAPE: Group = &[Pattern::DqEscape];
const ARGUMENT_MATCH: Group = &[Pattern::ArgMatch];
const MATH: Group = &[
    Pattern::MathOp,
    Pattern::MathHex,
    Pattern::MathSemi,
    Pattern::MathOctal,
    Pattern::MathOther,
    Pattern::MathInt,
    Pattern::MathName,
];
const MATH_OPERATORS: Group = &[
    Pattern::MathOp,
    Pattern::MathHex,
    Pattern::MathOctal,
    Pattern::MathOther,
    Pattern::MathInt,
];
const CASE_CONTEXT: Group = &[
    Pattern::CaseStar,
    Pattern::CasePlus,
    Pattern::CaseQuestion,
    Pattern::CaseAt,
    Pattern::CasePipe,
    Pattern::Escape,
    Pattern::CaseParenOpen,
    Pattern::CaseGroupB,
    Pattern::CaseClassB,
];
const OPTION_B: Group = &[Pattern::OptionB];
const ARG_B: Group = &[Pattern::ArgB];

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
    &[Pattern::WhileB],
    FUNCTION,
    &[Pattern::CmdStatementB],
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
    &[Pattern::RegexCompare],
    &[Pattern::ArithmeticSingleB],
    &[
        Pattern::ArithmeticSingleB,
        Pattern::LogicalOp,
        Pattern::LogicalTest,
    ],
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

fn flatten(groups: &[&[&[Pattern]]]) -> Box<[Pattern]> {
    groups
        .iter()
        .flat_map(|list| list.iter())
        .flat_map(|group| group.iter().copied())
        .collect()
}

fn definition(
    name: Scopes<S>,
    content: Scopes<S>,
    end: Option<Pattern>,
    groups: &[&[&[Pattern]]],
) -> RuleDefinition {
    RuleDefinition {
        name,
        content,
        content_delimiter: None,
        end,
        end_last: false,
        end_lit: b"",
        end_scopes: S::NO_SCOPES,
        order: flatten(groups),
        after: [0; 256],
        index: Vec::new(),
        arena: Vec::new(),
        skip_blanks: true,
    }
}

/// A rule that ends at the literal `lit`, whose scopes are `scopes`.
fn lit(
    name: Scopes<S>,
    content: Scopes<S>,
    lit: &'static [u8],
    scopes: Scopes<S>,
    groups: &[&[&[Pattern]]],
) -> RuleDefinition {
    RuleDefinition {
        end_lit: lit,
        end_scopes: scopes,
        ..definition(name, content, Some(Pattern::EndLit), groups)
    }
}

fn heredoc(prefix: &'static str, groups: &[&[&[Pattern]]]) -> RuleDefinition {
    RuleDefinition {
        content_delimiter: Some(prefix),
        ..definition(
            S::NO_SCOPES,
            S::NO_SCOPES,
            Some(Pattern::EndHeredoc),
            groups,
        )
    }
}

#[expect(clippy::too_many_lines, reason = "one arm per rule, a table")]
fn build(rule: Rule) -> RuleDefinition {
    const NORMAL: &[&[Pattern]] = NORMAL_CONTEXT;
    const INITIAL: &[&[Pattern]] = INITIAL_CONTEXT;
    const TYPICAL: &[&[Pattern]] = TYPICAL_STATEMENTS;
    const LOGICAL: &[&[Pattern]] = LOGICAL_CONTEXT;
    const OPTION: &[&[Pattern]] = OPTION_CONTEXT;
    match rule {
        Rule::Root | Rule::RetokenisedInitial => {
            definition(S::NO_SCOPES, S::NO_SCOPES, None, &[INITIAL])
        }
        Rule::Alias => definition(
            scopes![S::MetaExpressionAssignmentAlias],
            S::NO_SCOPES,
            Some(Pattern::EndAlias),
            &[NORMAL],
        ),
        Rule::Argument => definition(
            scopes![S::MetaArgument],
            S::NO_SCOPES,
            Some(Pattern::EndArg),
            &[&[ARGUMENT_MATCH], NORMAL, &[LINE_CONTINUATION]],
        ),
        Rule::ArithmeticDouble => definition(
            scopes![S::MetaArithmetic],
            S::NO_SCOPES,
            Some(Pattern::EndArithmeticDouble),
            &[&[VARIABLE, MATH, STRING]],
        ),
        Rule::ArithmeticSingle => lit(
            scopes![S::MetaArithmetic],
            S::NO_SCOPES,
            b")",
            scopes![S::PunctuationSectionArithmeticSingle],
            &[&[VARIABLE, MATH, STRING]],
        ),
        Rule::ArrayValue => lit(
            S::NO_SCOPES,
            S::NO_SCOPES,
            b")",
            scopes![S::PunctuationDefinitionArray],
            &[
                &[COMMENT, &[Pattern::ArrayNameEq, Pattern::ArrayBracketEq]],
                NORMAL,
                &[UNQUOTED],
            ],
        ),
        Rule::Case => definition(
            scopes![S::MetaCase],
            S::NO_SCOPES,
            Some(Pattern::EndCase),
            &[&[
                COMMENT,
                &[
                    Pattern::CaseDefault,
                    Pattern::CasePatternB,
                    Pattern::CaseBodyB,
                ],
            ]],
        ),
        Rule::CasePattern => definition(
            scopes![S::MetaCaseEntryPattern],
            S::NO_SCOPES,
            Some(Pattern::EndCasePattern),
            &[&[CASE_CONTEXT, STRING, &[Pattern::CaseChar]]],
        ),
        Rule::CaseBody => definition(
            scopes![S::MetaCaseEntryBody],
            S::NO_SCOPES,
            Some(Pattern::EndCaseBody),
            &[TYPICAL, INITIAL],
        ),
        Rule::CaseGroup => lit(
            scopes![S::MetaParen],
            S::NO_SCOPES,
            b")",
            scopes![
                S::PunctuationDefinitionGroup,
                S::PunctuationDefinitionRegexGroup,
            ],
            &[&[CASE_CONTEXT, STRING, &[Pattern::CaseChar]]],
        ),
        Rule::CaseClass => lit(
            scopes![S::StringRegexpCharacterClass],
            S::NO_SCOPES,
            b"]",
            scopes![S::PunctuationDefinitionCharacterClass],
            &[&[&[Pattern::Escape]]],
        ),
        Rule::CommandStatement => definition(
            scopes![S::MetaStatementCommand],
            S::NO_SCOPES,
            Some(Pattern::EndStatement),
            &[&[
                &[Pattern::CmdNameB],
                LINE_CONTINUATION,
                OPTION_B,
                ARG_B,
                STRING,
                HEREDOC,
            ]],
        ),
        Rule::CommandName => definition(
            scopes![S::MetaStatementCommandName],
            S::NO_SCOPES,
            Some(Pattern::EndCmdName),
            &[&[
                &[Pattern::CmdKeyword, Pattern::CmdBuiltin],
                VARIABLE,
                &[Pattern::CmdName, Pattern::CmdQuotedB],
                LINE_CONTINUATION,
                UNQUOTED,
            ]],
        ),
        Rule::CommandQuotedDouble => definition(
            S::NO_SCOPES,
            S::NO_SCOPES,
            Some(Pattern::EndCmdQuotedDouble),
            &[&[&[Pattern::ContinuationSingleB, Pattern::ContinuationDoubleB]]],
        ),
        Rule::CommandQuotedSingle => definition(
            S::NO_SCOPES,
            S::NO_SCOPES,
            Some(Pattern::EndCmdQuotedSingle),
            &[&[&[Pattern::ContinuationSingleB, Pattern::ContinuationDoubleB]]],
        ),
        Rule::ContinuationDouble => lit(
            S::NO_SCOPES,
            scopes![
                S::MetaStatementCommandNameContinuation,
                S::StringQuotedDoubleBare,
                S::EntityNameFunctionCallBare,
                S::EntityNameCommandBare,
            ],
            b"\"",
            scopes![
                S::StringQuotedDouble,
                S::PunctuationDefinitionStringEnd,
                S::EntityNameFunctionCall,
                S::EntityNameCommand,
            ],
            &[&[DQ_ESCAPE, VARIABLE, INTERPOLATION]],
        ),
        Rule::ContinuationSingle => lit(
            S::NO_SCOPES,
            scopes![
                S::MetaStatementCommandNameContinuation,
                S::StringQuotedSingleBare,
                S::EntityNameFunctionCallBare,
                S::EntityNameCommandBare,
            ],
            b"'",
            scopes![
                S::StringQuotedSingle,
                S::PunctuationDefinitionStringEnd,
                S::EntityNameFunctionCall,
                S::EntityNameCommand,
            ],
            &[],
        ),
        Rule::ForIn => definition(
            scopes![S::MetaForIn],
            S::NO_SCOPES,
            Some(Pattern::EndStatement),
            &[&[STRING, UNQUOTED], NORMAL],
        ),
        Rule::For => definition(
            scopes![S::MetaFor],
            S::NO_SCOPES,
            Some(Pattern::EndStatement),
            &[&[ARITH_DOUBLE], NORMAL],
        ),
        Rule::Function => RuleDefinition {
            end_last: true,
            ..definition(
                scopes![S::MetaFunction],
                S::NO_SCOPES,
                Some(Pattern::EndFunction),
                &[
                    &[&[Pattern::FuncBlank, Pattern::FuncBraceB, Pattern::FuncParenB]],
                    INITIAL,
                ],
            )
        },
        Rule::FunctionBrace => lit(
            scopes![S::MetaFunctionBody],
            S::NO_SCOPES,
            b"}",
            scopes![
                S::PunctuationDefinitionGroup,
                S::PunctuationSectionFunctionDefinition,
            ],
            &[INITIAL],
        ),
        Rule::FunctionParen => lit(
            scopes![S::MetaFunctionBody],
            S::NO_SCOPES,
            b")",
            scopes![
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
            scopes![S::MetaHerestring],
            scopes![S::StringQuotedSingle],
            b"'",
            scopes![S::StringQuotedSingle, S::PunctuationDefinitionStringEnd],
            &[],
        ),
        Rule::HerestringDouble => lit(
            scopes![S::MetaHerestring],
            scopes![S::StringQuotedDouble],
            b"\"",
            scopes![S::StringQuotedDouble, S::PunctuationDefinitionStringEnd],
            &[&[DQ_ESCAPE, VARIABLE, INTERPOLATION]],
        ),
        Rule::Backtick => lit(
            scopes![S::StringInterpolatedBacktick],
            S::NO_SCOPES,
            b"`",
            scopes![S::PunctuationDefinitionEvaluationBackticks],
            &[&[&[Pattern::BtEscape, Pattern::BacktickCommentB]], INITIAL],
        ),
        Rule::BacktickComment => definition(
            S::NO_SCOPES,
            S::NO_SCOPES,
            Some(Pattern::EndBacktickComment),
            &[&[&[Pattern::BacktickCommentLineB]]],
        ),
        Rule::BacktickCommentLine => definition(
            scopes![S::CommentLineNumberSign],
            S::NO_SCOPES,
            Some(Pattern::EndBacktickCommentLine),
            &[],
        ),
        Rule::LogicalDouble => lit(
            scopes![S::MetaScopeLogicalExpression],
            S::NO_SCOPES,
            b"]]",
            scopes![S::PunctuationDefinitionLogicalExpression],
            &[LOGICAL],
        ),
        Rule::LogicalSingle => lit(
            scopes![S::MetaScopeLogicalExpression],
            S::NO_SCOPES,
            b"]",
            scopes![S::PunctuationDefinitionLogicalExpression],
            &[LOGICAL],
        ),
        Rule::LoopFor => definition(
            scopes![S::MetaScopeForInLoop],
            S::NO_SCOPES,
            Some(Pattern::EndDone),
            &[INITIAL],
        ),
        Rule::LoopWhile => definition(
            scopes![S::MetaScopeWhileLoop],
            S::NO_SCOPES,
            Some(Pattern::EndDone),
            &[INITIAL],
        ),
        Rule::LoopSelect => definition(
            scopes![S::MetaScopeSelectBlock],
            S::NO_SCOPES,
            Some(Pattern::EndDone),
            &[INITIAL],
        ),
        Rule::LoopIf => definition(
            scopes![S::MetaScopeIfBlock],
            S::NO_SCOPES,
            Some(Pattern::EndFi),
            &[INITIAL],
        ),
        Rule::Group => lit(
            scopes![S::MetaScopeGroup],
            S::NO_SCOPES,
            b"}",
            scopes![S::PunctuationDefinitionGroup],
            &[INITIAL],
        ),
        Rule::Modified => definition(
            scopes![S::MetaStatement, S::MetaExpressionAssignmentModified],
            S::NO_SCOPES,
            Some(Pattern::EndStatement),
            &[
                &[&[Pattern::ModOption, Pattern::ArrayValueB, Pattern::ModMatch]],
                NORMAL,
            ],
        ),
        Rule::NormalAssign => definition(
            scopes![S::MetaExpressionAssignment],
            S::NO_SCOPES,
            Some(Pattern::EndStatement),
            &[
                &[
                    COMMENT,
                    STRING,
                    &[Pattern::NormalAssignB, Pattern::EnvCommandB],
                    UNQUOTED,
                ],
                NORMAL,
            ],
        ),
        Rule::EnvCommand => definition(
            scopes![S::MetaStatementCommandEnv],
            S::NO_SCOPES,
            Some(Pattern::EndStatement),
            &[&[
                &[Pattern::CmdNameB],
                LINE_CONTINUATION,
                OPTION_B,
                ARG_B,
                STRING,
            ]],
        ),
        Rule::NormalStatement => definition(
            scopes![S::MetaStatement],
            S::NO_SCOPES,
            Some(Pattern::EndStatement),
            &[TYPICAL],
        ),
        Rule::Option => definition(
            S::NO_SCOPES,
            scopes![S::StringUnquotedArgumentBare, S::ConstantOtherOptionBare],
            Some(Pattern::EndOption),
            &[OPTION],
        ),
        Rule::Paren => lit(
            scopes![S::MetaParenGroup],
            S::NO_SCOPES,
            b")",
            scopes![S::PunctuationSectionParen],
            &[INITIAL],
        ),
        Rule::Extglob => lit(
            scopes![S::MetaStructureExtglob],
            S::NO_SCOPES,
            b")",
            scopes![S::PunctuationDefinitionExtglob],
            &[INITIAL],
        ),
        Rule::ProcessSubstitution => lit(
            scopes![S::StringInterpolatedProcessSubstitution],
            S::NO_SCOPES,
            b")",
            scopes![S::PunctuationDefinitionStringEnd],
            &[INITIAL],
        ),
        Rule::StringSingle => lit(
            scopes![S::StringQuotedSingle],
            S::NO_SCOPES,
            b"'",
            scopes![S::PunctuationDefinitionStringEnd],
            &[],
        ),
        Rule::StringDouble => lit(
            scopes![S::StringQuotedDouble],
            S::NO_SCOPES,
            b"\"",
            scopes![S::PunctuationDefinitionStringEnd],
            &[&[DQ_ESCAPE, VARIABLE, INTERPOLATION]],
        ),
        Rule::StringAnsi => lit(
            scopes![S::StringQuotedSingleDollar],
            S::NO_SCOPES,
            b"'",
            scopes![S::PunctuationDefinitionStringEnd],
            &[&[&[
                Pattern::AnsiEscape,
                Pattern::AnsiOctal,
                Pattern::AnsiHex,
                Pattern::AnsiControl,
            ]]],
        ),
        Rule::Subshell => lit(
            scopes![S::MetaScopeSubshell],
            S::NO_SCOPES,
            b")",
            scopes![S::PunctuationDefinitionSubshellSingle],
            &[&[PAREN], INITIAL],
        ),
        Rule::VariablePositional => lit(
            S::NO_SCOPES,
            scopes![S::MetaParameterExpansion],
            b"}",
            scopes![
                S::PunctuationSectionBracketCurlyVariableEnd,
                S::PunctuationDefinitionVariable,
                S::VariableParameterPositional,
            ],
            &[&[
                &[
                    Pattern::SpecialExpansion,
                    Pattern::ArrayAccess,
                    Pattern::ParamDigits,
                    Pattern::ParamName,
                ],
                VARIABLE,
                STRING,
            ]],
        ),
        Rule::VariableBrace => lit(
            S::NO_SCOPES,
            scopes![S::MetaParameterExpansion],
            b"}",
            scopes![
                S::PunctuationSectionBracketCurlyVariableEnd,
                S::PunctuationDefinitionVariable,
            ],
            &[&[
                &[
                    Pattern::SpecialExpansion,
                    Pattern::ArrayAccess,
                    Pattern::ParamName,
                ],
                VARIABLE,
                STRING,
            ]],
        ),
        Rule::While => definition(
            scopes![S::MetaWhile],
            S::NO_SCOPES,
            Some(Pattern::EndStatement),
            &[
                &[LINE_CONTINUATION, MATH_OPERATORS, OPTION_B, UNQUOTED],
                NORMAL,
                &[STRING],
            ],
        ),
        Rule::RetokenisedArgument => definition(
            S::NO_SCOPES,
            S::NO_SCOPES,
            None,
            &[&[
                &[Pattern::WildcardStar],
                VARIABLE,
                NUMERIC,
                &[Pattern::BoolArg],
            ]],
        ),
        Rule::RetokenisedOptions => definition(
            S::NO_SCOPES,
            S::NO_SCOPES,
            None,
            &[&[&[Pattern::ModOption]]],
        ),
        Rule::RetokenisedString => definition(S::NO_SCOPES, S::NO_SCOPES, None, &[&[STRING]]),
        Rule::RetokenisedArrayAccess => definition(
            S::NO_SCOPES,
            S::NO_SCOPES,
            None,
            &[&[&[Pattern::SpecialExpansion], STRING, VARIABLE]],
        ),
        Rule::RetokenisedHeredocRest => definition(
            S::NO_SCOPES,
            S::NO_SCOPES,
            None,
            &[&[&[Pattern::RedirectFix]], TYPICAL],
        ),
    }
}

/// Builds the candidate tables of `info`.
/// For each byte at a position and each byte before it, they list the
/// patterns that can match there.
/// Positions whose list is empty are skipped without trying anything.
///
/// The zero-width patterns that depend on the byte before are listed in
/// order.
/// Each byte before a position activates a set of them, a bit each.
/// There are at most eight such patterns and at most 256 distinct sets.
///
/// `every_rule_fits_its_tables` checks both, so the fallbacks never apply.
/// What each pattern needs is computed once, so the lists only look it up.
fn with_tables(mut definition: RuleDefinition) -> RuleDefinition {
    let mut order: Vec<Pattern> = Vec::new();
    if let Some(end) = definition.end
        && !definition.end_last
    {
        order.push(end);
    }
    order.extend(definition.order.iter().copied());
    if let Some(end) = definition.end
        && definition.end_last
    {
        order.push(end);
    }
    let mut zero_width: Vec<(Pattern, &'static [u8])> = Vec::new();
    for &pattern in &order {
        if let Start::After(set) = pattern.start() {
            zero_width.push((pattern, set));
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
        definition.after[usize::from(b)] = u8::try_from(slot).unwrap_or_default();
    }
    let classes: Vec<([bool; 256], Option<usize>)> = order
        .iter()
        .map(|&pattern| {
            let mut class = [false; 256];
            let mut zero = None;
            match pattern.start() {
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
                Start::After(_) => zero = zero_width.iter().position(|&(p, _)| p == pattern),
                Start::Any => class = [true; 256],
                Start::Never => {
                    if let Some(&b) = definition
                        .end_lit
                        .first()
                        .filter(|_| pattern == Pattern::EndLit)
                    {
                        class[usize::from(b)] = true;
                    }
                }
            }
            (class, zero)
        })
        .collect();
    let mut known: HashMap<Vec<Pattern>, (usize, usize)> = HashMap::new();
    for variant in 0..4_usize {
        let (first, bol) = (variant & 1 != 0, variant & 2 != 0);
        for &mask in &masks {
            for b in 0..=u8::MAX {
                let list: Vec<Pattern> = order
                    .iter()
                    .zip(&classes)
                    .filter(|&(&pattern, &(class, zero))| {
                        class[usize::from(b)]
                            || zero.is_some_and(|index| mask & (1 << index.min(7)) != 0)
                            || (first && pattern.anchored())
                            || (bol && pattern.at_line_start())
                    })
                    .map(|(&pattern, _)| pattern)
                    .collect();
                let place = if let Some(&place) = known.get(&list) {
                    place
                } else {
                    let place = (definition.arena.len(), list.len());
                    definition.arena.extend_from_slice(&list);
                    known.insert(list, place);
                    place
                };
                definition.index.push(place);
            }
        }
    }
    definition.skip_blanks = !order
        .iter()
        .any(|pattern| matches!(pattern.start(), Start::Any));
    definition.order = order.into_boxed_slice();
    definition
}

/// Every rule's table, indexed by [`Rule::index`], built when first needed.
static TABLE: [OnceLock<RuleDefinition>; Rule::ALL.len()] =
    [const { OnceLock::new() }; Rule::ALL.len()];

/// The definition of `rule`.
pub(crate) fn rule_definition(rule: Rule) -> &'static RuleDefinition {
    TABLE[rule.index()].get_or_init(move || with_tables(build(rule)))
}

#[cfg(test)]
mod tests {
    use super::{Start, rule_definition};
    use crate::shell::state::Rule;

    #[test]
    fn every_rule_fits_its_tables() {
        for &rule in Rule::ALL {
            let definition = rule_definition(rule);
            let zero_width = definition
                .order
                .iter()
                .filter(|pattern| matches!(pattern.start(), Start::After(_)))
                .count();
            assert!(
                zero_width <= 8,
                "{rule:?}: {zero_width} zero-width patterns"
            );
            assert!(
                definition.slots() <= 256,
                "{rule:?}: {} sets",
                definition.slots()
            );
        }
    }
}
