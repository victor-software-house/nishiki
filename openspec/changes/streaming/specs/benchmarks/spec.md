## ADDED Requirements

### Requirement: Delta traces come from licensed sources

The repository SHALL hold delta traces of model replies, tool-written files and edit sequences in `bench/traces/`.
Each trace SHALL name its source file, its commit and its licence, read at the source.

#### Scenario: A trace whose source changed

- **WHEN** a trace and its source file disagree
- **THEN** the trace check fails and names the trace

### Requirement: The stream harness reports per-delta figures

For each engine and trace, the stream harness SHALL report setup, first-delta latency, and steady per-delta p50, p99 and maximum.

#### Scenario: nishiki on a reply trace

- **WHEN** the harness runs nishiki on a model-reply trace
- **THEN** it prints the five figures for that trace

### Requirement: The stream harness reports final fidelity

For each engine and trace, the stream harness SHALL report the final output's fidelity to Shiki on the whole text.
Fidelity is the share of non-whitespace bytes whose colour matches Shiki's.

#### Scenario: nishiki's final output

- **WHEN** the harness runs nishiki on any trace
- **THEN** its fidelity reads 100%

### Requirement: Each engine streams as its users drive it

The harness SHALL drive Shiki through `@shikijs/stream`, and Pierre through `StreamTokenizer` for appends and `LiveTokenizer` for edits.
An engine with no streaming API for a trace kind SHALL be left out of that trace's rows.

#### Scenario: Pierre on an edit trace

- **WHEN** the harness runs Pierre on an edit trace
- **THEN** it applies each edit with `LiveTokenizer.applyEdits`
