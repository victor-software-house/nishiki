//! Which fenced code blocks embed a language, and which language.
//!
//! The grammar has a rule for each of some sixty languages, but Shiki drops a
//! rule whose language grammar is not loaded, and then the fence is an
//! ordinary one with no embedded scope. So a language is listed here only when
//! nishiki lexes it. To add one: add a variant to [`Lang`], an entry to
//! [`LANGUAGES`], and an arm to the places that match on [`Lang`] in `rules.rs`
//! and `lexer.rs`.

use super::state::Lang;
use super::text::Cx;

/// How a fence's info string names a language, as the grammar spells it: each
/// word, and each `{.word ...}` attribute form, matched without regard to
/// case.
struct Language {
    words: &'static [&'static str],
    /// The text after `{.` in the attribute form, as in `{.rust .numberLines}`.
    brace: Option<&'static str>,
    lang: Lang,
}

const LANGUAGES: &[Language] = &[
    Language {
        words: &["rust", "rs"],
        brace: Some("rust"),
        lang: Lang::Rust,
    },
    Language {
        words: &["typescript", "ts"],
        brace: None,
        lang: Lang::TypeScript,
    },
    Language {
        words: &["markdown", "md"],
        brace: None,
        lang: Lang::Markdown,
    },
];

/// A language found in an info string.
pub(crate) struct Found {
    pub(crate) lang: Lang,
    /// Where the language ends, which is where its attributes start.
    pub(crate) name_end: usize,
}

/// The language the info string at `i` names, when the attributes after it
/// are allowed: nothing, or a space or one of `,:?{` and then no backtick.
pub(crate) fn language(cx: &Cx<'_>, i: usize) -> Option<Found> {
    for lang in LANGUAGES {
        for word in lang.words {
            if starts_ignore_case(cx, i, word.as_bytes()) && attributes_ok(cx, i + word.len()) {
                return Some(Found {
                    lang: lang.lang,
                    name_end: i + word.len(),
                });
            }
        }
        if let Some(name) = lang.brace {
            let open = 2 + name.len();
            if cx.starts(i, b"{.") && starts_ignore_case(cx, i + 2, name.as_bytes()) {
                // `.+?}`: at least one character, then the first `}` that the
                // attributes can follow.
                let mut close = i + open + 1;
                while let Some(b) = cx.get(close) {
                    if b == b'\n' {
                        break;
                    }
                    if b == b'}' && attributes_ok(cx, close + 1) {
                        return Some(Found {
                            lang: lang.lang,
                            name_end: close + 1,
                        });
                    }
                    close += 1;
                }
            }
        }
    }
    None
}

fn starts_ignore_case(cx: &Cx<'_>, i: usize, word: &[u8]) -> bool {
    word.iter()
        .enumerate()
        .all(|(k, &w)| cx.get(i + k).is_some_and(|b| b.eq_ignore_ascii_case(&w)))
}

/// The attributes that may follow a language name at `i`: nothing, or a space
/// or one of `,:?{` and then anything but a backtick, to the end of the line.
fn attributes_ok(cx: &Cx<'_>, i: usize) -> bool {
    match cx.get(i) {
        None | Some(b'\n') => true,
        Some(b) if cx.space_at(i) > 0 || matches!(b, b',' | b':' | b'?' | b'{') => {
            !has_backtick(cx, i)
        }
        Some(_) => false,
    }
}

/// Whether a backtick is anywhere in the text from `i`.
pub(crate) fn has_backtick(cx: &Cx<'_>, i: usize) -> bool {
    let line_end = cx.end.min(cx.line.len());
    cx.line
        .get(i..line_end)
        .is_some_and(|rest| memchr::memchr(b'`', rest).is_some())
}
