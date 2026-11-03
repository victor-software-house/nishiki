//! The scope names the lexer emits, in Shiki's naming.

crate::scope::scope_table! {
    MARKDOWN;
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
    #[cfg(feature = "lang-rust")]
    EmbeddedRust "meta.embedded.block.rust"
    #[cfg(feature = "lang-typescript")]
    EmbeddedTypeScript "meta.embedded.block.typescript"
    #[cfg(any(feature = "lang-javascript", feature = "lang-jsx"))]
    EmbeddedJavascript "meta.embedded.block.javascript"
    #[cfg(feature = "lang-tsx")]
    EmbeddedTypescriptreact "meta.embedded.block.typescriptreact"
    EmbeddedMarkdown "meta.embedded.block.markdown"
    #[cfg(feature = "lang-shell")]
    EmbeddedShell "meta.embedded.block.shellscript"
    #[cfg(feature = "lang-json")]
    EmbeddedJson "meta.embedded.block.json"
    #[cfg(feature = "lang-json")]
    EmbeddedJsonc "meta.embedded.block.jsonc"
    #[cfg(feature = "lang-json")]
    EmbeddedJsonl "meta.embedded.block.jsonl"
    #[cfg(feature = "lang-python")]
    EmbeddedPython "meta.embedded.block.python"
    #[cfg(feature = "lang-yaml")]
    EmbeddedYaml "meta.embedded.block.yaml"
    #[cfg(feature = "lang-make")]
    EmbeddedMake "meta.embedded.block.makefile"
    #[cfg(feature = "lang-diff")]
    EmbeddedDiff "meta.embedded.block.diff"
    #[cfg(feature = "lang-dockerfile")]
    EmbeddedDockerfile "meta.embedded.block.dockerfile"
    #[cfg(feature = "lang-java")]
    EmbeddedJava "meta.embedded.block.java"
    #[cfg(feature = "lang-c")]
    EmbeddedC "meta.embedded.block.c"
    #[cfg(feature = "lang-sql")]
    EmbeddedSql "meta.embedded.block.sql"
    #[cfg(feature = "lang-ini")]
    EmbeddedIni "meta.embedded.block.ini"
    #[cfg(feature = "lang-css")]
    EmbeddedCss "meta.embedded.block.css"
    #[cfg(feature = "lang-go")]
    EmbeddedGo "meta.embedded.block.go"
    EmbeddedFrontMatter "meta.embedded.block.frontmatter"
    FrontMatterBegin "punctuation.definition.begin.frontmatter"
    FrontMatterComment "comment.frontmatter"
    FrontMatterEnd "punctuation.definition.end.frontmatter"
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
