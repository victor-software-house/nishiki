# textmate-engine Specification

## MODIFIED Requirements

### Requirement: Nothing is read or downloaded at run time

The engine SHALL lex only with the grammars the caller passes.
It SHALL read no file and make no request when it lexes.

#### Scenario: A language is used for the first time

- **WHEN** a highlighter lexes its first line with a grammar the caller passed
- **THEN** it uses only that grammar and the grammars of the set the caller passed
- **AND** it reads no file and makes no request
