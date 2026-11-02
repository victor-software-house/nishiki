//! The languages nishiki highlights, and the one place their lexers are
//! called.

use crate::arena::Arena;
use crate::json::Dialect;
use crate::limits::{Limits, LineReport};
use crate::scratch::Scratch;
use crate::typescript::Dialect as ScriptDialect;
use crate::{Token, json, make, markdown, python, rust, shell, toml, typescript, yaml};

/// A language nishiki highlights, named as Shiki names it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Language {
    /// Rust, with the scopes of Shiki's `rust` grammar.
    Rust,
    /// Markdown, with the scopes of Shiki's `markdown` grammar. A fence of
    /// another supported language is lexed as that language.
    Markdown,
    /// `TypeScript`, with the scopes of Shiki's `typescript` grammar.
    TypeScript,
    /// `JavaScript`, with the scopes of Shiki's `javascript` grammar.
    JavaScript,
    /// `JSX`, with the scopes of Shiki's `jsx` grammar.
    Jsx,
    /// `TSX`, with the scopes of Shiki's `tsx` grammar.
    Tsx,
    /// Shell, with the scopes of Shiki's `shellscript` grammar.
    Shell,
    /// JSON, with the scopes of Shiki's `json` grammar.
    Json,
    /// JSON with comments, with the scopes of Shiki's `jsonc` grammar.
    Jsonc,
    /// JSON Lines, with the scopes of Shiki's `jsonl` grammar.
    Jsonl,
    /// Python, with the scopes of Shiki's `python` grammar.
    Python,
    /// YAML, with the scopes of Shiki's `yaml` grammar.
    Yaml,
    /// TOML, with the scopes of Shiki's `toml` grammar.
    Toml,
    /// Makefile, with the scopes of Shiki's `make` grammar.
    Make,
}

impl Language {
    /// Every supported language.
    pub const ALL: &[Language] = &[
        Language::Rust,
        Language::Markdown,
        Language::TypeScript,
        Language::JavaScript,
        Language::Jsx,
        Language::Tsx,
        Language::Shell,
        Language::Json,
        Language::Jsonc,
        Language::Jsonl,
        Language::Python,
        Language::Yaml,
        Language::Toml,
        Language::Make,
    ];

    /// Shiki's id for the language, such as `"rust"`.
    #[must_use]
    pub fn id(self) -> &'static str {
        match self {
            Language::Rust => "rust",
            Language::Markdown => "markdown",
            Language::TypeScript => "typescript",
            Language::JavaScript => "javascript",
            Language::Jsx => "jsx",
            Language::Tsx => "tsx",
            Language::Shell => "shellscript",
            Language::Json => "json",
            Language::Jsonc => "jsonc",
            Language::Jsonl => "jsonl",
            Language::Python => "python",
            Language::Yaml => "yaml",
            Language::Toml => "toml",
            Language::Make => "make",
        }
    }

    /// The language Shiki knows by `name`, its id or one of its aliases,
    /// such as `"rust"` or `"rs"`; ASCII case is ignored.
    ///
    /// ```
    /// use nishiki::Language;
    ///
    /// assert_eq!(Language::from_name("RS"), Some(Language::Rust));
    /// assert_eq!(Language::from_name("MD"), Some(Language::Markdown));
    /// assert_eq!(Language::from_name("cobol"), None);
    /// ```
    #[must_use]
    pub fn from_name(name: &str) -> Option<Language> {
        Language::ALL.iter().copied().find(|language| {
            language
                .names()
                .iter()
                .any(|known| known.eq_ignore_ascii_case(name))
        })
    }

    /// The id and aliases, as Shiki lists them.
    fn names(self) -> &'static [&'static str] {
        match self {
            Language::Rust => &["rust", "rs"],
            Language::Markdown => &["markdown", "md"],
            Language::TypeScript => &["typescript", "ts", "cts", "mts"],
            Language::JavaScript => &["javascript", "js", "cjs", "mjs"],
            Language::Jsx => &["jsx"],
            Language::Tsx => &["tsx"],
            Language::Shell => &["shellscript", "bash", "sh", "shell", "zsh"],
            Language::Json => &["json"],
            Language::Jsonc => &["jsonc"],
            Language::Jsonl => &["jsonl"],
            Language::Python => &["python", "py"],
            Language::Yaml => &["yaml", "yml"],
            Language::Toml => &["toml"],
            Language::Make => &["make", "makefile"],
        }
    }
}

/// What a lexer needs besides the line: the interned stacks and states, the
/// limits, the reusable buffers, and where its tokens and limits go.
pub(crate) struct Context<'a> {
    pub(crate) scratch: &'a mut Scratch,
    pub(crate) arena: &'a mut Arena,
    pub(crate) limits: &'a Limits,
    pub(crate) out: &'a mut Vec<Token>,
    pub(crate) report: &'a mut LineReport,
}

/// The line a lexer reads: all of it, and the byte where its own text starts,
/// which is past the containers when a Markdown fence embeds it.
#[derive(Clone, Copy)]
pub(crate) struct Line<'a> {
    pub(crate) text: &'a str,
    pub(crate) start: usize,
    /// Whether a Markdown fence embeds the language, which sets the anchor
    /// `\G` at `start` as the fence's `while` rule does in Shiki.
    pub(crate) embedded: bool,
}

/// The lexers, called the same way for every language: the one place a new
/// language is wired in, besides its names above, its scope table and the
/// Markdown fence names.
impl Language {
    /// Whether the language's rules end at the line feed of a line that the
    /// containers around a Markdown fence took whole, as Shiki's grammar sees
    /// it after them. The other lexers leave their state alone for such a
    /// line.
    pub(crate) fn reads_line_feed(self) -> bool {
        self == Language::Make
    }

    /// The state at the start of a document whose scopes sit on `base`.
    pub(crate) fn start(self, arena: &mut Arena, base: u32) -> u32 {
        match self {
            Language::Rust => rust::start(arena, base),
            Language::Markdown => markdown::start(arena, base),
            Language::TypeScript => typescript::start(arena, base, ScriptDialect::TypeScript),
            Language::JavaScript => typescript::start(arena, base, ScriptDialect::JavaScript),
            Language::Jsx => typescript::start(arena, base, ScriptDialect::Jsx),
            Language::Tsx => typescript::start(arena, base, ScriptDialect::Tsx),
            Language::Shell => shell::start(arena, base),
            Language::Json => json::start(arena, base, Dialect::Json),
            Language::Jsonc => json::start(arena, base, Dialect::Jsonc),
            Language::Jsonl => json::start(arena, base, Dialect::Jsonl),
            Language::Python => python::start(arena, base),
            Language::Yaml => yaml::start(arena, base),
            Language::Toml => toml::start(arena, base),
            Language::Make => make::start(arena, base),
        }
    }

    /// The state at the start of a Markdown fence's body, whose text sits on
    /// `base` with no root scope of the language's own, as in Shiki's
    /// embedding.
    pub(crate) fn start_embedded(self, arena: &mut Arena, base: u32) -> u32 {
        match self {
            Language::Rust | Language::Markdown => rust::start_embedded(arena, base),
            Language::TypeScript | Language::JavaScript | Language::Jsx | Language::Tsx => {
                typescript::start_embedded(arena, base)
            }
            Language::Shell => shell::start_embedded(arena, base),
            Language::Json | Language::Jsonc | Language::Jsonl => json::start_embedded(arena, base),
            Language::Python => python::start_embedded(arena, base),
            Language::Yaml => yaml::start_embedded(arena, base),
            Language::Toml => toml::start_embedded(arena, base),
            Language::Make => make::start_embedded(arena, base),
        }
    }

    /// Lexes `line` from its `start` and from `state`, appending tokens whose
    /// ends count from the start of the whole line, and returns the state
    /// after it.
    #[inline]
    pub(crate) fn lex(self, cx: &mut Context<'_>, line: Line<'_>, state: u32) -> u32 {
        let first = cx.out.len();
        let rest = &line.text[line.start..];
        let next = match self {
            Language::Rust => rust::line(
                cx.arena,
                cx.limits,
                rest.as_bytes(),
                state,
                cx.out,
                cx.report,
            ),
            Language::Markdown => markdown::line(
                cx.scratch, cx.arena, cx.limits, rest, state, cx.out, cx.report,
            ),
            Language::TypeScript => ts_line(cx, ScriptDialect::TypeScript, rest, state),
            Language::JavaScript => ts_line(cx, ScriptDialect::JavaScript, rest, state),
            Language::Jsx => ts_line(cx, ScriptDialect::Jsx, rest, state),
            Language::Tsx => ts_line(cx, ScriptDialect::Tsx, rest, state),
            Language::Json => json_line(cx, Dialect::Json, rest, state),
            Language::Jsonc => json_line(cx, Dialect::Jsonc, rest, state),
            Language::Jsonl => json_line(cx, Dialect::Jsonl, rest, state),
            Language::Yaml => yaml::line(
                cx.arena,
                cx.limits,
                rest.as_bytes(),
                line.start == 0,
                state,
                cx.out,
                cx.report,
            ),
            Language::Toml => toml::line(
                cx.arena,
                cx.limits,
                rest.as_bytes(),
                state,
                cx.out,
                cx.report,
            ),
            Language::Shell => {
                let text = shell::Text {
                    bytes: line.text.as_bytes(),
                    start: line.start,
                    anchored: line.embedded,
                };
                return shell::line(
                    cx.scratch, cx.arena, cx.limits, text, state, cx.out, cx.report,
                );
            }
            Language::Make => {
                let text = make::Text {
                    bytes: line.text.as_bytes(),
                    start: line.start,
                    anchored: line.embedded,
                };
                return make::line(cx.arena, cx.limits, text, state, cx.out, cx.report);
            }
            Language::Python => {
                let text = python::Text {
                    bytes: line.text.as_bytes(),
                    start: line.start,
                };
                return python::line(
                    cx.scratch, cx.arena, cx.limits, text, state, cx.out, cx.report,
                );
            }
        };
        if line.start > 0 {
            for token in &mut cx.out[first..] {
                token.end += line.start;
            }
        }
        next
    }
}

fn json_line(cx: &mut Context<'_>, dialect: Dialect, line: &str, state: u32) -> u32 {
    json::line(
        cx.arena,
        cx.limits,
        dialect,
        line.as_bytes(),
        state,
        cx.out,
        cx.report,
    )
}

fn ts_line(cx: &mut Context<'_>, dialect: ScriptDialect, line: &str, state: u32) -> u32 {
    typescript::line(cx.arena, cx.limits, dialect, line, state, cx.out, cx.report)
}
