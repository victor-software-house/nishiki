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
   A repeat is written in one of four ways, and the code never recurses once per turn:

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

Read from the code, not yet from a profile per function:

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

## Not verified

- A profile per function of the generated code.
  The three costs above come from reading it.
- The regex machine's step limit.
  The generated code counts no steps; a hostile line was not tried against it.
  Zig's hostile-line limits test passed.
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
