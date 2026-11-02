//! The rules the lexer opens, which are frames of the interned state.

use super::scope::S;

/// The high byte that marks a frame as one of the GraphQL lexer's.
const TAG: u16 = crate::tag::GRAPHQL;

/// A begin/end rule of the grammar, which can stay open at the end of a line.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub(crate) enum Rule {
    /// Outside every rule.
    Root,
    /// `"""` up to the next `"""`.
    CommentTriple,
    /// `"` up to the next `"`.
    CommentSingle,
    /// `fragment A on B` up to the `}` that closes its selection set.
    Fragment,
    /// `directive @a` up to the next token.
    DirectiveDefinition,
    /// `on` and the locations of a directive definition.
    DirectiveLocations,
    /// `type`, `interface` and `input` with their name.
    TypeInterface,
    /// `implements` up to the `{` of the type.
    Implements,
    /// `enum A` up to the `}` of its values.
    Enum,
    /// The `{` of an enum up to its `}`.
    EnumBody,
    /// `schema` up to the `}` of its body.
    Schema,
    /// The `{` of a schema up to its `}`.
    SchemaBody,
    /// A field of a schema, up to the next one.
    SchemaField,
    /// `union A`.
    Union,
    /// The `=` and members of a union.
    UnionMembers,
    /// `@name`.
    Directive,
    /// `(` up to `)` after a field or directive.
    Arguments,
    /// One argument.
    Argument,
    /// A string value.
    String,
    /// A list value.
    List,
    /// An object value.
    Object,
    /// The `(` of variable definitions up to `)`.
    Variables,
    /// One variable definition.
    Variable,
    /// A list type `[T]`.
    TypeList,
    /// The `= default` of a variable.
    Default,
    /// `{` up to `}` of selections.
    Selection,
    /// `...Name`.
    Spread,
    /// `... on T` and `...`.
    InlineFragment,
    /// The `{` of a type up to its `}`.
    TypeObject,
    /// One field of a type.
    TypeField,
    /// `${` up to `}` in a selection set, around a JavaScript expression.
    Interpolation,
}

impl Rule {
    /// The rules in code order, which [`Rule::code`] numbers.
    pub(super) const ALL: [Rule; 31] = [
        Rule::Root,
        Rule::CommentTriple,
        Rule::CommentSingle,
        Rule::Fragment,
        Rule::DirectiveDefinition,
        Rule::DirectiveLocations,
        Rule::TypeInterface,
        Rule::Implements,
        Rule::Enum,
        Rule::EnumBody,
        Rule::Schema,
        Rule::SchemaBody,
        Rule::SchemaField,
        Rule::Union,
        Rule::UnionMembers,
        Rule::Directive,
        Rule::Arguments,
        Rule::Argument,
        Rule::String,
        Rule::List,
        Rule::Object,
        Rule::Variables,
        Rule::Variable,
        Rule::TypeList,
        Rule::Default,
        Rule::Selection,
        Rule::Spread,
        Rule::InlineFragment,
        Rule::TypeObject,
        Rule::TypeField,
        Rule::Interpolation,
    ];

    /// The rule as a frame stores it: the GraphQL tag in the high byte, the
    /// discriminant in the low one.
    #[expect(
        clippy::as_conversions,
        reason = "a fieldless `repr(u8)` enum has no `From` impl for its discriminant"
    )]
    pub(crate) const fn code(self) -> u16 {
        TAG | self as u16
    }

    /// The rule a frame's code names; [`Rule::Root`] for a code that is not
    /// the GraphQL lexer's.
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
            Rule::CommentTriple | Rule::CommentSingle => &[S::CommentLine],
            Rule::Fragment => &[S::MetaFragment],
            Rule::TypeInterface => &[S::MetaTypeInterface],
            Rule::Enum => &[S::MetaEnum],
            Rule::EnumBody | Rule::TypeObject => &[S::MetaTypeObject],
            Rule::Arguments => &[S::MetaArguments],
            Rule::List => &[S::MetaList],
            Rule::Object => &[S::MetaObject],
            Rule::Variable => &[S::MetaVariables],
            Rule::TypeList => &[S::MetaTypeList],
            Rule::Selection => &[S::MetaSelection],
            Rule::Interpolation => &[S::Interpolation],
            Rule::Root
            | Rule::DirectiveDefinition
            | Rule::DirectiveLocations
            | Rule::Implements
            | Rule::Schema
            | Rule::SchemaBody
            | Rule::SchemaField
            | Rule::Union
            | Rule::UnionMembers
            | Rule::Directive
            | Rule::Argument
            | Rule::String
            | Rule::Variables
            | Rule::Default
            | Rule::Spread
            | Rule::InlineFragment
            | Rule::TypeField => &[],
        }
    }

    /// The scopes the rule adds to its content only, after its begin.
    pub(crate) fn content(self) -> &'static [S] {
        if self == Rule::String {
            &[S::StringDouble]
        } else {
            &[]
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
