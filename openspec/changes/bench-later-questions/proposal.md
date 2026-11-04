# Proposal

## Why

The `bench-methodology` change set two questions aside in its group 8.
They did not start before its other groups finished.
The lead moved them to this change on 2026-11-04, so `bench-methodology` can be archived.

## What Changes

- Each language's hostile lines move from `tests/it/limits/` into data.
  The limit tests and a new `hostile` bench both read that data.
  A record then shows the slowest hostile line per language under the default limits.
- A record lists the stripped binary size of a build with no language.
  It lists the growth each language feature adds alone.
- Output does not change.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

None.

## Impact

- `tests/it/limits/` reads its hostile lines from data instead of code.
- `bench/` gains the `hostile` bench and a size measurement.
- The record format of `bench/record.ts` gains the two figures.
