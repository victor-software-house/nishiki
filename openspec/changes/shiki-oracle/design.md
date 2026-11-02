# Design

## Fixture format

One text file per input, one record per line:

- `L <text>` starts a source line.
- `T <byte end> <scope> <scope> ...` is one TextMate token of that line: where it ends, as a UTF-8 byte offset, and its scope stack, outermost first.

Shiki merges tokens that share a colour, so the generator writes Shiki's explanations, the unmerged TextMate tokens, instead.
The format repeats the input, so a fixture is the whole test case, and it needs no JSON parser in the Rust test.
A line diff of a fixture shows which scopes changed when Shiki or an input moves.

## What is compared

Each non-whitespace byte's full scope stack, the measure of the engine comparison (section 2.4).
Whitespace is skipped because Shiki and a lexer may split a run of spaces between tokens differently without changing a colour.

## Limits off

The generator passes `tokenizeMaxLineLength: 0` and `tokenizeTimeLimit: 0`, so Shiki tokenizes every line in full and the fixture does not depend on the speed of the machine that wrote it.

## Bun, per task

The oracle task declares `bun` in its own `tools`, as the shared template tasks declare theirs, so CI and contributors who never regenerate fixtures install nothing.
The script imports `shiki@4.6.0` through Bun's auto-install, pinned to that version, so the repository needs no `package.json`.
CI does not regenerate fixtures: they are committed, and the test reads them offline.
