## Purpose

Say what nishiki measures about its own speed and memory, how each figure is produced, where it gates a change, and how it is compared with other highlighters.

## ADDED Requirements

### Requirement: Each figure answers a named question

Every published benchmark figure SHALL belong to a named question and state its unit, its statistic and the program that produced it.

#### Scenario: A reader looks up a figure

- **WHEN** a reader finds a figure in a record or in the bench README
- **THEN** the record names the question it answers, its unit, its statistic and the program that produced it

### Requirement: The corpus is pinned and licensed

Every file the wall-clock benchmarks and the comparison read SHALL be listed in the corpus manifest with its language, size tier, origin, source at a pinned commit, licence and SHA-256, and only permissively licensed files SHALL be used.

#### Scenario: A corpus file changes

- **WHEN** a corpus file's content no longer matches its SHA-256 in the manifest
- **THEN** the corpus check fails

#### Scenario: A file has no manifest row

- **WHEN** a file sits in the corpus directory without a row in the manifest
- **THEN** the corpus check fails

### Requirement: Figures from two corpus versions are never compared

A record SHALL name the corpus version it ran on, and a regression or speed claim SHALL compare only records of the same corpus version.

#### Scenario: The corpus gains a file

- **WHEN** a file is added to the corpus
- **THEN** the corpus version changes, and the next record is not compared with records of the previous version

### Requirement: A language's throughput weighs files by their size

A language's throughput in a tier SHALL be its files' total bytes over their total time, not the mean of per-file speeds.

#### Scenario: A tier holds a small and a large file

- **WHEN** a tier holds a 2 KB file and a 400 KB file of one language
- **THEN** the language's figure for that tier is 402 KB over the sum of both files' times

### Requirement: Per-line latency is measured over repeated passes

Per-line latency SHALL be each line's median time over repeated steady passes, with the timer's own overhead subtracted, and lines faster than the timer can resolve SHALL be reported as below resolution rather than given a number.

#### Scenario: A line lexes faster than the timer resolves

- **WHEN** a line's median time is under four times the measured timer overhead
- **THEN** the line is counted as below resolution, and no time is reported for it

### Requirement: First contact is reported apart from steady state

The first pass over a document from a fresh highlighter SHALL be reported apart from the steady passes, with its own maximum per-line time.

#### Scenario: A lazy structure is built on the first line

- **WHEN** the first pass over a document takes longer on a line than the steady passes do
- **THEN** the record shows that line's first-contact time apart from its steady median

### Requirement: Cold start is measured in fresh processes

Cold-start figures SHALL come from a fresh process per sample, timed from the start of the program, so that no lazy state built by an earlier sample is reused.

#### Scenario: The cold-start program runs

- **WHEN** cold start is measured for one language and file
- **THEN** every sample comes from its own process, and the figure is the median and p90 over those processes

### Requirement: Instruction counts gate every language

On a pull request labelled `bench`, CI SHALL count instructions of lexing the oracle fixtures, base against head, for every language with oracle fixtures, and SHALL fail when a language rises more than 0.5% at one codegen unit.

#### Scenario: A language with fixtures is not in a hand-kept list

- **WHEN** a pull request adds fixtures for a new language
- **THEN** the gate counts that language without any list being edited, and marks it new where the base has no fixtures for it

#### Scenario: A language rises

- **WHEN** a language's head count at one codegen unit exceeds its base count by more than 0.5%
- **THEN** the job fails and names the language

### Requirement: The instruction gate compares like with like

The gate SHALL build base and head each from their own tree and lockfile, SHALL count only the fixtures both sides have, and SHALL count only the measured lexing pass.

#### Scenario: A pull request adds a fixture

- **WHEN** a pull request adds a fixture to a language the base already has
- **THEN** that fixture is left out of both counts

#### Scenario: A pull request changes a dependency

- **WHEN** a pull request changes the crate's dependency versions
- **THEN** the base still builds with the base's own lockfile, and the gate runs

### Requirement: The gate's noise floor is measured and recorded

The bench rules SHALL state, per measured language, how far repeated counts of one build spread, and that spread SHALL stay below the gate's tolerance for every language.

#### Scenario: A reviewer reads a small move

- **WHEN** a pull request moves a language's count by less than the spread recorded for it
- **THEN** the reviewer reads the move as noise, not as a change of code

### Requirement: Layout-only counts never fail the gate

Counts at the default codegen units SHALL be shown for information only and SHALL never fail the job.

#### Scenario: Code moves between codegen units

- **WHEN** a change raises a language's count at sixteen codegen units by 3% and leaves it unchanged at one
- **THEN** the job passes and prints a note naming the language

### Requirement: Memory counts are reported in CI

The bench CI job SHALL report, for head and base, each language's allocations, bytes allocated and peak live bytes over one pass of its oracle fixtures from a fresh highlighter.

#### Scenario: A change grows the arena

- **WHEN** a change makes a language allocate more bytes on a first pass
- **THEN** the job's table shows both counts, and the job does not fail on them

### Requirement: Wall time never gates CI

No CI job SHALL fail on a wall-clock figure.

#### Scenario: A CI run is slow

- **WHEN** a CI runner is busy and a wall-clock figure doubles
- **THEN** no job fails because of it

### Requirement: A recorded run comes from a measured host

A recorded run SHALL come from the reference host class chosen by an A/A noise probe, SHALL measure an anchor workload at its start and end, and SHALL be rejected when the anchor moves more than the reference class's measured drift bound, which the record carries.

#### Scenario: The host drifts during a session

- **WHEN** the anchor workload moves by more than the reference class's bound between the start and the end of a session
- **THEN** the session is rejected and not recorded

#### Scenario: A session runs on another host class

- **WHEN** the recorder runs on a host that is not of the reference class
- **THEN** it records nothing and names the reference class

### Requirement: A record is data with a rendered summary

A recorded run SHALL be stored as machine-readable data naming the commit, corpus version, host, toolchain, anchor drift, compute and every figure with its rounds and spread, and its human-readable summary SHALL be rendered from that data.

#### Scenario: A summary disagrees with its data

- **WHEN** a record's summary or the bench README's results differ from what the newest record's data renders
- **THEN** the check fails

### Requirement: Comparators never enter the crate

Other highlighters SHALL be built only in a workspace of their own, outside the crate and outside the core benchmarks, and SHALL never be distributed.

#### Scenario: The crate is packaged

- **WHEN** the crate is packaged for crates.io
- **THEN** no comparator, comparison harness or corpus file is in the package

### Requirement: A comparison uses the same inputs and theme

A head-to-head comparison SHALL give every engine the same files and the same theme, and SHALL report per file, over the languages every compared engine supports.

#### Scenario: An engine lacks a language

- **WHEN** one compared engine has no grammar for a language in the corpus
- **THEN** that language is left out of the ratios that involve that engine, and the record lists it as left out

### Requirement: A comparison keeps setup apart from steady state

A comparison SHALL report setup, the first document in a fresh process, and warm throughput as three separate figures for every engine.

#### Scenario: An engine loads its grammars

- **WHEN** an engine spends time loading grammars and a theme before its first line
- **THEN** that time is in its setup figure and in no throughput figure

### Requirement: A comparison makes every engine do the full work

A comparison SHALL turn off every engine's time and line-length limits, SHALL build every Rust engine with the default release profile, and SHALL fail a run in which nishiki reports a limit reached.

#### Scenario: A line reaches a limit

- **WHEN** nishiki reports a limit reached on any corpus line during a comparison
- **THEN** the run fails instead of publishing figures for truncated work

### Requirement: Fidelity is reported beside speed

Every engine's comparison row SHALL carry its token count and its agreement with Shiki's colours per non-whitespace byte.

#### Scenario: An engine colours fewer tokens

- **WHEN** an engine is faster and agrees with Shiki on fewer bytes
- **THEN** its row shows both figures side by side
