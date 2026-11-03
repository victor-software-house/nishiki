//! The contexts the Rust lexer opens, which are frames of the interned state.

use super::scope::S;

/// The high byte that marks a frame as one of Rust's.
const TAG: u16 = crate::tag::RUST;

/// An open context. Each one is a TextMate-style begin/end rule of the Rust
/// grammar: it adds a scope (some add none) and limits which rules apply.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub(crate) enum Kind {
    /// Outside any context.
    Top,
    /// `fn name(` up to `{` or `;`.
    FnDef,
    /// `name(` up to `)`.
    Call,
    /// `Type<` up to `>`.
    Generic,
    /// `<[` up to `>`.
    AngleSq,
    /// `#[` up to `]`.
    Attr,
    /// `use ` up to `;`.
    Use,
    /// `extern crate` up to `;`.
    Import,
    /// `"` or `b"` up to `"`.
    Str,
    /// `r#"` up to `"#`; the hash count is the frame's payload.
    Raw,
    /// `'` or `b'` up to `'`.
    Char,
    /// `/*` up to `*/`; nests.
    Block,
    /// `/**` up to `*/`; nests.
    Doc,
}

impl Kind {
    /// The kind as a frame stores it: the Rust tag in the high byte, the
    /// discriminant in the low one.
    #[expect(
        clippy::as_conversions,
        reason = "a fieldless `repr(u8)` enum has no `From` impl for its discriminant"
    )]
    pub(crate) const fn code(self) -> u16 {
        TAG | self as u16
    }

    /// The kind a frame's code names; [`Kind::Top`] for a code that is not
    /// Rust's, such as the empty state's.
    pub(crate) fn from_code(code: u16) -> Kind {
        if code & 0xff00 != TAG {
            return Kind::Top;
        }
        Kind::ALL
            .get(usize::from(code & 0xff))
            .copied()
            .unwrap_or(Kind::Top)
    }

    const ALL: [Kind; 13] = [
        Kind::Top,
        Kind::FnDef,
        Kind::Call,
        Kind::Generic,
        Kind::AngleSq,
        Kind::Attr,
        Kind::Use,
        Kind::Import,
        Kind::Str,
        Kind::Raw,
        Kind::Char,
        Kind::Block,
        Kind::Doc,
    ];

    /// The scope the context adds to every token inside it, begin and end
    /// included.
    pub(crate) fn scope(self) -> Option<S> {
        match self {
            Kind::Top | Kind::Generic | Kind::AngleSq => None,
            Kind::FnDef => Some(S::MetaFnDef),
            Kind::Call => Some(S::MetaFnCall),
            Kind::Attr => Some(S::MetaAttr),
            Kind::Use => Some(S::MetaUse),
            Kind::Import => Some(S::MetaImport),
            Kind::Str | Kind::Raw => Some(S::StrDouble),
            Kind::Char => Some(S::StrChar),
            Kind::Block => Some(S::CommentBlock),
            Kind::Doc => Some(S::CommentBlockDoc),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `Kind::ALL` is a hand copy of the enum: each entry must sit at its own
    /// discriminant, or a frame's code reads back as another kind.
    #[test]
    fn every_kind_round_trips_through_its_code() {
        for (index, kind) in Kind::ALL.iter().enumerate() {
            assert_eq!(usize::from(kind.code() & 0xff), index, "{kind:?}");
            assert_eq!(Kind::from_code(kind.code()), *kind);
        }
    }
}
