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
                crate::scope::Scope::new(crate::scope::SQL, self as u16)
            }
        }

        /// The name of every scope, indexed by discriminant.
        pub(crate) const NAMES: &[&str] = &[$($name),*];
    };
}

scopes! {
    Source "source.sql"
    Variable "text.variable"
    Bracketed "text.bracketed"
    PunctBracketBegin "punctuation.definition.bracket.begin.sql"
    PunctBracketEnd "punctuation.definition.bracket.end.sql"
    BracketChar "constant.character.bracket.sql"
    LeadingSpace "punctuation.whitespace.comment.leading.sql"
    CommentDash "comment.line.double-dash.sql"
    PunctComment "punctuation.definition.comment.sql"
    CommentBlock "comment.block"
    MetaCreate "meta.create.sql"
    KwCreate "keyword.other.create.sql"
    KwOther "keyword.other.sql"
    EntityFunction "entity.name.function.sql"
    MetaDrop "meta.drop.sql"
    KwTable "keyword.other.table.sql"
    KwCascade "keyword.other.cascade.sql"
    MetaAlter "meta.alter.sql"
    StorageType "storage.type.sql"
    Numeric "constant.numeric.sql"
    StorageModifier "storage.modifier.sql"
    KwDml "keyword.other.DML.sql"
    KwDdl "keyword.other.DDL.create.II.sql"
    KwDmlValues "keyword.other.DML.II.sql"
    KwLuw "keyword.other.LUW.sql"
    KwAuthorization "keyword.other.authorization.sql"
    KwIntegrity "keyword.other.data-integrity.sql"
    KwComments "keyword.other.object-comments.sql"
    KwAlias "keyword.other.alias.sql"
    KwOrder "keyword.other.order.sql"
    OpStar "keyword.operator.star.sql"
    OpComparison "keyword.operator.comparison.sql"
    OpMath "keyword.operator.math.sql"
    OpConcat "keyword.operator.concatenator.sql"
    FnAggregate "support.function.aggregate.sql"
    FnAnalytic "support.function.analytic.sql"
    FnBitmanipulation "support.function.bitmanipulation.sql"
    FnConversion "support.function.conversion.sql"
    FnCollation "support.function.collation.sql"
    FnCryptographic "support.function.cryptographic.sql"
    FnCursor "support.function.cursor.sql"
    FnDatetime "support.function.datetime.sql"
    FnDatatype "support.function.datatype.sql"
    FnExpression "support.function.expression.sql"
    FnGlobalvar "support.function.globalvar.sql"
    FnJson "support.function.json.sql"
    FnLogical "support.function.logical.sql"
    FnMathematical "support.function.mathematical.sql"
    FnMetadata "support.function.metadata.sql"
    FnRanking "support.function.ranking.sql"
    FnRowset "support.function.rowset.sql"
    FnSecurity "support.function.security.sql"
    FnString "support.function.string.sql"
    FnSystem "support.function.system.sql"
    FnTextimage "support.function.textimage.sql"
    FnVector "support.function.vector.sql"
    DatabaseName "constant.other.database-name.sql"
    TableName "constant.other.table-name.sql"
    StrSingle "string.quoted.single.sql"
    PunctStrBegin "punctuation.definition.string.begin.sql"
    PunctStrEnd "punctuation.definition.string.end.sql"
    StrBacktick "string.quoted.other.backtick.sql"
    StrDouble "string.quoted.double.sql"
    StrBraces "string.other.quoted.brackets.sql"
    StrModr "string.regexp.modr.sql"
    Escape "constant.character.escape.sql"
    StrInterp "string.interpolated.sql"
    MetaBlock "meta.block.sql"
    PunctScopeBegin "punctuation.section.scope.begin.sql"
    PunctScopeEnd "punctuation.section.scope.end.sql"
}
