//! The scope names the lexer emits, in Shiki's naming.

macro_rules! scopes {
    ($($id:ident $name:literal)*) => {
        /// A scope name; the discriminant indexes [`NAMES`].
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[repr(u8)]
        pub(crate) enum S { $($id),* }

        impl S {
            /// The scope in the crate's shared numbering.
            #[expect(
                clippy::as_conversions,
                reason = "a fieldless `repr(u8)` enum has no `From` impl for its discriminant"
            )]
            pub(crate) const fn scope(self) -> crate::scope::Scope {
                crate::scope::Scope::new(crate::scope::GRAPHQL, self as u16)
            }
        }

        /// The name of every scope, indexed by discriminant.
        pub(crate) const NAMES: &[&str] = &[$($name),*];
    };
}

scopes! {
    Source "source.graphql"
    CommentLine "comment.line.graphql.js"
    LeadingSpace "punctuation.whitespace.comment.leading.graphql"
    Ampersand "keyword.operator.logical.graphql"
    Boolean "constant.language.boolean.graphql"
    Colon "punctuation.colon.graphql"
    Comma "punctuation.comma.graphql"
    EnumValue "constant.character.enum.graphql"
    Alias "string.unquoted.alias.graphql"
    Variable "variable.graphql"
    Float "constant.numeric.float.graphql"
    FunctionName "entity.name.function.graphql"
    Null "constant.language.null.graphql"
    ObjectKey "constant.object.key.graphql"
    ObjectKeyName "string.unquoted.graphql"
    Punctuation "punctuation.graphql"
    Operation "keyword.operation.graphql"
    KeywordScalar "keyword.scalar.graphql"
    ScalarName "entity.scalar.graphql"
    BuiltinType "support.type.builtin.graphql"
    NullType "keyword.operator.nulltype.graphql"
    SupportType "support.type.graphql"
    UnionOr "punctuation.or.graphql"
    LocationType "support.type.location.graphql"
    Escape "constant.character.escape.graphql"
    KeywordFragment "keyword.fragment.graphql"
    FragmentName "entity.name.fragment.graphql"
    KeywordOn "keyword.on.graphql"
    MetaFragment "meta.fragment.graphql"
    KeywordDirective "keyword.directive.graphql"
    DirectiveName "entity.name.function.directive.graphql"
    KeywordType "keyword.type.graphql"
    KeywordInterface "keyword.interface.graphql"
    KeywordInput "keyword.input.graphql"
    MetaTypeInterface "meta.type.interface.graphql"
    KeywordImplements "keyword.implements.graphql"
    KeywordEnum "keyword.enum.graphql"
    EnumType "support.type.enum.graphql"
    MetaEnum "meta.enum.graphql"
    Operator "punctuation.operation.graphql"
    MetaTypeObject "meta.type.object.graphql"
    KeywordSchema "keyword.schema.graphql"
    ArgumentsVariable "variable.arguments.graphql"
    KeywordUnion "keyword.union.graphql"
    Assignment "punctuation.assignment.graphql"
    BraceDirective "meta.brace.round.directive.graphql"
    MetaArguments "meta.arguments.graphql"
    Parameter "variable.parameter.graphql"
    BraceSquare "meta.brace.square.graphql"
    MetaList "meta.listvalues.graphql"
    BraceCurly "meta.brace.curly.graphql"
    MetaObject "meta.objectvalues.graphql"
    BraceRound "meta.brace.round.graphql"
    MetaVariables "meta.variables.graphql"
    MetaTypeList "meta.type.list.graphql"
    MetaSelection "meta.selectionset.graphql"
    Spread "keyword.operator.spread.graphql"
    FragmentVariable "variable.fragment.graphql"
    StringDouble "string.quoted.double.graphql"
    StringBegin "punctuation.definition.string.begin.graphql"
    StringEnd "punctuation.definition.string.end.graphql"
    Interpolation "native.interpolation"
    SubstitutionBegin "keyword.other.substitution.begin"
    SubstitutionEnd "keyword.other.substitution.end"
}
