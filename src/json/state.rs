//! The contexts the JSON lexer opens.
//! Each is a frame of the interned state.

use super::scope::S;

/// The high byte that marks a frame as one of JSON's.
const TAG: u16 = crate::tag::JSON;

crate::tag::frame_kinds! {
    /// An open context: a begin/end rule of the JSON grammar.
    pub(crate) enum Kind: TAG {
        /// Outside every context.
        Top,
        /// `[` up to `]`.
        Array,
        /// `{` up to `}`.
        Object,
        /// `:` up to `,` or the `}` that closes the object.
        Pair,
        /// A property name: `"` up to `"`.
        Key,
        /// A string value: `"` up to `"`.
        Str,
        /// `/*` up to `*/`.
        Block,
        /// `/**` up to `*/`.
        Doc,
    }
}

impl Kind {
    /// The scopes the context adds to every token inside it, begin and end
    /// included.
    pub(crate) fn scopes(self) -> &'static [S] {
        match self {
            Kind::Top => &[],
            Kind::Array => &[S::MetaArray],
            Kind::Object => &[S::MetaDict],
            Kind::Pair => &[S::MetaPair],
            Kind::Key => &[S::KeyString, S::KeyName],
            Kind::Str => &[S::StrQuoted],
            Kind::Block => &[S::CommentBlock],
            Kind::Doc => &[S::CommentDoc],
        }
    }
}
