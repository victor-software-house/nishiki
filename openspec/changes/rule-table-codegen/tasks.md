# Tasks

## 1. The first language

- [ ] 1.1 Protocol Buffers moves to generated tables, as the first family slice, after tasks 2.1 to 2.3 (design section 5).
  → verify: design section 4's proof passes on Protocol Buffers' files.
  `mise run codegen -- --check` and the differential and hostile-line tests of the default-matchers design pass in CI.
  Identity shows no change.
  - Its grammar goes into `tests/engine/grammars/`, and `unstable::codegen` writes its tables with default matchers.
  - This pull request builds on `codegen/`, which [rust-codegen][rust-codegen] task 1.3 creates with the word tables, and deletes `oracle/proto-grammar.ts`.
  - It needs [unstable-rule-engine][unstable-rule-engine] task 1.1, and the two fixes of the [default-matchers][default-matchers] design, section 2.5.
- [ ] 1.2 The generator refuses to write a shipped table from a test-only grammar.
  It reads `oracle/licences.ts`, whose `testData` record lists those grammars ([PR 762][nishiki#762]).
  → verify: a test points the generator at a `testData` grammar and sees the refusal

## 2. The slices

The slices of design section 5, in order.
Each proves byte-identical output, as design section 4 says.

- [ ] 2.1 `unstable::codegen` gains `starts.rs`, the first-byte analysis, and the blank-run analysis moves there from `codegen/`.
  codegen's check compares the start sets with every `grammar.rs`.
  → verify: no file changes, and the check lists each start set that differs
- [ ] 2.2 The scripts take the start sets from codegen, if 2.1 lists any that differ.
  → verify: the diff holds only the listed sets, with Callgrind rows, and Identity passes
- [ ] 2.3 `flatten.rs` and `split.rs` build each pattern list, and codegen's check compares it with `codegen/patterns/`.
  → verify: no file changes, and the check passes for the 34 languages whose grammar `tests/engine/grammars/` holds
- [ ] 2.4 One task per family, added as each starts, in design section 5's order, after task 1.1.
  → verify: section 4's proof passes on the family's files, whose scripts, pattern and start files are deleted
  - Objective-C's family keeps its `$base` slots.
    The writer emits `Pattern::Base`, each rule's base list and `BASE_SLOTS`, as `oracle/rule-generator.ts` does with `baseSlots`.
- [ ] 2.5 The last slice deletes the shared scripts, `oracle/rule-generator.ts` among them, `codegen/patterns/`, `codegen/starts/` and `check:rule-grammars`.
  → verify: no `oracle/*-grammar.ts` or `oracle/rule-generator.ts` remains, and no generated file changes
  - It also closes the checks of [rust-codegen][rust-codegen] task 1.3 for the 13 `words.rs` files those scripts write.
    Each keeps its entries in order, and the size example's `.text` and `.rodata` keep their hashes.
    `check:literals` and `check:line-width` pass with `mise-tasks/lib/rust-writers.ts` and its writer exemption deleted.

[default-matchers]: ../default-matchers/design.md
[nishiki#762]: https://github.com/victor-software-house/nishiki-archive/pull/762
[rust-codegen]: ../archive/2026-11-12-rust-codegen/design.md
[unstable-rule-engine]: ../unstable-rule-engine/tasks.md
