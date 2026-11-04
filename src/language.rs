//! The languages nishiki highlights: one table of rows, from which the
//! `Language` enum and every function that has an arm per language are
//! generated.

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
    (@start [$module:ident] [] [$($dialect:expr)?] $arena:ident $base:ident) => {
        crate::$module::start($arena, $base $(, $dialect)?)
    };
    (@start [] [$grammar:path] [] $arena:ident $base:ident) => {
        crate::textmate::start($arena, &$grammar, $base)
    };
    (@start_embedded [$module:ident] [] $arena:ident $base:ident) => {
        crate::$module::start_embedded($arena, $base)
    };
    (@start_embedded [] [$grammar:path] $arena:ident $base:ident) => {
        crate::textmate::start_embedded($arena, &$grammar, $base)
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
    (@embedded [] [$module:ident] [] [$($dialect:expr)?] [] $cx:ident $line:ident $state:ident) => {
        crate::$module::lex($cx, $($dialect,)? $line, $state)
    };
    (@embedded [true] [$module:ident] [] [] [] $cx:ident $line:ident $state:ident) => {
        $cx.lex_from_start($line, |cx, rest| crate::$module::line(cx, rest, $state))
    };
    (@embedded [] [] [$grammar:path] [] [] $cx:ident $line:ident $state:ident) => {
        crate::textmate::lex_embedded($cx, &$grammar, $line, $state)
    };
    (@embedded [] [] [$grammar:path] [] [true] $cx:ident $line:ident $state:ident) => {
        crate::textmate::lex_handing_embedded($cx, &$grammar, $line, $state)
    };
    (@until [] [$($module:ident)?] [$($grammar:path)?] [$($dialect:expr)?] [] $cx:ident $line:ident $state:ident $end:ident $after:ident) => {{
        let _ = ($cx, $line, $state, $end, $after);
        None
    }};
    (@until [true] [$module:ident] [] [$($dialect:expr)?] [] $cx:ident $line:ident $state:ident $end:ident $after:ident) => {
        Some(crate::$module::lex_until($cx, $($dialect,)? $line, $state, $end, $after))
    };
    (@until [true] [] [$grammar:path] [] [] $cx:ident $line:ident $state:ident $end:ident $after:ident) => {
        Some(crate::textmate::lex_until($cx, &$grammar, $line, $state, $end, $after))
    };
    (@until [true] [] [$grammar:path] [] [true] $cx:ident $line:ident $state:ident $end:ident $after:ident) => {
        Some(crate::textmate::lex_handing_until($cx, &$grammar, $line, $state, $end, $after))
    };
    ($(
        $(#[$meta:meta])*
        $variant:ident {
            feature: $feature:literal,
            names: [$id:literal $(, $alias:literal)*],
            $(module: $module:ident)?
            $(engine: $grammar:path)?
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
            pub(crate) fn reads_line_feed(self) -> bool {
                match self {
                    $(#[cfg(feature = $feature)] Language::$variant => languages!(@flag $($line_feed)?),)*
                }
            }

            /// The state at the start of a document whose scopes sit on
            /// `base`.
            pub(crate) fn start(self, arena: &mut Arena, base: u32) -> u32 {
                match self {
                    $(#[cfg(feature = $feature)] Language::$variant => languages!(@start [$($module)?] [$($grammar)?] [$($dialect)?] arena base),)*
                }
            }

            /// The state at the start of a Markdown fence's body, whose text
            /// sits on `base` with no root scope of the language's own, as in
            /// Shiki's embedding.
            pub(crate) fn start_embedded(self, arena: &mut Arena, base: u32) -> u32 {
                match self {
                    $(#[cfg(feature = $feature)] Language::$variant => languages!(@start_embedded [$($module)?] [$($grammar)?] arena base),)*
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
                    $(#[cfg(feature = $feature)] Language::$variant => languages!(@embedded [$($from_start)?] [$($module)?] [$($grammar)?] [$($dialect)?] [$($hands_off)?] cx line state),)*
                }
            }

            /// Lexes `line` like [`Language::lex`], for a language that
            /// another one embeds: the line halts at the first place where
            /// `end` holds that the code reaches with only the language's
            /// root open, or where `after` holds and nothing of the language
            /// matches, and the second value is where. `None` for a
            /// language that cannot halt.
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
            ) -> Option<(Finished, Option<usize>)> {
                match self {
                    $(#[cfg(feature = $feature)] Language::$variant => languages!(@until [$($halts)?] [$($module)?] [$($grammar)?] [$($dialect)?] [$($hands_off)?] cx line state end after),)*
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
    Rust { feature: "lang-rust", names: ["rust", "rs"], module: rust }
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
    Python { feature: "lang-python", names: ["python", "py"], module: python }
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
    /// PowerShell, with the scopes of Shiki's `powershell` grammar, which
    /// nishiki's `TextMate` engine runs.
    PowerShell {
        feature: "lang-powershell",
        names: ["powershell", "ps", "ps1", "pwsh"],
        engine: crate::powershell::grammar::GRAMMAR,
        reads_line_feed: true,
        halts: true,
    }
    /// Scala, with the scopes of Shiki's `scala` grammar, which nishiki's
    /// `TextMate` engine runs.
    Scala {
        feature: "lang-scala",
        names: ["scala"],
        engine: crate::scala::grammar::GRAMMAR,
        reads_line_feed: true,
        halts: true,
    }
    /// HTML, with the scopes of Shiki's `html` grammar.
    ///
    /// # Behaviour kept from Shiki
    ///
    /// - A `<` that no rule claims is an illegal angle bracket, even inside a quoted attribute value, as the grammar's injection makes it.
    /// - An attribute's name is a word of the grammar's lists, or any other run of name characters, which gets a scope built from its text, such as `meta.attribute.unrecognized.fooBar.html`.
    /// - `svg` and `math` elements have rules of their own, and read the attributes of a start tag that the line holds whole on that text alone.
    /// - The CSS of a `<style>` element is lexed by the CSS lexer, and the JavaScript of a `<script>` element and of an event handler by the JavaScript lexer.
    ///   The code ends at its end tag, in any case, only where it has no rule open, so a `</style>` inside an open CSS block is CSS.
    ///   The grammar's own `//` and `/* */` rules in a script end at a lower-case `</script` only.
    /// - A `<script>` whose `type` is an HTML template holds HTML, and one of any other type that is not JavaScript is `source.unknown`.
    /// - A line stops after `MAX_STALLED` (4,096) steps in a row that do not advance, and the rest of it keeps the innermost rule's scope.
    ///   At one byte a run of such steps opens and closes each rule of the grammar at most once, because a rule that repeats stops the line, so no line comes near it.
    ///   The bound is a constant in `src/html/engine.rs` until the next minor release moves it into `Limits`.
    /// - A Markdown fence names the languages `html`, `htm`, `shtml`, `xhtml`, `inc`, `tmpl` and `tpl`.
    Html { feature: "lang-html", names: ["html"], module: html, halts: true }
    /// Zig, with the scopes of Shiki's `zig` grammar, which nishiki's
    /// `TextMate` engine runs.
    Zig {
        feature: "lang-zig",
        names: ["zig"],
        engine: crate::zig::grammar::GRAMMAR,
        reads_line_feed: true,
        halts: true,
    }
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
    /// CMake, with the scopes of Shiki's `cmake` grammar, which nishiki's
    /// `TextMate` engine runs.
    CMake {
        feature: "lang-cmake",
        names: ["cmake"],
        engine: crate::cmake::grammar::GRAMMAR,
        reads_line_feed: true,
        halts: true,
    }
    /// Protobuf, with the scopes of Shiki's `proto` grammar, which nishiki's
    /// `TextMate` engine runs.
    Proto {
        feature: "lang-proto",
        names: ["proto", "protobuf"],
        engine: crate::proto::grammar::GRAMMAR,
        reads_line_feed: true,
        halts: true,
    }
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
    /// Batch, with the scopes of Shiki's `bat` grammar, which nishiki's
    /// `TextMate` engine runs.
    Bat {
        feature: "lang-bat",
        names: ["bat", "batch", "cmd"],
        engine: crate::bat::grammar::GRAMMAR,
        reads_line_feed: true,
        halts: true,
    }
    /// Nix, with the scopes of Shiki's `nix` grammar, which nishiki's
    /// `TextMate` engine runs.
    Nix {
        feature: "lang-nix",
        names: ["nix"],
        engine: crate::nix::grammar::GRAMMAR,
        reads_line_feed: true,
        halts: true,
    }
    /// Dart, with the scopes of Shiki's `dart` grammar, which nishiki's
    /// `TextMate` engine runs.
    Dart {
        feature: "lang-dart",
        names: ["dart"],
        engine: crate::dart::grammar::GRAMMAR,
        reads_line_feed: true,
        halts: true,
    }
    /// Julia, with the scopes of Shiki's `julia` grammar, which nishiki's
    /// `TextMate` engine runs.
    ///
    /// # Behaviour kept from Shiki
    ///
    /// - The C++ inside a `cxx"""` or `icxx"""` string macro and the R inside an `R"""` one are plain, where Shiki lexes them as C++ and R.
    ///   Julia's table leaves out the C++ and R grammars, which Julia embeds: R's may not be bundled, and C++'s brings GLSL's, which may not either.
    ///   The macro and its quotes keep Shiki's scopes, and so does `$` interpolation inside.
    /// - The Python, JavaScript and SQL inside `py"""`, `js"""` and `sql"""` string macros are lexed as Shiki lexes them, by engine copies of those grammars in Julia's table.
    Julia {
        feature: "lang-julia",
        names: ["julia", "jl"],
        engine: crate::julia::grammar::GRAMMAR,
        reads_line_feed: true,
        halts: true,
    }
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
    Scss { feature: "lang-scss", names: ["scss"], module: scss }
    /// Mermaid, with the scopes of Shiki's `mermaid` grammar, which nishiki's
    /// `TextMate` engine runs.
    Mermaid {
        feature: "lang-mermaid",
        names: ["mermaid", "mmd"],
        engine: crate::mermaid::grammar::GRAMMAR,
        reads_line_feed: true,
        halts: true,
    }
    /// Haskell, with the scopes of Shiki's `haskell` grammar, which nishiki's
    /// `TextMate` engine runs.
    Haskell {
        feature: "lang-haskell",
        names: ["haskell", "hs"],
        engine: crate::haskell::grammar::GRAMMAR,
        reads_line_feed: true,
        halts: true,
    }
    /// Less, with the scopes of Shiki's `less` grammar.
    ///
    /// # Behaviour kept from Shiki
    ///
    /// - A declaration whose property the grammar does not list reads as a selector, as `b: c` in `.a { b: c }`, where `b` is a tag.
    /// - `@viewport` reads as a variable: the grammar's at-rules include `#at-viewport`, which it never defines, and Shiki skips the include.
    /// - Outside a guard, `default()` is the guard function only when a word character follows its `)`; otherwise `default` is a value keyword.
    /// - A Markdown fence names the language `less`.
    Less { feature: "lang-less", names: ["less"], module: less }
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
    /// Sass, the indented syntax, with the scopes of Shiki's `sass` grammar,
    /// which nishiki's `TextMate` engine runs.
    Sass {
        feature: "lang-sass",
        names: ["sass"],
        engine: crate::sass::grammar::GRAMMAR,
        reads_line_feed: true,
        halts: true,
    }
    /// CoffeeScript, with the scopes of Shiki's `coffee` grammar, which
    /// nishiki's `TextMate` engine runs. Its embedded JavaScript goes to the
    /// JavaScript lexer.
    CoffeeScript {
        feature: "lang-coffee",
        names: ["coffee", "coffeescript"],
        engine: crate::coffee::grammar::GRAMMAR,
        reads_line_feed: true,
        halts: true,
        hands_off: true,
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
        let next = lex(self, &line.text[line.start..]);
        if line.start > 0 {
            for token in &mut self.out[first..] {
                token.end += line.start;
            }
        }
        next
    }
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
