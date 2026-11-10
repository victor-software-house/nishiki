# Default matchers

How many instructions does a default matcher cost against the hand-written matchers of Protocol Buffers, Zig and Lua, and can generated Rust match exactly what Oniguruma matches?

It is task 1.1 of the [`rust-codegen` change][tasks].
The decision it informs is design section 5.5.

## Answer

Rust code generated from each expression's parsed tree is exact, and costs 1.17 to 1.54 times the hand matchers.
The regex machine run per pattern costs 2.56 to 4.54 times.

- Generated code matched the regex machine at every position compared: 0 differences in 343 million matches.
- 243 of 245 patterns are generated.
  Two fall back to the regex machine: one has a back-reference, and one repeats a group that can match two ways at once.
- Allocations do not change.
- Generated code is not yet at the hand matchers' count.
  The method section names three costs, read from the code, not yet from a profile per function.

## Method

1. The prototype is the branch `research/default-matchers`, pinned at [5df96cd8][prototype].
   It is pushed with no pull request, so no CI runs.
   Its code is written from scratch for the prototype.
2. A grammar script dumps each pattern's expression into the language's `defaults.rs`.
   Each language's `matcher` tries, in order:
   - the generated matcher, from `generated.rs`;
   - the regex machine on the expression, `src/textmate/regex`, compiled once on first use;
   - the hand matcher.
3. The first measurement ran the regex machine alone, at [dd19ee98][vm-commit].
   It gives the line feed at the end of a line, and records every group below the grammar's group count.
4. [`emit.rs`][emit] writes `generated.rs` from the parsed tree of each expression.
   An ignored test runs it, then rustfmt formats the output.
5. Every generated matcher must match exactly what Oniguruma matches, because Shiki-exact output gates the release.
   A repeat is written in one of five ways, and the code never recurses once per turn:

   | Repeat | Written as | Exact because |
   |:--|:--|:--|
   | one character a turn | a loop that gives back one character at a time | it tries every count, as backtracking does |
   | at most one turn | the body, then the continuation, in both orders | it tries both choices in the regex's order |
   | possessive | a loop that keeps each turn's first match | that is what possessive means |
   | greedy or lazy, any other body | a loop that keeps each turn's first match | the analysis below proves no other match of a turn can succeed |
   | the same, when what follows can start like the body | a loop, then turns given back from an explicit stack | the turns are proven unique, so giving one back is the only choice left |

   Any other repeat sends the whole pattern to the regex machine.
6. The analysis proves a node unique given what follows it:
   - one character, an anchor, a lookaround or an atomic group has one match;
   - a sequence is unique when each part is, given what follows that part;
   - an alternation is unique when its arms are, and no two arms can start with the same character;
   - a repeat that can give back a turn is unique when what follows must read a character the body cannot start with.

   The sets are code points, from the class definitions.
   On lead bytes, `\w` and `\s` share every non-ASCII lead byte, which sent Protocol Buffers' `OptionStmt` to the regex machine at [c9bf3336][byte-commit].
7. A differential test compares each generated matcher with the regex machine on the same expression:
   - at every character of every line of the language's oracle fixtures;
   - with `\G` holding too, for an expression that holds `\G`;
   - on 2,000 fuzzed lines: half drawn from the fixtures' characters, half fixture lines with ranges deleted, repeated or reversed.

   It fails at the first differing end or group.
8. Callgrind counts each language's oracle fixtures with `mise run callgrind` against base [851351f0][base], at one codegen unit (`bench/AGENTS.md` rule 12), with valgrind 3.27.1.
9. The feature costs are clean release builds of nishiki with each feature set, with `cargo build --timings`.

## Results

### Paths

| Language | Patterns | Generated, no repeat of a group | Generated, loop | Regex machine |
|:--|--:|--:|--:|--:|
| Protocol Buffers | 40 | 37 | 2 | 1 |
| Zig | 56 | 55 | 0 | 1 |
| Lua | 149 | 149 | 0 | 0 |

No pattern needed the explicit stack.
The two patterns on the regex machine:

- Protocol Buffers' `String`, `(["'])(?:\\.|[^\\])*?\1`, has a back-reference.
- Zig's `NumbersAt6` repeats `([EPep][-+])|[0-9A-Z_a-z]`.
  Both arms can start with `e`, so backtracking inside a turn can change the match.

### Differential

| Language | Fixture lines | Fuzzed lines | Positions | Matches compared | Differences |
|:--|--:|--:|--:|--:|--:|
| Protocol Buffers | 3,374 | 2,000 | 207,300 | 8,084,700 | 0 |
| Zig | 43,320 | 2,000 | 1,447,153 | 79,593,415 | 0 |
| Lua | 40,717 | 2,000 | 1,680,199 | 255,390,248 | 0 |

The three tests took 11.7 s together in a release build.

### Instructions

Lexing pass, millions of instructions, at one codegen unit:

| Language | Hand | Regex machine | Ratio | Generated | Ratio |
|:--|--:|--:|--:|--:|--:|
| Protocol Buffers | 4.5 | 11.5 | 2.5644 | 5.8 | 1.2958 |
| Zig | 25.0 | 86.5 | 3.4591 | 29.4 | 1.1746 |
| Lua | 27.5 | 124.7 | 4.5401 | 42.2 | 1.5374 |

The generated rows are from [c9bf3336][byte-commit], where `OptionStmt` still ran on the regex machine.

The other two passes follow the same order:

| Language | First document, regex machine | First document, generated | Colour, regex machine | Colour, generated |
|:--|--:|--:|--:|--:|
| Protocol Buffers | 2.6415 | 1.2845 | 2.4911 | 1.2819 |
| Zig | 3.3268 | 1.1635 | 3.3075 | 1.1638 |
| Lua | 3.2517 | 1.3123 | 4.2985 | 1.5004 |

Allocations, KiB allocated and peak KiB are equal on both sides in every run: 148, 171 and 414 allocations.

### What the generated code costs

Read from the code at [5df96cd8][prototype], before any profile per function:

- `char_at` decodes UTF-8 at each step, where the hand matchers test bytes;
- `record` writes every group, where the hand matchers write the groups a scope reads;
- nothing tests the first byte before a pattern's code runs.

### Feature cost

Clean release builds of nishiki:

| Features | Crates | Wall seconds |
|:--|--:|--:|
| `all` | 2 | 58 |
| `all,codegen` | 5 | 78 |
| `all,codegen,format` | 7 | 87 |

From `--timings`, the crates `codegen` adds compile in about 3 s on the critical path: `proc-macro2`, `quote` and `unicode-ident`.
`format` adds `syn` (11.9 s) and `prettyplease` (7.7 s), about 20 s more.

## The steps, profiled

[The change's research][dm-research], section 2, lists the steps that change the generated code.
These sections hold the profiles per function behind steps 8 to 12.

### Where step 8's saving comes from

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

### The costs after step 8

The profile of step 8 against the hand matchers names the costs left.

Lua's own functions run 12.93 million instructions, against 9.89 million by hand.
Most of the gap is `matcher`'s own cost: 4.24 million, against 1.38 million.
It is entered 121,468 times, so it costs 34.9 instructions per entry, against 11.4 by hand.
Per entry, by pattern:

| Pattern | Generated, per entry | Hand, per entry |
|:--|--:|--:|
| `RootAt25`, `RootAt26` and `RootAt28`, the identifiers | 197 to 220 | 144 (`bounded_name`) |
| `RootAt16` | 146 | 111 (`global_declaration`) |
| `StringAt3` | 38.6 | 28.5 (`cdef_open`) |
| `LdocTag` | 24.4 | 12.8 (`ldoc_tag`) |
| the builtins | 188 | 203 (`word_in`) |

- The three identifier patterns are entered 11,629 times, and `bounded_name` 12,867 times.
- The builtins are the one pattern where the generated code wins.

Protocol Buffers' `matcher` costs 1.11 million over 26,476 entries, 41.9 each.
By hand it costs 0.93 million, 35.1 each.
The field pattern's inner loop costs 137,634 over 668 entries, 206 each.
Its hand code, `field_rest`, costs 33,207 over 275 entries, 121 each.
[`field-pattern-fixes`][field-pattern-fixes] changes that pattern.

These per-entry costs come from Callgrind as follows:

- The `lex` example lexes the fixtures three times: the first-document pass, the measured pass and the colouring pass.
- Callgrind counts instructions only in the measured pass, but it counts calls in all three.
- So the number of entries is the instruction count of a function's first instruction, from a profile taken with `--dump-instr=yes`.
  It is a third of the call count.

### The matcher's frame

At step 9, Lua's `matcher` costs 33 instructions per entry, against 11.4 by hand.
Its machine code shows where they go:

| Build | Per entry | Entry and exit | How the match comes back |
|:--|--:|:--|:--|
| hand | 11.4 | one push, one pop | a call to the pattern's function, in registers |
| step 9 | 33 | six pushes, a 136-byte frame, six pops | written to that frame, then read back |
| step 10 | 23 | three pushes, no frame, three pops | a call to the pattern's function, in registers |

Every pattern's code was inlined into `matcher`.
The largest pattern set the registers and the frame that every entry paid for.

Step 10 moves each pattern into its own function, marked `#[inline(never)]`, which calls `record` and returns `Option<usize>`.
The regex-engine fallback moves into a function of its own too.
Lua's `matcher` falls 1.23 million instructions, and Protocol Buffers' 0.55 million.

A first form of the same split was rejected ([8482d4a4][step-10-rejected]).
It marked each pattern's function `#[inline]`:

- At one codegen unit, the compiler inlined every pattern again, so the frame stayed.
- At 16 units, small helpers such as `Regex::match_at` went out of line.

Against step 9, at one codegen unit, with 16 units in parentheses:

| Measure | Protocol Buffers | Zig | Lua |
|:--|--:|--:|--:|
| lexing | 0.9814 (0.9815) | 0.9939 (1.0160) | 0.9925 (1.0099) |
| the first document | 0.9871 (0.9887) | 0.9946 (1.0139) | 0.9953 (1.0056) |
| colouring | 0.9822 (0.9823) | 0.9943 (1.0148) | 0.9930 (1.0093) |

### Small patterns inline

At 16 codegen units, step 10 raises Zig's lexing 1.26% against step 9.

- Zig's `matcher` rises 0.11 million instructions, and its pattern `m1` 0.17 million.
- At step 9, 11 Zig patterns were inline at 16 units, `m1` among them.
- Each tests one or two literal bytes, such as `$` or `//`.

A first form of step 12 ([cb948568][step-12-first]) marked a pattern's function `#[inline]` when its code had no loop and was at most 256 bytes.
Against step 11, at one codegen unit, with 16 units in parentheses:

| Measure | Protocol Buffers | Zig | Lua |
|:--|--:|--:|--:|
| lexing | 0.9993 (0.9289) | 0.9911 (0.9560) | 1.0081 (0.9990) |
| the first document | 0.9995 (0.9569) | 0.9920 (0.9616) | 1.0048 (1.0033) |
| colouring | 0.9993 (0.9321) | 0.9917 (0.9593) | 1.0076 (0.9991) |

Lua rose at one unit:

- Its `matcher` gained a 16-byte frame and 0.29 million instructions.
- Lua's `m1` holds a lookbehind, written as a closure, and it went inline.

Step 12 ([0647d46a][step-12]) keeps a pattern with a lookbehind out of line.
Then Lua's `matcher` has no frame, as at step 11.

- 16 Protocol Buffers patterns, 17 Zig patterns and 54 Lua patterns go inline.
- Lua's `matcher` rises 38,913 instructions, and Lua's count falls 21,482.
- Zig at 16 units, against step 9: 0.9676 lexing, 0.9718 the first document, 0.9700 colouring.
- Lua at one unit, against step 10: 0.9994 lexing, 0.9997 the first document, 0.9992 colouring.

## Not verified

- A profile per function of the code at [5df96cd8][prototype].
  The three costs under "What the generated code costs" come from reading it.
  Each later step in [the change's research][dm-research] has one.
- The work limit.
  At [5df96cd8][prototype], generated code counts no steps, and the regex machine runs with no budget.
  No hostile line was tried against either.
  Zig's hostile-line limits test passed.
  Design section 5.6 now requires both to spend the line's budget, with a hostile-line test in CI.
- The size the generated code adds to the binary.
- The differential test's time in a debug build, as CI runs tests.
- The program size of `examples/build-script`, which does not exist yet.
- Grammars past these three.
  A pattern shape they lack may need the explicit stack, or fall back.

## Compute

| Machine | Work | Minutes | Cost |
|:--|:--|--:|--:|
| A Boat `large` box, 8 vCPU and 16 GB | the regex machine's run and the feature builds | not recorded | not checked |
| A Boat `large` box, 8 vCPU and 16 GB | generation, the differential and Callgrind, five rounds | about 50 | not checked |

[tasks]: https://github.com/victor-software-house/nishiki-archive/blob/7b675823/openspec/changes/rust-codegen/tasks.md
[prototype]: https://github.com/victor-software-house/nishiki/tree/5df96cd86c12fd8e45eecb075f8fb6c94fdaf1c7
[vm-commit]: https://github.com/victor-software-house/nishiki-archive/commit/dd19ee9840841fb84e7fb5fe8949b18cf68eb306
[emit]: https://github.com/victor-software-house/nishiki-archive/blob/5df96cd86c12fd8e45eecb075f8fb6c94fdaf1c7/src/textmate/regex/emit.rs
[byte-commit]: https://github.com/victor-software-house/nishiki-archive/commit/c9bf3336bb2b0984d7764b6841210b6f335ff480
[base]: https://github.com/victor-software-house/nishiki-archive/commit/851351f064d86c429f4652af2c59af138dcf6ba2
[field-pattern-fixes]: ../changes/field-pattern-fixes/design.md
[step-10-rejected]: https://github.com/victor-software-house/nishiki-archive/commit/8482d4a4a97d34cb65a5dc2da5d0dc47672e8f4b
[dm-research]: ../changes/default-matchers/research.md
[step-12]: https://github.com/victor-software-house/nishiki-archive/commit/0647d46a4b192320f8212b852633bf6c7482c943
[step-12-first]: https://github.com/victor-software-house/nishiki-archive/commit/cb948568af41a5c26863b3b9ef3c7c6d3b483ef9
