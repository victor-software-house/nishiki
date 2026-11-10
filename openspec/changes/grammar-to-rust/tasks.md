# Tasks

## 1. Measurements

- [x] 1.1 The build time and crate count of each feature (design, section 5).
  → verify: the numbers from a box, in design section 5
  - 2026-11-10: clean release builds on a box: 58 s and 2 crates with `all`, 78 s and 5 with `codegen`, 87 s and 7 with `format`.
    The build-script example does not exist yet, so its program size moved to task 2.2, whose `check:size` covers it.

## 2. Groundwork and the API

- [x] 2.1 `compile::UnlinkedGrammar` holds `Cow` data, and the regex parser's entry and tree are `pub(crate)`.
  → verify: CI passes, and Identity shows no change
  - 2026-11-10: [#688][pr-688] gives `compile::UnlinkedGrammar` its `Cow` data.
  - 2026-11-10: [#701][pr-701] makes `regex::parse::parse`, `Parsed`, `Dialect` and `regex::ast::Node` `pub(crate)`.
    The types the tree holds become `pub(crate)` with it; their methods keep their visibility.
- [x] 2.2 The `codegen` and `format` features, `ToTokens` for `Grammar` and `Grammar::to_rust`, with doctests and a build-script example.
  → verify: the doctests and the example run in CI, and `check:size` shows no change for any tier
  - 2026-11-10: on a box, lint, tests, doctests, docs and `cargo deny` pass.
    `syn` reads the file `to_rust` writes for every engine grammar.
    The included Rust gives the same `to_bytes` as its JSON.
    The `json` and `all` tiers stay byte for byte the same.
    The two example programs become new tiers.

[pr-688]: https://github.com/victor-software-house/nishiki-archive/pull/688
[pr-701]: https://github.com/victor-software-house/nishiki-archive/pull/701
