//! The contexts the YAML lexer opens.
//! Each is a frame of the interned state.

use super::scope::S;

/// The high byte that marks a frame as one of YAML's.
const TAG: u16 = crate::tag::YAML;

crate::tag::frame_rules! {
    /// An open context.
    pub(crate) enum Rule: TAG {
        /// Outside every context.
        Top,
        /// A block scalar's lines.
        /// The payload is their indentation, 0 until the first line fixes it.
        Block,
        /// A double-quoted scalar that continues on the next line.
        Double,
        /// A single-quoted scalar that continues on the next line.
        Single,
        /// `[` up to `]`.
        FlowSequence,
        /// `{` up to `}`.
        FlowMapping,
        /// A flow pair from its `:` up to the `,` or closing bracket.
        Pair,
        /// `? ` in a block mapping.
        Complex,
        /// `?` in a flow collection, up to the `,` or closing bracket.
        Explicit,
    }
}

impl Rule {
    /// The scopes the context adds to every token inside it.
    pub(crate) fn scopes(self) -> &'static [S] {
        match self {
            Rule::Top => &[],
            Rule::Block => &[S::StrBlock],
            Rule::Double => &[S::StrDouble],
            Rule::Single => &[S::StrSingle],
            Rule::FlowSequence => &[S::MetaFlowSeq],
            Rule::FlowMapping => &[S::MetaFlowMap],
            Rule::Pair => &[S::MetaFlowPair, S::MetaFlowPairValue],
            Rule::Complex => &[S::MetaBlockMap],
            Rule::Explicit => &[S::MetaFlowPairExplicit],
        }
    }
}
