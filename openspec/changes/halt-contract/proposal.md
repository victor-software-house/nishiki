# Proposal

## Motivation

The hand lexers embed one another through the embedded language's `lex_until` and a `Halt` ([`src/language.rs`][language]).
The [`engine-handoff` change][handoff] makes nishiki's TextMate engine the embedding side of that contract.
That change runs on the two halt tests main had then, `end` and `after`.
Some embedders need more:

- ERB's `<%` is an injection, so it must halt the embedded language at any depth.
- Vue enters TypeScript at its `expression` rule, not at its root.
- Every language should embed another through one entry point.

This change was part of the `engine-handoff` change until that change was split to fit the OpenSpec word budgets.

## Changes

- One halt contract, `Halts`, holds three tests:
  - `before`: the embedding rule's end and the patterns listed before the include;
  - `after`: the patterns listed after the include;
  - `any`: an injection at any depth, as ERB's `<%`.

  Each test is a closure.
  An engine end with back-references can then halt a hand lexer.
- `Language::lex_until`, which [#159][pr159] adds, takes `&Halts`.
  Every language whose row sets `halts: true` honours all three tests.
- An embedded language can be entered at its root or at an entry it offers.
  An example is TypeScript's `expression`, for Vue.

The design is in [design.md](design.md).

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `engine-handoff`, added by the `engine-handoff` change: an injection halts at any depth, a language can start at an entry, and one entry point serves every embedding.

## Impact

- `src/language.rs`: `Halts`, `Entry`, `Language::lex_until` and `Language::start_embedded`.
- The rule engine and TypeScript: all three tests, the comment skip for `any`, the full state at an `any` halt.
- Each language module whose row sets `halts: true`: its `lex_until` on `&Halts`.

[language]: https://github.com/victor-software-house/nishiki-archive/blob/8f346d8e/src/language.rs
[handoff]: ../archive/2026-11-10-engine-handoff/proposal.md
[pr159]: https://github.com/victor-software-house/nishiki-archive/pull/159
