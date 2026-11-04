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
Each line gives tokens with Shiki's scope stacks, and a state the next line starts from.

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

The [crate docs](https://docs.rs/nishiki) cover the API:

- a quick start;
- themes;
- limits;
- cargo features and tiers;
- Markdown fences.

## Languages

| Language | Variant | Feature | Names `Language::from_name` takes |
|:--|:--|:--|:--|
| Batch | `Language::Bat` | `lang-bat` | `bat`, `batch`, `cmd` |
| C | `Language::C` | `lang-c` | `c` |
| CMake | `Language::CMake` | `lang-cmake` | `cmake` |
| C++ | `Language::Cpp` | `lang-cpp` | `cpp`, `c++` |
| C# | `Language::CSharp` | `lang-csharp` | `csharp`, `c#`, `cs` |
| CSS | `Language::Css` | `lang-css` | `css` |
| Dart | `Language::Dart` | `lang-dart` | `dart` |
| Diff | `Language::Diff` | `lang-diff` | `diff` |
| Dockerfile | `Language::Dockerfile` | `lang-dockerfile` | `docker`, `dockerfile` |
| ERB | `Language::Erb` | `lang-erb` | `erb` |
| Go | `Language::Go` | `lang-go` | `go`, `golang` |
| GraphQL | `Language::GraphQl` | `lang-graphql` | `graphql`, `gql` |
| Haskell | `Language::Haskell` | `lang-haskell` | `haskell`, `hs` |
| HTML | `Language::Html` | `lang-html` | `html` |
| HTML (Derivative) | `Language::HtmlDerivative` | `lang-html-derivative` | `html-derivative` |
| INI | `Language::Ini` | `lang-ini` | `ini`, `properties` |
| Java | `Language::Java` | `lang-java` | `java` |
| JavaScript | `Language::JavaScript` | `lang-javascript` | `javascript`, `js`, `cjs`, `mjs` |
| JSON | `Language::Json` | `lang-json` | `json` |
| JSON with comments | `Language::Jsonc` | `lang-json` | `jsonc` |
| JSON Lines | `Language::Jsonl` | `lang-json` | `jsonl` |
| JSX | `Language::Jsx` | `lang-jsx` | `jsx` |
| Julia | `Language::Julia` | `lang-julia` | `julia`, `jl` |
| Kotlin | `Language::Kotlin` | `lang-kotlin` | `kotlin`, `kt`, `kts` |
| Less | `Language::Less` | `lang-less` | `less` |
| Lua | `Language::Lua` | `lang-lua` | `lua` |
| Makefile | `Language::Make` | `lang-make` | `make`, `makefile` |
| Markdown | `Language::Markdown` | `lang-markdown` | `markdown`, `md` |
| Mermaid | `Language::Mermaid` | `lang-mermaid` | `mermaid`, `mmd` |
| Nix | `Language::Nix` | `lang-nix` | `nix` |
| PowerShell | `Language::PowerShell` | `lang-powershell` | `powershell`, `ps`, `ps1`, `pwsh` |
| Protobuf | `Language::Proto` | `lang-proto` | `proto`, `protobuf` |
| Python | `Language::Python` | `lang-python` | `python`, `py` |
| Ruby | `Language::Ruby` | `lang-ruby` | `ruby`, `rb` |
| Rust | `Language::Rust` | `lang-rust` | `rust`, `rs` |
| Scala | `Language::Scala` | `lang-scala` | `scala` |
| SCSS | `Language::Scss` | `lang-scss` | `scss` |
| Shell | `Language::Shell` | `lang-shell` | `shellscript`, `bash`, `sh`, `shell`, `zsh` |
| SQL | `Language::Sql` | `lang-sql` | `sql` |
| Swift | `Language::Swift` | `lang-swift` | `swift` |
| TOML | `Language::Toml` | `lang-toml` | `toml` |
| TSX | `Language::Tsx` | `lang-tsx` | `tsx` |
| TypeScript | `Language::TypeScript` | `lang-typescript` | `typescript`, `ts`, `cts`, `mts` |
| XML | `Language::Xml` | `lang-xml` | `xml` |
| YAML | `Language::Yaml` | `lang-yaml` | `yaml`, `yml` |
| Zig | `Language::Zig` | `lang-zig` | `zig` |

## Status

- The API is unstable while the version is `0.x`.
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
- With `lang-html-derivative`, html-derivative lexes the HTML in the five places below, where Shiki's grammar includes it.
  Without it, that HTML is plain text, as in Shiki without the grammar.
- An HTML block, a line that starts with a tag, runs to a line of blanks, not to an empty line, which Shiki does not tokenize.
- A `<script>`, `<style>` or `<pre>` block that its first line does not close runs to the line with the closing tag, whose text before the tag html-derivative lexes from a fresh state.
- In a paragraph, html-derivative's root rules come after the inline rules at each position, and a tag or entity hands the line back once it ends.
- In a heading, html-derivative reads the heading's text the same way, cut at the end of that text.
- In bold, italic and strikethrough, a `<` with a `>` later on the line hands the tags there to html-derivative, up to the first place after a `>` where no tag starts.
- An HTML block inside a quote or a list starts again on each line, from a fresh HTML state, because the grammar's `while` rules match only at the start of a line.
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

### C++

- A Markdown fence's C++ lines keep `source.cpp` on the stack, where C's lose `source.c`: Markdown's grammar names it in the fence's `contentName`.
- A Markdown fence names the languages `cpp`, `c++` and `cxx`.
- Raw strings are lexed without the grammars that Shiki embeds in `R"re(`, `R"_r(`, `R"regex(`, `R"glsl(` and `R"GLSL(`.
  Their text is plain raw-string text.
- A raw string whose delimiter is longer than 16 bytes closes on `)"` alone.
  Shiki closes it on the delimiter.
- `\d`, `\w` and `\s` for non-ASCII characters come from the JSON table above.
- Bounds the lexer sets itself, beside `Limits`:
  - template arguments and bracket groups nested deeper than 20 levels do not match, as in the Oniguruma engine Shiki runs;
  - a repetition that a matcher follows by recursion, such as a run of type specifiers, stops after 64 iterations;
  - an attribute tries its first 16 closings, so a line with more reads as having no later ones;
  - comments between two tokens give back at most 8 of themselves when what follows fails;
  - capture groups lexed again inside one another, as a long `a::b::c::` chain's are, stop after 64 levels, and the text of a deeper one keeps its own scopes.

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
  - Shiki writes `@styleset` or `@stylistic` spelled with `ſ` or a ligature (`@ﬆyleset`) under its own name,
    where nishiki gives the lower case name of the ASCII letters.
- A line that follows a backslash at its end belongs to the escape for the rest of the rule, as in Shiki.
  The grammar's end pattern `^(?<!\G)` never matches again.
- A Markdown fence names the languages `css` and `css.erb`.

### Swift

- An operator in an expression is a custom operator, the known ones included:
  `=`, `==`, `??` and `+` get `keyword.operator.custom.infix`.
  The grammar's patterns for the known operators start with `\G`, and `\G` does not hold at the start of a capture group's own patterns in Shiki's engine.
  A pattern that matches the operator directly names it, as with the `=` of a default argument in a parameter clause.
- `\p{L}`, `\p{N}` and `\p{M}` for non-ASCII characters come from tables measured on Shiki's engine (`oracle/classes.ts`).
- A Markdown fence names the language `swift`.
  In a fence, `\G` holds at the start of every line, as Shiki's fence rule leaves it:
  - the lines of a `"""` string after its opening line are `invalid.illegal.content-after-opening-delimiter`;
  - a `}` that starts a line does not end an enum's case list, so the enum stays open after it.

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

- A here document named for SQL, GraphQL, CSS, C, C++, HTML, JavaScript, shell, Lua or Ruby, such as `<<~SQL`, lexes its lines in that language, with Ruby's interpolation and escapes where the language has nothing open, as Shiki does.
- One named for YAML, HAML, Slim or XML, such as `<<~YAML`, keeps that language's scopes, but its lines are plain text with Ruby's interpolation.
  Shiki lexes them in that language.
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

### HTML

- A `<` that no rule claims is an illegal angle bracket, even inside a quoted attribute value, as the grammar's injection makes it.
- An attribute's name is a word of the grammar's lists, or any other run of name characters, which gets a scope built from its text, such as `meta.attribute.unrecognized.fooBar.html`.
- `svg` and `math` elements have rules of their own, and read the attributes of a start tag that the line holds whole on that text alone.
- The CSS of a `<style>` element is lexed by the CSS lexer, and the JavaScript of a `<script>` element and of an event handler by the JavaScript lexer.
  The code ends at its end tag, in any case, only where it has no rule open, so a `</style>` inside an open CSS block is CSS.
  The grammar's own `//` and `/* */` rules in a script end at a lower-case `</script` only.
- A `<script>` whose `type` is an HTML template holds HTML, and one of any other type that is not JavaScript is `source.unknown`.
- A line stops after `MAX_STALLED` (4,096) steps in a row that do not advance, and the rest of it keeps the innermost rule's scope.
  At one byte a run of such steps opens and closes each rule of the grammar at most once, because a rule that repeats stops the line, so no line comes near it.
  The bound is a constant in `src/html/engine.rs` until the next minor release moves it into `Limits`.
- A Markdown fence names the languages `html`, `htm`, `shtml`, `xhtml`, `inc`, `tmpl` and `tpl`.

### ERB

- The HTML lexer runs ERB, with ERB's injection in place of the HTML grammar's, so a bare `<` is text.
- A `<%` opens ERB where no HTML rule matches at it, outside comments: in text and in a quoted attribute value, but not inside a start tag, where an attribute rule takes the `<`.
- `<%#` is a comment up to `%>`, and `<%%` is no escape but code whose Ruby starts with `%`.
- The code is Ruby, lexed by the Ruby lexer, up to `-%>` or `%>`.
  The end counts only where Ruby has no context open, so a `%>` inside a Ruby string belongs to Ruby.
  A `#` before the end on its line starts ERB's comment, and Ruby's own comment otherwise.
- Code that its line does not close is `meta.embedded.block.erb`, and the blanks before it at the start of a line and after its end get scopes of their own.
- In a `<style>` element ERB opens between CSS rules, and in a `<script>` element where no JavaScript rule matches, so JavaScript's `<` operator takes `<%=` in code.
- A heredoc of another language inside the Ruby, such as `<<~SQL`, stays Ruby text.
- ERB that Shiki opens inside a CSS rule or string, or inside a JavaScript string, template or JSX element, as in `'a-<%= x %>'` in a script, stays that language's text, because the CSS and JavaScript lexers halt for ERB only at their top level.

### HTML (Derivative)

- The HTML lexer runs html-derivative, the HTML grammar with a rule of its own for a tag that no rule before it takes, such as `<Foo/>` or `<x:y>`: `meta.tag.other.unrecognized.html.derivative`, where HTML reads an invalid tag.
- That rule applies at the document's top level only; inside a template `<script>`, which the grammar reads as `text.html.basic`, unknown tags are HTML's invalid tags.
- A bare `<` is an illegal angle bracket, as in HTML, but not where another grammar embeds html-derivative, as Vue's templates do, since Shiki applies an embedded grammar's injection nowhere.

### Lua

- The body of `ffi.cdef[[...]]` is the plain content of its brackets, where Shiki lexes it as C.
- A long string or comment closes only at a bracket with as many `=` signs as it opened with, across lines.
- A Markdown fence names the language `lua`.

### Julia

- The C++ inside a `cxx"""` or `icxx"""` string macro and the R inside an `R"""` one are plain, where Shiki lexes them as C++ and R.
  Julia's table leaves out the C++ and R grammars, which Julia embeds: R's may not be bundled, and C++'s brings GLSL's, which may not either.
  The macro and its quotes keep Shiki's scopes, and so does `$` interpolation inside.
- The Python, JavaScript and SQL inside `py"""`, `js"""` and `sql"""` string macros are lexed as Shiki lexes them, by engine copies of those grammars in Julia's table.

### SCSS

- The rules the grammar takes from CSS run on the CSS lexer's matchers, and keep CSS's scope names.
- A `///` comment is a plain line comment.
  The grammar's doc-comment rule includes only SassDoc's grammar, which Shiki does not bundle, so Shiki drops the rule.
- The whole suffix after a parent selector, as `-b` in `&-b`, is `punctuation.definition.entity.css`.
- In an attribute selector such as `[x="y"]`, the quotes of the value are a string and the text between them, `y`, is not.
- A Markdown fence names the language `scss`.

### Less

- A declaration whose property the grammar does not list reads as a selector, as `b: c` in `.a { b: c }`, where `b` is a tag.
- `@viewport` reads as a variable: the grammar's at-rules include `#at-viewport`, which it never defines, and Shiki skips the include.
- Outside a guard, `default()` is the guard function only when a word character follows its `)`; otherwise `default` is a value keyword.
- A Markdown fence names the language `less`.

## Licence

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](https://docs.rs/crate/nishiki/latest/source/LICENSE-APACHE))
- MIT license ([LICENSE-MIT](https://docs.rs/crate/nishiki/latest/source/LICENSE-MIT))

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
