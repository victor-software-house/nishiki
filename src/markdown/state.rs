//! The contexts the Markdown lexer opens, which are frames of the interned
//! state.

use crate::Language;

/// The high byte that marks a frame as one of Markdown's.
const TAG: u16 = 0x0200;

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
    /// payload is the [`Fence`]. A `markdown` fence embeds Markdown itself:
    /// the grammar includes its own rules there.
    Fence(Language),
    /// A fence of any other language up to the closing fence. The payload is
    /// the [`Fence`].
    FencePlain,
    /// The lines inside a [`Kind::Fence`], which the embedded language's rules
    /// take. The payload of an embedded body is the state of the embedded
    /// lexer.
    Body(Language),
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
    /// A first line of dashes up to a line of the same dashes or `...`. The
    /// payload is the number of dashes.
    FrontMatter,
    /// The YAML inside [`Kind::FrontMatter`]; the payload is the YAML lexer's
    /// state.
    FrontMatterBody,
}

impl Kind {
    /// The kinds that name no language, in code order; a test checks that
    /// [`Kind::code`] agrees.
    const FIXED: [Kind; 15] = [
        Kind::Base,
        Kind::Quote,
        Kind::ListBullet,
        Kind::ListOrdered,
        Kind::FencePlain,
        Kind::RawBlock,
        Kind::Table,
        Kind::Paragraph,
        Kind::Comment,
        Kind::BoldStar,
        Kind::BoldUnder,
        Kind::ItalicStar,
        Kind::ItalicUnder,
        Kind::FrontMatter,
        Kind::FrontMatterBody,
    ];

    /// The kind as a frame stores it: the Markdown tag in the high byte and,
    /// in the low one, the position in [`Kind::FIXED`], or after those two
    /// codes per language, its fence then its body, in [`Language::ALL`]
    /// order. A new language therefore needs no code of its own.
    pub(crate) fn code(self) -> u16 {
        let language = |language: Language| {
            Language::ALL
                .iter()
                .position(|&l| l == language)
                .unwrap_or(0)
        };
        let low = match self {
            Kind::Base => 0,
            Kind::Quote => 1,
            Kind::ListBullet => 2,
            Kind::ListOrdered => 3,
            Kind::FencePlain => 4,
            Kind::RawBlock => 5,
            Kind::Table => 6,
            Kind::Paragraph => 7,
            Kind::Comment => 8,
            Kind::BoldStar => 9,
            Kind::BoldUnder => 10,
            Kind::ItalicStar => 11,
            Kind::ItalicUnder => 12,
            Kind::FrontMatter => 13,
            Kind::FrontMatterBody => 14,
            Kind::Fence(lang) => Kind::FIXED.len() + 2 * language(lang),
            Kind::Body(lang) => Kind::FIXED.len() + 2 * language(lang) + 1,
        };
        TAG | u16::try_from(low).unwrap_or(0)
    }

    /// The kind a frame's code names; [`Kind::Base`] for a code that is not
    /// Markdown's.
    pub(crate) fn from_code(code: u16) -> Kind {
        if code & 0xff00 != TAG {
            return Kind::Base;
        }
        let low = usize::from(code & 0xff);
        if let Some(&kind) = Kind::FIXED.get(low) {
            return kind;
        }
        let pair = low - Kind::FIXED.len();
        match Language::ALL.get(pair / 2) {
            Some(&lang) if pair.is_multiple_of(2) => Kind::Fence(lang),
            Some(&lang) => Kind::Body(lang),
            None => Kind::Base,
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_kind_round_trips_through_its_code() {
        let languages = Language::ALL
            .iter()
            .flat_map(|&lang| [Kind::Fence(lang), Kind::Body(lang)]);
        let kinds: Vec<Kind> = Kind::FIXED.into_iter().chain(languages).collect();
        for (index, kind) in kinds.iter().enumerate() {
            assert_eq!(usize::from(kind.code() & 0xff), index, "{kind:?}");
            assert_eq!(Kind::from_code(kind.code()), *kind);
        }
        assert!(kinds.len() <= 0x100, "the codes fit the low byte");
    }
}
