//! The contexts the TOML lexer opens.
//! Each is a frame of the interned state.

use super::scope::S;

/// The high byte that marks a frame as one of TOML's.
const TAG: u16 = crate::tag::TOML;

crate::tag::frame_rules! {
    /// An open context.
    pub(crate) enum Rule: TAG {
        /// Outside every context: a table of key/value pairs.
        Top,
        /// `[` up to `]`.
        /// The payload is an [`ArrayNext`].
        Array,
        /// `{` up to `}`.
        /// The payload is an [`InlineComments`].
        Inline,
        /// `"` up to `"`.
        Basic,
        /// `'` up to `'`.
        Literal,
        /// `"""` up to `"""`.
        BasicMulti,
        /// `'''` up to `'''`.
        LiteralMulti,
    }
}

impl Rule {
    /// The scope the context adds to every token inside it, begin and end
    /// included.
    pub(crate) fn scope(self) -> Option<S> {
        match self {
            Rule::Top => None,
            Rule::Array => Some(S::MetaArray),
            Rule::Inline => Some(S::MetaInline),
            Rule::Basic => Some(S::StrDouble),
            Rule::Literal => Some(S::StrSingle),
            Rule::BasicMulti => Some(S::StrTripleDouble),
            Rule::LiteralMulti => Some(S::StrTripleSingle),
        }
    }
}

/// What the array being read expects next, kept in its frame's payload.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ArrayNext {
    /// A value: the array has just opened or read a comma.
    Value,
    /// The comma after the value that has just ended.
    Comma,
}

impl ArrayNext {
    pub(crate) fn payload(self) -> u32 {
        match self {
            ArrayNext::Value => 0,
            ArrayNext::Comma => 1,
        }
    }

    pub(crate) fn from_payload(payload: u32) -> ArrayNext {
        if payload == 0 {
            ArrayNext::Value
        } else {
            ArrayNext::Comma
        }
    }
}

/// Whether a comment may start a line of the inline table being read.
/// It is kept in the frame's payload.
/// A comment may start a line only after `{` and `,`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum InlineComments {
    Allowed,
    Barred,
}

impl InlineComments {
    pub(crate) fn payload(self) -> u32 {
        match self {
            InlineComments::Barred => 0,
            InlineComments::Allowed => 1,
        }
    }

    pub(crate) fn from_payload(payload: u32) -> InlineComments {
        if payload == 0 {
            InlineComments::Barred
        } else {
            InlineComments::Allowed
        }
    }
}
