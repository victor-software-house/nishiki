# Design

This change fixes the first finding of [`default-matchers`][dm-design], section 2.5: Protocol Buffers' field pattern.
[Its research][dm-research], section 4, has the measurements.

## 1. The finding

The finding is Protocol Buffers' field pattern (lead, 2026-11-10):

- The pattern is `\s*(optional|repeated|required)?\s*(\.?[.\w]+)\s+(\w+)\s*(=)\s*(0[Xx]\h+|[0-9]+)`.
- The line is a type with no name after it: `a.` repeated and a space, inside a `message`.
- At each start the pattern takes the rest of the `[.\w]` run, gives it back, finds no name, and fails.
- At a start on a `.`, it does that twice: once with `\.?` taking the dot, once without.
  So the line costs more than `L²` steps.
- Shiki 4.6.0 lexes the line uncut, as one token, in under a millisecond at 101 and at 12,001 bytes.

The generated matcher cuts a 101-byte line that the hand matcher lexes whole.
The 12,001-byte line is the pattern's limit line.

Two fixes follow, in order.
Both keep the match exact (lead, 2026-11-10).

## 2. Fix 1: the arm drop

`codegen/` drops a repeat's arm when the arm it keeps gives the same match.
The unit compared is an arm followed by the rest of the pattern, up to the end of the innermost group around the repeat.

For `\.?`, that is `\.[.\w]+` against `[.\w]+`, both inside group 2.
The bare arms of `\.?`, a dot and nothing, never give the same end, so a check on them would never drop an arm.

Each unit gives its ends in its order of preference, each with its group spans.
`codegen/` drops an arm only when, at every position:

- the kept arm's unit gives every end the dropped arm's unit gives, with the same group spans;
- when the dropped arm is preferred, its unit's ends are the kept unit's first ends, in the same order.

At a shared end, what follows the group sees the same position and spans, so it succeeds or fails the same way.
So the first end where it succeeds is the same with the dropped arm or without it.
A dropped arm tried second only repeats ends already tried.

`\.?` before `[.\w]+` passes at a dot:

- `\.?` is greedy, so the arm that takes the dot is preferred.
- Its unit gives the ends from the run's end down to two bytes after the dot.
- The other unit gives the same ends in the same order, then one byte after the dot.
- Both units put the whole match in group 2, so their spans are equal.

A check for equal sets of ends would refuse this case, because the preferred unit cannot end one byte after the dot.
Dropping the arm halves the work at a dot.

### 2.1 Fix 1 measured alone

A prototype of fix 1 alone, [9d804fae][fix-1-prototype], was measured on a box in release and debug builds:

| Line | Fix 1 alone |
|:--|:--|
| 101 bytes | uncut |
| 12,001 bytes | ends at `Limit::Work` |

- This design expected fix 1 only to halve the work at a dot.
  It clears the 101-byte line as well.
- The 12,001-byte line still ends at the limit, so fix 2 is still needed.

## 3. Fix 2: the scan skip

A skip in the generated scan makes the line linear.
Fix 2 is what the 12,001-byte line needs (section 2.1).

The pattern is a prefix `P`, then `R`, the `[.\w]+` that takes a run, then a suffix `S`.

### 3.1 The conditions

`codegen/` writes the skip only when the expression meets four conditions, and writes none otherwise:

- `P` has an empty path with no assertion and no back-reference on it, so it matches empty at every start.
  - An assertion is `\b`, `^`, `$`, `\G` or a lookaround: each holds at some positions only.
    With `P` of `(?!a)`, `P` has no empty match before an `a`, so `R` never starts there.
  - A back-reference such as `\1` fails on an empty path when group 1 is unset or holds text.
  - The field pattern's `\s*(…)?\s*` has such a path.
- `P` holds no lookbehind.
  With one, the run bytes that begin a match of `P` depend on the bytes before the start, and so does `k`.
- `R` is one class under a `+` that gives back: greedy or lazy, not possessive, and not inside an atomic group.
- `S` holds no back-reference and no conditional `(?(n)…)` on a group of `P` or `R`.
  The same holds for every group `S` reaches through calls, directly or through other calls.
  `\g<0>` reruns the whole pattern, so it reaches every group.

The check lives in `codegen/`'s skip test, not only in the writer's fallback.
It refuses the skip when `S` reaches a call, so it holds even once the writer learns calls.
The parser refuses callouts such as `(*COUNT)` ([#720][pr-720]), so no callout carries state from one try to the next.

### 3.2 Why the skip is exact

Say the pattern fails at a start `p` in a run that ends at `r`.
The run is the longest run of `R`'s class from `p`.

- `P` matched empty at `p`, so `R` started at `p`.
- `R` gives back, so it was tried at every end up to `r`.
- A later start `q` before `r` puts `R`'s start inside the run when `P`'s match from `q` ends before `r`.
- `R`'s ends are then ends already tried from `p`.
- `S`'s result depends only on where it starts, so it fails from each of them again.

A back-reference and a conditional each read group state, which differs between starts.
They are the only constructs that make `S` depend on more than its start.
A call reruns a group's pattern and reads no captured text.

A lookbehind and `\b` read only the line, which is the same for every start.
One search has one start, so `\G` gives the same answer at each position from `p` and from every skipped `q`.
A `(?<=\G…)` in `S` can hold, and it holds alike from both.

### 3.3 Where the scan goes on

The scan then skips every start before `r` from which no match of `P` reaches `r`.
A start whose `P` can reach `r` is still tried.
The skip follows only a failure.
A try that ended at `Limit::Work` tried only some ends, so no skip follows it.

The general rule takes `k`, the length of the longest string of the run's bytes that begins a match of `P`.
That string need not be a match of `P` itself.
With `P` of `(ab\s)?`, `ab` is no match, yet `P` reaches past `r` from `r − 2`.

After the failure, the scan goes on at `r − k`, or at `p + 1` when that is later.
`codegen/` proves `k` from the expression.
When `k` has no bound, as for a `P` of `\w*`, it writes no skip.

Fix 2's `k` for the field pattern assumes fix 1 is applied:

- After fix 1, `P` takes nothing or one keyword from the run: `optional`, `repeated` or `required`.
  So `k` is 8.
- Without fix 1, `\.?` belongs to `P`, and `optional.` makes `k` 9.

On `xoptional foo bar = 1`, start 0 fails and start 1 matches, as on the regex machine.
A skip to the run's end would have missed that match.
With the skip, the 12,001-byte line no longer reaches the limit, and the pattern needs no limit line.

[The research of `default-matchers`][dm-research], section 4, has the results at 101 and 12,001 bytes before the fixes.

### 3.4 Fix 2 measured

Fix 2 is prototyped in the writer of `research/matchers-on-main`, on main [6a4ce168][main-base], at [30390626][fix-2-prototype].
Its rows are measured against step 14 of [`default-matchers`][dm-research], [dbcea536][step-14-base].
`research/default-matchers` holds fix 2 on the old base, at [e5280db7][fix-2-old-base], unmeasured.

The writer adds three rules to section 3.1:

- A run of `\s` characters earns no skip.
  The scan already passes the later blanks of a run of spaces and tabs.
  On Protocol Buffers, four such skips cost about 65,000 instructions.
- A failure is kept only when its run is longer than `k + 1` bytes.
  A shorter run holds no start the skip could pass.
- `k` counts bytes, as section 3.3 defines it.
  The skip's end is then the run's end less `k`, with no walk back over characters.

A kept failure is a 12-byte `Skip` in the lexer's per-line state, one per skipping pattern.
Its end is 0 when no failure is kept.
So each try first compares its start with that end alone.

Protocol Buffers has three skipping patterns: `Field`, `Kv` and `ReservedAt0`.
Zig has none.
Lua writes none, for two reasons:

- Its two candidates are emmydoc patterns.
- Without a skip, three emmydoc lines of 12,001 bytes end uncut in release and debug builds, and match Shiki 4.6.0's tokens.
  The test is `emmydoc_runs_end_uncut`.

Each commit measured, in order:

| Commit | Change |
|:--|:--|
| [a16e5b8b][form-a] | skips under section 3.1 alone |
| [e5e470b0][form-b] | no skip for a `\s` run |
| [26f320ab][form-c] | a failure kept only from a run longer than `k + 1` bytes |
| [fa35de31][follow-up-a] | the run's end carried out of the failed try in a `Cell` |
| [90195ea2][follow-up-b] | no skip for Lua |
| [b5934aba][byte-reach] | `k` in bytes |
| [30390626][fix-2-prototype] | `fa35de31` reverted, and the one-compare test |

Ratios against step 14, at one codegen unit, with 16 units in parentheses:

| Commit | Measure | Protocol Buffers | Zig | Lua |
|:--|:--|--:|--:|--:|
| `a16e5b8b` | lexing | 1.0244 (1.0240) | 1.0000 (0.9993) | 1.0010 (1.0005) |
| `a16e5b8b` | the first document | 1.0168 (1.0145) | 1.0000 (0.9994) | 1.0007 (1.0004) |
| `a16e5b8b` | colouring | 1.0233 (1.0229) | 1.0000 (0.9994) | 1.0009 (1.0004) |
| `e5e470b0` | lexing | 1.0042 (1.0041) | 1.0000 (0.9981) | 1.0008 (1.0001) |
| `e5e470b0` | the first document | 1.0029 (1.0025) | 1.0000 (0.9984) | 1.0007 (1.0001) |
| `e5e470b0` | colouring | 1.0040 (1.0039) | 1.0000 (0.9983) | 1.0007 (1.0003) |
| `26f320ab` | lexing | 1.0032 (1.0029) | 1.0000 (0.9981) | 1.0009 (1.0000) |
| `26f320ab` | the first document | 1.0022 (1.0018) | 1.0000 (0.9984) | 1.0003 (1.0001) |
| `26f320ab` | colouring | 1.0031 (1.0028) | 1.0000 (0.9983) | 1.0008 (1.0002) |
| `fa35de31` | lexing | 1.0036 (1.0031) | 1.0000 (0.9993) | 1.0009 (1.0002) |
| `fa35de31` | the first document | 1.0025 (1.0019) | 1.0000 (0.9994) | 1.0006 (1.0001) |
| `fa35de31` | colouring | 1.0035 (1.0030) | 1.0000 (0.9993) | 1.0009 (1.0003) |
| `90195ea2` | lexing | 1.0036 (1.0031) | 1.0000 (0.9993) | 0.9999 (0.9995) |
| `90195ea2` | the first document | 1.0025 (1.0019) | 1.0000 (0.9994) | 0.9999 (0.9998) |
| `90195ea2` | colouring | 1.0035 (1.0030) | 1.0000 (0.9993) | 1.0001 (0.9996) |
| `b5934aba` | lexing | 1.0036 (1.0031) | 1.0000 (0.9993) | 1.0000 (0.9994) |
| `b5934aba` | the first document | 1.0025 (1.0019) | 1.0000 (0.9994) | 0.9997 (0.9998) |
| `b5934aba` | colouring | 1.0035 (1.0030) | 1.0000 (0.9993) | 1.0000 (0.9995) |
| `30390626` | lexing | 1.0026 (1.0020) | 1.0000 (0.9976) | 1.0000 (0.9995) |
| `30390626` | the first document | 1.0018 (1.0012) | 1.0000 (0.9979) | 1.0001 (0.9996) |
| `30390626` | colouring | 1.0025 (1.0019) | 1.0000 (0.9978) | 1.0001 (0.9995) |

- Allocations are the same at every commit: 148 for Protocol Buffers, 171 for Zig and 414 for Lua.
- The differential test found 0 differences at every commit.
- Both field lines end uncut at every commit, in release and debug builds.

What the commits changed, from the lexing pass at one codegen unit:

- At `a16e5b8b`, a skip took 32 bytes, and Protocol Buffers' `lex` gained 2,211 calls to `memcpy`.
- `fa35de31` saved 1,138 instructions in the field pattern's function.
  The `Cell` cost 3,254 in the closure that reads the run.
- Lua's two skips cost 17,140 instructions in `lua::lex` and 6,407 in the pattern `m48`.
- `30390626`'s test saved 1,443 instructions against `b5934aba`.

Protocol Buffers ends above 1.000.
Its rise at `30390626`, 12,225 instructions:

| Function | Instructions | What it is |
|:--|--:|:--|
| `m36`, the field pattern | +8,515 | about 10 for each of its 825 tries |
| `proto::lex` | +3,710 | about one store for each of 3,374 lines, as the lexer is built |
| `char_at::<Proto>` | +6,370 | one folded function, named for Zig at step 14 |
| `char_at::<Zig>` | −6,370 | the same function |

- No skip passes a start in the fixtures, so the tries are as many as at step 14.
- The cost buys linear time on long field lines.
- After a failed try, `m36` reads the run again to its end.
  A read that stops `k + 2` bytes in would decide the store, and read on only to store it.
  That read is [`default-matchers`][dm-tasks] task 1.4.
  Whether the read is most of the `+8,515` is unverified: nobody has read the machine code.
- Lua's 1.0001 rows are the C library's allocator, 636 instructions.
  Lua's generated code is the same as at `b5934aba`.

The research branch is not clippy-clean.
The code that moves into `codegen/` must be.

## 4. Before the fixes

Until both land, a caller that needs such lines uncut raises `Limits::work`.
Protocol Buffers moves to generated matchers only after them, since Identity and the hostile-line tests must hold.

[dm-design]: ../default-matchers/design.md
[dm-research]: ../default-matchers/research.md
[dm-tasks]: ../default-matchers/tasks.md
[fix-1-prototype]: https://github.com/victor-software-house/nishiki-archive/commit/9d804fae0de3b6073f38e3faf57d77ca915583cf
[fix-2-prototype]: https://github.com/victor-software-house/nishiki-archive/commit/30390626b75a582a60da875573339ee08c0286a9
[form-a]: https://github.com/victor-software-house/nishiki-archive/commit/a16e5b8bfcf1728f5832a75d2a75a72261c94839
[form-b]: https://github.com/victor-software-house/nishiki-archive/commit/e5e470b0e4ced93466b3beb1a3b93524baefc634
[form-c]: https://github.com/victor-software-house/nishiki-archive/commit/26f320abb0c0928dcae67e649d42946febdbe2d9
[follow-up-a]: https://github.com/victor-software-house/nishiki-archive/commit/fa35de31405172090702f2e32a26a456c448e867
[follow-up-b]: https://github.com/victor-software-house/nishiki-archive/commit/90195ea2ed475989b84c8e023e742ba2d565e533
[byte-reach]: https://github.com/victor-software-house/nishiki-archive/commit/b5934abaa7eee719889120a2b7876a854f025e75
[fix-2-old-base]: https://github.com/victor-software-house/nishiki-archive/commit/e5280db7dbf98e2e6ac6de6e87149959d6b3fa93
[main-base]: https://github.com/victor-software-house/nishiki-archive/commit/6a4ce168c9eae2bbacf22a3a6c0600c6f12758ad
[step-14-base]: https://github.com/victor-software-house/nishiki-archive/commit/dbcea5367efd7c0fb79750b0f4930961ca987e3f
[pr-720]: https://github.com/victor-software-house/nishiki-archive/pull/720
