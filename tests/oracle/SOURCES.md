# Oracle inputs

`mise run oracle` fetches each input below at its pinned commit and writes Shiki's scopes for it to `<language>/<name>.txt`.
Each fixture repeats its input's text line by line.
The fixtures are test data: `Cargo.toml` leaves `tests/` out of the published crate.

| Fixture | Source | Licence |
|:--|:--|:--|
| `rust/sample` | [`samples/rust.sample`][rust-sample] in shikijs/textmate-grammars-themes | [MIT][shiki-samples-licence] |
| `rust/ripgrep-searcher` | [`crates/searcher/src/searcher/mod.rs`][ripgrep-searcher] in BurntSushi/ripgrep | [Unlicense or MIT][ripgrep-licence] |
| `rust/serde-de` | [`serde_core/src/de/mod.rs`][serde-de] in serde-rs/serde | [MIT or Apache-2.0][serde-licence] |
| `markdown/sample` | [`samples/markdown.sample`][markdown-sample] in shikijs/textmate-grammars-themes | [MIT][shiki-samples-licence] |
| `markdown/tokio-readme` | [`README.md`][tokio-readme] in tokio-rs/tokio | [MIT][tokio-licence] |
| `markdown/serde-readme` | [`README.md`][serde-readme] in serde-rs/serde | [MIT or Apache-2.0][serde-licence] |
| `markdown/book-ownership` | [`src/ch04-01-what-is-ownership.md`][book-ownership] in rust-lang/book | [MIT or Apache-2.0][book-licence] |
| `markdown/book-strings` | [`src/ch08-02-strings.md`][book-strings] in rust-lang/book | [MIT or Apache-2.0][book-licence] |
| `markdown/clap-readme` | [`README.md`][clap-readme] in clap-rs/clap | [MIT or Apache-2.0][clap-licence] |

Each fixture also has one `.colors` file per theme below: Shiki's colour and font style for each token.

| Theme | Source | Licence |
|:--|:--|:--|
| `themes/github-dark` | [primer/github-vscode-theme][github-theme], through `@shikijs/themes` 4.6.0 | [MIT][github-theme-licence] |
| `themes/one-light` | [akamud/vscode-theme-onelight][one-light], through `@shikijs/themes` 4.6.0 | [MIT][one-light-licence] |
| `themes/catppuccin-mocha` | [catppuccin/vscode][catppuccin], through `@shikijs/themes` 4.6.0 | [MIT][catppuccin-licence] |

`ripgrep-searcher` and `serde-de` are held out: they were not among the inputs the Rust lexer was written against.
`markdown/book-ownership`, `markdown/book-strings` and `markdown/clap-readme` are held out in the same way: they were added after the other Markdown inputs passed.

[rust-sample]: https://github.com/shikijs/textmate-grammars-themes/blob/8f1828ebccc57930cf0adde9b35ae929584fe191/samples/rust.sample
[shiki-samples-licence]: https://github.com/shikijs/textmate-grammars-themes/blob/8f1828ebccc57930cf0adde9b35ae929584fe191/LICENSE
[ripgrep-searcher]: https://github.com/BurntSushi/ripgrep/blob/91f5d22ef48bde459d90a8c7dbeab416e97d0f2b/crates/searcher/src/searcher/mod.rs
[ripgrep-licence]: https://github.com/BurntSushi/ripgrep/blob/91f5d22ef48bde459d90a8c7dbeab416e97d0f2b/COPYING
[serde-de]: https://github.com/serde-rs/serde/blob/0db9b3b504bcb0a7fa9cf001429869519afd3e9a/serde_core/src/de/mod.rs
[serde-licence]: https://github.com/serde-rs/serde/blob/0db9b3b504bcb0a7fa9cf001429869519afd3e9a/LICENSE-MIT
[github-theme]: https://github.com/primer/github-vscode-theme/tree/cd78e5e4e7bcf132a6f428ae0f32264bb1b729cf
[github-theme-licence]: https://github.com/primer/github-vscode-theme/blob/cd78e5e4e7bcf132a6f428ae0f32264bb1b729cf/LICENSE
[one-light]: https://github.com/akamud/vscode-theme-onelight/tree/5866e900db932d580e978a58db42f65cde07998b
[one-light-licence]: https://github.com/akamud/vscode-theme-onelight/blob/5866e900db932d580e978a58db42f65cde07998b/LICENSE
[catppuccin]: https://github.com/catppuccin/vscode/tree/befc9e6fc41980f4241408f7049755d47c06ff45
[catppuccin-licence]: https://github.com/catppuccin/vscode/blob/befc9e6fc41980f4241408f7049755d47c06ff45/LICENSE
[markdown-sample]: https://github.com/shikijs/textmate-grammars-themes/blob/8f1828ebccc57930cf0adde9b35ae929584fe191/samples/markdown.sample
[tokio-readme]: https://github.com/tokio-rs/tokio/blob/b499257ef62bd631b83a988d371f4330f545670c/README.md
[tokio-licence]: https://github.com/tokio-rs/tokio/blob/b499257ef62bd631b83a988d371f4330f545670c/LICENSE
[serde-readme]: https://github.com/serde-rs/serde/blob/0db9b3b504bcb0a7fa9cf001429869519afd3e9a/README.md
[book-ownership]: https://github.com/rust-lang/book/blob/2771df165f092a4fbb9f46b5b2e7450f19010adb/src/ch04-01-what-is-ownership.md
[book-strings]: https://github.com/rust-lang/book/blob/2771df165f092a4fbb9f46b5b2e7450f19010adb/src/ch08-02-strings.md
[book-licence]: https://github.com/rust-lang/book/blob/2771df165f092a4fbb9f46b5b2e7450f19010adb/LICENSE-MIT
[clap-readme]: https://github.com/clap-rs/clap/blob/db30b39a55c188763666268447050a0dacba3bf0/README.md
[clap-licence]: https://github.com/clap-rs/clap/blob/db30b39a55c188763666268447050a0dacba3bf0/LICENSE-MIT
