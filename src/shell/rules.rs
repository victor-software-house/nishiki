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
    PipeOperator,
    Separator,
    LineContinuation,
    VariableAt,
    VariableDigit,
    VariableSpecial,
    VariableName,
    HerestringWord,
    RedirectOperator,
    RedirectNumber,
    Tilde,
    Glob,
    FloatingKeyword,
    Support,
    Escape,
    Boolean,
    Numeric,
    Unquoted,
    DoubleQuoteEscape,
    BacktickEscape,
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
    MathOperator,
    MathHex,
    MathSemicolon,
    MathOctal,
    MathOther,
    MathInteger,
    MathName,
    SpecialExpansion,
    ArrayAccess,
    ParameterDigits,
    ParameterName,
    ArgumentMatch,
    WildcardStar,
    BooleanArgument,
    CommandKeyword,
    CommandBuiltin,
    CommandName,
    ArrayNameEquals,
    ArrayBracketEquals,
    ModifiedOption,
    ModifiedMatch,
    LogicalOperator,
    LogicalTest,
    RegexCompare,
    FunctionBlank,
    RedirectFix,
    // Begins of rules.
    AliasBegin,
    ArgumentBegin,
    ArithmeticDoubleBegin,
    ArithmeticSingleBegin,
    ArrayValueBegin,
    CaseBegin,
    CasePatternBegin,
    CaseBodyBegin,
    CaseGroupBegin,
    CaseClassBegin,
    CommandStatementBegin,
    CommandNameBegin,
    CommandQuotedBegin,
    ContinuationDoubleBegin,
    ContinuationSingleBegin,
    ForInBegin,
    ForBegin,
    FunctionBegin,
    FunctionBraceBegin,
    FunctionParenBegin,
    HeredocQuotedIndentBegin,
    HeredocQuotedBegin,
    HeredocIndentBegin,
    HeredocBegin,
    HerestringSingleBegin,
    HerestringDoubleBegin,
    BacktickBegin,
    BacktickCommentBegin,
    BacktickCommentLineBegin,
    LogicalDoubleBegin,
    LogicalSingleBegin,
    LoopForBegin,
    LoopWhileBegin,
    LoopSelectBegin,
    LoopIfBegin,
    GroupBegin,
    ModifiedBegin,
    NormalAssignBegin,
    EnvCommandBegin,
    NormalStatementBegin,
    OptionBegin,
    ParenBegin,
    ExtglobBegin,
    ProcessSubstitutionBegin,
    StringSingleBegin,
    StringDoubleBegin,
    StringAnsiBegin,
    SubshellBegin,
    VariablePositionalBegin,
    VariableBraceBegin,
    WhileBegin,
    // Ends of rules.
    EndStatement,
    EndOption,
    EndArgument,
    EndAlias,
    EndArithmeticDouble,
    EndCase,
    EndCasePattern,
    EndCaseBody,
    EndCommandName,
    EndCommandQuotedDouble,
    EndCommandQuotedSingle,
    EndFunction,
    EndHeredoc,
    EndDone,
    EndFi,
    EndBacktickComment,
    EndBacktickCommentLine,
    /// The rule's own literal, `RuleDefinition::end_literal`.
    EndLiteral,
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
            Pattern::RedirectOperator => Match(scopes![S::KeywordOperatorRedirect]),
            Pattern::AliasBegin => Begin(Rule::Alias),
            Pattern::ArgumentBegin => Begin(Rule::Argument),
            Pattern::ArithmeticDoubleBegin => Begin(Rule::ArithmeticDouble),
            Pattern::ArithmeticSingleBegin => Begin(Rule::ArithmeticSingle),
            Pattern::ArrayValueBegin => Begin(Rule::ArrayValue),
            Pattern::CaseBegin => Begin(Rule::Case),
            Pattern::CasePatternBegin => Begin(Rule::CasePattern),
            Pattern::CaseBodyBegin => Begin(Rule::CaseBody),
            Pattern::CaseGroupBegin => Begin(Rule::CaseGroup),
            Pattern::CaseClassBegin => Begin(Rule::CaseClass),
            Pattern::CommandStatementBegin => Begin(Rule::CommandStatement),
            Pattern::CommandNameBegin => Begin(Rule::CommandName),
            Pattern::CommandQuotedBegin => BeginChosen,
            Pattern::ContinuationDoubleBegin => Begin(Rule::ContinuationDouble),
            Pattern::ContinuationSingleBegin => Begin(Rule::ContinuationSingle),
            Pattern::ForInBegin => Begin(Rule::ForIn),
            Pattern::ForBegin => Begin(Rule::For),
            Pattern::FunctionBegin => Begin(Rule::Function),
            Pattern::FunctionBraceBegin => Begin(Rule::FunctionBrace),
            Pattern::FunctionParenBegin => Begin(Rule::FunctionParen),
            Pattern::HeredocQuotedIndentBegin => Begin(Rule::HeredocQuotedIndent),
            Pattern::HeredocQuotedBegin => Begin(Rule::HeredocQuoted),
            Pattern::HeredocIndentBegin => Begin(Rule::HeredocIndent),
            Pattern::HeredocBegin => Begin(Rule::Heredoc),
            Pattern::HerestringSingleBegin => Begin(Rule::HerestringSingle),
            Pattern::HerestringDoubleBegin => Begin(Rule::HerestringDouble),
            Pattern::BacktickBegin => Begin(Rule::Backtick),
            Pattern::BacktickCommentBegin => Begin(Rule::BacktickComment),
            Pattern::BacktickCommentLineBegin => Begin(Rule::BacktickCommentLine),
            Pattern::LogicalDoubleBegin => Begin(Rule::LogicalDouble),
            Pattern::LogicalSingleBegin => Begin(Rule::LogicalSingle),
            Pattern::LoopForBegin => Begin(Rule::LoopFor),
            Pattern::LoopWhileBegin => Begin(Rule::LoopWhile),
            Pattern::LoopSelectBegin => Begin(Rule::LoopSelect),
            Pattern::LoopIfBegin => Begin(Rule::LoopIf),
            Pattern::GroupBegin => Begin(Rule::Group),
            Pattern::ModifiedBegin => Begin(Rule::Modified),
            Pattern::NormalAssignBegin => Begin(Rule::NormalAssign),
            Pattern::EnvCommandBegin => Begin(Rule::EnvCommand),
            Pattern::NormalStatementBegin => Begin(Rule::NormalStatement),
            Pattern::OptionBegin => Begin(Rule::Option),
            Pattern::ParenBegin => Begin(Rule::Paren),
            Pattern::ExtglobBegin => Begin(Rule::Extglob),
            Pattern::ProcessSubstitutionBegin => Begin(Rule::ProcessSubstitution),
            Pattern::StringSingleBegin => Begin(Rule::StringSingle),
            Pattern::StringDoubleBegin => Begin(Rule::StringDouble),
            Pattern::StringAnsiBegin => Begin(Rule::StringAnsi),
            Pattern::SubshellBegin => Begin(Rule::Subshell),
            Pattern::VariablePositionalBegin => Begin(Rule::VariablePositional),
            Pattern::VariableBraceBegin => Begin(Rule::VariableBrace),
            Pattern::WhileBegin => Begin(Rule::While),
            Pattern::EndStatement
            | Pattern::EndOption
            | Pattern::EndArgument
            | Pattern::EndAlias
            | Pattern::EndArithmeticDouble
            | Pattern::EndCase
            | Pattern::EndCasePattern
            | Pattern::EndCaseBody
            | Pattern::EndCommandName
            | Pattern::EndCommandQuotedDouble
            | Pattern::EndCommandQuotedSingle
            | Pattern::EndFunction
            | Pattern::EndHeredoc
            | Pattern::EndDone
            | Pattern::EndFi
            | Pattern::EndBacktickComment
            | Pattern::EndBacktickCommentLine
            | Pattern::EndLiteral => End,
            Pattern::Comment
            | Pattern::PipeTime
            | Pattern::PipeOperator
            | Pattern::Separator
            | Pattern::LineContinuation
            | Pattern::VariableAt
            | Pattern::VariableDigit
            | Pattern::VariableSpecial
            | Pattern::VariableName
            | Pattern::RedirectNumber
            | Pattern::Tilde
            | Pattern::Glob
            | Pattern::FloatingKeyword
            | Pattern::Support
            | Pattern::Escape
            | Pattern::Boolean
            | Pattern::Numeric
            | Pattern::Unquoted
            | Pattern::DoubleQuoteEscape
            | Pattern::BacktickEscape
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
            | Pattern::MathOperator
            | Pattern::MathHex
            | Pattern::MathSemicolon
            | Pattern::MathOctal
            | Pattern::MathOther
            | Pattern::MathInteger
            | Pattern::MathName
            | Pattern::SpecialExpansion
            | Pattern::ArrayAccess
            | Pattern::ParameterDigits
            | Pattern::ParameterName
            | Pattern::ArgumentMatch
            | Pattern::WildcardStar
            | Pattern::BooleanArgument
            | Pattern::CommandKeyword
            | Pattern::CommandBuiltin
            | Pattern::CommandName
            | Pattern::ArrayNameEquals
            | Pattern::ArrayBracketEquals
            | Pattern::ModifiedOption
            | Pattern::ModifiedMatch
            | Pattern::LogicalOperator
            | Pattern::LogicalTest
            | Pattern::RegexCompare
            | Pattern::FunctionBlank
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
            Pattern::CommandNameBegin
                | Pattern::ContinuationDoubleBegin
                | Pattern::ContinuationSingleBegin
                | Pattern::FunctionBlank
        )
    }

    /// Whether the pattern can match at the start of the line without starting
    /// at a byte in its [`Start`]: it is `^`.
    pub(crate) fn at_line_start(self) -> bool {
        matches!(
            self,
            Pattern::EndHeredoc | Pattern::MathOperator | Pattern::NormalStatementBegin
        )
    }

    #[expect(clippy::too_many_lines, reason = "one arm per pattern, a table")]
    pub(crate) fn start(self) -> Start {
        use Start::{After, Any, Bytes, Class, Never};
        match self {
            Pattern::Comment => Bytes(b" \t#"),
            Pattern::PipeTime | Pattern::Boolean | Pattern::BooleanArgument => Bytes(b"tf"),
            Pattern::PipeOperator => Bytes(b"!|"),
            Pattern::Separator => Bytes(b";&|"),
            Pattern::LineContinuation
            | Pattern::Escape
            | Pattern::DoubleQuoteEscape
            | Pattern::BacktickEscape
            | Pattern::AnsiEscape
            | Pattern::AnsiOctal
            | Pattern::AnsiHex
            | Pattern::AnsiControl => Bytes(b"\\"),
            Pattern::VariableAt
            | Pattern::VariableDigit
            | Pattern::VariableSpecial
            | Pattern::VariableName
            | Pattern::StringAnsiBegin
            | Pattern::SubshellBegin
            | Pattern::VariablePositionalBegin
            | Pattern::VariableBraceBegin => Bytes(b"$"),
            Pattern::HerestringWord
            | Pattern::HeredocQuotedIndentBegin
            | Pattern::HeredocQuotedBegin
            | Pattern::HeredocIndentBegin
            | Pattern::HeredocBegin
            | Pattern::HerestringSingleBegin
            | Pattern::HerestringDoubleBegin => Bytes(b"<"),
            Pattern::RedirectOperator => Class(redirect_start),
            Pattern::RedirectNumber
            | Pattern::ParameterDigits
            | Pattern::MathOther
            | Pattern::MathInteger => Class(digit),
            Pattern::Tilde => Bytes(b"~"),
            Pattern::Glob => Bytes(b"*?"),
            Pattern::FloatingKeyword => Bytes(b"tedif"),
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
            | Pattern::CaseGroupBegin
            | Pattern::ArithmeticDoubleBegin
            | Pattern::ArithmeticSingleBegin
            | Pattern::FunctionParenBegin
            | Pattern::ParenBegin => Bytes(b"("),
            Pattern::CaseChar => Class(case_char),
            Pattern::MathOperator => Class(math_operator),
            Pattern::MathHex | Pattern::MathOctal => Bytes(b"0"),
            Pattern::MathSemicolon => Bytes(b";"),
            Pattern::MathName => Class(math_name),
            Pattern::SpecialExpansion => Bytes(b"!:*@#%/"),
            Pattern::ArrayAccess
            | Pattern::ArrayBracketEquals
            | Pattern::CaseClassBegin
            | Pattern::LogicalDoubleBegin
            | Pattern::LogicalSingleBegin => Bytes(b"["),
            Pattern::ParameterName | Pattern::ArrayNameEquals | Pattern::ModifiedMatch => {
                Class(is_name)
            }
            Pattern::ArgumentMatch => Class(blank_or_unquoted),
            Pattern::CommandKeyword => Bytes(b"crb"),
            Pattern::CommandBuiltin => Class(builtin_start),
            Pattern::CommandName => Class(is_command_name),
            Pattern::ModifiedOption | Pattern::LogicalTest => Bytes(b"-"),
            Pattern::LogicalOperator => Bytes(b"=!<>&|"),
            Pattern::RegexCompare => Bytes(b"="),
            Pattern::RedirectFix => Bytes(b">"),
            Pattern::AliasBegin => Bytes(b" \ta"),
            Pattern::ArgumentBegin | Pattern::OptionBegin => Bytes(b" \t"),
            Pattern::ArrayValueBegin | Pattern::NormalAssignBegin => Class(name_or_blank),
            Pattern::ModifiedBegin => Bytes(b"rdtel"),
            Pattern::CaseBegin => Bytes(b"c"),
            Pattern::CasePatternBegin | Pattern::EndBacktickComment => Any,
            Pattern::CaseBodyBegin => After(b")"),
            Pattern::CommandStatementBegin => Class(command_statement_start),
            Pattern::CommandNameBegin
            | Pattern::ContinuationDoubleBegin
            | Pattern::ContinuationSingleBegin
            | Pattern::FunctionBlank
            | Pattern::EndHeredoc
            | Pattern::EndLiteral => Never,
            Pattern::CommandQuotedBegin => Bytes(b"$\"'"),
            Pattern::ForInBegin | Pattern::ForBegin | Pattern::LoopForBegin | Pattern::EndFi => {
                Bytes(b"f")
            }
            Pattern::FunctionBegin => Class(function_start),
            Pattern::FunctionBraceBegin | Pattern::GroupBegin => Bytes(b"{"),
            Pattern::BacktickBegin | Pattern::EndBacktickCommentLine => Bytes(b"`"),
            Pattern::BacktickCommentBegin | Pattern::BacktickCommentLineBegin => Bytes(b"#"),
            Pattern::LoopWhileBegin | Pattern::WhileBegin => Bytes(b"wu"),
            Pattern::LoopSelectBegin => Bytes(b"s"),
            Pattern::LoopIfBegin => Bytes(b"i"),
            Pattern::EnvCommandBegin => After(b" \t"),
            Pattern::NormalStatementBegin => After(b"!&(;`{| "),
            Pattern::ExtglobBegin => Bytes(b"!*+?@"),
            Pattern::ProcessSubstitutionBegin => Bytes(b"<>"),
            Pattern::StringSingleBegin => Bytes(b"'"),
            Pattern::StringDoubleBegin => Bytes(b"$\""),
            Pattern::EndStatement | Pattern::EndOption => Bytes(b"\n&);`{|}] \t#"),
            Pattern::EndArgument => Bytes(b" \t&;|\n)`"),
            Pattern::EndAlias => Bytes(b" \t;&|\n"),
            Pattern::EndArithmeticDouble => Bytes(b")"),
            Pattern::EndCase => Bytes(b"e"),
            Pattern::EndCasePattern => Bytes(b"e)"),
            Pattern::EndCaseBody => Bytes(b";e"),
            Pattern::EndCommandName => Bytes(b" \t&;|\n)`<"),
            Pattern::EndCommandQuotedDouble => After(b"\""),
            Pattern::EndCommandQuotedSingle => After(b"'"),
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
    /// For [`Pattern::EndLiteral`]: the text that ends the rule.
    pub(crate) end_literal: &'static [u8],
    /// For [`Pattern::EndLiteral`]: the scopes of that text.
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
const PIPELINE: Group = &[Pattern::PipeTime, Pattern::PipeOperator];
const SEP: Group = &[Pattern::Separator];
const LOGICAL_DOUBLE: Group = &[Pattern::LogicalDoubleBegin];
const LOGICAL_SINGLE: Group = &[Pattern::LogicalSingleBegin];
const ASSIGNMENT: Group = &[
    Pattern::ArrayValueBegin,
    Pattern::ModifiedBegin,
    Pattern::NormalAssignBegin,
];
const CASE: Group = &[Pattern::CaseBegin];
const FOR: Group = &[Pattern::ForInBegin, Pattern::ForBegin];
const LOOP: Group = &[
    Pattern::LoopForBegin,
    Pattern::LoopWhileBegin,
    Pattern::LoopSelectBegin,
    Pattern::LoopIfBegin,
];
const FUNCTION: Group = &[Pattern::FunctionBegin];
const LINE_CONTINUATION: Group = &[Pattern::LineContinuation];
const ARITHMETIC_DOUBLE: Group = &[Pattern::ArithmeticDoubleBegin];
const MISC_RANGES: Group = &[
    Pattern::LogicalSingleBegin,
    Pattern::LogicalDoubleBegin,
    Pattern::SubshellBegin,
    Pattern::GroupBegin,
];
const VARIABLE: Group = &[
    Pattern::VariableAt,
    Pattern::VariableDigit,
    Pattern::VariableSpecial,
    Pattern::VariablePositionalBegin,
    Pattern::VariableBraceBegin,
    Pattern::VariableName,
];
const INTERPOLATION: Group = &[Pattern::SubshellBegin, Pattern::BacktickBegin];
const HEREDOC: Group = &[
    Pattern::HeredocQuotedIndentBegin,
    Pattern::HeredocQuotedBegin,
    Pattern::HeredocIndentBegin,
    Pattern::HeredocBegin,
];
const HERESTRING: Group = &[
    Pattern::HerestringSingleBegin,
    Pattern::HerestringDoubleBegin,
    Pattern::HerestringWord,
];
const REDIRECTION: Group = &[Pattern::ProcessSubstitutionBegin, Pattern::RedirectOperator];
const PATHNAME: Group = &[Pattern::Tilde, Pattern::Glob, Pattern::ExtglobBegin];
const FLOATING_KEYWORD: Group = &[Pattern::FloatingKeyword];
const ALIAS: Group = &[Pattern::AliasBegin];
const NORMAL_STATEMENT: Group = &[Pattern::NormalStatementBegin];
const STRING: Group = &[
    Pattern::Escape,
    Pattern::StringSingleBegin,
    Pattern::StringDoubleBegin,
    Pattern::StringAnsiBegin,
];
const SUPPORT: Group = &[Pattern::Support];
const BOOLEAN: Group = &[Pattern::Boolean];
const REDIRECT_NUMBER: Group = &[Pattern::RedirectNumber];
const NUMERIC: Group = &[Pattern::Numeric];
const PAREN: Group = &[Pattern::ParenBegin];
const UNQUOTED: Group = &[Pattern::Unquoted];
const DQ_ESCAPE: Group = &[Pattern::DoubleQuoteEscape];
const ARGUMENT_MATCH: Group = &[Pattern::ArgumentMatch];
const MATH: Group = &[
    Pattern::MathOperator,
    Pattern::MathHex,
    Pattern::MathSemicolon,
    Pattern::MathOctal,
    Pattern::MathOther,
    Pattern::MathInteger,
    Pattern::MathName,
];
const MATH_OPERATORS: Group = &[
    Pattern::MathOperator,
    Pattern::MathHex,
    Pattern::MathOctal,
    Pattern::MathOther,
    Pattern::MathInteger,
];
const CASE_CONTEXT: Group = &[
    Pattern::CaseStar,
    Pattern::CasePlus,
    Pattern::CaseQuestion,
    Pattern::CaseAt,
    Pattern::CasePipe,
    Pattern::Escape,
    Pattern::CaseParenOpen,
    Pattern::CaseGroupBegin,
    Pattern::CaseClassBegin,
];
const OPTION_BEGIN: Group = &[Pattern::OptionBegin];
const ARG_B: Group = &[Pattern::ArgumentBegin];

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
    ARITHMETIC_DOUBLE,
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
    &[Pattern::WhileBegin],
    FUNCTION,
    &[Pattern::CommandStatementBegin],
    LINE_CONTINUATION,
    ARITHMETIC_DOUBLE,
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
    &[Pattern::ArithmeticSingleBegin],
    &[
        Pattern::ArithmeticSingleBegin,
        Pattern::LogicalOperator,
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
        end_literal: b"",
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
        end_literal: lit,
        end_scopes: scopes,
        ..definition(name, content, Some(Pattern::EndLiteral), groups)
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
            Some(Pattern::EndArgument),
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
                &[
                    COMMENT,
                    &[Pattern::ArrayNameEquals, Pattern::ArrayBracketEquals],
                ],
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
                    Pattern::CasePatternBegin,
                    Pattern::CaseBodyBegin,
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
                &[Pattern::CommandNameBegin],
                LINE_CONTINUATION,
                OPTION_BEGIN,
                ARG_B,
                STRING,
                HEREDOC,
            ]],
        ),
        Rule::CommandName => definition(
            scopes![S::MetaStatementCommandName],
            S::NO_SCOPES,
            Some(Pattern::EndCommandName),
            &[&[
                &[Pattern::CommandKeyword, Pattern::CommandBuiltin],
                VARIABLE,
                &[Pattern::CommandName, Pattern::CommandQuotedBegin],
                LINE_CONTINUATION,
                UNQUOTED,
            ]],
        ),
        Rule::CommandQuotedDouble => definition(
            S::NO_SCOPES,
            S::NO_SCOPES,
            Some(Pattern::EndCommandQuotedDouble),
            &[&[&[
                Pattern::ContinuationSingleBegin,
                Pattern::ContinuationDoubleBegin,
            ]]],
        ),
        Rule::CommandQuotedSingle => definition(
            S::NO_SCOPES,
            S::NO_SCOPES,
            Some(Pattern::EndCommandQuotedSingle),
            &[&[&[
                Pattern::ContinuationSingleBegin,
                Pattern::ContinuationDoubleBegin,
            ]]],
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
            &[&[ARITHMETIC_DOUBLE], NORMAL],
        ),
        Rule::Function => RuleDefinition {
            end_last: true,
            ..definition(
                scopes![S::MetaFunction],
                S::NO_SCOPES,
                Some(Pattern::EndFunction),
                &[
                    &[&[
                        Pattern::FunctionBlank,
                        Pattern::FunctionBraceBegin,
                        Pattern::FunctionParenBegin,
                    ]],
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
            &[
                &[&[Pattern::BacktickEscape, Pattern::BacktickCommentBegin]],
                INITIAL,
            ],
        ),
        Rule::BacktickComment => definition(
            S::NO_SCOPES,
            S::NO_SCOPES,
            Some(Pattern::EndBacktickComment),
            &[&[&[Pattern::BacktickCommentLineBegin]]],
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
                &[&[
                    Pattern::ModifiedOption,
                    Pattern::ArrayValueBegin,
                    Pattern::ModifiedMatch,
                ]],
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
                    &[Pattern::NormalAssignBegin, Pattern::EnvCommandBegin],
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
                &[Pattern::CommandNameBegin],
                LINE_CONTINUATION,
                OPTION_BEGIN,
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
                    Pattern::ParameterDigits,
                    Pattern::ParameterName,
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
                    Pattern::ParameterName,
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
                &[LINE_CONTINUATION, MATH_OPERATORS, OPTION_BEGIN, UNQUOTED],
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
                &[Pattern::BooleanArgument],
            ]],
        ),
        Rule::RetokenisedOptions => definition(
            S::NO_SCOPES,
            S::NO_SCOPES,
            None,
            &[&[&[Pattern::ModifiedOption]]],
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
                        .end_literal
                        .first()
                        .filter(|_| pattern == Pattern::EndLiteral)
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
