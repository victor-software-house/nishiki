//! The contexts the YAML lexer opens, which are frames of the interned state.

use super::scope::S;

/// The high byte that marks a frame as one of YAML's.
const TAG: u16 = crate::tag::YAML;

crate::tag::frame_kinds! {
    /// An open context.
    pub(crate) enum Kind: TAG {
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
}

impl Kind {
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
