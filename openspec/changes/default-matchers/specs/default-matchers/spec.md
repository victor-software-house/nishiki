# default-matchers Specification

## ADDED Requirements

### Requirement: A grammar runs on the rule engine with no hand-written matcher

The rule engine's code writer SHALL give every pattern a default matcher built from its regular expression.
A language MAY name a hand-written matcher for any pattern instead.

#### Scenario: A caller's grammar

- **WHEN** a caller writes a grammar's rule-engine tables with no hand matcher
- **THEN** every pattern matches through its default matcher

### Requirement: A default matcher spends the line's work budget

A generated default matcher SHALL spend the line's `Limits::work` in its loops.
A loop that takes turns SHALL spend the bytes it took on every exit.
Each turn given back or popped SHALL spend one step.
A try that spends `s` steps SHALL do at most `c × (s + 1)` work, where `c` depends only on the pattern.

The regex engine, as the fallback, SHALL run on the steps the line has left.
When the budget runs out, either SHALL stop with no match.
The lexer SHALL then end the line at the work limit.

#### Scenario: A hostile line drives a give-back loop

- **WHEN** a line of about 12,000 bytes makes a repeat give back turns one at a time
- **THEN** the line finishes within the default limits, and its tokens cover it

#### Scenario: A pattern's work grows faster than the line

- **WHEN** a line drives a pattern into work that grows faster than the line's length
- **THEN** the line ends at the work limit, the rest of it is one token under the current scopes, and its report records `Limit::Work`

### Requirement: CI checks every generated matcher against the regex engine

For every language whose matchers `codegen/` writes, CI SHALL run a differential test.
It SHALL compare each generated matcher with the regex engine at every character of the oracle fixture lines and of fuzzed lines.
It SHALL run with no work limit.

#### Scenario: A generated matcher differs from the regex engine

- **WHEN** a generated matcher and the regex engine give a different end or group at one position
- **THEN** the differential test fails and names the pattern, the line and the position

### Requirement: CI runs hostile lines against every generated language

For every language whose matchers `codegen/` writes, CI SHALL run a hostile-line test under the default limits.

Its bound lines SHALL be about 12,000 bytes long and drive each give-back loop and stack.
A bound line SHALL finish under the default work limit.
Its retry lines SHALL be as long as the language's longest oracle fixture line.
In a retry line, a failing repeat is tried again at each later start.

#### Scenario: A hostile line overruns the budget

- **WHEN** any hostile line runs past the default work limit without ending there
- **THEN** the hostile-line test fails

#### Scenario: A bound line reaches the work limit

- **WHEN** a bound line ends at the default work limit
- **THEN** the hostile-line test fails, and its pattern gets a recorded finding and a limit line

#### Scenario: A retry line reaches the work limit

- **WHEN** a retry line of fixture length reaches the default work limit
- **THEN** the hostile-line test fails, and the finding is recorded with the pattern, the line and a plan to fix it

### Requirement: CI shows the work limit cutting superlinear work

For every language whose matchers `codegen/` writes, each pattern whose work grows faster than the line SHALL have a limit line of about 12,000 bytes.
CI SHALL run each limit line under the default limits, and it SHALL end at the work limit.

#### Scenario: A limit line runs to the end

- **WHEN** a limit line ends without reaching the default work limit
- **THEN** the hostile-line test fails, because the line no longer shows the limit cutting superlinear work
