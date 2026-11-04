# Proposal

## Why

Shiki is nishiki's acceptance test (classification decision), but the crate's tests hold hand-written expectations: nothing in the repository checks a lexer against Shiki's own output.
Every language after Rust needs that check while it is written, and the engine decision makes an oracle sample per grammar a CI gate for the TextMate engine.

## What Changes

- A Bun script, `oracle/generate.ts`, runs Shiki 4.6.0 with its per-line limits off on inputs pinned to commits, and writes each input's scope stacks as a fixture under `tests/oracle/<language>/`.
- `mise run oracle` runs it; the task declares Bun itself, so CI installs nothing new.
- An integration test compares nishiki's scope stack with Shiki's on every non-whitespace byte of every fixture.
- The first fixtures are Rust: Shiki's sample and two held-out files from ripgrep and serde.
- `tests/oracle/SOURCES.md` records each input's source and licence.

## Capabilities

### New Capabilities

- `shiki-fidelity`: each supported language gives every byte the scope stack Shiki gives it, checked against Shiki's own output.

### Modified Capabilities

None.

## Impact

- New: `oracle/`, `tests/oracle/`, `tests/it/`, `mise-tasks/oracle.sh`.
- `Cargo.toml` links integration tests as one binary, `it`; the published crate does not change.
- `mise.lock` and `mise.dev.lock` record Bun 1.4.4 for the oracle task.
