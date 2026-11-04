//! The patterns each rule tries, in the grammar's order.
//!
//! A grammar rule that can never win is left out. The grammar lists a
//! docstring rule and a `#` rule after its comment rule, but the comment
//! rule's `"""` and `#` begins match at the same place and come first, so
//! those two never match.

use super::scope::S;
use super::state::Rule;

/// A pattern of a rule: a match rule, or the begin of a rule it opens.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Pat {
    Open(Rule),
    /// `(\s*)(#).*`
    Hash,
    /// `\s*(&)`
    Ampersand,
    /// `\s*\b(true|false)\b`
    Boolean,
    /// `\s*(:)`
    Colon,
    /// `\s*(,)`
    Comma,
    /// `\s*([A-Z_a-z][0-9A-Z_a-z]*)`, as an enum value.
    EnumValue,
    /// `\s*(name)\s*(:)`, an alias.
    Alias,
    /// `\s*(name)`, a field.
    Field,
    /// A number.
    Float,
    /// `\s*(name)`, a definition's or operation's name.
    Name,
    /// `\s*\b(null)\b`
    Null,
    /// `\s*((name))\s*(:)`
    ObjectField,
    /// `\s*\b(query|mutation)\b`
    Operation,
    /// `\s*\b(scalar)\b\s*(name)`
    Scalar,
    /// `\s*\b(Int|Float|String|Boolean|ID)\b(?:\s*(!))?`
    BuiltinType,
    /// `\s*(name)(?:\s*(!))?`
    TypeName,
    /// `\s*\n`
    SkipNewlines,
    /// `\s*(\$name)`
    VariableName,
    /// `\s*(name)`, a type that a type implements.
    Implemented,
    /// `\s*(\|)\s*(name)`
    UnionMember,
    /// `\s*(\|)\s*([A-Z_a-z]*)`
    Location,
    /// `\s*(name)`, the type of a schema field.
    SchemaType,
    /// `\\["'/\\bfnrt]`
    EscapeSimple,
    /// `\\u(\h{4})`
    EscapeUnicode,
    /// `\s*`
    Spaces,
    /// The end pattern of the innermost rule.
    End,
}

impl Pat {
    /// The rule the pattern opens, when it is a begin.
    pub(super) fn opens(self) -> Option<Rule> {
        if let Pat::Open(rule) = self {
            Some(rule)
        } else {
            None
        }
    }

    /// The scopes a match rule adds to its whole match.
    pub(super) fn name(self) -> &'static [S] {
        match self {
            Pat::Hash => &[S::CommentLine],
            Pat::EnumValue => &[S::EnumValue],
            Pat::EscapeSimple | Pat::EscapeUnicode => &[S::Escape],
            Pat::Open(_)
            | Pat::Ampersand
            | Pat::Boolean
            | Pat::Colon
            | Pat::Comma
            | Pat::Alias
            | Pat::Field
            | Pat::Float
            | Pat::Name
            | Pat::Null
            | Pat::ObjectField
            | Pat::Operation
            | Pat::Scalar
            | Pat::BuiltinType
            | Pat::TypeName
            | Pat::SkipNewlines
            | Pat::VariableName
            | Pat::Implemented
            | Pat::UnionMember
            | Pat::Location
            | Pat::SchemaType
            | Pat::Spaces
            | Pat::End => &[],
        }
    }
}

use Pat::{
    Alias, Ampersand, Boolean, BuiltinType, Colon, Comma, EnumValue, EscapeSimple, EscapeUnicode,
    Field, Float, Hash, Implemented, Location, Name, Null, ObjectField, Open, Operation, Scalar,
    SchemaType, SkipNewlines, Spaces, TypeName, UnionMember, VariableName,
};

const ROOT: &[Pat] = &[
    Hash,
    Open(Rule::CommentTriple),
    Open(Rule::CommentSingle),
    Open(Rule::Fragment),
    Open(Rule::DirectiveDefinition),
    Open(Rule::TypeInterface),
    Open(Rule::Enum),
    Scalar,
    Open(Rule::Union),
    Open(Rule::Schema),
    Operation,
    Name,
    Open(Rule::Variables),
    Open(Rule::Directive),
    Open(Rule::Selection),
];

/// A rule that only has an end pattern, or whose text is not lexed by patterns.
const NONE: &[Pat] = &[];

const FRAGMENT: &[Pat] = &[
    Hash,
    Open(Rule::CommentTriple),
    Open(Rule::CommentSingle),
    Open(Rule::Selection),
    Open(Rule::Directive),
    SkipNewlines,
];

const DIRECTIVE_DEFINITION: &[Pat] = &[
    Open(Rule::Variables),
    Open(Rule::DirectiveLocations),
    SkipNewlines,
    Hash,
    Open(Rule::CommentTriple),
    Open(Rule::CommentSingle),
];

const DIRECTIVE_LOCATIONS: &[Pat] = &[
    SkipNewlines,
    Hash,
    Open(Rule::CommentTriple),
    Open(Rule::CommentSingle),
    Location,
];

const TYPE_INTERFACE: &[Pat] = &[
    Open(Rule::Implements),
    Hash,
    Open(Rule::CommentTriple),
    Open(Rule::CommentSingle),
    Open(Rule::Directive),
    Open(Rule::TypeObject),
    Spaces,
];

const IMPLEMENTS: &[Pat] = &[
    Implemented,
    Hash,
    Open(Rule::CommentTriple),
    Open(Rule::CommentSingle),
    Open(Rule::Directive),
    Ampersand,
    Comma,
];

const ENUM: &[Pat] = &[
    Open(Rule::EnumBody),
    Hash,
    Open(Rule::CommentTriple),
    Open(Rule::CommentSingle),
    Open(Rule::Directive),
];

const ENUM_BODY: &[Pat] = &[
    Hash,
    Open(Rule::CommentTriple),
    Open(Rule::CommentSingle),
    Open(Rule::Directive),
    EnumValue,
];

const SCHEMA: &[Pat] = &[
    Open(Rule::SchemaBody),
    Hash,
    Open(Rule::CommentTriple),
    Open(Rule::CommentSingle),
    Open(Rule::Directive),
    SkipNewlines,
];

const SCHEMA_BODY: &[Pat] = &[
    Open(Rule::SchemaField),
    Hash,
    Open(Rule::CommentTriple),
    Open(Rule::CommentSingle),
    SkipNewlines,
];

const SCHEMA_FIELD: &[Pat] = &[
    SchemaType,
    Hash,
    Open(Rule::CommentTriple),
    Open(Rule::CommentSingle),
    Colon,
    SkipNewlines,
];

const UNION: &[Pat] = &[
    Open(Rule::UnionMembers),
    Hash,
    Open(Rule::CommentTriple),
    Open(Rule::CommentSingle),
    SkipNewlines,
];

const UNION_MEMBERS: &[Pat] = &[
    Hash,
    Open(Rule::CommentTriple),
    Open(Rule::CommentSingle),
    SkipNewlines,
    UnionMember,
];

const DIRECTIVE: &[Pat] = &[
    Hash,
    Open(Rule::CommentTriple),
    Open(Rule::CommentSingle),
    Open(Rule::Arguments),
    SkipNewlines,
];

const ARGUMENTS: &[Pat] = &[
    Hash,
    Open(Rule::CommentTriple),
    Open(Rule::CommentSingle),
    Open(Rule::Argument),
];

const ARGUMENT: &[Pat] = &[
    Hash,
    Open(Rule::CommentTriple),
    Open(Rule::CommentSingle),
    Open(Rule::Directive),
    VariableName,
    Float,
    Open(Rule::String),
    Boolean,
    Null,
    EnumValue,
    Open(Rule::List),
    Open(Rule::Object),
    SkipNewlines,
];

const STRING: &[Pat] = &[EscapeSimple, EscapeUnicode];

const LIST: &[Pat] = &[
    Hash,
    Open(Rule::CommentTriple),
    Open(Rule::CommentSingle),
    VariableName,
    Float,
    Open(Rule::String),
    Boolean,
    Null,
    EnumValue,
    Open(Rule::List),
    Open(Rule::Object),
];

const OBJECT: &[Pat] = &[
    ObjectField,
    Hash,
    Open(Rule::CommentTriple),
    Open(Rule::CommentSingle),
    VariableName,
    Float,
    Open(Rule::String),
    Boolean,
    Null,
    EnumValue,
    Open(Rule::List),
    Open(Rule::Object),
];

const VARIABLES: &[Pat] = &[
    Hash,
    Open(Rule::CommentTriple),
    Open(Rule::CommentSingle),
    Open(Rule::Variable),
];

const VARIABLE: &[Pat] = &[
    Hash,
    Open(Rule::CommentTriple),
    Open(Rule::CommentSingle),
    Open(Rule::Directive),
    Colon,
    BuiltinType,
    TypeName,
    Open(Rule::TypeList),
    Open(Rule::Default),
    SkipNewlines,
];

const TYPE_LIST: &[Pat] = &[
    Hash,
    Open(Rule::CommentTriple),
    Open(Rule::CommentSingle),
    BuiltinType,
    TypeName,
    Open(Rule::TypeList),
    Comma,
];

const SELECTION: &[Pat] = &[
    Hash,
    Open(Rule::CommentTriple),
    Open(Rule::CommentSingle),
    Alias,
    Field,
    Open(Rule::Arguments),
    Open(Rule::Directive),
    Open(Rule::Selection),
    SkipNewlines,
    Open(Rule::Spread),
    Open(Rule::InlineFragment),
    Comma,
    Open(Rule::Interpolation),
];

const TYPE_OBJECT: &[Pat] = &[
    Hash,
    Open(Rule::CommentTriple),
    Open(Rule::CommentSingle),
    Open(Rule::TypeField),
];

const TYPE_FIELD: &[Pat] = &[
    Hash,
    Open(Rule::CommentTriple),
    Open(Rule::CommentSingle),
    Open(Rule::Directive),
    Open(Rule::Variables),
    Open(Rule::TypeObject),
    Colon,
    BuiltinType,
    TypeName,
    Open(Rule::TypeList),
];

/// The patterns of `rule`, in the order the grammar tries them.
pub(super) fn patterns(rule: Rule) -> &'static [Pat] {
    match rule {
        Rule::Root => ROOT,
        Rule::CommentTriple | Rule::CommentSingle | Rule::Interpolation => NONE,
        Rule::Fragment | Rule::Spread | Rule::InlineFragment => FRAGMENT,
        Rule::DirectiveDefinition => DIRECTIVE_DEFINITION,
        Rule::DirectiveLocations => DIRECTIVE_LOCATIONS,
        Rule::TypeInterface => TYPE_INTERFACE,
        Rule::Implements => IMPLEMENTS,
        Rule::Enum => ENUM,
        Rule::EnumBody => ENUM_BODY,
        Rule::Schema => SCHEMA,
        Rule::SchemaBody => SCHEMA_BODY,
        Rule::SchemaField => SCHEMA_FIELD,
        Rule::Union => UNION,
        Rule::UnionMembers => UNION_MEMBERS,
        Rule::Directive => DIRECTIVE,
        Rule::Arguments => ARGUMENTS,
        Rule::Argument => ARGUMENT,
        Rule::String => STRING,
        Rule::List | Rule::Default => LIST,
        Rule::Object => OBJECT,
        Rule::Variables => VARIABLES,
        Rule::Variable => VARIABLE,
        Rule::TypeList => TYPE_LIST,
        Rule::Selection => SELECTION,
        Rule::TypeObject => TYPE_OBJECT,
        Rule::TypeField => TYPE_FIELD,
    }
}
