//! The patterns of each rule, in the grammar's order.

use super::scope::S;
use super::state::Rule;
use crate::scope::{Scopes, scopes};

/// A pattern of the grammar: a rule that matches text, or one that opens a
/// [`Rule`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Pattern {
    /// The end pattern of the innermost rule.
    /// `Lexer::matcher` answers it by rule.
    End,
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

impl Pattern {
    /// The rule the pattern opens, or `None` for a pattern that matches text
    /// and is done.
    pub(crate) fn opens(self) -> Option<Rule> {
        match self {
            Pattern::End
            | Pattern::SimpleVar
            | Pattern::Comma
            | Pattern::Percent
            | Pattern::PercentStar
            | Pattern::Continuation
            | Pattern::AnyWord
            | Pattern::BuiltinParen
            | Pattern::BuiltinBrace
            | Pattern::SpecialTarget => None,
            Pattern::Comment => Some(Rule::Comment),
            Pattern::CommentHash => Some(Rule::CommentHash),
            Pattern::VarParen => Some(Rule::VarParen),
            Pattern::VarBrace => Some(Rule::VarBrace),
            Pattern::FuncParen => Some(Rule::FuncParen),
            Pattern::FuncBrace => Some(Rule::FuncBrace),
            Pattern::FlavorParen => Some(Rule::FlavorParen),
            Pattern::FlavorBrace => Some(Rule::FlavorBrace),
            Pattern::AnotherParen => Some(Rule::AnotherParen),
            Pattern::AnotherBrace => Some(Rule::AnotherBrace),
            Pattern::InterpParen => Some(Rule::InterpParen),
            Pattern::InterpBrace => Some(Rule::InterpBrace),
            Pattern::Include => Some(Rule::Include),
            Pattern::Vpath => Some(Rule::Vpath),
            Pattern::Define => Some(Rule::Define),
            Pattern::DefineHead => Some(Rule::DefineHead),
            Pattern::Export => Some(Rule::Export),
            Pattern::OverridePrivate => Some(Rule::OverridePrivate),
            Pattern::Unexport => Some(Rule::Unexport),
            Pattern::Conditional => Some(Rule::Conditional),
            Pattern::Condition => Some(Rule::Condition),
            Pattern::Else => Some(Rule::Else),
            Pattern::Target => Some(Rule::Target),
            Pattern::Prereq => Some(Rule::Prereq),
            Pattern::TargetName => Some(Rule::TargetName),
            Pattern::Recipe => Some(Rule::Recipe),
            Pattern::Assign => Some(Rule::Assign),
        }
    }

    /// The scopes a pattern that matches text gives the whole match.
    pub(crate) fn name(self) -> Scopes<S> {
        match self {
            Pattern::SimpleVar | Pattern::BuiltinParen | Pattern::BuiltinBrace => {
                scopes![S::VarLanguage]
            }
            Pattern::Comma => scopes![S::Comma],
            Pattern::Percent | Pattern::PercentStar => scopes![S::Placeholder],
            Pattern::Continuation => scopes![S::Continuation],
            Pattern::AnyWord => scopes![S::VarOther],
            Pattern::End
            | Pattern::SpecialTarget
            | Pattern::Comment
            | Pattern::CommentHash
            | Pattern::VarParen
            | Pattern::VarBrace
            | Pattern::FuncParen
            | Pattern::FuncBrace
            | Pattern::FlavorParen
            | Pattern::FlavorBrace
            | Pattern::AnotherParen
            | Pattern::AnotherBrace
            | Pattern::InterpParen
            | Pattern::InterpBrace
            | Pattern::Include
            | Pattern::Vpath
            | Pattern::Define
            | Pattern::DefineHead
            | Pattern::Export
            | Pattern::OverridePrivate
            | Pattern::Unexport
            | Pattern::Conditional
            | Pattern::Condition
            | Pattern::Else
            | Pattern::Target
            | Pattern::Prereq
            | Pattern::TargetName
            | Pattern::Recipe
            | Pattern::Assign => S::NO_SCOPES,
        }
    }
}

/// `#variables`
const VARS: [Pattern; 3] = [Pattern::SimpleVar, Pattern::VarParen, Pattern::VarBrace];

/// The grammar's top-level patterns.
/// `$self` includes them.
const TOP: &[Pattern] = &[
    Pattern::Comment,
    Pattern::SimpleVar,
    Pattern::VarParen,
    Pattern::VarBrace,
    Pattern::Assign,
    Pattern::Include,
    Pattern::Vpath,
    Pattern::Define,
    Pattern::Export,
    Pattern::OverridePrivate,
    Pattern::Unexport,
    Pattern::Conditional,
    Pattern::Recipe,
    Pattern::Target,
];

const INCLUDE: &[Pattern] = &[
    Pattern::Comment,
    Pattern::SimpleVar,
    Pattern::VarParen,
    Pattern::VarBrace,
    Pattern::Percent,
];

const DEFINE: &[Pattern] = &[
    Pattern::DefineHead,
    Pattern::SimpleVar,
    Pattern::VarParen,
    Pattern::VarBrace,
    Pattern::Include,
    Pattern::Vpath,
    Pattern::Define,
    Pattern::Export,
    Pattern::OverridePrivate,
    Pattern::Unexport,
    Pattern::Conditional,
];

const CONDITIONAL: &[Pattern] = &[
    Pattern::Condition,
    Pattern::Else,
    Pattern::Comment,
    Pattern::SimpleVar,
    Pattern::VarParen,
    Pattern::VarBrace,
    Pattern::Assign,
    Pattern::Include,
    Pattern::Vpath,
    Pattern::Define,
    Pattern::Export,
    Pattern::OverridePrivate,
    Pattern::Unexport,
    Pattern::Conditional,
    Pattern::Recipe,
    Pattern::Target,
];

const CONDITION: &[Pattern] = &[
    Pattern::Comma,
    Pattern::SimpleVar,
    Pattern::VarParen,
    Pattern::VarBrace,
    Pattern::Comment,
];

const PREREQ: &[Pattern] = &[
    Pattern::Continuation,
    Pattern::PercentStar,
    Pattern::Comment,
    Pattern::SimpleVar,
    Pattern::VarParen,
    Pattern::VarBrace,
];

const TARGET_NAME: &[Pattern] = &[
    Pattern::SimpleVar,
    Pattern::VarParen,
    Pattern::VarBrace,
    Pattern::Percent,
];

const RECIPE: &[Pattern] = &[
    Pattern::Continuation,
    Pattern::SimpleVar,
    Pattern::VarParen,
    Pattern::VarBrace,
];

const ASSIGN: &[Pattern] = &[
    Pattern::Continuation,
    Pattern::Comment,
    Pattern::SimpleVar,
    Pattern::VarParen,
    Pattern::VarBrace,
];

const VAR_PAREN: &[Pattern] = &[
    Pattern::SimpleVar,
    Pattern::VarParen,
    Pattern::VarBrace,
    Pattern::BuiltinParen,
    Pattern::FuncParen,
    Pattern::FlavorParen,
    Pattern::AnotherParen,
];

const VAR_BRACE: &[Pattern] = &[
    Pattern::SimpleVar,
    Pattern::VarParen,
    Pattern::VarBrace,
    Pattern::BuiltinBrace,
    Pattern::FuncBrace,
    Pattern::FlavorBrace,
    Pattern::AnotherBrace,
];

const FUNC: &[Pattern] = &[
    Pattern::Comma,
    Pattern::SimpleVar,
    Pattern::VarParen,
    Pattern::VarBrace,
    Pattern::InterpParen,
    Pattern::InterpBrace,
    Pattern::PercentStar,
    Pattern::Continuation,
];

const ANOTHER: &[Pattern] = &[
    Pattern::SimpleVar,
    Pattern::VarParen,
    Pattern::VarBrace,
    Pattern::Continuation,
];

const INTERP: &[Pattern] = &[
    Pattern::SimpleVar,
    Pattern::VarParen,
    Pattern::VarBrace,
    Pattern::InterpParen,
    Pattern::InterpBrace,
];

/// The patterns a rule tries, after its end pattern.
pub(crate) fn patterns(rule: Rule) -> &'static [Pattern] {
    match rule {
        Rule::Root => TOP,
        Rule::Comment => &[Pattern::CommentHash],
        Rule::CommentHash => &[Pattern::Continuation],
        Rule::Include | Rule::Vpath => INCLUDE,
        Rule::Define => DEFINE,
        Rule::DefineHead => &[Pattern::Comment],
        Rule::Export => &[Pattern::Comment, Pattern::Assign, Pattern::AnyWord],
        Rule::OverridePrivate => &[Pattern::Comment, Pattern::Assign],
        Rule::Unexport => &[Pattern::Comment, Pattern::AnyWord],
        Rule::Conditional => CONDITIONAL,
        Rule::Condition | Rule::Else => CONDITION,
        Rule::Target => &[Pattern::Prereq],
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
        Rule::RetokTarget => &[Pattern::SpecialTarget, Pattern::TargetName],
    }
}
