# Proposal

## Motivation

nishiki matches Shiki on every byte, but it colours a document about ten times slower than Pierre's lexers.
The comparison of 2026-11-04 ([record][compare]) reads these geometric means over 62 corpus files:

| Engine | Warm MiB/s | First document ms | Fidelity |
|:--|--:|--:|--:|
| nishiki | 24.5 | 4.8 | 100% |
| Pierre | 281.8 | 2.0 | 73.5% |

The operator asked for that speed at 100% fidelity.

A Callgrind profile of the whole corpus finds where the instructions go ([design](design.md)):

- Styling is 65% of the path from text to colours.
  `Theme::style` costs 4,082 instructions per token, against 2,211 for lexing it.
- Building each rule's candidate tables is 69% of the work a fresh process does once.
- In lexing, the rule engine's matchers take half the instructions, and its scan a quarter.

## Changes

- The Callgrind gate counts the first pass and the colour pass, besides warm lexing.
- `Theme::style` allocates nothing and hashes no strings.
- A rule's candidate tables build in time linear in its patterns.
- The rule engine's matchers stop repeating the same character steps at one position, if a measurement keeps the idea.
- Then per-language matchers, ranked by use and by instructions per byte.
- Last, a highlighter keeps each scope stack's style once computed, so a token's style costs one lookup.
  Its public method is agreed with the streaming design first.
- Output does not change.
  The Identity workflow, the oracle and the Shiki differential stay green on every slice.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `theme-colouring`: a highlighter styles a token from its stack, with the style cached per stack.
- `benchmarks`: the Callgrind gate counts the first document and the colour path.

## Impact

- `src/theme.rs`, `src/theme/trie.rs` and `src/highlighter.rs` change for styling.
- `src/rules/tables.rs` changes for the tables.
- `examples/lex.rs` and `mise-tasks/callgrind.sh` change for the new rows.
- The public API gains one method, `Highlighter::style`.
  It is additive, so the release is a patch.
- The crate docs' quick start and the comparison harness use the new method.
- No dependency is added.
- Each slice runs the Callgrind gate at one codegen unit, which fails a row that rises more than 0.5%.
- Review sets a stricter bar: every language at or below 1.000.

[compare]: https://github.com/victor-software-house/nishiki-archive/blob/68620d9ef4c7eb99a91c30f3a3d6788a2c4aaebf/bench/compare/results/namespace-8x16/2026-11-04-855723c6.md
