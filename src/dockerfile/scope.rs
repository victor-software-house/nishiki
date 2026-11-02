//! The scope names the lexer emits, in Shiki's naming.

crate::scope::scope_table! {
    DOCKERFILE;
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
