# build-script

A program that compiles a TextMate grammar ahead of time, built by `nishiki-build-script`, the package of [`Cargo.toml`](Cargo.toml) here.

| File | Does |
|:--|:--|
| [`demo.json`](demo.json) | A small grammar in the JSON form Shiki uses, written for this example |
| [`build.rs`](build.rs) | Compiles `demo.json` with `Grammar::from_json` and writes `Grammar::to_bytes` to `OUT_DIR` |
| [`main.rs`](main.rs) | Loads the bytes with `Grammar::from_bytes` and prints each token of standard input with its innermost scope |

```sh
echo 'let n = 1 # one' | cargo run --package nishiki-build-script
```

The program parses no JSON at run time.
Its test runs in `mise run test`.
