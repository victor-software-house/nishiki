# Research

The prototype behind [the design][design], measured on boxes.

## 1. Generated and regex-engine matchers

A prototype branch, `research/default-matchers`, swaps each hand matcher of Protocol Buffers, Zig and Lua for a default.
[The research note][default-matchers-note] links it at a pinned commit and gives the method.

Callgrind counts the lexing pass over each language's oracle fixtures at one codegen unit (`bench/AGENTS.md` rule 12), in millions of instructions:

| Language | Hand | Regex engine | Ratio | Generated | Ratio |
|:--|--:|--:|--:|--:|--:|
| Protocol Buffers | 4.5 | 11.5 | 2.5644 | 5.8 | 1.2958 |
| Zig | 25.0 | 86.5 | 3.4591 | 29.4 | 1.1746 |
| Lua | 27.5 | 124.7 | 4.5401 | 42.2 | 1.5374 |

- The generated rows are from the prototype before Protocol Buffers' `OptionStmt` was generated.
  Task 1.2 brings the row with it.
- Allocations do not change on either path.
- 243 of the 245 patterns are generated.
  Protocol Buffers' `String` has a back-reference.
  Zig's `NumbersAt6` repeats a group whose arms both start with `e`.
  Both run on the regex engine.

The differential test of [design section 2.3][design] found no difference:

| Language | Fixture lines | Fuzzed lines | Positions | Matches compared | Differences |
|:--|--:|--:|--:|--:|--:|
| Protocol Buffers | 3,374 | 2,000 | 207,300 | 8,084,700 | 0 |
| Zig | 43,320 | 2,000 | 1,447,153 | 79,593,415 | 0 |
| Lua | 40,717 | 2,000 | 1,680,199 | 255,390,248 | 0 |

## 2. The steps

Task 1.2 changes the generated code one step at a time.
Each step is backed by a profile per function and followed by the differential test.
Lexing ratios against the hand matchers, at one codegen unit:

| Step | Protocol Buffers | Zig | Lua |
|:--|--:|--:|--:|
| generated, before the steps | 1.2568 | 1.1746 | 1.5374 |
| 1. a node that holds only ASCII tests one byte | 1.2418 | 1.0748 | 1.4057 |
| 2. steps spent at each byte read | 1.2556 | 1.2468 | 1.6277 |
| 2b. steps spent in loops, replacing 2 | 1.2604 | 1.1062 | 1.4450 |
| hand matchers out of line, on 2 | 1.2565 | 1.2500 | 1.6189 |
| hand matchers out of line, on 2b | 1.2604 | 1.1094 | 1.4362 |
| 3. `\b` through `Chars::boundary` | 1.2592 | 0.9878 | 1.2977 |
| 4. a flat dispatch | 1.1351 | 0.9878 | 1.2395 |
| 5. a greedy repeat that gives back nothing | 1.1002 | 0.9108 | 1.1623 |
| 6. only the groups nishiki reads recorded | 1.0998 | 0.9027 | 1.1476 |
| 7. a back-reference to one character | 1.0696 | 0.9027 | 1.1476 |
| 8. group 0 left to the lexer | 1.0632 | 0.8537 | 1.1039 |
| 9. `record` always inlined | 1.0632 | 0.8516 | 1.0870 |
| 10. each pattern's code in its own function, out of line | 1.0495 | 0.8339 | 1.0351 |
| 11. an optional character its run covers dropped | 1.0409 | 0.8339 | 1.0352 |
| 12. a small pattern with no loop or lookbehind offered inline | 1.0402 | 0.8265 | 1.0345 |
| 13. a space run read with the lexer's `space_end` | 1.0348 | 0.8234 | 1.0256 |
| 14. one side of a word boundary tested when the other is known | 1.0346 | 0.8014 | 1.0028 |
| fix 2 of `field-pattern-fixes`, on 14 | 1.0373 | 0.8014 | 1.0028 |
| 15. an alternation of literals dispatched on its first byte | 1.0373 | 0.7991 | 0.9864 |
| 16. `\G` tested before the pattern's call | 1.0373 | 0.7991 | 0.9702 |
| 17. any leading position anchor tested before the call | 0.9964 | 0.7991 | 0.9700 |

- The first row for Protocol Buffers has `OptionStmt` generated.
- Step 2 kept the `\b` test out of line.
  Step 2b brings it back in line and spends once per loop.
- Moving the hand matchers out of line saves little.
  On 2b, Lua runs 0.6% fewer instructions, Zig 0.3% more and Protocol Buffers the same.
  The generated code covers nearly every pattern, so the hand path is cold.
- No step tests a pattern's first byte.
  The start tables already try each pattern only at a byte it can start with.
- Step 3 reads the character before the position through `Chars::boundary`, as the hand lexers do.
  The `\b` test decoded it before.
- Step 4 inlines the generated dispatch into the language's `matcher`, which calls the pattern's code at once.
- Step 5 applies when what follows the repeat must read a character outside its class ([design][design], section 2.1).
- Step 6 records a group only when the grammar reads it (section 2.2).
- Step 7 generates a back-reference to a group that matches one character.
  It compares the character at the group's start with the character at the position.
  Protocol Buffers' `String`, `(["'])(?:\\.|[^\\])*?\1`, leaves the regex engine, so all 40 of its patterns are generated.
- Step 8 stops the generated code writing group 0.
  The lexer writes it before any reader runs: `tokenize` after the scan picks a match, and `emit_captures` for every act and while pattern.
- Steps 6 to 8 are measured on [`research/default-matchers`][prototype] against the hand matchers at [851351f0][base].
- Step 9 makes the probe of section 2.3 a step: `record` is inlined at every call site ([e7f46604][step-9]).
- Step 10 gives each pattern's code its own function, kept out of line, which returns the match end in registers ([9101b496][step-10]).
  Section 2.5 has its shape.
- Step 11 is the arm drop, fix 1 of [`field-pattern-fixes`][field-pattern-fixes] ([9d804fae][arm-drop]).
  Of the three languages, only Protocol Buffers' generated code changes.
- Step 12 marks a pattern's function `#[inline]` when its code has no loop and no lookbehind and is at most 256 bytes ([0647d46a][step-12]).
  Section 2.6 has the reason.
- Step 13 reads a `\s` run that gives back nothing with the lexer's `space_end` ([0ecbda47][step-13]).
  It applies only when the class holds exactly the characters `space_end` takes.
- Step 14 tests one side of a `\b` or `\B` when the writer knows the other ([88bcd18c][step-14]).
  The known side is the node before, or the first character of what follows.
- Fix 2 is measured against step 14 at [30390626][fix-2].
  Its split is in [`field-pattern-fixes`][field-pattern-fixes], section 3.4.
- Step 15 applies to an alternation whose arms all start with a literal character, with at least three different first bytes ([11456f4b][step-15]).
  It reads the byte once and tries only the arms that start with it, in their order.
  Unlike the start tables, it works inside one pattern.
- Step 16 tests `\G` in the dispatch arm of a pattern that starts with it ([b4deb9f2][step-16]).
  The pattern's function runs only where the search began.
- Step 17 extends step 16 to `^`, `$`, `\A`, `\z` and `\Z` ([d5053f14][step-17]).
  Word boundaries stay in the function, where step 14 reads one side.
- Steps 9 to 17 are each measured against the step before.
  Their rows multiply that ratio by the row above.

Steps 5 to 14 in the other measures, and at 16 codegen units, are in [the note][note-measures].

Steps 15 to 17 against the step before, at one codegen unit, with 16 units in parentheses:

| Measure | Step | Protocol Buffers | Zig | Lua |
|:--|--:|--:|--:|--:|
| lexing | 15 | 1.0000 (0.9980) | 0.9971 (1.0386) | 0.9836 (0.9803) |
| lexing | 16 | 1.0000 (1.0000) | 1.0000 (1.0000) | 0.9836 (0.9868) |
| lexing | 17 | 0.9606 (0.9606) | 1.0000 (1.0000) | 0.9998 (1.0000) |
| the first document | 15 | 1.0000 (0.9987) | 0.9974 (1.0334) | 0.9904 (0.9886) |
| the first document | 16 | 1.0000 (1.0000) | 1.0000 (1.0000) | 0.9904 (0.9926) |
| the first document | 17 | 0.9728 (0.9761) | 1.0000 (1.0000) | 0.9999 (1.0004) |
| colouring | 15 | 1.0000 (0.9980) | 0.9973 (1.0355) | 0.9849 (0.9814) |
| colouring | 16 | 1.0000 (1.0000) | 1.0000 (1.0000) | 0.9847 (0.9877) |
| colouring | 17 | 0.9624 (0.9624) | 1.0000 (1.0000) | 0.9998 (1.0000) |

- Step 15 halves Lua's `m122`, from 757,323 to 385,062 instructions, and cuts `m124` by 90,461.
- At 16 units, Zig rises at step 15, for information only.
  Its `matcher` gains 795,870 instructions from changed inlining.
- Step 16 cuts Lua's `m111` from 489,890 to 44,275 instructions over 66,552 calls.
- Step 17 cuts Protocol Buffers' `m3`, `$\n?`, from 264,310 to 10,298 instructions.
  Its start table allows every byte, so the scan tried it at every byte of a line comment.
- Lua's 1.0004 at step 17 is at 16 units, for information only.
- The differential test found 0 differences at steps 15 to 17, and allocations did not change.

Step 17 measured directly against the hand matchers, on main at [6a4ce168][main-base]:

| Measure | Protocol Buffers | Zig | Lua |
|:--|--:|--:|--:|
| lexing | 0.9963 (0.9975) | 0.7992 (0.7997) | 0.9700 (0.9668) |
| the first document | 0.9975 (0.9985) | 0.8213 (0.8244) | 0.9822 (0.9798) |
| colouring | 0.9965 (0.9976) | 0.8115 (0.8120) | 0.9719 (0.9690) |

- Every row is at or below 1.000, at one and at 16 codegen units.
- The direct lexing rows match the chained rows of the first table within 0.0001.

### 2.1 A dropped step: a cache of class runs

A cache on the lexer kept the last run of an ASCII class.
Another pattern that took the same class at the same position read nothing.
Against step 5, at one codegen unit:

| Variant | Protocol Buffers | Zig | Lua |
|:--|--:|--:|--:|
| the cache | 1.0017 | 0.9747 | 1.0002 |
| the cache, called only when the run is not empty | 1.0026 | 0.9851 | 0.9981 |
| the same, with the cache's function always inlined | 1.0024 | 0.9847 | 0.9976 |

Protocol Buffers rises in every variant, so the step was dropped (lead, 2026-11-10).

- Protocol Buffers' and Lua's `lex` functions rise by 3,705 and 12,839 instructions in all three variants.
  The cache's field on the lexer may cause it; nobody has read the assembly.
- Lua's identifier matchers gain what the cache was for: about 0.3 million instructions.

An open question remains.
Lua's pattern `m2`, `(^[\t ]+)?` before `--`, reads a run only at a line's start.
Yet it costs 0.17 million more instructions in the last two variants, with the call guarded and with it inlined.
The cause is unknown.

### 2.2 The groups recorded

The profile of step 5 named two costs:

- Lua's `record` functions wrote every group of a match: 1.05 million instructions, 3.3% of Lua's count.
- Protocol Buffers' `String` pattern ran on the regex engine for its back-reference: 0.18 million instructions, 3.6% of its count.

Step 6 answers the first, and step 7 the second.
Step 6's writer reads each pattern's groups from the grammar.
A group is recorded when one of these reads it:

- a scope that names the group;
- the patterns of a capture, which lex the group's text;
- an end or while pattern, through the begin text its begin pattern carries.

| Language | Patterns | Patterns that read a group |
|:--|--:|--:|
| Protocol Buffers | 40 | 23 |
| Zig | 56 | 3 |
| Lua | 154 | 20 |

- Four Lua patterns carry a group to their end as begin text: `CommentAt0At0`, `CommentAt0At1`, `StringAt3` and `StringAt4`.
- Lua's `record` falls from 1.05 to 0.88 million instructions, and Zig's from 0.67 to 0.56 million.
- C# and Swift read groups in tests too.
  Both keep their hand matchers, so step 6 does not reach them.
- The differential test compares every group a pattern reads, at every step.
  It found 0 differences at steps 6, 7 and 8.
- At step 7 it compares 8,292,000 Protocol Buffers matches, 207,300 more than in section 1, because `String` is generated.

### 2.3 Where step 8's saving comes from

Most of step 8's Lua saving comes from inlining `record` instead of the group-0 write ([note][note-step-8]).

### 2.4 The costs after step 8

Lua's `matcher` costs 34.9 instructions per entry after step 8, against 11.4 by hand ([note][note-costs]).

### 2.5 The matcher's frame

At step 9, every pattern is inlined into `matcher`, and each entry pays for a 136-byte frame.
Step 10 removes the frame, and Lua's entry falls from 33 to 23 instructions ([note][note-frame]).

### 2.6 Small patterns inline

At step 10, 16 codegen units put tiny Zig patterns out of line that step 9 had inline, and Zig rose.
Step 12 offers such patterns inline again, but not one with a lookbehind, whose closure gave Lua's `matcher` a frame ([note][note-small]).

## 3. Coverage

Five Lua end patterns have neither generated code nor a regex engine path: the ends of long strings and long comments.
Each reads a group of its begin pattern through a back-reference, such as `]\1]`.
They keep their hand code, a coverage gap that task 1.3 closes.

## 4. The field line under the limits

These lines test Protocol Buffers' field pattern on step 2b ([`field-pattern-fixes`][field-pattern-fixes]).
Under the default limits, on a box:

| Line | Hand matcher, at base | Generated matcher, step 2b | Step 10 with the arm drop |
|:--|:--|:--|:--|
| 101 bytes, one short of the longest fixture line (102) | uncut | ends at `Limit::Work` | uncut |
| 12,001 bytes | ends at `Limit::Work` | ends at `Limit::Work` | ends at `Limit::Work` |

So the generated matcher cuts a 101-byte line that the hand matcher lexes whole.
The 12,001-byte line is the pattern's limit line.

The arm drop is fix 1 of [`field-pattern-fixes`][field-pattern-fixes], prototyped in the writer ([9d804fae][arm-drop]).
It clears the 101-byte line alone, in release and debug builds.
The 12,001-byte line still needs fix 2.

A 102-byte line was not measured.
From 101 bytes, its work grows by about `2L`, some 200 steps, and the budget by 64.
So it would end at the limit too.
The tests of [rule-table-codegen][rule-table-codegen] task 1.1 run it at 102 bytes.

[arm-drop]: https://github.com/victor-software-house/nishiki-archive/commit/9d804fae0de3b6073f38e3faf57d77ca915583cf
[base]: https://github.com/victor-software-house/nishiki-archive/commit/851351f064d86c429f4652af2c59af138dcf6ba2
[default-matchers-note]: ../../research/2026-11-10-default-matchers.md
[design]: design.md
[field-pattern-fixes]: ../field-pattern-fixes/design.md
[fix-2]: https://github.com/victor-software-house/nishiki-archive/commit/30390626b75a582a60da875573339ee08c0286a9
[main-base]: https://github.com/victor-software-house/nishiki-archive/commit/6a4ce168c9eae2bbacf22a3a6c0600c6f12758ad
[note-costs]: ../../research/2026-11-10-default-matchers.md#the-costs-after-step-8
[note-measures]: ../../research/2026-11-10-default-matchers.md#steps-5-to-14-in-every-measure
[note-frame]: ../../research/2026-11-10-default-matchers.md#the-matchers-frame
[note-small]: ../../research/2026-11-10-default-matchers.md#small-patterns-inline
[note-step-8]: ../../research/2026-11-10-default-matchers.md#where-step-8s-saving-comes-from
[prototype]: https://github.com/victor-software-house/nishiki/tree/818ebaa1aa569c6f2c0ac025090c644658f47b33
[rule-table-codegen]: ../rule-table-codegen/tasks.md
[step-9]: https://github.com/victor-software-house/nishiki-archive/commit/e7f466043e2d3440c2266c94e2de9cb5ef6a02d7
[step-10]: https://github.com/victor-software-house/nishiki-archive/commit/9101b49627ddeab7635df84334595e7a2fb8740a
[step-12]: https://github.com/victor-software-house/nishiki-archive/commit/0647d46a4b192320f8212b852633bf6c7482c943
[step-13]: https://github.com/victor-software-house/nishiki-archive/commit/0ecbda472d369abaa18367b174983002f3ffff41
[step-14]: https://github.com/victor-software-house/nishiki-archive/commit/88bcd18c6a0cb8297f28263a23a1b8e696c2ffd1
[step-15]: https://github.com/victor-software-house/nishiki-archive/commit/11456f4b1ce35e5d81a86c72b76619b5652ff6ea
[step-16]: https://github.com/victor-software-house/nishiki-archive/commit/b4deb9f2504f9e28e2c17138b5b22c926c726e6b
[step-17]: https://github.com/victor-software-house/nishiki-archive/commit/d5053f14a76b9c48b6f4b1959803d5d23d4357b9
