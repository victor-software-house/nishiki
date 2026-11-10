# grammar-loading Specification

## ADDED Requirements

### Requirement: A build script writes a grammar as Rust

With the `codegen` feature, a `Grammar` SHALL convert to Rust tokens that build the same grammar in a `static`.
With the `format` feature, `Grammar::to_rust` SHALL return those tokens laid out as a file.
Neither SHALL fail.

#### Scenario: A grammar compiled in a build script

- **WHEN** a build script writes a grammar's tokens to `OUT_DIR` and the program includes them
- **THEN** the program highlights with that grammar and reads no JSON or bytes at run time

#### Scenario: Tokens from another nishiki version

- **WHEN** a program includes tokens written by another nishiki version
- **THEN** it fails to compile, naming the format the tokens need
