# textmate-engine Specification

## Purpose

Run the TextMate grammars a caller brings, such as Shiki's, with nishiki's own engine.
Each runs with the contract of every other language, as fast as it can be made.

## Requirements

### Requirement: Output matches Shiki

No language runs on the engine by default.
It lexes the grammars a caller brings.
The engine SHALL give every non-whitespace byte the scope stack Shiki gives it, for every grammar Shiki 4.6.0 bundles.

#### Scenario: The fidelity check runs every Shiki grammar

- **WHEN** the Shiki fidelity check, `mise run check:fidelity`, loads each grammar Shiki 4.6.0 bundles through the public API and lexes its input
- **THEN** no non-whitespace byte differs from Shiki's scopes

#### Scenario: A test table runs the oracle fixtures

- **WHEN** the engine's tests lex the oracle fixtures of a language with its test table, linked from the grammars in `tests/engine/grammars/`
- **THEN** no non-whitespace byte differs from Shiki's scopes

### Requirement: Nothing is read or downloaded at run time

A grammar SHALL be compiled into the crate before it is built.
The engine SHALL read no file and make no request when it lexes.

#### Scenario: A language is used for the first time

- **WHEN** a highlighter lexes its first line of a TextMate language
- **THEN** it uses only tables inside the crate

### Requirement: The contract of every language

A TextMate language SHALL start from the arena's states and resume from any line's state.
It SHALL honour the configured limits, report each limit it reaches, and append tokens that cover the line.

#### Scenario: A line spends the work limit

- **WHEN** a line's matcher steps pass the work limit
- **THEN** the rest of the line is one token under the current stack
- **AND** the report names the limit
- **AND** the next line resumes from that state

### Requirement: The engine is a second oracle

The engine SHALL be able to lex any language that has both a hand lexer and a grammar.
A test can then compare the two on the same lines.

#### Scenario: A hand lexer is compared

- **WHEN** a test lexes a fixture with a hand lexer and with the engine
- **THEN** every line where their scope stacks differ is reported

### Requirement: Speed is measured

The cost of loading a grammar SHALL be measured in a fresh process, by `mise run bench:cold`.
It times `Grammar::from_json`, `Grammar::from_bytes` and the first `Highlighter::for_grammar`.
Each speed claim SHALL show its table before and after.

#### Scenario: A fast path is added

- **WHEN** a change claims to speed the engine up
- **THEN** its pull request shows the benchmark table before and after, with the output unchanged
