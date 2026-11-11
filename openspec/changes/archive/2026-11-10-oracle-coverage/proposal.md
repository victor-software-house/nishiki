# Proposal

## Motivation

The [oracle test][oracle-test] checks every hand-written lexer against Shiki.
It checks only the lines its fixtures hold.

Nobody knows how much of each Shiki grammar those lines reach.
A pattern no fixture matches can be wrong without any test failing.
A first measurement on the fixtures alone found Swift's at 264 of 448 grammar expressions (design, section 1).
It found C#'s at 224 of 547.

## Changes

- A Bun script, `oracle/coverage.ts`, measures which parts of each hand-lexed grammar Shiki matches.
  It measures them while Shiki tokenizes the oracle fixtures and a fixed fuzz corpus.
  It tags each expression with an Oniguruma comment and counts the matches through Shiki's own engine.
  No grammar engine is written or copied.
- `mise run oracle:coverage` writes the result to `tests/oracle/COVERAGE.md`.
  Per grammar, the report gives the share of expressions and captures exercised, and lists the rest.
- A pull request workflow, `coverage.yml`, fails in two cases:
  when that report is out of date, and when a grammar marked complete gains a gap.
- Gaps close with new pinned and licensed inputs, recorded in [`tests/oracle/SOURCES.md`][sources].
  Where no document can reach a pattern, a recorded waiver and an [`oracle/onig.ts`][onig] probe close the gap.
- Each hand-written lexer on main gets a task to reach complete coverage.

## Capabilities

### New Capabilities

- `oracle-coverage`: the share of each grammar's patterns and captures that the Shiki oracle exercises, for each grammar a hand-written lexer follows.
  The share is measured, reported, and kept from falling.

### Modified Capabilities

None.
The [`shiki-fidelity`][fidelity] requirements stay as they are.
Coverage measures what they check.
It does not change what a fixture must match.

## Impact

- New: `oracle/coverage.ts`, `oracle/coverage-waivers.ts`, `tests/oracle/COVERAGE.md`, `mise-tasks/oracle/coverage.sh`, `mise-tasks/check/oracle-coverage.sh`, `.github/workflows/coverage.yml`.
- New oracle inputs and fixtures, one or more per language, each recorded in `tests/oracle/SOURCES.md`.
- No new dependency.
  The script uses Shiki 4.6.0 and Bun 1.4.4, which the oracle already pins.
- The published crate does not change.

[oracle-test]: ../../../../tests/it/oracle.rs
[sources]: ../../../../tests/oracle/SOURCES.md
[onig]: ../../../../oracle/onig.ts
[fidelity]: ../../../specs/shiki-fidelity/spec.md
