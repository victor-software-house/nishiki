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
  Each `Language` variant on [docs.rs](https://docs.rs/nishiki/latest/nishiki/enum.Language.html) lists the quirks of its language.

## Licence

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](https://docs.rs/crate/nishiki/latest/source/LICENSE-APACHE))
- MIT license ([LICENSE-MIT](https://docs.rs/crate/nishiki/latest/source/LICENSE-MIT))

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
