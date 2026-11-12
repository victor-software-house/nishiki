//! The scope names the lexer emits, in Shiki's naming.
//! The grammar builds a tag's, an attribute's and an entity's names from the
//! text it matched.
//! Those names are made at run time.
//! The names here are the fixed ones.

crate::scope::scope_table! {
    HTML;
    TextHtmlBasic "text.html.basic"
    TagBegin "punctuation.definition.tag.begin.html"
    TagEnd "punctuation.definition.tag.end.html"
    TagPunctuation "punctuation.definition.tag.html"
    NameTag "entity.name.tag.html"
    UnrecognizedTag "invalid.illegal.unrecognized-tag.html"
    Deprecated "invalid.deprecated.html"
    NoLongerSupported "invalid.illegal.no-longer-supported.html"
    AttributeName "entity.other.attribute-name.html"
    AttributeNameDeprecated "invalid.deprecated.entity.other.attribute-name.html"
    CharacterNotAllowed "invalid.illegal.character-not-allowed-here.html"
    KeyValue "punctuation.separator.key-value.html"
    Unquoted "string.unquoted.html"
    StringDouble "string.quoted.double.html"
    StringSingle "string.quoted.single.html"
    StringBegin "punctuation.definition.string.begin.html"
    StringEnd "punctuation.definition.string.end.html"
    UnexpectedEquals "invalid.illegal.unexpected-equals-sign.html"
    CommentBlock "comment.block.html"
    CommentPunctuation "punctuation.definition.comment.html"
    CommentIllegal "invalid.illegal.characters-not-allowed-here.html"
    TagCdata "meta.tag.metadata.cdata.html"
    InlineData "string.other.inline-data.html"
    TagDoctype "meta.tag.metadata.doctype.html"
    TagProcessing "meta.tag.metadata.processing.xml.html"
    Embedded "meta.embedded.block.html"
    TagStyleStart "meta.tag.metadata.style.start.html"
    TagStyleEnd "meta.tag.metadata.style.end.html"
    TagScriptStart "meta.tag.metadata.script.start.html"
    TagScriptEnd "meta.tag.metadata.script.end.html"
    CssIgnored "source.css-ignored-vscode"
    JsIgnored "source.js-ignored-vscode"
    SourceCss "source.css"
    SourceJs "source.js"
    SourceUnknown "source.unknown"
    WhitespaceLeading "punctuation.whitespace.embedded.leading.html"
    WhitespaceTrailing "punctuation.whitespace.embedded.trailing.html"
    CommentLineJs "comment.line.double-slash.js"
    CommentBlockJs "comment.block.js"
    CommentPunctuationJs "punctuation.definition.comment.js"
    CommentBeginJs "punctuation.definition.comment.begin.js"
    CommentEndJs "punctuation.definition.comment.end.js"
    EmbeddedLineCss "meta.embedded.line.css"
    EmbeddedLineJs "meta.embedded.line.js"
    AttributeStyle "meta.attribute.style.html"
    EntityPunctuation "punctuation.definition.entity.html"
    EntityDecimal "constant.character.entity.numeric.decimal.html"
    EntityHexadecimal "constant.character.entity.numeric.hexadecimal.html"
    AmbiguousAmpersand "invalid.illegal.ambiguous-ampersand.html"
    BadAngleBracket "invalid.illegal.bad-angle-bracket.html"
    TagCustomStart "meta.tag.custom.start.html"
    TagCustomEnd "meta.tag.custom.end.html"
    ElementOtherInvalid "meta.element.other.invalid.html"
    TagOtherInvalidStart "meta.tag.other.invalid.start.html"
    TagOtherInvalidEnd "meta.tag.other.invalid.end.html"
    TagOtherInvalidVoid "meta.tag.other.invalid.void.html"
    TagDerivative "meta.tag.other.unrecognized.html.derivative"
    TagStartMetadata "meta.tag.metadata.start.html"
    TagStartStructure "meta.tag.structure.start.html"
    TagStartInline "meta.tag.inline.start.html"
    TagStartObject "meta.tag.object.start.html"
    TagStartOther "meta.tag.other.start.html"
    #[cfg(feature = "lang-erb")]
    TextHtmlErb "text.html.erb"
    #[cfg(feature = "lang-html-derivative")]
    TextHtmlDerivative "text.html.derivative"
    ErbBlock "meta.embedded.block.erb"
    ErbLine "meta.embedded.line.erb"
    SourceRuby "source.ruby"
    ErbBegin "punctuation.section.embedded.begin.erb"
    ErbEnd "punctuation.section.embedded.end.erb"
    ErbComment "comment.block.erb"
    ErbCommentBegin "punctuation.definition.comment.begin.erb"
    ErbCommentEnd "punctuation.definition.comment.end.erb"
    ErbHashComment "comment.line.number-sign.erb"
    ErbHashPunctuation "punctuation.definition.comment.erb"
    ErbCommentLeading "punctuation.whitespace.comment.leading.erb"
    ErbCommentTrailing "punctuation.whitespace.comment.trailing.erb"
    ErbEmbeddedLeading "punctuation.whitespace.embedded.leading.erb"
    ErbEmbeddedTrailing "punctuation.whitespace.embedded.trailing.erb"
    #[cfg(feature = "lang-blade")]
    TextHtmlPhpBlade "text.html.php.blade"
    SourcePhp "source.php"
    BladeComment "comment.block.blade"
    BladeCommentBegin "punctuation.definition.comment.begin.blade"
    BladeCommentEnd "punctuation.definition.comment.end.blade"
    BladePhpInComment "invalid.illegal.php-code-in-comment.blade"
    BladePhpBlockInComment "invalid.illegal.php-code-in-comment.blade.meta.embedded.block.php"
    BladePhpLineInComment "invalid.illegal.php-code-in-comment.blade.meta.embedded.line.php"
    BladeEcho "meta.function.echo.blade"
    BladeConstructBegin "support.function.construct.begin.blade"
    BladeConstructEnd "support.function.construct.end.blade"
    BladeRoundBegin "begin.bracket.round.blade"
    BladeRoundEnd "end.bracket.round.blade"
    BladeIndex "variable.other.index.php"
    BladeDirective "meta.directive.blade"
    BladeDirectiveCustom "meta.directive.custom.blade"
    BladeKeyword "keyword.blade"
    BladeFunction "entity.name.function.blade"
    BladeArgsBegin "begin.bracket.round.blade.php"
    BladeArgsEnd "end.bracket.round.blade.php"
    BladeCommentArgs "comment.blade"
    BladeEmbeddedBlock "meta.embedded.block.blade"
    PhpBlock "meta.embedded.block.php"
    PhpLine "meta.embedded.line.php"
    PhpBegin "punctuation.section.embedded.begin.php"
    PhpEnd "punctuation.section.embedded.end.php"
    PhpLeading "punctuation.whitespace.embedded.leading.php"
    PhpTrailing "punctuation.whitespace.embedded.trailing.php"
    PhpEmptyTag "meta.special.empty-tag.php"
    #[cfg(feature = "lang-vue")]
    TextHtmlVue "text.html.vue"
    #[cfg(feature = "lang-vue")]
    VueTagBegin "punctuation.definition.tag.begin.html.vue"
    #[cfg(feature = "lang-vue")]
    VueTagEnd "punctuation.definition.tag.end.html.vue"
    #[cfg(feature = "lang-vue")]
    VueComment "comment.block.vue"
    #[cfg(feature = "lang-vue")]
    VueCommentPunctuation "punctuation.definition.comment.vue"
    #[cfg(feature = "lang-vue")]
    VueBlockTag "punctuation.definition.block.tag.comment.vue"
    #[cfg(feature = "lang-vue")]
    VueStorageClass "storage.type.class.comment.vue"
    #[cfg(feature = "lang-vue")]
    SelfClosingTag "self-closing-tag"
    #[cfg(feature = "lang-vue")]
    TagStuff "meta.tag-stuff"
    #[cfg(feature = "lang-vue")]
    VueNameTag "entity.name.tag.html.vue"
    #[cfg(feature = "lang-vue")]
    TemplateTagStart "meta.template-tag.start"
    #[cfg(feature = "lang-vue")]
    TemplateTagEnd "meta.template-tag.end"
    #[cfg(feature = "lang-vue")]
    DirectiveVue "meta.attribute.directive.vue"
    #[cfg(feature = "lang-vue")]
    DirectiveControl "meta.attribute.directive.control.vue"
    #[cfg(feature = "lang-vue")]
    LoopVue "keyword.control.loop.vue"
    #[cfg(feature = "lang-vue")]
    ConditionalVue "keyword.control.conditional.vue"
    #[cfg(feature = "lang-vue")]
    VueAttributeName "entity.other.attribute-name.html.vue"
    #[cfg(feature = "lang-vue")]
    VueKeyValue "punctuation.separator.key-value.html.vue"
    #[cfg(feature = "lang-vue")]
    ShorthandBind "punctuation.attribute-shorthand.bind.html.vue"
    #[cfg(feature = "lang-vue")]
    ShorthandEvent "punctuation.attribute-shorthand.event.html.vue"
    #[cfg(feature = "lang-vue")]
    ShorthandSlot "punctuation.attribute-shorthand.slot.html.vue"
    #[cfg(feature = "lang-vue")]
    VueStringBegin "punctuation.definition.string.begin.html.vue"
    #[cfg(feature = "lang-vue")]
    VueStringEnd "punctuation.definition.string.end.html.vue"
    #[cfg(feature = "lang-vue")]
    TsEmbedded "source.ts.embedded.html.vue"
    #[cfg(feature = "lang-vue")]
    GenericAttribute "meta.attribute.generic.vue"
    #[cfg(feature = "lang-vue")]
    StyleAttribute "meta.attribute.style.vue"
    #[cfg(feature = "lang-vue")]
    TypeParameters "meta.type.parameters.vue"
    #[cfg(feature = "lang-vue")]
    CssEmbedded "source.css.embedded.html.vue"
    #[cfg(feature = "lang-vue")]
    Text "text"
    #[cfg(feature = "lang-vue")]
    TextHtmlMarkdown "text.html.markdown"
    #[cfg(feature = "lang-vue")]
    TextPug "text.pug"
    #[cfg(feature = "lang-vue")]
    SourceStylus "source.stylus"
    #[cfg(feature = "lang-vue")]
    SourcePostcss "source.postcss"
    #[cfg(feature = "lang-vue")]
    SourceSass "source.sass"
    #[cfg(feature = "lang-vue")]
    SourceScss "source.css.scss"
    #[cfg(feature = "lang-vue")]
    SourceLess "source.css.less"
    #[cfg(feature = "lang-vue")]
    SourceTs "source.ts"
    #[cfg(feature = "lang-vue")]
    SourceJsx "source.js.jsx"
    #[cfg(feature = "lang-vue")]
    SourceTsx "source.tsx"
    #[cfg(feature = "lang-vue")]
    SourceCoffee "source.coffee"
    #[cfg(feature = "lang-vue")]
    SourceJson "source.json"
    #[cfg(feature = "lang-vue")]
    SourceJsonComments "source.json.comments"
    #[cfg(feature = "lang-vue")]
    SourceJson5 "source.json5"
    #[cfg(feature = "lang-vue")]
    SourceYaml "source.yaml"
    #[cfg(feature = "lang-vue")]
    SourceToml "source.toml"
    #[cfg(feature = "lang-vue")]
    SourceGraphql "source.graphql"
}
