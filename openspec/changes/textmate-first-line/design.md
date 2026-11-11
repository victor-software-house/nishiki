# Design

## Context

[proposal.md](proposal.md) states why.

The first line reaches one rule list.
[`Scanner::build`][scanner] builds that list's scanner.
For every begin pattern, `Scanner::build` calls [`Cells::regex`][grammar].
`Cells::regex` fills its cell with [`Regex::for_scanner`][regex].
`Regex::for_scanner` parses the pattern.
It applies the lean rewrite for lean lines.
It compiles the exact program.

`Regex::for_scanner` also works out the pattern's first bytes.
It clones the tree in `relax::widen` and builds the relaxed `Hir` from the clone.
`Scanner::build` then compiles the list's relaxed `Hir`s into one `regex-automata` meta automaton.

### CMake's first-token cost

Callgrind measured it at one codegen unit, at [38ab4d76][head].
The harness is a scratch example outside the repository.
It calls `Highlighter::new`, then lexes lines from the first until one yields a token.
The input is CMake's first small bench-corpus file, and the measurement ran locally on a Linux box on 2026-11-04.

| Part | Instructions | Share |
|:--|--:|--:|
| Total to the first token | 144,780,061 | 100% |
| `Regex::build`, 21 patterns | 87,336,603 | 60.3% |
| · `relax::hir` | 24,127,008 | 16.7% |
| · parse | 18,868,551 | 13.0% |
| · lean rewrite in `ast.rs` | 11,329,262 | 7.8% |
| · program compile in `program.rs` | 10,819,434 | 7.5% |
| · dropping the parsed tree, its lean copy and the widened clone | 9,428,908 | 6.5% |
| · `relax::widen` | 5,691,301 | 3.9% |
| · `first_bytes` | 4,535,199 | 3.1% |
| meta automaton build for one list | 51,519,621 | 35.6% |
| · prefilter literal extraction | 22,080,257 | 15.3% |
| · prefilter searcher build | 6,990,992 | 4.8% |
| · forward and reverse NFAs | 20,198,044 | 14.0% |
| lazy DFA search, with its state building | 5,359,475 | 3.7% |

Allocation takes about 58 million instructions of the total.
The prefilter's literal cross product is the largest single allocator.
It makes 54,912 allocations and costs 9.6 million instructions.
The one-pass DFA, bounded backtracker and PikeVM builds fall below the profile's threshold.

### Which meta engines the scan uses

[`Scanner::candidate`][scanner] calls `search_with` on an unanchored `Input`.
It reads the match's start, end and pattern.
It reads no captures.
[`Core::search`][core] in `regex-automata` 0.4.18 tries the engines in this order:

1. The full DFA.
   nishiki's features leave it out.
2. The lazy DFA.
   It searches forward for the end, then in reverse for the start.
   The reverse search needs the reverse NFA.
3. `search_nofail`, only when the lazy DFA gives up.
   It tries the one-pass DFA, then the bounded backtracker, then the PikeVM.

The one-pass DFA serves anchored searches only.
The relaxed patterns are never anchored at the start, so the scan never uses the one-pass DFA.
It uses the forward NFA, the reverse NFA, the lazy DFA and the prefilter.
Only the fallback uses the bounded backtracker and the PikeVM.

## Goals / Non-Goals

**Goals:**

- Cut the cost of a scanner's build on the first line.
- Parse each pattern once.
- Keep steady state no higher than main for every TextMate language.
- Keep every pattern's output unchanged.
- Keep the work budget's charges unchanged.

**Non-Goals:**

- Hand lexers and the rule engine.
  Their first lines take 0.6 to 2.9 million instructions.
- A cheaper `relax::hir` per node.
  It is the largest single part.
  Rewriting its node walk is a separate change, after this one is measured.
- Keeping compiled grammars across processes.

## Decisions

### 1. One parse in two stages, with the parsed tree kept until the program compiles

A pattern goes through two stages.

The sketch stage runs at the scanner's build.
It parses the full tree once.
It works out the first bytes, the context check and the relaxed `Hir`.

The program stage runs at the first search that tries the pattern.
It applies the lean rewrite for lean lines.
It compiles the exact program, the prefilter and the anchors from the same parsed tree.

`Cells` keeps each pattern's sketch.
`Cells` keeps the parsed tree in a slot that the program stage takes once.
The program stage drops the tree after it compiles.
The regex cell's fast path stays one `OnceLock` load.

A search can try a pattern before any scanner reaches it.
That pattern then runs both stages from one parse, as `Regex::for_scanner` does today.

The sketch uses the full tree instead of the lean tree.
A lean list's scanner then no longer waits on the lean rewrite.

The lean rewrite only drops alternatives.
So the full tree's relaxed `Hir` matches a superset of the lean tree's, and its first bytes are a superset too.
The automaton can report extra candidates on a lean line.
The exact lean program rejects those candidates.
Output does not change.

Alternatives:

- Parse twice, once for the sketch and once for the program.
  This was slice 2's shape.
  It regressed first lines: Julia small 1.111, Nix medium 1.077, CMake small 1.054.
- Keep the parsed tree for the process's life.
  That needs no take-once slot.
  Every pattern then holds its tree for good.
  Memory decides between this and the take-once slot.
  The take-once slot is the default.

The program stage moves out of the scanner's build.
On CMake that is up to 22 million instructions of program compile and lean rewrite, plus their tree drops.
The saving depends on how many of the 21 patterns the first line tries.
Task 3.1 counts them before any code.

### 2. The relaxed `Hir` built in one walk of the parsed tree

`relax::widen` clones the tree.
It cuts each sequence at its first positive lookahead.
`relax::hir` then walks the clone.

One walk can do both.
At a sequence's first positive lookahead, the walk stops and continues into the lookahead's body.
The walk builds the `Hir` as it goes.

`Regex::build` checks that the widened pattern needs at least one character.
That check moves from `analysis::width` on the clone to the `Hir`'s minimum length.
A test checks that the two checks give the same answer on every bundled grammar's patterns.

This removes `relax::widen` and the clone's drop.
On CMake that is about 8.8 million instructions.
It costs nothing elsewhere.

### 3. The automaton builds only the engines the scan uses

Each option is measured on its own, on first token and steady state:

1. The default configuration, as today.
2. `onepass(false)` and `backtrack(false)`.
   Their builds fall below the profile's threshold.
   This option is expected to save little.
   It lands only if it measures.
3. `auto_prefilter(false)`.
   It removes the prefilter's 29 million instructions.
   The lazy DFA then reads every byte that the prefilter would have skipped.
4. A prefilter that nishiki builds and passes with `prefilter(Some(..))`.
   nishiki builds it from the union of the list's first bytes.
   The sketches already hold those bytes.
   It skips like a byte-set prefilter.
   It needs no literal extraction.

The option that lands is the one that lowers first token most.
Its steady state must stay no higher than main for every language.

## Risks / Trade-offs

- [Risk] A scanner can sketch a pattern that no search ever tries.
  That pattern holds its parsed tree for good.
  → `examples/memory.rs` measures every TextMate language against main, three builds per side.
  If the held trees cost more than they save, `Cells` keeps the tree only while `Scanner::build` runs.
- [Risk] A cheaper automaton reports more false candidates.
  The exact machine then runs more often.
  → Option 3 must keep steady state no higher than main for every language.
- [Rejected] Cutting the relaxed pattern short would shrink the automaton further.
  The work budget charges up to the candidate's end, in [`Tokenizer::scan_list`][tokenizer].
  A shorter relaxed pattern would change those charges.
- [Risk] In [#347][pr347], a type change inside `Regex` moved the register allocation of `Tokenizer::scan`.
  → The program stage keeps `Regex`'s fields as they are.
  Every step is read per function before it lands.

## Migration Plan

None.
No stored format or public API changes.
Each decision lands as its own pull request, stacked on [#347][pr347].
The order is decision 2, then 1, then 3.

[pr347]: https://github.com/victor-software-house/nishiki-archive/pull/347
[head]: https://github.com/victor-software-house/nishiki-archive/commit/38ab4d76d70a29fe976898d1368a343180b313b4
[scanner]: https://github.com/victor-software-house/nishiki-archive/blob/38ab4d76/src/textmate/scanner.rs
[grammar]: https://github.com/victor-software-house/nishiki-archive/blob/38ab4d76/src/textmate/grammar.rs
[regex]: https://github.com/victor-software-house/nishiki-archive/blob/38ab4d76/src/textmate/regex.rs
[tokenizer]: https://github.com/victor-software-house/nishiki-archive/blob/38ab4d76/src/textmate/tokenizer.rs
[core]: https://docs.rs/regex-automata/0.4.18/src/regex_automata/meta/strategy.rs.html
