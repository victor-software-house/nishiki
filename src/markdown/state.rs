//! The contexts the Markdown lexer opens, which are frames of the interned
//! state.

/// The high byte that marks a frame as one of Markdown's.
const TAG: u16 = 0x0200;

/// A language that a fence embeds.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) enum Lang {
    Rust,
    /// `TypeScript`, lexed by the TypeScript lexer.
    TypeScript,
    /// Markdown itself: the grammar includes its own rules in a `markdown`
    /// fence.
    Markdown,
    /// Shell, lexed by the shell lexer.
    Shell,
    /// JSON, lexed by the JSON lexer in the `json` spelling.
    Json,
    /// JSON with comments: the `jsonc` spelling.
    Jsonc,
    /// JSON Lines: the `jsonl` spelling.
    Jsonl,
    /// Python, lexed by the Python lexer.
    Python,
    /// YAML, lexed by the YAML lexer.
    Yaml,
    /// The YAML of front matter.
    FrontMatter,
}

impl Lang {
    /// The JSON spelling a fence of this language embeds, for JSON's.
    pub(crate) fn dialect(self) -> Option<crate::json::Dialect> {
        match self {
            Lang::Json => Some(crate::json::Dialect::Json),
            Lang::Jsonc => Some(crate::json::Dialect::Jsonc),
            Lang::Jsonl => Some(crate::json::Dialect::Jsonl),
            Lang::Rust
            | Lang::TypeScript
            | Lang::Markdown
            | Lang::Shell
            | Lang::Python
            | Lang::Yaml
            | Lang::FrontMatter => None,
        }
    }
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
    /// take. The payload of an embedded body is the state of the embedded
    /// lexer.
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
    /// A first line of dashes up to a line of the same dashes or `...`. The
    /// payload is the number of dashes.
    FrontMatter,
}

impl Kind {
    /// The kinds in code order, which [`Kind::code`] numbers; a test checks
    /// that the two agree.
    pub(crate) const ALL: [Kind; 33] = [
        Kind::Base,
        Kind::Quote,
        Kind::ListBullet,
        Kind::ListOrdered,
        Kind::Fence(Lang::Rust),
        Kind::Fence(Lang::TypeScript),
        Kind::Fence(Lang::Markdown),
        Kind::Fence(Lang::Shell),
        Kind::Fence(Lang::Json),
        Kind::Fence(Lang::Jsonc),
        Kind::Fence(Lang::Jsonl),
        Kind::Fence(Lang::Python),
        Kind::FencePlain,
        Kind::Body(Lang::Rust),
        Kind::Body(Lang::TypeScript),
        Kind::Body(Lang::Markdown),
        Kind::Body(Lang::Shell),
        Kind::Body(Lang::Json),
        Kind::Body(Lang::Jsonc),
        Kind::Body(Lang::Jsonl),
        Kind::Body(Lang::Python),
        Kind::RawBlock,
        Kind::Table,
        Kind::Paragraph,
        Kind::Comment,
        Kind::BoldStar,
        Kind::BoldUnder,
        Kind::ItalicStar,
        Kind::ItalicUnder,
        Kind::Fence(Lang::Yaml),
        Kind::Body(Lang::Yaml),
        Kind::FrontMatter,
        Kind::Body(Lang::FrontMatter),
    ];

    /// The kind as a frame stores it: the Markdown tag in the high byte, the
    /// position in [`Kind::ALL`] in the low one.
    pub(crate) fn code(self) -> u16 {
        TAG | match self {
            Kind::Base => 0,
            Kind::Quote => 1,
            Kind::ListBullet => 2,
            Kind::ListOrdered => 3,
            Kind::Fence(Lang::Rust) => 4,
            Kind::Fence(Lang::TypeScript) => 5,
            Kind::Fence(Lang::Markdown) => 6,
            Kind::Fence(Lang::Shell) => 7,
            Kind::Fence(Lang::Json) => 8,
            Kind::Fence(Lang::Jsonc) => 9,
            Kind::Fence(Lang::Jsonl) => 10,
            Kind::Fence(Lang::Python) => 11,
            Kind::FencePlain => 12,
            Kind::Body(Lang::Rust) => 13,
            Kind::Body(Lang::TypeScript) => 14,
            Kind::Body(Lang::Markdown) => 15,
            Kind::Body(Lang::Shell) => 16,
            Kind::Body(Lang::Json) => 17,
            Kind::Body(Lang::Jsonc) => 18,
            Kind::Body(Lang::Jsonl) => 19,
            Kind::Body(Lang::Python) => 20,
            Kind::RawBlock => 21,
            Kind::Table => 22,
            Kind::Paragraph => 23,
            Kind::Comment => 24,
            Kind::BoldStar => 25,
            Kind::BoldUnder => 26,
            Kind::ItalicStar => 27,
            Kind::ItalicUnder => 28,
            Kind::Fence(Lang::Yaml | Lang::FrontMatter) => 29,
            Kind::Body(Lang::Yaml) => 30,
            Kind::FrontMatter => 31,
            Kind::Body(Lang::FrontMatter) => 32,
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_kind_codes_to_its_place_in_all() {
        for (index, kind) in Kind::ALL.iter().enumerate() {
            assert_eq!(usize::from(kind.code() & 0xff), index, "{kind:?}");
            assert_eq!(Kind::from_code(kind.code()), *kind);
        }
    }
}
