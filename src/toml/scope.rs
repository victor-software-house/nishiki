//! The scope names the lexer emits, in Shiki's naming.

macro_rules! scopes {
    ($($id:ident $name:literal)*) => {
        /// A scope name; the discriminant indexes [`NAMES`].
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[repr(u8)]
        pub(crate) enum S { $($id),* }

        /// The name of every scope, in discriminant order.
        pub(crate) const NAMES: &[&str] = &[$($name),*];
    };
}

scopes! {
    Source "source.toml"
    MetaArray "meta.array.toml"
    MetaInline "meta.inline-table.toml"
    MetaGroup "meta.group.toml"
    MetaGroupDouble "meta.group.double.toml"
    Key "variable.other.key.toml"
    PunctKeyValue "punctuation.separator.key-value.toml"
    PunctKeySep "punctuation.separator.variable.toml"
    PunctKeyBegin "punctuation.definition.variable.begin.toml"
    PunctKeyEnd "punctuation.definition.variable.end.toml"
    SectionName "entity.name.section.toml"
    PunctSection "punctuation.definition.section.begin.toml"
    PunctArrayBegin "punctuation.definition.array.begin.toml"
    PunctArrayEnd "punctuation.definition.array.end.toml"
    PunctArraySep "punctuation.separator.array.toml"
    PunctInlineBegin "punctuation.definition.inline-table.begin.toml"
    PunctInlineEnd "punctuation.definition.inline-table.end.toml"
    PunctInlineSep "punctuation.separator.inline-table.toml"
    Integer "constant.numeric.integer.toml"
    Float "constant.numeric.float.toml"
    Hex "constant.numeric.hex.toml"
    Octal "constant.numeric.octal.toml"
    Binary "constant.numeric.binary.toml"
    Special "constant.numeric.toml"
    Date "constant.other.date.toml"
    Time "constant.other.time.toml"
    Boolean "constant.language.boolean.toml"
    StrDouble "string.quoted.double.toml"
    StrSingle "string.quoted.single.toml"
    StrTripleDouble "string.quoted.triple.double.toml"
    StrTripleSingle "string.quoted.triple.single.toml"
    PunctStrBegin "punctuation.definition.string.begin.toml"
    PunctStrEnd "punctuation.definition.string.end.toml"
    Escape "constant.character.escape.toml"
    InvalidEscape "invalid.illegal.escape.toml"
    Invalid "invalid.illegal.not-allowed-here.toml"
    Comment "comment.line.number-sign.toml"
    PunctComment "punctuation.definition.comment.toml"
    CommentLeading "punctuation.whitespace.comment.leading.toml"
}

impl S {
    /// The scope in the crate's shared numbering.
    #[expect(
        clippy::as_conversions,
        reason = "a fieldless `repr(u8)` enum has no `From` impl for its discriminant"
    )]
    pub(crate) const fn scope(self) -> crate::scope::Scope {
        crate::scope::Scope::new(crate::scope::TOML, self as u16)
    }
}
