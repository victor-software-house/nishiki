//! The contexts of the grammar: each begin/end rule that holds state across
//! tokens, with the scopes it adds.
//! Each is a frame of the highlighter's interned state.

use super::scope::S;

/// The high byte that marks a frame as one of TypeScript's.
const TAG: u16 = crate::tag::TYPESCRIPT;

macro_rules! kinds {
    ($($id:ident [$($name:expr),*] [$($content:expr),*];)*) => {
        /// An open context.
        /// The scopes in `names` apply to the context's begin and end tokens
        /// and to everything inside.
        /// Those in `content` apply only to what lies between begin and end.
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[repr(u8)]
        pub(crate) enum Kind { $($id),* }

        impl Kind {
            /// Every `Kind`, indexed by discriminant.
            const ALL: &[Kind] = &[$(Kind::$id),*];

            /// How many kinds there are.
            /// The count bounds the lexer's steps without advancing.
            #[cfg(debug_assertions)]
            pub(crate) const COUNT: usize = Self::ALL.len();

            /// The scopes of the begin and end tokens and of the content.
            pub(crate) const fn names(self) -> &'static [S] {
                match self { $(Kind::$id => &[$($name),*]),* }
            }

            /// The scopes of the content only.
            pub(crate) const fn content(self) -> &'static [S] {
                match self { $(Kind::$id => &[$($content),*]),* }
            }
        }
    };
}

kinds! {
    Top [] [];
    Started [] [];
    Script [] [];
    Expression [] [];
    EntryObjLit [] [];
    EntryObjBind [] [];
    EntryObjBindConst [] [];
    EntryArrBind [] [];
    EntryArrBindConst [] [];
    EntryTypeParams [] [];
    Block [S::MetaBlock] [];
    ObjLit [S::MetaObjectliteral] [];
    ArrLit [S::MetaArrayLiteral] [];
    ArrBind [] [];
    ArrBindConst [] [];
    ObjBind [] [];
    ObjBindConst [] [];
    ParenExpr [] [];
    ArrowHead [S::MetaArrow] [];
    ArrowBody [S::MetaArrow] [];
    ArrowRet [S::MetaReturnTypeArrow] [];
    ArrowRetObj [] [];
    PossArrowRet [] [S::MetaArrow, S::MetaReturnTypeArrow];
    Params [S::MetaParameters] [];
    Cast [S::CastExpr] [];
    JsBraces [] [];
    JsBrackets [] [];
    ClassDecl [S::MetaClass] [];
    ClassBody [] [];
    ClassColon [] [];
    Heritage [] [];
    CommentDoc [S::CommentBlockDocumentation] [];
    CommentBlock [S::CommentBlock] [];
    CommentLine [] [S::CommentLineDoubleSlash];
    CommentLineNl [] [S::CommentLineDoubleSlash];
    ScriptBlockComment [S::CommentBlock] [];
    ScriptLineComment [S::CommentLineDoubleSlash] [];
    ForLoop [] [];
    ForParen [] [];
    Return [] [];
    SwitchStmt [S::SwitchStatementExpr] [];
    SwitchExpr [S::SwitchExpressionExpr] [];
    SwitchBlock [S::SwitchBlockExpr] [];
    CaseClause [S::CaseClauseExpr] [];
    CaseBlock [] [S::MetaBlock];
    IfOuter [] [];
    IfParen [] [];
    Decorator [S::MetaDecorator] [];
    ObjBindVar [S::MetaObjectBindingPatternVariable] [];
    ArrBindVar [S::MetaArrayBindingPatternVariable] [];
    ObjBindVarConst [S::MetaObjectBindingPatternVariable] [];
    ArrBindVarConst [S::MetaArrayBindingPatternVariable] [];
    ParamObjBind [S::MetaParameterObjectBindingPattern] [];
    ParamArrBind [S::MetaParameterArrayBindingPattern] [];
    ParamObjPattern [] [];
    ParamArrPattern [] [];
    Directive [S::CommentLineTripleSlashDirective] [];
    DirectiveTag [S::MetaTag] [];
    JsExample [S::MetaExampleJsdoc] [];
    JsCaption [] [S::ConstantOtherDescriptionJsdoc];
    JsTypeTag [] [];
    JsParamTag [] [];
    JsTypedefTag [] [];
    JsRetTag [] [];
    JsDefaultQuote [] [S::VariableOtherJsdoc];
    JsInlineLink [S::EntityNameTypeInstanceJsdoc] [];
    JsType [] [S::EntityNameTypeInstanceJsdoc];
    EnumDecl [S::MetaEnumDeclaration] [];
    EnumBody [] [];
    EnumMember [] [];
    EnumKey [] [];
    ExportDefault [S::MetaExportDefault] [];
    Export [S::MetaExport] [];
    Field [S::MetaFieldDeclaration] [];
    CallOuter [] [];
    CallTarget [S::MetaFunctionCall] [];
    Call2Outer [] [];
    Call2Target [S::MetaFunctionCall] [];
    FuncDecl [S::MetaFunction] [];
    FuncExpr [S::MetaFunctionExpression] [];
    Import [S::MetaImport] [];
    ImportFrom [] [];
    ImportEqExt [S::MetaImportEqualsExternal] [];
    ImportEqInt [S::MetaImportEqualsInternal] [];
    AssertClause [] [];
    ImportBlock [S::MetaBlock] [];
    Indexer [S::MetaIndexerDeclaration] [];
    IndexerMapped [S::MetaIndexerMappedtypeDeclaration] [];
    InstanceOf [] [];
    InterfaceDecl [S::MetaInterface] [];
    LabelBlock [] [];
    Method [S::MetaMethodDeclaration] [];
    MethodName [] [];
    ObjMethod [S::MetaMethodDeclaration] [];
    ObjMethodNested [] [];
    NamespaceDecl [S::MetaNamespaceDeclaration] [];
    NewExpr [S::NewExpr] [];
    ObjBindElem [] [];
    ObjBindElemConst [] [];
    ObjBindProp [] [];
    ParamObjElem [] [];
    ObjKeyBracket [S::MetaObjectMember, S::MetaObjectLiteralKey] [];
    ObjKeyStr [S::MetaObjectMember, S::MetaObjectLiteralKey] [];
    ObjKeyNum [S::MetaObjectMember, S::MetaObjectLiteralKey] [];
    ObjMethodAfterKey [S::MetaMethodDeclaration] [];
    ObjMember [S::MetaObjectMember] [];
    ObjSpread [S::MetaObjectMember] [];
    ObjAs [S::MetaObjectMember] [];
    ObjInit [S::MetaObjectMember] [];
    ObjValue [S::MetaObjectMember] [];
    ObjArrowGeneric [] [];
    ObjArrowParen [] [];
    ObjArrowOpen [] [];
    AsType [] [];
    YieldComment [] [];
    ParamTypeAnn [S::MetaTypeAnnotation] [];
    PossArrow [] [];
    ParenArrowParams [] [];
    Regex [S::StringRegexp] [];
    RxGroup [S::MetaGroupRegexp] [];
    RxAssert [S::MetaGroupAssertionRegexp] [];
    RxSet [S::ConstantOtherCharacterClassSetRegexp] [];
    ReturnType1 [S::MetaReturnType] [];
    ReturnType2 [S::MetaReturnType] [];
    RetObj [] [];
    StrSingle [S::StringQuotedSingle] [];
    StrDouble [S::StringQuotedDouble] [];
    Template [] [S::StringTemplate];
    TemplateExpr [S::MetaTemplateExpression] [S::MetaEmbeddedLine];
    TemplateCall [] [];
    TemplateCallTarget [] [];
    TemplateCall2 [] [];
    TemplateType [] [S::StringTemplate];
    TemplateTypeExpr [S::MetaTemplateExpression] [S::MetaEmbeddedLine];
    Ternary [] [];
    TypeAnn1 [S::MetaTypeAnnotation] [];
    TypeAnn2 [S::MetaTypeAnnotation] [];
    TypeArgs [S::MetaTypeParameters] [];
    TypeNameArgs [] [S::MetaTypeParameters];
    TypeAlias [S::MetaTypeDeclaration] [];
    TypeAliasEq [] [];
    TypeCond [] [];
    TypeCondQ [] [];
    TypeCtorTp [] [];
    TypeCtor [S::MetaTypeConstructor] [];
    TypeFn [S::MetaTypeFunction] [];
    TypeFnRet1 [S::MetaTypeFunctionReturn] [];
    TypeFnRet2 [S::MetaTypeFunctionReturn] [];
    TypeFnRetObj [] [];
    TypeObj [S::MetaObjectType] [];
    TypeSpread [] [];
    TypeOpObj [] [];
    TypeOp [] [];
    TypeParams [S::MetaTypeParameters] [];
    TypeParen [S::MetaTypeParenCover] [];
    TypeTuple [S::MetaTypeTuple] [];
    Typeof [] [];
    VarLet [S::MetaVarExpr] [];
    VarConst [S::MetaVarExpr] [];
    VarUsing [S::MetaVarExpr] [];
    VarKw [] [];
    VarComma [] [];
    VarCommaConst [] [];
    VarCommaUsing [] [];
    VarSingle [S::MetaVarSingleVariableExpr] [];
    VarInit1 [] [];
    VarInit2 [] [];
    JsxExprNoAttrs [] [];
    JsxNoAttrs [S::MetaTagWithoutAttributes] [S::MetaJsxChildren];
    JsxExprTag [] [];
    JsxTag [S::MetaTag] [];
    JsxTagOpen [] [];
    JsxTagBody [] [S::MetaJsxChildren];
    JsxAttributes [S::MetaTagAttributes] [];
    JsxEval [] [S::MetaEmbeddedExpression];
    JsxStrDouble [S::StringQuotedDouble] [];
    JsxStrSingle [S::StringQuotedSingle] [];
    // Last, so that no other kind's code moves: placed among the entry
    // kinds, it made every dialect run up to 0.15% more instructions.
    EntryTemplate [] [];
}

impl Kind {
    /// The `Kind` as a frame stores it: the TypeScript tag in the high byte,
    /// the discriminant in the low one.
    #[expect(
        clippy::as_conversions,
        reason = "a fieldless `repr(u8)` enum has no `From` impl for its discriminant"
    )]
    pub(crate) const fn code(self) -> u16 {
        TAG | self as u16
    }

    /// The `Kind` a frame's code names.
    /// It is [`Kind::Top`] for a code that is not TypeScript's, such as the
    /// empty state's.
    pub(crate) fn from_code(code: u16) -> Kind {
        if code & 0xff00 != TAG {
            return Kind::Top;
        }
        Kind::ALL
            .get(usize::from(code & 0xff))
            .copied()
            .unwrap_or(Kind::Top)
    }
}
