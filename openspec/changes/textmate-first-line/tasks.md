# Tasks

Every step is measured against its base on a Linux box.
Builds use one codegen unit and `MBX_DISABLE=1`.
Each measurement covers first token for every TextMate language in both tiers.
It covers steady state with `mise run callgrind`'s harness.
It covers `examples/memory.rs` and `size -A`.
Each side builds three times from clean target directories.
Each side must repeat across its three builds.

## 1. Design

- [x] 1.1 The lead agrees the design → verify: their reply recorded here
  - Lead, 2026-11-04: approved.
    The order is design decision 2, then 1, then 3, one pull request each.
    Decision 1 is skipped if task 3.1 finds that the first line tries most of a list's patterns.
    For decision 3, the lead prefers option 4.
    The lead asks for steady state on every engine language.

## 2. The relaxed `Hir` in one walk

- [x] 2.1 One walk of the parsed tree builds the relaxed `Hir` and cuts at the first positive lookahead.
  → verify: a one-off check finds the old and new `Hir`s equal on every shipped grammar's patterns.
  Unit tests cover the cut, and the oracle passes.
  - 2026-11-04, locally on a Linux box.
    The one-off check compared 3,152 pattern and line-kind pairs, and all were equal.
    `the_relaxed_form_ends_with_the_first_positive_lookahead` and `a_relaxed_form_needs_a_character` pass.
    Clippy, the `textmate::` unit tests and the oracle pass.
- [ ] 2.2 Measured → verify: first token falls, steady state is at or under main for every TextMate language, and memory is equal or lower

## 3. One parse in two stages

- [x] 3.1 Count how many of a list's patterns the first line tries, on CMake and PowerShell → verify: the counts are recorded here
  - 2026-11-04: the first line tries under 10% of the patterns its scanners list, so decision 1 goes ahead ([measurements][measurements], section 3.1).
- [ ] 3.2 The sketch stage and the program stage run from one parse, with the parsed tree kept until the program stage takes it.
  → verify: the oracle and the Identity workflow pass
  - 2026-11-04, locally on a Linux box: the oracle passes.
    `every_listed_pattern_that_parses_compiles` passes.
    The Identity workflow runs on the pull request.
- [ ] 3.3 Measured, with the held trees' memory.
  → verify: first token falls, and steady state and memory are at or under main for every TextMate language
  - 2026-11-04: first token falls by 3% to 23%, and steady state is 1.0000 to 1.0015 ([measurements][measurements], section 3.3).
  - Lead, 2026-11-04: land it.
    The extra instruction is not extra time, since `imul` takes about three cycles.

## 4. The automaton's engines

- [x] 4.1 Options 2 to 4 of the design, each measured against option 1.
  → verify: a table of first token and steady state per language and option is recorded here
  - 2026-11-04: first token and steady state for options 2 to 6b, each against option 1 ([measurements][measurements], section 4.1).
- [ ] 4.2 The option that meets the bar lands → verify: first token falls, and steady state is at or under main for every TextMate language
  - 2026-11-04: option 6b cuts first token by 0.3% to 60%, and steady state is at most 1.0013 ([measurements][measurements], section 4.2).
  - Lead, 2026-11-04: land it, with that split in the pull request.

[measurements]: ../../research/2026-11-10-textmate-first-line-measurements.md
