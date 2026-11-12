# Tasks

## 1. Default matchers

- [x] 1.1 Default matchers against hand matchers for Protocol Buffers, Zig and Lua ([research][research], section 1).
  → verify: Callgrind's lexing counts for both, from a box, in the research
  - 2026-11-10: on a box, both defaults against base [851351f0][base], at one codegen unit.
    The regex engine runs 2.56 to 4.54 times the hand matchers, and generated Rust 1.17 to 1.54 times.
    The differential test found 0 differences in 343 million matches.
    [The research note][default-matchers-note] has the method and every row.
- [x] 1.2 The steps of the research's section 2, one at a time.
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
  - 2026-11-10: steps 9, 10 and 11 measured on boxes, each with 0 differences ([research][research], section 2).
    Step 11 reads 1.0409, 0.8339 and 1.0352.
    - Step 10 cuts each entry to Lua's `matcher` from 33 to 23 instructions, but raises Zig at 16 units.
    - Step 11, the arm drop, clears the 101-byte field line.
  - 2026-11-10: step 12 measured on a box, with 0 differences ([research][research], section 2.6).
    It reads 1.0402, 0.8265 and 1.0345, and brings Zig at 16 units back below step 9.
  - 2026-11-11: steps 13 and 14 measured on a box, each with 0 differences ([research][research], section 2).
    Step 14 reads 1.0346, 0.8014 and 1.0028.
    Lua's patterns `m127`, `m128` and `m130` fall from 2.39 to 1.98 million instructions.
  - 2026-11-11: fix 2 and steps 15, 16 and 17 measured on boxes ([research][research], section 2).
    Each found 0 differences.
    Step 17 reads 0.9963, 0.7992 and 0.9700, measured directly against the hand matchers.
    Every row of the three measures is at or below 1.000, at one and at 16 codegen units.
- [ ] 1.3 The regex engine path gives an end pattern's back-reference the groups its begin pattern took.
  Then no pattern is left with hand code only.
  It covers Lua's five long-string and long-comment ends ([research][research], section 3).
  → verify: the differential test covers those five patterns, and Lua's matcher has no hand path left
- [ ] 1.4 After a failed try, a skipping pattern reads its run again only up to `k + 2` bytes.
  It reads on to the run's end only to keep the failure.
  It follows up fix 2's cost ([`field-pattern-fixes`][field-pattern-fixes], section 3.4).
  → verify: `m36` costs fewer instructions per try than at `30390626`, with 0 differences and both field lines uncut
- [ ] 1.5 The scan passes a mixed run of `\s` characters that starts with a wide space in one step, where the grammar allows it.
  An analysis per grammar decides where it does.
  It extends the blank-run analysis to runs that start with a wide space.
  - [#6][pr-6] passed such a run whole.
    A space-only pattern such as `[ \t]+$` then lost its try at a later space.
  - [#11][pr-11] passes a run of one repeated wide space only.
    So the Protocol Buffers limit line, `"\u{a0} "` repeated 4,000 times, ends at the work limit.

  → verify: on a box, against the base:
  - the no-break-space line stays at 32,008 steps and the U+3000 line at 48,008;
  - the `"\u{a0} "` line ends under the work limit, in steps linear in its length;
  - the Shiki differential over three seeds shows no new difference;
  - every Callgrind lexing row reads at most 1.000, or the pull request explains it.

The first language moves to default matchers in [rule-table-codegen][rule-table-codegen] task 1.1.

[base]: https://github.com/victor-software-house/nishiki-archive/commit/851351f064d86c429f4652af2c59af138dcf6ba2
[default-matchers-note]: ../../research/2026-11-10-default-matchers.md
[field-pattern-fixes]: ../field-pattern-fixes/design.md
[pr-6]: https://github.com/victor-software-house/nishiki/pull/6
[pr-11]: https://github.com/victor-software-house/nishiki/pull/11
[research]: research.md
[rule-table-codegen]: ../rule-table-codegen/tasks.md
