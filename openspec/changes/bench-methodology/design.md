# Design

## Context

See the proposal for what is wrong today.
The [survey][survey] covers how other projects measure.

What exists:

- [`examples/bench.rs`][bench-rs]: throughput, per-line latency, "cold" start and theme styling over the oracle fixtures, run by `mise run bench`.
- [`examples/lex.rs`][lex-rs] and [`mise-tasks/callgrind.sh`][callgrind-sh]: instruction counts, head against base, run by the [Callgrind workflow][callgrind-yml] on a pull request labelled `bench`.
  It gates at 0.5% with both sides built at one codegen unit.
  The team relies on it, and this change keeps every guarantee it gives.
- [`bench/AGENTS.md`][bench-rules], [`bench/targets.toml`][targets] and one [recorded run][record].
- `tests/it/alloc.rs`: lexing a document a second time allocates nothing.
- `tests/it/limits/`: each language's hostile lines finish within the default limits.

Constraints:

- No cargo runs on the operator's Mac; CI verifies.
  Recorded runs use a remote box, chosen by cost.
- A measurement program lives in its own Cargo workspace, outside the root ([CODING_STANDARDS 5.3][standards]).
- No code from another highlighter enters the repository (invariant 1).
- Every new dependency waits for the lead's approval.

## Goals / Non-Goals

**Goals:**

- Every figure answers a question a user or a reviewer asks, and says how sure it is.
- A deterministic gate in CI for every language; wall time only where the machine is known.
- A comparison with other highlighters that their authors would accept as fair.
- A reader can rerun any published figure from the repository alone.

**Non-Goals:**

- Making nishiki faster. This change measures; speed work stays with each language.
- A hosted dashboard or a history service.
- Benchmarks for an HTML renderer, which nishiki does not have.
- Incremental re-highlighting after an edit. It can be added later as one more question.

## Decisions

### 1. Each benchmark answers one question

| # | Question | Figure | Statistic | Program | Where it counts |
|:--|:--|:--|:--|:--|:--|
| Q1 | How fast does nishiki lex a document? | MiB/s per language and tier | Criterion's estimate, with its 95% interval | `throughput` | recorded runs |
| Q2 | How fast does it colour one? | MiB/s for lexing and styling; ns per token for styling alone | the same | `styling` | recorded runs |
| Q3 | What does one streamed line cost? | µs per line: p50, p99 and maximum over lines | per line, the median of repeated passes | `latency` | targets, on recorded runs |
| Q4 | How soon is the first line coloured? | µs to create a highlighter, to lex the first line, to lex the first document; theme loading | median and p90 over fresh processes | `cold` | recorded runs |
| Q5 | How much memory does a document take? | allocations, bytes and peak live bytes on a document's first pass | exact counts | `memory` | CI table, for information |
| Q6 | Did a change make lexing slower? | instructions, head over base | exact counts | `lex` under callgrind | CI gate |
| Q7 | What is the worst one line can cost? | µs for each hostile line under the default limits | maximum | `hostile` | targets, on recorded runs |
| Q8 | How does nishiki compare with others? | Q1 to Q4 for each engine, and agreement with Shiki | ratio per file, geometric mean per language | `bench/compare/` | published |

Q7 and the binary size of each language feature come after the rest (tasks, group 8).
"Fast and small" is the README's first claim, so both belong here in the end.

### 2. A corpus of its own

The oracle fixtures stay the input of the callgrind gate, because they reach the most code paths.
The wall-clock programs and the comparison read a separate corpus, `bench/corpus/`, because:

- the oracle grows with fidelity work, and a new fixture moves every figure for its language;
- its files run from 0.3 KB to 100 KB, with no idea of size behind the choice;
- it holds Shiki's samples, which are short and unlike real files;
- comparators need plain source files, and the oracle stores text inside its scope records.

The corpus:

1. Three tiers per language, all real files, none repeated or generated:
   - small, 1 to 4 KB: a snippet or a code block in a reply;
   - medium, 16 to 64 KB: a typical source file;
   - large, 200 KB to 1 MB: a big real file, for the languages the comparison covers.
2. A reply set: Markdown replies with code blocks in several languages, nishiki's main use.
3. Each file is pinned to a commit, with its licence read at the source.
   Only permissive licences.
4. Each file is labelled by origin, as [syntax-highlighter-bench][shb] does:
   - neutral, a file no measured engine was tuned on, preferred;
   - oracle, a file nishiki's oracle also uses;
   - shared, a file another engine's benchmark uses, such as jQuery 2.1.4 in [syntect's][syntect-bench] and giallo's.
     A shared file lets a reader check our syntect figure against syntect's own.
5. `bench/corpus.toml` lists every file: language, tier, origin, source URL at its commit, licence and SHA-256.
   A check task fails when a file and its row disagree.
6. Changing the corpus starts a new corpus version.
   Records name the version, and figures from two versions are never compared.

A language's figure is total bytes over total time across its files in a tier.
It is not the mean of per-file speeds, which weighs a 1 KB file as much as a 1 MB one, as [tree-sitter's benchmark][ts-bench] does.

### 3. Two homes, split by what they measure

| Directory | Programs | Lockfile |
|:--|:--|:--|
| `examples/` in the root | `lex.rs` and `memory.rs`: deterministic counts that CI compares between base and head | each tree's own `Cargo.lock` |
| `bench/`, its own workspace | `throughput`, `styling`, `latency`, `cold`: wall time on a recorded host | `bench/Cargo.lock` |
| `bench/compare/`, its own workspace | the other engines and their harnesses | `bench/compare/Cargo.lock` |

The callgrind gate builds the base from `git archive` of the base commit, with the base's own lock.
A program in the root's `examples/` uses only the crate and its dev-dependencies, so it builds in either tree.
A program in the `bench/` workspace would carry head's lock into the base tree, and `--locked` fails when a pull request changes a dependency.
So the deterministic programs stay in the root, and everything else moves out.

`bench/compare/` is apart from `bench/` so that the comparators' dependencies, some of which build C, never enter the lock the core benches use.

`examples/bench.rs` is removed once its figures exist in `bench/`.

### 4. Criterion for throughput and styling, a custom program for the rest

Criterion measures Q1 and Q2:

- a bootstrap interval around every estimate, and outliers flagged rather than dropped;
- `Throughput::Bytes`, saved baselines (`--save-baseline`, `--baseline`) and JSON output;
- the harness syntect, giallo and lumis already use, so their authors can read ours.

Its model is one closure timed over many iterations.
Q3 needs a distribution over lines, and Q4 a fresh process per sample, so both are small programs in the same workspace.

Rejected:

- divan: no release since April 2025, no JSON output, no baselines.
- Keeping the custom example for everything: its faults are in the statistics, and Criterion already solves them.
- tango-bench: paired base and head timing, but early-stage, and its design changed in October 2026.
- hyperfine: it times a whole process, so exec and loading sit in every figure; `cold` times from `main`.

### 5. Throughput

- A document is lexed line by line from a fresh highlighter, as an editor opening a file does.
  The small tier then shows what creating a highlighter costs; the medium and large tiers show lexing.
- Criterion's defaults: 3 s warm-up, 5 s measurement, 100 samples.
- `styling` times `Theme::style` over the real token stream of each file, and the whole path from text to colours.
  The whole path is the figure Q8 compares, because every engine can produce colours.
- Creating a highlighter is timed on its own, warm, in nanoseconds.

### 6. Latency per line

1. Run P passes over each document, 11 by default, each from a fresh highlighter.
   Time every line on its own.
2. Measure the timer's overhead first, as the median of 10,000 empty timings, and subtract it.
   A line under four times that overhead is counted as below resolution, not given a number.
3. Pass 1 is first contact: the engine's lazy structures, such as the TextMate engine's lazy DFA, are still being built.
   Report its maximum apart.
4. For passes 2 to P, take each line's median.
   Report p50, p99 and the maximum over lines of those medians.

A per-line median removes machine noise, while the spread over lines is the property being measured.
Today one sample per line makes the maximum a measure of noise.

The 4 ms targets apply to the steady p99 and to the first-contact maximum.

### 7. Cold start

- `cold` runs once per process: it creates a highlighter, lexes the first line, then the rest of the document, and prints the three times from the start of `main`.
- A runner starts 31 processes per language and file, with the languages interleaved, and drops the first three.
- It reports the median and p90.
- Lazy state in the process (pattern tables, decoded grammars, lazy DFAs) starts empty.
  The binary is in the page cache, so this is a fresh process, not a cold disk; the record says so.
- `Theme::from_json` on each oracle theme is timed the same way.

### 8. Memory

- `memory.rs` lexes each oracle fixture once from a fresh highlighter and prints allocations, bytes and peak live bytes per language.
- It needs byte and peak counts from `nishiki-alloc-count`, which today counts calls only.
  Extending that crate keeps the one unsafe allocator in one place ([CODING_STANDARDS 3.3][standards]).
- The counts are exact, so the Callgrind job prints them for head and base beside the instruction table, for information.
- Zero allocations on a second pass stays a test (`tests/it/alloc.rs`).

DHAT under valgrind was the alternative: it needs no code, but counts the whole process, including reading the fixtures.

### 9. The callgrind gate keeps its guarantees

It keeps:

- base and head each built from their own tree and lock;
- one codegen unit for the gate, with the default sixteen shown for information only;
- only the second pass counted (`--toggle-collect`);
- only the fixtures both sides have;
- the 0.5% tolerance, and the note when sixteen units cost 3% more than one.

It changes:

- The language list comes from the fixture directories, not a hand-kept line.
  All 46 languages with fixtures are counted, not 27.
  [Bench rule 15][bench-rules] now states that the list comes from the fixture directories.
  The rule exists because engine pull requests collided on that line.
- Languages run in parallel, one valgrind per core, so the larger list fits the job's 20-minute step.
- The noise floor is measured and recorded.
  On an unchanged tree, languages read 0.9993 to 1.0003 in [#242's run][run-242].
  The cause is the arena's map of interned strings.
  The map keeps std's seeded hasher, so crafted text cannot make its keys collide.
  A deterministic map (`BTreeMap`) made the counts repeatable.
  It cost HTML 2% more instructions, and the map stays seeded (lead, 2026-11-04).
  The spread is at most 0.09%, five times under the gate.
  Bench rule 16 records it per language.
  The engine's maps keyed by arena ids move to the arena's fixed hasher, since no adversary picks those keys.

Rejected: gungraun.
It would add cache metrics and JSON output, but it compares runs of one bench binary, needs a runner binary pinned to its exact version, and gives the gate no guarantee it lacks.
It is worth revisiting if cache-miss counts are ever needed.

### 10. Wall time never gates CI

- Shared CI runners move wall time by more than any change we care about.
  The [first record][record] saw a factor of three on shell between runs.
- Instructions gate in CI (decision 9).
  Wall time counts only in recorded runs on a host class whose noise was measured.

### 11. Machine and noise

1. A probe picks the host class.
   It measures throughput six times per mode on one commit (an A/A run) on each candidate, unpinned and pinned to one core:
   an instabox box, a Namespace 4x8, a Namespace 8x16 and a Boat `large`.
   A candidate passes when its A/A medians agree within 2% on every language.
   The result is a research note.
2. No candidate passes, with the old harness ([first probe][probe]) or with Criterion ([second probe][probe-criterion]).
   So wall-clock records are indicative, and speed decisions rest on Callgrind instruction counts (decision 9).
3. The reference class is the least noisy candidate: Namespace 8x16, unpinned (lead, 2026-11-04).
   Its figures spread by 8.5% over six runs (median over languages), and pinning widened its drift.
   `bench/record.ts` writes a record only there.
   It checks the host's shape (8 CPUs, 2 threads a core, about 16 GB) and Namespace's `/nsc` directory.
   It refuses on any other host.
   Boat stays fine for Callgrind, whose counts do not depend on the host's speed.
   Boat's 37% spread rules it out for wall-clock work.
4. Each session measures an anchor at its start and end: one fixed file, lexed for 2 seconds.
   The bound is the reference class's measured first-to-last drift, 11%.
   It is the 90th percentile over languages, rounded up.
   A session whose anchor moves more than the bound is rejected and rerun.
   The record carries that bound beside its anchor drift.
5. At least five rounds of every figure; the median, with the minimum and maximum beside it, and its spread.
6. Where the host exposes `instructions:u`, the record carries it next to wall time.
   Neither instabox nor Namespace was shown to expose it ([first probe][probe]).

### 12. Results are data first

- A recorded run writes `bench/results/<host class>/<date>-<sha8>.json` with:
  the schema version, commit, corpus version, host (CPU, cores, memory, kernel), toolchain, anchor drift, compute minutes and cost, and every figure with its rounds and spread.
- `mise run bench:report` renders `<date>-<sha8>.md` from it.
  Nobody writes a record by hand.
- The raw harness output stays beside it as `<date>-<sha8>.txt`.
- `bench/README.md` carries the newest record's summary, rendered by the same task; a check fails when they disagree.
- The `bench/<full sha>` tag keeps the commit reachable ([bench rule 4][bench-rules]).
- `targets.toml` keeps its targets and its regression rule.
  The throughput floors are set from the first valid record, by a dated decision.

### 13. A comparison with other highlighters

Engines:

| Engine | Why | Runs as |
|:--|:--|:--|
| syntect 5.3.0, with two-face 0.5.2 | the incumbent in Rust: bat, delta | Rust, Oniguruma, the backend bat uses |
| tree-sitter-highlight 0.27.1, with 15 grammar crates | the parser-based approach of Zed and Helix | Rust, whole documents only |
| giallo 0.5.2 | a TextMate engine in Rust on Shiki's grammars: the closest output to ours | Rust, if the lead allows its licence |
| Shiki 4.6.0 | the fidelity reference, the version the oracle pins | Bun, Oniguruma and JavaScript engines |
| `@pierre/highlights` | the fastest lexers in the earlier comparison | Bun, optional |

The rules, each one a fault the [survey][survey] found elsewhere:

1. Every engine reads the same files; results are per file.
   The languages are the intersection, listed in the record.
2. Every engine colours with github-dark.
   syntect takes a theme built from Shiki's JSON rules.
   tree-sitter maps each capture name to a TextMate scope, and takes that scope's colour.
3. Setup, first document in a fresh process, and warm throughput are three figures, never one.
   lumis times setup inside every iteration for its Rust rows and outside for its JavaScript rows.
4. Every engine does the full work: Shiki's time and line limits are off, and nishiki's line reports must show no limit reached.
5. Every Rust engine builds with the default release profile and the same rustc, as nishiki's own figures do ([bench rule 11][bench-rules]).
6. Per-line engines lex line by line, carrying their state.
   tree-sitter takes whole documents, and appears only in the throughput and cold-start tables.
7. Each row carries the engine's token count and its agreement with Shiki's colours per non-whitespace byte.
   A faster engine that colours differently does less work, and the reader sees how much less.
8. The engine order rotates every round, at least five rounds, median reported.
9. The record names every engine's version, features and regex backend, and the harness commit.
10. Shiki's rows say they run in Bun.
    They answer what a user of each library gets, not which algorithm is faster.

The comparison runs on the reference host with each minor release.

syntect builds one regex backend into a binary.
The comparison runs Oniguruma, the backend bat uses.
The pure-Rust `fancy-regex` backend would need a second build, and is left for a later row.
Every comparator's licence is read at its source and listed in `bench/compare/README.md`.
That covers the 15 tree-sitter grammar crates and two-face's bundled syntaxes.
All are permissive except giallo.

Decided (operator, 2026-11-03): giallo is allowed under its EUPL-1.2 licence, built on the bench box only and never distributed.
`@pierre/highlights` is allowed once its licence is read at its source.

### 14. Dependencies

None enters the published crate.

| Dependency | Workspace | Licence | Reason |
|:--|:--|:--|:--|
| criterion 0.8.2, default features off | `bench/` | Apache-2.0 OR MIT | decision 4 |
| serde, serde_json | `bench/` | MIT OR Apache-2.0 | the JSON record; criterion already depends on both |
| syntect 5.3.0 | `bench/compare/` | MIT | decision 13; its default backend builds Oniguruma, in C |
| two-face 0.5.2 | `bench/compare/` | MIT OR Apache-2.0 | bat's syntax set, which adds to syntect's defaults |
| tree-sitter-highlight 0.27.1 and one grammar crate per language | `bench/compare/` | MIT; each grammar read before use | decision 13; builds C |
| giallo 0.5.2 | `bench/compare/` | EUPL-1.2, copyleft | decision 13; never distributed, built only on the bench host |
| `shiki@4.6.0` | `bench/compare/`, Bun | MIT | already pinned by `oracle/generate.ts` |
| `@pierre/highlights` | `bench/compare/`, Bun | not yet read | optional |

Decided (operator, 2026-11-03): every dependency in the table is approved.
criterion goes in with its default features off.
giallo and `@pierre/highlights` carry the conditions in decision 13.

valgrind already comes from the bench mise profile; no new tool is needed in CI.

### 15. Layout

```text
bench/
  README.md        the questions, how to run them, the newest results
  AGENTS.md        the rules
  Cargo.toml       workspace nishiki-bench, publish = false
  src/             corpus loader, statistics, JSON writer; bin/latency.rs, bin/cold.rs
  benches/         throughput.rs, styling.rs
  corpus.toml      the corpus manifest
  corpus/          <language>/<file>, README.md
  targets.toml
  report.ts        renders a record and the README summary
  results/         <host class>/<date>-<sha8>.{json,md,txt}
  compare/         own workspace and README: the other engines
examples/
  lex.rs           callgrind's program, unchanged
  memory.rs        allocation counts
```

`bench/` is excluded from the root workspace, so the root's lints, deny rules and versions never read it ([CODING_STANDARDS 5.3][standards]).
`bench/compare/` is excluded from `bench/`.

### 16. The rules

`bench/AGENTS.md` is rewritten around these decisions:

- kept: the cost check before a paid run, the record tag, output unchanged by a speed change, termination as a test, the default release profile, and the callgrind rules 12 to 14;
- replaced: what a local run is for (decision 10), the record format (decision 12), and the before-and-after rule (decision 17);
- replaced: rule 8, which puts comparisons under `research/`, a directory the `repo-layout` change removes.
  The new rule: every comparison with another engine runs in `bench/compare/`, a non-published workspace of its own, never a dependency of the crate or of `bench/` (decisions 3 and 13);
- rewritten: rule 15. It now says the language list comes from the fixture directories (decision 9).

### 17. A speed claim

A pull request that claims a speed change shows, in this order:

1. the Callgrind job's table at one codegen unit, which decides;
2. when wall time matters, Criterion on the reference host: base saved with `--save-baseline`, head compared with `--baseline`, each built from its own tree, run in the order base, head, head, base.
   The claim holds when both pairs agree in direction and Criterion's interval for the change excludes zero.

This replaces today's three runs and a median.

## Risks / Trade-offs

- [Criterion's dependencies slow the bench build] → default features off, and the bench workspace builds only in bench jobs.
- [The bench workspace drifts from the crate's API, because verify does not build it] → the bench job builds it and runs each program once, on pull requests labelled `bench` or touching `bench/`.
- [A comparator is set up unfairly by mistake] → the fidelity column exposes an engine that does less work, and the harness is committed beside the results.
- [No host class passes the probe] → decision 11, point 2: records are indicative, and speed decisions rest on instructions.
- [Forty-six languages make the Callgrind job slower] → parallel valgrind runs; the task that adds them measures the job and keeps it under its step timeout.
- [giallo's EUPL-1.2] → it is built only on the bench host and never distributed, the condition the operator set on 2026-11-03.
- [The corpus costs licence work] → `bench/corpus.toml` holds each file's licence source, and `check:corpus` refuses any licence off the permissive list.

## Migration Plan

1. The callgrind list and the memory table land first; they need no new dependency.
2. The corpus lands next, then the `bench/` workspace, which replaces `examples/bench.rs` and `mise-tasks/bench.sh`.
3. The first record under this method follows the host probe.
4. The comparison comes last.

The old record stays, frozen, as the last one under the old method.
Rollback is reverting a pull request; the crate never depends on any of this.

## Open Questions

- Which files fill each tier: chosen with their licences in the corpus task.
- Whether the memory counts gate later: decided once a few pull requests have shown how they move.
- Whether the README publishes a short dated summary of the comparison: decided when the first comparison exists.

[bench-rs]: https://github.com/victor-software-house/nishiki-archive/blob/fc6d39a0bb1df915e26d66243810318470602c82/examples/bench.rs
[bench-rules]: https://github.com/victor-software-house/nishiki-archive/blob/fc6d39a0bb1df915e26d66243810318470602c82/bench/AGENTS.md
[callgrind-sh]: https://github.com/victor-software-house/nishiki-archive/blob/fc6d39a0bb1df915e26d66243810318470602c82/mise-tasks/callgrind.sh
[callgrind-yml]: https://github.com/victor-software-house/nishiki-archive/blob/fc6d39a0bb1df915e26d66243810318470602c82/.github/workflows/callgrind.yml
[lex-rs]: https://github.com/victor-software-house/nishiki-archive/blob/fc6d39a0bb1df915e26d66243810318470602c82/examples/lex.rs
[record]: https://github.com/victor-software-house/nishiki-archive/blob/fc6d39a0bb1df915e26d66243810318470602c82/bench/results/namespace-4x8/2026-11-03-574fad99.md
[shb]: https://github.com/ryanatkn/syntax-highlighter-bench/blob/dcc0a8897aaf115337eadc7845ffa1179c69a4da/README.md
[standards]: https://github.com/victor-software-house/nishiki-archive/blob/fc6d39a0bb1df915e26d66243810318470602c82/CODING_STANDARDS.md
[survey]: ../../research/2026-11-03-benchmark-practice.md
[syntect-bench]: https://github.com/trishume/syntect/blob/4efbec173db6ea83caaf7a34994cb7e4d038a13e/benches/highlighting.rs
[targets]: https://github.com/victor-software-house/nishiki-archive/blob/fc6d39a0bb1df915e26d66243810318470602c82/bench/targets.toml
[ts-bench]: https://github.com/tree-sitter/tree-sitter/blob/9632b561db2f647e9c0be9e61cf91f10671a6491/crates/cli/benches/benchmark.rs
[run-242]: https://github.com/victor-software-house/nishiki-archive/actions/runs/41754856763
[probe]: ../../research/2026-11-04-bench-host-probe.md
[probe-criterion]: ../../research/2026-11-04-bench-host-probe-criterion.md
