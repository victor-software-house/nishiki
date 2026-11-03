//! The rules the lexer opens, which are frames of the interned state.

use super::scope::S;

/// The high byte that marks a frame as one of the SQL lexer's.
const TAG: u16 = crate::tag::SQL;

crate::tag::frame_kinds! {
    /// A rule of the grammar that can stay open at the end of a line.
    pub(crate) enum Rule: TAG {
        /// Outside every rule.
        Root,
        /// `[` up to `]`.
        Bracket,
        /// The outer rule of a `--` comment, which closes right after it.
        CommentDash,
        /// The outer rule of a `#` comment, which holds nothing and closes at
        /// once.
        CommentHash,
        /// `--` up to the line feed.
        LineComment,
        /// `/*` up to `*/`, which nests.
        BlockComment,
        /// `'` up to `'`.
        StrSingle,
        /// `` ` `` up to `` ` ``.
        StrBacktick,
        /// `"` up to `"`.
        StrDouble,
        /// `%{` up to `}`.
        StrBrace,
        /// `%r{` up to `}`.
        StrModr,
    }
}

impl Rule {
    /// The scopes the rule adds to its begin, its content and its end.
    pub(crate) fn name(self) -> &'static [S] {
        match self {
            Rule::Bracket => &[S::Bracketed],
            Rule::LineComment => &[S::CommentDash],
            Rule::BlockComment => &[S::CommentBlock],
            Rule::StrSingle => &[S::StrSingle],
            Rule::StrBacktick => &[S::StrBacktick],
            Rule::StrDouble => &[S::StrDouble],
            Rule::StrBrace => &[S::StrBraces],
            Rule::StrModr => &[S::StrModr],
            Rule::Root | Rule::CommentDash | Rule::CommentHash => &[],
        }
    }
}
