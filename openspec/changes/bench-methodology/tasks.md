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
    - The `BTreeMap` cost HTML 2.04% more instructions.
      html-derivative, erb and Markdown held parity.
      The lead kept the seeded map, because it resists crafted collisions.
    - The engine's two maps keyed by arena ids moved to the arena's `IdHasher`.
      That change alone left the counts as noisy as before.
    - Running valgrind with address randomization off (`setarch -R`) changed nothing.
    - A residual of up to 0.02% in Markdown, Ruby and C stays with every variant.
      Its cause was not looked for further.
    - Six counts of main spread by 0.09% at most (haskell, erb), recorded in bench rule 16.
- [x] 2.1 `mise-tasks/callgrind.sh` takes its default languages from the fixture directories and runs one valgrind per core; bench rule 15 says so → verify: the pull request's Callgrind job prints a row for each of the 46 languages, and its step finishes inside the 20-minute timeout
  - 2026-11-04 UTC: [#242][pr-242]'s [Callgrind run][run-242] printed 46 rows and passed.
    Its compare step took 2 min 4 s.
    The four runs before it took 4.5 to 5 min for 27 languages.
    Untouched languages read 0.9993 to 1.0003 there.
    The counts carry some run-to-run noise of their own, well inside the 0.5% gate.
    The noise's cause is not established.
- [x] 2.2 `nishiki-alloc-count` counts bytes and peak live bytes beside calls → verify: `tests/it/alloc.rs` passes in CI unchanged, and a unit test in the crate covers the new counts
  - 2026-11-04 UTC: [#244][pr-244] merged with CI green.
    `measure` reports allocations, bytes and peak live bytes.
    Its unit tests and Miri passed.
    `tests/it/alloc.rs` ran unchanged.
- [x] 2.3 `examples/memory.rs` prints each language's allocations, bytes and peak over one pass of its fixtures, and the Callgrind job shows it for base and head → verify: the job summary of the pull request shows the memory table, and the job passes
  - 2026-11-04 UTC: [#270][pr-270] added `examples/memory.rs` and the memory table below the instruction table.
    On [#360's Callgrind run][run-360], the job printed the table for base and head and passed.

## 3. The reference host

- [x] 3.1 Run the A/A probe of design decision 11 on an instabox box, a Namespace 4x8 and a Namespace 8x16 pinned to one core, and check `perf stat -e instructions:u` on each → verify: a research note under `openspec/research/` names the reference host class, each candidate's A/A spread per language, and each run's minutes and cost
  - 2026-11-04 UTC: [the probe note](../../research/2026-11-04-bench-host-probe.md) found no candidate that passed.
    On every host and mode, 14 to 33 of 46 languages moved by more than 2% between the halves of one session.
    Pinning did not fix it.
    instabox has no `perf` for its kernel.
    On Namespace the `perf` install failed silently.
    Compute: about 36 Namespace unit minutes ($0.06) and 460 s of instabox.
- [x] 3.2 Run the same probe with the Criterion harness of task 5.1, before the first record (task 6.3) → verify: a research note gives each candidate's A/A gap and spread per language under the new harness, and names the reference host class or confirms that no host passes
  - 2026-11-04 UTC: [the second probe note](../../research/2026-11-04-bench-host-probe-criterion.md) found no host that passed.
    Each host's per-language lines sit beside the note.
    On every host and mode, Boat `large` included, 13 to 46 of 46 languages moved by more than 2%.
    Namespace 8x16, unpinned, is the reference class because it is the least noisy.
    Its median spread is 8.5%, and its first-to-last drift is 1.9% median and 10.2% p90.
    The anchor bound is 11% (decision 11, points 3 and 4, lead).
    Compute: about 565 Namespace unit minutes ($0.85), 2,940 s of instabox, and 4,686 s of Boat `large`.
    1,836 s of the Boat time was a run that `boat exec`'s timeout killed.

The probe uses the current `examples/bench.rs` throughput figure, so it does not wait for group 5.

## 4. The corpus

- [x] 4.1 `bench/corpus/` with small and medium files for every language, large files for the languages the comparison covers, and the reply set; `bench/corpus.toml` lists each file's language, tier, origin, source at its commit, licence and its source, size and SHA-256 → verify: every source and licence link resolves (`gh api` for each repository), and no file is copyleft or unlicensed
  - 2026-11-04 UTC: 111 files over all 46 languages.
    Every language has a small and a medium file, 16 have a large one, and there are three replies.
    CSS has no large file, because every CSS file found in range was generated or vendored.
    Each licence was read at its source.
    All are MIT, Apache-2.0, BSD-2-Clause, BSD-3-Clause, ISC, CC0-1.0, or dual MIT with Apache-2.0 or the Unlicense.
    All 219 source and licence links resolved through `gh api`.
    110 files are neutral.
    jQuery 2.1.4 is shared with syntect's and giallo's benchmarks.
- [x] 4.2 A check task fails when a corpus file and its manifest row disagree, or a file has no row, and `check:local` runs it → verify: CI passes, and the pull request shows the check failing on a deliberately edited copy
  - 2026-11-04 UTC: `mise-tasks/check/corpus.sh` also fails on a size outside its tier, an unpinned source, or a licence off the permissive list.
    Locally, one byte appended to `c/nginx-ngx-hash.c` and a stray `c/stray.c` gave three failures (size, SHA-256, no row).
    The restored tree passes.
    CI's verify runs it through `check:local`.

## 5. The `bench/` workspace

- [x] 5.1 The workspace, its corpus loader (language and tier read from `corpus/<language>/<tier>/`), and Criterion benches `throughput` and `styling`; `mise run bench` runs them; `bench/README.md` says what each question measures and how to run it, and `mise-tasks/check/layout.sh` moves `bench` from `guide_only` to `both`, dropping the list now empty → verify: a Bench job, on pull requests labelled `bench`, runs `mise run bench:check` (rustfmt, clippy with warnings denied, and every bench once in Criterion's untimed test mode, all with `--locked`), and passes; it times nothing, so wall time cannot gate it
  - 2026-11-04 UTC: [#273][pr-273] added the workspace, its corpus loader and the `throughput` and `styling` benches.
    On [#360's run][run-360], the Bench job ran `mise run bench:check` and passed.
- [x] 5.2 `latency`: repeated passes, timer overhead subtracted, first contact apart, as design decision 6 → verify: the bench job runs it on the small tier with three passes, and its output names steady p50, p99 and maximum, the first-contact maximum, and the count of lines below resolution
  - 2026-11-04 UTC: [#276][pr-276] added `latency`.
    On [#360's run][run-360], the Bench job ran it on the small tier with three passes.
    Its table named steady p50, p99 and maximum, the first-contact maximum and the lines below resolution.
- [x] 5.3 `cold`: one sample per process, and `examples/bench.rs` goes once 5.2 and 5.3 give its figures; a runner that interleaves languages and drops the first three samples, and theme loading → verify: the bench job runs it with five processes for one language, and its output gives the median and p90 of each of the three times
  - 2026-11-04 UTC: [#277][pr-277] added `cold` and removed `examples/bench.rs`.
    On [#360's run][run-360], the Bench job ran it with five processes for one file.
    Its table gave the median and p90 of each of the three times, and the theme's load time.

## 6. Records

- [x] 6.1 The JSON record of design decision 12, `bench/report.ts` to render a record and the README summary, and a check that the README matches the newest record → verify: CI passes, and the pull request shows the check failing when the README summary is edited by hand
  - 2026-11-04 UTC: [#317][pr-317] added the recorder and [#318][pr-318] the report and its check.
    Both merged with CI green.
    Locally, `check` failed when a number in the README summary was edited by hand.
- [x] 6.2 Rewrite `bench/AGENTS.md` as design decision 16 says, and update `targets.toml` to the new figures (steady p99 and first-contact maximum) → verify: every kept rule is still present, and `rg` finds no rule naming `examples/bench.rs` or `research/`
  - 2026-11-04 UTC: `bench/AGENTS.md` follows design decision 16.
    Rules 1, 3, 5 and 8 are replaced, and rule 15 was already rewritten.
    Rules 2, 4, 9, 10, 11 and 12 to 14 keep their content.
    `rg 'examples/bench.rs|research/' bench/AGENTS.md` finds nothing.
    `targets.toml` binds the steady p99 and the first-contact maximum to 4 ms (lead).
    The first record misses the first-contact target on bat, cmake, haskell and powershell at the medium tier, and on c at the large tier.
- [x] 6.3 The first record under this method, on the reference host, with the `bench/<full sha>` tag pushed; set the throughput floors in `targets.toml` by a dated decision → verify: the record's anchor drift is under the reference class's bound (decision 11, point 4), the rendered summary matches the JSON, and `git ls-remote` shows the tag
  - 2026-11-04 UTC: the record is `bench/results/namespace-8x16/2026-11-04-0a58343a.json`.
    It ran main `0a58343a` on a Namespace 8x16, five rounds.
    The anchor moved 2.24%, inside the 11% bound.
    `check:bench-report` passes.
    `git ls-remote` shows the tag `bench/0a58343ad39d593b1565956ed1f7792023043a29`.
    A throughput floor is a language's lowest round less 11%, rounded down to 0.1 MiB/s (lead).
    First-contact and first-line times spread by about 30% and get no floor yet.
    Compute: 6,210 s, about 828 unit minutes, $1.24.

## 7. The comparison

- [x] 7.1 `bench/compare/`, its own workspace: syntect with two-face on Oniguruma, and tree-sitter-highlight with the grammar crates for the shared languages; github-dark built from Shiki's JSON theme; colour agreement with Shiki per non-whitespace byte → verify: on one box, each engine's run on the small tier prints setup, first-document and warm figures, its token count and its agreement, and the run fails when nishiki reports a limit
  - 2026-11-04 UTC: on one instabox box, `--tier small` ran on Rust and JSON.
    It printed setup, first document from a fresh process, warm MiB/s, tokens and agreement for nishiki, syntect and tree-sitter.
    nishiki agreed with Shiki on every byte, syntect on 59 to 63%, tree-sitter on 59 to 73%.
    With a 10-byte line limit patched in on the box, the run stopped before its first row with `rust/small/anyhow-chain.rs: nishiki reached a limit` and exit code 1.
    clippy with warnings denied passes with and without giallo.
    The Bench job lints the workspace without giallo.
    These are one box's figures, not a record.
    The record is task 7.4.
- [x] 7.2 giallo in the same workspace, if 1.2 allows it → verify: the same output as 7.1 for giallo
  - 2026-11-04 UTC: the same run with `--features giallo` printed giallo's row for each file.
    giallo agreed with Shiki on every byte.
    giallo's setup took 38 to 41 ms.
    It builds only on the bench box, behind the off-by-default feature.
- [ ] 7.3 Shiki 4.6.0 in Bun, with both engines and its limits off, and `@pierre/highlights` if 1.1 allows it → verify: the same output as 7.1 for each
  - 2026-11-04 UTC: `shiki.ts reference` wrote the colours of all 111 corpus files, and `shiki.ts time 1` printed setup, first document and warm MiB/s for both engines on all of them (222 rows) on an instabox box.
    Not done: `time` prints no token count and no agreement.
    The JavaScript engine is not yet scored against Oniguruma.
    `@pierre/highlights` is not added.
- [ ] 7.4 The first recorded comparison on the reference host, rendered into `bench/compare/README.md` → verify: the record lists every engine's version, features and backend, the languages left out, and the rounds; its anchor drift is under the reference class's bound

## 8. Later questions

Moved to the `bench-later-questions` change (lead, 2026-11-04).

## Workflow follow-up

- Archive the change once groups 2 to 7 are done, with `openspec archive bench-methodology`.

[pr-239]: https://github.com/victor-software-house/nishiki-archive/pull/239
[pr-270]: https://github.com/victor-software-house/nishiki-archive/pull/270
[pr-273]: https://github.com/victor-software-house/nishiki-archive/pull/273
[pr-276]: https://github.com/victor-software-house/nishiki-archive/pull/276
[pr-277]: https://github.com/victor-software-house/nishiki-archive/pull/277
[pr-317]: https://github.com/victor-software-house/nishiki-archive/pull/317
[pr-318]: https://github.com/victor-software-house/nishiki-archive/pull/318
[run-360]: https://github.com/victor-software-house/nishiki-archive/actions/runs/41818929609
[pr-242]: https://github.com/victor-software-house/nishiki-archive/pull/242
[run-242]: https://github.com/victor-software-house/nishiki-archive/actions/runs/41754856763
[pr-244]: https://github.com/victor-software-house/nishiki-archive/pull/244
