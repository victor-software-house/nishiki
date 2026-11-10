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

### Requirement: A default matcher spends the line's work budget

A generated default matcher SHALL spend one step of the line's `Limits::work` for each byte it reads.
A byte read again after a give-back SHALL cost one more step.

The regex engine, as the fallback, SHALL run on the steps the line has left.
When the budget runs out, either SHALL stop with no match.
The lexer SHALL then end the line at the work limit.

#### Scenario: A hostile line drives a give-back loop

- **WHEN** a line under the default limits holds a long run of a repeat's body with no continuation
- **THEN** the line ends at the work limit, the rest of it is one token under the current scopes, and its report records `Limit::Work`

### Requirement: CI checks every generated matcher against the regex engine

For every language whose matchers `codegen/` writes, CI SHALL run a differential test and a hostile-line test.
The differential test SHALL compare each generated matcher with the regex engine at every character of the oracle fixture lines and of fuzzed lines.
It SHALL run with no work limit.

The hostile-line test SHALL drive each give-back loop and stack under the default limits.

#### Scenario: A generated matcher differs from the regex engine

- **WHEN** a generated matcher and the regex engine give a different end or group at one position
- **THEN** the differential test fails and names the pattern, the line and the position

#### Scenario: A generated matcher overruns the budget

- **WHEN** a hostile line runs past the default work limit without ending there
- **THEN** the hostile-line test fails
