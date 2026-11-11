# Proposal

## Motivation

TypeScript scripts under `oracle/` write the generated Rust under `src/`:

- 48 per-language scripts, `oracle/<language>-grammar.ts`, write the rule-engine tables.
  With the three files they share, they hold 18,848 lines, and most of those lines are copies.
- 7 word scripts write `words.rs` files from grammar expressions.
- `oracle/unicode-tables.ts`, `oracle/case-folds.ts` and `oracle/classes.ts` measure Oniguruma and write the tables in the same run.

A fix to an emitter has to be made in each copy.
The scripts read grammars and regular expressions a second time, beside the compiler in `src/textmate/compile` and the parser in `src/textmate/regex`.

The operator chose Rust for every generator (2026-11-10).

## Changes

- Generated Rust is written by Rust.
  A script under `oracle/` only asks Shiki or Oniguruma for facts.
  It records them as formatted JSON under `oracle/facts/`.
- A crate, `codegen/` (`publish = false`), in a workspace of its own, writes every committed generated file.
  `mise run codegen` runs it.
  CI runs it and fails when a file would change.
- The Unicode, case-fold and class tables are written from the fact files.
- The 7 word scripts and `fence-stops.ts` move into `codegen/`.
- The rule-engine tables move in [`rule-table-codegen`][rule-table-codegen], through the same crate.

## Related changes

`rust-codegen` was split into five changes, so each fits the OpenSpec word budgets (lead, 2026-11-10):

- [`rust-codegen`][rust-codegen]: the `codegen/` crate, the fact files and the writers of numeric tables.
- [`grammar-to-rust`][grammar-to-rust]: the `codegen` and `format` features, `ToTokens` and `Grammar::to_rust`.
- [`unstable-rule-engine`][unstable-rule-engine]: `nishiki::unstable`, under `--cfg nishiki_unstable`.
- [`default-matchers`][default-matchers]: a matcher for each pattern, written from its expression, with the prototype that measures it.
- [`rule-table-codegen`][rule-table-codegen]: the rule-engine tables, their names, and the move of each family.

## Capabilities

### New Capabilities

- `generated-rust`: every generated Rust file comes from a Rust writer that reads grammars and recorded facts.

## Impact

- `codegen/`: a new crate, with its own manifest, lock file and target directory.
- Every generated file: a `// @generated` header, and `#[rustfmt::skip]` on its `mod`.
  prettyplease lays out the structured files, so their lines change but their tokens do not.
- `oracle/`: the 7 word scripts go, and `fence-stops.ts` becomes a check in `codegen/`.
- `oracle/facts/`: three fact files, written by the scripts that keep measuring.
- `AGENTS.md` and `CODING_STANDARDS.md`: the rule that Rust writes generated Rust.

[default-matchers]: ../default-matchers/proposal.md
[grammar-to-rust]: ../grammar-to-rust/proposal.md
[rule-table-codegen]: ../rule-table-codegen/proposal.md
[rust-codegen]: proposal.md
[unstable-rule-engine]: ../unstable-rule-engine/proposal.md
