# nishiki

A syntax highlighting engine in Rust, written from scratch.
Read [README.md](README.md) for what it is.

## Guides

Each directory below has a README that says what it holds, and an `AGENTS.md` with the rules for working there.

- [CODING_STANDARDS.md](CODING_STANDARDS.md): the numbered coding rules.
- [src/](src/README.md): the crate, by module.
- [openspec/](openspec/README.md): specs, changes in flight and research notes, with [their rules](openspec/AGENTS.md).
- [oracle/](oracle/README.md): the Bun scripts that run Shiki, with [their rules](oracle/AGENTS.md).
- [tests/](tests/README.md): the integration tests and fixtures, with [their rules](tests/AGENTS.md).
- [bench/AGENTS.md](bench/AGENTS.md): the rules for measuring.
- [mise-tasks/](mise-tasks/README.md): the repository's tasks.

The decisions that created nishiki live in a private planning repository:

- `openspec/decisions/2026-10-21-highlighting-*.md`
- `openspec/decisions/2026-10-21-engine-name-nishiki.md`
- `openspec/decisions/2026-10-24-highlighting-engine.md`
- the research notes `docs/research/2026-10-21-syntax-engine-direction.md`
  and `docs/research/2026-10-22-highlighting-engine-comparison.md`

## Invariants

1. **Written from scratch.**
   - No code is copied, translated or seeded from another highlighter:
     Pierre's `@pierre/highlights` (or its wasm2rs translation), Shiki, syntect, tree-sitter grammars or any other.
   - They are references for behaviour and output only.
2. **Permissively licensed: MIT OR Apache-2.0.**
   - No GPL or other copyleft code or grammar enters the crate.
     A language whose only grammar is copyleft gets a hand-written lexer.
   - A third-party grammar is bundled or followed only after its licence is found at its source and permits it.
   - Every grammar is read as the reference for its language, whatever its licence (operator, 2026-11-03).
     A language whose grammar has no licence found at its source, such as GLSL's, gets a hand-written lexer, and that grammar is never bundled.
   - A licence can sit outside a `LICENSE` file: TOML's bundle carries the TextMate bundle licence in its README, and YAML's grammar carries its own MIT licence in a file beside it.
     Read the README and the files beside the grammar before calling one unlicensed.
   - A grammar's licence text goes in `THIRD-PARTY-NOTICES.md`, which the crate ships, and nowhere else.
   - The README gets a section for open-source sources only once there are several of them.
3. **Shiki is the fidelity target.**
   - Scopes and colours for every language are measured against Shiki's samples.
   - Themes are TextMate themes in Shiki's format.
4. **Nothing is downloaded at run time.**
   Every supported language is built in when its feature is on.
   Languages on nishiki's TextMate engine sit in the opt-in `textmate` tier, off by default and outside `all`, because the engine adds about 1.7 MB once (operator, 2026-11-03).
5. **Nothing private ships.**
   The repository is private, but the crate's published source is public on crates.io,
   so nothing private goes into a packaged file.
6. **Releases.**
   - Conventional Commits; the first release was `0.0.1`.
   - Versions follow semver but never take a major.
     A release is a patch by default (operator, 2026-11-02).
   - A change that breaks the public API is a minor,
     unless the previous release was itself a minor shortly before.
     Two minors within hours or days make no sense,
     so later breaks ride on the pending or recent minor as patches.
   - `0.1.0` (2026-11-02) was the one exception,
     published by an automation slip after an almost empty `0.0.1`.
   - No AI attribution in commits, pull requests or comments.
7. **One guide file.**
   `AGENTS.md` is the only guide file; add no `CLAUDE.md`.
8. **No npm or PyPI yet.**
   Nothing is published there until nishiki ships something real for them.

## Working here

### Verify

1. The pull request's CI is the verifier.
   `mise run verify` runs:
   - format and lint;
   - tests and doctests;
   - docs, as docs.rs builds them;
   - `cargo deny`;
   - the repository checks.
2. Three more workflows check a pull request against its base:

   | Workflow | Runs | Fails when |
   |:--|:--|:--|
   | Identity, `identity.yml` | `mise run identity` | a fixture, or an edited copy of one, lexes differently |
   | Callgrind, `callgrind.yml` | `mise run callgrind`, on a pull request labelled `bench` | a language's instruction count rises more than 0.5% |
   | Oracle coverage, `coverage.yml` | `mise run check:oracle-coverage` | the report is stale, or a complete grammar has a gap ([oracle/AGENTS.md](oracle/AGENTS.md) rule 6) |

3. A pull request that means to change output says so in a body line, `identity: changes <languages>`, such as `identity: changes python java`.
4. Callgrind counts the lexing of every language's oracle fixtures, as a table in the job summary.
   - Label a pull request that touches a hot path.
   - The gate builds both sides at one codegen unit, so the counts follow the code and not its layout.
   - Columns at the default sixteen units are for information only (`bench/AGENTS.md` rule 12).
   - Below the table, what one pass over the fixtures allocates on each side, for information only (`bench/AGENTS.md` rule 17).
   - valgrind comes from the bench mise profile, `mise.bench.toml` with `mise.bench.lock`, which only this workflow loads (`MISE_ENV=ci,bench`).
   - A local run on Linux uses `MISE_ENV=dev,bench mise run callgrind`.
5. Nobody runs the full suite before a push; CI does.
   - Work CI cannot do runs locally: a lexer's build-and-probe loop, its oracle fixtures, the Shiki differential and the bench.
   - The pre-commit hook runs the cheapest checks: `mise run format` (rustfmt) and `mise run check:literals` when a commit touches a Rust file, and `mise run lint:md` when it touches Markdown.
   - Before a push, run the other cheap checks: `typos` and `mise run format:toml` (taplo).
   - When CI fails, fix forward.
6. Formatting uses rustfmt on the nightly pinned in `mise.toml`.
   - `mise run format` checks it, as the pre-commit hook does.
   - `mise x -- cargo $RUSTFMT_TOOLCHAIN fmt --all` applies it.
7. `check:local` (`mise-tasks/check/local.sh`) fails when the packaged crate lacks the README, a licence text or the notices file.
   It also runs `check:layout` (every browsable directory has its README and guide) and `check:literals`.

### Documentation

1. The crate-root docs in `src/lib.rs` are the docs.rs landing page.
   - They hold the quick start, the themes, the limits, the cargo features and Markdown fences.
   - They link items with intra-doc links.
   - Their Rust examples run as doctests, and so does the README's one example (`ReadmeDoctests` in `src/lib.rs`).
   - rustfmt's `wrap_comments` reflows a list item in a doc comment, so its lines break where rustfmt puts them.
2. The README is the GitHub page: what nishiki is, `cargo add`, one example, a docs.rs link, the language table, the status and the licence.
   - It holds no API reference; the crate docs do.
   - It links to files through docs.rs source URLs, because the repository is private.
3. Every document, from guides to research notes, is written to be scanned (operator, 2026-11-02 and 2026-11-03):
   - short sentences, one idea each, with Semantic Line Breaks;
   - a list for a set of items, a table for a comparison, a heading per topic;
   - a blank line between blocks;
   - plain words: no filler, no flourish, no mannered phrasing;
   - no walls of prose: a paragraph past four lines becomes a list or splits;
   - headers that name their content, never vague ones like "Piece" or "What it holds".
4. A directory a reader browses on GitHub has a `README.md` that says what it holds.
   Where it has rules for working there, an `AGENTS.md` sits beside it.
5. Each language's quirks sit in the docs of its `Language` variant, in the `languages!` rows of `src/language.rs`.
   They go under a "Behaviour kept from Shiki" heading, as a bullet list, one quirk per bullet.
   A variant that shares another's lexer links to it, as `Language::Jsx` links to `Language::TypeScript`.

### Release

1. A change users see adds a `.changeset/*.md` fragment (`nishiki: patch`).
2. verctl turns fragments into a Version PR that bumps `Cargo.toml` and writes `CHANGELOG.md`.
   Never edit either by hand.
3. Publishing to crates.io waits for the operator's approval.
   The Version PR has no auto-merge, and only the operator merges it, because merging it publishes.
4. Work merges in batches through `gh stack`, one idea per pull request (operator, 2026-11-02 and 2026-11-03):
   - a workstream's slices go up as one stack, each pull request based on the branch below, opened without waiting for its parent to land;
   - a stacked pull request takes no auto-merge;
   - once every head is green and reviewed, `gh stack merge <stack> --yes` puts the whole stack in the merge queue;
   - a pull request already queued is never linked into a stack, since linking drops it from the queue.

### Languages

1. A language lives in `src/<language>/`.
2. Every language gets a hand-written lexer, in order of use, several in parallel (operator, 2026-11-02, revising the 2026-10-24 decision).
   With the oracle and a Shiki differential, Python took under an hour, not the days its line count suggested.
3. nishiki's own TextMate engine is built in parallel, as fast as it can be made.
   - Its one approved dependency beyond `memchr` is `regex-automata`.
   - It covers every bundled language not yet hand-written.
   - It checks the hand lexers as a second oracle.
   - Every grammar it compiles has its licence, read at its source, in `oracle/licences.ts`, and a table the crate ships holds only grammars allowed there; `oracle/engine-tables.ts` checks both.
4. Several lexers are written in parallel, each by its own agent, and land one at a time.

### The Shiki oracle

The rules are in [oracle/AGENTS.md](oracle/AGENTS.md) and [tests/AGENTS.md](tests/AGENTS.md).

### Benchmarks

1. `mise run bench` measures, in release mode on the oracle fixtures:
   - throughput;
   - streaming latency per line;
   - cold start;
   - theme styling.

   `mise run bench python java` measures only the languages it names.
2. It is not part of `verify`, because timings depend on the machine.
   `bench/AGENTS.md` says where a run counts and how it is recorded.

### Limits

1. Every per-line limit is configurable and overridable by the caller (operator, 2026-11-02).
2. Defaults may follow the resources the caller has, such as memory or a time budget.
   A caller that pins the limits gets the same output on every machine.
3. Until the next minor release, a language's own bounds, such as C++'s `MAX_LOOPS`, are named constants listed in the docs of its `Language` variant, under "Behaviour kept from Shiki" (operator, 2026-11-03).
   Adding them to `Limits` and `Limit` breaks the public API, so the next minor does it once, with the `feat/non-exhaustive-next-minor` branch:
   - it marks both `#[non_exhaustive]`;
   - it adds a builder;
   - it moves every language bound into `Limits`.
