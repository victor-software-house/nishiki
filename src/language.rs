//! The languages nishiki highlights: one table of rows, from which the
//! `Language` enum and every function that has an arm per language are
//! generated.
//!
//! Past fifty languages, LLVM treats each arm of a dispatch as a cold call
//! site, with an inline threshold of 45.
//! A language's entry wrapper costs more than that, so it would become a
//! call once its arm turned cold.
//! Each arm of [`Language::start`] and [`Language::lex_embedded`] therefore
//! calls a nested function of its own, whose one caller is that arm.
//! Inside the nested function, the wrapper sits on a warm path.
//! LLVM then decides it as it did at fewer languages.
//! The nested function itself may stay a call.
//! Each arm then costs at most one call, whatever the number of languages.

use crate::Token;
use crate::arena::Arena;
use crate::limits::{Finished, Limits, LineReport};
use crate::scratch::Scratch;

/// The language list, and with it every function that has one arm per
/// language: [`Language`], [`Language::ALL`], [`Language::id`], the names
/// [`Language::from_name`] reads, and the three lexer entry points.
///
/// A row is one language: its variant, its Shiki names (the first is its id),
/// what lexes it, and whether its rules end at the line feed of a line that
/// the containers around a Markdown fence took whole. A hand-written lexer is
/// a `module`, optionally with the dialect the module is told; a language that
/// nishiki's `TextMate` engine runs names its grammar table as `engine`, and
/// the three entry points call the engine with it. An engine grammar that
/// hands regions to languages lexed by hand says `hands_off: true`, and its
/// entry points call the engine's handing loop.
/// An engine row may name an `embedded` table root.
/// A language that embeds it starts it at that root.
/// PHP's names the root of a Markdown `php` fence, which tries HTML and PHP.
/// A module that lexes the text from the line's start says `from_start: true`.
/// Each entry point then calls its `line` through
/// [`Context::lex_from_start`], with a closure of its own.
/// Each closure gets its own copy of `lex_from_start`, with one caller.
/// A small hand-written lexer says `inlined: true`.
/// Its arm of [`Language::lex`] holds the lexer inline.
/// Every other arm calls a `lex_arm` of its own, which has that one caller.
/// A new hand-written language is one row here, its scope table in
/// `src/scope.rs`, its frame tag in `src/tag.rs`, its fence names in
/// `src/markdown/fence.rs`, its field in `src/scratch.rs` when it keeps
/// buffers, and its `lang-*` feature in `Cargo.toml`, with a `cfg` on each of
/// these. A new engine language needs no scope table, frame tag or field: its
/// module holds only the generated `grammar.rs`.
macro_rules! languages {
    (@flag) => { false };
    (@flag $flag:literal) => { $flag };
    (@pick [$grammar:path] []) => { $grammar };
    (@pick [$grammar:path] [$embedded:path]) => { $embedded };
    (@start [$module:ident] [] [$($dialect:expr)?] $arena:ident $base:ident) => {
        crate::$module::start($arena, $base $(, $dialect)?)
    };
    (@start [] [$grammar:path] [] $arena:ident $base:ident) => {
        crate::textmate::start($arena, &$grammar, $base)
    };
    (@start_embedded [$module:ident] [] [] $arena:ident $base:ident) => {
        crate::$module::start_embedded($arena, $base)
    };
    (@start_embedded [] [$grammar:path] [$($embedded:path)?] $arena:ident $base:ident) => {
        crate::textmate::start_embedded($arena, &languages!(@pick [$grammar] [$($embedded)?]), $base)
    };
    (@lex [] [$module:ident] [] [$($dialect:expr)?] [] $cx:ident $line:ident $state:ident) => {
        crate::$module::lex($cx, $($dialect,)? $line, $state)
    };
    (@lex [true] [$module:ident] [] [] [] $cx:ident $line:ident $state:ident) => {
        $cx.lex_from_start($line, |cx, rest| crate::$module::line(cx, rest, $state))
    };
    (@lex [] [] [$grammar:path] [] [] $cx:ident $line:ident $state:ident) => {
        crate::limits::Finished::unspent(crate::textmate::lex($cx, &$grammar, $line, $state))
    };
    (@inlined [true] $from_start:tt $module:tt $grammar:tt $dialect:tt $hands_off:tt $cx:ident $line:ident $state:ident) => {
        languages!(@lex $from_start $module $grammar $dialect $hands_off $cx $line $state).state()
    };
    (@inlined [] $from_start:tt $module:tt $grammar:tt $dialect:tt $hands_off:tt $cx:ident $line:ident $state:ident) => {{
        /// The language's lexer, called from its arm of [`Language::lex`]
        /// alone.
        #[inline]
        fn lex_arm(cx: &mut Context<'_>, line: Line<'_>, state: u32) -> u32 {
            languages!(@lex $from_start $module $grammar $dialect $hands_off cx line state).state()
        }
        lex_arm($cx, $line, $state)
    }};
    (@lex [] [] [$grammar:path] [] [true] $cx:ident $line:ident $state:ident) => {
        crate::limits::Finished::unspent(crate::textmate::lex_handing($cx, &$grammar, $line, $state))
    };
    (@embedded [] [$module:ident] [] [] [$($dialect:expr)?] [] $cx:ident $line:ident $state:ident) => {
        crate::$module::lex($cx, $($dialect,)? $line, $state)
    };
    (@embedded [true] [$module:ident] [] [] [] [] $cx:ident $line:ident $state:ident) => {
        $cx.lex_from_start($line, |cx, rest| crate::$module::line(cx, rest, $state))
    };
    (@embedded [] [] [$grammar:path] [$($embedded:path)?] [] [] $cx:ident $line:ident $state:ident) => {
        crate::textmate::lex_embedded($cx, &languages!(@pick [$grammar] [$($embedded)?]), $line, $state)
    };
    (@embedded [] [] [$grammar:path] [$($embedded:path)?] [] [true] $cx:ident $line:ident $state:ident) => {
        crate::textmate::lex_handing_embedded($cx, &languages!(@pick [$grammar] [$($embedded)?]), $line, $state)
    };
    (@until [] [$($module:ident)?] [$($grammar:path)?] [$($embedded:path)?] [$($dialect:expr)?] [] $cx:ident $line:ident $state:ident $end:ident $after:ident) => {{
        let _ = ($cx, $line, $state, $end, $after);
        None
    }};
    (@until [true] [$module:ident] [] [] [$($dialect:expr)?] [] $cx:ident $line:ident $state:ident $end:ident $after:ident) => {
        Some(crate::$module::lex_until($cx, $($dialect,)? $line, $state, $end, $after))
    };
    (@until [true] [] [$grammar:path] [$($embedded:path)?] [] [] $cx:ident $line:ident $state:ident $end:ident $after:ident) => {
        Some(crate::textmate::lex_until($cx, &languages!(@pick [$grammar] [$($embedded)?]), $line, $state, $end, $after))
    };
    (@until [true] [] [$grammar:path] [$($embedded:path)?] [] [true] $cx:ident $line:ident $state:ident $end:ident $after:ident) => {
        Some(crate::textmate::lex_handing_until($cx, &languages!(@pick [$grammar] [$($embedded)?]), $line, $state, $end, $after))
    };
    ($(
        $(#[$meta:meta])*
        $variant:ident {
            feature: $feature:literal,
            names: [$id:literal $(, $alias:literal)*],
            $(module: $module:ident)?
            $(engine: $grammar:path)?
            $(, embedded: $embedded:path)?
            $(, dialect: $dialect:expr)?
            $(, reads_line_feed: $line_feed:literal)?
            $(, halts: $halts:tt)?
            $(, hands_off: $hands_off:tt)?
            $(, from_start: $from_start:tt)?
            $(, inlined: $inlined:tt)?
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
            ///
            /// It reads a table, not a `match`.
            /// LLVM lowers a `match` on the language as a switch, and weighs
            /// the switch's branches by how many rows set the flag.
            /// Markdown's `step` inlines this test.
            /// A `match` here let one row's flag move the blocks of `step`.
            #[cfg(feature = "lang-markdown")]
            pub(crate) fn reads_line_feed(self) -> bool {
                /// Each row's flag, in the order of [`Language::ALL`].
                static FLAGS: [bool; Language::ALL.len()] =
                    [$(#[cfg(feature = $feature)] languages!(@flag $($line_feed)?)),*];
                FLAGS.get(self.index()).copied().unwrap_or(false)
            }

            /// The state at the start of a document whose scopes sit on
            /// `base`.
            pub(crate) fn start(self, arena: &mut Arena, base: u32) -> u32 {
                match self {
                    $(#[cfg(feature = $feature)] Language::$variant => {
                        /// This arm's start, with the arm as its one caller.
                        fn start_arm(arena: &mut Arena, base: u32) -> u32 {
                            languages!(@start [$($module)?] [$($grammar)?] [$($dialect)?] arena base)
                        }
                        start_arm(arena, base)
                    })*
                }
            }

            /// The state at the start of a Markdown fence's body, whose text
            /// sits on `base` with no root scope of the language's own, as in
            /// Shiki's embedding.
            pub(crate) fn start_embedded(self, arena: &mut Arena, base: u32) -> u32 {
                match self {
                    $(#[cfg(feature = $feature)] Language::$variant => languages!(@start_embedded [$($module)?] [$($grammar)?] [$($embedded)?] arena base),)*
                }
            }

            /// Lexes `line` from its `start` and from `state`, appending
            /// tokens whose ends count from the start of the whole line, and
            /// returns the state after it. The highlighter calls it for the
            /// line's own language, which needs no count back.
            ///
            /// The rows marked `inlined` lex here.
            /// Every other arm calls a function that holds its lexer.
            #[inline(always)]
            pub(crate) fn lex(self, cx: &mut Context<'_>, line: Line<'_>, state: u32) -> u32 {
                match self {
                    $(#[cfg(feature = $feature)] Language::$variant => languages!(@inlined [$($inlined)?] [$($from_start)?] [$($module)?] [$($grammar)?] [$($dialect)?] [$($hands_off)?] cx line state),)*
                }
            }

            /// Lexes `line` like [`Language::lex`], for a language that
            /// another one embeds, and returns the steps it spent beside the
            /// state, which the embedding lexer adds to its own count.
            ///
            /// Embedded lines are the rare path, so this dispatch is cold
            /// and out of line: the lexers inline into [`Language::lex`]
            /// and its out-of-line arms alone, and the count stays off the
            /// highlighter's path.
            #[cfg_attr(
                not(any(feature = "lang-markdown", feature = "_embedding", feature = "_textmate")),
                expect(
                    dead_code,
                    reason = "only Markdown, the rule engine's embedding and the engine embed through the dispatch"
                )
            )]
            #[cold]
            #[inline(never)]
            pub(crate) fn lex_embedded(
                self,
                cx: &mut Context<'_>,
                line: Line<'_>,
                state: u32,
            ) -> Finished {
                match self {
                    $(#[cfg(feature = $feature)] Language::$variant => {
                        /// This arm's lexer, with the arm as its one caller.
                        fn embedded_arm(cx: &mut Context<'_>, line: Line<'_>, state: u32) -> Finished {
                            languages!(@embedded [$($from_start)?] [$($module)?] [$($grammar)?] [$($embedded)?] [$($dialect)?] [$($hands_off)?] cx line state)
                        }
                        embedded_arm(cx, line, state)
                    })*
                }
            }

            /// Lexes `line` like [`Language::lex`], for a language that
            /// another one embeds: the line halts at the first place where
            /// `end` holds that the code reaches with only the language's
            /// root open, or where `after` holds and nothing of the language
            /// matches, and the second value says where and why it stopped
            /// ([`Stopped`]). `None` for a language that cannot halt.
            #[cfg_attr(
                not(any(feature = "_embedding", feature = "_textmate")),
                expect(
                    dead_code,
                    reason = "only the rule engine's embedding and the engine embed other languages"
                )
            )]
            #[inline]
            pub(crate) fn lex_until(
                self,
                cx: &mut Context<'_>,
                line: Line<'_>,
                state: u32,
                end: &dyn Halt,
                after: Option<&dyn Halt>,
            ) -> Option<(Finished, Option<Stopped>)> {
                match self {
                    $(#[cfg(feature = $feature)] Language::$variant => languages!(@until [$($halts)?] [$($module)?] [$($grammar)?] [$($embedded)?] [$($dialect)?] [$($hands_off)?] cx line state end after),)*
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
    ///
    /// # Behaviour kept from Shiki
    ///
    /// - `||` is two `|` operators.
    /// - `a!=b` reads `a!` as a macro.
    /// - A camel-case name such as `fooBar` gets no scope.
    /// - Every non-ASCII byte counts as a word character.
    Rust { feature: "lang-rust", names: ["rust", "rs"], module: rust, inlined: true }
    /// Markdown, with the scopes of Shiki's `markdown` grammar. A fence of
    /// another supported language is lexed as that language.
    ///
    /// # Behaviour kept from Shiki
    ///
    /// - An empty line leaves the state unchanged.
    ///   A line of spaces does not.
    /// - With `lang-html-derivative`, html-derivative lexes the HTML in the five places below, where Shiki's grammar includes it.
    ///   Without it, that HTML is plain text, as in Shiki without the grammar.
    /// - An HTML block, a line that starts with a tag, runs to a line of blanks, not to an empty line, which Shiki does not tokenize.
    /// - A `<script>`, `<style>` or `<pre>` block that its first line does not close runs to the line with the closing tag, whose text before the tag html-derivative lexes from a fresh state.
    /// - In a paragraph, html-derivative's root rules come after the inline rules at each position, and a tag or entity hands the line back once it ends.
    /// - In a heading, html-derivative reads the heading's text the same way, cut at the end of that text.
    /// - In bold, italic and strikethrough, a `<` with a `>` later on the line hands the tags there to html-derivative, up to the first place after a `>` where no tag starts.
    /// - An HTML block inside a quote or a list starts again on each line, from a fresh HTML state, because the grammar's `while` rules match only at the start of a line.
    /// - A label or link destination nested deeper than 32 brackets ends there.
    /// - The grammar's backtracking is followed where it matters in practice, not everywhere.
    ///   A code span gives back backticks,
    ///   but an escaped `>` does not end an angle-bracket destination.
    /// - A fence of more than 255 characters is never closed.
    /// - With `lang-html`, a `php` fence is HTML under `meta.embedded.block.php`, since the grammar includes HTML there before PHP, which nishiki does not lex.
    /// - A dot in the fence names `css.erb`, `Berksfile.lock` and `.textmate_init` matches any one character.
    ///   The grammar leaves it unescaped, so `cssXerb` embeds CSS.
    Markdown {
        feature: "lang-markdown",
        names: ["markdown", "md"],
        module: markdown,
        halts: true,
        from_start: true,
        inlined: true,
    }
    /// `TypeScript`, with the scopes of Shiki's `typescript` grammar.
    ///
    /// # Behaviour kept from Shiki
    ///
    /// - `Number.MAX_VALUE` gets no scope.
    /// - An empty line leaves the state as it is.
    /// - JavaScript, JSX and TSX are one grammar with three spellings of its scope names.
    /// - They differ from TypeScript in two places:
    ///   - they read `<` as a JSX tag;
    ///   - they change the generic arrow function lookaheads that tags would confuse.
    TypeScript {
        feature: "lang-typescript",
        names: ["typescript", "ts", "cts", "mts"],
        module: typescript,
        dialect: crate::typescript::Dialect::TypeScript,
        halts: true,
    }
    /// `JavaScript`, with the scopes of Shiki's `javascript` grammar.
    ///
    /// [`Language::TypeScript`] lists the behaviour it keeps from Shiki.
    JavaScript {
        feature: "lang-javascript",
        names: ["javascript", "js", "cjs", "mjs"],
        module: typescript,
        dialect: crate::typescript::Dialect::JavaScript,
        halts: true,
    }
    /// `JSX`, with the scopes of Shiki's `jsx` grammar.
    ///
    /// [`Language::TypeScript`] lists the behaviour it keeps from Shiki.
    Jsx {
        feature: "lang-jsx",
        names: ["jsx"],
        module: typescript,
        dialect: crate::typescript::Dialect::Jsx,
    }
    /// `TSX`, with the scopes of Shiki's `tsx` grammar.
    ///
    /// [`Language::TypeScript`] lists the behaviour it keeps from Shiki.
    Tsx {
        feature: "lang-tsx",
        names: ["tsx"],
        module: typescript,
        dialect: crate::typescript::Dialect::Tsx,
    }
    /// Shell, with the scopes of Shiki's `shellscript` grammar.
    ///
    /// # Behaviour kept from Shiki
    ///
    /// - An unterminated string or here-document keeps every later line.
    /// - `||` outside a statement is two `|` operators.
    /// - An argument such as `ab>c` is split as `a` and `b>c`.
    /// - An empty line leaves the state as it is, because Shiki does not tokenize empty lines.
    /// - A non-ASCII character is a word character when it is alphanumeric, so combining marks are not.
    /// - A non-ASCII character is white space when Unicode says so.
    Shell {
        feature: "lang-shell",
        names: ["shellscript", "bash", "sh", "shell", "zsh"],
        module: shell,
        reads_line_feed: true,
        halts: true,
    }
    /// JSON, with the scopes of Shiki's `json` grammar.
    ///
    /// # Behaviour kept from Shiki
    ///
    /// - Every dialect's comments are read.
    /// - A stray character in an array or object gets the grammar's `invalid.illegal` scope.
    /// - `\d`, `\w` and `\s` for non-ASCII characters come from a table measured on Shiki 4.6.0 (`oracle/classes.ts`).
    ///   A later Unicode version in Shiki's engine would need the table regenerated.
    Json {
        feature: "lang-json",
        names: ["json"],
        module: json,
        dialect: crate::json::Dialect::Json,
        halts: true,
        inlined: true,
    }
    /// JSON with comments, with the scopes of Shiki's `jsonc` grammar.
    ///
    /// [`Language::Json`] lists the behaviour it keeps from Shiki.
    Jsonc {
        feature: "lang-json",
        names: ["jsonc"],
        module: json,
        dialect: crate::json::Dialect::Jsonc,
        halts: true,
        inlined: true,
    }
    /// JSON Lines, with the scopes of Shiki's `jsonl` grammar.
    ///
    /// [`Language::Json`] lists the behaviour it keeps from Shiki.
    Jsonl {
        feature: "lang-json",
        names: ["jsonl"],
        module: json,
        dialect: crate::json::Dialect::Jsonl,
        halts: true,
        inlined: true,
    }
    /// Python, with the scopes of Shiki's `python` grammar.
    ///
    /// # Behaviour kept from Shiki
    ///
    /// - `def` opens a function after the end of any word, as in `x def f(): pass`.
    /// - An empty line leaves the state as it is.
    /// - A single-quoted docstring ends at the line end even after a backslash, so the next line is code; a single-quoted string goes on.
    /// - `\d`, `\w`, `\s`, `[[:alpha:]]` and `\p{upper}` for non-ASCII characters come from tables measured on Shiki 4.6.0 (`oracle/classes.ts`).
    ///   A later Unicode version in Shiki's engine would need them regenerated.
    Python { feature: "lang-python", names: ["python", "py"], module: python, halts: true }
    /// YAML, with the scopes of Shiki's `yaml` grammar.
    ///
    /// # Behaviour kept from Shiki
    ///
    /// - The lexer follows the YAML 1.1 type patterns that Shiki's grammar uses:
    ///   - `0o17` is plain text;
    ///   - an exponent needs its sign;
    ///   - `y` and `n` are booleans, as keys too.
    /// - Each line is read on its own, so a plain scalar on several lines is scoped line by line.
    /// - A comment whose text holds a tab is not a comment.
    /// - Front matter starts only on the first line of a Markdown document.
    ///   The state of a document's first line therefore differs from the states after it.
    /// - A line of a block scalar is text up to the first line indented less than the scalar's first line.
    Yaml { feature: "lang-yaml", names: ["yaml", "yml"], module: yaml, halts: true, inlined: true }
    /// TOML, with the scopes of Shiki's `toml` grammar.
    ///
    /// # Behaviour kept from Shiki
    ///
    /// - These are `invalid.illegal`:
    ///   - a value that follows another in an array without a comma;
    ///   - text that is neither a key, a value nor a table header;
    ///   - a stray character after a value.
    /// - `01` is the number `0` and then an invalid `1`.
    /// - A table header is read only at the start of a line.
    /// - A key that begins with a `"` ends at the first later `"` that an `=` follows.
    /// - `\s` for non-ASCII characters is JSON's table.
    ///   A pass over every character confirmed it for this grammar too.
    Toml { feature: "lang-toml", names: ["toml"], module: toml, inlined: true }
    /// Makefile, with the scopes of Shiki's `make` grammar.
    ///
    /// # Behaviour kept from Shiki
    ///
    /// - `$` at the end of a recipe line is not a variable.
    /// - A `:` in any line that is not an assignment makes it a target.
    /// - The `endef` of a definition has the scope of `override`.
    /// - A Markdown fence's Makefile lines see the line feed of a line that a quote or list took whole.
    ///   They close the references that line left open.
    Make { feature: "lang-make", names: ["make", "makefile"], module: make, reads_line_feed: true }
    /// Diff, with the scopes of Shiki's `diff` grammar.
    ///
    /// # Behaviour kept from Shiki
    ///
    /// - Diff lines are lexed on their own, as the grammar has no state.
    /// - A Markdown fence's diff lines lose their `^` anchors when a quote or list indents the fence.
    Diff { feature: "lang-diff", names: ["diff"], module: diff, inlined: true }
    /// Dockerfile, with the scopes of Shiki's `docker` grammar.
    ///
    /// # Behaviour kept from Shiki
    ///
    /// - A comment is read only at the start of a line, and never inside a string.
    /// - A string runs across lines until its quote.
    ///   An apostrophe in a `RUN` line therefore opens a string that swallows the instructions after it.
    /// - A line that continues another still reads its first word as an instruction.
    /// - A Markdown fence names the language `dockerfile`.
    ///   The `docker` fence is plain text, as in Shiki.
    Dockerfile {
        feature: "lang-dockerfile",
        names: ["docker", "dockerfile"],
        module: dockerfile,
        inlined: true,
    }
    /// Java, with the scopes of Shiki's `java` grammar.
    ///
    /// # Behaviour kept from Shiki
    ///
    /// - A string without its closing quote keeps the lines after it until a quote closes it.
    /// - A Javadoc comment is read only where `/**` starts a line, after blanks.
    /// - A declaration is found by a lookahead over the rest of the line, as in the grammar:
    ///   - a method is a line that has a blank and then a `(`, with no `=` or `//` before it;
    ///   - a variable is a type, a name and a `:`, `;` or `=`.
    /// - `\w`, `\s` and `\d` for non-ASCII characters come from the table measured for JSON (`oracle/classes.ts`).
    Java { feature: "lang-java", names: ["java"], module: java }
    /// C, with the scopes of Shiki's `c` grammar.
    ///
    /// # Behaviour kept from Shiki
    ///
    /// - White space before a comment belongs to the comment.
    /// - A word that starts with `or`, `not` or `and` after a type opens an initialization,
    ///   as in `int order(int y);` inside a function.
    /// - `#define` is found anywhere in a line.
    ///   The other directives need the start of the line,
    ///   so in a Markdown fence indented by a list item a directive is plain text.
    /// - An unterminated string keeps every later line.
    /// - `\d`, `\w` and `\s` for non-ASCII characters come from the table measured for JSON (`oracle/classes.ts`).
    C { feature: "lang-c", names: ["c"], module: c, reads_line_feed: true, halts: true }
    /// C++, with the scopes of Shiki's `cpp` grammar.
    ///
    /// # Behaviour kept from Shiki
    ///
    /// - A Markdown fence's C++ lines keep `source.cpp` on the stack, where C's lose `source.c`: Markdown's grammar names it in the fence's `contentName`.
    /// - A Markdown fence names the languages `cpp`, `c++` and `cxx`.
    /// - Raw strings are lexed without the grammars that Shiki embeds in `R"re(`, `R"_r(`, `R"regex(`, `R"glsl(` and `R"GLSL(`.
    ///   Their text is plain raw-string text.
    /// - `\d`, `\w` and `\s` for non-ASCII characters come from the table measured for JSON (`oracle/classes.ts`).
    /// - Bounds the lexer sets itself, beside `Limits`:
    ///   - template arguments and bracket groups nested deeper than 20 levels do not match, as in the Oniguruma engine Shiki runs;
    ///   - a repetition that a matcher follows by recursion, such as a run of type specifiers, stops after 64 iterations;
    ///   - an attribute tries its first 16 closings, so a line with more reads as having no later ones;
    ///   - comments between two tokens give back at most 8 of themselves when what follows fails;
    ///   - capture groups lexed again inside one another, as a long `a::b::c::` chain's are, stop after 64 levels, and the text of a deeper one keeps its own scopes.
    Cpp { feature: "lang-cpp", names: ["cpp", "c++"], module: cpp, reads_line_feed: true, halts: true }
    /// SQL, with the scopes of Shiki's `sql` grammar.
    ///
    /// # Behaviour kept from Shiki
    ///
    /// - `#` starts no comment.
    /// - A string that closes on its line ignores a backslash before its quote.
    /// - A name after a `.` is a table name, even when it is a number.
    /// - A keyword is matched without regard to case, as Shiki's engine does, so `ſelect` is `select`.
    Sql { feature: "lang-sql", names: ["sql"], module: sql, halts: true }
    /// INI, with the scopes of Shiki's `ini` grammar.
    ///
    /// # Behaviour kept from Shiki
    ///
    /// - A `#` or `;` starts a comment anywhere outside a string, so `a = b # c` has one.
    /// - A `key = value` is found anywhere in a line, so `a = b = c` has two keys.
    /// - A double-quoted string has no escapes.
    /// - A string runs across lines until its quote.
    /// - A section header is read only at the start of a line.
    ///   A Markdown fence's INI lines lose that anchor when a quote or list indents the fence.
    /// - A Markdown fence names the language `ini` or `conf`.
    ///   A `properties` fence is plain text, as in Shiki.
    Ini { feature: "lang-ini", names: ["ini", "properties"], module: ini, inlined: true }
    /// GraphQL, with the scopes of Shiki's `graphql` grammar.
    ///
    /// # Behaviour kept from Shiki
    ///
    /// - The grammar's comment rule comes first everywhere.
    ///   It reads every `"..."` and `"""..."""` as a comment that runs across lines.
    ///   The exception is a string that follows white space where a value is expected, which is a string.
    /// - An enum value's scope includes the white space before it.
    /// - A rule that matches nothing closes the rule it sits in, so `type A x` ends the type at `x`.
    /// - A Markdown fence is plain text, as Shiki's Markdown grammar has no rule that embeds GraphQL.
    /// - A `${...}` in a selection set is lexed as JavaScript by the JavaScript lexer.
    ///   It carries its state across lines, up to the first `}` that no context of the code is waiting for.
    GraphQl { feature: "lang-graphql", names: ["graphql", "gql"], module: graphql, halts: true, inlined: true }
    /// CSS, with the scopes of Shiki's `css` grammar.
    ///
    /// # Behaviour kept from Shiki
    ///
    /// - `@charset` is read as the charset rule only on the first line of a document, never in a Markdown fence, as in Shiki.
    /// - A scope name that Shiki builds from the matched text keeps its case:
    ///   - Shiki writes `ONLY` or `Not` in a media query as typed;
    ///   - Shiki writes `@styleset` or `@stylistic` spelled with `ſ` or a ligature (`@ﬆyleset`) under its own name,
    ///     where nishiki gives the lower case name of the ASCII letters.
    /// - A line that follows a backslash at its end belongs to the escape for the rest of the rule, as in Shiki.
    ///   The grammar's end pattern `^(?<!\G)` never matches again.
    /// - A Markdown fence names the languages `css` and `css.erb`.
    Css { feature: "lang-css", names: ["css"], module: css, halts: true }
    /// Kotlin, with the scopes of Shiki's `kotlin` grammar.
    ///
    /// # Behaviour kept from Shiki
    ///
    /// - `/**/` opens a `KDoc` comment, because `/**` is read before `/**/` could close, so the comment runs on until a later `*/`.
    /// - A `KDoc` tag is read anywhere inside a `KDoc` comment, not only at the start of a line.
    /// - `a==b` is a key, `a=`, and an assignment: the key pattern needs one word character, `=` and then another `=`.
    /// - `@a.b:` is the annotation `@a.`, because the pattern gives characters back until no `:` follows.
    /// - A template expression `${...}` ends at its first `}`, so a lambda inside it ends it early.
    /// - `in` and `out` in a type parameter list are types, not modifiers: the pattern for any word comes first.
    /// - A type annotation reads on over words, blanks, `?`, `->` and brackets without quotes.
    ///   `a: Foo, b` ends the annotation at the comma.
    /// - `fun Foo.` with no name after the dot makes `Foo` the function's name.
    /// - A line that opens a group of generics a hundred thousand times and closes it once runs into the work limit.
    ///   The grammar's pattern scans from each opener, and so does the lexer.
    /// - `\w`, `\s` and `\d` for non-ASCII characters come from the table measured for JSON (`oracle/classes.ts`).
    Kotlin { feature: "lang-kotlin", names: ["kotlin", "kt", "kts"], module: kotlin }
    /// Go, with the scopes of Shiki's `go` grammar.
    ///
    /// # Behaviour kept from Shiki
    ///
    /// - A number takes every letter, digit, `_` and `.` after it, and a run that is not one literal, such as `1x` or `0x`, is `invalid.illegal.constant.numeric.go`.
    /// - A `case` line, a label on its own line and a `break`, `goto` or `continue` with its label are read only from the start of the line.
    ///   Inside a Markdown fence that a quote or list indents, they lose that anchor.
    /// - A line that opens a struct many times, such as `type T struct {` repeated, may run into the work limit.
    ///   The grammar's expressions scan from each opener, and so does the lexer.
    /// - Three cases differ from Shiki, all found by mutating the inputs, never in the inputs themselves:
    ///   - after a struct or interface field that ends in text such as `interface{&=`, Shiki leaves a different state for the next line;
    ///   - a non-ASCII character earlier in a line can change which expression Shiki's engine matches;
    ///   - on a long line of many words Shiki's engine stops an expression at its retry limit and leaves the rest of the line plain, where nishiki lexes it.
    /// - A Markdown fence names the language `go` or `golang`.
    /// - `\w`, `\s` and `\d` for non-ASCII characters come from the table measured for JSON (`oracle/classes.ts`).
    Go { feature: "lang-go", names: ["go", "golang"], module: go }
    /// Ruby, with the scopes of Shiki's `ruby` grammar.
    ///
    /// # Behaviour kept from Shiki
    ///
    /// - A here document named for SQL, GraphQL, CSS, C, C++, HTML, XML, JavaScript, YAML, shell, Lua or Ruby, such as `<<~SQL`, lexes its lines in that language, with Ruby's interpolation and escapes where the language has nothing open, as Shiki does.
    /// - In one named for YAML, an indented `#{` is a YAML comment, as in Shiki: the grammar's comment rule takes the blanks before it.
    /// - One named for HAML, such as `<<~HAML`, keeps that language's scopes, but its lines are plain text with Ruby's interpolation.
    ///   Shiki lexes them as HAML.
    /// - One named for Slim is plain text with Ruby's interpolation, as in Shiki, which has no Slim grammar.
    /// - `__END__` is a constant, and the text after it is Ruby, as in Shiki: the grammar's rule for the data after `__END__` comes after the rule for constants, so it never matches.
    Ruby { feature: "lang-ruby", names: ["ruby", "rb"], module: ruby, halts: true }
    /// C#, with the scopes of Shiki's `csharp` grammar.
    ///
    /// # Behaviour kept from Shiki
    ///
    /// - A `///` or `//` comment continues through each later line that starts with the same marker after blanks, as one comment.
    /// - A `(` opens a tuple when a `,` or `:` follows anywhere later on the line, as the grammar's lookahead reads the whole line.
    /// - A `/**` comment's lines are checked with `^`, which matches only at column 0.
    ///   In a Markdown fence indented by a list item or a quote, the comment loses its line prefixes.
    /// - Numeric literals follow the grammar's five nested patterns, so `5.` and `1_a` are invalid numbers.
    /// - A type nested more than 64 deep in angle brackets or parentheses reads as no type.
    ///   A type with more than 64 dotted names, pointer marks and array suffixes reads as its first 64.
    ///   A base-class constructor call with more than 32 dotted names reads as its first 32.
    ///   These bounds are constants in `src/csharp/limits.rs`; Oniguruma has larger ones of its own.
    /// - A Markdown fence names the language `cs`, `csharp` or `c#`.
    ///   The `csx` and `c-sharp` fences are plain text, as in Shiki.
    CSharp { feature: "lang-csharp", names: ["csharp", "c#", "cs"], module: csharp }
    /// XML, with the scopes of Shiki's `xml` grammar.
    ///
    /// # Behaviour kept from Shiki
    ///
    /// - A `--` inside a comment opens a rule that has no end.
    ///   The rest of the document is then a comment, and `-->` ends nothing.
    /// - `<%--` opens a rule that has no end and no scope.
    ///   The rest of the document has the scope of the document alone.
    /// - An attribute is read only after white space or at the start of a line, so `a="b"c="d"` has one.
    ///   An attribute's name takes the white space before it, not the `=`.
    /// - A `&` that does not start an entity, as in `&a b`, is an illegal ampersand.
    /// - A tag whose first line holds its closing tag, as in `<a b></a>`, is one rule, and the closing tag must repeat the name.
    /// - `<% ... %>` is Java, lexed by the Java lexer.
    ///   A `%>` ends the code only when Java has no context open, so one inside a string, a block or a `//` comment belongs to Java.
    /// - A Markdown fence names the languages `xml`, `xsd`, `tld`, `jsp`, `pt`, `cpt`, `dtml`, `rss`, `opml`, `xsl` and `xslt`.
    Xml { feature: "lang-xml", names: ["xml"], module: xml, halts: true }
    /// Swift, with the scopes of Shiki's `swift` grammar.
    ///
    /// # Behaviour kept from Shiki
    ///
    /// - An operator in an expression is a custom operator, the known ones included:
    ///   `=`, `==`, `??` and `+` get `keyword.operator.custom.infix`.
    ///   The grammar's patterns for the known operators start with `\G`, and `\G` does not hold at the start of a capture group's own patterns in Shiki's engine.
    ///   A pattern that matches the operator directly names it, as with the `=` of a default argument in a parameter clause.
    /// - `\p{L}`, `\p{N}` and `\p{M}` for non-ASCII characters come from tables measured on Shiki's engine (`oracle/classes.ts`).
    /// - A Markdown fence names the language `swift`.
    ///   In a fence, `\G` holds at the start of every line, as Shiki's fence rule leaves it:
    ///   - the lines of a `"""` string after its opening line are `invalid.illegal.content-after-opening-delimiter`;
    ///   - a `}` that starts a line does not end an enum's case list, so the enum stays open after it.
    Swift { feature: "lang-swift", names: ["swift"], module: swift }
    /// PowerShell, with the scopes of Shiki's `powershell` grammar.
    ///
    /// # Behaviour kept from Shiki
    ///
    /// - A parameter such as `-Path` is a `-`, named `keyword.operator.assignment.powershell` as `+` and `=` are, and plain text after it.
    /// - A number followed by a letter ends at its dot: `1.x` is the number `1.` and `x`.
    /// - In a double-quoted string, `$a.Count` names `$a` only, and `.Count` is text.
    ///   A `$(...)` substatement in the string holds code, members included.
    /// - Keywords, commands and operators match without regard to case, and `ſ`, the Kelvin sign, `ß` and the Latin ligatures match the letters they fold to.
    /// - `\w`, `\s`, `\d`, `\p{L}` and `\p{N}` for non-ASCII characters come from tables measured on Shiki's engine (`oracle/classes.ts`).
    PowerShell { feature: "lang-powershell", names: ["powershell", "ps", "ps1", "pwsh"], module: powershell }
    /// Scala, with the scopes of Shiki's `scala` grammar.
    ///
    /// # Behaviour kept from Shiki
    ///
    /// - `\w`, `\s` and `\d` for non-ASCII characters come from the table measured for JSON (`oracle/classes.ts`).
    /// - Upper-case, lower-case and operator names beyond ASCII use three more tables measured on Shiki 4.6.0, in `oracle/classes.ts`.
    /// - In an interpolated string, `$` before a character that starts no name, such as `$1`, ends the string.
    /// - In a single-line interpolated string, `$$` is an escape, because the grammar lists `\$["$]` before the interpolation patterns.
    /// - In a triple-quoted interpolated string, `$$` is an interpolation, because there the interpolation patterns come first.
    /// - Bounds such as `<:`, `>:` and `<%` are operators.
    ///   The grammar's keyword rule comes before its bounds rule and takes every run of two or more operator characters.
    Scala { feature: "lang-scala", names: ["scala"], module: scala }
    /// HTML, with the scopes of Shiki's `html` grammar.
    ///
    /// # Behaviour kept from Shiki
    ///
    /// - A `<` that no rule claims is an illegal angle bracket, even inside a quoted attribute value, as the grammar's injection makes it.
    /// - An attribute's name is a word of the grammar's lists, or any other run of name characters, which gets a scope built from its text, such as `meta.attribute.unrecognized.fooBar.html`.
    /// - `svg` and `math` elements have rules of their own, and read the attributes of a start tag that the line holds whole on that text alone.
    /// - An invalid tag, such as `<foo>`, reads one of two attribute lists.
    ///   With MathML's, `close` is a known attribute and `for` is not; with HTML's, the reverse.
    ///   The list depends on the document's grammar, the outermost scope of the stack.
    ///   Shiki compiles the rule once per document grammar, and its attributes resolve through the first rule that reaches it.
    ///   Where html-derivative is loaded and the grammar reaches it first, the list is HTML's.
    ///   With Shiki 4.6.0 and every bundled grammar loaded:
    ///
    ///   | Document | Where HTML sits | List |
    ///   |:--|:--|:--|
    ///   | HTML | the document | MathML's |
    ///   | ERB | the document | MathML's |
    ///   | Perl, PHP, Ruby | a heredoc | MathML's |
    ///   | Elixir | a `~H` sigil | MathML's |
    ///   | HAML | a Markdown fence in `:markdown` | MathML's |
    ///   | Markdown | a fence, or PHP or Perl inside one | HTML's |
    ///   | Svelte | a Markdown fence in an `@component` comment | HTML's |
    ///   | html-derivative, Vue | the document | HTML's |
    /// - The CSS of a `<style>` element is lexed by the CSS lexer, and the JavaScript of a `<script>` element and of an event handler by the JavaScript lexer.
    ///   The code ends at its end tag, in any case, only where it has no rule open, so a `</style>` inside an open CSS block is CSS.
    ///   The grammar's own `//` and `/* */` rules in a script end at a lower-case `</script` only.
    /// - A `<script>` whose `type` is an HTML template holds HTML, and one of any other type that is not JavaScript is `source.unknown`.
    /// - A line stops after `MAX_STALLED` (4,096) steps in a row that do not advance, and the rest of it keeps the innermost rule's scope.
    ///   At one byte a run of such steps opens and closes each rule of the grammar at most once, because a rule that repeats stops the line, so no line comes near it.
    ///   The bound is a constant in `src/html/engine.rs` until the next minor release moves it into `Limits`.
    /// - A Markdown fence names the languages `html`, `htm`, `shtml`, `xhtml`, `inc`, `tmpl` and `tpl`.
    Html { feature: "lang-html", names: ["html"], module: html, halts: true }
    /// Zig, with the scopes of Shiki's `zig` grammar.
    ///
    /// # Behaviour kept from Shiki
    ///
    /// - `and` and `or` are keywords, never comparison operators.
    /// - `inline` before `fn` is a storage keyword, and a loop keyword anywhere else.
    /// - The comparison rule comes before the bitwise one, so `<<` is two `<` operators, and `<<%=` is `<`, `<` and `%=`.
    /// - A whole number such as `2` is a float.
    /// - `1e+` is the invalid number `1e` and an operator.
    ///   `1.e` is the number `1`, an accessor and the name `e`.
    Zig { feature: "lang-zig", names: ["zig"], module: zig }
    /// ERB, with the scopes of Shiki's `erb` grammar: HTML whose `<% %>`
    /// tags hold Ruby.
    ///
    /// # Behaviour kept from Shiki
    ///
    /// - The HTML lexer runs ERB, with ERB's injection in place of the HTML grammar's, so a bare `<` is text.
    /// - A `<%` opens ERB where no HTML rule matches at it, outside comments: in text and in a quoted attribute value, but not inside a start tag, where an attribute rule takes the `<`.
    /// - `<%#` is a comment up to `%>`, and `<%%` is no escape but code whose Ruby starts with `%`.
    /// - The code is Ruby, lexed by the Ruby lexer, up to `-%>` or `%>`.
    ///   The end counts only where Ruby has no context open, so a `%>` inside a Ruby string belongs to Ruby.
    ///   A `#` before the end on its line starts ERB's comment, and Ruby's own comment otherwise.
    /// - Code that its line does not close is `meta.embedded.block.erb`, and the blanks before it at the start of a line and after its end get scopes of their own.
    /// - In a `<style>` element ERB opens between CSS rules, and in a `<script>` element where no JavaScript rule matches, so JavaScript's `<` operator takes `<%=` in code.
    /// - A heredoc of another language inside the Ruby, such as `<<~SQL`, stays Ruby text.
    /// - ERB that Shiki opens inside a CSS rule or string, or inside a JavaScript string, template or JSX element, as in `'a-<%= x %>'` in a script, stays that language's text, because the CSS and JavaScript lexers halt for ERB only at their top level.
    Erb { feature: "lang-erb", names: ["erb"], module: erb }
    /// Lua, with the scopes of Shiki's `lua` grammar.
    ///
    /// # Behaviour kept from Shiki
    ///
    /// - The body of `ffi.cdef[[...]]` is the plain content of its brackets, where Shiki lexes it as C.
    /// - A long string or comment closes only at a bracket with as many `=` signs as it opened with, across lines.
    /// - A Markdown fence names the language `lua`.
    Lua { feature: "lang-lua", names: ["lua"], module: lua, halts: true }
    /// CMake, with the scopes of Shiki's `cmake` grammar.
    ///
    /// # Behaviour kept from Shiki
    ///
    /// - A `${…}` or `$ENV{…}` expansion inside a quoted string is string text; outside one it is `storage.source.cmake`.
    /// - A command is named only at the start of a line, and the blanks before it are part of its token.
    /// - Command names, keywords, variables and properties match without regard to case, and `ſ`, the Kelvin sign, `ß` and the Latin ligatures match the letters they fold to.
    /// - `\w` for non-ASCII characters comes from the table measured for JSON (`oracle/classes.ts`).
    CMake { feature: "lang-cmake", names: ["cmake"], module: cmake }
    /// Protobuf, with the scopes of Shiki's `proto` grammar.
    ///
    /// # Behaviour kept from Shiki
    ///
    /// - A field with no type takes its label as the type: in `required x = 1;`, `required` is the type.
    /// - A number before a letter ends at its dot: `1.5x` is the number `1.` and the text `5x`.
    /// - `reserved` is a statement only inside a message.
    /// - `\w`, `\s` and `\d` for non-ASCII characters come from the table measured for JSON (`oracle/classes.ts`).
    Proto { feature: "lang-proto", names: ["proto", "protobuf"], module: proto }
    /// html-derivative, with the scopes of Shiki's `html-derivative`
    /// grammar: HTML with a rule of its own for the tags no other rule takes.
    ///
    /// # Behaviour kept from Shiki
    ///
    /// - The HTML lexer runs html-derivative, the HTML grammar with a rule of its own for a tag that no rule before it takes, such as `<Foo/>` or `<x:y>`: `meta.tag.other.unrecognized.html.derivative`, where HTML reads an invalid tag.
    /// - That rule applies at the document's top level only; inside a template `<script>`, which the grammar reads as `text.html.basic`, unknown tags are HTML's invalid tags.
    /// - A bare `<` is an illegal angle bracket, as in HTML, but not where another grammar embeds html-derivative, as Vue's templates do, since Shiki applies an embedded grammar's injection nowhere.
    HtmlDerivative {
        feature: "lang-html-derivative",
        names: ["html-derivative"],
        module: html_derivative,
    }
    /// Batch, with the scopes of Shiki's `bat` grammar.
    ///
    /// # Behaviour kept from Shiki
    ///
    /// - Commands, keywords and operators match without regard to case, and `ſ`, the Kelvin sign, `ß` and the Latin ligatures match the letters they fold to.
    /// - A `for` line's `in` clause ends at the end of its line only after `)`, `^` or a blank, so the clause and the `for` rule can stay open on the lines after it.
    ///   The grammar's inline `(?i)` takes its `\n` alternative into the lookbehind before `do`.
    /// - Inside a `for` line, the grammar's injection reads a loop parameter such as `%%i` first, before any other rule and before the end of the rule open there.
    ///   In a Markdown fence it does not, since Shiki applies an embedded grammar's injection nowhere.
    /// - `\s` and `\d` for non-ASCII characters come from the table measured for JSON (`oracle/classes.ts`).
    Bat { feature: "lang-bat", names: ["bat", "batch", "cmd"], module: bat }
    /// Nix, with the scopes of Shiki's `nix` grammar.
    ///
    /// # Behaviour kept from Shiki
    ///
    /// - `then` and `else` are two tokens each, `th` and `en`, `el` and `se`.
    /// - The blank after `...` in a function head belongs to the operator.
    /// - A block comment's first match runs past `**/`, so `/* a **/ b */` is one comment.
    Nix { feature: "lang-nix", names: ["nix"], module: nix }
    /// Dart, with the scopes of Shiki's `dart` grammar.
    ///
    /// # Behaviour kept from Shiki
    ///
    /// - A number followed by a letter ends at its dot: `1.x` is the number `1.` and `x`.
    /// - In `async* {`, `sync*` and `yield*` before a blank, the `*` is an arithmetic operator.
    ///   The keyword takes the `*` only when a word character follows it.
    /// - `is!` before a blank is `is` and the logical operator `!`.
    /// - `&=`, `^=`, `|=`, `<<=`, `>>=` and `>>>=` are a bitwise operator and an assignment.
    ///   The bitwise pattern comes first and takes the operator.
    /// - `get(` and `set(` are function names, not keywords.
    /// - A string left open at the end of a line goes on to the next line.
    /// - A function name needs no word boundary before it, so in `9abc(` the name is `abc`.
    /// - `\w`, `\s` and `\d` for non-ASCII characters come from the table measured for JSON (`oracle/classes.ts`).
    Dart { feature: "lang-dart", names: ["dart"], module: dart }
    /// Julia, with the scopes of Shiki's `julia` grammar.
    ///
    /// # Behaviour kept from Shiki
    ///
    /// - The Python, JavaScript, SQL and R inside `py"""`, `js"""`, `sql"""` and `R"""` string macros are lexed as those languages, up to the macro's closing quotes or a `$` interpolation that the embedded language does not match.
    /// - The end of `py"""` takes the spaces and word characters before its quotes, so a name before `"""` loses its Python scopes.
    /// - The C++ inside a `cxx"""` or `icxx"""` string macro is plain.
    ///   The grammar includes `source.cpp#root_context`, a rule the C++ grammar does not have.
    /// - A `for` head runs to the end of a line that does not end in a comma.
    /// - A type declaration needs a literal `!:_` before `struct`, as the grammar's pattern reads, so in `struct Foo` the name gets no scope.
    /// - The identifier classes and the operator classes for non-ASCII characters come from tables measured on Shiki 4.6.0 (`oracle/julia-grammar.ts`).
    ///   A later Unicode version in Shiki's engine would need them regenerated.
    Julia { feature: "lang-julia", names: ["julia", "jl"], module: julia, reads_line_feed: true }
    /// SCSS, with the scopes of Shiki's `scss` grammar.
    ///
    /// # Behaviour kept from Shiki
    ///
    /// - The rules the grammar takes from CSS run on the CSS lexer's matchers, and keep CSS's scope names.
    /// - A `///` comment is a plain line comment.
    ///   The grammar's doc-comment rule includes only `SassDoc`'s grammar, which Shiki does not bundle, so Shiki drops the rule.
    /// - The whole suffix after a parent selector, as `-b` in `&-b`, is `punctuation.definition.entity.css`.
    /// - In an attribute selector such as `[x="y"]`, the quotes of the value are a string and the text between them, `y`, is not.
    /// - A Markdown fence names the language `scss`.
    Scss { feature: "lang-scss", names: ["scss"], module: scss, halts: true }
    /// Mermaid, with the scopes of Shiki's `mermaid` grammar.
    ///
    /// # Behaviour kept from Shiki
    ///
    /// - A document is plain until a line starts a diagram, such as `graph TD` or `sequenceDiagram`, or opens a ```` ```mermaid ```` or `::: mermaid` block.
    /// - A diagram runs to a line of three or more colons, backticks or tildes, and the next diagram needs such a line before it.
    /// - Most keywords match without regard to case, and `ſ` and the Kelvin sign match `s` and `k`; the words that start a diagram match only as written.
    Mermaid { feature: "lang-mermaid", names: ["mermaid", "mmd"], module: mermaid, reads_line_feed: true }
    /// Haskell, with the scopes of Shiki's `haskell` grammar.
    ///
    /// # Behaviour kept from Shiki
    ///
    /// - A declaration such as `class` or `data` runs on while the lines after it are indented further than it, blank, or only a comment.
    /// - `^` holds only at the start of a line, so a line comment after code never continues a Haddock comment.
    /// - Pragmas match without regard to case, and `ſ`, the Kelvin sign, `ß` and the Latin ligatures match the letters they fold to.
    /// - `\p{Ll}`, `\p{Lt}`, `[\p{Lu}\p{Lt}]` and `[\p{S}\p{P}]` for non-ASCII characters come from tables measured on Shiki's engine (`oracle/classes.ts`).
    Haskell { feature: "lang-haskell", names: ["haskell", "hs"], module: haskell }
    /// Less, with the scopes of Shiki's `less` grammar.
    ///
    /// # Behaviour kept from Shiki
    ///
    /// - A declaration whose property the grammar does not list reads as a selector, as `b: c` in `.a { b: c }`, where `b` is a tag.
    /// - `@viewport` reads as a variable: the grammar's at-rules include `#at-viewport`, which it never defines, and Shiki skips the include.
    /// - Outside a guard, `default()` is the guard function only when a word character follows its `)`; otherwise `default` is a value keyword.
    /// - A Markdown fence names the language `less`.
    Less { feature: "lang-less", names: ["less"], module: less, halts: true }
    /// HAML, with the scopes of Shiki's `haml` grammar, which nishiki's
    /// `TextMate` engine runs. Its filters and Ruby lines go to the Ruby,
    /// JavaScript, CSS, Markdown, Sass and CoffeeScript lexers.
    ///
    /// # Behaviour kept from Shiki
    ///
    /// - A `:scss` filter is plain text. The grammar includes `source.scss`, a scope no Shiki grammar has.
    /// - A `:sass` filter scopes only its own line. Its body lexes as HAML.
    Haml {
        feature: "lang-haml",
        names: ["haml"],
        engine: crate::haml::grammar::GRAMMAR,
        reads_line_feed: true,
        halts: true,
        hands_off: true,
    }
    /// Sass, the indented syntax, with the scopes of Shiki's `sass` grammar.
    ///
    /// # Behaviour kept from Shiki
    ///
    /// - A block comment goes on while each line starts with a blank and the indent of its `/*`.
    ///   An empty line keeps an unindented comment open.
    /// - `@import` runs to the line's end, and the text from ` as` on is marked invalid.
    /// - A unit needs a digit or `}` before it and no word boundary after it, so `1sec` is `1`, `s` and `ec`.
    /// - `\w`, `\s` and `\d` for non-ASCII characters come from the table measured for JSON (`oracle/classes.ts`).
    Sass { feature: "lang-sass", names: ["sass"], module: sass, halts: true }
    /// CoffeeScript, with the scopes of Shiki's `coffee` grammar.
    ///
    /// # Behaviour kept from Shiki
    ///
    /// - The JavaScript between backticks goes to the JavaScript lexer, up to the last backtick of the line.
    ///   It is lexed as if the line ended there.
    /// - A regular expression literal is plain operators and words.
    ///   The grammar's rule for it includes only `source.js.regexp`, which no grammar provides, so Shiki skips the rule.
    CoffeeScript { feature: "lang-coffee", names: ["coffee", "coffeescript"], module: coffee, halts: true }
    /// PHP, with the scopes of Shiki's `php` grammar. HTML, SQL, JavaScript,
    /// JSON, CSS and XML in its strings and heredocs go to their lexers.
    ///
    /// # Behaviour kept from Shiki
    ///
    /// - The grammar is PHP code alone. `<?php` and HTML tags around it read as PHP operators and constants.
    /// - A Markdown `php` fence tries HTML first, then PHP where HTML matches nothing. The text between two tags reads as PHP.
    /// - Without `lang-php`, a Markdown `php` fence is HTML, as Shiki reads it without its PHP grammar.
    /// - A language that embeds PHP starts it at the fence's root, so HTML and PHP mix as in that fence.
    Php { feature: "lang-php", names: ["php"], module: php, halts: true }
    /// Objective-C, with the scopes of Shiki's `objective-c` grammar.
    ///
    /// # Behaviour kept from Shiki
    ///
    /// - A Markdown fence named `h` embeds C, not Objective-C: the grammar's C fence comes first.
    /// - A member name after `.` or `->` that starts with a C type, such as `integer` or `int8_t`, is no member: the grammar excludes the type as a prefix.
    /// - `#define` names its parameters only when `...`, if any, follows the last name: `(a, ...)` leaves the parameters plain.
    /// - Capture groups lexed again inside one another, as member access chains are, stop after 64 levels, and the text of a deeper one keeps its own scopes.
    ///   The bound is a constant in `src/objective_c/engine.rs` until the next minor release moves it into `Limits`.
    ObjectiveC {
        feature: "lang-objective-c",
        names: ["objective-c", "objc"],
        module: objective_c,
        reads_line_feed: true,
        halts: true,
    }
    /// Perl, with the scopes of Shiki's `perl` grammar. HTML, XML, CSS,
    /// JavaScript and SQL in its heredocs go to their lexers, a line at a
    /// time.
    ///
    /// # Behaviour kept from Shiki
    ///
    /// - A heredoc hands each of its lines to the language alone, so an HTML comment or a CSS block cannot span its lines.
    /// - Only the first heredoc a line opens is a heredoc. The rest of the line, with any later `<<`, lexes as Perl.
    /// - `__END__` and `__DATA__` make the rest of the document a comment.
    /// - POD's `C<\s text \s>` form needs a backslash and an `s`, as the grammar writes `\\s` where it means a blank.
    /// - `sub name {` names the space before the brace as part of the declaration.
    /// - Capture groups lexed again inside one another, as a line of heredoc openers is, stop after 64 levels, where Shiki goes on. The text of a deeper one keeps the scopes around it, and the line reports the depth limit.
    ///   The bound is a constant in `src/perl/engine.rs` until the next minor release moves it into `Limits`.
    Perl { feature: "lang-perl", names: ["perl"], module: perl, halts: true }
    /// Elixir, with the scopes of Shiki's `elixir` grammar. HTML in its `~H`
    /// and `~L` sigils goes to HTML's lexer.
    ///
    /// # Behaviour kept from Shiki
    ///
    /// - Shiki bundles no `EEx` grammar, so `<%= %>` tags and `{…}` attribute values in a `~H` or `~L` sigil are HTML text.
    /// - Without `lang-html`, a `~H` or `~L` sigil's text keeps the sigil's scope alone.
    /// - A sigil whose delimiter is the line feed, such as `~s` at the end of a line, runs to the end of the next line.
    Elixir {
        feature: "lang-elixir",
        names: ["elixir"],
        module: elixir,
        reads_line_feed: true,
        halts: true,
    }
    /// Vue, with the scopes of Shiki's `vue` grammar: single-file components
    /// whose blocks hold a template, scripts and styles.
    ///
    /// # Behaviour kept from Shiki
    ///
    /// - The HTML lexer runs Vue.
    ///   Its root holds Vue's comments, block tags, and tags that their line closes with `/>`.
    /// - A block's `lang` attribute picks the lexer of its body: CSS, SCSS, Less, JavaScript, TypeScript, JSX, TSX, YAML or GraphQL.
    /// - A block in a language that nishiki does not lex, such as Pug, JSON or Markdown, is plain text under that language's scope.
    /// - A `<template>` holds html-derivative, with Vue's rules for a nested `<template>` and for a tag whose name starts with a capital.
    /// - Directives such as `v-if`, `:prop` and `@event` count only in Vue's own tags.
    ///   On an HTML tag they read as HTML attributes, and `{{ x }}` is text, because Shiki does not apply Vue's injection grammars.
    /// - A directive's value is a TypeScript expression, which the TypeScript lexer reads.
    /// - A dynamic argument such as `:[key]` is `source.ts.embedded.html.vue` with no tokens inside it.
    /// - In Vue's tags, `generic` and `style` read as HTML attributes.
    /// - A Markdown fence names the language `vue`, through the `markdown-vue` grammar that Vue injects into Markdown.
    ///   Its info string may hold no tilde after `vue`.
    /// - A `vue` fence has no `while` rule.
    ///   It closes only where Vue is outside every rule, so a block left open runs past the closing fence.
    /// - A `vue` fence also opens at the start of a line inside Vue's rules, on their scopes.
    ///   Elsewhere it opens only where Markdown's own fences open.
    ///   Shiki's injection also opens it in another language's fence or in a raw block.
    /// - A `vue` fence opens only at the start of a line, or after a quote's or list's prefix.
    ///   Shiki's injection also opens one where an inner rule's match ended, at its `\G`, such as after a JSX tag's `>`.
    /// - In a Less block, a `)` or `}` at Less's root ends the Less.
    ///   Less's `(;)|(?=[)}])` matches there without advancing, and vscode-textmate then pops the innermost rule, Vue's style block.
    ///   The rest of the block is plain text up to `</style>`.
    Vue { feature: "lang-vue", names: ["vue"], module: vue }
    /// x86 and x86-64 assembly, with the scopes of Shiki's `asm` grammar,
    /// which nishiki's `TextMate` engine runs.
    Asm {
        feature: "lang-asm",
        names: ["asm"],
        engine: crate::asm::grammar::GRAMMAR,
        reads_line_feed: true,
        halts: true,
    }
    /// Visual Basic .NET, with the scopes of Shiki's `vb` grammar.
    ///
    /// # Behaviour kept from Shiki
    ///
    /// - Under `(?i)`, a name may hold `ſ` and the Kelvin sign, and a keyword may hold `ß` and the Latin ligatures, for the letters they fold to.
    /// - A word list takes its first word in order that `\b` follows, so `Else If` is the two keywords `Else` and `If`.
    /// - A string's end comes before `""`, so `"a""b"` is two strings.
    /// - A number gives digits back to a bare `.` when a `\w` follows it, so `1.5é` holds the number `1.`.
    Vb {
        feature: "lang-vb",
        names: ["vb"],
        module: vb,
        reads_line_feed: true,
    }
    /// MATLAB, with the scopes of Shiki's `matlab` grammar.
    ///
    /// # Behaviour kept from Shiki
    ///
    /// - A quote right after a name, `)`, `]` or `}` is a transpose; after a blank or an operator it opens a string, so `[a ' b']` holds a string.
    /// - A line that starts with a name and a blank, such as `hold on`, is a command, whose arguments read as one interpolated string; `ans`, `eps`, `i`, `j`, `inf`, `NaN`, `nargin`, `nargout` and `pi` never start one.
    /// - A file-level `function` with no `end` stays open to the end of the document.
    /// - Capture groups lexed again inside one another stop after 64 levels, and the text of a deeper one keeps its own scopes.
    ///   The bound is a constant in `src/matlab/engine.rs` until the next minor release moves it into `Limits`.
    Matlab {
        feature: "lang-matlab",
        names: ["matlab"],
        module: matlab,
        reads_line_feed: true,
        halts: true,
    }
    /// Pascal, including Delphi and Free Pascal, with the scopes of Shiki's
    /// `pascal` grammar.
    ///
    /// # Behaviour kept from Shiki
    ///
    /// - `read`, `write`, `result` and `file` are always `keyword.pascal`: the keyword list comes first, so a property accessor, the `result` variable and the `file` type never get their own scopes.
    /// - `''` inside a string ends the string and starts another, since a string's end is tried before its escape.
    /// - A `{…}` or `(*…*)` comment ends at its first closer; comments do not nest.
    /// - `{$…}` is a compiler directive, scoped `string.regexp`.
    /// - An `asm` block lexes as Pascal.
    /// - A number's first branch, `\b\d+`, has no `\b` after it, so `12ab` starts with the number `12`; its second, `\d*\.\d+`, has none before it, so `x1.5` holds the number `1.5`.
    /// - Words match without regard to case, and `ſ`, the Kelvin sign, `ß` and the Latin ligatures match the letters they fold to.
    /// - `\w`, `\s` and `\d` for non-ASCII characters come from the table measured for JSON (`oracle/classes.ts`).
    Pascal { feature: "lang-pascal", names: ["pascal"], module: pascal }
    /// GDScript, Godot's scripting language, with the scopes of Shiki's
    /// `gdscript` grammar.
    ///
    /// # Behaviour kept from Shiki
    ///
    /// - A bare node path such as `$Node` does not start right after a `/` and any blanks, as the grammar's `(?<!/\s*)` says, although Oniguruma rarely accepts a lookbehind of varying length.
    /// - A line continuation, `\` at the end of a line, goes on only while the next line opens a string.
    GdScript {
        feature: "lang-gdscript",
        names: ["gdscript", "gd"],
        module: gdscript,
        reads_line_feed: true,
    }
    /// PostCSS, with the scopes of Shiki's `postcss` grammar.
    ///
    /// # Behaviour kept from Shiki
    ///
    /// - A property value takes two scopes, and the first keeps a comma: `meta.property-value.css.postcss,` and `support.constant.property-value.css.postcss`, as vscode-textmate splits the grammar's name at its space.
    /// - An at-rule's name runs to the first blank, unless a media word such as `screen` or `only` and a comma or blank follow it: `@media screen and` names `@media screen`.
    /// - The text inside `url(`, `format(` and `attr(` is a double-quoted string up to the first `)`.
    /// - `//` starts a comment to the end of the line, as in Sass.
    /// - A ` -` before `$` or `(` is an operator, even after a run of blanks.
    /// - `\w`, `\s` and `\d` for non-ASCII characters come from the table measured for JSON (`oracle/classes.ts`).
    PostCss {
        feature: "lang-postcss",
        names: ["postcss"],
        module: postcss,
        halts: true,
    }
    /// Common Lisp, with the scopes of Shiki's `common-lisp` grammar.
    ///
    /// # Behaviour kept from Shiki
    ///
    /// - Words match without regard to case, and `ſ`, the Kelvin sign, `ß` and the Latin ligatures match the letters they fold to.
    /// - A word is scoped only when a `(`, a `)` or a blank follows it, and most only after a `(`, a blank or the start of the line.
    /// - `(defun (foo) …)` names `o`: the list form's head gives back its last character when no name follows it.
    /// - A `'` parameter of a `format` directive takes the next character, a `"` too, so `"~'"a"` holds the directive `~'"a` and the string goes on.
    /// - `#|…|#` comments nest.
    /// - `\s` for non-ASCII characters comes from the table measured for JSON (`oracle/classes.ts`).
    CommonLisp {
        feature: "lang-common-lisp",
        names: ["common-lisp", "lisp"],
        module: common_lisp,
        reads_line_feed: true,
    }
    /// Objective-C++, with the scopes of Shiki's `objective-cpp` grammar.
    /// It runs Objective-C's matchers for the expressions the two grammars
    /// share, as [`Language::ObjectiveC`] does.
    ///
    /// # Behaviour kept from Shiki
    ///
    /// - A `namespace` block that a bracket ends, as in `namespace a)`, names that bracket `keyword.control.namespace.$2.objcpp`: the name's `$2` reads a group the end's expression lacks, and stays as written.
    /// - The grammar's C++ constructor rule never matches, so `Foo(int x)` inside a `namespace` block reads as a function definition: the rule's only patterns include `function-innards` and `probably_a_parameter`, which no repository in its reach defines, and vscode-textmate drops such a rule.
    /// - Capture groups lexed again inside one another, as member access chains are, stop after 64 levels, and the text of a deeper one keeps its own scopes.
    ///   The bound is a constant in `src/objective_cpp/engine.rs` until the next minor release moves it into `Limits`.
    ObjectiveCpp {
        feature: "lang-objective-cpp",
        names: ["objective-cpp"],
        module: objective_cpp,
        reads_line_feed: true,
        halts: true,
    }
    /// Svelte, with the scopes of Shiki's `svelte` grammar.
    /// Its expressions and `<script>` bodies are lexed by the TypeScript and
    /// JavaScript lexers, and its `<style>` bodies by the CSS, PostCSS, SCSS,
    /// Less and Sass lexers.
    /// A `lang="coffee"` body goes to the CoffeeScript lexer when `lang-coffee` is on.
    /// An `@component` doc comment goes to the Markdown lexer when `lang-markdown` is on.
    /// Without its feature, each is plain text.
    ///
    /// # Behaviour kept from Shiki
    ///
    /// - A `lang="stylus"` or `pug` body is plain text under `meta.lang.<lang>.svelte`, as nishiki lexes neither language.
    /// - A special tag's mode is picked by a word anywhere on the line before the tag's keyword ends.
    ///   In `{#if a}{:else b}`, the `if` puts `{:else}` in the mode that lexes `b` as TypeScript.
    ///   A `{:else b}` alone leaves `b` plain.
    /// - TypeScript lexes a `{@debug}` name and an `{#each}` index again, each on its own, so a keyword there, such as `else`, reads as a keyword.
    /// - The text of an `@component` comment's line is Markdown only where it starts the line, since Markdown's block patterns need `^` or `\G`.
    /// - `\w`, `\s` and `[[:alpha:]]` for non-ASCII characters come from the table measured for JSON (`oracle/classes.ts`).
    /// - In a Less body, a `)` or `}` at Less's root ends the Less.
    ///   Less's `(;)|(?=[)}])` matches there without advancing, and vscode-textmate then pops the innermost rule, Svelte's Less body.
    ///   The rest of the body is plain text up to `</style>`.
    ///
    /// # Differences from Shiki
    ///
    /// - Svelte's `$` prefixes reach only the TypeScript and JavaScript that Svelte embeds itself.
    ///   Shiki's injection adds them in every `source.ts` and `source.js` of the document.
    ///   One example is a script in the HTML of an `@component` comment's Markdown.
    Svelte { feature: "lang-svelte", names: ["svelte"], module: svelte }
    /// R, with the scopes of Shiki's `r` grammar.
    ///
    /// # Behaviour kept from Shiki
    ///
    /// - A fence opened in a roxygen comment, such as `` #' ``` ``, stays open past the comment's line.
    ///   Until a run of as many backticks closes it, anywhere, the lines are plain text, apart from a leading `#'`.
    ///   So R code after a fence that the comment never closes is not highlighted.
    /// - After `#' @examples`, the lines are R, with each leading `#'` a comment.
    ///   The block ends at a line that does not start with `#'`, or that goes on with a tag such as `#' @export`.
    ///   An indented `#'` line ends it too, one blank before the `#`.
    /// - `.5` is a name, not a number, because names come first and may start with a dot.
    /// - `return`, `library`, `stop` and the other built-in functions are `keyword.other` only before a `(`.
    /// - `%` starts an operator that runs to the next `%` on the line, such as `%in%` or `%>%`.
    /// - A number's exponent may have no digits, so `1e` and `1e+` are numbers.
    /// - `\w` and `\d` for non-ASCII characters come from the tables measured on Shiki's engine, and so do the `\p{Nl}`, `[\p{Mn}\p{Mc}]` and `\p{Pc}` of names (`oracle/classes.ts`).
    /// - A Markdown fence names the language `r`, `s` or `rprofile`, in any case, or `{.r …}`.
    R { feature: "lang-r", names: ["r"], module: r, halts: true }
    /// Jinja, with the scopes of Shiki's `jinja` grammar: its tags,
    /// expressions and comments over plain text.
    ///
    /// # Behaviour kept from Shiki
    ///
    /// - The text between tags is plain, as the grammar reads it: Shiki's `jinja` is the only Jinja language a user can name.
    /// - A name, `[A-Z_a-z][0-9A-Z_a-z]*`, needs no `\b` before it, so `1abc` holds the name `abc`.
    /// - The name after `{%` or `{%-` that no `,` or `=` follows is the tag's keyword, as `endfor` is.
    Jinja { feature: "lang-jinja", names: ["jinja"], module: jinja }
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
///
/// A lexer that embeds another passes it limits whose work is what the line
/// has left, and adds what it spent, which comes back from its `lex`, so the
/// embedded language spends the line's one work budget
/// (`CODING_STANDARDS.md` 1.6).
pub(crate) struct Context<'a> {
    pub(crate) scratch: &'a mut Scratch,
    pub(crate) arena: &'a mut Arena,
    pub(crate) limits: &'a Limits,
    pub(crate) out: &'a mut Vec<Token>,
    pub(crate) report: &'a mut LineReport,
}

/// A [`Context`] without its scratch, for a lexer that keeps its buffers in
/// one field of the scratch and borrows that field beside these.
pub(crate) struct Parts<'a> {
    pub(crate) arena: &'a mut Arena,
    pub(crate) limits: &'a Limits,
    pub(crate) out: &'a mut Vec<Token>,
    pub(crate) report: &'a mut LineReport,
}

/// Where a line that another language embeds stopped, and why
/// ([`Language::lex_until`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Stopped {
    /// The byte the line stopped at.
    pub(crate) at: usize,
    pub(crate) kind: StopKind,
}

/// Why a line that another language embeds stopped ([`Stopped`]).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) enum StopKind {
    /// A halt of the language that embeds it held.
    /// That language goes on from the byte.
    #[default]
    Halt,
    /// At its root, the language matched an empty text without advancing.
    /// vscode-textmate then pops the innermost rule, which there is the rule
    /// that embeds the language, and ends the line.
    /// The language has lexed the rest of the line as its own, from the byte.
    PopHost,
}

/// Cuts the tokens in `out` back to `at`, for a host that pops the rule that
/// embeds a language ([`StopKind::PopHost`]).
/// The language lexed the rest of the line from `at` as its own.
#[cfg(any(feature = "_embedding", feature = "lang-vue"))]
pub(crate) fn cut_tokens(out: &mut Vec<Token>, at: usize) {
    let keep = out.partition_point(|token| token.end <= at);
    let start = keep.checked_sub(1).map_or(0, |last| out[last].end);
    match out.get_mut(keep) {
        Some(token) if start < at => {
            token.end = at;
            out.truncate(keep + 1);
        }
        Some(_) | None => out.truncate(keep),
    }
}

impl Stopped {
    /// A stop at `at` where a halt held.
    pub(crate) const fn halt(at: usize) -> Stopped {
        Stopped {
            at,
            kind: StopKind::Halt,
        }
    }
}

/// The end of the rule that embeds a language, as a test of the line's bytes
/// at a place: a `TextMate` engine tries that end at each position while no
/// rule of the embedded grammar is open, before the embedded patterns, so a
/// match that starts before it still wins and one that starts at it loses.
/// It never holds at a space or tab, because the rule engine's scan skips
/// runs of them.
///
/// Within one call of a lexer's `lex_until`, `s` is the whole line.
/// It holds the same bytes on every call.
/// Places count from the line's start.
/// A halt may keep context, such as the level of a long bracket.
/// Its answer still depends only on the bytes and the place.
/// A function `fn(&[u8], usize) -> bool` is a halt without context.
pub(crate) trait Halt {
    /// The first character start in `from..=to` where the halt holds.
    fn first(&self, s: &[u8], from: usize, to: usize) -> Option<usize>;

    /// Whether the halt holds at `p`.
    fn holds(&self, s: &[u8], p: usize) -> bool {
        self.first(s, p, p) == Some(p)
    }
}

/// The halts of a line that another language embeds.
/// The first is the embedding rule's end.
/// A lexer tries it before the embedded patterns at each place.
/// The second stands for the patterns the embedding rule lists after the
/// embedded grammar. A lexer tries it where none of the embedded patterns
/// match. A lexer keeps one reference to the caller's pair.
pub(crate) type Halts<'a> = (&'a dyn Halt, Option<&'a dyn Halt>);

/// The halts of a scan compiled without halts.
/// They never hold.
pub(crate) const NO_HALTS: Halts<'static> = (&never, None);

/// A halt that never holds.
fn never(_: &[u8], _: usize) -> bool {
    false
}

impl std::fmt::Debug for dyn Halt + '_ {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Halt")
    }
}

impl<F: Fn(&[u8], usize) -> bool> Halt for F {
    fn first(&self, s: &[u8], from: usize, to: usize) -> Option<usize> {
        let mut p = from;
        while p <= to {
            if self(s, p) {
                return Some(p);
            }
            p += 1;
            while p < to && s.get(p).is_some_and(|&b| b & 0xc0 == 0x80) {
                p += 1;
            }
        }
        None
    }

    #[inline]
    fn holds(&self, s: &[u8], p: usize) -> bool {
        self(s, p)
    }
}

/// The first character start of `s` from `from` where a language that
/// embeds another takes the line back before the embedded language's next
/// match, which starts at `to`: `end` up to and at `to`, since `end` is tried
/// before the patterns, and `after` only before it, since a match at a place
/// beats `after` there. With no match ahead, `to` is `lim`, the end of the
/// text, which is tested too. Returns the place, and how many bytes the scan
/// passed, for the caller's work budget.
///
/// `#[inline]` because the HTML and XML lexers, in other modules, call it
/// (`bench/AGENTS.md` rule 13).
#[cfg(feature = "_utf8")]
#[inline]
pub(crate) fn halt_before(
    s: &[u8],
    from: usize,
    to: usize,
    lim: usize,
    end: &dyn Halt,
    after: Option<&dyn Halt>,
) -> (Option<usize>, usize) {
    if from > to {
        return (None, 0);
    }
    let at_end = end.first(s, from, to);
    let at_after = after
        .and_then(|after| after.first(s, from, at_end.unwrap_or(to)))
        .filter(|&p| p < to);
    let halted = match (at_end, at_after) {
        (Some(a), Some(b)) => Some(a.min(b)),
        (one, None) | (None, one) => one,
    };
    let passed = match halted {
        Some(p) => p,
        None if to < lim => to + crate::utf8::char_len(s[to]),
        None => lim,
    };
    (halted, passed - from)
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
    /// The context split into its scratch and the rest.
    #[inline]
    pub(crate) fn parts(&mut self) -> (Parts<'_>, &mut Scratch) {
        let parts = Parts {
            arena: &mut *self.arena,
            limits: self.limits,
            out: &mut *self.out,
            report: &mut *self.report,
        };
        (parts, &mut *self.scratch)
    }

    /// Runs `lex` on the line's text from its start, for a lexer that counts
    /// token ends from there, then moves the new tokens' ends to count from
    /// the start of the whole line, as [`Language::lex`] promises.
    /// A start past the text lexes an empty rest, so no caller can make it
    /// panic.
    /// That rest comes from [`past_end`], out of line, so the common path
    /// keeps the shape a slice had: with the rest inline, Rust's lexer ran
    /// 0.29% more instructions at one codegen unit.
    ///
    /// Each caller passes a closure of its own type.
    /// Each instance then has one caller, so forcing it inline copies nothing.
    #[expect(
        clippy::inline_always,
        reason = "measured at one codegen unit: in Markdown's arm of the dispatch it stayed a call, at 1.0039 of main; forced, 0.9993"
    )]
    #[inline(always)]
    pub(crate) fn lex_from_start(
        &mut self,
        line: Line<'_>,
        lex: impl FnOnce(&mut Self, &str) -> Finished,
    ) -> Finished {
        let first = self.out.len();
        let next = lex(
            self,
            line.text.get(line.start..).unwrap_or_else(|| past_end()),
        );
        if line.start > 0 {
            for token in &mut self.out[first..] {
                token.end += line.start;
            }
        }
        next
    }
}

/// The rest of a line whose start lies past its text: nothing.
/// Cold and out of line, so [`Context::lex_from_start`] keeps the shape a
/// slice had.
#[cold]
#[inline(never)]
fn past_end() -> &'static str {
    ""
}

#[cfg(test)]
mod tests {
    use super::Halt;

    /// A halt with context.
    /// It finds the close of a long bracket with `level` `=` signs, as Lua's
    /// `]==]` is.
    struct Close {
        level: usize,
    }

    impl Halt for Close {
        fn first(&self, s: &[u8], from: usize, to: usize) -> Option<usize> {
            (from..=to).find(|&p| {
                s.get(p) == Some(&b']')
                    && s.get(p + 1..p + 1 + self.level)
                        .is_some_and(|run| run.iter().all(|&b| b == b'='))
                    && s.get(p + 1 + self.level) == Some(&b']')
            })
        }
    }

    /// A halt without context.
    /// It holds at a `<`.
    fn at_lt(s: &[u8], p: usize) -> bool {
        s.get(p) == Some(&b'<')
    }

    /// Where `halt` first holds in `s`.
    /// Both kinds of halt pass through this one `&dyn Halt` parameter.
    fn first_of(halt: &dyn Halt, s: &[u8]) -> Option<usize> {
        halt.first(s, 0, s.len())
    }

    #[test]
    fn a_function_and_a_halt_with_context_pass_as_one_type() {
        let s = b"a[b[1]] < ]=] ]]";
        assert_eq!(first_of(&at_lt, s), Some(8));
        assert_eq!(first_of(&Close { level: 1 }, s), Some(10));
        assert_eq!(first_of(&Close { level: 0 }, s), Some(5));
        assert!(Close { level: 1 }.holds(s, 10));
        assert!(!at_lt.holds(s, 0));
    }
}
