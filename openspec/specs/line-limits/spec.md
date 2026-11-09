# line-limits Specification

## Purpose

Bound the work and memory one line of input can make a highlighter spend.
The caller sets every bound, and every truncated line is reported.

## Requirements

### Requirement: Every limit is configurable

A caller SHALL be able to set, raise, lower or turn off each limit for each highlighter it creates.
The limits are line length, work per line, nesting depth and interned stacks.

#### Scenario: A caller turns a limit off

- **WHEN** a caller creates a highlighter with the work limit turned off
- **THEN** no line is truncated for work, however long it runs

#### Scenario: A caller lowers a limit

- **WHEN** a caller sets the line length limit to 1,000 bytes
- **THEN** a line of 1,001 bytes is emitted as one token under the current stack and the state passes over it unchanged

### Requirement: Pinned limits give the same output

Limits SHALL be counted in work and bytes, never in time, so the same input and limits give the same tokens on every machine.

#### Scenario: Two machines lex a hostile line

- **WHEN** two machines lex the same line with the same limits
- **THEN** both give the same tokens and the same report

### Requirement: Defaults never truncate the oracle

The default limits SHALL be high enough that no line of any oracle fixture reaches one.

#### Scenario: The oracle runs with default limits

- **WHEN** the oracle test lexes every fixture with the default limits
- **THEN** no line reports a limit

### Requirement: Limits can follow stated resources

A caller SHALL be able to derive limits from a memory budget and a per-line time budget it states, and then override any field.

#### Scenario: A caller derives limits from a frame budget

- **WHEN** a caller derives limits from a 4 ms line budget and a stated speed in steps per second
- **THEN** the work limit is 4 ms times that speed, and the caller can still override it

### Requirement: Truncated lines are reported

Each lexed line SHALL report every limit it reached, and a line lexed in full SHALL report none.

#### Scenario: A line reaches the work limit

- **WHEN** a line runs out of work part way
- **THEN** the rest of the line is one token under the current stack
- **AND** the report names the work limit
- **AND** the next line starts from the state at that point
