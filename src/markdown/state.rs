//! The contexts the Markdown lexer opens.
//! Each is a frame of the interned state.

use crate::Language;

/// The high byte that marks a frame as one of Markdown's.
const TAG: u16 = crate::tag::MARKDOWN;

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
    /// A fence of an embedded language up to the closing fence.
    /// The frame's payload is the [`Fence`].
    /// A `markdown` fence embeds Markdown itself.
    /// The grammar includes its own rules there.
    Fence(Language),
    /// A fence of any other language up to the closing fence.
    /// The payload is the [`Fence`].
    FencePlain,
    /// The lines inside a [`Kind::Fence`].
    /// The embedded language's rules take them.
    /// The payload of an embedded body is the state of the embedded lexer.
    Body(Language),
    /// Four spaces or a tab up to a line without them.
    RawBlock,
    /// `|` up to a line that does not start with `|`.
    Table,
    /// A paragraph.
    /// It ends with the next line unless that line is indented.
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
    /// A first line of dashes up to a line of the same dashes or `...`.
    /// The payload is the number of dashes.
    FrontMatter,
    /// The YAML inside [`Kind::FrontMatter`].
    /// The payload is the YAML lexer's state.
    FrontMatterBody,
    /// A `<script>`, `<style>` or `<pre>` HTML block whose line does not close
    /// it, up to the line that does.
    HtmlRaw,
    /// The lines inside [`Kind::HtmlRaw`] before the closing one.
    /// html-derivative takes them, and the payload is its state.
    HtmlRawBody,
    /// Any other HTML block, up to a blank line.
    /// html-derivative takes it, and the payload is its state.
    HtmlBlock,
    /// A match of html-derivative's root patterns in a paragraph or a
    /// heading, up to where the grammar is back at its root.
    /// The payload is html-derivative's state.
    HtmlInline,
    /// `(?=<[^>]*?>)` in emphasis up to `(?<=>)`.
    /// html-derivative takes its text, and the payload is its state.
    HtmlWrap,
    /// A `php` fence up to the closing fence.
    /// The payload is the [`Fence`].
    /// The grammar's `php` rule includes `text.html.basic` before
    /// `source.php`.
    /// Without a PHP grammar, HTML therefore takes its body, as in Shiki.
    #[cfg(feature = "lang-html")]
    FencePhp,
    /// The lines inside a [`Kind::FencePhp`].
    /// The HTML lexer takes them under `meta.embedded.block.php`, and the
    /// payload is its state.
    #[cfg(feature = "lang-html")]
    BodyPhp,
}

impl Kind {
    /// The kinds whose code is their own, not a language's, in code order.
    /// A test checks that [`Kind::code`] agrees.
    const FIXED: &[Kind] = &[
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
        Kind::HtmlRaw,
        Kind::HtmlRawBody,
        Kind::HtmlBlock,
        Kind::HtmlInline,
        Kind::HtmlWrap,
        #[cfg(feature = "lang-html")]
        Kind::FencePhp,
        #[cfg(feature = "lang-html")]
        Kind::BodyPhp,
    ];

    /// The kind as a frame stores it: the Markdown tag in the high byte and an
    /// index in the low one.
    /// The index is the position in [`Kind::FIXED`].
    /// After those come two codes per language, its fence then its body, in
    /// [`Language::ALL`] order.
    /// A new language therefore needs no code of its own.
    pub(crate) fn code(self) -> u16 {
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
            Kind::HtmlRaw => 15,
            Kind::HtmlRawBody => 16,
            Kind::HtmlBlock => 17,
            Kind::HtmlInline => 18,
            Kind::HtmlWrap => 19,
            #[cfg(feature = "lang-html")]
            Kind::FencePhp => 20,
            #[cfg(feature = "lang-html")]
            Kind::BodyPhp => 21,
            Kind::Fence(lang) => Kind::FIXED.len() + 2 * lang.index(),
            Kind::Body(lang) => Kind::FIXED.len() + 2 * lang.index() + 1,
        };
        TAG | low_byte(low)
    }

    /// The kind a frame's code names.
    /// It is [`Kind::Base`] for a code that is not Markdown's.
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

/// The fixed kinds and a fence and a body per language must fit the low byte of
/// a code.
/// A language past the limit would otherwise share a code with another kind.
/// Adding one that does not fit therefore fails the build.
const _: () = assert!(
    Kind::FIXED.len() + 2 * Language::ALL.len() <= 0x100,
    "the Markdown kind codes no longer fit one byte"
);

/// The low byte of a kind code.
/// The assertion above keeps it below 0x100.
#[expect(
    clippy::as_conversions,
    clippy::cast_possible_truncation,
    reason = "`u16::try_from` is not infallible here; the assert bounds the value"
)]
const fn low_byte(low: usize) -> u16 {
    (low & 0xff) as u16
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
    /// An indent that holds a tab or more than 254 bytes.
    /// No closing fence is taken to repeat it.
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
        let kinds: Vec<Kind> = Kind::FIXED.iter().copied().chain(languages).collect();
        for (index, kind) in kinds.iter().enumerate() {
            assert_eq!(usize::from(kind.code() & 0xff), index, "{kind:?}");
            assert_eq!(Kind::from_code(kind.code()), *kind);
        }
    }
}
