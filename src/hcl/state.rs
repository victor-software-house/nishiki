//! The contexts the HCL lexer opens.
//! Each is a frame of the interned state.

use super::scope::S;

/// The high byte that marks a frame as one of HCL's.
const TAG: u16 = crate::tag::HCL;

crate::tag::frame_kinds! {
    /// An open context, one per begin/end rule of Shiki's grammar.
    pub(crate) enum Kind: TAG {
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

impl Kind {
    /// The scope the context adds to every token inside it, begin and end
    /// included.
    pub(crate) fn scope(self) -> Option<S> {
        match self {
            Kind::Top
            | Kind::Brackets
            | Kind::TupleFor
            | Kind::ObjectFor
            | Kind::InlineFor
            | Kind::Parens
            | Kind::InlineIf => None,
            Kind::Block => Some(S::MetaBlock),
            Kind::Objects => Some(S::MetaBraces),
            Kind::ObjectKey => Some(S::MetaMappingKey),
            Kind::Functions => Some(S::MetaFunctionCall),
            Kind::String => Some(S::StrDouble),
            Kind::Heredoc => Some(S::Heredoc),
            Kind::Interpolation => Some(S::MetaInterpolation),
            Kind::BlockComment => Some(S::CommentBlock),
        }
    }
}
