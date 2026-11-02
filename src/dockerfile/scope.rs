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
                crate::scope::Scope::new(crate::scope::DOCKERFILE, self as u16)
            }
        }

        /// The name of every scope, indexed by discriminant.
        pub(crate) const NAMES: &[&str] = &[$($name),*];
    };
}

scopes! {
    Source "source.dockerfile"
    Special "keyword.other.special-method.dockerfile"
    Control "keyword.control.dockerfile"
    Double "string.quoted.double.dockerfile"
    Single "string.quoted.single.dockerfile"
    Escape "constant.character.escaped.dockerfile"
    CommentLeading "punctuation.whitespace.comment.leading.dockerfile"
    Comment "comment.line.number-sign.dockerfile"
    PunctComment "punctuation.definition.comment.dockerfile"
}
