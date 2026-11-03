//! The contexts the TOML lexer opens, which are frames of the interned state.

use super::scope::S;

/// The high byte that marks a frame as one of TOML's.
const TAG: u16 = crate::tag::TOML;

crate::tag::frame_kinds! {
    /// An open context.
    pub(crate) enum Kind: TAG {
        /// Outside every context: a table of key/value pairs.
        Top,
        /// `[` up to `]`; the payload is an [`ArrayNext`].
        Array,
        /// `{` up to `}`; the payload is an [`InlineComments`].
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

impl Kind {
    /// The scope the context adds to every token inside it, begin and end
    /// included.
    pub(crate) fn scope(self) -> Option<S> {
        match self {
            Kind::Top => None,
            Kind::Array => Some(S::MetaArray),
            Kind::Inline => Some(S::MetaInline),
            Kind::Basic => Some(S::StrDouble),
            Kind::Literal => Some(S::StrSingle),
            Kind::BasicMulti => Some(S::StrTripleDouble),
            Kind::LiteralMulti => Some(S::StrTripleSingle),
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

/// Whether a comment may start a line of the inline table being read, kept
/// in its frame's payload: one may after `{` and `,` only.
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
