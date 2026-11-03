# nishiki

A syntax highlighting engine in Rust, written from scratch.

nishiki aims to be:

- **fast and small;**
- **faithful to Shiki**, colouring code as Shiki does;
- **self-contained**, with every supported language built in and nothing downloaded at run time.

The name comes from nishiki-e (錦絵, "brocade picture"),
the multi-colour Japanese woodblock prints of the 1760s.

## Usage

```sh
cargo add nishiki
```

A `Highlighter` lexes one language, one line at a time.

| Piece | What it holds |
|:--|:--|
| Token | The byte offset where it ends, and the TextMate scope stack Shiki gives it. |
| `State` | Where the lexer stands after a line. It is the input of the next line. |

Because the state passes from line to line:

- an editor can re-highlight from any line;
- a streamed reply can be highlighted as it arrives.

```rust
use nishiki::{Highlighter, Language, State};

let mut highlighter = Highlighter::new(Language::Rust);
let mut state = State::default();
let mut tokens = Vec::new();
let line = "let x = 42;";
let report = highlighter.line(line, &mut state, &mut tokens);
assert!(report.is_complete());

let mut start = 0;
for token in &tokens {
    let mut scopes: Vec<&str> = highlighter.scopes(token.stack()).collect();
    scopes.reverse();
    println!("{:?} {}", &line[start..token.end()], scopes.join(" "));
    start = token.end();
}
```

`scopes` lists a stack innermost first,
the order theme rules match it in.

## Themes

A `Theme` is a TextMate theme in Shiki's format.
`Theme::style` gives a scope stack the colour and font style Shiki would.

| Constructor | Feature | Takes |
|:--|:--|:--|
| `Theme::from_json` | `json` | A theme file, as `@shikijs/themes` ships it. |
| `Theme::new` | none | The rules as values. nishiki then depends on `memchr` alone. |

```rust
use nishiki::{Highlighter, Language, RuleScope, State, Theme, ThemeRule, ThemeSource};

let theme = Theme::new(ThemeSource {
    rules: vec![ThemeRule {
        scope: RuleScope::Selectors("storage.type".into()),
        foreground: Some("#f97583".into()),
        ..ThemeRule::default()
    }],
    ..ThemeSource::default()
});
let mut highlighter = Highlighter::new(Language::Rust);
let mut tokens = Vec::new();
let _ = highlighter.line("let x = 42;", &mut State::default(), &mut tokens);
let style = theme.style(highlighter.scopes(tokens[0].stack()));
assert_eq!(style.foreground.to_string(), "#F97583");
```

## Limits

`Limits` bounds every line:

- its length in bytes;
- the lexer steps it may take;
- the contexts open at once;
- the scope stacks and states a highlighter interns.

How they behave:

- Each limit can be set, raised, lowered or turned off.
- `Limits::for_budget` derives them from a memory budget and a per-line time budget.
- Limits count bytes and steps, never time.
  The same input and limits give the same tokens on every machine.
- Each line's `LineReport` names the limits it reached.

```rust
use nishiki::{Highlighter, Language, Limits};

let limits = Limits { depth: None, ..Limits::default() };
let highlighter = Highlighter::new_with_limits(Language::Rust, limits);
assert_eq!(highlighter.limits().depth, None);
```

## Languages

| Language | Variant | Feature | Names `Language::from_name` takes |
|:--|:--|:--|:--|
| Rust | `Language::Rust` | `lang-rust` | `rust`, `rs` |
| Markdown | `Language::Markdown` | `lang-markdown` | `markdown`, `md` |
| TypeScript | `Language::TypeScript` | `lang-typescript` | `typescript`, `ts`, `cts`, `mts` |
| JavaScript | `Language::JavaScript` | `lang-javascript` | `javascript`, `js`, `cjs`, `mjs` |
| JSX | `Language::Jsx` | `lang-jsx` | `jsx` |
| TSX | `Language::Tsx` | `lang-tsx` | `tsx` |
| Shell | `Language::Shell` | `lang-shell` | `shellscript`, `bash`, `sh`, `shell`, `zsh` |
| JSON | `Language::Json` | `lang-json` | `json` |
| JSON with comments | `Language::Jsonc` | `lang-json` | `jsonc` |
| JSON Lines | `Language::Jsonl` | `lang-json` | `jsonl` |
| Python | `Language::Python` | `lang-python` | `python`, `py` |
| YAML | `Language::Yaml` | `lang-yaml` | `yaml`, `yml` |
| TOML | `Language::Toml` | `lang-toml` | `toml` |
| Makefile | `Language::Make` | `lang-make` | `make`, `makefile` |
| Diff | `Language::Diff` | `lang-diff` | `diff` |
| Dockerfile | `Language::Dockerfile` | `lang-dockerfile` | `docker`, `dockerfile` |
| Java | `Language::Java` | `lang-java` | `java` |
| C | `Language::C` | `lang-c` | `c` |
| SQL | `Language::Sql` | `lang-sql` | `sql` |
| INI | `Language::Ini` | `lang-ini` | `ini`, `properties` |
| GraphQL | `Language::GraphQl` | `lang-graphql` | `graphql`, `gql` |
| CSS | `Language::Css` | `lang-css` | `css` |
| Kotlin | `Language::Kotlin` | `lang-kotlin` | `kotlin`, `kt`, `kts` |
| Go | `Language::Go` | `lang-go` | `go`, `golang` |
| Ruby | `Language::Ruby` | `lang-ruby` | `ruby`, `rb` |
| C# | `Language::CSharp` | `lang-csharp` | `csharp`, `c#`, `cs` |
| XML | `Language::Xml` | `lang-xml` | `xml` |

### Features

Every language is on by default.
To build only some, turn off the default features and name what you highlight:

```toml
nishiki = { version = "0.1", default-features = false, features = ["lang-rust", "lang-markdown"] }
```

- Each language is a `lang-*` feature, named in the table above.
  One module lexes TypeScript, JavaScript, JSX and TSX, so each of the four is its own feature and a build pays for the copies it turns on.
- A language that embeds another needs it: `lang-graphql` turns on `lang-javascript`, and `lang-xml` turns on `lang-java`.
- Tiers turn on several at once, and overlap:

  | Feature | Languages |
  |:--|:--|
  | `all` (the default) | Every language |
  | `config` | JSON, TOML, YAML, INI, Diff, Dockerfile, Makefile, XML |
  | `web` | TypeScript, JavaScript, JSX, TSX, CSS, JSON, GraphQL |
  | `systems` | Rust, C, Shell, Java, Kotlin, Go |
  | `docs` | Markdown and `config` |

- `json` reads theme files with `Theme::from_json`. It is not the JSON language, which is `lang-json`.
- `Language::ALL` and `Language::from_name` know only the languages that are built.
  A Markdown fence of a language that is off is plain text, as in Shiki when that grammar is not loaded.
- Nothing is downloaded at run time; every language you turn on is in the binary.

### Inside Markdown

- A fence is lexed as one of these languages when its info string names it the way Shiki's Markdown grammar does.
- Any other fence is plain text, as in Shiki when only these languages are loaded.
  That includes `toml` and `kotlin`: Shiki's Markdown grammar has no rule that embeds them.
- YAML front matter is lexed as YAML.

## Status

- The API is unstable while the version is `0.x`.
- A theme colour that is not hex, such as a CSS variable, sets no colour.
  Shiki passes it through.
- The scopes match Shiki's, quirks included.
  The next section lists the quirks per language.

## Behaviour kept from Shiki

### Rust

- `||` is two `|` operators.
- `a!=b` reads `a!` as a macro.
- A camel-case name such as `fooBar` gets no scope.
- Every non-ASCII byte counts as a word character.

### Markdown

- An empty line leaves the state unchanged.
  A line of spaces does not.
- HTML blocks other than comments are plain text.
- A label or link destination nested deeper than 32 brackets ends there.
- The grammar's backtracking is followed where it matters in practice, not everywhere.
  A code span gives back backticks,
  but an escaped `>` does not end an angle-bracket destination.
- A fence of more than 255 characters is never closed.

### TypeScript, JavaScript, JSX and TSX

- `Number.MAX_VALUE` gets no scope.
- An empty line leaves the state as it is.
- JavaScript, JSX and TSX are one grammar with three spellings of its scope names.
- They differ from TypeScript in two places:
  - they read `<` as a JSX tag;
  - they change the generic arrow function lookaheads that tags would confuse.

### Shell

- An unterminated string or here-document keeps every later line.
- `||` outside a statement is two `|` operators.
- An argument such as `ab>c` is split as `a` and `b>c`.
- An empty line leaves the state as it is, because Shiki does not tokenize empty lines.
- A non-ASCII character is a word character when it is alphanumeric, so combining marks are not.
- A non-ASCII character is white space when Unicode says so.

### JSON

- Every dialect's comments are read.
- A stray character in an array or object gets the grammar's `invalid.illegal` scope.
- `\d`, `\w` and `\s` for non-ASCII characters come from a table measured on Shiki 4.6.0 (`oracle/classes.ts`).
  A later Unicode version in Shiki's engine would need the table regenerated.

### Python

- `def` opens a function after the end of any word, as in `x def f(): pass`.
- An empty line leaves the state as it is.
- `\d`, `\w`, `\s`, `[[:alpha:]]` and `\p{upper}` for non-ASCII characters come from tables measured on Shiki 4.6.0 (`oracle/classes.ts`).
  A later Unicode version in Shiki's engine would need them regenerated.

### YAML

- The lexer follows the YAML 1.1 type patterns that Shiki's grammar uses:
  - `0o17` is plain text;
  - an exponent needs its sign;
  - `y` and `n` are booleans, as keys too.
- Each line is read on its own, so a plain scalar on several lines is scoped line by line.
- A comment whose text holds a tab is not a comment.
- Front matter starts only on the first line of a Markdown document.
  The state of a document's first line therefore differs from the states after it.
- A line of a block scalar is text up to the first line indented less than the scalar's first line.

### TOML

- These are `invalid.illegal`:
  - a value that follows another in an array without a comma;
  - text that is neither a key, a value nor a table header;
  - a stray character after a value.
- `01` is the number `0` and then an invalid `1`.
- A table header is read only at the start of a line.
- A key that begins with a `"` ends at the first later `"` that an `=` follows.
- The grammar has no licence,
  so the lexer is written from the TOML specification and from Shiki's output alone.
- `\s` for non-ASCII characters is JSON's table.
  A pass over every character confirmed it for this grammar too.

### Makefile

- `$` at the end of a recipe line is not a variable.
- A `:` in any line that is not an assignment makes it a target.
- The `endef` of a definition has the scope of `override`.
- A Markdown fence's Makefile lines see the line feed of a line that a quote or list took whole.
  They close the references that line left open.

### Diff

- Diff lines are lexed on their own, as the grammar has no state.
- A Markdown fence's diff lines lose their `^` anchors when a quote or list indents the fence.

### C

- White space before a comment belongs to the comment.
- A word that starts with `or`, `not` or `and` after a type opens an initialization,
  as in `int order(int y);` inside a function.
- `#define` is found anywhere in a line.
  The other directives need the start of the line,
  so in a Markdown fence indented by a list item a directive is plain text.
- An unterminated string keeps every later line.
- `\d`, `\w` and `\s` for non-ASCII characters come from the JSON table above.

### Java

- A string without its closing quote keeps the lines after it until a quote closes it.
- A Javadoc comment is read only where `/**` starts a line, after blanks.
- A declaration is found by a lookahead over the rest of the line, as in the grammar:
  - a method is a line that has a blank and then a `(`, with no `=` or `//` before it;
  - a variable is a type, a name and a `:`, `;` or `=`.
- `\w`, `\s` and `\d` for non-ASCII characters come from the table measured for JSON (`oracle/classes.ts`).

### CSS

- `@charset` is read as the charset rule only on the first line of a document, never in a Markdown fence, as in Shiki.
- A scope name that Shiki builds from the matched text keeps its case:
  - Shiki writes `ONLY` or `Not` in a media query as typed;
  - Shiki writes a unit spelled with the long s (`ſ`) under its own name,
    where nishiki gives the lower case name of the ASCII letter.
- A line that follows a backslash at its end belongs to the escape for the rest of the rule, as in Shiki.
  The grammar's end pattern `^(?<!\G)` never matches again.
- A Markdown fence names the languages `css` and `css.erb`.

### Dockerfile

- A comment is read only at the start of a line, and never inside a string.
- A string runs across lines until its quote.
  An apostrophe in a `RUN` line therefore opens a string that swallows the instructions after it.
- A line that continues another still reads its first word as an instruction.
- A Markdown fence names the language `dockerfile`.
  The `docker` fence is plain text, as in Shiki.

### SQL

- `#` starts no comment.
- A string that closes on its line ignores a backslash before its quote.
- A name after a `.` is a table name, even when it is a number.
- A keyword is matched without regard to case, as Shiki's engine does, so `ſelect` is `select`.

### INI

- A `#` or `;` starts a comment anywhere outside a string, so `a = b # c` has one.
- A `key = value` is found anywhere in a line, so `a = b = c` has two keys.
- A double-quoted string has no escapes.
- A string runs across lines until its quote.
- A section header is read only at the start of a line.
  A Markdown fence's INI lines lose that anchor when a quote or list indents the fence.
- A Markdown fence names the language `ini` or `conf`.
  A `properties` fence is plain text, as in Shiki.

### GraphQL

- The grammar's comment rule comes first everywhere.
  It reads every `"..."` and `"""..."""` as a comment that runs across lines.
  The exception is a string that follows white space where a value is expected, which is a string.
- An enum value's scope includes the white space before it.
- A rule that matches nothing closes the rule it sits in, so `type A x` ends the type at `x`.
- A Markdown fence is plain text, as Shiki's Markdown grammar has no rule that embeds GraphQL.
- A `${...}` in a selection set is lexed as JavaScript by the JavaScript lexer.
  It carries its state across lines, up to the first `}` that no context of the code is waiting for.

### Kotlin

- `/**/` opens a KDoc comment, because `/**` is read before `/**/` could close, so the comment runs on until a later `*/`.
- A KDoc tag is read anywhere inside a KDoc comment, not only at the start of a line.
- `a==b` is a key, `a=`, and an assignment: the key pattern needs one word character, `=` and then another `=`.
- `@a.b:` is the annotation `@a.`, because the pattern gives characters back until no `:` follows.
- A template expression `${...}` ends at its first `}`, so a lambda inside it ends it early.
- `in` and `out` in a type parameter list are types, not modifiers: the pattern for any word comes first.
- A type annotation reads on over words, blanks, `?`, `->` and brackets without quotes.
  `a: Foo, b` ends the annotation at the comma.
- `fun Foo.` with no name after the dot makes `Foo` the function's name.
- A line that opens a group of generics a hundred thousand times and closes it once runs into the work limit.
  The grammar's pattern scans from each opener, and so does the lexer.
- `\w`, `\s` and `\d` for non-ASCII characters come from the JSON table above.

### Go

- A number takes every letter, digit, `_` and `.` after it, and a run that is not one literal, such as `1x` or `0x`, is `invalid.illegal.constant.numeric.go`.
- A `case` line, a label on its own line and a `break`, `goto` or `continue` with its label are read only from the start of the line.
  Inside a Markdown fence that a quote or list indents, they lose that anchor.
- A line that opens a struct many times, such as `type T struct {` repeated, may run into the work limit.
  The grammar's expressions scan from each opener, and so does the lexer.
- Three cases differ from Shiki, all found by mutating the inputs, never in the inputs themselves:
  - after a struct or interface field that ends in text such as `interface{&=`, Shiki leaves a different state for the next line;
  - a non-ASCII character earlier in a line can change which expression Shiki's engine matches;
  - on a long line of many words Shiki's engine stops an expression at its retry limit and leaves the rest of the line plain, where nishiki lexes it.
- A Markdown fence names the language `go` or `golang`.
- `\w`, `\s` and `\d` for non-ASCII characters come from the JSON table above.

### Ruby

- A here document named for another language, such as `<<~SQL` or `<<~RUBY`, is lexed as an ordinary here document.
  Shiki lexes its lines in that language.
- The text after `__END__` is plain, where Shiki lexes it as HTML when it starts like HTML.

### C\#

- A `///` or `//` comment continues through each later line that starts with the same marker after blanks, as one comment.
- A `(` opens a tuple when a `,` or `:` follows anywhere later on the line, as the grammar's lookahead reads the whole line.
- A `/**` comment's lines are checked with `^`, which matches only at column 0.
  In a Markdown fence indented by a list item or a quote, the comment loses its line prefixes.
- Numeric literals follow the grammar's five nested patterns, so `5.` and `1_a` are invalid numbers.
- A type nested more than 64 deep in angle brackets or parentheses reads as no type.
  A type with more than 64 dotted names, pointer marks and array suffixes reads as its first 64.
  A base-class constructor call with more than 32 dotted names reads as its first 32.
  These bounds are constants in `src/csharp/limits.rs`; Oniguruma has larger ones of its own.
- A Markdown fence names the language `cs`, `csharp` or `c#`.
  The `csx` and `c-sharp` fences are plain text, as in Shiki.

### XML

- A `--` inside a comment opens a rule that has no end.
  The rest of the document is then a comment, and `-->` ends nothing.
- `<%--` opens a rule that has no end and no scope.
  The rest of the document has the scope of the document alone.
- An attribute is read only after white space or at the start of a line, so `a="b"c="d"` has one.
  An attribute's name takes the white space before it, not the `=`.
- A `&` that does not start an entity, as in `&a b`, is an illegal ampersand.
- A tag whose first line holds its closing tag, as in `<a b></a>`, is one rule, and the closing tag must repeat the name.
- `<% ... %>` is Java, lexed by the Java lexer.
  A `%>` ends the code only when Java has no context open, so one inside a string, a block or a `//` comment belongs to Java.
- A Markdown fence names the languages `xml`, `xsd`, `tld`, `jsp`, `pt`, `cpt`, `dtml`, `rss`, `opml`, `xsl` and `xslt`.

## Licence

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](https://docs.rs/crate/nishiki/latest/source/LICENSE-APACHE))
- MIT license ([LICENSE-MIT](https://docs.rs/crate/nishiki/latest/source/LICENSE-MIT))

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
