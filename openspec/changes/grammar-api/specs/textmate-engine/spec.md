# textmate-engine Specification

## MODIFIED Requirements

### Requirement: Nothing is read or downloaded at run time

The engine SHALL lex with tables built into the crate, or with grammar text the caller passes.
It SHALL read no file and make no request when it lexes.

#### Scenario: A language is used for the first time

- **WHEN** a highlighter lexes its first line of a TextMate language
- **THEN** it uses only tables inside the crate, or the grammar the caller passed
