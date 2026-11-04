# Proposal

## Why

A fresh highlighter's first line on nishiki's TextMate engine costs far more than on a hand lexer.
After [#336][pr336] and [#347][pr347], CMake takes 144.8 million instructions to its first token.
PowerShell takes 72.6 million.
Every hand lexer takes 0.6 to 2.9 million.
The TextMate tier is opt-in.
Board item nishiki-archive#60 still asks for it to be as fast as it can be made.

## What Changes

- A rule list's scanner builds from one parse per pattern.
- A pattern's exact program compiles when a search first tries the pattern.
- `relax` builds the relaxed expression from the parsed tree without cloning the tree.
- The scanner's `regex-automata` automaton builds only the engines its search uses.
  Each engine choice is measured before it lands.
- Output does not change.
  The oracle and the Identity workflow stay green.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

None.
What nishiki highlights stays as specified.

## Impact

- `src/textmate/regex.rs`, `src/textmate/regex/relax.rs`, `src/textmate/grammar.rs` and `src/textmate/scanner.rs` change.
- A pattern's parsed tree can live from the scanner's build until the pattern's program compiles.
  `examples/memory.rs` measures that memory.
  That memory is part of the bar.
- No dependency is added.
- The public API does not change.
- The bar is first token and steady state at or under main for every TextMate language, with three clean builds per side.

[pr336]: https://github.com/victor-software-house/nishiki-archive/pull/336
[pr347]: https://github.com/victor-software-house/nishiki-archive/pull/347
