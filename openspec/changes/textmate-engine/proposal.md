# Proposal

## Why

Every language gets a hand-written lexer, in order of use, several at once ([AGENTS.md][agents]).
A hand lexer takes an hour or a day, but Shiki bundles more than 240 grammars, and the long tail will not all get one soon.
A TextMate engine of nishiki's own covers every bundled grammar the hand lexers have not reached, and it is a second oracle for the hand lexers that are written: any line where the two disagree is a bug in one of them (operator, 2026-11-02, [#55][pr55]).

The [research note][note] built a prototype and measured it.
It matched Shiki on all 23 oracle fixtures of four languages and ran 30 to 300 times slower than the hand lexers, so speed is this engine's first goal, as fidelity is the oracle's.

## What Changes

- A TextMate engine in the crate, written from scratch: it runs the begin, end, while, match, capture, include and injection rules of Shiki's grammars as the grammar says, and keeps vscode-textmate's corner cases because Shiki's output depends on them.
- `regex-automata` is the one dependency beyond `memchr`, approved by the operator on 2026-11-02, for the patterns that need no lookaround, backreference, `\G` or atomic group.
  The patterns that do need them run on nishiki's own backtracking matcher.
- Grammars are compiled before the crate is built into tables the crate includes: nothing is downloaded or parsed from JSON at run time, and each language costs nothing until it is used.
- The engine runs on the shared arena, `Limits` and `Highlighter` contract: a TextMate language is a `Language` like any other, its state a hash-consed frame, its per-line work counted and bounded.
- The engine is held to the same oracle as every language, and its speed is recorded under `bench/AGENTS.md`.

## Capabilities

### New Capabilities

- `textmate-engine`: nishiki runs bundled TextMate grammars with its own engine and the same contract as a hand lexer.

### Modified Capabilities

None.

## Impact

- One new dependency, `regex-automata`, with the licence check `cargo deny` runs; no other dependency is added without the operator's approval.
- `Language` gains a variant per grammar the engine runs, and a language that later gets a hand lexer keeps its variant and changes its implementation.
- The packaged crate grows by the compiled tables; the size per grammar is measured and reported before a grammar is bundled.

[agents]: https://github.com/victor-software-house/nishiki-archive/blob/9bb1d8b/AGENTS.md
[pr55]: https://github.com/victor-software-house/nishiki-archive/pull/55
[note]: https://github.com/victor-software-house/nishiki-archive/blob/9bb1d8b/docs/research/2026-11-02-textmate-engine.md
