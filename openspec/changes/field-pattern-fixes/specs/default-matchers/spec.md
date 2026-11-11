# default-matchers Specification

## ADDED Requirements

### Requirement: A generated scan skips starts only under four conditions

For a pattern of a prefix `P`, a run `R` and a suffix `S`, `codegen/` SHALL write a scan skip only when four conditions hold.
`P` SHALL have an empty path with no assertion and no back-reference on it, and SHALL hold no lookbehind.

`R` SHALL be one class under a `+` that gives back.
`S`, with every group it reaches through calls, SHALL hold no back-reference and no conditional on a group of `P` or `R`.

#### Scenario: A prefix with an assertion

- **WHEN** `P` is `(?!a)`
- **THEN** `codegen/` writes no skip for the pattern

### Requirement: A generated scan goes on where a later start can still match

After a failure at `p` in a run that ends at `r`, the scan SHALL go on at `r − k`, or at `p + 1` when that is later.
`k` SHALL be the length of the longest string of run bytes that begins a match of `P`.
`codegen/` SHALL write no skip when `k` has no bound.
No skip SHALL follow a try that ended at the work limit.

#### Scenario: A keyword inside the run

- **WHEN** the field pattern fails at start 0 of `xoptional foo bar = 1`
- **THEN** it matches from start 1, as on the regex engine

### Requirement: codegen drops a repeat's arm only when the match stays the same

A unit SHALL be an arm followed by the rest of the pattern, up to the end of the innermost group around the repeat.

`codegen/` SHALL drop a repeat's arm only when the kept unit gives every end the dropped unit gives, at every position.
Each such end SHALL carry the same group spans in both units.
When the dropped arm is preferred, its unit's ends SHALL be the kept unit's first ends, in the same order.

#### Scenario: The optional dot

- **WHEN** `\.?[.\w]+` runs at a dot, with one arm of `\.?` dropped
- **THEN** it gives the same match and groups as with both arms

### Requirement: Protocol Buffers' field lines end uncut

Protocol Buffers' generated matchers SHALL lex a field line with no name, of 101 and of 12,001 bytes, without reaching the default work limit.

#### Scenario: A 101-byte field line

- **WHEN** a 101-byte line of `a.` repeated and a space is lexed inside a `message`
- **THEN** it ends uncut

#### Scenario: A 12,001-byte field line

- **WHEN** the same line is 12,001 bytes long
- **THEN** it ends uncut
