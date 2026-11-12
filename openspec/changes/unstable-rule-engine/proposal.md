# Proposal

## Motivation

The rule engine is private.
A caller can run a grammar of their own only on the TextMate engine.
The rule engine's API still changes with each language that moves to it, so it cannot take semver's promises yet.

## Changes

- The rule engine becomes public under `--cfg nishiki_unstable`, as tokio does with `tokio_unstable`.
- `nishiki::unstable` holds the engine's grammar trait, its tables and its code writer.
- Its items are exempt from semver, and the semver check runs without the cfg.
- The engine's `Payload` becomes `CarriedState` as it becomes public.

## Related changes

`rust-codegen` was split into five changes, so each fits the OpenSpec word budgets (lead, 2026-11-10):

- [`rust-codegen`][rust-codegen]: the `codegen/` crate, the fact files and the writers of numeric tables.
- [`grammar-to-rust`][grammar-to-rust]: the `codegen` and `format` features, `ToTokens` and `Grammar::to_rust`.
- [`unstable-rule-engine`][unstable-rule-engine]: `nishiki::unstable`, under `--cfg nishiki_unstable`.
- [`default-matchers`][default-matchers]: a matcher for each pattern, written from its expression, with the prototype that measures it.
- [`rule-table-codegen`][rule-table-codegen]: the rule-engine tables, their names, and the move of each family.

## Capabilities

### New Capabilities

- `unstable-rule-engine`: the rule engine under `nishiki::unstable`.

## Impact

- `src/unstable.rs`: a new module, compiled only with `--cfg nishiki_unstable`.
- `src/rules.rs`: its trait and tables become reachable through `nishiki::unstable`.
- `Cargo.toml`: the cfg's `check-cfg` entry and the docs.rs settings.
- CI: one more build and test with the cfg.
- `AGENTS.md`: the semver exemption of `nishiki::unstable`.

[default-matchers]: ../default-matchers/proposal.md
[grammar-to-rust]: ../grammar-to-rust/proposal.md
[rule-table-codegen]: ../rule-table-codegen/proposal.md
[rust-codegen]: ../archive/2026-11-12-rust-codegen/proposal.md
[unstable-rule-engine]: proposal.md
