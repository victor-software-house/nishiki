//! The rules the lexer opens.
//! Each is a frame of the interned state.

use super::scope::S;
use crate::scope::{Scopes, scopes};

/// The high byte that marks a frame as one of the SQL lexer's.
const TAG: u16 = crate::tag::SQL;

crate::tag::frame_rules! {
    /// A rule of the grammar that can stay open at the end of a line.
    pub(crate) enum Rule: TAG {
        /// Outside every rule.
        Root,
        /// `[` up to `]`.
        Bracket,
        /// The outer rule of a `--` comment.
        /// It closes right after the comment.
        CommentDash,
        /// The outer rule of a `#` comment.
        /// It holds nothing and closes at once.
        CommentHash,
        /// `--` up to the line feed.
        LineComment,
        /// `/*` up to `*/`.
        /// It nests.
        BlockComment,
        /// `'` up to `'`.
        StringSingle,
        /// `` ` `` up to `` ` ``.
        StringBacktick,
        /// `"` up to `"`.
        StringDouble,
        /// `%{` up to `}`.
        StringBrace,
        /// `%r{` up to `}`.
        StringRegex,
    }
}

impl Rule {
    /// The scopes the rule adds to its begin, its content and its end.
    pub(crate) fn name(self) -> Scopes<S> {
        match self {
            Rule::Bracket => scopes![S::Bracketed],
            Rule::LineComment => scopes![S::CommentDash],
            Rule::BlockComment => scopes![S::CommentBlock],
            Rule::StringSingle => scopes![S::StrSingle],
            Rule::StringBacktick => scopes![S::StrBacktick],
            Rule::StringDouble => scopes![S::StrDouble],
            Rule::StringBrace => scopes![S::StrBraces],
            Rule::StringRegex => scopes![S::StrModr],
            Rule::Root | Rule::CommentDash | Rule::CommentHash => S::NO_SCOPES,
        }
    }
}
