//! The rules the C lexer opens, and the pattern lists that lex a capture
//! group again.
//! Each is a frame of the interned state.

use super::scope::S;

/// The high byte that marks a frame as one of C's.
const TAG: u16 = crate::tag::C;

macro_rules! rules {
    ($($id:ident $name:tt $content:tt;)*) => {
        /// A begin/end rule of the grammar, or a list of patterns that lexes
        /// a capture group again.
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[repr(u8)]
        pub(crate) enum R { $($id),* }

        impl R {
            /// Every kind, in discriminant order.
            pub(super) const ALL: &[R] = &[$(R::$id),*];

            /// The scopes of the rule's `name`: on its begin, its text and
            /// its end.
            pub(super) const fn name(self) -> &'static [S] {
                match self { $(R::$id => &$name),* }
            }

            /// The scopes of the rule's `contentName`: on its text only.
            pub(super) const fn content(self) -> &'static [S] {
                match self { $(R::$id => &$content),* }
            }
        }
    };
}

rules! {
    Top [] [];
    Block [S::MetaBlock] [];
    BlockInner [] [];
    Parens [S::MetaParens] [];
    ParensBlock [S::MetaParensBlock] [];
    FuncDef [S::MetaFunction] [];
    FuncParams [S::MetaFunctionParams] [];
    FuncParens [] [];
    FuncCall [S::MetaFunctionCall] [];
    CallArgs [] [];
    CallParens [] [];
    Method [] [S::MetaFunctionCallMember];
    Init [S::MetaInitialization] [];
    Ternary [] [];
    Bracket [S::MetaBracketAccess] [];
    DocLine [S::CommentLineDoc] [];
    DocBlock [S::CommentBlockDoc] [];
    BlockComment [S::CommentBlock] [];
    LineLeading [] [];
    LineSlash [S::CommentLine] [];
    LineComment [S::CommentLine] [];
    DQuote [S::StringDouble] [];
    SQuote [S::StringSingle] [];
    Define [S::MetaPreprocessorMacro] [];
    DefineBlock [S::MetaBlock] [];
    DefineBlockInner [] [];
    DefineFunc [S::MetaFunction] [];
    DefineCall [] [];
    DefineParens [] [];
    DefineDQuote [S::StringDouble] [];
    DefineSQuote [S::StringSingle] [];
    Diag [S::MetaPreprocessorDiagnostic] [];
    DiagDQuote [S::StringDouble] [];
    DiagSQuote [S::StringSingle] [];
    DiagText [S::StringUnquoted] [];
    Include [S::MetaPreprocessorInclude] [];
    IncludeDQ [S::StringDoubleInclude] [];
    IncludeLt [S::StringLtGt] [];
    LineDir [S::MetaPreprocessor] [];
    Undef [S::MetaPreprocessor] [];
    PragmaDir [S::MetaPreprocessorPragma] [];
    CondIf [] [];
    CondIfB [] [];
    EnabledIf [] [];
    EnabledIfB [] [];
    DisabledIf [] [];
    DisabledIfB [] [];
    HeadCont [S::MetaPreprocessor] [];
    HeadPlain [S::MetaPreprocessor] [];
    CondElif [S::MetaPreprocessor] [];
    CondTernary [] [];
    CondParens [] [];
    LiveElif [] [];
    LiveElifB [] [];
    ElifTail [] [];
    ElifTailB [] [];
    ElifElse [] [S::CommentElifBranch];
    ElifElseB [] [S::CommentElifBranchIn];
    ElifElif [] [S::CommentElifBranch];
    LiveElse [] [];
    LiveElseB [] [];
    DisabledElif [] [];
    DisabledElifBody [] [S::CommentElifBranch];
    DisabledElifOther [] [];
    DisabledElifOtherB [] [];
    DisabledBody [] [S::CommentIfBranch];
    DisabledBodyB [] [S::CommentIfBranchIn];
    ElseBranch [] [S::CommentElseBranch];
    ElseBranchB [] [S::CommentElseBranchIn];
    ElifBranch [] [S::CommentIfBranch];
    ElifBranchB [] [S::CommentIfBranchIn];
    EnabledBody [] [];
    EnabledBodyB [] [];
    Nest [] [];
    Switch [S::MetaBlockSwitch] [];
    SwitchHead [S::MetaHeadSwitch] [];
    SwitchBody [S::MetaBodySwitch] [];
    SwitchTail [S::MetaTailSwitch] [];
    SwitchParens [S::MetaConditionalSwitch] [];
    Case [S::MetaCase] [];
    Default [S::MetaCase] [];
    Asm [S::MetaAsm] [];
    AsmParens [] [];
    AsmString [S::StringDouble] [S::MetaEmbeddedAsm];
    AsmInner [] [];
    Doxy [] [];
    Inline [] [];
    InlineClose [] [];
    Access [] [];
    DefineParams [] [];
    ParamDir [] [];
    NumSep [] [];
    Number [] [];
}

impl R {
    /// The kind's position in [`R::ALL`].
    #[expect(
        clippy::as_conversions,
        reason = "a fieldless `repr(u8)` enum has no `From` impl for its discriminant"
    )]
    pub(super) const fn index(self) -> usize {
        self as usize
    }

    /// The kind as a frame stores it: the C tag in the high byte, the
    /// discriminant in the low one.
    #[expect(
        clippy::as_conversions,
        reason = "a fieldless `repr(u8)` enum has no `From` impl for its discriminant"
    )]
    pub(super) const fn code(self) -> u16 {
        TAG | self as u16
    }

    /// The kind a frame's code names.
    /// It is [`R::Top`] for a code that is not C's, such as the empty state's.
    pub(super) fn from_code(code: u16) -> R {
        if code & 0xfe00 != TAG {
            return R::Top;
        }
        R::ALL
            .get(usize::from(code & 0xff))
            .copied()
            .unwrap_or(R::Top)
    }
}
