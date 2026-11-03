## Purpose

Run Shiki's TextMate grammars with nishiki's own engine, with the contract of every other language and as fast as it can be made.

## ADDED Requirements

### Requirement: Output matches Shiki

The engine SHALL give every non-whitespace byte of every oracle fixture of a language it runs the scope stack Shiki gives it.

#### Scenario: A grammar runs its oracle

- **WHEN** the engine lexes each oracle fixture of a language it runs
- **THEN** no non-whitespace byte differs from Shiki's scopes

### Requirement: Nothing is read or downloaded at run time

A grammar SHALL be compiled into the crate before it is built, and the engine SHALL read no file and make no request when it lexes.

#### Scenario: A language is used for the first time

- **WHEN** a highlighter lexes its first line of a TextMate language
- **THEN** it uses only tables inside the crate

### Requirement: The contract of every language

A TextMate language SHALL start from the arena's states, resume from any line's state, honour the configured limits, report each limit it reaches, and append tokens that cover the line.

#### Scenario: A line spends its work

- **WHEN** a line's matcher steps pass the work limit
- **THEN** the rest of the line is one token under the current stack, the report names the limit, and the next line resumes from that state

### Requirement: The engine is a second oracle

The engine SHALL be able to lex any language that has both a hand lexer and a grammar, so that a test can compare the two on the same lines.

#### Scenario: A hand lexer is compared

- **WHEN** a test lexes a fixture with a hand lexer and with the engine
- **THEN** every line where their scope stacks differ is reported

### Requirement: Speed is measured and bound

The engine's per-line time on every oracle fixture SHALL meet `bench/targets.toml`, and each speed claim SHALL show its table before and after.

#### Scenario: A fast path is added

- **WHEN** a change claims to speed the engine up
- **THEN** its pull request shows the benchmark table before and after, with the output unchanged
