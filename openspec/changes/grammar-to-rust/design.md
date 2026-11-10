# Design

This change lets a build script write a caller's grammar as Rust, so the program holds it in a `static`.
[The proposal][proposal] links the four changes split from `rust-codegen` with it.

## 1. Features

| Feature | Dependencies | What it adds |
|:--|:--|:--|
| `codegen` | `proc-macro2`, `quote` | `ToTokens` for `Grammar`, and the code writer under `nishiki::unstable` |
| `format` | `codegen`, `prettyplease`, `syn` | `Grammar::to_rust`, a formatted file |

Neither is on by default, and neither is in `all`.
A caller turns them on for a build script's dependency, so the program's own build gains nothing.
[prost-build][prost-format] has the same split: tokens always, formatting behind `format`.
The operator chose these dependencies on 2026-11-10.

## 2. A caller's grammar as Rust

- `impl quote::ToTokens for Grammar`, with `codegen`.
  The tokens are an expression of type `Grammar`, built in a `static` with no work at run time.
  A build script writes them to `OUT_DIR`, and the program reads them with `include!`, as the [Cargo book][cargo-build-scripts] shows.
- `Grammar::to_rust(&self) -> String`, with `format`.
  It borrows, as the API guidelines' `to_` prefix does ([C-CONV][c-conv]), so a caller can keep using the grammar.
  It returns a file that holds `pub static GRAMMAR: nishiki::Grammar = …;`, laid out by prettyplease.
  The build script sets `DEMO_RUST` to the file's path.
  The program writes `mod demo { include!(env!("DEMO_RUST")); }` and uses `demo::GRAMMAR`, since `clippy.toml` bans `concat!`.
- Neither can fail, so neither has an `# Errors` section.
- Both get a doctest, and `examples/build-script/` gains a second program that uses `to_rust`.
- It saves the read of the bytes, which took 312 µs for TypeScript's grammar on a box (`bench:cold`).
  Linking, 2,565 µs there, still happens at the first `Highlighter::for_grammar`.

The tokens name the unlinked model's types.
They sit in a `#[doc(hidden)] pub mod __private`, outside the documented API.
That module needs only `textmate`, because the program's normal dependency builds without `codegen`.

The tokens are a block expression, `{ const _: () = ::nishiki::__private::FORMAT_1; <the Grammar value> }`.
A `static` initializer accepts a block that holds an item.
A program built with tokens from another nishiki version then fails to compile.
The missing name shows the cause, and no program runs with wrong tables.

The build-script example writes the tokens on every build, so its tables always match the nishiki it links.

Its build-dependency on nishiki takes `format`, and its normal dependency does not.
Cargo's version 2 feature resolver, which resolver 3 keeps, does not unify a build-dependency's features with a normal dependency's ([Cargo reference][cargo-resolver]).
So the example program, and `check:size`, build nishiki without them.

## 3. The unlinked model in a `static`

`compile::UnlinkedGrammar` holds `String` and `Vec` fields, which a `static` cannot hold.
Its strings become `Cow<'static, str>` and its lists `Cow<'static, [T]>`.

- The compiler and `from_bytes` build owned values, as today.
- The tokens build borrowed ones, and `Cow::Borrowed` is a `const` constructor.
- The linker reads both through `Deref`, unchanged.
- No second model exists, so the bytes format, the compiler and the tokens share one type.

This refactor is its own first pull request, with no behaviour change.

## 4. Dependencies

| Crate | Feature | Version | Why |
|:--|:--|:--|:--|
| `proc-macro2` | `codegen` | 1.0.107 | the token type the public functions return |
| `quote` | `codegen` | 1.0.47 | builds tokens |
| `syn` | `format` | 3.0.10 | prettyplease reads a `syn::File` |
| `prettyplease` | `format` | 0.3.0 | lays out tokens as a file |

- `codegen` adds 3 crates: `proc-macro2`, `quote` and `unicode-ident`.
  `format` adds 2 more ([research][research], "Dependency cost").
- Default builds and `all` gain none.
- The size and build time of each feature are measured on a box (task 1.1), in section 5.

## 5. Feature cost

On a box, from a clean target:

| Features | Crates | Wall seconds |
|:--|--:|--:|
| `all` | 2 | 58 |
| `all,codegen` | 5 | 78 |
| `all,codegen,format` | 7 | 87 |

- `codegen` adds `proc-macro2`, `quote` and `unicode-ident`, about 3 s on the critical path.
- `format` adds `syn` and `prettyplease`, about 20 s more.
- Neither is a default feature, so a caller who asks for neither builds as before.
- The program size of `examples/build-script` is measured when task 2.2 adds the example.

[c-conv]: https://rust-lang.github.io/api-guidelines/naming.html#ad-hoc-conversions-follow-as_-to_-into_-conventions-c-conv
[cargo-build-scripts]: https://doc.rust-lang.org/cargo/reference/build-scripts.html
[cargo-resolver]: https://doc.rust-lang.org/cargo/reference/resolver.html#feature-resolver-version-2
[proposal]: proposal.md
[prost-format]: https://github.com/tokio-rs/prost/blob/b3f73911a2dca69a798fff418d35ed07e30edbc3/prost-build/src/config.rs
[research]: ../../research/2026-11-10-rust-codegen-practice.md
