//! The state carried from one line to the next: a stack of open contexts.

use super::scope::S;

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
    /// `r#"` up to `"#`; the hash count is in [`State::hashes`].
    Raw,
    /// `'` or `b'` up to `'`.
    Char,
    /// `/*` up to `*/`; nests.
    Block,
    /// `/**` up to `*/`; nests.
    Doc,
}

impl Kind {
    /// The discriminant, which fits the four bits of a frame.
    #[expect(
        clippy::as_conversions,
        reason = "a fieldless `repr(u8)` enum has no `From` impl for its discriminant"
    )]
    const fn bits(self) -> u8 {
        self as u8
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

/// The lexer state between lines; [`State::default`] starts a document.
///
/// Small and `Copy`, so a caller can keep one per line and re-highlight from
/// any line. Two equal states lex the following lines identically.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct State {
    /// Open contexts, 4 bits each, outermost in the lowest bits.
    frames: u128,
    depth: u8,
    /// Hash count of the innermost raw string.
    pub(crate) hashes: u8,
    /// Block comments opened beyond [`State::CAPACITY`]; they close first.
    pub(crate) over: u16,
}

impl State {
    /// Most contexts that nest. A deeper context is not tracked: block
    /// comments only count, other contexts are lexed as plain text.
    pub(crate) const CAPACITY: usize = 32;

    pub(crate) fn depth(self) -> usize {
        usize::from(self.depth)
    }

    /// The kind at `index`, outermost first.
    pub(crate) fn kind(self, index: usize) -> Kind {
        let nibble = (self.frames >> (4 * index)) & 0xf;
        Kind::ALL[usize::try_from(nibble).unwrap_or_default()]
    }

    pub(crate) fn top(self) -> Kind {
        match self.depth {
            0 => Kind::Top,
            d => self.kind(usize::from(d) - 1),
        }
    }

    /// Opens a context; false when the stack is full.
    pub(crate) fn push(&mut self, kind: Kind) -> bool {
        if self.depth() == Self::CAPACITY {
            return false;
        }
        self.frames |= u128::from(kind.bits()) << (4 * self.depth());
        self.depth += 1;
        true
    }

    pub(crate) fn pop(&mut self) {
        self.depth -= 1;
        self.frames &= !(0xf << (4 * self.depth()));
    }
}
