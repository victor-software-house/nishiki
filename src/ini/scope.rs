//! The scope names the lexer emits, in Shiki's naming.

crate::scope::scope_table! {
    INI;
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
