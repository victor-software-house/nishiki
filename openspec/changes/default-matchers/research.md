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
- Steps 9 to 14 are each measured against the step before.
  Their rows multiply that ratio by the row above.

Steps 5 and 8 in two more measures:

| Measure | Step | Protocol Buffers | Zig | Lua |
|:--|--:|--:|--:|--:|
| lexing, at 16 codegen units | 5 | 1.1032 | 0.8973 | 1.1452 |
| lexing, at 16 codegen units | 8 | 1.0596 | 0.8139 | 1.0615 |
| the first document, at one codegen unit | 5 | 1.0765 | 0.9229 | 1.0952 |
| the first document, at one codegen unit | 8 | 1.0431 | 0.8710 | 1.0608 |
| colouring, at one codegen unit | 8 | 1.0602 | 0.8626 | 1.0967 |

Steps 9 to 14 against the step before, at one codegen unit, with 16 units in parentheses:

| Measure | Step | Protocol Buffers | Zig | Lua |
|:--|--:|--:|--:|--:|
| lexing | 9 | 1.0000 (0.9999) | 0.9975 (0.9998) | 0.9847 (0.9900) |
| lexing | 10 | 0.9871 (0.9877) | 0.9792 (1.0126) | 0.9523 (0.9777) |
| lexing | 11 | 0.9918 (0.9962) | 1.0000 (0.9995) | 1.0001 (0.9999) |
| lexing | 12 | 0.9993 (0.9312) | 0.9911 (0.9560) | 0.9993 (0.9988) |
| lexing | 13 | 0.9948 (1.0668) | 0.9963 (1.0011) | 0.9914 (0.9927) |
| lexing | 14 | 0.9998 (0.9999) | 0.9733 (0.9784) | 0.9778 (0.9806) |
| the first document | 10 | 0.9910 (0.9925) | 0.9815 (1.0110) | 0.9710 (0.9869) |
| the first document | 11 | 0.9943 (0.9977) | 1.0000 (0.9996) | 1.0001 (1.0000) |
| the first document | 12 | 0.9995 (0.9582) | 0.9920 (0.9616) | 0.9996 (1.0034) |
| the first document | 13 | 0.9964 (1.0394) | 0.9967 (1.0009) | 0.9949 (0.9920) |
| the first document | 14 | 0.9999 (0.9999) | 0.9763 (0.9813) | 0.9868 (0.9889) |
| colouring | 10 | 0.9877 (0.9882) | 0.9807 (1.0116) | 0.9553 (0.9791) |
| colouring | 11 | 0.9921 (0.9964) | 1.0000 (0.9996) | 0.9999 (1.0000) |
| colouring | 12 | 0.9993 (0.9343) | 0.9917 (0.9593) | 0.9993 (0.9990) |
| colouring | 13 | 0.9950 (1.0636) | 0.9966 (1.0010) | 0.9918 (0.9931) |
| colouring | 14 | 0.9998 (0.9999) | 0.9753 (0.9801) | 0.9793 (0.9819) |

- The gate is one codegen unit, and there every row of steps 10, 12, 13 and 14 falls (lead, 2026-11-10).
- Zig rises at 16 units at step 10, and step 12 brings it back below step 9.
- At step 11, the field pattern's inner loop falls from 137,634 to 99,118 instructions.
- Lua's 1.0001 at step 11 is 547 instructions.
  545 of them are in the C library's allocator and `memcpy`.
  Lua's generated code does not change at step 11.
- At 16 units, Protocol Buffers rises at step 13, for information only.
  Its `matcher` goes from 252,950 to 568,727 instructions, back to its count at one unit.
- Step 14 writes 119 one-sided tests.
  Lua's generated patterns `m127`, `m128` and `m130` fall from 2.39 to 1.98 million instructions.
- The differential test found 0 differences at steps 9 to 14.

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

Most of step 8's Lua saving is inlining of `record`, not the group-0 write ([note][note-step-8]).

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
[note-costs]: ../../research/2026-11-10-default-matchers.md#the-costs-after-step-8
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
