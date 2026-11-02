# Oracle inputs

`mise run oracle` fetches each input below at its pinned commit and writes Shiki's scopes for it to `<language>/<name>.txt`.
Each fixture repeats its input's text line by line.
The fixtures are test data: `Cargo.toml` leaves `tests/` out of the published crate.

| Fixture | Source | Licence |
|:--|:--|:--|
| `rust/sample` | [`samples/rust.sample`][rust-sample] in shikijs/textmate-grammars-themes | [MIT][shiki-samples-licence] |
| `rust/ripgrep-searcher` | [`crates/searcher/src/searcher/mod.rs`][ripgrep-searcher] in BurntSushi/ripgrep | [Unlicense or MIT][ripgrep-licence] |
| `rust/serde-de` | [`serde_core/src/de/mod.rs`][serde-de] in serde-rs/serde | [MIT or Apache-2.0][serde-licence] |

`ripgrep-searcher` and `serde-de` are held out: they were not among the inputs the Rust lexer was written against.

[rust-sample]: https://github.com/shikijs/textmate-grammars-themes/blob/8f1828ebccc57930cf0adde9b35ae929584fe191/samples/rust.sample
[shiki-samples-licence]: https://github.com/shikijs/textmate-grammars-themes/blob/8f1828ebccc57930cf0adde9b35ae929584fe191/LICENSE
[ripgrep-searcher]: https://github.com/BurntSushi/ripgrep/blob/91f5d22ef48bde459d90a8c7dbeab416e97d0f2b/crates/searcher/src/searcher/mod.rs
[ripgrep-licence]: https://github.com/BurntSushi/ripgrep/blob/91f5d22ef48bde459d90a8c7dbeab416e97d0f2b/COPYING
[serde-de]: https://github.com/serde-rs/serde/blob/0db9b3b504bcb0a7fa9cf001429869519afd3e9a/serde_core/src/de/mod.rs
[serde-licence]: https://github.com/serde-rs/serde/blob/0db9b3b504bcb0a7fa9cf001429869519afd3e9a/LICENSE-MIT
