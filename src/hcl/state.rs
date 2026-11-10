//! The contexts the HCL lexer opens.
//! Each is a frame of the interned state.

use super::scope::S;

/// The high byte that marks a frame as one of HCL's.
const TAG: u16 = crate::tag::HCL;

crate::tag::frame_rules! {
    /// An open context, one per begin/end rule of Shiki's grammar.
    pub(crate) enum Rule: TAG {
        /// Outside every context: the body of the file.
        Top,
        /// A block's body, from its header's `{` up to `}`.
        Block,
        /// `{` up to `}`: an object.
        Objects,
        /// A line-leading `(` in an object up to `)` and `=` or `:`: a key
        /// expression.
        ObjectKey,
        /// `[` up to `]`.
        Brackets,
        /// `[for` up to `]`.
        TupleFor,
        /// `{for` up to `}`.
        ObjectFor,
        /// The rest of a line after a `for` that an object or a bracket reads.
        /// It never outlives its line.
        InlineFor,
        /// A function's name and `(` up to `)`.
        Functions,
        /// `(` up to `)`.
        Parens,
        /// `if` up to the line feed.
        InlineIf,
        /// `"` up to `"`.
        String,
        /// `<<LABEL` up to a line that holds only the label.
        /// The payload is the label's interned string.
        Heredoc,
        /// `${` or `%{` up to `}`.
        Interpolation,
        /// `/*` up to `*/`.
        BlockComment,
    }
}

impl Rule {
    /// The scope the context adds to every token inside it, begin and end
    /// included.
    pub(crate) fn scope(self) -> Option<S> {
        match self {
            Rule::Top
            | Rule::Brackets
            | Rule::TupleFor
            | Rule::ObjectFor
            | Rule::InlineFor
            | Rule::Parens
            | Rule::InlineIf => None,
            Rule::Block => Some(S::MetaBlock),
            Rule::Objects => Some(S::MetaBraces),
            Rule::ObjectKey => Some(S::MetaMappingKey),
            Rule::Functions => Some(S::MetaFunctionCall),
            Rule::String => Some(S::StrDouble),
            Rule::Heredoc => Some(S::Heredoc),
            Rule::Interpolation => Some(S::MetaInterpolation),
            Rule::BlockComment => Some(S::CommentBlock),
        }
    }
}
