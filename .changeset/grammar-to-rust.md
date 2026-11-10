---
nishiki: patch
---

A build script can write a grammar as Rust, which a program includes as a `static` that takes no work to load.
The `codegen` feature gives `Grammar` an implementation of `quote::ToTokens`, and the `format` feature adds `Grammar::to_rust`.
Both are off by default and not in `all`.
