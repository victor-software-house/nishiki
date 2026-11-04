//! The patterns of each rule, in the grammar's order.

use super::scope::S;
use super::state::Rule;
use crate::scope::{Scopes, scopes};

/// A pattern of the grammar: a rule that matches text, or one that opens a
/// [`Rule`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Pat {
    // Matches.
    /// `((?<!@)@)\b(\w+)\b`
    Variable,
    /// `\[|\]\]` inside brackets.
    BracketChar,
    /// `create ... table name`
    Create,
    /// `drop ... table`
    Drop,
    /// `drop table name cascade`
    DropTable,
    /// `alter ... table`
    Alter,
    /// The type names.
    Types,
    /// `primary key`, `references` and the like.
    Modifier,
    /// `\b\d+\b`
    Number,
    /// `select`, `from`, `join` and the like.
    Dml,
    /// `on`, `off`, `null`.
    Ddl,
    /// `values`
    Values,
    /// `begin`, `commit` and the like.
    Luw,
    /// `grant` and `revoke`.
    Authorization,
    /// `in`
    In,
    /// `comment on table`
    CommentOn,
    /// `as`
    As,
    /// `asc` and `desc`.
    Order,
    /// `*`
    Star,
    /// `=`, `<>`, `<` and the like.
    Comparison,
    /// `-`, `+` and `/`.
    Math,
    /// `||`
    Concat,
    /// `name(` of a function.
    Function,
    /// `database.table`
    DbTable,
    /// `'...'` closed on the line.
    StrSingleFull,
    /// `` `...` `` closed on the line.
    StrBacktickFull,
    /// `"..."` closed on the line.
    StrDoubleFull,
    /// Every other word of the keyword rule.
    Keyword,
    /// `()`
    Parens,
    /// `\\.` inside a string.
    Escape,
    /// `#{...}` inside a string.
    Interpolation,
    /// The end pattern of the innermost rule.
    End,
    // Begins.
    BracketBegin,
    CommentDashBegin,
    CommentHashBegin,
    LineCommentBegin,
    BlockBegin,
    StrSingleBegin,
    StrBacktickBegin,
    StrDoubleBegin,
    StrBraceBegin,
    StrModrBegin,
}

impl Pat {
    /// The rule the pattern opens, or `None` for a pattern that matches text
    /// and is done.
    pub(crate) fn opens(self) -> Option<Rule> {
        match self {
            Pat::BracketBegin => Some(Rule::Bracket),
            Pat::CommentDashBegin => Some(Rule::CommentDash),
            Pat::CommentHashBegin => Some(Rule::CommentHash),
            Pat::LineCommentBegin => Some(Rule::LineComment),
            Pat::BlockBegin => Some(Rule::BlockComment),
            Pat::StrSingleBegin => Some(Rule::StrSingle),
            Pat::StrBacktickBegin => Some(Rule::StrBacktick),
            Pat::StrDoubleBegin => Some(Rule::StrDouble),
            Pat::StrBraceBegin => Some(Rule::StrBrace),
            Pat::StrModrBegin => Some(Rule::StrModr),
            Pat::Variable
            | Pat::BracketChar
            | Pat::Create
            | Pat::Drop
            | Pat::DropTable
            | Pat::Alter
            | Pat::Types
            | Pat::Modifier
            | Pat::Number
            | Pat::Dml
            | Pat::Ddl
            | Pat::Values
            | Pat::Luw
            | Pat::Authorization
            | Pat::In
            | Pat::CommentOn
            | Pat::As
            | Pat::Order
            | Pat::Star
            | Pat::Comparison
            | Pat::Math
            | Pat::Concat
            | Pat::Function
            | Pat::DbTable
            | Pat::StrSingleFull
            | Pat::StrBacktickFull
            | Pat::StrDoubleFull
            | Pat::Keyword
            | Pat::Parens
            | Pat::Escape
            | Pat::Interpolation
            | Pat::End => None,
        }
    }

    /// The scopes a pattern that matches text gives the whole match.
    pub(crate) fn name(self) -> Scopes<S> {
        match self {
            Pat::Variable => scopes![S::Variable],
            Pat::BracketChar => scopes![S::BracketChar],
            Pat::Create => scopes![S::MetaCreate],
            Pat::Drop | Pat::DropTable => scopes![S::MetaDrop],
            Pat::Alter => scopes![S::MetaAlter],
            Pat::Modifier => scopes![S::StorageModifier],
            Pat::Number => scopes![S::Numeric],
            Pat::Dml => scopes![S::KwDml],
            Pat::Ddl => scopes![S::KwDdl],
            Pat::Values => scopes![S::KwDmlValues],
            Pat::Luw => scopes![S::KwLuw],
            Pat::Authorization => scopes![S::KwAuthorization],
            Pat::In => scopes![S::KwIntegrity],
            Pat::CommentOn => scopes![S::KwComments],
            Pat::As => scopes![S::KwAlias],
            Pat::Order => scopes![S::KwOrder],
            Pat::Star => scopes![S::OpStar],
            Pat::Comparison => scopes![S::OpComparison],
            Pat::Math => scopes![S::OpMath],
            Pat::Concat => scopes![S::OpConcat],
            Pat::StrSingleFull => scopes![S::StrSingle],
            Pat::StrBacktickFull => scopes![S::StrBacktick],
            Pat::StrDoubleFull => scopes![S::StrDouble],
            Pat::Keyword => scopes![S::KwOther],
            Pat::Parens => scopes![S::MetaBlock],
            Pat::Escape => scopes![S::Escape],
            Pat::Interpolation => scopes![S::StrInterp],
            Pat::Types
            | Pat::Function
            | Pat::DbTable
            | Pat::BracketBegin
            | Pat::CommentDashBegin
            | Pat::CommentHashBegin
            | Pat::LineCommentBegin
            | Pat::BlockBegin
            | Pat::StrSingleBegin
            | Pat::StrBacktickBegin
            | Pat::StrDoubleBegin
            | Pat::StrBraceBegin
            | Pat::StrModrBegin
            | Pat::End => S::NO_SCOPES,
        }
    }
}

/// The patterns a rule tries, after its end pattern (before it, in a bracket).
pub(crate) fn patterns(rule: Rule) -> &'static [Pat] {
    match rule {
        Rule::Root => &[
            Pat::Variable,
            Pat::BracketBegin,
            Pat::CommentDashBegin,
            Pat::CommentHashBegin,
            Pat::BlockBegin,
            Pat::Create,
            Pat::Drop,
            Pat::DropTable,
            Pat::Alter,
            Pat::Types,
            Pat::Modifier,
            Pat::Number,
            Pat::Dml,
            Pat::Ddl,
            Pat::Values,
            Pat::Luw,
            Pat::Authorization,
            Pat::In,
            Pat::CommentOn,
            Pat::As,
            Pat::Order,
            Pat::Star,
            Pat::Comparison,
            Pat::Math,
            Pat::Concat,
            Pat::Function,
            Pat::DbTable,
            Pat::StrSingleFull,
            Pat::StrSingleBegin,
            Pat::StrBacktickFull,
            Pat::StrBacktickBegin,
            Pat::StrDoubleFull,
            Pat::StrDoubleBegin,
            Pat::StrBraceBegin,
            Pat::StrModrBegin,
            Pat::Keyword,
            Pat::Parens,
        ],
        Rule::Bracket => &[Pat::BracketChar],
        Rule::CommentDash => &[Pat::LineCommentBegin],
        Rule::BlockComment => &[Pat::BlockBegin],
        Rule::StrSingle | Rule::StrBacktick => &[Pat::Escape],
        Rule::StrDouble | Rule::StrBrace | Rule::StrModr => &[Pat::Interpolation],
        Rule::CommentHash | Rule::LineComment => &[],
    }
}
