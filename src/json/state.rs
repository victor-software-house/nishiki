//! The contexts the JSON lexer opens.
//! Each is a frame of the interned state.

use super::scope::S;

/// The high byte that marks a frame as one of JSON's.
const TAG: u16 = crate::tag::JSON;

crate::tag::frame_rules! {
    /// An open context: a begin/end rule of the JSON grammar.
    pub(crate) enum Rule: TAG {
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
        String,
        /// `/*` up to `*/`.
        Block,
        /// `/**` up to `*/`.
        Doc,
    }
}

impl Rule {
    /// The scopes the context adds to every token inside it, begin and end
    /// included.
    pub(crate) fn scopes(self) -> &'static [S] {
        match self {
            Rule::Top => &[],
            Rule::Array => &[S::MetaArray],
            Rule::Object => &[S::MetaDict],
            Rule::Pair => &[S::MetaPair],
            Rule::Key => &[S::KeyString, S::KeyName],
            Rule::String => &[S::StrQuoted],
            Rule::Block => &[S::CommentBlock],
            Rule::Doc => &[S::CommentDoc],
        }
    }
}
