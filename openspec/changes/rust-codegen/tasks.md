# Tasks

## 1. Measurements

- [x] 1.1 Default matchers against hand matchers for Protocol Buffers, Zig and Lua (design, section 12.1).
  → verify: Callgrind's lexing counts for both, from a box, in design section 12.1
  - 2026-11-10: on a box, both defaults against base [851351f0][base], at one codegen unit.
    The regex engine runs 2.56 to 4.54 times the hand matchers, and generated Rust 1.17 to 1.54 times.
    The differential test found 0 differences in 343 million matches.
    [The research note][default-matchers-note] has the method and every row.
- [x] 1.2 The build time and crate count of each feature (design, section 12.2).
  → verify: the numbers from a box, in design section 12.2
  - 2026-11-10: clean release builds on a box: 58 s and 2 crates with `all`, 78 s and 5 with `codegen`, 87 s and 7 with `format`.
    The build-script example does not exist yet, so its program size moved to task 2.2, whose `check:size` covers it.
- [ ] 1.3 The steps of design section 12.1, one at a time.
  → verify: each step's Callgrind rows and a clean differential test, in a research note
  - 2026-11-10: steps 1, 2 and 2b measured on boxes, each with 0 differences.
    The steps go on until each language reaches 1.000.
    The retry lines at fixture length have not run yet.
    Still to come: the first-byte test, the groups recorded, the `\b` test and a flat dispatch (lead, 2026-11-10).

## 2. Groundwork

- [ ] 2.1 `compile::UnlinkedGrammar` holds `Cow` data, and the regex parser's entry and tree are `pub(crate)`.
  → verify: CI passes, and Identity shows no change
- [ ] 2.2 The `codegen` and `format` features, `ToTokens` for `Grammar` and `Grammar::to_rust`, with doctests and a build-script example.
  → verify: the doctests and the example run in CI, and `check:size` shows no change for any tier
- [ ] 2.3 `nishiki::unstable` and its cfg setup, with the docs.rs settings and a CI build with the cfg; `Payload` becomes `CarriedState`.
  → verify: the docs.rs preview shows the module with its badge, and the semver check runs without the cfg

## 3. The writer

- [ ] 3.1 Protocol Buffers' grammar in `tests/engine/grammars/`, then `unstable::codegen` with default matchers, and `codegen/` writing its tables; `oracle/proto-grammar.ts` is deleted.
  → verify: `codegen --compare` shows equal tokens outside the start sets the pull request lists. `mise run codegen -- --check` and the differential and hostile-line tests of design section 5.6 pass in CI, and Identity shows no change
- [ ] 3.2 `AGENTS.md`, `CODING_STANDARDS.md` and `oracle/AGENTS.md` gain the rules of the proposal and how to move one language.
  → verify: review
- [ ] 3.3 Unicode sets, case folds and classes are written from `oracle/facts/`.
  → verify: `codegen --compare` shows equal tokens for the three files

## 4. Families

- [ ] 4.1 One task per family, added as each starts.
  → verify: the proof of design section 11 passes, and the family's scripts are deleted
- [ ] 4.2 The last family deletes the shared scripts and `check:rule-grammars`.
  → verify: no `oracle/*-grammar.ts` remains
- [ ] 4.3 The word scripts and `fence-stops.ts` move into `codegen/`.
  → verify: each `words.rs` keeps its tokens, and the fence check fails on a wrong `pops_fence` list

[default-matchers-note]: ../../research/2026-11-10-default-matchers.md
[base]: https://github.com/victor-software-house/nishiki-archive/commit/851351f064d86c429f4652af2c59af138dcf6ba2
