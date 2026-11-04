# Tasks

Each group is one pull request unless it says otherwise.
Each lands the docs its work changes: `bench/AGENTS.md`, `bench/README.md` and the root guide's Benchmarks section.

## 1. Decisions

- [x] 1.1 The lead approves or declines each dependency in design decision 14 → verify: the answer is recorded under decision 14, dated
  - 2026-11-04 UTC (2026-11-03 BRT): the operator approved every dependency, criterion with its default features off; recorded under decision 14.
    The lead relayed it after opening [#239][pr-239].
- [x] 1.2 The lead decides on giallo's EUPL-1.2 licence in a never-distributed comparison → verify: the answer is recorded under decision 13, dated
  - 2026-11-04 UTC (2026-11-03 BRT): the operator allowed giallo, built on the bench box only and never distributed, and `@pierre/highlights` once its licence is read at its source; recorded under decision 13.

## 2. The callgrind gate over every language, and memory counts

No new dependency.

- [x] 2.0 Measure the instruction counts' noise floor: run `examples/lex.rs` under callgrind several times on one build, find what differs between runs, remove each cause that costs nothing to remove, and record the spread that stays in bench rule 16 → verify: bench rule 16 lists the measured spread per language, and every spread is under the 0.5% gate
  - 2026-11-04 UTC: two instabox runs, three builds at one codegen unit, three to six counts each.
    - The arena's map of interned strings (std's seeded hasher) is the cause for the HTML family, Python and the TextMate languages. A fixed hasher or a `BTreeMap` there made all of them repeatable.
    - The `BTreeMap` cost HTML 2.04% more instructions; html-derivative, erb and Markdown held parity. The lead kept the seeded map for its resistance to crafted collisions.
    - The engine's two maps keyed by arena ids moved to the arena's `IdHasher`; alone, that left the counts as noisy as before.
    - Running valgrind with address randomization off (`setarch -R`) changed nothing.
    - A residual of up to 0.02% in Markdown, Ruby and C stays with every variant; its cause was not looked for further.
    - Six counts of main spread by 0.09% at most (haskell, erb), recorded in bench rule 16.
- [x] 2.1 `mise-tasks/callgrind.sh` takes its default languages from the fixture directories and runs one valgrind per core; bench rule 15 says so → verify: the pull request's Callgrind job prints a row for each of the 46 languages, and its step finishes inside the 20-minute timeout
  - 2026-11-04 UTC: [#242][pr-242]'s [Callgrind run][run-242] printed 46 rows and passed; its compare step took 2 min 4 s, against 4.5 to 5 min for 27 languages on the four runs before it.
    Untouched languages read 0.9993 to 1.0003 there, so the counts carry some run-to-run noise of their own, well inside the 0.5% gate; its cause is not established.
- [x] 2.2 `nishiki-alloc-count` counts bytes and peak live bytes beside calls → verify: `tests/it/alloc.rs` passes in CI unchanged, and a unit test in the crate covers the new counts
  - 2026-11-04 UTC: [#244][pr-244] merged with CI green: `measure` reports allocations, bytes and peak live bytes; its unit tests and Miri passed, and `tests/it/alloc.rs` ran unchanged.
- [ ] 2.3 `examples/memory.rs` prints each language's allocations, bytes and peak over one pass of its fixtures, and the Callgrind job shows it for base and head → verify: the job summary of the pull request shows the memory table, and the job passes

## 3. The reference host

- [x] 3.1 Run the A/A probe of design decision 11 on an instabox box, a Namespace 4x8 and a Namespace 8x16 pinned to one core, and check `perf stat -e instructions:u` on each → verify: a research note under `openspec/research/` names the reference host class, each candidate's A/A spread per language, and each run's minutes and cost
  - 2026-11-04 UTC: [the probe note](../../research/2026-11-04-bench-host-probe.md): no candidate passed; 14 to 33 of 46 languages moved by more than 2% between the halves of one session on every host and mode, and pinning did not fix it.
    instabox has no `perf` for its kernel; on Namespace the `perf` install failed silently. Compute: about 36 Namespace unit minutes ($0.06) and 460 s of instabox.
- [ ] 3.2 Run the same probe with the Criterion harness of task 5.1, before the first record (task 6.3) → verify: a research note gives each candidate's A/A gap and spread per language under the new harness, and names the reference host class or confirms decision 11, point 6

The probe uses the current `examples/bench.rs` throughput figure, so it does not wait for group 5.

## 4. The corpus

- [x] 4.1 `bench/corpus/` with small and medium files for every language, large files for the languages the comparison covers, and the reply set; `bench/corpus.toml` lists each file's language, tier, origin, source at its commit, licence and its source, size and SHA-256 → verify: every source and licence link resolves (`gh api` for each repository), and no file is copyleft or unlicensed
  - 2026-11-04 UTC: 111 files over all 46 languages: small and medium for each, large for 16 (CSS has none: every CSS file found in range was generated or vendored), and three replies.
    Each licence was read at its source; all are MIT, Apache-2.0, BSD-2-Clause, BSD-3-Clause, ISC, CC0-1.0, or dual MIT with Apache-2.0 or the Unlicense.
    All 219 source and licence links resolved through `gh api`. 110 files are neutral; jQuery 2.1.4 is shared with syntect's and giallo's benchmarks.
- [x] 4.2 A check task fails when a corpus file and its manifest row disagree, or a file has no row, and `check:local` runs it → verify: CI passes, and the pull request shows the check failing on a deliberately edited copy
  - 2026-11-04 UTC: `mise-tasks/check/corpus.sh` also fails on a size outside its tier, an unpinned source, or a licence off the permissive list.
    Locally, one byte appended to `c/nginx-ngx-hash.c` and a stray `c/stray.c` gave three failures (size, SHA-256, no row); the restored tree passes. CI's verify runs it through `check:local`.

## 5. The `bench/` workspace

- [ ] 5.1 The workspace, its corpus loader, and Criterion benches `throughput` and `styling`; `mise run bench` runs them; `examples/bench.rs` and the old `mise-tasks/bench.sh` go; `bench/README.md` says what each question measures and how to run it, and `mise-tasks/check/layout.sh` moves `bench` from `guide_only` to `both` → verify: a bench job, on pull requests labelled `bench` or touching `bench/`, builds the workspace with `--locked` and runs every bench once in test mode, and passes
- [ ] 5.2 `latency`: repeated passes, timer overhead subtracted, first contact apart, as design decision 6 → verify: the bench job runs it on the small tier with three passes, and its output names steady p50, p99 and maximum, the first-contact maximum, and the count of lines below resolution
- [ ] 5.3 `cold`: one sample per process, a runner that interleaves languages and drops the first three samples, and theme loading → verify: the bench job runs it with five processes for one language, and its output gives the median and p90 of each of the three times

## 6. Records

- [ ] 6.1 The JSON record of design decision 12, `bench/report.ts` to render a record and the README summary, and a check that the README matches the newest record → verify: CI passes, and the pull request shows the check failing when the README summary is edited by hand
- [ ] 6.2 Rewrite `bench/AGENTS.md` as design decision 16 says, and update `targets.toml` to the new figures (steady p99 and first-contact maximum) → verify: every kept rule is still present, and `rg` finds no rule naming `examples/bench.rs` or `research/`
- [ ] 6.3 The first record under this method, on the reference host, with the `bench/<full sha>` tag pushed; set the throughput floors in `targets.toml` by a dated decision → verify: the record's anchor drift is under 2%, the rendered summary matches the JSON, and `git ls-remote` shows the tag

## 7. The comparison

- [ ] 7.1 `bench/compare/`, its own workspace: syntect with two-face, both regex backends, and tree-sitter-highlight with the grammar crates for the shared languages; github-dark as a `.tmTheme` converted from Shiki's file; colour agreement with Shiki per non-whitespace byte → verify: on one box, each engine's run on the small tier prints setup, first-document and warm figures, its token count and its agreement, and the run fails when nishiki reports a limit
- [ ] 7.2 giallo in the same workspace, if 1.2 allows it → verify: the same output as 7.1 for giallo
- [ ] 7.3 Shiki 4.6.0 in Bun, with both engines and its limits off, and `@pierre/highlights` if 1.1 allows it → verify: the same output as 7.1 for each
- [ ] 7.4 The first recorded comparison on the reference host, rendered into `bench/compare/README.md` → verify: the record lists every engine's version, features and backend, the languages left out, and the rounds; its anchor drift is under 2%

## 8. Later questions

- [ ] 8.1 Q7: move each language's hostile lines from `tests/it/limits/` into data that both the limit tests and a `hostile` bench read, and report the slowest line per language under the default limits → verify: the limit tests pass in CI unchanged in what they check, and a recorded run shows the new figure against the 4 ms target
- [ ] 8.2 The stripped binary size each language feature adds, in recorded runs → verify: a record lists the size of a build with no language, and the growth for each language alone

## Workflow follow-up

- Archive the change once groups 2 to 7 are done, with `openspec archive bench-methodology`.
- Group 8 can stay open in a later change if it has not started by then.

[pr-239]: https://github.com/victor-software-house/nishiki-archive/pull/239
[pr-242]: https://github.com/victor-software-house/nishiki-archive/pull/242
[run-242]: https://github.com/victor-software-house/nishiki-archive/actions/runs/41754856763
[pr-244]: https://github.com/victor-software-house/nishiki-archive/pull/244
