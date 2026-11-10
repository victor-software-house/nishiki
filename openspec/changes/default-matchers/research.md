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

Step 5 in two more measures:

| Measure | Protocol Buffers | Zig | Lua |
|:--|--:|--:|--:|
| lexing, at 16 codegen units | 1.1032 | 0.8973 | 1.1452 |
| the first document, at one codegen unit | 1.0765 | 0.9229 | 1.0952 |

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

### 2.2 The costs after step 5

The profile of step 5 against the hand matchers names the costs left:

- Lua's `record` functions write every group of a match: 1.05 million instructions, 3.3% of Lua's count.
  A matcher could record only the groups a scope or a back-reference reads.
- Protocol Buffers' `String` pattern runs on the regex engine for its back-reference: 0.18 million instructions, 3.6% of its count.

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

[default-matchers-note]: ../../research/2026-11-10-default-matchers.md
[design]: design.md
[field-pattern-fixes]: ../field-pattern-fixes/design.md
[rule-table-codegen]: ../rule-table-codegen/tasks.md
