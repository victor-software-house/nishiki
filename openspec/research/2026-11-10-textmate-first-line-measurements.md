# TextMate first-line measurements

What did tasks 3.1, 3.3, 4.1 and 4.2 of the `textmate-first-line` change measure?

The measurements were taken on 2026-11-04.
They were recorded under those tasks, in the change's [tasks file at `c9dcc3b1`][tasks-source].
This note holds them in full, so that each task keeps a one-line note within its word budget.
The figures and their wording are unchanged.

## 3.1 Patterns the first line tries

A throwaway patch on the base commit of step 2.
It logs each scanner's rule count and each distinct pattern a search tries, up to the first token.
The input is each language's first bench-corpus file of each tier.

The first line tries 1 to 6 patterns of the 21 to 131 its scanners list.
That is under 10% everywhere, so decision 1 goes ahead.

| Language | Tier | Scanners | Patterns listed | Patterns tried |
|:--|:--|--:|--:|--:|
| CMake | small | 1 | 21 | 2 |
| CMake | medium | 1 | 21 | 1 |
| PowerShell | small | 2 | 58 | 4 |
| PowerShell | medium | 2 | 58 | 4 |
| Bat | small | 2 | 32 | 3 |
| Bat | medium | 2 | 27 | 3 |
| Haskell | small | 3 | 69 | 6 |
| Haskell | medium | 3 | 69 | 5 |
| Mermaid | small | 2 | 32 | 2 |
| Mermaid | medium | 2 | 32 | 2 |
| Julia | small | 2 | 131 | 6 |
| Julia | medium | 2 | 70 | 2 |
| Scala | small | 2 | 79 | 3 |
| Scala | medium | 2 | 76 | 2 |

## 3.3 One parse, with the held trees' memory

First token falls by 3% to 23%.
Memory is equal or lower for every TextMate language.
Steady state is 1.0000 to 1.0015 by instruction count.

`Regex` lost its first bytes, so a regex cell went from 152 to 144 bytes.
LLVM indexes 144 bytes with `lea` and `shl`, where 152 bytes took one `imul`.
Layouts of 128, 136 and 160 bytes did no better.

## 4.1 The automaton's engines, against option 1

Measured locally on a Linux box, one build per side, against step 2 of the change.
Each cell gives the range over the 11 TextMate languages.
First token covers both tiers.

| Option | First token | Steady state |
|:--|:--|:--|
| 2: no one-pass DFA and no backtracker | 0.999 to 1.000 | 0.9997 to 1.0013 |
| 3: no prefilter | 0.37 to 0.99 | 0.9994 to 1.0241 |
| 4: a prefilter on the relaxed first bytes | 0.38 to 1.03 | 1.0066 to 1.0886 |
| 5: nishiki's extraction, literals of 4 bytes, at most 64 | 0.39 to 1.00 | 0.9999 to 1.0085 |
| 5: literals of 4 bytes, at most 250 | 0.42 to 1.00 | 0.9997 to 1.0087 |
| 5: literals of 8 bytes, at most 250 | 0.47 to 1.00 | 1.0000 to 1.0087 |
| 5: literals of 16 bytes, at most 250 | 0.49 to 1.00 | 0.9999 to 1.0086 |
| 6: option 5, with literal lists left to `regex-automata` | 0.39 to 1.00 | 0.9998 to 1.0008 |
| 6b: option 6, at most 64 literals of any length | 0.40 to 1.00 | 0.9993 to 1.0006 |

Option 2 changes nothing; its engines' builds were below the profile's threshold.
Option 4's byte-set prefilter skips less than the literal one, and is worse than none.
Option 5 misses steady state at every literal length.

A caller's prefilter skips `regex-automata`'s pure literal and Aho-Corasick strategies.
Option 6 keeps those strategies for a list of literals.
Option 6b also keeps them for a list whose prefixes are exact under 64 literals.

## 4.2 Option 6b on main

Option 6b, three builds per side on main after [#356][pr356].
First token falls by 0.3% to 60%.
Steady state is 0.9986 to 1.0013.

Every nishiki function is equal on both sides.
The rows above 1.0000 are glibc's `memcmp`, which moves with the heap layout.
Dart adds 1,100 instructions of its own.
`regex-automata` skips its `ReverseInner` strategy when a list's prefilter is fast.

## Not verified

- Nobody ran these measurements again for this note.
- They compare instruction counts, memory and size, not scopes, so they name no Shiki version.

## Compute

The task notes recorded no machine minutes or cost.
Writing this note ran nothing.

[tasks-source]: https://github.com/victor-software-house/nishiki-archive/blob/c9dcc3b1754dc1ad7f221e5a7eda1fcdea15ad49/openspec/changes/textmate-first-line/tasks.md
[pr356]: https://github.com/victor-software-house/nishiki-archive/pull/356
