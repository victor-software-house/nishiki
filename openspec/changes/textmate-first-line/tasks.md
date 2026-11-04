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

- [x] 2.1 One walk of the parsed tree builds the relaxed `Hir` and cuts at the first positive lookahead → verify: a one-off check finds the old and new `Hir`s equal on every shipped grammar's patterns, unit tests cover the cut, and the oracle passes
  - 2026-11-04, locally on a Linux box.
    The one-off check compared 3,152 pattern and line-kind pairs, and all were equal.
    `the_relaxed_form_ends_with_the_first_positive_lookahead` and `a_relaxed_form_needs_a_character` pass.
    Clippy, the `textmate::` unit tests and the oracle pass.
- [ ] 2.2 Measured → verify: first token falls, steady state is at or under main for every TextMate language, and memory is equal or lower

## 3. One parse in two stages

- [x] 3.1 Count how many of a list's patterns the first line tries, on CMake and PowerShell → verify: the counts are recorded here
  - 2026-11-04, a throwaway patch on the base commit of step 2.
    It logs each scanner's rule count and each distinct pattern a search tries, up to the first token.
    The input is each language's first bench-corpus file of each tier.
    The first line tries 1 to 6 patterns of the 21 to 131 its scanners list.
    That is under 10% everywhere, so decision 1 goes ahead.

    | Language | Tier | Scanners | Patterns listed | Patterns tried |
    |:--|:--|--:|--:|--:|
    | CMake | small | 1 | 21 | 2 |
    | CMake | medium | 1 | 21 | 1 |
    | PowerShell | small | 2 | 58 | 4 |
    | PowerShell | medium | 2 | 58 | 4 |
    | Bat | small | 2 | 32 | 3 |
    | Bat | medium | 2 | 27 | 3 |
    | Haskell | small | 3 | 69 | 6 |
    | Haskell | medium | 3 | 69 | 5 |
    | Mermaid | small | 2 | 32 | 2 |
    | Mermaid | medium | 2 | 32 | 2 |
    | Julia | small | 2 | 131 | 6 |
    | Julia | medium | 2 | 70 | 2 |
    | Scala | small | 2 | 79 | 3 |
    | Scala | medium | 2 | 76 | 2 |

- [ ] 3.2 The sketch stage and the program stage run from one parse, with the parsed tree kept until the program stage takes it → verify: the oracle and the Identity workflow pass
  - 2026-11-04, locally on a Linux box: the oracle passes.
    `every_listed_pattern_that_parses_compiles` passes.
    The Identity workflow runs on the pull request.
- [ ] 3.3 Measured, with the held trees' memory → verify: first token falls, and steady state and memory are at or under main for every TextMate language
  - 2026-11-04: first token falls by 3% to 23%.
    Memory is equal or lower for every TextMate language.
    Steady state is 1.0000 to 1.0015 by instruction count.
    `Regex` lost its first bytes, so a regex cell went from 152 to 144 bytes.
    LLVM indexes 144 bytes with `lea` and `shl`, where 152 bytes took one `imul`.
    Layouts of 128, 136 and 160 bytes did no better.
  - Lead, 2026-11-04: land it.
    The extra instruction is not extra time, since `imul` takes about three cycles.

## 4. The automaton's engines

- [ ] 4.1 Options 2 to 4 of the design, each measured against option 1 → verify: a table of first token and steady state per language and option is recorded here
- [ ] 4.2 The option that meets the bar lands → verify: first token falls, and steady state is at or under main for every TextMate language
