//! The contexts the Markdown lexer opens.
//! Each is a frame of the interned state.

use crate::Language;

/// The high byte that marks a frame as one of Markdown's.
const TAG: u16 = crate::tag::MARKDOWN;

/// A container left open at the end of a line: a begin/end or begin/while rule
/// of the Markdown grammar.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) enum Container {
    /// The document: outside every container.
    Base,
    /// `>` up to a line without `>`.
    Quote,
    /// `-`, `*` or `+` up to a line that is not indented.
    ListBullet,
    /// `1.` or `1)` up to a line that is not indented.
    ListOrdered,
    /// A fence of an embedded language up to the closing fence.
    /// The frame's carried value is the [`Fence`].
    /// A `markdown` fence embeds Markdown itself.
    /// The grammar includes its own rules there.
    Fence(Language),
    /// A fence of any other language up to the closing fence.
    /// The carried value is the [`Fence`].
    FencePlain,
    /// The lines inside a [`Container::Fence`].
    /// The embedded language's rules take them.
    /// An embedded body's carried value is the state of the embedded
    /// lexer.
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
    /// The carried value is the number of dashes.
    FrontMatter,
    /// The YAML inside [`Container::FrontMatter`].
    /// The carried value is the YAML lexer's state.
    FrontMatterBody,
    /// A `<script>`, `<style>` or `<pre>` HTML block whose line does not close
    /// it, up to the line that does.
    HtmlRaw,
    /// The lines inside [`Container::HtmlRaw`] before the closing one.
    /// html-derivative takes them, and the carried value is its state.
    HtmlRawBody,
    /// Any other HTML block, up to a blank line.
    /// html-derivative takes it, and the carried value is its state.
    HtmlBlock,
    /// A match of html-derivative's root patterns in a paragraph or a
    /// heading, up to where the grammar is back at its root.
    /// The carried value is html-derivative's state.
    HtmlInline,
    /// `(?=<[^>]*?>)` in emphasis up to `(?<=>)`.
    /// html-derivative takes its text, and the carried value is its state.
    HtmlWrap,
    /// A `php` fence up to the closing fence.
    /// The carried value is the [`Fence`].
    /// The grammar's `php` rule includes `text.html.basic` before
    /// `source.php`.
    /// Without a PHP grammar, HTML therefore takes its body, as in Shiki.
    #[cfg(feature = "lang-html")]
    FencePhp,
    /// The lines inside a [`Container::FencePhp`].
    /// The HTML lexer takes them under `meta.embedded.block.php`, and the
    /// carried value is its state.
    #[cfg(feature = "lang-html")]
    BodyPhp,
}

impl Container {
    /// The `Container` values whose code is their own and no language's, in
    /// code order.
    /// A test checks that [`Container::code`] matches it.
    pub(super) const FIXED: &[Container] = &[
        Container::Base,
        Container::Quote,
        Container::ListBullet,
        Container::ListOrdered,
        Container::FencePlain,
        Container::RawBlock,
        Container::Table,
        Container::Paragraph,
        Container::Comment,
        Container::BoldStar,
        Container::BoldUnder,
        Container::ItalicStar,
        Container::ItalicUnder,
        Container::FrontMatter,
        Container::FrontMatterBody,
        Container::HtmlRaw,
        Container::HtmlRawBody,
        Container::HtmlBlock,
        Container::HtmlInline,
        Container::HtmlWrap,
        #[cfg(feature = "lang-html")]
        Container::FencePhp,
        #[cfg(feature = "lang-html")]
        Container::BodyPhp,
    ];

    /// The `Container` as a frame stores it: the Markdown tag in the high byte
    /// and an index in the low one.
    /// The index is the position in [`Container::FIXED`].
    /// After those come two codes per language, its fence then its body, in
    /// [`Language::ALL`] order.
    /// A new language therefore needs no code of its own.
    pub(crate) fn code(self) -> u16 {
        let low = match self {
            Container::Base => 0,
            Container::Quote => 1,
            Container::ListBullet => 2,
            Container::ListOrdered => 3,
            Container::FencePlain => 4,
            Container::RawBlock => 5,
            Container::Table => 6,
            Container::Paragraph => 7,
            Container::Comment => 8,
            Container::BoldStar => 9,
            Container::BoldUnder => 10,
            Container::ItalicStar => 11,
            Container::ItalicUnder => 12,
            Container::FrontMatter => 13,
            Container::FrontMatterBody => 14,
            Container::HtmlRaw => 15,
            Container::HtmlRawBody => 16,
            Container::HtmlBlock => 17,
            Container::HtmlInline => 18,
            Container::HtmlWrap => 19,
            #[cfg(feature = "lang-html")]
            Container::FencePhp => 20,
            #[cfg(feature = "lang-html")]
            Container::BodyPhp => 21,
            Container::Fence(lang) => Container::FIXED.len() + 2 * lang.index(),
            Container::Body(lang) => Container::FIXED.len() + 2 * lang.index() + 1,
        };
        TAG | low_byte(low)
    }

    /// The `Container` a frame's code stands for.
    /// It is [`Container::Base`] for a code that is not Markdown's.
    pub(crate) fn from_code(code: u16) -> Container {
        if code & 0xff00 != TAG {
            return Container::Base;
        }
        let low = usize::from(code & 0xff);
        if let Some(&container) = Container::FIXED.get(low) {
            return container;
        }
        let pair = low - Container::FIXED.len();
        match Language::ALL.get(pair / 2) {
            Some(&lang) if pair.is_multiple_of(2) => Container::Fence(lang),
            Some(&lang) => Container::Body(lang),
            None => Container::Base,
        }
    }
}

/// The fixed containers and a fence and a body per language must fit the low
/// byte of a code.
/// A language past the limit would otherwise share a code with another
/// container. Adding one that does not fit therefore fails the build.
const _: () = assert!(
    Container::FIXED.len() + 2 * Language::ALL.len() <= 0x100,
    "the Markdown container codes no longer fit one byte"
);

/// The low byte of a `Container` code.
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

    /// The fence as a frame's carried value.
    pub(crate) fn carried(self) -> u32 {
        u32::from(self.delim) << 16 | u32::from(self.len) << 8 | u32::from(self.indent)
    }

    /// The fence a frame's carried value holds.
    pub(crate) fn from_carried(carried: u32) -> Fence {
        let byte = |shift: u32| u8::try_from(carried >> shift & 0xff).unwrap_or(0);
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
    fn every_container_round_trips_through_its_code() {
        let languages = Language::ALL
            .iter()
            .flat_map(|&lang| [Container::Fence(lang), Container::Body(lang)]);
        let containers: Vec<Container> =
            Container::FIXED.iter().copied().chain(languages).collect();
        for (index, container) in containers.iter().enumerate() {
            assert_eq!(usize::from(container.code() & 0xff), index, "{container:?}");
            assert_eq!(Container::from_code(container.code()), *container);
        }
    }
}
