//! The rules the shell lexer opens, which are frames of the interned state.

/// The high byte that marks a frame as one of the shell lexer's.
const TAG: u16 = 0x0400;

/// A rule of the grammar that is open at the end of a line, or a pseudo rule
/// that only lists the patterns a capture is retokenised with.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub(crate) enum Rule {
    /// Outside any rule.
    Root,
    Alias,
    Argument,
    ArithDouble,
    ArithSingle,
    ArrayValue,
    Case,
    CasePattern,
    CaseBody,
    CaseGroup,
    CaseClass,
    CommandStatement,
    CommandName,
    CommandQuotedDouble,
    CommandQuotedSingle,
    ContinuationDouble,
    ContinuationSingle,
    ForIn,
    For,
    Function,
    FunctionBrace,
    FunctionParen,
    HeredocQuotedIndent,
    HeredocQuoted,
    HeredocIndent,
    Heredoc,
    HerestringSingle,
    HerestringDouble,
    Backtick,
    BacktickComment,
    BacktickCommentLine,
    LogicalDouble,
    LogicalSingle,
    LoopFor,
    LoopWhile,
    LoopSelect,
    LoopIf,
    Group,
    Modified,
    NormalAssign,
    EnvCommand,
    NormalStatement,
    Option,
    Paren,
    Extglob,
    ProcessSubstitution,
    StringSingle,
    StringDouble,
    StringAnsi,
    Subshell,
    VarPositional,
    VarBrace,
    While,
    /// A capture of an argument, retokenised.
    RetokArgument,
    /// The options of an `alias`, retokenised.
    RetokOptions,
    /// A capture retokenised with the whole top-level rule list.
    RetokInitial,
    /// A capture retokenised with the string rules.
    RetokString,
    /// The `[…]` of an expansion, retokenised.
    RetokArrayAccess,
    /// The rest of a line after a here-document delimiter, retokenised.
    RetokHeredocRest,
}

impl Rule {
    /// Every rule by discriminant.
    pub(crate) const ALL: [Rule; 59] = [
        Rule::Root,
        Rule::Alias,
        Rule::Argument,
        Rule::ArithDouble,
        Rule::ArithSingle,
        Rule::ArrayValue,
        Rule::Case,
        Rule::CasePattern,
        Rule::CaseBody,
        Rule::CaseGroup,
        Rule::CaseClass,
        Rule::CommandStatement,
        Rule::CommandName,
        Rule::CommandQuotedDouble,
        Rule::CommandQuotedSingle,
        Rule::ContinuationDouble,
        Rule::ContinuationSingle,
        Rule::ForIn,
        Rule::For,
        Rule::Function,
        Rule::FunctionBrace,
        Rule::FunctionParen,
        Rule::HeredocQuotedIndent,
        Rule::HeredocQuoted,
        Rule::HeredocIndent,
        Rule::Heredoc,
        Rule::HerestringSingle,
        Rule::HerestringDouble,
        Rule::Backtick,
        Rule::BacktickComment,
        Rule::BacktickCommentLine,
        Rule::LogicalDouble,
        Rule::LogicalSingle,
        Rule::LoopFor,
        Rule::LoopWhile,
        Rule::LoopSelect,
        Rule::LoopIf,
        Rule::Group,
        Rule::Modified,
        Rule::NormalAssign,
        Rule::EnvCommand,
        Rule::NormalStatement,
        Rule::Option,
        Rule::Paren,
        Rule::Extglob,
        Rule::ProcessSubstitution,
        Rule::StringSingle,
        Rule::StringDouble,
        Rule::StringAnsi,
        Rule::Subshell,
        Rule::VarPositional,
        Rule::VarBrace,
        Rule::While,
        Rule::RetokArgument,
        Rule::RetokOptions,
        Rule::RetokInitial,
        Rule::RetokString,
        Rule::RetokArrayAccess,
        Rule::RetokHeredocRest,
    ];

    /// The rule as a frame stores it: the shell tag in the high byte, the
    /// discriminant in the low one.
    #[expect(
        clippy::as_conversions,
        reason = "a fieldless `repr(u8)` enum has no `From` impl for its discriminant"
    )]
    pub(crate) const fn code(self) -> u16 {
        TAG | self as u16
    }

    #[expect(
        clippy::as_conversions,
        reason = "a fieldless `repr(u8)` enum has no `From` impl for its discriminant"
    )]
    pub(crate) const fn index(self) -> usize {
        self as usize
    }

    /// The rule a frame's code names; [`Rule::Root`] for a code that is not
    /// the shell's, such as the empty state's.
    pub(crate) fn from_code(code: u16) -> Rule {
        if code & 0xff00 != TAG {
            return Rule::Root;
        }
        Rule::ALL
            .get(usize::from(code & 0xff))
            .copied()
            .unwrap_or(Rule::Root)
    }

    pub(crate) fn is_heredoc(self) -> bool {
        matches!(
            self,
            Rule::HeredocQuotedIndent | Rule::HeredocQuoted | Rule::HeredocIndent | Rule::Heredoc
        )
    }
}
