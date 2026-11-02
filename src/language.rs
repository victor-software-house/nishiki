//! The languages nishiki highlights: one table of rows, from which the
//! `Language` enum and every function that has an arm per language are
//! generated.

use crate::arena::Arena;
use crate::json::Dialect;
use crate::limits::{Limits, LineReport};
use crate::scratch::Scratch;
use crate::typescript::Dialect as ScriptDialect;
use crate::{
    Token, c, css, diff, dockerfile, graphql, ini, java, json, make, markdown, python, rust, shell,
    sql, toml, typescript, yaml,
};

/// The language list, and with it every function that has one arm per
/// language: [`Language`], [`Language::ALL`], [`Language::id`], the names
/// [`Language::from_name`] reads, and the three lexer entry points.
///
/// A row is one language: its variant, its Shiki names (the first is its id),
/// the module that lexes it, and optionally the dialect the module is told,
/// and whether its rules end at the line feed of a line that the containers
/// around a Markdown fence took whole.
/// A new language is one row here, its scope table in `src/scope.rs`, its
/// frame tag in `src/tag.rs`, its fence names in `src/markdown/fence.rs`, and
/// its field in `src/scratch.rs` when it keeps buffers.
macro_rules! languages {
    (@flag) => { false };
    (@flag $flag:literal) => { $flag };
    ($(
        $(#[$meta:meta])*
        $variant:ident {
            names: [$id:literal $(, $alias:literal)*],
            module: $module:ident
            $(, dialect: $dialect:expr)?
            $(, reads_line_feed: $line_feed:literal)?
            $(,)?
        }
    )*) => {
        /// A language nishiki highlights, named as Shiki names it.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        #[non_exhaustive]
        pub enum Language {
            $($(#[$meta])* $variant,)*
        }

        impl Language {
            /// Every supported language.
            pub const ALL: &[Language] = &[$(Language::$variant),*];

            /// Shiki's id for the language, such as `"rust"`.
            #[must_use]
            pub fn id(self) -> &'static str {
                match self {
                    $(Language::$variant => $id,)*
                }
            }

            /// The id and aliases, as Shiki lists them.
            fn names(self) -> &'static [&'static str] {
                match self {
                    $(Language::$variant => &[$id $(, $alias)*],)*
                }
            }

            /// Whether the language's rules end at the line feed of a line
            /// that the containers around a Markdown fence took whole, as
            /// Shiki's grammar sees it after them. The other lexers leave
            /// their state alone for such a line.
            pub(crate) fn reads_line_feed(self) -> bool {
                match self {
                    $(Language::$variant => languages!(@flag $($line_feed)?),)*
                }
            }

            /// The state at the start of a document whose scopes sit on
            /// `base`.
            pub(crate) fn start(self, arena: &mut Arena, base: u32) -> u32 {
                match self {
                    $(Language::$variant => $module::start(arena, base $(, $dialect)?),)*
                }
            }

            /// The state at the start of a Markdown fence's body, whose text
            /// sits on `base` with no root scope of the language's own, as in
            /// Shiki's embedding.
            pub(crate) fn start_embedded(self, arena: &mut Arena, base: u32) -> u32 {
                match self {
                    $(Language::$variant => $module::start_embedded(arena, base),)*
                }
            }

            /// Lexes `line` from its `start` and from `state`, appending
            /// tokens whose ends count from the start of the whole line, and
            /// returns the state after it.
            #[inline]
            pub(crate) fn lex(self, cx: &mut Context<'_>, line: Line<'_>, state: u32) -> u32 {
                match self {
                    $(Language::$variant => $module::lex(cx, $($dialect,)? line, state),)*
                }
            }
        }
    };
}

languages! {
    /// Rust, with the scopes of Shiki's `rust` grammar.
    Rust { names: ["rust", "rs"], module: rust }
    /// Markdown, with the scopes of Shiki's `markdown` grammar. A fence of
    /// another supported language is lexed as that language.
    Markdown { names: ["markdown", "md"], module: markdown }
    /// `TypeScript`, with the scopes of Shiki's `typescript` grammar.
    TypeScript {
        names: ["typescript", "ts", "cts", "mts"],
        module: typescript,
        dialect: ScriptDialect::TypeScript,
    }
    /// `JavaScript`, with the scopes of Shiki's `javascript` grammar.
    JavaScript {
        names: ["javascript", "js", "cjs", "mjs"],
        module: typescript,
        dialect: ScriptDialect::JavaScript,
    }
    /// `JSX`, with the scopes of Shiki's `jsx` grammar.
    Jsx { names: ["jsx"], module: typescript, dialect: ScriptDialect::Jsx }
    /// `TSX`, with the scopes of Shiki's `tsx` grammar.
    Tsx { names: ["tsx"], module: typescript, dialect: ScriptDialect::Tsx }
    /// Shell, with the scopes of Shiki's `shellscript` grammar.
    Shell {
        names: ["shellscript", "bash", "sh", "shell", "zsh"],
        module: shell,
        reads_line_feed: true,
    }
    /// JSON, with the scopes of Shiki's `json` grammar.
    Json { names: ["json"], module: json, dialect: Dialect::Json }
    /// JSON with comments, with the scopes of Shiki's `jsonc` grammar.
    Jsonc { names: ["jsonc"], module: json, dialect: Dialect::Jsonc }
    /// JSON Lines, with the scopes of Shiki's `jsonl` grammar.
    Jsonl { names: ["jsonl"], module: json, dialect: Dialect::Jsonl }
    /// Python, with the scopes of Shiki's `python` grammar.
    Python { names: ["python", "py"], module: python }
    /// YAML, with the scopes of Shiki's `yaml` grammar.
    Yaml { names: ["yaml", "yml"], module: yaml }
    /// TOML, with the scopes of Shiki's `toml` grammar.
    Toml { names: ["toml"], module: toml }
    /// Makefile, with the scopes of Shiki's `make` grammar.
    Make { names: ["make", "makefile"], module: make, reads_line_feed: true }
    /// Diff, with the scopes of Shiki's `diff` grammar.
    Diff { names: ["diff"], module: diff }
    /// Dockerfile, with the scopes of Shiki's `docker` grammar.
    Dockerfile { names: ["docker", "dockerfile"], module: dockerfile }
    /// Java, with the scopes of Shiki's `java` grammar.
    Java { names: ["java"], module: java }
    /// C, with the scopes of Shiki's `c` grammar.
    C { names: ["c"], module: c, reads_line_feed: true }
    /// SQL, with the scopes of Shiki's `sql` grammar.
    Sql { names: ["sql"], module: sql }
    /// INI, with the scopes of Shiki's `ini` grammar.
    Ini { names: ["ini", "properties"], module: ini }
    /// GraphQL, with the scopes of Shiki's `graphql` grammar.
    GraphQl { names: ["graphql", "gql"], module: graphql }
    /// CSS, with the scopes of Shiki's `css` grammar.
    Css { names: ["css"], module: css }
}

impl Language {
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

impl Context<'_> {
    /// Runs `lex` on the line's text from its start, for a lexer that counts
    /// token ends from there, then moves the new tokens' ends to count from
    /// the start of the whole line, as [`Language::lex`] promises.
    #[inline]
    pub(crate) fn lex_from_start(
        &mut self,
        line: Line<'_>,
        lex: impl FnOnce(&mut Self, &str) -> u32,
    ) -> u32 {
        let first = self.out.len();
        let next = lex(self, &line.text[line.start..]);
        if line.start > 0 {
            for token in &mut self.out[first..] {
                token.end += line.start;
            }
        }
        next
    }
}
