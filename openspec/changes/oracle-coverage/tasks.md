# Tasks

## 1. Measure

- [x] 1.1 Write `oracle/coverage.ts`: units and reachability per hand-lexed grammar, the comment tags, the wrapped engine, and the run over every fixture's text (design, sections 2 and 3) → verify: on `swift` it finds the 264 matched expressions of the 2026-11-04 measurement, and the instrumented scope stacks equal every fixture's
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

- [ ] 3.1 C#: `csharp`
- [ ] 3.2 Swift: `swift`
- [ ] 3.3 Python: `python`
- [ ] 3.4 Go: `go`
- [ ] 3.5 Java: `java`
- [ ] 3.6 TypeScript: `typescript`, `javascript`, `jsx`, `tsx`
- [ ] 3.7 C and C++: `c`, `cpp`, `cpp-macro`
- [x] 3.8 Rust: `rust`
  - 2026-11-04 UTC, [PR 259][pr259]: `rust/core-macros` and `rust/coverage` added; Rust complete with one waiver. On `b672f999` the [oracle test][run-259-ci] and the [coverage check][run-259-cov] passed.
- [ ] 3.9 Ruby: `ruby`
- [ ] 3.10 Kotlin: `kotlin`
- [ ] 3.11 Lua: `lua`
- [ ] 3.12 shell: `shellscript`
- [ ] 3.13 Markdown: `markdown`
- [ ] 3.14 CSS, Less and SCSS: `css`, `less`, `scss`
- [ ] 3.15 HTML, html-derivative and XML: `html`, `html-derivative`, `xml`
- [ ] 3.16 ERB: `erb`
- [ ] 3.17 JSON: `json`, `jsonc`, `jsonl`
- [ ] 3.18 YAML and TOML: `yaml`, `toml`
- [ ] 3.19 SQL: `sql`
- [ ] 3.20 GraphQL: `graphql`
- [ ] 3.21 diff, Dockerfile, INI and Make: `diff`, `docker`, `ini`, `make`

[pr238]: https://github.com/victor-software-house/nishiki-archive/pull/238
[pr252]: https://github.com/victor-software-house/nishiki-archive/pull/252
[run-252]: https://github.com/victor-software-house/nishiki-archive/actions/runs/41756482197
[pr259]: https://github.com/victor-software-house/nishiki-archive/pull/259
[run-259-ci]: https://github.com/victor-software-house/nishiki-archive/actions/runs/41757301609
[run-259-cov]: https://github.com/victor-software-house/nishiki-archive/actions/runs/41757301124
