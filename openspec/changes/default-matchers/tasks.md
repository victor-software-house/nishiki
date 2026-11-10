# Tasks

## 1. Default matchers

- [x] 1.1 Default matchers against hand matchers for Protocol Buffers, Zig and Lua ([research][research], section 1).
  → verify: Callgrind's lexing counts for both, from a box, in the research
  - 2026-11-10: on a box, both defaults against base [851351f0][base], at one codegen unit.
    The regex engine runs 2.56 to 4.54 times the hand matchers, and generated Rust 1.17 to 1.54 times.
    The differential test found 0 differences in 343 million matches.
    [The research note][default-matchers-note] has the method and every row.
- [ ] 1.2 The steps of the research's section 2, one at a time.
  → verify: each step's Callgrind rows and a clean differential test, in the research
  - 2026-11-10: steps 1, 2 and 2b measured on boxes, each with 0 differences.
    The steps go on until each language reaches 1.000.
  - 2026-11-10: the retry lines, on 2b, in release and debug builds:
    - Zig and Lua pass at fixture length.
    - Protocol Buffers' field line reaches the work limit at 101 bytes, one short of its longest fixture line.
      It is a finding with two fixes, in [`field-pattern-fixes`][field-pattern-fixes].
  - 2026-11-10: steps 3, 4 and 5 measured on boxes, each with 0 differences ([research][research], section 2).
    Step 5 reads 1.1002, 0.9108 and 1.1623.
    A cache of class runs raised Protocol Buffers against step 5, so it was dropped.
  - 2026-11-10: steps 6, 7 and 8 measured on boxes, each with 0 differences.
    [The research][research] has them in sections 2.2 and 2.3.
    Step 8 reads 1.0632, 0.8537 and 1.1039.
    Most of Lua's saving at step 8 came from inlining, not from the group-0 write.
  - Next: Lua's identifier patterns, the costliest shape left ([research][research], section 2.4).
- [ ] 1.3 The regex engine path gives an end pattern's back-reference the groups its begin pattern took.
  Then no pattern is left with hand code only.
  It covers Lua's five long-string and long-comment ends ([research][research], section 3).
  → verify: the differential test covers those five patterns, and Lua's matcher has no hand path left

The first language moves to default matchers in [rule-table-codegen][rule-table-codegen] task 1.1.

[base]: https://github.com/victor-software-house/nishiki-archive/commit/851351f064d86c429f4652af2c59af138dcf6ba2
[default-matchers-note]: ../../research/2026-11-10-default-matchers.md
[field-pattern-fixes]: ../field-pattern-fixes/design.md
[research]: research.md
[rule-table-codegen]: ../rule-table-codegen/tasks.md
