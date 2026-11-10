# build-script

Two programs that use TextMate grammars compiled ahead of time, built by `nishiki-build-script`, the package of [`Cargo.toml`](Cargo.toml) here.

| File | Does |
|:--|:--|
| [`demo.json`](demo.json) | A small grammar in the JSON form Shiki uses, written for this example |
| [`full.json`](full.json) | A grammar that fills in every field and every list of nishiki's grammar model, for a test |
| [`build.rs`](build.rs) | Compiles both with `Grammar::from_json`, and writes `Grammar::to_bytes` and `Grammar::to_rust` to `OUT_DIR` |
| [`highlight.rs`](highlight.rs) | Prints each token of standard input with its innermost scope, for both programs |
| [`main.rs`](main.rs) | `nishiki-build-script`: loads the demo bytes with `Grammar::from_bytes` |
| [`rust.rs`](rust.rs) | `nishiki-build-script-rust`: includes the demo Rust with `include!` |

```sh
echo 'let n = 1 # one' | cargo run -p nishiki-build-script
echo 'let n = 1 # one' | cargo run -p nishiki-build-script --bin nishiki-build-script-rust
```

Neither program parses JSON at run time.
The build script's nishiki takes the `format` feature, and the programs' nishiki does not.
`check:size` measures both programs.
Their tests run in `mise run test`.
