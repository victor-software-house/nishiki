# Proposal

## Motivation

A caller's TextMate grammar loads at run time, from its JSON or from bytes a build script compiled.
Either way the program reads and decodes the grammar before its first line.
For TypeScript's grammar, reading the bytes took 312 µs on a box.

## Changes

- nishiki gains two non-default features:
  - `codegen`: `proc-macro2` and `quote`, and `ToTokens` for `Grammar`;
  - `format`: `prettyplease` and `syn`, and `Grammar::to_rust`, which lays the tokens out as a file.
- A build script writes a grammar's tokens to `OUT_DIR`, and the program holds the grammar as a `static`.
- `compile::UnlinkedGrammar` holds `Cow` data, so a `static` can hold it.
- The regex parser's entry and tree become `pub(crate)`, for [`rule-table-codegen`][rule-table-codegen].

## Related changes

`rust-codegen` was split into five changes, so each fits the OpenSpec word budgets (lead, 2026-11-10):

- [`rust-codegen`][rust-codegen]: the `codegen/` crate, the fact files and the writers of numeric tables.
- [`grammar-to-rust`][grammar-to-rust]: the `codegen` and `format` features, `ToTokens` and `Grammar::to_rust`.
- [`unstable-rule-engine`][unstable-rule-engine]: `nishiki::unstable`, under `--cfg nishiki_unstable`.
- [`default-matchers`][default-matchers]: a matcher for each pattern, written from its expression, with the prototype that measures it.
- [`rule-table-codegen`][rule-table-codegen]: the rule-engine tables, their names, and the move of each family.

## Capabilities

### Modified Capabilities

- `grammar-loading`: a build script can write a caller's grammar as Rust tokens.

## Impact

- `Cargo.toml`: the `codegen` and `format` features, with four optional dependencies.
  Default builds gain no crate.
- `src/grammar.rs`: one public item behind `codegen`, and one behind `format`.
- `src/textmate/compile.rs`: the unlinked model holds borrowed or owned data.
- `examples/build-script/`: a second program, built from `to_rust`.

[default-matchers]: ../default-matchers/proposal.md
[grammar-to-rust]: proposal.md
[rule-table-codegen]: ../rule-table-codegen/proposal.md
[rust-codegen]: ../rust-codegen/proposal.md
[unstable-rule-engine]: ../unstable-rule-engine/proposal.md
