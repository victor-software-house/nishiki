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

Steps 5 and 8 in two more measures:

| Measure | Step | Protocol Buffers | Zig | Lua |
|:--|--:|--:|--:|--:|
| lexing, at 16 codegen units | 5 | 1.1032 | 0.8973 | 1.1452 |
| lexing, at 16 codegen units | 8 | 1.0596 | 0.8139 | 1.0615 |
| the first document, at one codegen unit | 5 | 1.0765 | 0.9229 | 1.0952 |
| the first document, at one codegen unit | 8 | 1.0431 | 0.8710 | 1.0608 |
| colouring, at one codegen unit | 8 | 1.0602 | 0.8626 | 1.0967 |

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

Step 8 dropped the group-0 write.
It also changed what the compiler inlined: Lua's `record` copies went inline.
A probe splits the two: step 7 with `record` always inlined.
Lexing ratios at one codegen unit, with 16 units in parentheses:

| Change | Protocol Buffers | Zig | Lua |
|:--|--:|--:|--:|
| step 7 to the probe | 1.0000 (1.0000) | 0.9681 (0.9718) | 0.9682 (0.9506) |
| the probe to step 8 | 0.9940 (0.9938) | 0.9768 (0.9391) | 0.9934 (0.9867) |

The three builds call each function the same number of times.
In Lua, `scan` runs 87,906 times and `matcher` 364,404 times.

- Step 7 to the probe saves 1.00 million instructions in Lua.
  The four `record` copies, 0.88 million, go inline and cost nothing of their own.
  `matcher` falls 0.12 million.
- The probe to step 8 saves 0.20 million in Lua.
  `scan` falls 0.24 million and `matcher` 0.21 million.
  Three `record` copies go out of line again, at 0.25 million.

So most of step 8's Lua saving is inlining, not the group-0 write.

### 2.4 The costs after step 8

The profile of step 8 against the hand matchers names the costs left.

Lua's own functions run 12.93 million instructions, against 9.89 million by hand.
Most of the gap is `matcher`'s own cost: 4.24 million, against 1.38 million.
Per call, by pattern:

| Pattern | Generated, per call | Hand, per call |
|:--|--:|--:|
| `RootAt25`, `RootAt26` and `RootAt28`, the identifiers | 66 to 73 | 48.1 (`bounded_name`) |
| `RootAt16` | 48.6 | 37.1 |
| `StringAt3` | 12.9 | 9.5 |
| `LdocTag` | 8.1 | 4.3 |
| the builtins | 62.8 | 67.5 (`word_in`) |

- The three identifier patterns run 34,887 times, and `bounded_name` 38,601 times.
- The builtins are the one pattern where the generated code wins.

Protocol Buffers' `matcher` costs 1.11 million, against 0.93 million by hand, over 79,428 calls.
The field pattern's inner loop costs 137,634 over 2,004 calls.
Its hand code, `field_rest`, costs 33,207 over 825 calls.
[`field-pattern-fixes`][field-pattern-fixes] changes that pattern.

## 3. Coverage

Five Lua end patterns have neither generated code nor a regex engine path: the ends of long strings and long comments.
Each reads a group of its begin pattern through a back-reference, such as `]\1]`.
They keep their hand code, a coverage gap that task 1.3 closes.

## 4. The field line under the limits

These lines test Protocol Buffers' field pattern on step 2b ([`field-pattern-fixes`][field-pattern-fixes]).
Under the default limits, on a box:

| Line | Hand matcher, at base | Generated matcher, step 2b |
|:--|:--|:--|
| 101 bytes, one short of the longest fixture line (102) | uncut | ends at `Limit::Work` |
| 12,001 bytes | ends at `Limit::Work` | ends at `Limit::Work` |

So the generated matcher cuts a 101-byte line that the hand matcher lexes whole.
The 12,001-byte line is the pattern's limit line.

A 102-byte line was not measured.
From 101 bytes, its work grows by about `2L`, some 200 steps, and the budget by 64.
So it would end at the limit too.
The tests of [rule-table-codegen][rule-table-codegen] task 1.1 run it at 102 bytes.

[base]: https://github.com/victor-software-house/nishiki-archive/commit/851351f064d86c429f4652af2c59af138dcf6ba2
[default-matchers-note]: ../../research/2026-11-10-default-matchers.md
[design]: design.md
[field-pattern-fixes]: ../field-pattern-fixes/design.md
[prototype]: https://github.com/victor-software-house/nishiki/tree/818ebaa1aa569c6f2c0ac025090c644658f47b33
[rule-table-codegen]: ../rule-table-codegen/tasks.md
