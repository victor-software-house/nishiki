# Tasks

## 1. The first language

- [ ] 1.1 Protocol Buffers moves to generated tables.
  → verify: `codegen --compare` shows equal tokens outside the start sets the pull request lists.
  `mise run codegen -- --check` and the differential and hostile-line tests of the default-matchers design pass in CI.
  Identity shows no change.
  - Its grammar goes into `tests/engine/grammars/`, and `unstable::codegen` writes its tables with default matchers.
  - This pull request builds on `codegen/`, which [rust-codegen][rust-codegen] task 1.3 creates with the word tables, and deletes `oracle/proto-grammar.ts`.
  - It needs [unstable-rule-engine][unstable-rule-engine] task 1.1, and the two fixes of the [default-matchers][default-matchers] design, section 2.5.

## 2. Families

- [ ] 2.1 One task per family, added as each starts.
  → verify: the proof of design section 4 passes, and the family's scripts are deleted
- [ ] 2.2 The last family deletes the shared scripts and `check:rule-grammars`.
  → verify: no `oracle/*-grammar.ts` remains

[default-matchers]: ../default-matchers/design.md
[rust-codegen]: ../rust-codegen/design.md
[unstable-rule-engine]: ../unstable-rule-engine/tasks.md
