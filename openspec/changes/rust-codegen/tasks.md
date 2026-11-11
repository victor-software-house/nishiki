# Tasks

Task 1.3's word tables create `codegen/`, ahead of [rule-table-codegen][rule-table-codegen] task 1.1 (design section 7).

## 1. The writer

- [ ] 1.1 `AGENTS.md`, `CODING_STANDARDS.md` and `oracle/AGENTS.md` gain the rules of the proposal and how to move one language.
  → verify: review
- [ ] 1.2 Unicode sets, case folds and classes are written from `oracle/facts/`.
  → verify: `codegen --compare` shows equal tokens for the three files
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

[rule-table-codegen]: ../rule-table-codegen/tasks.md
