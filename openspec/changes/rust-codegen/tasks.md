# Tasks

Task 1.3's word tables create `codegen/`, ahead of [rule-table-codegen][rule-table-codegen] task 1.1 (design section 7).

## 1. The writer

- [ ] 1.1 `AGENTS.md`, `CODING_STANDARDS.md` and `oracle/AGENTS.md` gain the rules of the proposal and how to move one language.
  → verify: review
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
- [ ] 1.5 Each language alone builds without dead code, so `check:features` drops `-A dead_code`.
  `lang-c` alone still warns 13 times, for these items:
  - `start_embedded` in `c.rs`, `rules.rs` and `language.rs`;
  - `Halt::halt` and `halt_before`;
  - the field `first_token`, and the field `spent` with the `Steps` methods `take_back`, `add` and `into_spent`;
  - the `Grammar` items `BLANK_RUNS`, `deep_halts_open`, `DEFERS_NESTED`, `defers_nested`, `SKIPS_PLACES` and `next_place`, and `BlankRuns`;
  - the variants `Backref`, `Embed` and `Lowered`, and `Lexer::char_end`.

  → verify: `check:features` passes without `-A dead_code`.
- [ ] 1.6 `check:features` builds each test gate of two or more features, one line per gate.
  Only `--all-features` compiles that test code now.
  A narrower gate inside it then goes unbuilt.
  The generator writes the lines, or a check compares them with the cfgs.
  The gates found on 2026-11-12:
  - TypeScript's tests, with its four dialects;
  - Svelte's tests, with `lang-html` and `lang-markdown`;
  - `scope.rs`'s tests, with `all`;
  - 34 probe test gates in 31 files, each with its language and `json`;
  - CoffeeScript's probe tests, with `json` and `lang-svelte` or the TextMate engine.

  → verify: `check:features` has a line for every test gate of two or more features, and fails when one lacks it.
  - 2026-11-12: a build of the four TypeScript dialects alone failed to resolve `start_embedded` in their tests.
    [Pull request #12][pr-12] gates those tests on the function's own gate.

[pr-12]: https://github.com/victor-software-house/nishiki/pull/12
[rule-table-codegen]: ../rule-table-codegen/tasks.md
