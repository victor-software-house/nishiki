//! The scope names the lexer emits, in Shiki's naming.

macro_rules! scopes {
    ($($id:ident $name:literal)*) => {
        /// A scope name; the discriminant indexes [`NAMES`].
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[repr(u8)]
        pub(crate) enum S { $($id),* }

        impl S {
            /// The scope in the crate's shared numbering.
            #[expect(
                clippy::as_conversions,
                reason = "a fieldless `repr(u8)` enum has no `From` impl for its discriminant"
            )]
            pub(crate) const fn scope(self) -> crate::scope::Scope {
                crate::scope::Scope::new(crate::scope::DIFF, self as u16)
            }
        }

        /// The name of every scope, indexed by discriminant.
        pub(crate) const NAMES: &[&str] = &[$($name),*];
    };
}

scopes! {
    Source "source.diff"
    MetaSeparator "meta.separator.diff"
    PunctSeparator "punctuation.definition.separator.diff"
    MetaRangeNormal "meta.diff.range.normal"
    MetaRangeUnified "meta.diff.range.unified"
    PunctRange "punctuation.definition.range.diff"
    LineNumber "meta.toc-list.line-number.diff"
    MetaRangeContext "meta.diff.range.context"
    MetaHeaderGit "meta.diff.header.git"
    MetaHeaderCommand "meta.diff.header.command"
    MetaFromFile "meta.diff.header.from-file"
    PunctFromFile "punctuation.definition.from-file.diff"
    MetaToFile "meta.diff.header.to-file"
    PunctToFile "punctuation.definition.to-file.diff"
    Inserted "markup.inserted.diff"
    PunctInserted "punctuation.definition.inserted.diff"
    Changed "markup.changed.diff"
    PunctChanged "punctuation.definition.changed.diff"
    Deleted "markup.deleted.diff"
    PunctDeleted "punctuation.definition.deleted.diff"
    Comment "comment.line.number-sign.diff"
    PunctComment "punctuation.definition.comment.diff"
    MetaIndexGit "meta.diff.index.git"
    MetaIndex "meta.diff.index"
    PunctKeyValue "punctuation.separator.key-value.diff"
    FileName "meta.toc-list.file-name.diff"
    MetaOnlyIn "meta.diff.only-in"
}
