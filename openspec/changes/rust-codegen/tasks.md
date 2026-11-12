# Tasks

Task 1.3's word tables create `codegen/`, ahead of [rule-table-codegen][rule-table-codegen] task 1.1 (design section 7).

## 1. The writer

- [x] 1.1 `AGENTS.md`, `CODING_STANDARDS.md` and `oracle/AGENTS.md` gain the rules of the proposal and how to move one language.
  → verify: review
  - 2026-11-12: [pull request #35][pr-35] adds `CODING_STANDARDS.md` 3.11, with the steps the seven word scripts took to move.
    Reviewed by the lead, 2026-11-12, [#35][pr-35].
- [x] 1.2 Unicode sets, case folds and classes are written from `oracle/facts/`.
  → verify: `codegen --compare` shows equal tokens for the three files
  - 2026-11-11: `codegen/src/classes.rs` writes `src/classes/table.rs` from `classes.json` and `unicode-sets.json`.
    Each table holds its set among the points `classes.json` lists, with the same ranges as before.
  - 2026-11-11: the lookups call `contains` in the hand-written `src/classes.rs`.
    The generated file holds only data.
    `oracle/classes.ts` keeps the five sets it measures by tokenizing and drops its twelve Oniguruma scans, which `unicode-sets.json` already holds.
  - 2026-11-11: `codegen/src/folds.rs` writes `src/textmate/regex/folds.rs` from `case-folds.json`.
    It derives the next member of each orbit, the folds in canonical characters, and the bytes of the wide characters they involve.
  - 2026-11-11: the tokens of `folds.rs` are the same as before; the numbers now fill lines of up to 100 columns.
    `oracle/case-folds.ts` keeps its measuring and drops the derivation and its Rust template.
  - 2026-11-11: `codegen/src/unicode.rs` writes `src/textmate/regex/unicode.rs` from `unicode-sets.json`, with the same tables.
    The `lookup` match becomes `NAMES`, a sorted list of each name with its set, which `ast::property_named` searches by hand.
    On a box, each file's tables hold the tokens the Bun scripts wrote.
- [ ] 1.3 The word scripts and `fence-stops.ts` move into `codegen/`.
  `numbers.rs` fills each word table into lines of up to 100 columns, with no backslash line continuation.
  → verify: each `words.rs` keeps its entries in order, and the size example's `.text` and `.rodata` keep their hashes.
  `check:literals` passes on it without the writer exemption, and the fence check fails on a wrong `pops_host` list.
  - 2026-11-10: `oracle/rule-generator.ts` writes the Objective-C, TeX and LaTeX blobs as `concat!` of one literal per line, with the same bytes.
    `check:literals` now checks every generated Rust file.
    The move into `codegen/` stays open.
  - 2026-11-10: `codegen/src/fence_stops.rs` replaces `fence-stops.ts`.
    It reads the fence grammars committed under `tests/engine/grammars/`, and `mise run codegen -- --check` runs it.
    Its test `compare_fails_on_a_different_list` covers a wrong `pops_host` list.
  - 2026-11-11: the standalone word scripts are in `codegen/src/words/`.
    The other `words.rs` files come from the `oracle/<language>-grammar.ts` scripts, which the families of [rule-table-codegen][rule-table-codegen] move.
    The verify waits for them: until then `check:literals` and `check:line-width` still leave those scripts alone.
- [x] 1.4 The library `codegen/` builds, `textmate` and `lang-c` with `--cfg nishiki_unstable`, warns of nothing, and so does `lang-vue` with `lang-markdown`.
  Each item those builds left unused takes a cfg gate at its source, named for the features whose modules use it.
  → verify: `check:features` builds both libraries with warnings denied, dead code included.
  - 2026-11-10: the private features `_class-widths`, `_blank-runs-to-last` and `_capture-names` hold the helpers that one set of languages shares.
    On a box, `check:features`, lint, `check:unstable`, `codegen:lint`, `codegen:test` and the tests pass.
- [x] 1.5 An item compiles only under the features of the language, engine, embedding or test code that uses it.
  [Design section 6.3][design-gates] holds the rule.
  Shared helpers stay ungated, and `check:features` keeps `-A dead_code`.
  → verify: on a box, with `-W dead_code`, no single-language or TextMate set reports a dead item that one language or engine owns.
  - 2026-11-12: [pull request #7][pr-7] gates Vue's and PHP's own code, and [pull request #8][pr-8] the language-specific items and test helpers.
  - 2026-11-12: [pull request #16][pr-16] gates the line context's fields, the keyed scope lists and the arena's inserts on their readers.
  - 2026-11-12: on a box, on main at 77313ff3, 86 sets built with `-W dead_code`: the 81 of `check:features`, four languages with `textmate`, and the four TypeScript dialects together.
    All 86 built, with 752 dead-code warnings.
    Each names a shared helper of `src/rules/`, `src/classes/`, `src/limits.rs` or `src/test_support/probe.rs`.
    Two or more languages call each one, or another shared helper does.
- [x] 1.6 `check:test-gates` builds each test gate of two or more features, one line per gate.
  Before it, only `--all-features` compiled that test code.
  A narrower gate inside it then went unbuilt.
  `codegen/src/test_gates.rs` reads the cfgs of `src/` and writes the lines.
  The gates found on 2026-11-12:
  - TypeScript's tests, with its four dialects;
  - Svelte's tests, with `lang-html` and `lang-markdown`;
  - `scope.rs`'s tests, with `all`;
  - 34 probe test gates in 31 files, each with its language and `json`;
  - CoffeeScript's probe tests, with `json` and `lang-svelte` or the TextMate engine.

  → verify: `check:test-gates` has a line for every test gate of two or more features, and `codegen:check` fails when one lacks it.
  - 2026-11-12: a build of the four TypeScript dialects alone failed to resolve `start_embedded` in their tests.
    [Pull request #12][pr-12] gates those tests on the function's own gate.
  - 2026-11-12: the generator writes 111 builds, which cover every gate above.
    On a box, all 111 passed in 192 s.

[design-gates]: design.md#63-feature-gates-and-dead-code
[pr-12]: https://github.com/victor-software-house/nishiki/pull/12
[pr-16]: https://github.com/victor-software-house/nishiki/pull/16
[pr-35]: https://github.com/victor-software-house/nishiki/pull/35
[pr-7]: https://github.com/victor-software-house/nishiki/pull/7
[pr-8]: https://github.com/victor-software-house/nishiki/pull/8
[rule-table-codegen]: ../rule-table-codegen/tasks.md
