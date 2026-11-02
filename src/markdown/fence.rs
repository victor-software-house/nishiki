//! Which fenced code blocks embed a language, and which language.
//!
//! The grammar has a rule for each of some sixty languages, but Shiki drops a
//! rule whose language grammar is not loaded, and then the fence is an
//! ordinary one with no embedded scope. So a language is listed here only when
//! nishiki lexes it. To add one, add its names to [`NAMES`] and its embedded
//! scope to `rules::content_name`.

use super::text::Cx;
use crate::Language;

/// How a fence's info string names a language, as the grammar spells it: each
/// word, and each `{.word ...}` attribute form, matched without regard to
/// case.
struct Name {
    words: &'static [&'static str],
    /// The text after `{.` in the attribute form, as in `{.rust .numberLines}`.
    brace: Option<&'static str>,
    lang: Language,
}

const NAMES: &[Name] = &[
    Name {
        words: &["rust", "rs"],
        brace: Some("rust"),
        lang: Language::Rust,
    },
    Name {
        words: &["typescript", "ts"],
        brace: None,
        lang: Language::TypeScript,
    },
    // The grammar sends `jsx` to the JavaScript grammar, not the JSX one.
    Name {
        words: &["javascript", "jsx", "js", "es6", "mjs", "cjs", "dataviewjs"],
        brace: Some("js"),
        lang: Language::JavaScript,
    },
    Name {
        words: &["tsx"],
        brace: None,
        lang: Language::Tsx,
    },
    Name {
        words: &["markdown", "md"],
        brace: None,
        lang: Language::Markdown,
    },
    // The grammar's `.textmate_init` has an unescaped dot, which matches any
    // character; it is matched here as a literal dot.
    Name {
        words: &[
            "shell",
            "sh",
            "bash",
            "zsh",
            "bashrc",
            "bash_profile",
            "bash_login",
            "profile",
            "bash_logout",
            ".textmate_init",
        ],
        brace: Some("bash"),
        lang: Language::Shell,
    },
    Name {
        words: &[
            "json",
            "json5",
            "sublime-settings",
            "sublime-menu",
            "sublime-keymap",
            "sublime-mousemap",
            "sublime-theme",
            "sublime-build",
            "sublime-project",
            "sublime-completions",
        ],
        brace: None,
        lang: Language::Json,
    },
    Name {
        words: &["jsonc"],
        brace: None,
        lang: Language::Jsonc,
    },
    Name {
        words: &["jsonl", "jsonlines"],
        brace: None,
        lang: Language::Jsonl,
    },
    Name {
        words: &[
            "python",
            "py",
            "py3",
            "rpy",
            "pyw",
            // The grammar's word, which the spelling check reads as a typo.
            "c\x70y",
            "SConstruct",
            "Sconstruct",
            "sconstruct",
            "SConscript",
            "gyp",
            "gypi",
        ],
        brace: Some("python"),
        lang: Language::Python,
    },
    Name {
        words: &["yaml", "yml"],
        brace: None,
        lang: Language::Yaml,
    },
    Name {
        words: &["Makefile", "makefile", "GNUmakefile", "OCamlMakefile"],
        brace: None,
        lang: Language::Make,
    },
    Name {
        words: &["patch", "diff", "rej"],
        brace: None,
        lang: Language::Diff,
    },
    Name {
        words: &["dockerfile"],
        brace: None,
        lang: Language::Dockerfile,
    },
    Name {
        words: &["java"],
        brace: None,
        lang: Language::Java,
    },
];

/// A language found in an info string.
pub(crate) struct Found {
    pub(crate) lang: Language,
    /// Where the language ends, which is where its attributes start.
    pub(crate) name_end: usize,
}

/// The language the info string at `i` names, when the attributes after it
/// are allowed: nothing, or a space or one of `,:?{` and then no backtick.
pub(crate) fn language(cx: &Cx<'_>, i: usize) -> Option<Found> {
    for lang in NAMES {
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
