//! The rules the lexer opens.
//! Each is a frame of the interned state.

use super::scope::S;
use crate::scope::{Scopes, scopes};

/// The high byte that marks a frame as one of the Makefile lexer's.
const TAG: u16 = crate::tag::MAKE;

crate::tag::frame_kinds! {
    /// A rule of the grammar: a begin/end rule that can stay open at the end of a
    /// line, or a pseudo rule that only lists the patterns a capture is lexed
    /// again with.
    pub(crate) enum Rule: TAG {
        /// Outside every rule.
        Root,
        /// The outer rule of a comment.
        /// It closes right after the `#` rule.
        Comment,
        /// `#` up to the end of the line, or to the end of a continued one.
        CommentHash,
        /// `include`, `-include` and `sinclude` up to the next line.
        Include,
        /// `vpath` up to the next line.
        Vpath,
        /// `define` up to `endef`.
        Define,
        /// The rest of the line that opens a `define`.
        DefineHead,
        /// `export` up to the next line.
        Export,
        /// `override` and `private` up to the next line.
        OverridePrivate,
        /// `unexport` and `undefine` up to the next line.
        Unexport,
        /// `ifeq` and its kin up to `endif`.
        Conditional,
        /// The condition of a conditional, up to the next line.
        Condition,
        /// `else` up to the next line.
        Else,
        /// A target, up to the end of its last prerequisite line.
        Target,
        /// The prerequisites after the `:`.
        Prereq,
        /// One word of the targets.
        TargetName,
        /// A recipe line, up to the end of the line it continues to.
        Recipe,
        /// A variable assignment, up to the end of its line.
        Assign,
        /// `$(` up to `)`.
        VarParen,
        /// `${` up to `}`.
        VarBrace,
        /// A function call inside `$(`.
        FuncParen,
        /// A function call inside `${`.
        FuncBrace,
        /// `origin` and `flavor` inside `$(`.
        FlavorParen,
        /// `origin` and `flavor` inside `${`.
        FlavorBrace,
        /// A variable name inside `$(`.
        AnotherParen,
        /// A variable name inside `${`.
        AnotherBrace,
        /// A `(`...`)` inside a function call.
        InterpParen,
        /// A `{`...`}` inside a function call.
        InterpBrace,
        /// The patterns that lex the targets of a rule again.
        RetokTarget,
        /// The patterns that lex a variable's name again.
        RetokVars,
    }
}

impl Rule {
    /// The scopes the rule adds to its begin, its content and its end.
    pub(crate) fn name(self) -> Scopes<S> {
        match self {
            Rule::CommentHash => scopes![S::CommentLine],
            Rule::Define | Rule::Conditional => scopes![S::MetaConditional],
            Rule::Condition => scopes![S::MetaCondition],
            Rule::Target => scopes![S::MetaTarget],
            Rule::Prereq => scopes![S::MetaPrereq],
            Rule::TargetName => scopes![S::TargetName],
            Rule::Recipe => scopes![S::MetaRecipe],
            Rule::VarParen | Rule::VarBrace => scopes![S::StringInterp],
            Rule::FuncParen | Rule::FuncBrace | Rule::FlavorParen | Rule::FlavorBrace => {
                scopes![S::MetaFunc]
            }
            Rule::AnotherParen | Rule::AnotherBrace => scopes![S::VarOther],
            Rule::Root
            | Rule::Comment
            | Rule::Include
            | Rule::Vpath
            | Rule::DefineHead
            | Rule::Export
            | Rule::OverridePrivate
            | Rule::Unexport
            | Rule::Else
            | Rule::Assign
            | Rule::InterpParen
            | Rule::InterpBrace
            | Rule::RetokTarget
            | Rule::RetokVars => S::NO_SCOPES,
        }
    }

    /// The scopes the rule adds to its content only, after its begin.
    pub(crate) fn content(self) -> Scopes<S> {
        match self {
            Rule::FlavorParen | Rule::FlavorBrace => scopes![S::VarOther],
            Rule::Root
            | Rule::Comment
            | Rule::CommentHash
            | Rule::Include
            | Rule::Vpath
            | Rule::Define
            | Rule::DefineHead
            | Rule::Export
            | Rule::OverridePrivate
            | Rule::Unexport
            | Rule::Conditional
            | Rule::Condition
            | Rule::Else
            | Rule::Target
            | Rule::Prereq
            | Rule::TargetName
            | Rule::Recipe
            | Rule::Assign
            | Rule::VarParen
            | Rule::VarBrace
            | Rule::FuncParen
            | Rule::FuncBrace
            | Rule::AnotherParen
            | Rule::AnotherBrace
            | Rule::InterpParen
            | Rule::InterpBrace
            | Rule::RetokTarget
            | Rule::RetokVars => S::NO_SCOPES,
        }
    }
}
