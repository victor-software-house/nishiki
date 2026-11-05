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

Print each token of a line of Rust with its scopes:

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

## Documentation

The API docs are on [docs.rs](https://docs.rs/nishiki):

- the quick start, themes, limits and cargo features, on the crate page;
- each language's quirks, on its `Language` variant.

## Languages

Each language is a cargo feature.
The Tier column names the tier that turns each one on.
The `all` tier is on by default, and the `textmate` tier is opt-in.

| Language | Variant | Feature | Tier | Names `Language::from_name` takes |
|:--|:--|:--|:--|:--|
| Assembly | `Language::Asm` | `lang-asm` | `textmate` | `asm` |
| Batch | `Language::Bat` | `lang-bat` | `all` | `bat`, `batch`, `cmd` |
| C | `Language::C` | `lang-c` | `all` | `c` |
| CMake | `Language::CMake` | `lang-cmake` | `all` | `cmake` |
| CoffeeScript | `Language::CoffeeScript` | `lang-coffee` | `all` | `coffee`, `coffeescript` |
| Common Lisp | `Language::CommonLisp` | `lang-common-lisp` | `all` | `common-lisp`, `lisp` |
| C++ | `Language::Cpp` | `lang-cpp` | `all` | `cpp`, `c++` |
| C# | `Language::CSharp` | `lang-csharp` | `all` | `csharp`, `c#`, `cs` |
| CSS | `Language::Css` | `lang-css` | `all` | `css` |
| Dart | `Language::Dart` | `lang-dart` | `all` | `dart` |
| Diff | `Language::Diff` | `lang-diff` | `all` | `diff` |
| Dockerfile | `Language::Dockerfile` | `lang-dockerfile` | `all` | `docker`, `dockerfile` |
| Elixir | `Language::Elixir` | `lang-elixir` | `all` | `elixir` |
| ERB | `Language::Erb` | `lang-erb` | `all` | `erb` |
| GDScript | `Language::GdScript` | `lang-gdscript` | `all` | `gdscript`, `gd` |
| Go | `Language::Go` | `lang-go` | `all` | `go`, `golang` |
| GraphQL | `Language::GraphQl` | `lang-graphql` | `all` | `graphql`, `gql` |
| HAML | `Language::Haml` | `lang-haml` | `textmate` | `haml` |
| Haskell | `Language::Haskell` | `lang-haskell` | `all` | `haskell`, `hs` |
| HTML | `Language::Html` | `lang-html` | `all` | `html` |
| HTML (Derivative) | `Language::HtmlDerivative` | `lang-html-derivative` | `all` | `html-derivative` |
| INI | `Language::Ini` | `lang-ini` | `all` | `ini`, `properties` |
| Java | `Language::Java` | `lang-java` | `all` | `java` |
| JavaScript | `Language::JavaScript` | `lang-javascript` | `all` | `javascript`, `js`, `cjs`, `mjs` |
| JSON | `Language::Json` | `lang-json` | `all` | `json` |
| JSON with comments | `Language::Jsonc` | `lang-json` | `all` | `jsonc` |
| JSON Lines | `Language::Jsonl` | `lang-json` | `all` | `jsonl` |
| JSX | `Language::Jsx` | `lang-jsx` | `all` | `jsx` |
| Julia | `Language::Julia` | `lang-julia` | `all` | `julia`, `jl` |
| Kotlin | `Language::Kotlin` | `lang-kotlin` | `all` | `kotlin`, `kt`, `kts` |
| Less | `Language::Less` | `lang-less` | `all` | `less` |
| Lua | `Language::Lua` | `lang-lua` | `all` | `lua` |
| Makefile | `Language::Make` | `lang-make` | `all` | `make`, `makefile` |
| Markdown | `Language::Markdown` | `lang-markdown` | `all` | `markdown`, `md` |
| MATLAB | `Language::Matlab` | `lang-matlab` | `all` | `matlab` |
| Mermaid | `Language::Mermaid` | `lang-mermaid` | `all` | `mermaid`, `mmd` |
| Nix | `Language::Nix` | `lang-nix` | `all` | `nix` |
| Objective-C | `Language::ObjectiveC` | `lang-objective-c` | `all` | `objective-c`, `objc` |
| Objective-C++ | `Language::ObjectiveCpp` | `lang-objective-cpp` | `all` | `objective-cpp` |
| Pascal | `Language::Pascal` | `lang-pascal` | `all` | `pascal` |
| Perl | `Language::Perl` | `lang-perl` | `all` | `perl` |
| PHP | `Language::Php` | `lang-php` | `all` | `php` |
| PostCSS | `Language::PostCss` | `lang-postcss` | `all` | `postcss` |
| PowerShell | `Language::PowerShell` | `lang-powershell` | `all` | `powershell`, `ps`, `ps1`, `pwsh` |
| Protobuf | `Language::Proto` | `lang-proto` | `all` | `proto`, `protobuf` |
| Python | `Language::Python` | `lang-python` | `all` | `python`, `py` |
| R | `Language::R` | `lang-r` | `all` | `r` |
| Ruby | `Language::Ruby` | `lang-ruby` | `all` | `ruby`, `rb` |
| Rust | `Language::Rust` | `lang-rust` | `all` | `rust`, `rs` |
| Sass | `Language::Sass` | `lang-sass` | `all` | `sass` |
| Scala | `Language::Scala` | `lang-scala` | `all` | `scala` |
| SCSS | `Language::Scss` | `lang-scss` | `all` | `scss` |
| Shell | `Language::Shell` | `lang-shell` | `all` | `shellscript`, `bash`, `sh`, `shell`, `zsh` |
| SQL | `Language::Sql` | `lang-sql` | `all` | `sql` |
| Svelte | `Language::Svelte` | `lang-svelte` | `all` | `svelte` |
| Swift | `Language::Swift` | `lang-swift` | `all` | `swift` |
| TOML | `Language::Toml` | `lang-toml` | `all` | `toml` |
| TSX | `Language::Tsx` | `lang-tsx` | `all` | `tsx` |
| TypeScript | `Language::TypeScript` | `lang-typescript` | `all` | `typescript`, `ts`, `cts`, `mts` |
| Visual Basic | `Language::Vb` | `lang-vb` | `all` | `vb` |
| Vue | `Language::Vue` | `lang-vue` | `all` | `vue` |
| XML | `Language::Xml` | `lang-xml` | `all` | `xml` |
| YAML | `Language::Yaml` | `lang-yaml` | `all` | `yaml`, `yml` |
| Zig | `Language::Zig` | `lang-zig` | `all` | `zig` |

## Status

- The API is unstable while the version is `0.x`.
- The scopes match Shiki's, quirks included.

## Licence

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](https://docs.rs/crate/nishiki/latest/source/LICENSE-APACHE))
- MIT license ([LICENSE-MIT](https://docs.rs/crate/nishiki/latest/source/LICENSE-MIT))

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
