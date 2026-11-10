# generated-rust Specification

## ADDED Requirements

### Requirement: Rust writes generated Rust

Every generated Rust file under `src/` SHALL be written by the `codegen/` workspace crate.
A script under `oracle/` SHALL only record facts from Shiki or Oniguruma, as formatted JSON under `oracle/facts/`.

#### Scenario: A table that depends on Oniguruma

- **WHEN** Oniguruma's answer for a set changes
- **THEN** its script writes a new fact file
- **AND** `mise run codegen` writes the Rust table from that file

### Requirement: CI holds every generated file to the inputs

CI SHALL run `mise run codegen -- --check` and fail when a generated file differs from what the writer produces.
The failure SHALL name the command that regenerates the files.

#### Scenario: A grammar changes and the tables stay

- **WHEN** a pull request changes a grammar under `tests/engine/grammars/` and leaves its tables unchanged
- **THEN** the check fails and names `mise run codegen`

### Requirement: Generated files name how they were made

Each generated file SHALL open with a `// @generated` header that names the command that regenerates it.
Its docs SHALL name the writer module and each input.
Its `mod` line SHALL carry `#[rustfmt::skip]`.

#### Scenario: A reader opens a generated file

- **WHEN** a reader opens a generated file
- **THEN** its first line names the command
- **AND** its docs name the fact file or grammar it came from

### Requirement: The output is the same on every run

The writer SHALL write the same bytes for the same inputs, whatever the machine or toolchain.
It SHALL NOT call the system rustfmt.

#### Scenario: Two runs on two machines

- **WHEN** the writer runs twice on the same inputs, on different machines
- **THEN** both runs write the same bytes

### Requirement: A moved language keeps the same tokens

Moving a language from its script to the Rust writer SHALL leave each generated file's tokens unchanged, doc comments included.
A first-byte start set MAY differ when the pull request lists it with its Identity result and its Callgrind rows.
A generated name MAY change where the script's name breaks the naming rules; `codegen --compare` maps it by pattern id.

#### Scenario: A family moves

- **WHEN** a pull request moves a family to the writer
- **THEN** `codegen --compare` finds the same tokens in every file the family's scripts wrote, outside the start sets it lists, once it maps renamed names
- **AND** the pull request deletes those scripts

#### Scenario: A case-insensitive pattern gains a start byte

- **WHEN** the Rust analysis adds the first byte of a `(?i)` orbit member to a start set
- **THEN** `codegen --compare` reports that set by its pattern
- **AND** the pull request lists it, Identity passes, and Callgrind shows its rows

### Requirement: Generated names follow the naming rules

Each generated identifier SHALL follow `CODING_STANDARDS.md` rule 3.9 and SHALL stay within 40 characters.
The writer SHALL tell two equal names apart by the nearest enclosing repository key where their paths differ.
It SHALL NOT append a pattern's whole path.

#### Scenario: Swift moves to the writer

- **WHEN** the first Swift move runs the writer
- **THEN** no generated Swift name passes 40 characters
- **AND** no line that holds one passes 100 columns

#### Scenario: A derived name is too long

- **WHEN** a derived name passes 40 characters and the language's `RuleTables` gives none
- **THEN** the writer fails and names the pattern id
