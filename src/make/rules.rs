//! The patterns of each rule, in the grammar's order.

use super::scope::S;
use super::state::Rule;

/// A pattern of the grammar: a rule that matches text, or one that opens a
/// [`Rule`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Pat {
    // Matches.
    /// `\$[^(){}]`
    SimpleVar,
    /// `,`
    Comma,
    /// `%`
    Percent,
    /// `%|\*`
    PercentStar,
    /// `\\\n`
    Continuation,
    /// `[^\s]+`
    AnyWord,
    /// The builtin variable names inside `$(`.
    BuiltinParen,
    /// The builtin variable names inside `${`.
    BuiltinBrace,
    /// `^\s*(\.(PHONY|...))\s*$`
    SpecialTarget,
    // Begins.
    Comment,
    CommentHash,
    VarParen,
    VarBrace,
    FuncParen,
    FuncBrace,
    FlavorParen,
    FlavorBrace,
    AnotherParen,
    AnotherBrace,
    InterpParen,
    InterpBrace,
    Include,
    Vpath,
    Define,
    DefineHead,
    Export,
    OverridePrivate,
    Unexport,
    Conditional,
    Condition,
    Else,
    Target,
    Prereq,
    TargetName,
    Recipe,
    Assign,
}

impl Pat {
    /// The rule the pattern opens, or `None` for a pattern that matches text
    /// and is done.
    pub(crate) fn opens(self) -> Option<Rule> {
        match self {
            Pat::SimpleVar
            | Pat::Comma
            | Pat::Percent
            | Pat::PercentStar
            | Pat::Continuation
            | Pat::AnyWord
            | Pat::BuiltinParen
            | Pat::BuiltinBrace
            | Pat::SpecialTarget => None,
            Pat::Comment => Some(Rule::Comment),
            Pat::CommentHash => Some(Rule::CommentHash),
            Pat::VarParen => Some(Rule::VarParen),
            Pat::VarBrace => Some(Rule::VarBrace),
            Pat::FuncParen => Some(Rule::FuncParen),
            Pat::FuncBrace => Some(Rule::FuncBrace),
            Pat::FlavorParen => Some(Rule::FlavorParen),
            Pat::FlavorBrace => Some(Rule::FlavorBrace),
            Pat::AnotherParen => Some(Rule::AnotherParen),
            Pat::AnotherBrace => Some(Rule::AnotherBrace),
            Pat::InterpParen => Some(Rule::InterpParen),
            Pat::InterpBrace => Some(Rule::InterpBrace),
            Pat::Include => Some(Rule::Include),
            Pat::Vpath => Some(Rule::Vpath),
            Pat::Define => Some(Rule::Define),
            Pat::DefineHead => Some(Rule::DefineHead),
            Pat::Export => Some(Rule::Export),
            Pat::OverridePrivate => Some(Rule::OverridePrivate),
            Pat::Unexport => Some(Rule::Unexport),
            Pat::Conditional => Some(Rule::Conditional),
            Pat::Condition => Some(Rule::Condition),
            Pat::Else => Some(Rule::Else),
            Pat::Target => Some(Rule::Target),
            Pat::Prereq => Some(Rule::Prereq),
            Pat::TargetName => Some(Rule::TargetName),
            Pat::Recipe => Some(Rule::Recipe),
            Pat::Assign => Some(Rule::Assign),
        }
    }

    /// The scopes a pattern that matches text gives the whole match.
    pub(crate) fn name(self) -> &'static [S] {
        match self {
            Pat::SimpleVar | Pat::BuiltinParen | Pat::BuiltinBrace => &[S::VarLanguage],
            Pat::Comma => &[S::Comma],
            Pat::Percent | Pat::PercentStar => &[S::Placeholder],
            Pat::Continuation => &[S::Continuation],
            Pat::AnyWord => &[S::VarOther],
            Pat::SpecialTarget
            | Pat::Comment
            | Pat::CommentHash
            | Pat::VarParen
            | Pat::VarBrace
            | Pat::FuncParen
            | Pat::FuncBrace
            | Pat::FlavorParen
            | Pat::FlavorBrace
            | Pat::AnotherParen
            | Pat::AnotherBrace
            | Pat::InterpParen
            | Pat::InterpBrace
            | Pat::Include
            | Pat::Vpath
            | Pat::Define
            | Pat::DefineHead
            | Pat::Export
            | Pat::OverridePrivate
            | Pat::Unexport
            | Pat::Conditional
            | Pat::Condition
            | Pat::Else
            | Pat::Target
            | Pat::Prereq
            | Pat::TargetName
            | Pat::Recipe
            | Pat::Assign => &[],
        }
    }
}

/// `#variables`
const VARS: [Pat; 3] = [Pat::SimpleVar, Pat::VarParen, Pat::VarBrace];

/// The grammar's top-level patterns, which `$self` includes.
const TOP: &[Pat] = &[
    Pat::Comment,
    Pat::SimpleVar,
    Pat::VarParen,
    Pat::VarBrace,
    Pat::Assign,
    Pat::Include,
    Pat::Vpath,
    Pat::Define,
    Pat::Export,
    Pat::OverridePrivate,
    Pat::Unexport,
    Pat::Conditional,
    Pat::Recipe,
    Pat::Target,
];

const INCLUDE: &[Pat] = &[
    Pat::Comment,
    Pat::SimpleVar,
    Pat::VarParen,
    Pat::VarBrace,
    Pat::Percent,
];

const DEFINE: &[Pat] = &[
    Pat::DefineHead,
    Pat::SimpleVar,
    Pat::VarParen,
    Pat::VarBrace,
    Pat::Include,
    Pat::Vpath,
    Pat::Define,
    Pat::Export,
    Pat::OverridePrivate,
    Pat::Unexport,
    Pat::Conditional,
];

const CONDITIONAL: &[Pat] = &[
    Pat::Condition,
    Pat::Else,
    Pat::Comment,
    Pat::SimpleVar,
    Pat::VarParen,
    Pat::VarBrace,
    Pat::Assign,
    Pat::Include,
    Pat::Vpath,
    Pat::Define,
    Pat::Export,
    Pat::OverridePrivate,
    Pat::Unexport,
    Pat::Conditional,
    Pat::Recipe,
    Pat::Target,
];

const CONDITION: &[Pat] = &[
    Pat::Comma,
    Pat::SimpleVar,
    Pat::VarParen,
    Pat::VarBrace,
    Pat::Comment,
];

const PREREQ: &[Pat] = &[
    Pat::Continuation,
    Pat::PercentStar,
    Pat::Comment,
    Pat::SimpleVar,
    Pat::VarParen,
    Pat::VarBrace,
];

const TARGET_NAME: &[Pat] = &[Pat::SimpleVar, Pat::VarParen, Pat::VarBrace, Pat::Percent];

const RECIPE: &[Pat] = &[
    Pat::Continuation,
    Pat::SimpleVar,
    Pat::VarParen,
    Pat::VarBrace,
];

const ASSIGN: &[Pat] = &[
    Pat::Continuation,
    Pat::Comment,
    Pat::SimpleVar,
    Pat::VarParen,
    Pat::VarBrace,
];

const VAR_PAREN: &[Pat] = &[
    Pat::SimpleVar,
    Pat::VarParen,
    Pat::VarBrace,
    Pat::BuiltinParen,
    Pat::FuncParen,
    Pat::FlavorParen,
    Pat::AnotherParen,
];

const VAR_BRACE: &[Pat] = &[
    Pat::SimpleVar,
    Pat::VarParen,
    Pat::VarBrace,
    Pat::BuiltinBrace,
    Pat::FuncBrace,
    Pat::FlavorBrace,
    Pat::AnotherBrace,
];

const FUNC: &[Pat] = &[
    Pat::Comma,
    Pat::SimpleVar,
    Pat::VarParen,
    Pat::VarBrace,
    Pat::InterpParen,
    Pat::InterpBrace,
    Pat::PercentStar,
    Pat::Continuation,
];

const ANOTHER: &[Pat] = &[
    Pat::SimpleVar,
    Pat::VarParen,
    Pat::VarBrace,
    Pat::Continuation,
];

const INTERP: &[Pat] = &[
    Pat::SimpleVar,
    Pat::VarParen,
    Pat::VarBrace,
    Pat::InterpParen,
    Pat::InterpBrace,
];

/// The patterns a rule tries, after its end pattern.
pub(crate) fn patterns(rule: Rule) -> &'static [Pat] {
    match rule {
        Rule::Root => TOP,
        Rule::Comment => &[Pat::CommentHash],
        Rule::CommentHash => &[Pat::Continuation],
        Rule::Include | Rule::Vpath => INCLUDE,
        Rule::Define => DEFINE,
        Rule::DefineHead => &[Pat::Comment],
        Rule::Export => &[Pat::Comment, Pat::Assign, Pat::AnyWord],
        Rule::OverridePrivate => &[Pat::Comment, Pat::Assign],
        Rule::Unexport => &[Pat::Comment, Pat::AnyWord],
        Rule::Conditional => CONDITIONAL,
        Rule::Condition | Rule::Else => CONDITION,
        Rule::Target => &[Pat::Prereq],
        Rule::Prereq => PREREQ,
        Rule::TargetName => TARGET_NAME,
        Rule::Recipe => RECIPE,
        Rule::Assign => ASSIGN,
        Rule::VarParen => VAR_PAREN,
        Rule::VarBrace => VAR_BRACE,
        Rule::FuncParen | Rule::FuncBrace => FUNC,
        Rule::FlavorParen | Rule::FlavorBrace | Rule::RetokVars => &VARS,
        Rule::AnotherParen | Rule::AnotherBrace => ANOTHER,
        Rule::InterpParen | Rule::InterpBrace => INTERP,
        Rule::RetokTarget => &[Pat::SpecialTarget, Pat::TargetName],
    }
}
