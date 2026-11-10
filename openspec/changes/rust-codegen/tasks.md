# Tasks

## 1. Measurements

- [ ] 1.1 Default matchers against hand matchers for Protocol Buffers, Zig and Lua (design, section 12.1).
  → verify: Callgrind's lexing counts for both, from a box, in design section 12.1
- [ ] 1.2 The build time, crate count and program size of each feature (design, section 12.2).
  → verify: the numbers from a box, in design section 12.2

## 2. Groundwork

- [ ] 2.1 `compile::UnlinkedGrammar` holds `Cow` data, and the regex parser's entry and tree are `pub(crate)`.
  → verify: CI passes, and Identity shows no change
- [ ] 2.2 The `codegen` and `format` features, `ToTokens` for `Grammar` and `Grammar::to_rust`, with doctests and a build-script example.
  → verify: the doctests and the example run in CI, and `check:size` shows no change for any tier
- [ ] 2.3 `nishiki::unstable` and its cfg setup, with the docs.rs settings and a CI build with the cfg; `Payload` becomes `CarriedState`.
  → verify: the docs.rs preview shows the module with its badge, and the semver check runs without the cfg

## 3. The writer

- [ ] 3.1 `unstable::codegen` with default matchers, and `codegen/` writing Protocol Buffers' tables; `oracle/proto-grammar.ts` is deleted.
  → verify: `codegen --compare` shows equal tokens, `mise run codegen -- --check` passes in CI, and Identity shows no change
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
