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
                crate::scope::Scope::new(crate::scope::RUST, self as u16)
            }
        }

        /// The name of every scope, indexed by discriminant.
        pub(crate) const NAMES: &[&str] = &[$($name),*];
    };
}

scopes! {
    Source "source.rust"
    MetaFnDef "meta.function.definition.rust"
    MetaFnCall "meta.function.call.rust"
    MetaUse "meta.use.rust"
    MetaImport "meta.import.rust"
    MetaAttr "meta.attribute.rust"
    MetaMacro "meta.macro.rust"
    MetaMacroRules "meta.macro.rules.rust"
    MetaMetavar "meta.macro.metavariable.rust"
    MetaMetavarType "meta.macro.metavariable.type.rust"
    MetaInterp "meta.interpolation.rust"
    CommentLineDoc "comment.line.documentation.rust"
    CommentLine "comment.line.double-slash.rust"
    PunctDefComment "punctuation.definition.comment.rust"
    CommentBlock "comment.block.rust"
    CommentBlockDoc "comment.block.documentation.rust"
    StrDouble "string.quoted.double.rust"
    StrByteRaw "string.quoted.byte.raw.rust"
    PunctDefString "punctuation.definition.string.rust"
    PunctDefStringRaw "punctuation.definition.string.raw.rust"
    StrChar "string.quoted.single.char.rust"
    PunctDefChar "punctuation.definition.char.rust"
    Escape "constant.character.escape.rust"
    EscBackslash "constant.character.escape.backslash.rust"
    EscBit "constant.character.escape.bit.rust"
    EscUnicode "constant.character.escape.unicode.rust"
    EscUnicodePunct "constant.character.escape.unicode.punctuation.rust"
    PunctDefInterp "punctuation.definition.interpolation.rust"
    ConstCaps "constant.other.caps.rust"
    NumDecimal "constant.numeric.decimal.rust"
    NumHex "constant.numeric.hex.rust"
    NumOct "constant.numeric.oct.rust"
    NumBin "constant.numeric.bin.rust"
    LangBool "constant.language.bool.rust"
    PunctDot "punctuation.separator.dot.decimal.rust"
    OpExp "keyword.operator.exponent.rust"
    OpExpSign "keyword.operator.exponent.sign.rust"
    NumExpMantissa "constant.numeric.decimal.exponent.mantissa.rust"
    TypeNumeric "entity.name.type.numeric.rust"
    TypeOption "entity.name.type.option.rust"
    TypeResult "entity.name.type.result.rust"
    KwControl "keyword.control.rust"
    KwOther "keyword.other.rust"
    StorageType "storage.type.rust"
    StorageModifier "storage.modifier.rust"
    KwDeclType "keyword.declaration.type.rust"
    KwDeclEnum "keyword.declaration.enum.rust"
    KwDeclTrait "keyword.declaration.trait.rust"
    KwDeclStruct "keyword.declaration.struct.rust"
    KwFn "keyword.other.fn.rust"
    KwCrate "keyword.other.crate.rust"
    StorageMut "storage.modifier.mut.rust"
    StorageVis "storage.modifier.visibility.rust"
    OpLogical "keyword.operator.logical.rust"
    OpBorrowAnd "keyword.operator.borrow.and.rust"
    OpAssign "keyword.operator.assignment.rust"
    OpAssignEq "keyword.operator.assignment.equal.rust"
    OpCompare "keyword.operator.comparison.rust"
    OpMath "keyword.operator.math.rust"
    OpNamespace "keyword.operator.namespace.rust"
    OpDeref "keyword.operator.dereference.rust"
    OpSubpattern "keyword.operator.subpattern.rust"
    OpDot "keyword.operator.access.dot.rust"
    OpRange "keyword.operator.range.rust"
    OpKeyValue "keyword.operator.key-value.rust"
    OpArrowSkinny "keyword.operator.arrow.skinny.rust"
    OpArrowFat "keyword.operator.arrow.fat.rust"
    OpMacroDollar "keyword.operator.macro.dollar.rust"
    OpQuestion "keyword.operator.question.rust"
    OpBorrow "keyword.operator.borrow.rust"
    LifetimePunct "punctuation.definition.lifetime.rust"
    LifetimeName "entity.name.type.lifetime.rust"
    VarSelf "variable.language.self.rust"
    VarSuper "variable.language.super.rust"
    EntMacroFn "entity.name.function.macro.rust"
    EntMacroType "entity.name.type.macro.rust"
    EntMacroRulesFn "entity.name.function.macro.rules.rust"
    EntNamespace "entity.name.namespace.rust"
    EntFunction "entity.name.function.rust"
    EntModule "entity.name.module.rust"
    EntType "entity.name.type.rust"
    EntTypePrimitive "entity.name.type.primitive.rust"
    EntTypeTrait "entity.name.type.trait.rust"
    EntTypeStruct "entity.name.type.struct.rust"
    EntTypeEnum "entity.name.type.enum.rust"
    EntTypeDecl "entity.name.type.declaration.rust"
    EntTypeMetavar "entity.name.type.metavariable.rust"
    VarOther "variable.other.rust"
    VarMetaName "variable.other.metavariable.name.rust"
    VarMetaSpec "variable.other.metavariable.specifier.rust"
    Comma "punctuation.comma.rust"
    Curly "punctuation.brackets.curly.rust"
    Round "punctuation.brackets.round.rust"
    Semi "punctuation.semi.rust"
    Square "punctuation.brackets.square.rust"
    Angle "punctuation.brackets.angle.rust"
    AttrBracket "punctuation.brackets.attribute.rust"
    PunctDefAttr "punctuation.definition.attribute.rust"
}
