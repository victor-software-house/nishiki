//! The scope names the lexer emits, in Shiki's naming.

macro_rules! scopes {
    ($($id:ident $name:literal)*) => {
        /// A scope name; the discriminant indexes [`NAMES`].
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        #[repr(u8)]
        pub(crate) enum S { $($id),* }

        impl S {
            /// The scope in the crate's shared numbering.
            #[expect(
                clippy::as_conversions,
                reason = "a fieldless `repr(u8)` enum has no `From` impl for its discriminant"
            )]
            pub(crate) const fn scope(self) -> crate::scope::Scope {
                crate::scope::Scope::new(crate::scope::MARKDOWN, self as u16)
            }
        }

        /// The name of every scope, indexed by discriminant.
        pub(crate) const NAMES: &[&str] = &[$($name),*];
    };
}

scopes! {
    Root "text.html.markdown"
    // Blocks.
    Paragraph "meta.paragraph.markdown"
    Heading "markup.heading.markdown"
    Heading1 "heading.1.markdown"
    Heading2 "heading.2.markdown"
    Heading3 "heading.3.markdown"
    Heading4 "heading.4.markdown"
    Heading5 "heading.5.markdown"
    Heading6 "heading.6.markdown"
    HeadingPunct "punctuation.definition.heading.markdown"
    HeadingName "entity.name.section.markdown"
    Setext1 "markup.heading.setext.1.markdown"
    Setext2 "markup.heading.setext.2.markdown"
    Separator "meta.separator.markdown"
    Quote "markup.quote.markdown"
    QuotePunct "punctuation.definition.quote.begin.markdown"
    ListBullet "markup.list.unnumbered.markdown"
    ListOrdered "markup.list.numbered.markdown"
    ListPunct "punctuation.definition.list.begin.markdown"
    Fence "markup.fenced_code.block.markdown"
    FencePunct "punctuation.definition.markdown"
    FenceLang "fenced_code.block.language.markdown"
    FenceAttrs "fenced_code.block.language.attributes.markdown"
    FenceInfo "fenced_code.block.language"
    EmbeddedRust "meta.embedded.block.rust"
    EmbeddedTypeScript "meta.embedded.block.typescript"
    EmbeddedMarkdown "meta.embedded.block.markdown"
    EmbeddedShell "meta.embedded.block.shellscript"
    Raw "markup.raw.block.markdown"
    Table "markup.table.markdown"
    TablePunct "punctuation.definition.table.markdown"
    TableSeparator "punctuation.separator.table.markdown"
    CommentHtml "comment.block.html"
    CommentHtmlPunct "punctuation.definition.comment.html"
    // Inline.
    Ampersand "meta.other.valid-ampersand.markdown"
    Bracket "meta.other.valid-bracket.markdown"
    Bold "markup.bold.markdown"
    BoldPunct "punctuation.definition.bold.markdown"
    Italic "markup.italic.markdown"
    ItalicPunct "punctuation.definition.italic.markdown"
    InlineRaw "markup.inline.raw.string.markdown"
    RawPunct "punctuation.definition.raw.markdown"
    Strike "markup.strikethrough.markdown"
    StrikePunct "punctuation.definition.strikethrough.markdown"
    Escape "constant.character.escape.markdown"
    // Links and images.
    ImageInline "meta.image.inline.markdown"
    ImageRef "meta.image.reference.markdown"
    DescBegin "punctuation.definition.link.description.begin.markdown"
    DescName "string.other.link.description.markdown"
    DescEnd "punctuation.definition.link.description.end.markdown"
    Metadata "punctuation.definition.metadata.markdown"
    LinkPunct "punctuation.definition.link.markdown"
    UnderlineImage "markup.underline.link.image.markdown"
    UnderlineLink "markup.underline.link.markdown"
    TitleName "string.other.link.description.title.markdown"
    TitleBegin "punctuation.definition.string.begin.markdown"
    TitleEnd "punctuation.definition.string.end.markdown"
    ConstPunct "punctuation.definition.constant.markdown"
    ConstBegin "punctuation.definition.constant.begin.markdown"
    ConstEnd "punctuation.definition.constant.end.markdown"
    ConstRef "constant.other.reference.link.markdown"
    LinkDef "meta.link.reference.def.markdown"
    KeyValue "punctuation.separator.key-value.markdown"
    LinkEmail "meta.link.email.lt-gt.markdown"
    LinkInet "meta.link.inet.markdown"
    LinkInline "meta.link.inline.markdown"
    LinkRef "meta.link.reference.markdown"
    LinkRefLiteral "meta.link.reference.literal.markdown"
    TitleBeginLink "punctuation.definition.link.title.begin.markdown"
    LinkTitle "string.other.link.title.markdown"
    TitleEndLink "punctuation.definition.link.title.end.markdown"
}
