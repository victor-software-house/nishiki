//! The languages nishiki highlights.

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
    /// Shell, with the scopes of Shiki's `shellscript` grammar.
    Shell,
    /// JSON, with the scopes of Shiki's `json` grammar.
    Json,
    /// JSON with comments, with the scopes of Shiki's `jsonc` grammar.
    Jsonc,
    /// JSON Lines, with the scopes of Shiki's `jsonl` grammar.
    Jsonl,
}

impl Language {
    /// Every supported language.
    pub const ALL: &[Language] = &[
        Language::Rust,
        Language::Markdown,
        Language::TypeScript,
        Language::Shell,
        Language::Json,
        Language::Jsonc,
        Language::Jsonl,
    ];

    /// Shiki's id for the language, such as `"rust"`.
    #[must_use]
    pub fn id(self) -> &'static str {
        match self {
            Language::Rust => "rust",
            Language::Markdown => "markdown",
            Language::TypeScript => "typescript",
            Language::Shell => "shellscript",
            Language::Json => "json",
            Language::Jsonc => "jsonc",
            Language::Jsonl => "jsonl",
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
            Language::Shell => &["shellscript", "bash", "sh", "shell", "zsh"],
            Language::Json => &["json"],
            Language::Jsonc => &["jsonc"],
            Language::Jsonl => &["jsonl"],
        }
    }
}
