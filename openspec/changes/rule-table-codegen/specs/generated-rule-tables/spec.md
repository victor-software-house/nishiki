# generated-rule-tables Specification

## ADDED Requirements

### Requirement: A moved language keeps the same tokens

Moving a language from its script to the Rust writer SHALL leave each generated file's tokens unchanged, doc comments included.
A first-byte start set MAY differ when the pull request lists it with its Identity result and its Callgrind rows.
A generated name MAY change to the name design section 2.4 derives; `codegen --compare` maps it by pattern id.

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
The writer SHALL distinguish two equal names by the nearest enclosing repository key where their paths differ.
Where no enclosing key differs, it SHALL add each pattern's place in its pattern list as a suffix.
It SHALL NOT append a pattern's whole path.

#### Scenario: Swift moves to the writer

- **WHEN** the first Swift move runs the writer
- **THEN** no generated Swift name passes 40 characters
- **AND** no line that holds one passes 100 columns

#### Scenario: A derived name is too long

- **WHEN** a derived name passes 40 characters and the language's `RuleTables` gives none
- **THEN** the writer fails and names the pattern id

### Requirement: Generated names are unique and checked

Every generated name SHALL be unique within its language, whether derived or given in the language's `names`.
A `names` entry SHALL name a pattern the grammar has, and SHALL follow the same naming rules.

#### Scenario: Two patterns take the same name

- **WHEN** two patterns end with the same name, derived or from `names`
- **THEN** the writer fails and names both pattern ids

#### Scenario: A names entry is stale or breaks the rules

- **WHEN** a `names` entry names a pattern the grammar lacks, breaks rule 3.9, or passes 40 characters
- **THEN** the writer fails and names that entry
