//! The languages nishiki highlights: one table of rows, from which the
//! `Language` enum and every function that has an arm per language are
//! generated.

use crate::Token;
use crate::arena::Arena;
use crate::limits::{Limits, LineReport};
use crate::scratch::Scratch;

/// The language list, and with it every function that has one arm per
/// language: [`Language`], [`Language::ALL`], [`Language::id`], the names
/// [`Language::from_name`] reads, and the three lexer entry points.
///
/// A row is one language: its variant, its Shiki names (the first is its id),
/// the module that lexes it, and optionally the dialect the module is told,
/// and whether its rules end at the line feed of a line that the containers
/// around a Markdown fence took whole.
/// A new language is one row here, its scope table in `src/scope.rs`, its
/// frame tag in `src/tag.rs`, its fence names in `src/markdown/fence.rs`, its
/// field in `src/scratch.rs` when it keeps buffers, and its `lang-*` feature
/// in `Cargo.toml`, with a `cfg` on each of these.
macro_rules! languages {
    (@flag) => { false };
    (@flag $flag:literal) => { $flag };
    ($(
        $(#[$meta:meta])*
        $variant:ident {
            feature: $feature:literal,
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
            $(
                $(#[$meta])*
                #[cfg(feature = $feature)]
                #[cfg_attr(docsrs, doc(cfg(feature = $feature)))]
                $variant,
            )*
        }

        impl Language {
            /// Every supported language.
            pub const ALL: &[Language] = &[$(#[cfg(feature = $feature)] Language::$variant),*];

            /// The language's position in [`Language::ALL`]: its discriminant,
            /// since the enum and `ALL` list the rows in one order, which the
            /// assertion below checks. Markdown numbers its fence kinds by it.
            #[cfg(feature = "lang-markdown")]
            #[expect(
                clippy::as_conversions,
                reason = "a fieldless enum's discriminant is its position in `ALL`"
            )]
            pub(crate) const fn index(self) -> usize {
                self as usize
            }

            /// Shiki's id for the language, such as `"rust"`.
            #[must_use]
            pub fn id(self) -> &'static str {
                match self {
                    $(#[cfg(feature = $feature)] Language::$variant => $id,)*
                }
            }

            /// The id and aliases, as Shiki lists them.
            fn names(self) -> &'static [&'static str] {
                match self {
                    $(#[cfg(feature = $feature)] Language::$variant => &[$id $(, $alias)*],)*
                }
            }

            /// Whether the language's rules end at the line feed of a line
            /// that the containers around a Markdown fence took whole, as
            /// Shiki's grammar sees it after them. The other lexers leave
            /// their state alone for such a line.
            pub(crate) fn reads_line_feed(self) -> bool {
                match self {
                    $(#[cfg(feature = $feature)] Language::$variant => languages!(@flag $($line_feed)?),)*
                }
            }

            /// The state at the start of a document whose scopes sit on
            /// `base`.
            pub(crate) fn start(self, arena: &mut Arena, base: u32) -> u32 {
                match self {
                    $(#[cfg(feature = $feature)] Language::$variant => crate::$module::start(arena, base $(, $dialect)?),)*
                }
            }

            /// The state at the start of a Markdown fence's body, whose text
            /// sits on `base` with no root scope of the language's own, as in
            /// Shiki's embedding.
            pub(crate) fn start_embedded(self, arena: &mut Arena, base: u32) -> u32 {
                match self {
                    $(#[cfg(feature = $feature)] Language::$variant => crate::$module::start_embedded(arena, base),)*
                }
            }

            /// Lexes `line` from its `start` and from `state`, appending
            /// tokens whose ends count from the start of the whole line, and
            /// returns the state after it.
            #[inline]
            pub(crate) fn lex(self, cx: &mut Context<'_>, line: Line<'_>, state: u32) -> u32 {
                match self {
                    $(#[cfg(feature = $feature)] Language::$variant => crate::$module::lex(cx, $($dialect,)? line, state),)*
                }
            }
        }
    };
}

/// Every language's [`Language::index`] is its position in [`Language::ALL`].
#[cfg(feature = "lang-markdown")]
const _: () = {
    let mut i = 0;
    while i < Language::ALL.len() {
        assert!(
            Language::ALL[i].index() == i,
            "Language::ALL is out of order"
        );
        i += 1;
    }
};

languages! {
    /// Rust, with the scopes of Shiki's `rust` grammar.
    Rust { feature: "lang-rust", names: ["rust", "rs"], module: rust }
    /// Markdown, with the scopes of Shiki's `markdown` grammar. A fence of
    /// another supported language is lexed as that language.
    Markdown { feature: "lang-markdown", names: ["markdown", "md"], module: markdown }
    /// `TypeScript`, with the scopes of Shiki's `typescript` grammar.
    TypeScript {
        feature: "lang-typescript",
        names: ["typescript", "ts", "cts", "mts"],
        module: typescript,
        dialect: crate::typescript::Dialect::TypeScript,
    }
    /// `JavaScript`, with the scopes of Shiki's `javascript` grammar.
    JavaScript {
        feature: "lang-javascript",
        names: ["javascript", "js", "cjs", "mjs"],
        module: typescript,
        dialect: crate::typescript::Dialect::JavaScript,
    }
    /// `JSX`, with the scopes of Shiki's `jsx` grammar.
    Jsx {
        feature: "lang-jsx",
        names: ["jsx"],
        module: typescript,
        dialect: crate::typescript::Dialect::Jsx,
    }
    /// `TSX`, with the scopes of Shiki's `tsx` grammar.
    Tsx {
        feature: "lang-tsx",
        names: ["tsx"],
        module: typescript,
        dialect: crate::typescript::Dialect::Tsx,
    }
    /// Shell, with the scopes of Shiki's `shellscript` grammar.
    Shell {
        feature: "lang-shell",
        names: ["shellscript", "bash", "sh", "shell", "zsh"],
        module: shell,
        reads_line_feed: true,
    }
    /// JSON, with the scopes of Shiki's `json` grammar.
    Json {
        feature: "lang-json",
        names: ["json"],
        module: json,
        dialect: crate::json::Dialect::Json,
    }
    /// JSON with comments, with the scopes of Shiki's `jsonc` grammar.
    Jsonc {
        feature: "lang-json",
        names: ["jsonc"],
        module: json,
        dialect: crate::json::Dialect::Jsonc,
    }
    /// JSON Lines, with the scopes of Shiki's `jsonl` grammar.
    Jsonl {
        feature: "lang-json",
        names: ["jsonl"],
        module: json,
        dialect: crate::json::Dialect::Jsonl,
    }
    /// Python, with the scopes of Shiki's `python` grammar.
    Python { feature: "lang-python", names: ["python", "py"], module: python }
    /// YAML, with the scopes of Shiki's `yaml` grammar.
    Yaml { feature: "lang-yaml", names: ["yaml", "yml"], module: yaml }
    /// TOML, with the scopes of Shiki's `toml` grammar.
    Toml { feature: "lang-toml", names: ["toml"], module: toml }
    /// Makefile, with the scopes of Shiki's `make` grammar.
    Make { feature: "lang-make", names: ["make", "makefile"], module: make, reads_line_feed: true }
    /// Diff, with the scopes of Shiki's `diff` grammar.
    Diff { feature: "lang-diff", names: ["diff"], module: diff }
    /// Dockerfile, with the scopes of Shiki's `docker` grammar.
    Dockerfile { feature: "lang-dockerfile", names: ["docker", "dockerfile"], module: dockerfile }
    /// Java, with the scopes of Shiki's `java` grammar.
    Java { feature: "lang-java", names: ["java"], module: java }
    /// C, with the scopes of Shiki's `c` grammar.
    C { feature: "lang-c", names: ["c"], module: c, reads_line_feed: true }
    /// C++, with the scopes of Shiki's `cpp` grammar.
    Cpp { feature: "lang-cpp", names: ["cpp", "c++"], module: cpp, reads_line_feed: true }
    /// SQL, with the scopes of Shiki's `sql` grammar.
    Sql { feature: "lang-sql", names: ["sql"], module: sql }
    /// INI, with the scopes of Shiki's `ini` grammar.
    Ini { feature: "lang-ini", names: ["ini", "properties"], module: ini }
    /// GraphQL, with the scopes of Shiki's `graphql` grammar.
    GraphQl { feature: "lang-graphql", names: ["graphql", "gql"], module: graphql }
    /// CSS, with the scopes of Shiki's `css` grammar.
    Css { feature: "lang-css", names: ["css"], module: css }
    /// Kotlin, with the scopes of Shiki's `kotlin` grammar.
    Kotlin { feature: "lang-kotlin", names: ["kotlin", "kt", "kts"], module: kotlin }
    /// Go, with the scopes of Shiki's `go` grammar.
    Go { feature: "lang-go", names: ["go", "golang"], module: go }
    /// Ruby, with the scopes of Shiki's `ruby` grammar.
    Ruby { feature: "lang-ruby", names: ["ruby", "rb"], module: ruby }
    /// C#, with the scopes of Shiki's `csharp` grammar.
    CSharp { feature: "lang-csharp", names: ["csharp", "c#", "cs"], module: csharp }
    /// XML, with the scopes of Shiki's `xml` grammar.
    Xml { feature: "lang-xml", names: ["xml"], module: xml }
    /// Swift, with the scopes of Shiki's `swift` grammar.
    Swift { feature: "lang-swift", names: ["swift"], module: swift }
    /// PowerShell, with the scopes of Shiki's `powershell` grammar, which
    /// nishiki's `TextMate` engine runs.
    PowerShell {
        feature: "lang-powershell",
        names: ["powershell", "ps", "ps1", "pwsh"],
        module: powershell,
        reads_line_feed: true,
    }
    /// Scala, with the scopes of Shiki's `scala` grammar, which nishiki's
    /// `TextMate` engine runs.
    Scala {
        feature: "lang-scala",
        names: ["scala"],
        module: scala,
        reads_line_feed: true,
    }
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

/// The end of the rule that embeds a language, as a test of the line's bytes
/// at a place: a `TextMate` engine tries that end at each position while no
/// rule of the embedded grammar is open, before the embedded patterns, so a
/// match that starts before it still wins and one that starts at it loses.
/// It never holds at a space or tab, because the rule engine's scan skips
/// runs of them.
pub(crate) type Halt = fn(&[u8], usize) -> bool;

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
