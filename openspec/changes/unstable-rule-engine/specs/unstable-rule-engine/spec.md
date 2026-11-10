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
