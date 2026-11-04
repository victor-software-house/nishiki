//! What each rule of the grammar is called, which rules apply where, and which
//! scope each of a rule's capture groups gets.

use super::scope::S;
use super::state::Kind;
use crate::Language;

/// A rule that matches text: a `match` rule, or the `begin` of a rule that
/// opens a container.
/// The closing rules ([`End`]) are separate.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum P {
    // Blocks.
    /// The zero-width beginning of front matter: `\A(?=(-{3,}))`.
    FrontMatter,
    Separator,
    Heading,
    Quote,
    Bullet,
    Ordered,
    /// A fence of any language.
    /// Matching decides which of the two below it is.
    Fence,
    FenceLanguage,
    FencePlain,
    RawBlock,
    LinkDef,
    Comment,
    /// A raw HTML block: `<script>`, `<style>` or `<pre>`.
    /// Its line does not close it.
    #[cfg(feature = "lang-html-derivative")]
    HtmlRaw,
    /// `(\s*|$)`.
    /// It starts the lines inside a raw HTML block.
    #[cfg(feature = "lang-html-derivative")]
    HtmlRawBody,
    /// The two other HTML blocks, whose matches are the same.
    #[cfg(feature = "lang-html-derivative")]
    HtmlBlock,
    Table,
    Paragraph,
    /// The rule inside an embedded fence that takes its lines.
    EmbedBegin,
    /// The rule inside front matter that takes its first line.
    FrontMatterBegin,
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
    /// The root patterns of html-derivative.
    /// A paragraph and a heading include them after the inline rules.
    #[cfg(feature = "lang-html-derivative")]
    Html,
    /// `(?=<[^>]*?>)`.
    /// Emphasis tries it first.
    #[cfg(feature = "lang-html-derivative")]
    HtmlWrap,
}

impl P {
    /// Whether a block rule can match at a position, given the byte there
    /// (`at`), the first byte after the blanks that follow (`first`), and
    /// whether `^` or `\G` matches there (`anchored`).
    /// Every block rule but the definition starts at such a position.
    /// Each begins by testing one of the two bytes.
    /// The lexer asserts in debug builds that a rule this skips would have
    /// failed.
    pub(crate) fn opens_block(self, anchored: bool, at: u8, first: u8) -> bool {
        if !anchored {
            return self == P::LinkDef;
        }
        match self {
            P::FrontMatter => at == b'-',
            P::Separator => matches!(first, b'-' | b'*' | b'_'),
            P::Heading => first == b'#',
            P::Quote => first == b'>',
            P::Bullet => matches!(first, b'-' | b'*' | b'+'),
            P::Ordered => first.is_ascii_digit(),
            P::Fence => matches!(first, b'`' | b'~'),
            P::RawBlock => matches!(at, b' ' | b'\t'),
            P::LinkDef => first == b'[',
            P::Comment => first == b'<',
            #[cfg(feature = "lang-html-derivative")]
            P::HtmlRaw | P::HtmlBlock => first == b'<',
            #[cfg(feature = "lang-html-derivative")]
            P::HtmlRawBody | P::Html | P::HtmlWrap => true,
            P::Table => at == b'|',
            P::Paragraph
            | P::FenceLanguage
            | P::FencePlain
            | P::EmbedBegin
            | P::FrontMatterBegin
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

    /// Whether an inline rule can start with `byte`.
    /// The lexer then tries only those that can.
    /// The block rules answer true, because they are not chosen by their first
    /// byte.
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
            #[cfg(feature = "lang-html-derivative")]
            P::Html => matches!(byte, b'<' | b'&'),
            #[cfg(feature = "lang-html-derivative")]
            P::HtmlWrap => byte == b'<',
            #[cfg(feature = "lang-html-derivative")]
            P::HtmlRaw | P::HtmlRawBody | P::HtmlBlock => true,
            P::FrontMatter
            | P::FrontMatterBegin
            | P::Separator
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
    FrontMatter,
    Fence,
    Comment,
    Bold,
    Italic,
    /// `(?i)(.*)((</)(script|style|pre)(>))`.
    /// It closes a raw HTML block.
    #[cfg(feature = "lang-html-derivative")]
    HtmlRaw,
}

/// The rules a capture group hands its text to.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum List {
    /// A heading's first group: the rule for its level.
    Heading,
    /// The text of a table cell.
    Inline,
    /// The text of a heading: the inline rules, then html-derivative.
    HeadingText,
    /// The text of a link.
    LinkText,
    /// The text of a strikethrough.
    StrikeText,
}

const BLOCK: &[P] = &[
    P::FrontMatter,
    P::Separator,
    P::Heading,
    P::Quote,
    P::Bullet,
    P::Ordered,
    P::Fence,
    P::RawBlock,
    P::LinkDef,
    P::Comment,
    #[cfg(feature = "lang-html-derivative")]
    P::HtmlRaw,
    #[cfg(feature = "lang-html-derivative")]
    P::HtmlBlock,
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

const HEADING_TEXT: &[P] = &[
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
    #[cfg(feature = "lang-html-derivative")]
    P::Html,
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
    #[cfg(feature = "lang-html-derivative")]
    P::Html,
    P::Setext1,
    P::Setext2,
];

const BOLD: &[P] = &[
    #[cfg(feature = "lang-html-derivative")]
    P::HtmlWrap,
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
    #[cfg(feature = "lang-html-derivative")]
    P::HtmlWrap,
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
    #[cfg(feature = "lang-html-derivative")]
    P::HtmlWrap,
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
        #[cfg(feature = "lang-html")]
        Kind::FencePhp => &[P::EmbedBegin],
        #[cfg(feature = "lang-html")]
        Kind::BodyPhp => &[],
        Kind::FrontMatter => &[P::FrontMatterBegin],
        Kind::Base | Kind::Quote | Kind::ListBullet | Kind::ListOrdered => BLOCK,
        Kind::Body(language) => {
            if language == Language::Markdown {
                BLOCK
            } else {
                &[]
            }
        }
        Kind::FencePlain | Kind::FrontMatterBody | Kind::RawBlock | Kind::Comment => &[],
        #[cfg(feature = "lang-html-derivative")]
        Kind::HtmlRaw => &[P::HtmlRawBody],
        #[cfg(not(feature = "lang-html-derivative"))]
        Kind::HtmlRaw => &[],
        Kind::HtmlRawBody | Kind::HtmlBlock | Kind::HtmlInline | Kind::HtmlWrap => &[],
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
        List::HeadingText => HEADING_TEXT,
        List::LinkText => LINK_TEXT,
        List::StrikeText => STRIKE_TEXT,
    }
}

/// The rule that closes a container, if it has one.
pub(crate) fn end(kind: Kind) -> Option<End> {
    match kind {
        Kind::FrontMatter => Some(End::FrontMatter),
        Kind::Fence(_) | Kind::FencePlain => Some(End::Fence),
        #[cfg(feature = "lang-html")]
        Kind::FencePhp => Some(End::Fence),
        #[cfg(feature = "lang-html")]
        Kind::BodyPhp => None,
        Kind::Comment => Some(End::Comment),
        Kind::BoldStar | Kind::BoldUnder => Some(End::Bold),
        Kind::ItalicStar | Kind::ItalicUnder => Some(End::Italic),
        #[cfg(feature = "lang-html-derivative")]
        Kind::HtmlRaw => Some(End::HtmlRaw),
        #[cfg(not(feature = "lang-html-derivative"))]
        Kind::HtmlRaw => None,
        Kind::HtmlRawBody | Kind::HtmlBlock | Kind::HtmlInline | Kind::HtmlWrap => None,
        Kind::Base
        | Kind::Quote
        | Kind::ListBullet
        | Kind::ListOrdered
        | Kind::Body(_)
        | Kind::FrontMatterBody
        | Kind::RawBlock
        | Kind::Table
        | Kind::Paragraph => None,
    }
}

/// The scopes a container adds to everything inside it, the text that opens
/// and closes it included.
pub(crate) fn names(kind: Kind) -> &'static [S] {
    match kind {
        Kind::Base
        | Kind::Body(_)
        | Kind::FrontMatter
        | Kind::FrontMatterBody
        | Kind::HtmlRaw
        | Kind::HtmlRawBody
        | Kind::HtmlBlock
        | Kind::HtmlInline
        | Kind::HtmlWrap => &[],
        Kind::Quote => &[S::Quote],
        Kind::ListBullet => &[S::ListBullet],
        Kind::ListOrdered => &[S::ListOrdered],
        Kind::Fence(_) | Kind::FencePlain => &[S::Fence],
        #[cfg(feature = "lang-html")]
        Kind::FencePhp => &[S::Fence],
        #[cfg(feature = "lang-html")]
        Kind::BodyPhp => &[],
        Kind::RawBlock => &[S::Raw],
        Kind::Table => &[S::Table],
        Kind::Paragraph => &[S::Paragraph],
        Kind::Comment => &[S::CommentHtml],
        Kind::BoldStar | Kind::BoldUnder => &[S::Bold],
        Kind::ItalicStar | Kind::ItalicUnder => &[S::Italic],
    }
}

/// The scopes each embedded language's fence adds to its text.
/// Each adds one, except C++, whose `contentName` also names `source.cpp`.
/// Shiki's Markdown grammar has no rule that embeds TOML or GraphQL, so they
/// have none.
/// Neither has a language whose feature is off.
const EMBEDDED: &[(Language, &[S])] = &[
    #[cfg(feature = "lang-rust")]
    (Language::Rust, &[S::EmbeddedRust]),
    #[cfg(feature = "lang-typescript")]
    (Language::TypeScript, &[S::EmbeddedTypeScript]),
    #[cfg(feature = "lang-javascript")]
    (Language::JavaScript, &[S::EmbeddedJavascript]),
    #[cfg(feature = "lang-jsx")]
    (Language::Jsx, &[S::EmbeddedJavascript]),
    #[cfg(feature = "lang-tsx")]
    (Language::Tsx, &[S::EmbeddedTypescriptreact]),
    (Language::Markdown, &[S::EmbeddedMarkdown]),
    #[cfg(feature = "lang-shell")]
    (Language::Shell, &[S::EmbeddedShell]),
    #[cfg(feature = "lang-json")]
    (Language::Json, &[S::EmbeddedJson]),
    #[cfg(feature = "lang-json")]
    (Language::Jsonc, &[S::EmbeddedJsonc]),
    #[cfg(feature = "lang-json")]
    (Language::Jsonl, &[S::EmbeddedJsonl]),
    #[cfg(feature = "lang-python")]
    (Language::Python, &[S::EmbeddedPython]),
    #[cfg(feature = "lang-yaml")]
    (Language::Yaml, &[S::EmbeddedYaml]),
    #[cfg(feature = "lang-make")]
    (Language::Make, &[S::EmbeddedMake]),
    #[cfg(feature = "lang-diff")]
    (Language::Diff, &[S::EmbeddedDiff]),
    #[cfg(feature = "lang-dockerfile")]
    (Language::Dockerfile, &[S::EmbeddedDockerfile]),
    #[cfg(feature = "lang-java")]
    (Language::Java, &[S::EmbeddedJava]),
    #[cfg(feature = "lang-c")]
    (Language::C, &[S::EmbeddedC]),
    #[cfg(feature = "lang-cpp")]
    (Language::Cpp, &[S::EmbeddedCpp, S::SourceCpp]),
    #[cfg(feature = "lang-sql")]
    (Language::Sql, &[S::EmbeddedSql]),
    #[cfg(feature = "lang-ini")]
    (Language::Ini, &[S::EmbeddedIni]),
    #[cfg(feature = "lang-css")]
    (Language::Css, &[S::EmbeddedCss]),
    #[cfg(feature = "lang-go")]
    (Language::Go, &[S::EmbeddedGo]),
    #[cfg(feature = "lang-ruby")]
    (Language::Ruby, &[S::EmbeddedRuby]),
    #[cfg(feature = "lang-csharp")]
    (Language::CSharp, &[S::EmbeddedCsharp]),
    #[cfg(feature = "lang-xml")]
    (Language::Xml, &[S::EmbeddedXml]),
    #[cfg(feature = "lang-swift")]
    (Language::Swift, &[S::EmbeddedSwift]),
    #[cfg(feature = "lang-powershell")]
    (Language::PowerShell, &[S::EmbeddedPowershell]),
    #[cfg(feature = "lang-scala")]
    (Language::Scala, &[S::EmbeddedScala]),
    #[cfg(feature = "lang-html")]
    (Language::Html, &[S::EmbeddedHtml]),
    #[cfg(feature = "lang-lua")]
    (Language::Lua, &[S::EmbeddedLua]),
    #[cfg(feature = "lang-dart")]
    (Language::Dart, &[S::EmbeddedDart]),
    #[cfg(feature = "lang-julia")]
    (Language::Julia, &[S::EmbeddedJulia]),
    #[cfg(feature = "lang-scss")]
    (Language::Scss, &[S::EmbeddedScss]),
    #[cfg(feature = "lang-less")]
    (Language::Less, &[S::EmbeddedLess]),
    #[cfg(feature = "lang-bat")]
    (Language::Bat, &[S::EmbeddedDosbatch]),
    #[cfg(feature = "lang-coffee")]
    (Language::CoffeeScript, &[S::EmbeddedCoffee]),
    #[cfg(feature = "lang-php")]
    (Language::Php, &[S::EmbeddedPhp]),
    #[cfg(feature = "lang-objective-c")]
    (Language::ObjectiveC, &[S::EmbeddedObjc]),
];

/// The scopes a container adds to the text between its opening and closing.
pub(crate) fn content_name(kind: Kind) -> &'static [S] {
    match kind {
        Kind::Body(language) => EMBEDDED
            .iter()
            .find_map(|&(embedded, scopes)| (embedded == language).then_some(scopes))
            .unwrap_or_default(),
        #[cfg(feature = "lang-html")]
        Kind::BodyPhp => &[S::EmbeddedPhp],
        #[cfg(feature = "lang-html")]
        Kind::FencePhp => &[],
        Kind::FrontMatterBody => &[S::EmbeddedFrontMatter],
        Kind::Base
        | Kind::Quote
        | Kind::ListBullet
        | Kind::ListOrdered
        | Kind::Fence(_)
        | Kind::FencePlain
        | Kind::FrontMatter
        | Kind::RawBlock
        | Kind::Table
        | Kind::Paragraph
        | Kind::Comment
        | Kind::BoldStar
        | Kind::BoldUnder
        | Kind::ItalicStar
        | Kind::ItalicUnder
        | Kind::HtmlRaw
        | Kind::HtmlRawBody
        | Kind::HtmlBlock
        | Kind::HtmlInline
        | Kind::HtmlWrap => &[],
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
        | P::FrontMatter
        | P::FrontMatterBegin
        | P::TableSeparator
        | P::TableCell
        | P::Bold
        | P::Italic => return None,
        #[cfg(feature = "lang-html-derivative")]
        P::HtmlRaw | P::HtmlRawBody | P::HtmlBlock | P::Html | P::HtmlWrap => return None,
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

/// The title groups of a link, image or definition, starting at group
/// `first`.
/// They are the parenthesised, double-quoted and single-quoted forms.
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
            2 => both(S::HeadingName, List::HeadingText),
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
        P::FrontMatterBegin => match group {
            1 => scope(S::FrontMatterBegin),
            2 => scope(S::FrontMatterComment),
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
        P::FrontMatter
        | P::Separator
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
        #[cfg(feature = "lang-html-derivative")]
        P::HtmlRaw | P::HtmlRawBody | P::HtmlBlock | P::Html | P::HtmlWrap => NONE,
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
///
/// [`capture`] has taken every rule that is not inline, so the last arm
/// gives the rest none.
#[expect(
    clippy::wildcard_enum_match_arm,
    reason = "`capture` handles every rule that is not inline"
)]
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
        _ => NONE,
    }
}

/// The scope of group `group` of the closing rule `end`.
pub(crate) fn end_capture(end: End, group: usize) -> Capture {
    match (end, group) {
        (End::FrontMatter, 0) => scope(S::FrontMatterEnd),
        (End::Fence, 3) => scope(S::FencePunct),
        (End::Comment, 1) => scope(S::CommentHtmlPunct),
        (End::Bold, 1) => scope(S::BoldPunct),
        (End::Italic, 1) => scope(S::ItalicPunct),
        _ => NONE,
    }
}
