//! What each rule of the grammar is called, which rules apply where, and which
//! scope each of a rule's capture groups gets.

use super::scope::S;
use super::state::{Kind, Lang};

/// A rule that matches text: a `match` rule, or the `begin` of a rule that
/// opens a container. The closing rules ([`End`]) are separate.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum P {
    // Blocks.
    Separator,
    Heading,
    Quote,
    Bullet,
    Ordered,
    /// A fence of any language; matching decides which of the two below it is.
    Fence,
    FenceLanguage,
    FencePlain,
    RawBlock,
    LinkDef,
    Comment,
    Table,
    Paragraph,
    /// The rule inside an embedded fence that takes its lines.
    EmbedBegin,
    // Inside a table.
    TablePipe,
    TableSeparator,
    TableCell,
    // Inside a paragraph.
    Setext1,
    Setext2,
    // Inside a heading's first group, one rule per level.
    Level1,
    Level2,
    Level3,
    Level4,
    Level5,
    Level6,
    // Inline.
    Ampersand,
    Bracket,
    Bold,
    Italic,
    Raw,
    Strike,
    Escape,
    ImageInline,
    ImageRef,
    LinkEmail,
    LinkInet,
    LinkInline,
    LinkRef,
    LinkRefLiteral,
    LinkRefShortcut,
}

impl P {
    /// Whether a block rule can match at a position, given the byte there
    /// (`at`), the first byte after the blanks that follow (`first`), and
    /// whether `^` or `\G` matches there (`anchored`). Every block rule but
    /// the definition starts at such a position; each begins by testing one of
    /// the two bytes, and the lexer asserts in debug builds that a rule this
    /// skips would have failed.
    pub(crate) fn opens_block(self, anchored: bool, at: u8, first: u8) -> bool {
        if !anchored {
            return self == P::LinkDef;
        }
        match self {
            P::Separator => matches!(first, b'-' | b'*' | b'_'),
            P::Heading => first == b'#',
            P::Quote => first == b'>',
            P::Bullet => matches!(first, b'-' | b'*' | b'+'),
            P::Ordered => first.is_ascii_digit(),
            P::Fence => matches!(first, b'`' | b'~'),
            P::RawBlock => matches!(at, b' ' | b'\t'),
            P::LinkDef => first == b'[',
            P::Comment => first == b'<',
            P::Table => at == b'|',
            P::Paragraph
            | P::FenceLanguage
            | P::FencePlain
            | P::EmbedBegin
            | P::TablePipe
            | P::TableSeparator
            | P::TableCell
            | P::Setext1
            | P::Setext2
            | P::Level1
            | P::Level2
            | P::Level3
            | P::Level4
            | P::Level5
            | P::Level6
            | P::Ampersand
            | P::Bracket
            | P::Bold
            | P::Italic
            | P::Raw
            | P::Strike
            | P::Escape
            | P::ImageInline
            | P::ImageRef
            | P::LinkEmail
            | P::LinkInet
            | P::LinkInline
            | P::LinkRef
            | P::LinkRefLiteral
            | P::LinkRefShortcut => true,
        }
    }

    /// Whether an inline rule can start with `byte`, so that the lexer tries
    /// only those that can. The block rules answer true, as they are not
    /// chosen by their first byte.
    pub(crate) fn opens_with(self, byte: u8) -> bool {
        match self {
            P::Ampersand => byte == b'&',
            P::Bracket | P::LinkEmail | P::LinkInet => byte == b'<',
            P::Bold | P::Italic => matches!(byte, b'*' | b'_'),
            P::Raw => byte == b'`',
            P::Strike => byte == b'~',
            P::Escape => byte == b'\\',
            P::ImageInline | P::ImageRef => byte == b'!',
            P::LinkInline | P::LinkRef | P::LinkRefLiteral | P::LinkRefShortcut => byte == b'[',
            P::Setext1 => byte == b'=',
            P::Setext2 => byte == b'-',
            P::Separator
            | P::Heading
            | P::Quote
            | P::Bullet
            | P::Ordered
            | P::Fence
            | P::FenceLanguage
            | P::FencePlain
            | P::RawBlock
            | P::LinkDef
            | P::Comment
            | P::Table
            | P::Paragraph
            | P::EmbedBegin
            | P::TablePipe
            | P::TableSeparator
            | P::TableCell
            | P::Level1
            | P::Level2
            | P::Level3
            | P::Level4
            | P::Level5
            | P::Level6 => true,
        }
    }
}

/// The rule that closes a container.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum End {
    Fence,
    Comment,
    Bold,
    Italic,
}

/// The rules a capture group hands its text to.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum List {
    /// A heading's first group: the rule for its level.
    Heading,
    /// The text of a heading or a table cell.
    Inline,
    /// The text of a link.
    LinkText,
    /// The text of a strikethrough.
    StrikeText,
}

const BLOCK: &[P] = &[
    P::Separator,
    P::Heading,
    P::Quote,
    P::Bullet,
    P::Ordered,
    P::Fence,
    P::RawBlock,
    P::LinkDef,
    P::Comment,
    P::Table,
    P::Paragraph,
];

const INLINE: &[P] = &[
    P::Ampersand,
    P::Bracket,
    P::Bold,
    P::Italic,
    P::Raw,
    P::Strike,
    P::Escape,
    P::ImageInline,
    P::ImageRef,
    P::LinkEmail,
    P::LinkInet,
    P::LinkInline,
    P::LinkRef,
    P::LinkRefLiteral,
    P::LinkRefShortcut,
];

const PARAGRAPH: &[P] = &[
    P::Ampersand,
    P::Bracket,
    P::Bold,
    P::Italic,
    P::Raw,
    P::Strike,
    P::Escape,
    P::ImageInline,
    P::ImageRef,
    P::LinkEmail,
    P::LinkInet,
    P::LinkInline,
    P::LinkRef,
    P::LinkRefLiteral,
    P::LinkRefShortcut,
    P::Setext1,
    P::Setext2,
];

const BOLD: &[P] = &[
    P::Escape,
    P::Ampersand,
    P::Bracket,
    P::Raw,
    P::Bold,
    P::Italic,
    P::ImageInline,
    P::LinkInline,
    P::LinkInet,
    P::LinkEmail,
    P::ImageRef,
    P::LinkRefLiteral,
    P::LinkRef,
    P::LinkRefShortcut,
    P::Strike,
];

const ITALIC: &[P] = &[
    P::Escape,
    P::Ampersand,
    P::Bracket,
    P::Raw,
    P::Bold,
    P::ImageInline,
    P::LinkInline,
    P::LinkInet,
    P::LinkEmail,
    P::ImageRef,
    P::LinkRefLiteral,
    P::LinkRef,
    P::LinkRefShortcut,
    P::Strike,
];

const STRIKE_TEXT: &[P] = &[
    P::Escape,
    P::Ampersand,
    P::Bracket,
    P::Raw,
    P::Bold,
    P::Italic,
    P::ImageInline,
    P::LinkInline,
    P::LinkInet,
    P::LinkEmail,
    P::ImageRef,
    P::LinkRefLiteral,
    P::LinkRef,
    P::LinkRefShortcut,
];

const LINK_TEXT: &[P] = &[P::Raw, P::Bold, P::Italic, P::Strike, P::ImageInline];

const HEADING: &[P] = &[
    P::Level6,
    P::Level5,
    P::Level4,
    P::Level3,
    P::Level2,
    P::Level1,
];

const TABLE: &[P] = &[P::TablePipe, P::TableSeparator, P::TableCell];

/// The rules that apply inside a container.
pub(crate) fn patterns(kind: Kind) -> &'static [P] {
    match kind {
        Kind::Fence(_) => &[P::EmbedBegin],
        Kind::Base
        | Kind::Quote
        | Kind::ListBullet
        | Kind::ListOrdered
        | Kind::Body(Lang::Markdown) => BLOCK,
        Kind::FencePlain
        | Kind::Body(
            Lang::Rust | Lang::TypeScript | Lang::Shell | Lang::Json | Lang::Jsonc | Lang::Jsonl,
        )
        | Kind::RawBlock
        | Kind::Comment => &[],
        Kind::Table => TABLE,
        Kind::Paragraph => PARAGRAPH,
        Kind::BoldStar | Kind::BoldUnder => BOLD,
        Kind::ItalicStar | Kind::ItalicUnder => ITALIC,
    }
}

/// The rules that a capture group's text is handed to.
pub(crate) fn list_patterns(list: List) -> &'static [P] {
    match list {
        List::Heading => HEADING,
        List::Inline => INLINE,
        List::LinkText => LINK_TEXT,
        List::StrikeText => STRIKE_TEXT,
    }
}

/// The rule that closes a container, if it has one.
pub(crate) fn end(kind: Kind) -> Option<End> {
    match kind {
        Kind::Fence(_) | Kind::FencePlain => Some(End::Fence),
        Kind::Comment => Some(End::Comment),
        Kind::BoldStar | Kind::BoldUnder => Some(End::Bold),
        Kind::ItalicStar | Kind::ItalicUnder => Some(End::Italic),
        Kind::Base
        | Kind::Quote
        | Kind::ListBullet
        | Kind::ListOrdered
        | Kind::Body(_)
        | Kind::RawBlock
        | Kind::Table
        | Kind::Paragraph => None,
    }
}

/// The scopes a container adds to everything inside it, the text that opens
/// and closes it included.
pub(crate) fn names(kind: Kind) -> &'static [S] {
    match kind {
        Kind::Base | Kind::Body(_) => &[],
        Kind::Quote => &[S::Quote],
        Kind::ListBullet => &[S::ListBullet],
        Kind::ListOrdered => &[S::ListOrdered],
        Kind::Fence(_) | Kind::FencePlain => &[S::Fence],
        Kind::RawBlock => &[S::Raw],
        Kind::Table => &[S::Table],
        Kind::Paragraph => &[S::Paragraph],
        Kind::Comment => &[S::CommentHtml],
        Kind::BoldStar | Kind::BoldUnder => &[S::Bold],
        Kind::ItalicStar | Kind::ItalicUnder => &[S::Italic],
    }
}

/// The scope a container adds to the text between its opening and closing.
pub(crate) fn content_name(kind: Kind) -> Option<S> {
    match kind {
        Kind::Body(Lang::Rust) => Some(S::EmbeddedRust),
        Kind::Body(Lang::TypeScript) => Some(S::EmbeddedTypeScript),
        Kind::Body(Lang::Markdown) => Some(S::EmbeddedMarkdown),
        Kind::Body(Lang::Shell) => Some(S::EmbeddedShell),
        Kind::Body(Lang::Json) => Some(S::EmbeddedJson),
        Kind::Body(Lang::Jsonc) => Some(S::EmbeddedJsonc),
        Kind::Body(Lang::Jsonl) => Some(S::EmbeddedJsonl),
        Kind::Base
        | Kind::Quote
        | Kind::ListBullet
        | Kind::ListOrdered
        | Kind::Fence(_)
        | Kind::FencePlain
        | Kind::RawBlock
        | Kind::Table
        | Kind::Paragraph
        | Kind::Comment
        | Kind::BoldStar
        | Kind::BoldUnder
        | Kind::ItalicStar
        | Kind::ItalicUnder => None,
    }
}

/// The scope of a rule that matches text and opens no container.
pub(crate) fn match_name(rule: P) -> Option<S> {
    Some(match rule {
        P::Separator => S::Separator,
        P::Heading => S::Heading,
        P::Level1 => S::Heading1,
        P::Level2 => S::Heading2,
        P::Level3 => S::Heading3,
        P::Level4 => S::Heading4,
        P::Level5 => S::Heading5,
        P::Level6 => S::Heading6,
        P::LinkDef => S::LinkDef,
        P::TablePipe => S::TablePunct,
        P::Setext1 => S::Setext1,
        P::Setext2 => S::Setext2,
        P::Ampersand => S::Ampersand,
        P::Bracket => S::Bracket,
        P::Raw => S::InlineRaw,
        P::Strike => S::Strike,
        P::Escape => S::Escape,
        P::ImageInline => S::ImageInline,
        P::ImageRef => S::ImageRef,
        P::LinkEmail => S::LinkEmail,
        P::LinkInet => S::LinkInet,
        P::LinkInline => S::LinkInline,
        P::LinkRef | P::LinkRefShortcut => S::LinkRef,
        P::LinkRefLiteral => S::LinkRefLiteral,
        P::Quote
        | P::Bullet
        | P::Ordered
        | P::Fence
        | P::FenceLanguage
        | P::FencePlain
        | P::RawBlock
        | P::Comment
        | P::Table
        | P::Paragraph
        | P::EmbedBegin
        | P::TableSeparator
        | P::TableCell
        | P::Bold
        | P::Italic => return None,
    })
}

/// What a capture group does with its text: add a scope, hand it to a list of
/// rules, or both.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Capture {
    pub(crate) scope: Option<S>,
    pub(crate) list: Option<List>,
}

impl Capture {
    /// Whether the group has no scope and no rules.
    pub(crate) fn is_none(self) -> bool {
        self.scope.is_none() && self.list.is_none()
    }
}

/// A group that does nothing.
const NONE: Capture = Capture {
    scope: None,
    list: None,
};

const fn scope(scope: S) -> Capture {
    Capture {
        scope: Some(scope),
        list: None,
    }
}

const fn list(list: List) -> Capture {
    Capture {
        scope: None,
        list: Some(list),
    }
}

const fn both(scope: S, list: List) -> Capture {
    Capture {
        scope: Some(scope),
        list: Some(list),
    }
}

/// `capture` when `group` is `only`, and nothing otherwise.
const fn only(group: usize, only: usize, capture: Capture) -> Capture {
    if group == only { capture } else { NONE }
}

/// The title groups of a link, image or definition, which start at group
/// `first`: the parenthesised, double-quoted and single-quoted forms.
fn title(group: usize, first: usize) -> Capture {
    match group.checked_sub(first) {
        Some(0 | 3 | 6) => scope(S::TitleName),
        Some(1 | 4 | 7) => scope(S::TitleBegin),
        Some(2 | 5 | 8) => scope(S::TitleEnd),
        _ => NONE,
    }
}

/// The scope or rules of group `group` of `rule`'s match.
pub(crate) fn capture(rule: P, group: usize) -> Capture {
    match rule {
        P::Heading => only(group, 1, list(List::Heading)),
        P::Level1 | P::Level2 | P::Level3 | P::Level4 | P::Level5 | P::Level6 => match group {
            1 | 3 => scope(S::HeadingPunct),
            2 => both(S::HeadingName, List::Inline),
            _ => NONE,
        },
        P::Quote => only(group, 2, scope(S::QuotePunct)),
        P::Bullet | P::Ordered => only(group, 3, scope(S::ListPunct)),
        P::FenceLanguage => match group {
            3 => scope(S::FencePunct),
            4 => scope(S::FenceLang),
            5 => scope(S::FenceAttrs),
            _ => NONE,
        },
        P::FencePlain => match group {
            3 => scope(S::FencePunct),
            4 => scope(S::FenceInfo),
            _ => NONE,
        },
        P::Comment => only(group, 2, scope(S::CommentHtmlPunct)),
        P::Table => only(group, 2, scope(S::TablePunct)),
        P::TableSeparator => only(group, 1, scope(S::TableSeparator)),
        P::TableCell => only(group, 1, list(List::Inline)),
        P::LinkDef => match group {
            1 | 3 => scope(S::ConstPunct),
            2 => scope(S::ConstRef),
            4 => scope(S::KeyValue),
            5 | 7 => scope(S::LinkPunct),
            6 | 8 => scope(S::UnderlineLink),
            _ => title(group, 9),
        },
        P::Separator
        | P::Fence
        | P::RawBlock
        | P::Paragraph
        | P::EmbedBegin
        | P::TablePipe
        | P::Setext1
        | P::Setext2
        | P::Ampersand
        | P::Bracket
        | P::Escape => NONE,
        P::Bold
        | P::Italic
        | P::Raw
        | P::Strike
        | P::ImageInline
        | P::ImageRef
        | P::LinkEmail
        | P::LinkInet
        | P::LinkInline
        | P::LinkRef
        | P::LinkRefLiteral
        | P::LinkRefShortcut => inline_capture(rule, group),
    }
}

/// [`capture`] for the inline rules.
fn inline_capture(rule: P, group: usize) -> Capture {
    match rule {
        P::Bold => only(group, 1, scope(S::BoldPunct)),
        P::Italic => only(group, 1, scope(S::ItalicPunct)),
        P::Raw => match group {
            1 | 3 => scope(S::RawPunct),
            _ => NONE,
        },
        P::Strike => match group {
            1 | 3 => scope(S::StrikePunct),
            2 => list(List::StrikeText),
            _ => NONE,
        },
        P::ImageInline => match group {
            1 => scope(S::DescBegin),
            2 => scope(S::DescName),
            4 => scope(S::DescEnd),
            5 | 21 => scope(S::Metadata),
            7 | 9 => scope(S::LinkPunct),
            8 | 10 => scope(S::UnderlineImage),
            _ => title(group, 12),
        },
        P::ImageRef => match group {
            1 => scope(S::DescBegin),
            2 => scope(S::DescName),
            4 => scope(S::DescEnd),
            5 | 7 => scope(S::ConstPunct),
            6 => scope(S::ConstRef),
            _ => NONE,
        },
        P::LinkEmail => match group {
            1 => scope(S::LinkPunct),
            2 => scope(S::UnderlineLink),
            _ => NONE,
        },
        P::LinkInet => match group {
            1 | 3 => scope(S::LinkPunct),
            2 => scope(S::UnderlineLink),
            _ => NONE,
        },
        P::LinkInline => match group {
            1 => scope(S::TitleBeginLink),
            2 => both(S::LinkTitle, List::LinkText),
            4 => scope(S::TitleEndLink),
            5 | 21 => scope(S::Metadata),
            7 | 9 => scope(S::LinkPunct),
            8 | 10 => scope(S::UnderlineLink),
            _ => title(group, 12),
        },
        P::LinkRef => match group {
            1 => scope(S::TitleBeginLink),
            2 => both(S::LinkTitle, List::LinkText),
            4 => scope(S::TitleEndLink),
            5 => scope(S::ConstBegin),
            6 => scope(S::ConstRef),
            7 => scope(S::ConstEnd),
            _ => NONE,
        },
        P::LinkRefLiteral => match group {
            1 => scope(S::TitleBeginLink),
            2 => scope(S::LinkTitle),
            4 => scope(S::TitleEndLink),
            5 => scope(S::ConstBegin),
            6 => scope(S::ConstEnd),
            _ => NONE,
        },
        P::LinkRefShortcut => match group {
            1 => scope(S::TitleBeginLink),
            2 => scope(S::LinkTitle),
            3 => scope(S::TitleEndLink),
            _ => NONE,
        },
        P::Separator
        | P::Heading
        | P::Quote
        | P::Bullet
        | P::Ordered
        | P::Fence
        | P::FenceLanguage
        | P::FencePlain
        | P::RawBlock
        | P::LinkDef
        | P::Comment
        | P::Table
        | P::Paragraph
        | P::EmbedBegin
        | P::TablePipe
        | P::TableSeparator
        | P::TableCell
        | P::Setext1
        | P::Setext2
        | P::Level1
        | P::Level2
        | P::Level3
        | P::Level4
        | P::Level5
        | P::Level6
        | P::Ampersand
        | P::Bracket
        | P::Escape => NONE,
    }
}

/// The scope of group `group` of the closing rule `end`.
pub(crate) fn end_capture(end: End, group: usize) -> Capture {
    match (end, group) {
        (End::Fence, 3) => scope(S::FencePunct),
        (End::Comment, 1) => scope(S::CommentHtmlPunct),
        (End::Bold, 1) => scope(S::BoldPunct),
        (End::Italic, 1) => scope(S::ItalicPunct),
        _ => NONE,
    }
}
