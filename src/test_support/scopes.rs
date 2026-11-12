//! Reading one token's scopes from a lexed text.

use super::rows::lex;

/// The scopes of the first token whose text is `token` when `text` is lexed as
/// `language`, outermost first.
pub(crate) fn scopes_of(language: crate::Language, text: &str, token: &str) -> Vec<String> {
    lex(language, text)
        .into_iter()
        .flatten()
        .find(|(t, _)| t == token)
        .unwrap_or_else(|| panic!("no token {token:?} in {text:?}"))
        .1
}

/// The innermost scope of the first token whose text is `token` when `text`
/// is lexed as `language`.
#[cfg(any(feature = "lang-astro", feature = "lang-html", feature = "lang-java"))]
pub(crate) fn leaf(language: crate::Language, text: &str, token: &str) -> String {
    scopes_of(language, text, token).pop().unwrap_or_default()
}
