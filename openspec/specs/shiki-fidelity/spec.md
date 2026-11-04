# shiki-fidelity Specification

## Purpose

Each language nishiki supports gives every byte of its input the TextMate scope stack Shiki gives it, so a theme in Shiki's format colours nishiki's output as Shiki would.

## Requirements

### Requirement: Scope stacks match Shiki on every fixture

For every fixture of a supported language, nishiki SHALL give each non-whitespace byte the same scope stack, outermost first, as Shiki gave it when the fixture was written.

#### Scenario: A lexer agrees with Shiki

- **WHEN** the oracle test lexes a fixture's lines in order from the default state
- **THEN** every non-whitespace byte carries the scope stack the fixture records for it

#### Scenario: A lexer disagrees with Shiki

- **WHEN** one non-whitespace byte carries a different scope stack
- **THEN** the test fails and names the fixture, line, column, Shiki's stack and nishiki's stack

### Requirement: Fixtures come from pinned inputs

Each fixture SHALL be written by Shiki, at a pinned version and with its per-line limits off, from an input fetched at a pinned commit, so that writing the fixtures again gives the same bytes.

#### Scenario: Fixtures are written again

- **WHEN** the oracle task runs with unchanged inputs and Shiki version
- **THEN** every fixture is byte for byte the same

### Requirement: Every input has a recorded licence

Every oracle input SHALL be listed with its source and licence before its fixture is committed, and the fixtures SHALL stay out of the published crate.

#### Scenario: A language adds an input

- **WHEN** an input is added to the oracle
- **THEN** its source and licence are listed beside the fixtures, and the packaged crate holds no fixture
