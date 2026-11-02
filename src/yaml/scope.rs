//! The scope names the lexer emits, in Shiki's naming.

macro_rules! scopes {
    ($($id:ident $name:literal)*) => {
        /// A scope name; the discriminant indexes [`NAMES`].
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[repr(u8)]
        pub(crate) enum S { $($id),* }

        /// The name of every scope, in the order of [`S`].
        pub(crate) const NAMES: &[&str] = &[$($name,)*];
    };
}

scopes! {
    Source "source.yaml"
    PlainOut "string.unquoted.plain.out.yaml"
    PlainIn "string.unquoted.plain.in.yaml"
    KeyName "entity.name.tag.yaml"
    PunctKeyValue "punctuation.separator.key-value.mapping.yaml"
    PunctSeqItem "punctuation.definition.block.sequence.item.yaml"
    StrBlock "string.unquoted.block.yaml"
    StrDouble "string.quoted.double.yaml"
    StrSingle "string.quoted.single.yaml"
    PunctStrBegin "punctuation.definition.string.begin.yaml"
    PunctStrEnd "punctuation.definition.string.end.yaml"
    Escape "constant.character.escape.yaml"
    EscapeSingle "constant.character.escape.single-quoted.yaml"
    EscapeNewline "constant.character.escape.double-quoted.newline.yaml"
    Comment "comment.line.number-sign.yaml"
    PunctComment "punctuation.definition.comment.yaml"
    CommentLeading "punctuation.whitespace.comment.leading.yaml"
    Boolean "constant.language.boolean.yaml"
    Null "constant.language.null.yaml"
    Merge "constant.language.merge.yaml"
    Value "constant.language.value.yaml"
    Integer "constant.numeric.integer.yaml"
    Float "constant.numeric.float.yaml"
    Timestamp "constant.other.timestamp.yaml"
    MetaFlowSeq "meta.flow-sequence.yaml"
    MetaFlowMap "meta.flow-mapping.yaml"
    MetaFlowPair "meta.flow-pair.yaml"
    MetaFlowPairKey "meta.flow-pair.key.yaml"
    MetaFlowPairValue "meta.flow-pair.value.yaml"
    MetaFlowPairExplicit "meta.flow-pair.explicit.yaml"
    PunctKeyValueBegin "punctuation.definition.key-value.begin.yaml"
    PunctSeqBegin "punctuation.definition.sequence.begin.yaml"
    PunctSeqEnd "punctuation.definition.sequence.end.yaml"
    PunctSeqSep "punctuation.separator.sequence.yaml"
    PunctMapBegin "punctuation.definition.mapping.begin.yaml"
    PunctMapEnd "punctuation.definition.mapping.end.yaml"
    PunctMapSep "punctuation.separator.mapping.yaml"
    BlockLiteral "keyword.control.flow.block-scalar.literal.yaml"
    BlockFolded "keyword.control.flow.block-scalar.folded.yaml"
    Chomping "storage.modifier.chomping-indicator.yaml"
    IndentIndicator "constant.numeric.indentation-indicator.yaml"
    MetaProperty "meta.property.yaml"
    TagHandle "storage.type.tag-handle.yaml"
    AnchorKeyword "keyword.control.property.anchor.yaml"
    PunctAnchor "punctuation.definition.anchor.yaml"
    AnchorName "entity.name.type.anchor.yaml"
    AliasKeyword "keyword.control.flow.alias.yaml"
    PunctAlias "punctuation.definition.alias.yaml"
    AliasName "variable.other.alias.yaml"
    DocBegin "entity.other.document.begin.yaml"
    DocEnd "entity.other.document.end.yaml"
    MetaDirective "meta.directive.yaml"
    PunctDirective "punctuation.definition.directive.begin.yaml"
    DirectiveYaml "keyword.other.directive.yaml.yaml"
    DirectiveTag "keyword.other.directive.tag.yaml"
    DirectiveReserved "support.other.directive.reserved.yaml"
    DirectiveName "string.unquoted.directive-name.yaml"
    DirectiveParam "string.unquoted.directive-parameter.yaml"
    YamlVersion "constant.numeric.yaml-version.yaml"
    TagPrefix "support.type.tag-prefix.yaml"
    InvalidTagHandle "invalid.illegal.tag-handle.yaml"
    InvalidAnchor "invalid.illegal.character.anchor.yaml"
    InvalidUnrecognized "invalid.illegal.unrecognized.yaml"
    MetaBlockMap "meta.block-mapping.yaml"
    InvalidNewline "invalid.illegal.expected-newline.yaml"
    InvalidCommentNewline "invalid.illegal.expected-comment-or-newline.yaml"
}

impl S {
    /// The scope in the crate's shared numbering.
    #[expect(
        clippy::as_conversions,
        reason = "a fieldless `repr(u8)` enum has no `From` impl for its discriminant"
    )]
    pub(crate) const fn scope(self) -> crate::scope::Scope {
        crate::scope::Scope::new(crate::scope::YAML, self as u16)
    }
}
