# grammar-loading Specification

## ADDED Requirements

### Requirement: A caller highlights any TextMate grammar

The crate SHALL compile a TextMate grammar that a caller passes in Shiki's JSON form.
It SHALL highlight text with that grammar under the caller's limits.

#### Scenario: A grammar loads at run time

- **WHEN** a caller passes a grammar's JSON to `Grammar::from_json`
- **THEN** a highlighter built from the result highlights text in that language

#### Scenario: A grammar loads from a build script

- **WHEN** a build script writes a compiled grammar's bytes and the crate loads them with `Grammar::from_bytes`
- **THEN** the output equals the output of the same grammar loaded from JSON

#### Scenario: Grammars include each other

- **WHEN** two grammars include each other, and a caller loads them in either order into one `Grammars` set
- **THEN** each grammar's regions of the other get the other grammar's scopes

### Requirement: Output matches Shiki for every Shiki grammar

Every grammar Shiki 4.6.0 bundles SHALL load through the public API.
Each non-whitespace byte of its sample SHALL get the scope stack Shiki gives it.

#### Scenario: CI runs the Shiki grammars

- **WHEN** the fidelity job highlights each Shiki grammar's sample
- **THEN** no non-whitespace byte's scopes differ from Shiki's

### Requirement: The crate bundles no grammar

The crate SHALL ship no TextMate grammar a caller can load, and SHALL read no grammar it was not given.

#### Scenario: The package is inspected

- **WHEN** the published package is listed
- **THEN** it holds no grammar file
