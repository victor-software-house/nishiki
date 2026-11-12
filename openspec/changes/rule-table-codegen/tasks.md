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
  - [ ] 2.3.1 The model in `tables.rs`, `flatten.rs`, and the check for Go, Protocol Buffers, Python, Sass and Scala.
    → verify: the check passes for those five
  - [ ] 2.3.2 `split.rs`, and the names a language gives its patterns: Java, Dart, PowerShell, VB and Jinja.
    → verify: the check passes for those five, and for the languages before them
  - [ ] 2.3.3 Includes that add nothing, and where a list that reaches itself stops: Common Lisp, GDScript and Less.
    → verify: the check passes for those three, and for the languages before them
  - [ ] 2.3.4 Includes of other grammars: Elixir, Julia and Lua.
    → verify: the check passes for those three, and for the languages before them
  - [ ] 2.3.5 The scripts with logic of their own: CSS, Assembly, Swift, CoffeeScript, HAML, Ruby, Perl, SCSS, C++, C#, Bat and R.
    → verify: the check passes for those twelve, and for the languages before them
  - [ ] 2.3.6 `oracle/rule-generator.ts`'s five languages, and PHP.
    → verify: the check passes for all 34
- [ ] 2.4 One task per family, added as each starts, in design section 5's order, after task 1.1.
  → verify: section 4's proof passes on the family's files, whose scripts, pattern and start files are deleted
  - Objective-C's family keeps its `$base` slots.
    The writer emits `Pattern::Base`, each rule's base list and `BASE_SLOTS`, as `oracle/rule-generator.ts` does with `baseSlots`.
- [ ] 2.5 The last slice deletes the shared scripts, `oracle/rule-generator.ts` among them, `codegen/patterns/`, `codegen/starts/` and `check:rule-grammars`.
  → verify: no `oracle/*-grammar.ts` or `oracle/rule-generator.ts` remains, and no generated file changes
  - It also closes the checks of [rust-codegen][rust-codegen] task 1.3 for the 13 `words.rs` files those scripts write.
    Each keeps its entries in order, and the size example's `.text` and `.rodata` keep their hashes.
    `check:literals` and `check:line-width` pass with `mise-tasks/lib/rust-writers.ts` and its writer exemption deleted.
  - 2026-11-12: each grammar's impl of `rules::WideSpaceLookbehind` is in `src/rules/blank_runs.rs`, which `codegen/src/blank_runs.rs` writes.
    Each `grammar.rs` still reads its `MIXED_BLANK_RUNS` from a constant there.
    Once Rust writes `grammar.rs`, that line can move into the same Rust output.

[default-matchers]: ../default-matchers/design.md
[nishiki#762]: https://github.com/victor-software-house/nishiki-archive/pull/762
[rust-codegen]: ../archive/2026-11-12-rust-codegen/tasks.md
[unstable-rule-engine]: ../unstable-rule-engine/tasks.md
