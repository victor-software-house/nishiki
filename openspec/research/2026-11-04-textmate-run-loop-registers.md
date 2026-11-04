# Moving cold arms out of the TextMate machine's run loop

## Question

At 16 codegen units, the TextMate engine's `Machine::run` keeps `self` on the stack.
Three hot blocks then reload it, and CMake runs 2.7% more instructions than at one unit ([#382]).
Board item nishiki-archive#65 asked whether moving `run`'s rare arms out of the loop frees that register.
The gate was every TextMate language at or under its base at one unit.

This note records what was tried.
It informs the decision to close nishiki-archive#65 unmet; it does not make it.

## Answer

1. Moving the rare arms out of line cuts instructions on 10 of the 11 TextMate languages, at one unit and at 16.
   CMake falls 6%, Mermaid 5%, PowerShell 3.4%.
2. Proto rises 1.7% at one unit, so the change fails the gate.
3. Proto's cost is `run`'s entry.
   With the arms out of line, LLVM loads the text, the program and the context at the entry and keeps them on the stack.
   That adds about 10 instructions to each call of `run`.
   Proto calls `run` about 50,000 times for short attempts, so it pays more than it saves.
4. Three shapes that aimed at the entry cost each moved the cost between languages, or made every language slower.
   None meets the gate.

## 1. What was run

Each row comes from `MISE_ENV=dev,bench mise run callgrind` against main at [b3781ca4], which builds both sides itself.

| Field | Value |
|:--|:--|
| Base | [b3781ca4] |
| Host | a Boat `large` box: 8 vCPUs, 15 GB, x86-64 Linux |
| Toolchain | rustc 1.99.1, valgrind 3.27.1 from `mise.bench.toml` |
| When | 2026-11-04, 12:10 to 13:44 UTC |

The variants, each a pushed commit:

| Variant | Commit | Change |
|:--|:--|:--|
| A | [30beb3dd] | Atomic groups, group calls, the unwind on a spent budget and a lookaround's two loops move into helpers that are not inlined. Each helper takes its instruction's payload. |
| B | [88e6aa7f] | A, and `run` takes the search's start positions, so a search enters `run` once. |
| C | [9a1d9ab0] | B, with the next start position free to inline. |
| D | [714f726e] | A, and a small inlined front answers a failing first instruction before the loop. |
| E | [44730045] | A, and the program and text are arguments of `run`, not fields of the machine. |

## 2. Results

Head over base, at one unit unless the column says 16 units.

| Language | A, 1 unit | A, 16 units | B | C | D | E |
|:--|--:|--:|--:|--:|--:|--:|
| bat | 0.9789 | 0.9819 | 1.0005 | 1.0003 | 1.1446 | 0.9969 |
| cmake | 0.9395 | 0.9361 | 0.9500 | 0.9501 | 1.3587 | 0.9727 |
| dart | 0.9834 | 0.9795 | 1.0065 | 1.0059 | 1.1586 | 0.9988 |
| haskell | 0.9798 | 0.9725 | 1.0127 | 1.0121 | 1.1969 | 1.0064 |
| julia | 0.9742 | 0.9702 | 0.9834 | 0.9831 | 1.1806 | 0.9901 |
| mermaid | 0.9486 | 0.9437 | 0.9652 | 0.9652 | 1.3170 | 0.9806 |
| nix | 0.9870 | 0.9900 | 1.0123 | 1.0123 | 1.1208 | 1.0026 |
| powershell | 0.9658 | 0.9668 | 0.9740 | 0.9737 | 1.2369 | 0.9900 |
| proto | 1.0166 | 1.0176 | 0.9852 | 0.9856 | 1.1779 | 1.0586 |
| scala | 0.9881 | 0.9898 | 1.0150 | 1.0151 | 1.1248 | 1.0020 |
| zig | 0.9853 | 0.9877 | 1.0023 | 1.0021 | 1.1364 | 1.0003 |

## 3. Why each variant moved

- **A.**
  A bisect from [9f6f5468], an earlier form of A, removed one change at a time.
  It used one-unit builds counted with the same valgrind, against main at the merge of [#382].
  That form also kept `run` from inlining, which the bisect showed changes nothing.
  The cold unwind and the out-of-line loops carry the gain.
  With the unwind inlined, CMake read 1.1252.
  With the loops inlined, it read 0.9682.
  Proto's cost stayed in every form.
- **Proto under A.**
  Builds of [9f6f5468] and of the merge of [#382] with line tables at one unit showed the cost in one block: `run`'s entry and exit, run 25,117 times.
  It grows from 29 to 39 instructions.
  Head stores the text's pointer and length and three context fields on the stack there; base reloads them from `self` where it uses them.
  Proto barely reaches the helpers: 22 calls of `commit`, and no other helper above 1,000 instructions.
- **B and C.**
  Proto gains, but each start position now costs a call that dispatches on the source of positions and on the prefilter.
  Haskell makes about 105,000 such calls at about 15 instructions each, and rises 1.3%.
  The loop over start positions in `Regex::find` had paid a few instructions per position.
  In C, LLVM still kept the call out of line.
- **D.**
  The front and the loop both call `consume`, the arm that reads text.
  With two callers, LLVM kept `consume` and `ClassTable::matches` out of the loop, even with `#[inline]` on `consume`.
  The loop then makes a call for every literal and class instruction: on CMake, `consume` holds 3.3 million instructions of its own.
- **E.**
  Passing the program and text as arguments did not remove the entry cost.
  Proto rose further, to 1.0586, and the gains shrank.

## 4. Not verified

- Five reruns of each binary were not run for these rows.
  Rows within 0.2% of 1.0000 may move by that much from run to run.
  Every conclusion above rests on gaps of 0.5% or more.
- Why LLVM keeps those fields on the stack was not traced in its passes.
  Moving the two context reads, for `\A` and `\G`, into methods that are not inlined left the copies in place.
  Argument promotion is a guess, not a finding.
- Wall-clock time was not measured.

## 5. Compute

One Boat `large` box, about 95 minutes for this work, shared with no other run.
Its cost was not itemized.

[#382]: https://github.com/victor-software-house/nishiki-archive/pull/382
[b3781ca4]: https://github.com/victor-software-house/nishiki-archive/commit/b3781ca46a6cec66885a0efe46eec034763ce56b
[30beb3dd]: https://github.com/victor-software-house/nishiki-archive/commit/30beb3ddedafdfacf3b9a6ad916350b5a89e5189
[88e6aa7f]: https://github.com/victor-software-house/nishiki-archive/commit/88e6aa7fffd618f95f63a34a2006be525b413028
[9f6f5468]: https://github.com/victor-software-house/nishiki-archive/commit/9f6f5468acbfd18dd090a961f12727ca77e20448
[9a1d9ab0]: https://github.com/victor-software-house/nishiki-archive/commit/9a1d9ab06c09f64a6bc59654f121917e6ec9d322
[714f726e]: https://github.com/victor-software-house/nishiki-archive/commit/714f726e86d1793378db988ba979b657bcb9b579
[44730045]: https://github.com/victor-software-house/nishiki-archive/commit/447300450b7e3feedaec5f353f6e83074a48957d
