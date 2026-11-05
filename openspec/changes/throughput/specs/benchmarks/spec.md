## MODIFIED Requirements

### Requirement: Instruction counts gate every language

On a pull request labelled `bench`, CI SHALL count instructions of three passes over the oracle fixtures, base against head, for every language with oracle fixtures: lexing, the first document of a fresh process, and colouring with a theme.
It SHALL fail when any pass of a language rises more than 0.5% at one codegen unit.

#### Scenario: A language with fixtures is not in a hand-kept list

- **WHEN** a pull request adds fixtures for a new language
- **THEN** the gate counts that language without any list being edited, and marks it new where the base has no fixtures for it

#### Scenario: A language rises

- **WHEN** a language's head count of any pass at one codegen unit exceeds its base count by more than 0.5%
- **THEN** the job fails and names the language and the pass

#### Scenario: A change slows the first document

- **WHEN** a change makes building a language's tables cost more instructions than at its base, past the tolerance
- **THEN** the language's first-document row fails the job

### Requirement: The instruction gate compares like with like

The gate SHALL build base and head each from their own tree and lockfile, SHALL count only the fixtures both sides have, and SHALL run the same harness on both sides.
Its lexing pass SHALL count lexing alone, after the work a process does once.

#### Scenario: A pull request adds a fixture

- **WHEN** a pull request adds a fixture to a language the base already has
- **THEN** that fixture is left out of both counts

#### Scenario: A pull request changes a dependency

- **WHEN** a pull request changes the crate's dependency versions
- **THEN** the base still builds with the base's own lockfile, and the gate runs
