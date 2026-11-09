# examples

The programs the benchmark tasks run, built by `nishiki-examples`, the package of [`Cargo.toml`](Cargo.toml) here.
The package is never published, so the published nishiki declares none of them.

| Program | Run by | Does |
|:--|:--|:--|
| [`identity.rs`](identity.rs) | `mise run identity` | Prints every oracle fixture's tokens, and those of edited copies, at a pull request's base and head |
| [`lex.rs`](lex.rs) | `mise run callgrind` | Lexes one language's fixtures in three passes, whose instructions callgrind counts |
| [`memory.rs`](memory.rs) | `mise run callgrind` | Prints what one pass over a language's fixtures allocates |

The features `json` and `textmate` turn on nishiki's features of the same names.
