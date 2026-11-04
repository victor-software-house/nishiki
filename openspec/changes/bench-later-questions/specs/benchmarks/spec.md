## ADDED Requirements

### Requirement: A record reports the slowest hostile line

A recorded run SHALL report, per language, the time of its slowest hostile line under the default limits.
The limit tests and the `hostile` bench SHALL read the same hostile lines.

#### Scenario: A hostile line slows down

- **WHEN** a change makes a language's slowest hostile line slower than the 4 ms target
- **THEN** the record shows the line's time beside the target

### Requirement: A record reports binary size per language

A recorded run SHALL list the stripped size of a build with no language feature.
It SHALL list the size each language feature adds alone.

#### Scenario: A language grows the binary

- **WHEN** a change adds tables to one language
- **THEN** the record shows that language's growth against the build with no language
