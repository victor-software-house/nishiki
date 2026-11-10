# Proposal

## Motivation

Every pattern of a rule-engine grammar has a matcher written by hand.
A caller's grammar has none, and each new language costs a matcher per pattern.
Running every pattern on the crate's regex engine costs 2.56 to 4.54 times the hand matchers.

## Changes

- Each pattern gets a default matcher: Rust code written from its parsed expression.
- A pattern the writer cannot express exactly falls back to the crate's regex engine.
- A generated matcher matches exactly what Oniguruma matches, and spends the line's work budget in its loops.
- CI checks each generated matcher against the regex engine, and runs hostile lines under the default limits.
- A prototype measures the generated code against the hand matchers, one step at a time.
  Hand matchers go once a language's generated code reaches them.

## Related changes

`rust-codegen` was split into five changes, so each fits the OpenSpec word budgets (lead, 2026-11-10):

- [`rust-codegen`][rust-codegen]: the `codegen/` crate, the fact files and the writers of numeric tables.
- [`grammar-to-rust`][grammar-to-rust]: the `codegen` and `format` features, `ToTokens` and `Grammar::to_rust`.
- [`unstable-rule-engine`][unstable-rule-engine]: `nishiki::unstable`, under `--cfg nishiki_unstable`.
- [`default-matchers`][default-matchers]: a matcher for each pattern, written from its expression, with the prototype that measures it.
- [`rule-table-codegen`][rule-table-codegen]: the rule-engine tables, their names, and the move of each family.

## Capabilities

### New Capabilities

- `default-matchers`: a matcher for every pattern, generated from its regular expression.

## Impact

- `unstable::codegen`: the matcher writer and its analysis.
- `src/rules/`: the default matcher's fallback to the regex engine.
- `tests/it/limits/`: hostile lines for each generated language.
- Each moved language's `matchers/` directory, once its generated code measures as fast.

[default-matchers]: proposal.md
[grammar-to-rust]: ../grammar-to-rust/proposal.md
[rule-table-codegen]: ../rule-table-codegen/proposal.md
[rust-codegen]: ../rust-codegen/proposal.md
[unstable-rule-engine]: ../unstable-rule-engine/proposal.md
