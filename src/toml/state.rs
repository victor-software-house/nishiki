//! The contexts the TOML lexer opens, which are frames of the interned state.

use super::scope::S;

/// The high byte that marks a frame as one of TOML's.
const TAG: u16 = crate::tag::TOML;

/// An open context.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub(crate) enum Kind {
    /// Outside every context: a table of key/value pairs.
    Top,
    /// `[` up to `]`; the payload says whether a value has just ended.
    Array,
    /// `{` up to `}`.
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

impl Kind {
    /// The kind as a frame stores it: the TOML tag in the high byte, the
    /// discriminant in the low one.
    #[expect(
        clippy::as_conversions,
        reason = "a fieldless `repr(u8)` enum has no `From` impl for its discriminant"
    )]
    pub(crate) const fn code(self) -> u16 {
        TAG | self as u16
    }

    /// The kind a frame's code names; [`Kind::Top`] for a code that is not
    /// TOML's, such as the empty state's.
    pub(crate) fn from_code(code: u16) -> Kind {
        if code & 0xff00 != TAG {
            return Kind::Top;
        }
        match code & 0xff {
            1 => Kind::Array,
            2 => Kind::Inline,
            3 => Kind::Basic,
            4 => Kind::Literal,
            5 => Kind::BasicMulti,
            6 => Kind::LiteralMulti,
            _ => Kind::Top,
        }
    }

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
