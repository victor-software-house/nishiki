//! The languages nishiki lexes by hand that a linked grammar can hand a
//! region to, by the root scope of Shiki's grammar for each.
//!
//! The list follows `oracle/hand-lexed.ts`, which the oracle's hand-off
//! tables read.
//! A language is listed only when its feature is on.

use crate::language::Language;

/// Each hand-lexed language a grammar can hand off to, with the root scope
/// of Shiki's grammar for it.
static HAND_LEXED: &[(&str, Language)] = &[
    #[cfg(feature = "lang-coffee")]
    ("source.coffee", Language::CoffeeScript),
    #[cfg(feature = "lang-css")]
    ("source.css", Language::Css),
    #[cfg(feature = "lang-html")]
    ("text.html.basic", Language::Html),
    #[cfg(feature = "lang-javascript")]
    ("source.js", Language::JavaScript),
    #[cfg(feature = "lang-json")]
    ("source.json", Language::Json),
    #[cfg(feature = "lang-markdown")]
    ("text.html.markdown", Language::Markdown),
    #[cfg(feature = "lang-ruby")]
    ("source.ruby", Language::Ruby),
    #[cfg(feature = "lang-sass")]
    ("source.sass", Language::Sass),
    #[cfg(feature = "lang-sql")]
    ("source.sql", Language::Sql),
    #[cfg(feature = "lang-xml")]
    ("text.xml", Language::Xml),
];

/// Every hand-lexed language a grammar can hand off to, with its scope.
pub(super) fn all() -> &'static [(&'static str, Language)] {
    HAND_LEXED
}

/// The root scope of Shiki's grammar for `language`, when a grammar can hand
/// a region to it.
pub(super) fn scope(language: Language) -> Option<&'static str> {
    HAND_LEXED
        .iter()
        .find(|&&(_, listed)| listed == language)
        .map(|&(scope, _)| scope)
}
