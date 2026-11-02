//! The rules the lexer opens, which are frames of the interned state.

use super::scope::S;

/// The high byte that marks a frame as one of the Makefile lexer's.
const TAG: u16 = 0x0b00;

/// A rule of the grammar: a begin/end rule that can stay open at the end of a
/// line, or a pseudo rule that only lists the patterns a capture is lexed
/// again with.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub(crate) enum Rule {
    /// Outside every rule.
    Root,
    /// The outer rule of a comment, which closes right after the `#` rule.
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

impl Rule {
    /// The rules in code order, which [`Rule::code`] numbers.
    const ALL: [Rule; 30] = [
        Rule::Root,
        Rule::Comment,
        Rule::CommentHash,
        Rule::Include,
        Rule::Vpath,
        Rule::Define,
        Rule::DefineHead,
        Rule::Export,
        Rule::OverridePrivate,
        Rule::Unexport,
        Rule::Conditional,
        Rule::Condition,
        Rule::Else,
        Rule::Target,
        Rule::Prereq,
        Rule::TargetName,
        Rule::Recipe,
        Rule::Assign,
        Rule::VarParen,
        Rule::VarBrace,
        Rule::FuncParen,
        Rule::FuncBrace,
        Rule::FlavorParen,
        Rule::FlavorBrace,
        Rule::AnotherParen,
        Rule::AnotherBrace,
        Rule::InterpParen,
        Rule::InterpBrace,
        Rule::RetokTarget,
        Rule::RetokVars,
    ];

    /// The rule as a frame stores it: the Makefile tag in the high byte, the
    /// discriminant in the low one.
    #[expect(
        clippy::as_conversions,
        reason = "a fieldless `repr(u8)` enum has no `From` impl for its discriminant"
    )]
    pub(crate) const fn code(self) -> u16 {
        TAG | self as u16
    }

    /// The rule a frame's code names; [`Rule::Root`] for a code that is not
    /// the Makefile lexer's.
    pub(crate) fn from_code(code: u16) -> Rule {
        if code & 0xff00 != TAG {
            return Rule::Root;
        }
        Rule::ALL
            .get(usize::from(code & 0xff))
            .copied()
            .unwrap_or(Rule::Root)
    }

    /// The scopes the rule adds to its begin, its content and its end.
    pub(crate) fn name(self) -> &'static [S] {
        match self {
            Rule::CommentHash => &[S::CommentLine],
            Rule::Define | Rule::Conditional => &[S::MetaConditional],
            Rule::Condition => &[S::MetaCondition],
            Rule::Target => &[S::MetaTarget],
            Rule::Prereq => &[S::MetaPrereq],
            Rule::TargetName => &[S::TargetName],
            Rule::Recipe => &[S::MetaRecipe],
            Rule::VarParen | Rule::VarBrace => &[S::StringInterp],
            Rule::FuncParen | Rule::FuncBrace | Rule::FlavorParen | Rule::FlavorBrace => {
                &[S::MetaFunc]
            }
            Rule::AnotherParen | Rule::AnotherBrace => &[S::VarOther],
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
            | Rule::RetokVars => &[],
        }
    }

    /// The scopes the rule adds to its content only, after its begin.
    pub(crate) fn content(self) -> &'static [S] {
        match self {
            Rule::FlavorParen | Rule::FlavorBrace => &[S::VarOther],
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
            | Rule::RetokVars => &[],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_rule_codes_to_its_place_in_all() {
        for (index, rule) in Rule::ALL.iter().enumerate() {
            assert_eq!(usize::from(rule.code() & 0xff), index, "{rule:?}");
            assert_eq!(Rule::from_code(rule.code()), *rule);
        }
    }
}
