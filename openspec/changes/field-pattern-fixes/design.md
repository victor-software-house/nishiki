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

## 4. Before the fixes

Until both land, a caller that needs such lines uncut raises `Limits::work`.
Protocol Buffers moves to generated matchers only after them, since Identity and the hostile-line tests must hold.

[dm-design]: ../default-matchers/design.md
[dm-research]: ../default-matchers/research.md
[fix-1-prototype]: https://github.com/victor-software-house/nishiki-archive/commit/9d804fae0de3b6073f38e3faf57d77ca915583cf
[pr-720]: https://github.com/victor-software-house/nishiki-archive/pull/720
