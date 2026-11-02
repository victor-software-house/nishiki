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
                crate::scope::Scope::new(crate::scope::INI, self as u16)
            }
        }

        /// The name of every scope, indexed by discriminant.
        pub(crate) const NAMES: &[&str] = &[$($name),*];
    };
}

scopes! {
    Source "source.ini"
    LeadingSpace "punctuation.whitespace.comment.leading.ini"
    CommentHash "comment.line.number-sign.ini"
    CommentSemicolon "comment.line.semicolon.ini"
    PunctComment "punctuation.definition.comment.ini"
    Key "keyword.other.definition.ini"
    KeyValue "punctuation.separator.key-value.ini"
    Section "entity.name.section.group-title.ini"
    PunctEntity "punctuation.definition.entity.ini"
    Single "string.quoted.single.ini"
    Double "string.quoted.double.ini"
    PunctBegin "punctuation.definition.string.begin.ini"
    PunctEnd "punctuation.definition.string.end.ini"
    Escape "constant.character.escape.ini"
}
