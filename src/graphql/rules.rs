//! The patterns each rule tries, in the grammar's order.
//!
//! A grammar rule that can never win is left out.
//! The grammar lists a docstring rule and a `#` rule after its comment rule.
//! The comment rule's `"""` and `#` begins match at the same place and come
//! first.
//! Those two rules therefore never match.

use super::scope::S;
use super::state::Rule;
use crate::scope::{Scopes, scopes};

/// A pattern of a rule: a match rule, or the begin of a rule it opens.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Pattern {
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

impl Pattern {
    /// The rule the pattern opens, when it is a begin.
    pub(super) fn opens(self) -> Option<Rule> {
        if let Pattern::Open(rule) = self {
            Some(rule)
        } else {
            None
        }
    }

    /// The scopes a match rule adds to its whole match.
    pub(super) fn name(self) -> Scopes<S> {
        match self {
            Pattern::Hash => scopes![S::CommentLine],
            Pattern::EnumValue => scopes![S::EnumValue],
            Pattern::EscapeSimple | Pattern::EscapeUnicode => scopes![S::Escape],
            Pattern::Open(_)
            | Pattern::Ampersand
            | Pattern::Boolean
            | Pattern::Colon
            | Pattern::Comma
            | Pattern::Alias
            | Pattern::Field
            | Pattern::Float
            | Pattern::Name
            | Pattern::Null
            | Pattern::ObjectField
            | Pattern::Operation
            | Pattern::Scalar
            | Pattern::BuiltinType
            | Pattern::TypeName
            | Pattern::SkipNewlines
            | Pattern::VariableName
            | Pattern::Implemented
            | Pattern::UnionMember
            | Pattern::Location
            | Pattern::SchemaType
            | Pattern::Spaces
            | Pattern::End => S::NO_SCOPES,
        }
    }
}

use Pattern::{
    Alias, Ampersand, Boolean, BuiltinType, Colon, Comma, EnumValue, EscapeSimple, EscapeUnicode,
    Field, Float, Hash, Implemented, Location, Name, Null, ObjectField, Open, Operation, Scalar,
    SchemaType, SkipNewlines, Spaces, TypeName, UnionMember, VariableName,
};

const ROOT: &[Pattern] = &[
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
const NONE: &[Pattern] = &[];

const FRAGMENT: &[Pattern] = &[
    Hash,
    Open(Rule::CommentTriple),
    Open(Rule::CommentSingle),
    Open(Rule::Selection),
    Open(Rule::Directive),
    SkipNewlines,
];

const DIRECTIVE_DEFINITION: &[Pattern] = &[
    Open(Rule::Variables),
    Open(Rule::DirectiveLocations),
    SkipNewlines,
    Hash,
    Open(Rule::CommentTriple),
    Open(Rule::CommentSingle),
];

const DIRECTIVE_LOCATIONS: &[Pattern] = &[
    SkipNewlines,
    Hash,
    Open(Rule::CommentTriple),
    Open(Rule::CommentSingle),
    Location,
];

const TYPE_INTERFACE: &[Pattern] = &[
    Open(Rule::Implements),
    Hash,
    Open(Rule::CommentTriple),
    Open(Rule::CommentSingle),
    Open(Rule::Directive),
    Open(Rule::TypeObject),
    Spaces,
];

const IMPLEMENTS: &[Pattern] = &[
    Implemented,
    Hash,
    Open(Rule::CommentTriple),
    Open(Rule::CommentSingle),
    Open(Rule::Directive),
    Ampersand,
    Comma,
];

const ENUM: &[Pattern] = &[
    Open(Rule::EnumBody),
    Hash,
    Open(Rule::CommentTriple),
    Open(Rule::CommentSingle),
    Open(Rule::Directive),
];

const ENUM_BODY: &[Pattern] = &[
    Hash,
    Open(Rule::CommentTriple),
    Open(Rule::CommentSingle),
    Open(Rule::Directive),
    EnumValue,
];

const SCHEMA: &[Pattern] = &[
    Open(Rule::SchemaBody),
    Hash,
    Open(Rule::CommentTriple),
    Open(Rule::CommentSingle),
    Open(Rule::Directive),
    SkipNewlines,
];

const SCHEMA_BODY: &[Pattern] = &[
    Open(Rule::SchemaField),
    Hash,
    Open(Rule::CommentTriple),
    Open(Rule::CommentSingle),
    SkipNewlines,
];

const SCHEMA_FIELD: &[Pattern] = &[
    SchemaType,
    Hash,
    Open(Rule::CommentTriple),
    Open(Rule::CommentSingle),
    Colon,
    SkipNewlines,
];

const UNION: &[Pattern] = &[
    Open(Rule::UnionMembers),
    Hash,
    Open(Rule::CommentTriple),
    Open(Rule::CommentSingle),
    SkipNewlines,
];

const UNION_MEMBERS: &[Pattern] = &[
    Hash,
    Open(Rule::CommentTriple),
    Open(Rule::CommentSingle),
    SkipNewlines,
    UnionMember,
];

const DIRECTIVE: &[Pattern] = &[
    Hash,
    Open(Rule::CommentTriple),
    Open(Rule::CommentSingle),
    Open(Rule::Arguments),
    SkipNewlines,
];

const ARGUMENTS: &[Pattern] = &[
    Hash,
    Open(Rule::CommentTriple),
    Open(Rule::CommentSingle),
    Open(Rule::Argument),
];

const ARGUMENT: &[Pattern] = &[
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

const STRING: &[Pattern] = &[EscapeSimple, EscapeUnicode];

const LIST: &[Pattern] = &[
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

const OBJECT: &[Pattern] = &[
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

const VARIABLES: &[Pattern] = &[
    Hash,
    Open(Rule::CommentTriple),
    Open(Rule::CommentSingle),
    Open(Rule::Variable),
];

const VARIABLE: &[Pattern] = &[
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

const TYPE_LIST: &[Pattern] = &[
    Hash,
    Open(Rule::CommentTriple),
    Open(Rule::CommentSingle),
    BuiltinType,
    TypeName,
    Open(Rule::TypeList),
    Comma,
];

const SELECTION: &[Pattern] = &[
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

const TYPE_OBJECT: &[Pattern] = &[
    Hash,
    Open(Rule::CommentTriple),
    Open(Rule::CommentSingle),
    Open(Rule::TypeField),
];

const TYPE_FIELD: &[Pattern] = &[
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
pub(super) fn patterns(rule: Rule) -> &'static [Pattern] {
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
