//! The contexts of the grammar: each begin/end rule that holds state across
//! tokens, with the scopes it adds.
//! Each is a frame of the highlighter's interned state.

use super::scope::S;

/// The high byte that marks a frame as one of TypeScript's.
const TAG: u16 = crate::tag::TYPESCRIPT;

macro_rules! rules {
    ($($id:ident [$($name:expr),*] [$($content:expr),*];)*) => {
        /// An open context.
        /// The scopes in `names` apply to the context's begin and end tokens
        /// and to everything inside.
        /// Those in `content` apply only to what lies between begin and end.
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[repr(u8)]
        pub(crate) enum Rule { $($id),* }

        impl Rule {
            /// Every `Rule`, indexed by discriminant.
            const ALL: &[Rule] = &[$(Rule::$id),*];

            /// How many rules there are.
            /// The count bounds the lexer's steps without advancing.
            #[cfg(debug_assertions)]
            pub(crate) const COUNT: usize = Self::ALL.len();

            /// The scopes of the begin and end tokens and of the content.
            pub(crate) const fn names(self) -> &'static [S] {
                match self { $(Rule::$id => &[$($name),*]),* }
            }

            /// The scopes of the content only.
            pub(crate) const fn content(self) -> &'static [S] {
                match self { $(Rule::$id => &[$($content),*]),* }
            }
        }
    };
}

rules! {
    Top [] [];
    Started [] [];
    Script [] [];
    Expression [] [];
    EntryObjectLiteral [] [];
    EntryObjectBinding [] [];
    EntryObjectBindingConst [] [];
    EntryArrayBinding [] [];
    EntryArrayBindingConst [] [];
    EntryTypeParameters [] [];
    Block [S::MetaBlock] [];
    ObjectLiteral [S::MetaObjectliteral] [];
    ArrayLiteral [S::MetaArrayLiteral] [];
    ArrayBinding [] [];
    ArrayBindingConst [] [];
    ObjectBinding [] [];
    ObjectBindingConst [] [];
    ParenExpression [] [];
    ArrowHead [S::MetaArrow] [];
    ArrowBody [S::MetaArrow] [];
    ArrowReturn [S::MetaReturnTypeArrow] [];
    ArrowReturnObject [] [];
    PossibleArrowReturn [] [S::MetaArrow, S::MetaReturnTypeArrow];
    Parameters [S::MetaParameters] [];
    Cast [S::CastExpr] [];
    JsDocBraces [] [];
    JsDocBrackets [] [];
    ClassDeclaration [S::MetaClass] [];
    ClassBody [] [];
    ClassColon [] [];
    Heritage [] [];
    CommentDoc [S::CommentBlockDocumentation] [];
    CommentBlock [S::CommentBlock] [];
    CommentLine [] [S::CommentLineDoubleSlash];
    CommentLineNewline [] [S::CommentLineDoubleSlash];
    ScriptBlockComment [S::CommentBlock] [];
    ScriptLineComment [S::CommentLineDoubleSlash] [];
    ForLoop [] [];
    ForParen [] [];
    Return [] [];
    SwitchStatement [S::SwitchStatementExpr] [];
    SwitchExpression [S::SwitchExpressionExpr] [];
    SwitchBlock [S::SwitchBlockExpr] [];
    CaseClause [S::CaseClauseExpr] [];
    CaseBlock [] [S::MetaBlock];
    IfOuter [] [];
    IfParen [] [];
    Decorator [S::MetaDecorator] [];
    ObjectBindingVariable [S::MetaObjectBindingPatternVariable] [];
    ArrayBindingVariable [S::MetaArrayBindingPatternVariable] [];
    ObjectBindingVariableConst [S::MetaObjectBindingPatternVariable] [];
    ArrayBindingVariableConst [S::MetaArrayBindingPatternVariable] [];
    ParameterObjectBinding [S::MetaParameterObjectBindingPattern] [];
    ParameterArrayBinding [S::MetaParameterArrayBindingPattern] [];
    ParameterObjectPattern [] [];
    ParameterArrayPattern [] [];
    Directive [S::CommentLineTripleSlashDirective] [];
    DirectiveTag [S::MetaTag] [];
    JsDocExample [S::MetaExampleJsdoc] [];
    JsDocCaption [] [S::ConstantOtherDescriptionJsdoc];
    JsDocTypeTag [] [];
    JsDocParamTag [] [];
    JsDocTypedefTag [] [];
    JsDocReturnTag [] [];
    JsDocDefaultQuote [] [S::VariableOtherJsdoc];
    JsDocInlineLink [S::EntityNameTypeInstanceJsdoc] [];
    JsDocType [] [S::EntityNameTypeInstanceJsdoc];
    EnumDeclaration [S::MetaEnumDeclaration] [];
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
    FunctionDeclaration [S::MetaFunction] [];
    FunctionExpression [S::MetaFunctionExpression] [];
    Import [S::MetaImport] [];
    ImportFrom [] [];
    ImportEqualsExternal [S::MetaImportEqualsExternal] [];
    ImportEqualsInternal [S::MetaImportEqualsInternal] [];
    AssertClause [] [];
    ImportBlock [S::MetaBlock] [];
    Indexer [S::MetaIndexerDeclaration] [];
    IndexerMapped [S::MetaIndexerMappedtypeDeclaration] [];
    InstanceOf [] [];
    InterfaceDeclaration [S::MetaInterface] [];
    LabelBlock [] [];
    Method [S::MetaMethodDeclaration] [];
    MethodName [] [];
    ObjectMethod [S::MetaMethodDeclaration] [];
    ObjectMethodNested [] [];
    NamespaceDeclaration [S::MetaNamespaceDeclaration] [];
    NewExpression [S::NewExpr] [];
    ObjectBindingElement [] [];
    ObjectBindingElementConst [] [];
    ObjectBindingProperty [] [];
    ParameterObjectElement [] [];
    ObjectKeyBracket [S::MetaObjectMember, S::MetaObjectLiteralKey] [];
    ObjectKeyString [S::MetaObjectMember, S::MetaObjectLiteralKey] [];
    ObjectKeyNumber [S::MetaObjectMember, S::MetaObjectLiteralKey] [];
    ObjectMethodAfterKey [S::MetaMethodDeclaration] [];
    ObjectMember [S::MetaObjectMember] [];
    ObjectSpread [S::MetaObjectMember] [];
    ObjectAs [S::MetaObjectMember] [];
    ObjectInitializer [S::MetaObjectMember] [];
    ObjectPropertyValue [S::MetaObjectMember] [];
    ObjectArrowGeneric [] [];
    ObjectArrowParen [] [];
    ObjectArrowOpen [] [];
    AsType [] [];
    YieldComment [] [];
    ParameterTypeAnnotation [S::MetaTypeAnnotation] [];
    PossibleArrow [] [];
    ParenArrowParameters [] [];
    Regex [S::StringRegexp] [];
    RegexGroup [S::MetaGroupRegexp] [];
    RegexAssert [S::MetaGroupAssertionRegexp] [];
    RegexSet [S::ConstantOtherCharacterClassSetRegexp] [];
    ReturnType1 [S::MetaReturnType] [];
    ReturnType2 [S::MetaReturnType] [];
    ReturnObject [] [];
    StringSingle [S::StringQuotedSingle] [];
    StringDouble [S::StringQuotedDouble] [];
    Template [] [S::StringTemplate];
    TemplateExpression [S::MetaTemplateExpression] [S::MetaEmbeddedLine];
    TemplateCall [] [];
    TemplateCallTarget [] [];
    TemplateCall2 [] [];
    TemplateType [] [S::StringTemplate];
    TemplateTypeExpression [S::MetaTemplateExpression] [S::MetaEmbeddedLine];
    Ternary [] [];
    TypeAnnotation1 [S::MetaTypeAnnotation] [];
    TypeAnnotation2 [S::MetaTypeAnnotation] [];
    TypeArguments [S::MetaTypeParameters] [];
    TypeNameArguments [] [S::MetaTypeParameters];
    TypeAlias [S::MetaTypeDeclaration] [];
    TypeAliasEquals [] [];
    TypeConditional [] [];
    TypeConditionalQuestionMark [] [];
    TypeConstructorTypeParameters [] [];
    TypeConstructor [S::MetaTypeConstructor] [];
    TypeFunction [S::MetaTypeFunction] [];
    TypeFunctionReturn1 [S::MetaTypeFunctionReturn] [];
    TypeFunctionReturn2 [S::MetaTypeFunctionReturn] [];
    TypeFunctionReturnObject [] [];
    TypeObject [S::MetaObjectType] [];
    TypeSpread [] [];
    TypeOperatorObject [] [];
    TypeOperator [] [];
    TypeParameters [S::MetaTypeParameters] [];
    TypeParen [S::MetaTypeParenCover] [];
    TypeTuple [S::MetaTypeTuple] [];
    Typeof [] [];
    VariableLet [S::MetaVarExpr] [];
    VariableConst [S::MetaVarExpr] [];
    VariableUsing [S::MetaVarExpr] [];
    VariableKeyword [] [];
    VariableComma [] [];
    VariableCommaConst [] [];
    VariableCommaUsing [] [];
    VariableSingle [S::MetaVarSingleVariableExpr] [];
    VariableInitializer1 [] [];
    VariableInitializer2 [] [];
    JsxExpressionNoAttributes [] [];
    JsxNoAttributes [S::MetaTagWithoutAttributes] [S::MetaJsxChildren];
    JsxExpressionTag [] [];
    JsxTag [S::MetaTag] [];
    JsxTagOpen [] [];
    JsxTagBody [] [S::MetaJsxChildren];
    JsxAttributes [S::MetaTagAttributes] [];
    JsxEmbeddedExpression [] [S::MetaEmbeddedExpression];
    JsxStringDouble [S::StringQuotedDouble] [];
    JsxStringSingle [S::StringQuotedSingle] [];
    // Last, so that no other rule's code moves: placed among the entry
    // rules, it made every dialect run up to 0.15% more instructions.
    EntryTemplate [] [];
}

impl Rule {
    /// The `Rule` as a frame stores it: the TypeScript tag in the high byte,
    /// the discriminant in the low one.
    #[expect(
        clippy::as_conversions,
        reason = "a fieldless `repr(u8)` enum has no `From` impl for its discriminant"
    )]
    pub(crate) const fn code(self) -> u16 {
        TAG | self as u16
    }

    /// The `Rule` a frame's code names.
    /// It is [`Rule::Top`] for a code that is not TypeScript's, such as the
    /// empty state's.
    pub(crate) fn from_code(code: u16) -> Rule {
        if code & 0xff00 != TAG {
            return Rule::Top;
        }
        Rule::ALL
            .get(usize::from(code & 0xff))
            .copied()
            .unwrap_or(Rule::Top)
    }
}
