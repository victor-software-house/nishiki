# Proposal

## Motivation

48 per-language scripts, `oracle/<language>-grammar.ts`, write the rule-engine tables.
With the three files they share, they hold 18,848 lines, and most of those lines are copies (design, section 1).
A fix to an emitter has to be made in each copy.
The scripts read grammars and regular expressions a second time, beside the crate's compiler and parser.

## Changes

- `unstable::codegen` writes a language's rule-engine tables from its grammar.
  It reads the grammar with the crate's compiler and each expression with the crate's parser.
- Each language gives its extra values as one typed `RuleTables` value in `codegen/`.
- Generated names follow the naming rules, and stay within 40 characters.
- Languages move one family at a time.
  Each pull request deletes the scripts it replaces and keeps every token, outside the start sets it lists.

## Related changes

`rust-codegen` was split into five changes, so each fits the OpenSpec word budgets (lead, 2026-11-10):

- [`rust-codegen`][rust-codegen]: the `codegen/` crate, the fact files and the writers of numeric tables.
- [`grammar-to-rust`][grammar-to-rust]: the `codegen` and `format` features, `ToTokens` and `Grammar::to_rust`.
- [`unstable-rule-engine`][unstable-rule-engine]: `nishiki::unstable`, under `--cfg nishiki_unstable`.
- [`default-matchers`][default-matchers]: a matcher for each pattern, written from its expression, with the prototype that measures it.
- [`rule-table-codegen`][rule-table-codegen]: the rule-engine tables, their names, and the move of each family.

## Capabilities

### New Capabilities

- `generated-rule-tables`: the rule-engine tables of every language come from `unstable::codegen`.

## Impact

- `oracle/`: the 48 grammar scripts and their 3 shared files go.
- `tests/engine/grammars/`: the grammars of the 32 rule-engine languages it lacks.
  Each family appends its grammars to `oracle/engine-grammars.ts` and their licences to `oracle/licences.ts`.
- `codegen/src/languages/`: one `RuleTables` value per rule-engine language.
- Swift's matcher names, which pass 40 characters today.

[default-matchers]: ../default-matchers/proposal.md
[grammar-to-rust]: ../grammar-to-rust/proposal.md
[rule-table-codegen]: proposal.md
[rust-codegen]: ../rust-codegen/proposal.md
[unstable-rule-engine]: ../unstable-rule-engine/proposal.md
