//! The contexts the JSON lexer opens, which are frames of the interned state.

use super::scope::S;

/// The high byte that marks a frame as one of JSON's.
const TAG: u16 = crate::tag::JSON;

/// An open context: a begin/end rule of the JSON grammar.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub(crate) enum Kind {
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

impl Kind {
    /// The kind as a frame stores it: the JSON tag in the high byte, the
    /// discriminant in the low one.
    #[expect(
        clippy::as_conversions,
        reason = "a fieldless `repr(u8)` enum has no `From` impl for its discriminant"
    )]
    pub(crate) const fn code(self) -> u16 {
        TAG | self as u16
    }

    /// The kind a frame's code names; [`Kind::Top`] for a code that is not
    /// JSON's, such as the empty state's.
    pub(crate) fn from_code(code: u16) -> Kind {
        if code & 0xff00 != TAG {
            return Kind::Top;
        }
        match code & 0xff {
            1 => Kind::Array,
            2 => Kind::Object,
            3 => Kind::Pair,
            4 => Kind::Key,
            5 => Kind::Str,
            6 => Kind::Block,
            7 => Kind::Doc,
            _ => Kind::Top,
        }
    }

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
