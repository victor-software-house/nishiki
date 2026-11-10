//! The patterns of each rule, in the grammar's order.

use super::scope::S;
use super::state::Rule;
use crate::scope::{Scopes, scopes};

/// A pattern of the grammar: a rule that matches text, or one that opens a
/// [`Rule`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Pattern {
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

impl Pattern {
    /// The rule the pattern opens, or `None` for a pattern that matches text
    /// and is done.
    pub(crate) fn opens(self) -> Option<Rule> {
        match self {
            Pattern::BracketBegin => Some(Rule::Bracket),
            Pattern::CommentDashBegin => Some(Rule::CommentDash),
            Pattern::CommentHashBegin => Some(Rule::CommentHash),
            Pattern::LineCommentBegin => Some(Rule::LineComment),
            Pattern::BlockBegin => Some(Rule::BlockComment),
            Pattern::StrSingleBegin => Some(Rule::StrSingle),
            Pattern::StrBacktickBegin => Some(Rule::StrBacktick),
            Pattern::StrDoubleBegin => Some(Rule::StrDouble),
            Pattern::StrBraceBegin => Some(Rule::StrBrace),
            Pattern::StrModrBegin => Some(Rule::StrModr),
            Pattern::Variable
            | Pattern::BracketChar
            | Pattern::Create
            | Pattern::Drop
            | Pattern::DropTable
            | Pattern::Alter
            | Pattern::Types
            | Pattern::Modifier
            | Pattern::Number
            | Pattern::Dml
            | Pattern::Ddl
            | Pattern::Values
            | Pattern::Luw
            | Pattern::Authorization
            | Pattern::In
            | Pattern::CommentOn
            | Pattern::As
            | Pattern::Order
            | Pattern::Star
            | Pattern::Comparison
            | Pattern::Math
            | Pattern::Concat
            | Pattern::Function
            | Pattern::DbTable
            | Pattern::StrSingleFull
            | Pattern::StrBacktickFull
            | Pattern::StrDoubleFull
            | Pattern::Keyword
            | Pattern::Parens
            | Pattern::Escape
            | Pattern::Interpolation
            | Pattern::End => None,
        }
    }

    /// The scopes a pattern that matches text gives the whole match.
    pub(crate) fn name(self) -> Scopes<S> {
        match self {
            Pattern::Variable => scopes![S::Variable],
            Pattern::BracketChar => scopes![S::BracketChar],
            Pattern::Create => scopes![S::MetaCreate],
            Pattern::Drop | Pattern::DropTable => scopes![S::MetaDrop],
            Pattern::Alter => scopes![S::MetaAlter],
            Pattern::Modifier => scopes![S::StorageModifier],
            Pattern::Number => scopes![S::Numeric],
            Pattern::Dml => scopes![S::KwDml],
            Pattern::Ddl => scopes![S::KwDdl],
            Pattern::Values => scopes![S::KwDmlValues],
            Pattern::Luw => scopes![S::KwLuw],
            Pattern::Authorization => scopes![S::KwAuthorization],
            Pattern::In => scopes![S::KwIntegrity],
            Pattern::CommentOn => scopes![S::KwComments],
            Pattern::As => scopes![S::KwAlias],
            Pattern::Order => scopes![S::KwOrder],
            Pattern::Star => scopes![S::OpStar],
            Pattern::Comparison => scopes![S::OpComparison],
            Pattern::Math => scopes![S::OpMath],
            Pattern::Concat => scopes![S::OpConcat],
            Pattern::StrSingleFull => scopes![S::StrSingle],
            Pattern::StrBacktickFull => scopes![S::StrBacktick],
            Pattern::StrDoubleFull => scopes![S::StrDouble],
            Pattern::Keyword => scopes![S::KwOther],
            Pattern::Parens => scopes![S::MetaBlock],
            Pattern::Escape => scopes![S::Escape],
            Pattern::Interpolation => scopes![S::StrInterp],
            Pattern::Types
            | Pattern::Function
            | Pattern::DbTable
            | Pattern::BracketBegin
            | Pattern::CommentDashBegin
            | Pattern::CommentHashBegin
            | Pattern::LineCommentBegin
            | Pattern::BlockBegin
            | Pattern::StrSingleBegin
            | Pattern::StrBacktickBegin
            | Pattern::StrDoubleBegin
            | Pattern::StrBraceBegin
            | Pattern::StrModrBegin
            | Pattern::End => S::NO_SCOPES,
        }
    }
}

/// The patterns a rule tries, after its end pattern (before it, in a bracket).
pub(crate) fn patterns(rule: Rule) -> &'static [Pattern] {
    match rule {
        Rule::Root => &[
            Pattern::Variable,
            Pattern::BracketBegin,
            Pattern::CommentDashBegin,
            Pattern::CommentHashBegin,
            Pattern::BlockBegin,
            Pattern::Create,
            Pattern::Drop,
            Pattern::DropTable,
            Pattern::Alter,
            Pattern::Types,
            Pattern::Modifier,
            Pattern::Number,
            Pattern::Dml,
            Pattern::Ddl,
            Pattern::Values,
            Pattern::Luw,
            Pattern::Authorization,
            Pattern::In,
            Pattern::CommentOn,
            Pattern::As,
            Pattern::Order,
            Pattern::Star,
            Pattern::Comparison,
            Pattern::Math,
            Pattern::Concat,
            Pattern::Function,
            Pattern::DbTable,
            Pattern::StrSingleFull,
            Pattern::StrSingleBegin,
            Pattern::StrBacktickFull,
            Pattern::StrBacktickBegin,
            Pattern::StrDoubleFull,
            Pattern::StrDoubleBegin,
            Pattern::StrBraceBegin,
            Pattern::StrModrBegin,
            Pattern::Keyword,
            Pattern::Parens,
        ],
        Rule::Bracket => &[Pattern::BracketChar],
        Rule::CommentDash => &[Pattern::LineCommentBegin],
        Rule::BlockComment => &[Pattern::BlockBegin],
        Rule::StrSingle | Rule::StrBacktick => &[Pattern::Escape],
        Rule::StrDouble | Rule::StrBrace | Rule::StrModr => &[Pattern::Interpolation],
        Rule::CommentHash | Rule::LineComment => &[],
    }
}
