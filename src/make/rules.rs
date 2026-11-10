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
    VariableParen,
    VariableBrace,
    FunctionParen,
    FunctionBrace,
    FlavorParen,
    FlavorBrace,
    VariableNameParen,
    VariableNameBrace,
    NestedParen,
    NestedBrace,
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
    Prerequisites,
    TargetName,
    Recipe,
    Assignment,
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
            Pattern::VariableParen => Some(Rule::VariableParen),
            Pattern::VariableBrace => Some(Rule::VariableBrace),
            Pattern::FunctionParen => Some(Rule::FunctionParen),
            Pattern::FunctionBrace => Some(Rule::FunctionBrace),
            Pattern::FlavorParen => Some(Rule::FlavorParen),
            Pattern::FlavorBrace => Some(Rule::FlavorBrace),
            Pattern::VariableNameParen => Some(Rule::VariableNameParen),
            Pattern::VariableNameBrace => Some(Rule::VariableNameBrace),
            Pattern::NestedParen => Some(Rule::NestedParen),
            Pattern::NestedBrace => Some(Rule::NestedBrace),
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
            Pattern::Prerequisites => Some(Rule::Prerequisites),
            Pattern::TargetName => Some(Rule::TargetName),
            Pattern::Recipe => Some(Rule::Recipe),
            Pattern::Assignment => Some(Rule::Assignment),
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
            | Pattern::VariableParen
            | Pattern::VariableBrace
            | Pattern::FunctionParen
            | Pattern::FunctionBrace
            | Pattern::FlavorParen
            | Pattern::FlavorBrace
            | Pattern::VariableNameParen
            | Pattern::VariableNameBrace
            | Pattern::NestedParen
            | Pattern::NestedBrace
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
            | Pattern::Prerequisites
            | Pattern::TargetName
            | Pattern::Recipe
            | Pattern::Assignment => S::NO_SCOPES,
        }
    }
}

/// `#variables`
const VARS: [Pattern; 3] = [
    Pattern::SimpleVar,
    Pattern::VariableParen,
    Pattern::VariableBrace,
];

/// The grammar's top-level patterns.
/// `$self` includes them.
const TOP: &[Pattern] = &[
    Pattern::Comment,
    Pattern::SimpleVar,
    Pattern::VariableParen,
    Pattern::VariableBrace,
    Pattern::Assignment,
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
    Pattern::VariableParen,
    Pattern::VariableBrace,
    Pattern::Percent,
];

const DEFINE: &[Pattern] = &[
    Pattern::DefineHead,
    Pattern::SimpleVar,
    Pattern::VariableParen,
    Pattern::VariableBrace,
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
    Pattern::VariableParen,
    Pattern::VariableBrace,
    Pattern::Assignment,
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
    Pattern::VariableParen,
    Pattern::VariableBrace,
    Pattern::Comment,
];

const PREREQ: &[Pattern] = &[
    Pattern::Continuation,
    Pattern::PercentStar,
    Pattern::Comment,
    Pattern::SimpleVar,
    Pattern::VariableParen,
    Pattern::VariableBrace,
];

const TARGET_NAME: &[Pattern] = &[
    Pattern::SimpleVar,
    Pattern::VariableParen,
    Pattern::VariableBrace,
    Pattern::Percent,
];

const RECIPE: &[Pattern] = &[
    Pattern::Continuation,
    Pattern::SimpleVar,
    Pattern::VariableParen,
    Pattern::VariableBrace,
];

const ASSIGN: &[Pattern] = &[
    Pattern::Continuation,
    Pattern::Comment,
    Pattern::SimpleVar,
    Pattern::VariableParen,
    Pattern::VariableBrace,
];

const VAR_PAREN: &[Pattern] = &[
    Pattern::SimpleVar,
    Pattern::VariableParen,
    Pattern::VariableBrace,
    Pattern::BuiltinParen,
    Pattern::FunctionParen,
    Pattern::FlavorParen,
    Pattern::VariableNameParen,
];

const VAR_BRACE: &[Pattern] = &[
    Pattern::SimpleVar,
    Pattern::VariableParen,
    Pattern::VariableBrace,
    Pattern::BuiltinBrace,
    Pattern::FunctionBrace,
    Pattern::FlavorBrace,
    Pattern::VariableNameBrace,
];

const FUNC: &[Pattern] = &[
    Pattern::Comma,
    Pattern::SimpleVar,
    Pattern::VariableParen,
    Pattern::VariableBrace,
    Pattern::NestedParen,
    Pattern::NestedBrace,
    Pattern::PercentStar,
    Pattern::Continuation,
];

const ANOTHER: &[Pattern] = &[
    Pattern::SimpleVar,
    Pattern::VariableParen,
    Pattern::VariableBrace,
    Pattern::Continuation,
];

const INTERP: &[Pattern] = &[
    Pattern::SimpleVar,
    Pattern::VariableParen,
    Pattern::VariableBrace,
    Pattern::NestedParen,
    Pattern::NestedBrace,
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
        Rule::Export => &[Pattern::Comment, Pattern::Assignment, Pattern::AnyWord],
        Rule::OverridePrivate => &[Pattern::Comment, Pattern::Assignment],
        Rule::Unexport => &[Pattern::Comment, Pattern::AnyWord],
        Rule::Conditional => CONDITIONAL,
        Rule::Condition | Rule::Else => CONDITION,
        Rule::Target => &[Pattern::Prerequisites],
        Rule::Prerequisites => PREREQ,
        Rule::TargetName => TARGET_NAME,
        Rule::Recipe => RECIPE,
        Rule::Assignment => ASSIGN,
        Rule::VariableParen => VAR_PAREN,
        Rule::VariableBrace => VAR_BRACE,
        Rule::FunctionParen | Rule::FunctionBrace => FUNC,
        Rule::FlavorParen | Rule::FlavorBrace | Rule::RetokenisedVariable => &VARS,
        Rule::VariableNameParen | Rule::VariableNameBrace => ANOTHER,
        Rule::NestedParen | Rule::NestedBrace => INTERP,
        Rule::RetokenisedTargets => &[Pattern::SpecialTarget, Pattern::TargetName],
    }
}
