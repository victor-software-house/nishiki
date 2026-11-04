//! The rules the lexer opens.
//! Each is a frame of the interned state.

use super::scope::S;

/// The high byte that marks a frame as one of the GraphQL lexer's.
const TAG: u16 = crate::tag::GRAPHQL;

crate::tag::frame_kinds! {
    /// A begin/end rule of the grammar.
    /// It can stay open at the end of a line.
    pub(crate) enum Rule: TAG {
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
}

impl Rule {
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
