//! The rules the C lexer opens, and the pattern lists that lex a capture
//! group again.
//! Each is a frame of the interned state.

use super::scope::S;
use crate::scope::Scopes;

/// The high byte that marks a frame as one of C's.
const TAG: u16 = crate::tag::C;

macro_rules! rules {
    (@scopes []) => {
        S::NO_SCOPES
    };
    (@scopes [$($scope:expr),+ $(,)?]) => {
        crate::scope::scopes![$($scope),+]
    };
    ($($id:ident $name:tt $content:tt;)*) => {
        /// A begin/end rule of the grammar, or a list of patterns that lexes
        /// a capture group again.
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[repr(u8)]
        pub(crate) enum Rule { $($id),* }

        impl Rule {
            /// Every rule, in discriminant order.
            pub(super) const ALL: &[Rule] = &[$(Rule::$id),*];

            /// The scopes of the rule's `name`: on its begin, its text and
            /// its end.
            pub(super) const fn name(self) -> Scopes<S> {
                match self { $(Rule::$id => rules!(@scopes $name)),* }
            }

            /// The scopes of the rule's `contentName`: on its text only.
            pub(super) const fn content(self) -> Scopes<S> {
                match self { $(Rule::$id => rules!(@scopes $content)),* }
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
    FunctionDefinition [S::MetaFunction] [];
    FunctionParameters [S::MetaFunctionParams] [];
    FunctionParens [] [];
    FunctionCall [S::MetaFunctionCall] [];
    CallArguments [] [];
    CallParens [] [];
    Method [] [S::MetaFunctionCallMember];
    Initialization [S::MetaInitialization] [];
    Ternary [] [];
    Bracket [S::MetaBracketAccess] [];
    DocLine [S::CommentLineDoc] [];
    DocBlock [S::CommentBlockDoc] [];
    BlockComment [S::CommentBlock] [];
    LineLeading [] [];
    LineSlash [S::CommentLine] [];
    LineComment [S::CommentLine] [];
    DoubleQuote [S::StringDouble] [];
    SingleQuote [S::StringSingle] [];
    Define [S::MetaPreprocessorMacro] [];
    DefineBlock [S::MetaBlock] [];
    DefineBlockInner [] [];
    DefineFunction [S::MetaFunction] [];
    DefineCall [] [];
    DefineParens [] [];
    DefineDoubleQuote [S::StringDouble] [];
    DefineSingleQuote [S::StringSingle] [];
    Diagnostic [S::MetaPreprocessorDiagnostic] [];
    DiagnosticDoubleQuote [S::StringDouble] [];
    DiagnosticSingleQuote [S::StringSingle] [];
    DiagnosticText [S::StringUnquoted] [];
    Include [S::MetaPreprocessorInclude] [];
    IncludeDoubleQuote [S::StringDoubleInclude] [];
    IncludeAngle [S::StringLtGt] [];
    LineDirective [S::MetaPreprocessor] [];
    Undef [S::MetaPreprocessor] [];
    PragmaDirective [S::MetaPreprocessorPragma] [];
    ConditionalIf [] [];
    ConditionalIfBlock [] [];
    EnabledIf [] [];
    EnabledIfBlock [] [];
    DisabledIf [] [];
    DisabledIfBlock [] [];
    HeadContinuation [S::MetaPreprocessor] [];
    HeadPlain [S::MetaPreprocessor] [];
    ConditionalElif [S::MetaPreprocessor] [];
    ConditionalTernary [] [];
    ConditionalParens [] [];
    LiveElif [] [];
    LiveElifBlock [] [];
    ElifTail [] [];
    ElifTailBlock [] [];
    ElifElse [] [S::CommentElifBranch];
    ElifElseBlock [] [S::CommentElifBranchIn];
    ElifElif [] [S::CommentElifBranch];
    LiveElse [] [];
    LiveElseBlock [] [];
    DisabledElif [] [];
    DisabledElifBody [] [S::CommentElifBranch];
    DisabledElifOther [] [];
    DisabledElifOtherBlock [] [];
    DisabledBody [] [S::CommentIfBranch];
    DisabledBodyBlock [] [S::CommentIfBranchIn];
    ElseBranch [] [S::CommentElseBranch];
    ElseBranchBlock [] [S::CommentElseBranchIn];
    ElifBranch [] [S::CommentIfBranch];
    ElifBranchBlock [] [S::CommentIfBranchIn];
    EnabledBody [] [];
    EnabledBodyBlock [] [];
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
    Doxygen [] [];
    Inline [] [];
    InlineClose [] [];
    Access [] [];
    DefineParameters [] [];
    ParameterDirections [] [];
    NumberSeparator [] [];
    Number [] [];
}

impl Rule {
    /// The rule's position in [`Rule::ALL`].
    #[expect(
        clippy::as_conversions,
        reason = "a fieldless `repr(u8)` enum has no `From` impl for its discriminant"
    )]
    pub(super) const fn index(self) -> usize {
        self as usize
    }

    /// The rule as a frame stores it: the C tag in the high byte, the
    /// discriminant in the low one.
    #[expect(
        clippy::as_conversions,
        reason = "a fieldless `repr(u8)` enum has no `From` impl for its discriminant"
    )]
    pub(super) const fn code(self) -> u16 {
        TAG | self as u16
    }

    /// The rule a frame's code names.
    /// It is [`Rule::Top`] for a code that is not C's, such as the empty
    /// state's.
    pub(super) fn from_code(code: u16) -> Rule {
        if code & 0xfe00 != TAG {
            return Rule::Top;
        }
        Rule::ALL
            .get(usize::from(code & 0xff))
            .copied()
            .unwrap_or(Rule::Top)
    }
}
