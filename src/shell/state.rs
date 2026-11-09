//! The rules the shell lexer opens.
//! Each is a frame of the interned state.

/// The high byte that marks a frame as one of the shell lexer's.
const TAG: u16 = crate::tag::SHELL;

crate::tag::frame_kinds! {
    /// A rule of the grammar.
    /// It is a rule that is open at the end of a line, or a pseudo rule.
    /// A pseudo rule only lists the patterns a capture is retokenised with.
    pub(crate) enum Rule: TAG {
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
}

impl Rule {
    #[expect(
        clippy::as_conversions,
        reason = "a fieldless `repr(u8)` enum has no `From` impl for its discriminant"
    )]
    pub(crate) const fn index(self) -> usize {
        self as usize
    }

    pub(crate) fn is_heredoc(self) -> bool {
        matches!(
            self,
            Rule::HeredocQuotedIndent | Rule::HeredocQuoted | Rule::HeredocIndent | Rule::Heredoc
        )
    }
}
