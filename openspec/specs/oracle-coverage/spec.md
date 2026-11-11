# oracle-coverage Specification

## Purpose

Measure how much of each grammar that a hand-written lexer follows the oracle fixtures exercise.
Keep a grammar complete once every unit it reaches is exercised or waived.

## Requirements

### Requirement: Coverage is measured for every hand-lexed grammar

For every Shiki grammar that a hand-written lexer follows, the repository SHALL measure which of its reachable units the oracle fixtures exercise.
A unit is an expression (`match`, `begin`, `end` or `while`) or a capture group that the grammar names or gives patterns.

#### Scenario: A hand-lexed grammar is measured

- **WHEN** the coverage task runs
- **THEN** every grammar a hand-written lexer follows has a row, with its exercised expressions and captures as counts and percentages

#### Scenario: An engine grammar is left out

- **WHEN** a language runs on nishiki's TextMate engine
- **THEN** its grammar has no row

### Requirement: Exercised means Shiki matched the unit on a fixture

An expression SHALL count as exercised when Shiki, tokenizing a fixture, takes its match at least once.
A capture SHALL count as exercised when its group takes part in such a match and holds at least one non-whitespace byte.

#### Scenario: A capture of blanks alone

- **WHEN** a capture group only ever holds whitespace on the fixtures
- **THEN** the capture is reported as not exercised

### Requirement: The measure leaves Shiki's output unchanged

The coverage measure SHALL use Shiki's own tokenizer and regex engine.
It SHALL fail when the scope stacks it observes differ from any committed fixture.

#### Scenario: The instrumented run matches the fixtures

- **WHEN** the coverage task tokenizes the fixtures
- **THEN** every scope stack equals the committed fixture's, or the task fails and names the fixture

### Requirement: The report lists every gap

The repository SHALL hold a coverage report, written by the coverage task.
For each grammar, the report SHALL list every reachable unit no fixture exercises, by its path in the grammar.
Writing the report twice from the same inputs SHALL give the same bytes.

#### Scenario: A unit no fixture reaches

- **WHEN** no fixture exercises a reachable unit
- **THEN** the report lists it under its grammar, with its path and the start of its regex

#### Scenario: The fuzz corpus reaches a unit

- **WHEN** only the fixed fuzz corpus exercises a unit
- **THEN** the report marks it as reached by fuzz, and still counts it as not exercised by the fixtures

### Requirement: Pull requests keep the report current

A pull request SHALL fail its checks when the committed coverage report differs from the one the coverage task writes.
This holds for a pull request that changes the oracle's inputs, fixtures or scripts.

#### Scenario: A stale report

- **WHEN** a pull request adds an input without writing the report again
- **THEN** the coverage check fails and shows the difference

### Requirement: A complete grammar stays complete

A grammar is complete when each of its reachable units is exercised by a fixture or has a waiver with a reason.
Once a grammar is listed complete, a pull request that leaves one of its units unexercised and unwaived SHALL fail its checks.

#### Scenario: A complete grammar loses a unit

- **WHEN** a change removes the only fixture line that exercises a unit of a complete grammar
- **THEN** the coverage check fails and names the unit

#### Scenario: A grammar with gaps is not gated

- **WHEN** a grammar is not listed complete
- **THEN** its gaps appear in the report and do not fail the check

### Requirement: Waivers are for units no document reaches

A waiver SHALL give a reason why no document can reach the unit, and SHALL name an `oracle/onig.ts` probe case where the lexer has probe support.
A waiver for a unit that a fixture exercises SHALL fail the coverage task.

#### Scenario: A waived unit becomes reachable

- **WHEN** a new fixture exercises a waived unit
- **THEN** the coverage task fails and names the waiver to remove

### Requirement: Gaps close with licensed, pinned inputs

An input added to close a coverage gap SHALL be pinned to a commit with a licence found at its source.
Otherwise it SHALL be written for nishiki under nishiki's licence.
It SHALL be recorded with its source and licence before its fixture is committed.

#### Scenario: A written input closes gaps

- **WHEN** no third-party file exercises a set of units
- **THEN** a short input written for nishiki is added, recorded as written for nishiki, MIT OR Apache-2.0
