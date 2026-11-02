//! The contexts the YAML lexer opens, which are frames of the interned state.

use super::scope::S;

/// The high byte that marks a frame as one of YAML's.
const TAG: u16 = 0x0800;

/// An open context.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub(crate) enum Kind {
    /// Outside every context.
    Top,
    /// A block scalar's lines; the payload is their indentation, 0 until the
    /// first line fixes it.
    Block,
    /// A double-quoted scalar that continues on the next line.
    Double,
    /// A single-quoted scalar that continues on the next line.
    Single,
    /// `[` up to `]`.
    FlowSeq,
    /// `{` up to `}`.
    FlowMap,
    /// A flow pair from its `:` up to the `,` or closing bracket.
    Pair,
    /// `? ` in a block mapping.
    Complex,
    /// `?` in a flow collection, up to the `,` or closing bracket.
    Explicit,
}

impl Kind {
    /// The kind as a frame stores it.
    #[expect(
        clippy::as_conversions,
        reason = "a fieldless `repr(u8)` enum has no `From` impl for its discriminant"
    )]
    pub(crate) const fn code(self) -> u16 {
        TAG | self as u16
    }

    /// The kind a frame's code names; [`Kind::Top`] for a code that is not
    /// YAML's, such as the empty state's.
    pub(crate) fn from_code(code: u16) -> Kind {
        if code & 0xff00 != TAG {
            return Kind::Top;
        }
        match code & 0xff {
            1 => Kind::Block,
            2 => Kind::Double,
            3 => Kind::Single,
            4 => Kind::FlowSeq,
            5 => Kind::FlowMap,
            6 => Kind::Pair,
            7 => Kind::Complex,
            8 => Kind::Explicit,
            _ => Kind::Top,
        }
    }

    /// The scopes the context adds to every token inside it.
    pub(crate) fn scopes(self) -> &'static [S] {
        match self {
            Kind::Top => &[],
            Kind::Block => &[S::StrBlock],
            Kind::Double => &[S::StrDouble],
            Kind::Single => &[S::StrSingle],
            Kind::FlowSeq => &[S::MetaFlowSeq],
            Kind::FlowMap => &[S::MetaFlowMap],
            Kind::Pair => &[S::MetaFlowPair, S::MetaFlowPairValue],
            Kind::Complex => &[S::MetaBlockMap],
            Kind::Explicit => &[S::MetaFlowPairExplicit],
        }
    }
}
