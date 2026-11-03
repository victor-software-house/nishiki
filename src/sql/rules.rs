//! The patterns of each rule, in the grammar's order.

use super::scope::S;
use super::state::Rule;

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
    pub(crate) fn name(self) -> &'static [S] {
        match self {
            Pat::Variable => &[S::Variable],
            Pat::BracketChar => &[S::BracketChar],
            Pat::Create => &[S::MetaCreate],
            Pat::Drop | Pat::DropTable => &[S::MetaDrop],
            Pat::Alter => &[S::MetaAlter],
            Pat::Modifier => &[S::StorageModifier],
            Pat::Number => &[S::Numeric],
            Pat::Dml => &[S::KwDml],
            Pat::Ddl => &[S::KwDdl],
            Pat::Values => &[S::KwDmlValues],
            Pat::Luw => &[S::KwLuw],
            Pat::Authorization => &[S::KwAuthorization],
            Pat::In => &[S::KwIntegrity],
            Pat::CommentOn => &[S::KwComments],
            Pat::As => &[S::KwAlias],
            Pat::Order => &[S::KwOrder],
            Pat::Star => &[S::OpStar],
            Pat::Comparison => &[S::OpComparison],
            Pat::Math => &[S::OpMath],
            Pat::Concat => &[S::OpConcat],
            Pat::StrSingleFull => &[S::StrSingle],
            Pat::StrBacktickFull => &[S::StrBacktick],
            Pat::StrDoubleFull => &[S::StrDouble],
            Pat::Keyword => &[S::KwOther],
            Pat::Parens => &[S::MetaBlock],
            Pat::Escape => &[S::Escape],
            Pat::Interpolation => &[S::StrInterp],
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
            | Pat::End => &[],
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
