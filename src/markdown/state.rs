//! The contexts the Markdown lexer opens, which are frames of the interned
//! state.

/// The high byte that marks a frame as one of Markdown's.
const TAG: u16 = 0x0200;

/// A language that a fence embeds.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) enum Lang {
    Rust,
    /// Markdown itself: the grammar includes its own rules in a `markdown`
    /// fence.
    Markdown,
}

/// A container left open at the end of a line: a begin/end or begin/while rule
/// of the Markdown grammar.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) enum Kind {
    /// The document: outside every container.
    Base,
    /// `>` up to a line without `>`.
    Quote,
    /// `-`, `*` or `+` up to a line that is not indented.
    ListBullet,
    /// `1.` or `1)` up to a line that is not indented.
    ListOrdered,
    /// A fence of an embedded language up to the closing fence. The frame's
    /// payload is the [`Fence`].
    Fence(Lang),
    /// A fence of any other language up to the closing fence. The payload is
    /// the [`Fence`].
    FencePlain,
    /// The lines inside a [`Kind::Fence`], which the embedded language's rules
    /// take. The payload of a Rust body is the state of the Rust lexer.
    Body(Lang),
    /// Four spaces or a tab up to a line without them.
    RawBlock,
    /// `|` up to a line that does not start with `|`.
    Table,
    /// A paragraph, which ends with the next line unless that is indented.
    Paragraph,
    /// `<!--` up to `-->`.
    Comment,
    /// `**` up to `**`.
    BoldStar,
    /// `__` up to `__`.
    BoldUnder,
    /// `*` up to `*`.
    ItalicStar,
    /// `_` up to `_`.
    ItalicUnder,
}

impl Kind {
    const ALL: [Kind; 17] = [
        Kind::Base,
        Kind::Quote,
        Kind::ListBullet,
        Kind::ListOrdered,
        Kind::Fence(Lang::Rust),
        Kind::Fence(Lang::Markdown),
        Kind::FencePlain,
        Kind::Body(Lang::Rust),
        Kind::Body(Lang::Markdown),
        Kind::RawBlock,
        Kind::Table,
        Kind::Paragraph,
        Kind::Comment,
        Kind::BoldStar,
        Kind::BoldUnder,
        Kind::ItalicStar,
        Kind::ItalicUnder,
    ];

    /// The kind as a frame stores it: the Markdown tag in the high byte, the
    /// index in the low one.
    pub(crate) fn code(self) -> u16 {
        let index = Kind::ALL
            .iter()
            .position(|&kind| kind == self)
            .unwrap_or(Kind::ALL.len());
        TAG | u16::try_from(index).unwrap_or(0)
    }

    /// The kind a frame's code names; [`Kind::Base`] for a code that is not
    /// Markdown's.
    pub(crate) fn from_code(code: u16) -> Kind {
        if code & 0xff00 != TAG {
            return Kind::Base;
        }
        Kind::ALL
            .get(usize::from(code & 0xff))
            .copied()
            .unwrap_or(Kind::Base)
    }
}

/// What the closing fence must match: the open fence.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub(crate) struct Fence {
    /// The fence character, `` ` `` or `~`.
    pub(crate) delim: u8,
    /// How many of them opened the fence, at most 255.
    pub(crate) len: u8,
    /// The width of the whitespace before the opening fence when it is
    /// spaces only, else [`Fence::MIXED`].
    pub(crate) indent: u8,
}

impl Fence {
    /// An indent that holds a tab or more than 254 bytes, which no closing
    /// fence is taken to repeat.
    pub(crate) const MIXED: u8 = u8::MAX;

    /// The fence as a frame's payload.
    pub(crate) fn payload(self) -> u32 {
        u32::from(self.delim) << 16 | u32::from(self.len) << 8 | u32::from(self.indent)
    }

    /// The fence a frame's payload holds.
    pub(crate) fn from_payload(payload: u32) -> Fence {
        let byte = |shift: u32| u8::try_from(payload >> shift & 0xff).unwrap_or(0);
        Fence {
            delim: byte(16),
            len: byte(8),
            indent: byte(0),
        }
    }
}
