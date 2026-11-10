# examples

The programs the benchmark tasks run, and a grammar compiled ahead of time.
The benchmark programs are built by `nishiki-examples`, the package of [`Cargo.toml`](Cargo.toml) here.
The package is never published, so the published nishiki declares none of them.

| Program | Run by | Does |
|:--|:--|:--|
| [`identity.rs`](identity.rs) | `mise run identity` | Prints every oracle fixture's tokens, and those of edited copies, at a pull request's base and head |
| [`lex.rs`](lex.rs) | `mise run callgrind` | Lexes one language's fixtures in three passes, whose instructions callgrind counts |
| [`memory.rs`](memory.rs) | `mise run callgrind` | Prints what one pass over a language's fixtures allocates |
| [`size.rs`](size.rs) | `mise run check:size` | Highlights standard input with every language the build turns on |
| [`build-script/`](build-script/README.md) | `mise run test` and `mise run check:size` | Compiles grammars in its build script, then highlights standard input with the bytes or the Rust |

`build-script/` is a package of its own, `nishiki-build-script`.
Its build script needs nishiki's `format` feature, and its programs need `textmate`.
The other programs then build without either.

The features `json` and `textmate` turn on nishiki's features of the same names.
The default feature turns on nishiki's default features.
`check:size` turns it off and names one tier's features instead.
