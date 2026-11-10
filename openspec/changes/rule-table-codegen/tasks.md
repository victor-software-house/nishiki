# Tasks

## 1. The first language

- [ ] 1.1 Protocol Buffers' grammar in `tests/engine/grammars/`, then `unstable::codegen` with default matchers, and `codegen/` writing its tables; `oracle/proto-grammar.ts` is deleted.
  It needs [unstable-rule-engine][unstable-rule-engine] task 1.1, and the two fixes of the [default-matchers][default-matchers] design, section 2.5.
  This pull request creates `codegen/`, as the [rust-codegen][rust-codegen] design describes.
  → verify: `codegen --compare` shows equal tokens outside the start sets the pull request lists. `mise run codegen -- --check` and the differential and hostile-line tests of the default-matchers design pass in CI, and Identity shows no change

## 2. Families

- [ ] 2.1 One task per family, added as each starts.
  → verify: the proof of design section 4 passes, and the family's scripts are deleted
- [ ] 2.2 The last family deletes the shared scripts and `check:rule-grammars`.
  → verify: no `oracle/*-grammar.ts` remains

[default-matchers]: ../default-matchers/design.md
[rust-codegen]: ../rust-codegen/design.md
[unstable-rule-engine]: ../unstable-rule-engine/tasks.md
