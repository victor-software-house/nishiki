# Tasks

## 1. Measure

- [x] 1.1 Write `oracle/coverage.ts` (design, sections 2 and 3).
  It holds units and reachability per hand-lexed grammar, the comment tags, the wrapped engine, and the run over every fixture's text.
  → verify: on `swift` it finds the 264 matched expressions of the 2026-11-04 measurement.
  The instrumented scope stacks equal every fixture's.
  - 2026-11-04 UTC, [PR 238][pr238]: over all fixtures, every scope stack came out equal with the tags in place, and with one scope of `tests/oracle/ini/black-tox.txt` edited the run failed and named it. Swift: 267 of 448 expressions, the 264 of its own fixtures and 3 from Swift fences in other fixtures.
- [x] 1.2 Settle injection scans as vscode-textmate does, for HTML, html-derivative and ERB → verify: a test document where the rule list wins and one where the injection wins credits only the winner
  - 2026-11-04 UTC: on the fixtures, whose `<` always opens a tag, the rule list wins every tie and HTML's injection `<` is not counted (131 of 227 expressions, 132 before settling). With a probe fixture `<p>a < b</p>` added, the injection wins at the lone `<` and HTML reads 132 of 227; the probe was then removed.
- [x] 1.3 Measure the fixed fuzz corpus from `oracle/fuzz.ts` in a temporary directory → verify: two runs report the same fuzz column
  - 2026-11-04 UTC: 100 mutated windows per hand-lexed language, and 100 generated documents for C++, C#, Lua and Swift, at seed 1. Two runs wrote the same report, fuzz column included.
- [x] 1.4 Write `tests/oracle/COVERAGE.md` from `mise run oracle:coverage` (design, section 4) → verify: a second run leaves it byte for byte the same, and every hand-lexed grammar of the design's table has a row
  - 2026-11-04 UTC: `mise run oracle:coverage` wrote 36 rows, one per grammar of the design's table, in 28 seconds; a second run left the file byte for byte the same.

## 2. Gate

- [x] 2.1 Add `oracle/coverage-waivers.ts`, with its complete list empty, and fail on a waiver whose unit a fixture exercises → verify: a waiver on an exercised Swift unit fails the task with its path
  - 2026-11-04 UTC: a waiver on Swift's exercised `#comments.patterns[5].begin` failed the run with "is exercised by a fixture: remove its waiver". A waiver on the unexercised `#comments.patterns[2].begin` failed without a `probe`, since Swift has probe support, and passed with one, the report reading `waived:` and the reason. A waiver on a path the grammar lacks failed with "names no unit the report counts".
- [x] 2.2 Add `mise run check:oracle-coverage` and `.github/workflows/coverage.yml`, after `tables.yml` → verify: on the pull request, the check passes; on a scratch branch, a stale report fails it, and so does a complete grammar with a gap
  - 2026-11-04 UTC, locally: the check passed on the committed report; with a line added to the report it failed and printed the difference; with `rust` listed complete it failed and named Rust's unexercised paths. The first run on [PR 252][pr252] failed: `fuzz.ts` read the fixtures in `readdirSync` order, so CI's Linux runner drew another fuzz corpus; with the fixtures sorted, [the run on `c4ca16d9`][run-252] passed.
- [x] 2.3 Document the report, the waivers and the gate in `AGENTS.md` under "The Shiki oracle", and in `tests/oracle/SOURCES.md`'s note on inputs → verify: `mise run lint:md` passes
  - 2026-11-04 UTC: `AGENTS.md` item 6 under "The Shiki oracle" and a sentence in `tests/oracle/SOURCES.md`; `mise run lint:md` and `typos` pass.

## 3. Close the gaps

Each task adds inputs, and the fixes they expose, until the grammar is complete, then lists it complete.
Each verifies the same way: the grammar's row reads complete in `tests/oracle/COVERAGE.md`, and the oracle test and the coverage check pass in CI.

- [x] 3.1 C#: `csharp`
  - 2026-11-04 UTC, [PR 261][pr261]: twenty-two C# inputs added.
    C# is complete with twelve waivers, each with its probe.
    On `a1288bbe` the [oracle test][run-261-ci] and the [coverage check][run-261-cov] passed.
- [x] 3.2 Swift: `swift`
  - 2026-11-04 UTC, [PR 309][pr309]: fifteen swiftlang/swift parser tests and a written input added.
    Swift is complete with 30 waived units.
    18 of them are operator patterns inside captures that start with `\G`.
    On `0d6097a5` the [oracle test][run-309-ci] and the [coverage check][run-309-cov] passed.
- [x] 3.3 Python: `python`
  - 2026-11-04 UTC, [PR 288][pr288] and [PR 289][pr289]: seven black test cases, one ruff parser fixture and a written input added.
    The lexer now closes a docstring at a line feed as Shiki does.
    Python is complete with 62 waived units.
    On `18e2a261` the [oracle test][run-289-ci] and the [coverage check][run-289-cov] passed.
- [x] 3.4 Go: `go`
  - 2026-11-04 UTC, [PR 339][pr339] and [PR 340][pr340]: Go inputs and a written input added.
    Go is complete with 29 waived units.
    On `add9c1dd` the [oracle test][run-340-ci] and the [coverage check][run-340-cov] passed.
- [x] 3.5 Java: `java`
  - 2026-11-04 UTC, [PR 333][pr333]: google-java-format inputs and a written input added.
    Java is complete with 1 waived unit.
    On `dd4e13fb` the [oracle test][run-333-ci] and the [coverage check][run-333-cov] passed.
- [x] 3.6 TypeScript: `typescript`, `javascript`, `jsx`, `tsx`
  - 2026-11-04 UTC, [PR 281][pr281], [PR 275][pr275] and [PR 282][pr282]: 67 TypeScript test cases and a written input added.
    The lexer now reads a Unicode digit in a numeric object key.
    The four grammars are complete with seven waivers each.
    On `869e82a4` the [oracle test][run-282-ci] and the [coverage check][run-282-cov] passed.
- [x] 3.7 C and C++: `c`, `cpp`, `cpp-macro`
  - 2026-11-04 UTC, [PR 297][pr297], [PR 312][pr312] and [PR 313][pr313]: 22 picked inputs and two written ones added.
    The written inputs put comments in the gaps the grammars capture.
    `c`, `cpp` and `cpp-macro` are complete with 44, 234 and 131 waived units.
    On `1f2a1d5f` the [oracle test][run-313-ci] and the [coverage check][run-313-cov] passed.
- [x] 3.8 Rust: `rust`
  - 2026-11-04 UTC, [PR 259][pr259]: `rust/core-macros` and `rust/coverage` added.
    Rust is complete with one waiver.
    On `b672f999` the [oracle test][run-259-ci] and the [coverage check][run-259-cov] passed.
- [x] 3.9 Ruby: `ruby`
  - 2026-11-04 UTC, [PR 327][pr327] and [PR 328][pr328]: rack, rake and RuboCop inputs and a written input added.
    Ruby is complete with 17 waived units.
    On `6c824aab` the [oracle test][run-328-ci] and the [coverage check][run-328-cov] passed.
- [x] 3.10 Kotlin: `kotlin`
  - 2026-11-04 UTC, [PR 334][pr334]: a written input added.
    Kotlin is complete with 2 waived units.
    On `49a7c05f` the [oracle test][run-334-ci] and the [coverage check][run-334-cov] passed.
- [x] 3.11 Lua: `lua`
  - 2026-11-04 UTC, [PR 315][pr315] and [PR 425][pr425]: lua/lua and LuaLS inputs and a written input added.
    The Lua lexer now lexes an `ffi.cdef[[ ]]` body as C.
    Lua is complete with 7 waived units.
    On `ceba1f88` the [oracle test][run-425-ci] and the [coverage check][run-425-cov] passed.
- [x] 3.12 shell: `shellscript`
  - 2026-11-04 UTC, [PR 439][pr439] and [PR 440][pr440]: bats-core, docker-install, Homebrew and Oh My Zsh inputs and a written input added.
    The TextMate engine now splits a filled scope name at spaces only.
    shell is complete with 13 waived units.
    On `30c22c12` the [oracle test][run-440-ci] and the [coverage check][run-440-cov] passed.
- [x] 3.13 Markdown: `markdown`
  - 2026-11-04 UTC, [PR 303][pr303], [PR 298][pr298] and [PR 299][pr299]: two markdownlint test files and a written input added.
    PHP, Batch and XSL fences now read as Shiki reads them.
    The engine keeps each grammar's caches across a grammar switch.
    Markdown is complete with 213 waived units.
    On `2b67928e` the [oracle test][run-299-ci] and the [coverage check][run-299-cov] passed.
- [x] 3.14 CSS, Less and SCSS: `css`, `less`, `scss`
  - 2026-11-04 UTC, [PR 287][pr287]: less.js, Angular Components and Bootstrap inputs and a written input per language added.
    The three grammars are complete with 6, 29 and 50 waived units.
    On `dd3a3df5` the [oracle test][run-287-ci] and the [coverage check][run-287-cov] passed.
- [ ] 3.15 HTML, html-derivative and XML: `html`, `html-derivative`, `xml`
  - 2026-11-04 UTC, [PR 447][pr447]: two written inputs added for XML.
    `xml` is complete with 11 waived units.
    On `00603eab` the [oracle test][run-447-ci] and the [coverage check][run-447-cov] passed.
    `html` and `html-derivative` stay open.
- [x] 3.16 ERB: `erb`
  - 2026-11-04 UTC, [PR 444][pr444]: a written input added.
    ERB is complete with 4 waived units.
    On `9f559133` the [oracle test][run-444-ci] and the [coverage check][run-444-cov] passed.
- [x] 3.17 JSON: `json`, `jsonc`, `jsonl`
  - 2026-11-04 UTC, [PR 436][pr436]: a written input per grammar added.
    The three grammars are complete with no waived unit.
    On `95304367` the [oracle test][run-436-ci] and the [coverage check][run-436-cov] passed.
- [x] 3.18 YAML and TOML: `yaml`, `toml`
  - 2026-11-04 UTC, [PR 306][pr306] and [PR 307][pr307]: toml-test and YAML test suite files and a written input per language added.
    The YAML lexer now reads a directive's name without a copy.
    TOML and YAML are complete with 14 and 6 waived units.
    On `bf5e8ec5` the [oracle test][run-307-ci] and the [coverage check][run-307-cov] passed.
- [x] 3.19 SQL: `sql`
  - 2026-11-04 UTC, [PR 437][pr437]: a written input added.
    SQL is complete with 7 waived units.
    On `804cf8fa` the [oracle test][run-437-ci] and the [coverage check][run-437-cov] passed.
- [x] 3.20 GraphQL: `graphql`
  - 2026-11-04 UTC, [PR 450][pr450]: a written input added.
    GraphQL is complete with 27 waived units.
    On `855d0da7` the [oracle test][run-450-ci] and the [coverage check][run-450-cov] passed.
- [x] 3.21 diff, Dockerfile, INI and Make: `diff`, `docker`, `ini`, `make`
  - 2026-11-04 UTC, [PR 320][pr320]: a written input per language added.
    The four grammars are complete with 1, 8, 2 and 4 waived units.
    On `3de20c2f` the [oracle test][run-320-ci] and the [coverage check][run-320-cov] passed.

[pr238]: https://github.com/victor-software-house/nishiki-archive/pull/238
[pr252]: https://github.com/victor-software-house/nishiki-archive/pull/252
[run-252]: https://github.com/victor-software-house/nishiki-archive/actions/runs/41756482197
[pr259]: https://github.com/victor-software-house/nishiki-archive/pull/259
[run-259-ci]: https://github.com/victor-software-house/nishiki-archive/actions/runs/41757301609
[run-259-cov]: https://github.com/victor-software-house/nishiki-archive/actions/runs/41757301124
[pr261]: https://github.com/victor-software-house/nishiki-archive/pull/261
[run-261-ci]: https://github.com/victor-software-house/nishiki-archive/actions/runs/41764640336
[run-261-cov]: https://github.com/victor-software-house/nishiki-archive/actions/runs/41764639889
[pr275]: https://github.com/victor-software-house/nishiki-archive/pull/275
[pr281]: https://github.com/victor-software-house/nishiki-archive/pull/281
[pr282]: https://github.com/victor-software-house/nishiki-archive/pull/282
[run-282-ci]: https://github.com/victor-software-house/nishiki-archive/actions/runs/41769127920
[run-282-cov]: https://github.com/victor-software-house/nishiki-archive/actions/runs/41769127433
[pr287]: https://github.com/victor-software-house/nishiki-archive/pull/287
[run-287-ci]: https://github.com/victor-software-house/nishiki-archive/actions/runs/41772490259
[run-287-cov]: https://github.com/victor-software-house/nishiki-archive/actions/runs/41772489900
[pr288]: https://github.com/victor-software-house/nishiki-archive/pull/288
[pr289]: https://github.com/victor-software-house/nishiki-archive/pull/289
[run-289-ci]: https://github.com/victor-software-house/nishiki-archive/actions/runs/41774739380
[run-289-cov]: https://github.com/victor-software-house/nishiki-archive/actions/runs/41774738906
[pr297]: https://github.com/victor-software-house/nishiki-archive/pull/297
[pr298]: https://github.com/victor-software-house/nishiki-archive/pull/298
[pr299]: https://github.com/victor-software-house/nishiki-archive/pull/299
[pr303]: https://github.com/victor-software-house/nishiki-archive/pull/303
[pr312]: https://github.com/victor-software-house/nishiki-archive/pull/312
[pr313]: https://github.com/victor-software-house/nishiki-archive/pull/313
[run-313-ci]: https://github.com/victor-software-house/nishiki-archive/actions/runs/41795993407
[run-313-cov]: https://github.com/victor-software-house/nishiki-archive/actions/runs/41795992947
[run-299-ci]: https://github.com/victor-software-house/nishiki-archive/actions/runs/41789909151
[run-299-cov]: https://github.com/victor-software-house/nishiki-archive/actions/runs/41789908503
[pr306]: https://github.com/victor-software-house/nishiki-archive/pull/306
[pr307]: https://github.com/victor-software-house/nishiki-archive/pull/307
[run-307-ci]: https://github.com/victor-software-house/nishiki-archive/actions/runs/41790993527
[run-307-cov]: https://github.com/victor-software-house/nishiki-archive/actions/runs/41790992884
[pr309]: https://github.com/victor-software-house/nishiki-archive/pull/309
[run-309-ci]: https://github.com/victor-software-house/nishiki-archive/actions/runs/41793809745
[run-309-cov]: https://github.com/victor-software-house/nishiki-archive/actions/runs/41793809258
[pr315]: https://github.com/victor-software-house/nishiki-archive/pull/315
[pr320]: https://github.com/victor-software-house/nishiki-archive/pull/320
[run-320-ci]: https://github.com/victor-software-house/nishiki-archive/actions/runs/41798651099
[run-320-cov]: https://github.com/victor-software-house/nishiki-archive/actions/runs/41798650762
[pr425]: https://github.com/victor-software-house/nishiki-archive/pull/425
[run-425-ci]: https://github.com/victor-software-house/nishiki-archive/actions/runs/41852238500
[run-425-cov]: https://github.com/victor-software-house/nishiki-archive/actions/runs/41852237629
[pr327]: https://github.com/victor-software-house/nishiki-archive/pull/327
[pr328]: https://github.com/victor-software-house/nishiki-archive/pull/328
[run-328-ci]: https://github.com/victor-software-house/nishiki-archive/actions/runs/41806968953
[run-328-cov]: https://github.com/victor-software-house/nishiki-archive/actions/runs/41806968342
[pr334]: https://github.com/victor-software-house/nishiki-archive/pull/334
[run-334-ci]: https://github.com/victor-software-house/nishiki-archive/actions/runs/41802551825
[run-334-cov]: https://github.com/victor-software-house/nishiki-archive/actions/runs/41802551709
[pr436]: https://github.com/victor-software-house/nishiki-archive/pull/436
[run-436-ci]: https://github.com/victor-software-house/nishiki-archive/actions/runs/41852933916
[run-436-cov]: https://github.com/victor-software-house/nishiki-archive/actions/runs/41852932938
[pr437]: https://github.com/victor-software-house/nishiki-archive/pull/437
[run-437-ci]: https://github.com/victor-software-house/nishiki-archive/actions/runs/41854493283
[run-437-cov]: https://github.com/victor-software-house/nishiki-archive/actions/runs/41854492140
[pr439]: https://github.com/victor-software-house/nishiki-archive/pull/439
[pr440]: https://github.com/victor-software-house/nishiki-archive/pull/440
[run-440-ci]: https://github.com/victor-software-house/nishiki-archive/actions/runs/41856223286
[run-440-cov]: https://github.com/victor-software-house/nishiki-archive/actions/runs/41856222637
[pr444]: https://github.com/victor-software-house/nishiki-archive/pull/444
[run-444-ci]: https://github.com/victor-software-house/nishiki-archive/actions/runs/41860162804
[run-444-cov]: https://github.com/victor-software-house/nishiki-archive/actions/runs/41860161973
[pr447]: https://github.com/victor-software-house/nishiki-archive/pull/447
[run-447-ci]: https://github.com/victor-software-house/nishiki-archive/actions/runs/41862419805
[run-447-cov]: https://github.com/victor-software-house/nishiki-archive/actions/runs/41862419076
[pr450]: https://github.com/victor-software-house/nishiki-archive/pull/450
[run-450-ci]: https://github.com/victor-software-house/nishiki-archive/actions/runs/41867949614
[run-450-cov]: https://github.com/victor-software-house/nishiki-archive/actions/runs/41867948643
[pr333]: https://github.com/victor-software-house/nishiki-archive/pull/333
[run-333-ci]: https://github.com/victor-software-house/nishiki-archive/actions/runs/41802546339
[run-333-cov]: https://github.com/victor-software-house/nishiki-archive/actions/runs/41802545894
[pr339]: https://github.com/victor-software-house/nishiki-archive/pull/339
[pr340]: https://github.com/victor-software-house/nishiki-archive/pull/340
[run-340-ci]: https://github.com/victor-software-house/nishiki-archive/actions/runs/41805706593
[run-340-cov]: https://github.com/victor-software-house/nishiki-archive/actions/runs/41805706070
