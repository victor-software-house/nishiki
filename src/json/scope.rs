//! The scope names the lexer emits, in Shiki's naming.
//!
//! The `json`, `jsonc` and `jsonl` grammars differ only in the suffix of
//! their names.
//! One table therefore holds the three spellings one after another.
//! A scope's number is its index plus a whole table's length for each dialect
//! before its own.

macro_rules! scopes {
    ($($id:ident $json:literal $comments:literal $lines:literal)*) => {
        /// A scope name.
        /// The discriminant indexes the table of each dialect.
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[repr(u8)]
        pub(crate) enum S { $($id),* }

        /// The number of scope names a dialect has.
        #[expect(
            clippy::as_conversions,
            clippy::cast_possible_truncation,
            reason = "`u16::try_from` is not const; the table has a few dozen names"
        )]
        const COUNT: u16 = [$($json),*].len() as u16;

        /// The name of every scope of every dialect: `json`, then `jsonc`,
        /// then `jsonl`.
        pub(crate) const NAMES: &[&str] = &[
            $($json,)*
            $($comments,)*
            $($lines,)*
        ];
    };
}

scopes! {
    Source "source.json" "source.json.comments" "source.json.lines"
    MetaArray "meta.structure.array.json" "meta.structure.array.json.comments" "meta.structure.array.json.lines"
    MetaDict "meta.structure.dictionary.json" "meta.structure.dictionary.json.comments" "meta.structure.dictionary.json.lines"
    MetaPair "meta.structure.dictionary.value.json" "meta.structure.dictionary.value.json.comments" "meta.structure.dictionary.value.json.lines"
    PunctArrayBegin "punctuation.definition.array.begin.json" "punctuation.definition.array.begin.json.comments" "punctuation.definition.array.begin.json.lines"
    PunctArrayEnd "punctuation.definition.array.end.json" "punctuation.definition.array.end.json.comments" "punctuation.definition.array.end.json.lines"
    PunctDictBegin "punctuation.definition.dictionary.begin.json" "punctuation.definition.dictionary.begin.json.comments" "punctuation.definition.dictionary.begin.json.lines"
    PunctDictEnd "punctuation.definition.dictionary.end.json" "punctuation.definition.dictionary.end.json.comments" "punctuation.definition.dictionary.end.json.lines"
    PunctKeyValue "punctuation.separator.dictionary.key-value.json" "punctuation.separator.dictionary.key-value.json.comments" "punctuation.separator.dictionary.key-value.json.lines"
    PunctPair "punctuation.separator.dictionary.pair.json" "punctuation.separator.dictionary.pair.json.comments" "punctuation.separator.dictionary.pair.json.lines"
    PunctArraySep "punctuation.separator.array.json" "punctuation.separator.array.json.comments" "punctuation.separator.array.json.lines"
    InvalidArraySep "invalid.illegal.expected-array-separator.json" "invalid.illegal.expected-array-separator.json.comments" "invalid.illegal.expected-array-separator.json.lines"
    InvalidDictSep "invalid.illegal.expected-dictionary-separator.json" "invalid.illegal.expected-dictionary-separator.json.comments" "invalid.illegal.expected-dictionary-separator.json.lines"
    Constant "constant.language.json" "constant.language.json.comments" "constant.language.json.lines"
    Number "constant.numeric.json" "constant.numeric.json.comments" "constant.numeric.json.lines"
    StrQuoted "string.quoted.double.json" "string.quoted.double.json.comments" "string.quoted.double.json.lines"
    PunctStrBegin "punctuation.definition.string.begin.json" "punctuation.definition.string.begin.json.comments" "punctuation.definition.string.begin.json.lines"
    PunctStrEnd "punctuation.definition.string.end.json" "punctuation.definition.string.end.json.comments" "punctuation.definition.string.end.json.lines"
    KeyString "string.json" "string.json.comments" "string.json.lines"
    KeyName "support.type.property-name.json" "support.type.property-name.json.comments" "support.type.property-name.json.lines"
    PunctKeyBegin "punctuation.support.type.property-name.begin.json" "punctuation.support.type.property-name.begin.json.comments" "punctuation.support.type.property-name.begin.json.lines"
    PunctKeyEnd "punctuation.support.type.property-name.end.json" "punctuation.support.type.property-name.end.json.comments" "punctuation.support.type.property-name.end.json.lines"
    Escape "constant.character.escape.json" "constant.character.escape.json.comments" "constant.character.escape.json.lines"
    InvalidEscape "invalid.illegal.unrecognized-string-escape.json" "invalid.illegal.unrecognized-string-escape.json.comments" "invalid.illegal.unrecognized-string-escape.json.lines"
    CommentDoc "comment.block.documentation.json" "comment.block.documentation.json.comments" "comment.block.documentation.json.lines"
    CommentBlock "comment.block.json" "comment.block.json.comments" "comment.block.json.lines"
    PunctComment "punctuation.definition.comment.json" "punctuation.definition.comment.json.comments" "punctuation.definition.comment.json.lines"
    CommentLine "comment.line.double-slash.js" "comment.line.double-slash.js" "comment.line.double-slash.js"
}

impl S {
    /// The scope in the crate's shared numbering, spelled for `dialect`.
    #[expect(
        clippy::as_conversions,
        reason = "a fieldless `repr(u8)` enum has no `From` impl for its discriminant"
    )]
    pub(crate) const fn scope(self, dialect: super::Dialect) -> crate::scope::Scope {
        crate::scope::Scope::new(crate::scope::JSON, dialect.index() * COUNT + self as u16)
    }
}
