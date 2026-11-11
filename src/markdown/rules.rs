//! What each rule of the grammar is called, which rules apply where, and which
//! scope each of a rule's capture groups gets.

use super::scope::S;
use super::state::Container;
use crate::Language;

/// A rule that matches text: a `match` rule, or the `begin` of a rule that
/// opens a container.
/// The closing rules ([`End`]) are separate.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Pattern {
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
    LinkDefinition,
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
    ImageReference,
    LinkEmail,
    LinkInternet,
    LinkInline,
    LinkReference,
    LinkReferenceLiteral,
    LinkReferenceShortcut,
    /// The root patterns of html-derivative.
    /// A paragraph and a heading include them after the inline rules.
    #[cfg(feature = "lang-html-derivative")]
    Html,
    /// `(?=<[^>]*?>)`.
    /// Emphasis tries it first.
    #[cfg(feature = "lang-html-derivative")]
    HtmlWrap,
}

impl Pattern {
    /// Whether a block rule can match at a position.
    ///
    /// It takes the byte there (`at`) and the first byte after the blanks that
    /// follow (`first`). It also takes whether `^` or `\G` matches there
    /// (`anchored`).
    /// Every block rule but the definition starts at such a position, and each
    /// begins by testing one of the two bytes.
    /// The lexer asserts in debug builds that a rule this skips would have
    /// failed.
    pub(crate) fn opens_block(self, anchored: bool, at: u8, first: u8) -> bool {
        if !anchored {
            return self == Pattern::LinkDefinition;
        }
        match self {
            Pattern::FrontMatter => at == b'-',
            Pattern::Separator => matches!(first, b'-' | b'*' | b'_'),
            Pattern::Heading => first == b'#',
            Pattern::Quote => first == b'>',
            Pattern::Bullet => matches!(first, b'-' | b'*' | b'+'),
            Pattern::Ordered => first.is_ascii_digit(),
            Pattern::Fence => matches!(first, b'`' | b'~'),
            Pattern::RawBlock => matches!(at, b' ' | b'\t'),
            Pattern::LinkDefinition => first == b'[',
            Pattern::Comment => first == b'<',
            #[cfg(feature = "lang-html-derivative")]
            Pattern::HtmlRaw | Pattern::HtmlBlock => first == b'<',
            #[cfg(feature = "lang-html-derivative")]
            Pattern::HtmlRawBody | Pattern::Html | Pattern::HtmlWrap => true,
            Pattern::Table => at == b'|',
            Pattern::Paragraph
            | Pattern::FenceLanguage
            | Pattern::FencePlain
            | Pattern::EmbedBegin
            | Pattern::FrontMatterBegin
            | Pattern::TablePipe
            | Pattern::TableSeparator
            | Pattern::TableCell
            | Pattern::Setext1
            | Pattern::Setext2
            | Pattern::Level1
            | Pattern::Level2
            | Pattern::Level3
            | Pattern::Level4
            | Pattern::Level5
            | Pattern::Level6
            | Pattern::Ampersand
            | Pattern::Bracket
            | Pattern::Bold
            | Pattern::Italic
            | Pattern::Raw
            | Pattern::Strike
            | Pattern::Escape
            | Pattern::ImageInline
            | Pattern::ImageReference
            | Pattern::LinkEmail
            | Pattern::LinkInternet
            | Pattern::LinkInline
            | Pattern::LinkReference
            | Pattern::LinkReferenceLiteral
            | Pattern::LinkReferenceShortcut => true,
        }
    }

    /// Whether an inline rule can start with `byte`.
    /// The lexer then tries only those that can.
    /// The block rules answer true, because they are not chosen by their first
    /// byte.
    pub(crate) fn opens_with(self, byte: u8) -> bool {
        match self {
            Pattern::Ampersand => byte == b'&',
            Pattern::Bracket | Pattern::LinkEmail | Pattern::LinkInternet => byte == b'<',
            Pattern::Bold | Pattern::Italic => matches!(byte, b'*' | b'_'),
            Pattern::Raw => byte == b'`',
            Pattern::Strike => byte == b'~',
            Pattern::Escape => byte == b'\\',
            Pattern::ImageInline | Pattern::ImageReference => byte == b'!',
            Pattern::LinkInline
            | Pattern::LinkReference
            | Pattern::LinkReferenceLiteral
            | Pattern::LinkReferenceShortcut => byte == b'[',
            Pattern::Setext1 => byte == b'=',
            Pattern::Setext2 => byte == b'-',
            #[cfg(feature = "lang-html-derivative")]
            Pattern::Html => matches!(byte, b'<' | b'&'),
            #[cfg(feature = "lang-html-derivative")]
            Pattern::HtmlWrap => byte == b'<',
            #[cfg(feature = "lang-html-derivative")]
            Pattern::HtmlRaw | Pattern::HtmlRawBody | Pattern::HtmlBlock => true,
            Pattern::FrontMatter
            | Pattern::FrontMatterBegin
            | Pattern::Separator
            | Pattern::Heading
            | Pattern::Quote
            | Pattern::Bullet
            | Pattern::Ordered
            | Pattern::Fence
            | Pattern::FenceLanguage
            | Pattern::FencePlain
            | Pattern::RawBlock
            | Pattern::LinkDefinition
            | Pattern::Comment
            | Pattern::Table
            | Pattern::Paragraph
            | Pattern::EmbedBegin
            | Pattern::TablePipe
            | Pattern::TableSeparator
            | Pattern::TableCell
            | Pattern::Level1
            | Pattern::Level2
            | Pattern::Level3
            | Pattern::Level4
            | Pattern::Level5
            | Pattern::Level6 => true,
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

const BLOCK: &[Pattern] = &[
    Pattern::FrontMatter,
    Pattern::Separator,
    Pattern::Heading,
    Pattern::Quote,
    Pattern::Bullet,
    Pattern::Ordered,
    Pattern::Fence,
    Pattern::RawBlock,
    Pattern::LinkDefinition,
    Pattern::Comment,
    #[cfg(feature = "lang-html-derivative")]
    Pattern::HtmlRaw,
    #[cfg(feature = "lang-html-derivative")]
    Pattern::HtmlBlock,
    Pattern::Table,
    Pattern::Paragraph,
];

const INLINE: &[Pattern] = &[
    Pattern::Ampersand,
    Pattern::Bracket,
    Pattern::Bold,
    Pattern::Italic,
    Pattern::Raw,
    Pattern::Strike,
    Pattern::Escape,
    Pattern::ImageInline,
    Pattern::ImageReference,
    Pattern::LinkEmail,
    Pattern::LinkInternet,
    Pattern::LinkInline,
    Pattern::LinkReference,
    Pattern::LinkReferenceLiteral,
    Pattern::LinkReferenceShortcut,
];

const HEADING_TEXT: &[Pattern] = &[
    Pattern::Ampersand,
    Pattern::Bracket,
    Pattern::Bold,
    Pattern::Italic,
    Pattern::Raw,
    Pattern::Strike,
    Pattern::Escape,
    Pattern::ImageInline,
    Pattern::ImageReference,
    Pattern::LinkEmail,
    Pattern::LinkInternet,
    Pattern::LinkInline,
    Pattern::LinkReference,
    Pattern::LinkReferenceLiteral,
    Pattern::LinkReferenceShortcut,
    #[cfg(feature = "lang-html-derivative")]
    Pattern::Html,
];

const PARAGRAPH: &[Pattern] = &[
    Pattern::Ampersand,
    Pattern::Bracket,
    Pattern::Bold,
    Pattern::Italic,
    Pattern::Raw,
    Pattern::Strike,
    Pattern::Escape,
    Pattern::ImageInline,
    Pattern::ImageReference,
    Pattern::LinkEmail,
    Pattern::LinkInternet,
    Pattern::LinkInline,
    Pattern::LinkReference,
    Pattern::LinkReferenceLiteral,
    Pattern::LinkReferenceShortcut,
    #[cfg(feature = "lang-html-derivative")]
    Pattern::Html,
    Pattern::Setext1,
    Pattern::Setext2,
];

const BOLD: &[Pattern] = &[
    #[cfg(feature = "lang-html-derivative")]
    Pattern::HtmlWrap,
    Pattern::Escape,
    Pattern::Ampersand,
    Pattern::Bracket,
    Pattern::Raw,
    Pattern::Bold,
    Pattern::Italic,
    Pattern::ImageInline,
    Pattern::LinkInline,
    Pattern::LinkInternet,
    Pattern::LinkEmail,
    Pattern::ImageReference,
    Pattern::LinkReferenceLiteral,
    Pattern::LinkReference,
    Pattern::LinkReferenceShortcut,
    Pattern::Strike,
];

const ITALIC: &[Pattern] = &[
    #[cfg(feature = "lang-html-derivative")]
    Pattern::HtmlWrap,
    Pattern::Escape,
    Pattern::Ampersand,
    Pattern::Bracket,
    Pattern::Raw,
    Pattern::Bold,
    Pattern::ImageInline,
    Pattern::LinkInline,
    Pattern::LinkInternet,
    Pattern::LinkEmail,
    Pattern::ImageReference,
    Pattern::LinkReferenceLiteral,
    Pattern::LinkReference,
    Pattern::LinkReferenceShortcut,
    Pattern::Strike,
];

const STRIKE_TEXT: &[Pattern] = &[
    #[cfg(feature = "lang-html-derivative")]
    Pattern::HtmlWrap,
    Pattern::Escape,
    Pattern::Ampersand,
    Pattern::Bracket,
    Pattern::Raw,
    Pattern::Bold,
    Pattern::Italic,
    Pattern::ImageInline,
    Pattern::LinkInline,
    Pattern::LinkInternet,
    Pattern::LinkEmail,
    Pattern::ImageReference,
    Pattern::LinkReferenceLiteral,
    Pattern::LinkReference,
    Pattern::LinkReferenceShortcut,
];

const LINK_TEXT: &[Pattern] = &[
    Pattern::Raw,
    Pattern::Bold,
    Pattern::Italic,
    Pattern::Strike,
    Pattern::ImageInline,
];

const HEADING: &[Pattern] = &[
    Pattern::Level6,
    Pattern::Level5,
    Pattern::Level4,
    Pattern::Level3,
    Pattern::Level2,
    Pattern::Level1,
];

const TABLE: &[Pattern] = &[
    Pattern::TablePipe,
    Pattern::TableSeparator,
    Pattern::TableCell,
];

/// The rules that apply inside a container.
pub(crate) fn patterns(container: Container) -> &'static [Pattern] {
    match container {
        Container::Fence(_) => &[Pattern::EmbedBegin],
        #[cfg(feature = "lang-html")]
        Container::FencePhp => &[Pattern::EmbedBegin],
        #[cfg(feature = "lang-html")]
        Container::BodyPhp => &[],
        Container::FrontMatter => &[Pattern::FrontMatterBegin],
        Container::Base | Container::Quote | Container::ListBullet | Container::ListOrdered => {
            BLOCK
        }
        #[cfg(feature = "lang-vue")]
        Container::Body(Language::Vue) => &[Pattern::Fence],
        Container::Body(language) => {
            if language == Language::Markdown {
                BLOCK
            } else {
                &[]
            }
        }
        Container::FencePlain
        | Container::FrontMatterBody
        | Container::RawBlock
        | Container::Comment => &[],
        #[cfg(feature = "lang-html-derivative")]
        Container::HtmlRaw => &[Pattern::HtmlRawBody],
        #[cfg(not(feature = "lang-html-derivative"))]
        Container::HtmlRaw => &[],
        Container::HtmlRawBody
        | Container::HtmlBlock
        | Container::HtmlInline
        | Container::HtmlWrap => &[],
        Container::Table => TABLE,
        Container::Paragraph => PARAGRAPH,
        Container::BoldStar | Container::BoldUnder => BOLD,
        Container::ItalicStar | Container::ItalicUnder => ITALIC,
    }
}

/// The rules that a capture group's text is handed to.
pub(crate) fn list_patterns(list: List) -> &'static [Pattern] {
    match list {
        List::Heading => HEADING,
        List::Inline => INLINE,
        List::HeadingText => HEADING_TEXT,
        List::LinkText => LINK_TEXT,
        List::StrikeText => STRIKE_TEXT,
    }
}

/// The rule that closes a container, if it has one.
pub(crate) fn end(container: Container) -> Option<End> {
    match container {
        Container::FrontMatter => Some(End::FrontMatter),
        Container::Fence(_) | Container::FencePlain => Some(End::Fence),
        #[cfg(feature = "lang-html")]
        Container::FencePhp => Some(End::Fence),
        #[cfg(feature = "lang-html")]
        Container::BodyPhp => None,
        Container::Comment => Some(End::Comment),
        Container::BoldStar | Container::BoldUnder => Some(End::Bold),
        Container::ItalicStar | Container::ItalicUnder => Some(End::Italic),
        #[cfg(feature = "lang-html-derivative")]
        Container::HtmlRaw => Some(End::HtmlRaw),
        #[cfg(not(feature = "lang-html-derivative"))]
        Container::HtmlRaw => None,
        Container::HtmlRawBody
        | Container::HtmlBlock
        | Container::HtmlInline
        | Container::HtmlWrap => None,
        Container::Base
        | Container::Quote
        | Container::ListBullet
        | Container::ListOrdered
        | Container::Body(_)
        | Container::FrontMatterBody
        | Container::RawBlock
        | Container::Table
        | Container::Paragraph => None,
    }
}

/// The scopes a container adds to everything inside it, the text that opens
/// and closes it included.
pub(crate) fn names(container: Container) -> &'static [S] {
    match container {
        Container::Base
        | Container::Body(_)
        | Container::FrontMatter
        | Container::FrontMatterBody
        | Container::HtmlRaw
        | Container::HtmlRawBody
        | Container::HtmlBlock
        | Container::HtmlInline
        | Container::HtmlWrap => &[],
        Container::Quote => &[S::Quote],
        Container::ListBullet => &[S::ListBullet],
        Container::ListOrdered => &[S::ListOrdered],
        Container::Fence(_) | Container::FencePlain => &[S::Fence],
        #[cfg(feature = "lang-html")]
        Container::FencePhp => &[S::Fence],
        #[cfg(feature = "lang-html")]
        Container::BodyPhp => &[],
        Container::RawBlock => &[S::Raw],
        Container::Table => &[S::Table],
        Container::Paragraph => &[S::Paragraph],
        Container::Comment => &[S::CommentHtml],
        Container::BoldStar | Container::BoldUnder => &[S::Bold],
        Container::ItalicStar | Container::ItalicUnder => &[S::Italic],
    }
}

/// The scopes each embedded language's fence adds to its text, sorted by
/// language.
/// Each adds one, except C++, whose `contentName` also names `source.cpp`.
/// Shiki's Markdown grammar has no rule that embeds TOML or GraphQL, so they
/// have none.
/// Neither has a language whose feature is off.
const EMBEDDED: &[(Language, &[S])] = &[
    #[cfg(feature = "lang-bat")]
    (Language::Bat, &[S::EmbeddedDosbatch]),
    #[cfg(feature = "lang-c")]
    (Language::C, &[S::EmbeddedC]),
    #[cfg(feature = "lang-coffee")]
    (Language::CoffeeScript, &[S::EmbeddedCoffee]),
    #[cfg(feature = "lang-cpp")]
    (Language::Cpp, &[S::EmbeddedCpp, S::SourceCpp]),
    #[cfg(feature = "lang-csharp")]
    (Language::CSharp, &[S::EmbeddedCsharp]),
    #[cfg(feature = "lang-css")]
    (Language::Css, &[S::EmbeddedCss]),
    #[cfg(feature = "lang-dart")]
    (Language::Dart, &[S::EmbeddedDart]),
    #[cfg(feature = "lang-diff")]
    (Language::Diff, &[S::EmbeddedDiff]),
    #[cfg(feature = "lang-dockerfile")]
    (Language::Dockerfile, &[S::EmbeddedDockerfile]),
    #[cfg(feature = "lang-elixir")]
    (Language::Elixir, &[S::EmbeddedElixir]),
    #[cfg(feature = "lang-go")]
    (Language::Go, &[S::EmbeddedGo]),
    #[cfg(feature = "lang-html")]
    (Language::Html, &[S::EmbeddedHtml]),
    #[cfg(feature = "lang-ini")]
    (Language::Ini, &[S::EmbeddedIni]),
    #[cfg(feature = "lang-java")]
    (Language::Java, &[S::EmbeddedJava]),
    #[cfg(feature = "lang-javascript")]
    (Language::JavaScript, &[S::EmbeddedJavascript]),
    #[cfg(feature = "lang-json")]
    (Language::Json, &[S::EmbeddedJson]),
    #[cfg(feature = "lang-json")]
    (Language::Jsonc, &[S::EmbeddedJsonc]),
    #[cfg(feature = "lang-json")]
    (Language::Jsonl, &[S::EmbeddedJsonl]),
    #[cfg(feature = "lang-jsx")]
    (Language::Jsx, &[S::EmbeddedJavascript]),
    #[cfg(feature = "lang-julia")]
    (Language::Julia, &[S::EmbeddedJulia]),
    #[cfg(feature = "lang-latex")]
    (Language::Latex, &[S::EmbeddedLatex]),
    #[cfg(feature = "lang-less")]
    (Language::Less, &[S::EmbeddedLess]),
    #[cfg(feature = "lang-lua")]
    (Language::Lua, &[S::EmbeddedLua]),
    #[cfg(feature = "lang-make")]
    (Language::Make, &[S::EmbeddedMake]),
    (Language::Markdown, &[S::EmbeddedMarkdown]),
    #[cfg(feature = "lang-objective-c")]
    (Language::ObjectiveC, &[S::EmbeddedObjc]),
    #[cfg(feature = "lang-perl")]
    (Language::Perl, &[S::EmbeddedPerl]),
    #[cfg(feature = "lang-php")]
    (Language::Php, &[S::EmbeddedPhp]),
    #[cfg(feature = "lang-powershell")]
    (Language::PowerShell, &[S::EmbeddedPowershell]),
    #[cfg(feature = "lang-python")]
    (Language::Python, &[S::EmbeddedPython]),
    #[cfg(feature = "lang-r")]
    (Language::R, &[S::EmbeddedR]),
    #[cfg(feature = "lang-ruby")]
    (Language::Ruby, &[S::EmbeddedRuby]),
    #[cfg(feature = "lang-rust")]
    (Language::Rust, &[S::EmbeddedRust]),
    #[cfg(feature = "lang-scala")]
    (Language::Scala, &[S::EmbeddedScala]),
    #[cfg(feature = "lang-scss")]
    (Language::Scss, &[S::EmbeddedScss]),
    #[cfg(feature = "lang-shell")]
    (Language::Shell, &[S::EmbeddedShell]),
    #[cfg(feature = "lang-sql")]
    (Language::Sql, &[S::EmbeddedSql]),
    #[cfg(feature = "lang-swift")]
    (Language::Swift, &[S::EmbeddedSwift]),
    #[cfg(feature = "lang-tsx")]
    (Language::Tsx, &[S::EmbeddedTypescriptreact]),
    #[cfg(feature = "lang-typescript")]
    (Language::TypeScript, &[S::EmbeddedTypeScript]),
    #[cfg(feature = "lang-vb")]
    (Language::Vb, &[S::EmbeddedVsNet]),
    #[cfg(feature = "lang-xml")]
    (Language::Xml, &[S::EmbeddedXml]),
    #[cfg(feature = "lang-yaml")]
    (Language::Yaml, &[S::EmbeddedYaml]),
];

/// The scopes of [`EMBEDDED`], at each language's [`Language::index`].
/// A language without a row gets none.
const BY_LANGUAGE: [&[S]; Language::ALL.len()] = {
    let mut table: [&[S]; Language::ALL.len()] = [&[]; Language::ALL.len()];
    let mut row = 0;
    while row < EMBEDDED.len() {
        let (language, scopes) = EMBEDDED[row];
        table[language.index()] = scopes;
        row += 1;
    }
    table
};

/// The scopes a container adds to the text between its opening and closing.
pub(crate) fn content_name(container: Container) -> &'static [S] {
    match container {
        Container::Body(language) => BY_LANGUAGE[language.index()],
        #[cfg(feature = "lang-html")]
        Container::BodyPhp => &[S::EmbeddedPhp],
        #[cfg(feature = "lang-html")]
        Container::FencePhp => &[],
        Container::FrontMatterBody => &[S::EmbeddedFrontMatter],
        Container::Base
        | Container::Quote
        | Container::ListBullet
        | Container::ListOrdered
        | Container::Fence(_)
        | Container::FencePlain
        | Container::FrontMatter
        | Container::RawBlock
        | Container::Table
        | Container::Paragraph
        | Container::Comment
        | Container::BoldStar
        | Container::BoldUnder
        | Container::ItalicStar
        | Container::ItalicUnder
        | Container::HtmlRaw
        | Container::HtmlRawBody
        | Container::HtmlBlock
        | Container::HtmlInline
        | Container::HtmlWrap => &[],
    }
}

/// The scope of a rule that matches text and opens no container.
pub(crate) fn match_name(rule: Pattern) -> Option<S> {
    Some(match rule {
        Pattern::Separator => S::Separator,
        Pattern::Heading => S::Heading,
        Pattern::Level1 => S::Heading1,
        Pattern::Level2 => S::Heading2,
        Pattern::Level3 => S::Heading3,
        Pattern::Level4 => S::Heading4,
        Pattern::Level5 => S::Heading5,
        Pattern::Level6 => S::Heading6,
        Pattern::LinkDefinition => S::LinkDef,
        Pattern::TablePipe => S::TablePunct,
        Pattern::Setext1 => S::Setext1,
        Pattern::Setext2 => S::Setext2,
        Pattern::Ampersand => S::Ampersand,
        Pattern::Bracket => S::Bracket,
        Pattern::Raw => S::InlineRaw,
        Pattern::Strike => S::Strike,
        Pattern::Escape => S::Escape,
        Pattern::ImageInline => S::ImageInline,
        Pattern::ImageReference => S::ImageRef,
        Pattern::LinkEmail => S::LinkEmail,
        Pattern::LinkInternet => S::LinkInet,
        Pattern::LinkInline => S::LinkInline,
        Pattern::LinkReference | Pattern::LinkReferenceShortcut => S::LinkRef,
        Pattern::LinkReferenceLiteral => S::LinkRefLiteral,
        Pattern::Quote
        | Pattern::Bullet
        | Pattern::Ordered
        | Pattern::Fence
        | Pattern::FenceLanguage
        | Pattern::FencePlain
        | Pattern::RawBlock
        | Pattern::Comment
        | Pattern::Table
        | Pattern::Paragraph
        | Pattern::EmbedBegin
        | Pattern::FrontMatter
        | Pattern::FrontMatterBegin
        | Pattern::TableSeparator
        | Pattern::TableCell
        | Pattern::Bold
        | Pattern::Italic => return None,
        #[cfg(feature = "lang-html-derivative")]
        Pattern::HtmlRaw
        | Pattern::HtmlRawBody
        | Pattern::HtmlBlock
        | Pattern::Html
        | Pattern::HtmlWrap => return None,
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
pub(crate) fn capture(rule: Pattern, group: usize) -> Capture {
    match rule {
        Pattern::Heading => only(group, 1, list(List::Heading)),
        Pattern::Level1
        | Pattern::Level2
        | Pattern::Level3
        | Pattern::Level4
        | Pattern::Level5
        | Pattern::Level6 => match group {
            1 | 3 => scope(S::HeadingPunct),
            2 => both(S::HeadingName, List::HeadingText),
            _ => NONE,
        },
        Pattern::Quote => only(group, 2, scope(S::QuotePunct)),
        Pattern::Bullet | Pattern::Ordered => only(group, 3, scope(S::ListPunct)),
        Pattern::FenceLanguage => match group {
            3 => scope(S::FencePunct),
            4 => scope(S::FenceLang),
            5 => scope(S::FenceAttrs),
            _ => NONE,
        },
        Pattern::FencePlain => match group {
            3 => scope(S::FencePunct),
            4 => scope(S::FenceInfo),
            _ => NONE,
        },
        Pattern::FrontMatterBegin => match group {
            1 => scope(S::FrontMatterBegin),
            2 => scope(S::FrontMatterComment),
            _ => NONE,
        },
        Pattern::Comment => only(group, 2, scope(S::CommentHtmlPunct)),
        Pattern::Table => only(group, 2, scope(S::TablePunct)),
        Pattern::TableSeparator => only(group, 1, scope(S::TableSeparator)),
        Pattern::TableCell => only(group, 1, list(List::Inline)),
        Pattern::LinkDefinition => match group {
            1 | 3 => scope(S::ConstPunct),
            2 => scope(S::ConstRef),
            4 => scope(S::KeyValue),
            5 | 7 => scope(S::LinkPunct),
            6 | 8 => scope(S::UnderlineLink),
            _ => title(group, 9),
        },
        Pattern::FrontMatter
        | Pattern::Separator
        | Pattern::Fence
        | Pattern::RawBlock
        | Pattern::Paragraph
        | Pattern::EmbedBegin
        | Pattern::TablePipe
        | Pattern::Setext1
        | Pattern::Setext2
        | Pattern::Ampersand
        | Pattern::Bracket
        | Pattern::Escape => NONE,
        #[cfg(feature = "lang-html-derivative")]
        Pattern::HtmlRaw
        | Pattern::HtmlRawBody
        | Pattern::HtmlBlock
        | Pattern::Html
        | Pattern::HtmlWrap => NONE,
        Pattern::Bold
        | Pattern::Italic
        | Pattern::Raw
        | Pattern::Strike
        | Pattern::ImageInline
        | Pattern::ImageReference
        | Pattern::LinkEmail
        | Pattern::LinkInternet
        | Pattern::LinkInline
        | Pattern::LinkReference
        | Pattern::LinkReferenceLiteral
        | Pattern::LinkReferenceShortcut => inline_capture(rule, group),
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
fn inline_capture(rule: Pattern, group: usize) -> Capture {
    match rule {
        Pattern::Bold => only(group, 1, scope(S::BoldPunct)),
        Pattern::Italic => only(group, 1, scope(S::ItalicPunct)),
        Pattern::Raw => match group {
            1 | 3 => scope(S::RawPunct),
            _ => NONE,
        },
        Pattern::Strike => match group {
            1 | 3 => scope(S::StrikePunct),
            2 => list(List::StrikeText),
            _ => NONE,
        },
        Pattern::ImageInline => match group {
            1 => scope(S::DescBegin),
            2 => scope(S::DescName),
            4 => scope(S::DescEnd),
            5 | 21 => scope(S::Metadata),
            7 | 9 => scope(S::LinkPunct),
            8 | 10 => scope(S::UnderlineImage),
            _ => title(group, 12),
        },
        Pattern::ImageReference => match group {
            1 => scope(S::DescBegin),
            2 => scope(S::DescName),
            4 => scope(S::DescEnd),
            5 | 7 => scope(S::ConstPunct),
            6 => scope(S::ConstRef),
            _ => NONE,
        },
        Pattern::LinkEmail => match group {
            1 => scope(S::LinkPunct),
            2 => scope(S::UnderlineLink),
            _ => NONE,
        },
        Pattern::LinkInternet => match group {
            1 | 3 => scope(S::LinkPunct),
            2 => scope(S::UnderlineLink),
            _ => NONE,
        },
        Pattern::LinkInline => match group {
            1 => scope(S::TitleBeginLink),
            2 => both(S::LinkTitle, List::LinkText),
            4 => scope(S::TitleEndLink),
            5 | 21 => scope(S::Metadata),
            7 | 9 => scope(S::LinkPunct),
            8 | 10 => scope(S::UnderlineLink),
            _ => title(group, 12),
        },
        Pattern::LinkReference => match group {
            1 => scope(S::TitleBeginLink),
            2 => both(S::LinkTitle, List::LinkText),
            4 => scope(S::TitleEndLink),
            5 => scope(S::ConstBegin),
            6 => scope(S::ConstRef),
            7 => scope(S::ConstEnd),
            _ => NONE,
        },
        Pattern::LinkReferenceLiteral => match group {
            1 => scope(S::TitleBeginLink),
            2 => scope(S::LinkTitle),
            4 => scope(S::TitleEndLink),
            5 => scope(S::ConstBegin),
            6 => scope(S::ConstEnd),
            _ => NONE,
        },
        Pattern::LinkReferenceShortcut => match group {
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
