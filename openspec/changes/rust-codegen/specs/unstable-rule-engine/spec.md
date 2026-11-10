# unstable-rule-engine Specification

## ADDED Requirements

### Requirement: The rule engine is public under an unstable cfg

The crate SHALL expose the rule engine under `nishiki::unstable` only when built with `--cfg nishiki_unstable`.
Its items MAY change in any release, a patch included.

#### Scenario: A build without the cfg

- **WHEN** a caller builds nishiki without `--cfg nishiki_unstable`
- **THEN** `nishiki::unstable` does not exist

#### Scenario: A semver check

- **WHEN** the semver check runs
- **THEN** it builds without the cfg, so no item of `nishiki::unstable` counts

### Requirement: A grammar runs on the rule engine with no hand-written matcher

The rule engine's code writer SHALL give every pattern a default matcher built from its regular expression.
A language MAY name a hand-written matcher for any pattern instead.

#### Scenario: A caller's grammar

- **WHEN** a caller writes a grammar's rule-engine tables with no hand matcher
- **THEN** every pattern matches through its default matcher
