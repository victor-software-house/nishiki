# Proposal

## Motivation

Protocol Buffers' field pattern gives back the rest of a `[.\w]` run at every start of the run.
On a generated matcher, a 101-byte field line reaches the work limit, which the hand matcher and Shiki never reach.
Protocol Buffers cannot move to generated matchers while that line is cut.

## Changes

- `codegen/` drops a repeat's arm when the arm it keeps gives the same match.
- The generated scan skips the starts a failure has already tried, under conditions `codegen/` proves.
- Protocol Buffers' field lines of 101 and 12,001 bytes end uncut under the default limits.

## Related changes

This change was split from [`default-matchers`][default-matchers], whose design was over its word budget (lead, 2026-11-10).
Its section 2.5 now links here.

## Capabilities

### Modified Capabilities

- `default-matchers`: an exact arm drop and scan skip in generated matchers.

## Impact

- `unstable::codegen`: the arm-drop and scan-skip analysis, and the scan the writer emits.
- `tests/it/limits/`: Protocol Buffers' field lines.

[default-matchers]: ../default-matchers/proposal.md
