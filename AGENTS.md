# nishiki

A syntax highlighting engine in Rust, written from scratch.
Read [README.md](README.md) for what it is.

## Guides

| File | Holds |
|:--|:--|
| [CODING_STANDARDS.md](CODING_STANDARDS.md) | The numbered coding rules. |
| [docs/AGENTS.md](docs/AGENTS.md) | The rules for research notes. |
| [bench/AGENTS.md](bench/AGENTS.md) | The rules for measuring. |

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
   - A language whose grammar has no licence, such as YAML's, gets a hand-written lexer.
     It is built from the language's specification and Shiki's output alone, and nobody reads the grammar (operator, 2026-11-02).
   - A grammar's licence text goes in `THIRD-PARTY-NOTICES.md`, which the crate ships, and nowhere else.
   - The README gets a section for open-source sources only once there are several of them.
3. **Shiki is the fidelity target.**
   - Scopes and colours for every language are measured against Shiki's samples.
   - Themes are TextMate themes in Shiki's format.
4. **Nothing is downloaded at run time.**
   Every supported language is built in.
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
2. Formatting uses rustfmt on the nightly pinned in `mise.toml`.

   | Command | Does |
   |:--|:--|
   | `mise run format` | Checks formatting, as the pre-commit hook does. |
   | `mise x -- cargo $RUSTFMT_TOOLCHAIN fmt --all` | Applies it. |

3. `check:local` (`mise-tasks/check/local.sh`) fails when the packaged crate lacks the README, a licence text or the notices file.

### Documentation

1. The README's Rust examples run as doctests (`ReadmeDoctests` in `src/lib.rs`).
2. The README links to files through docs.rs source URLs, because the repository is private.
3. Guides and the README are written to be scanned (operator, 2026-11-02):
   - short sentences, one idea each, with Semantic Line Breaks;
   - a list for a set of items, a table for a comparison, a heading per topic;
   - a blank line between blocks.
4. A language's section under "Behaviour kept from Shiki" in the README is a bullet list, one quirk per bullet.

### Release

1. A change users see adds a `.changeset/*.md` fragment (`nishiki: patch`).
2. verctl turns fragments into a Version PR that bumps `Cargo.toml` and writes `CHANGELOG.md`.
   Never edit either by hand.
3. Publishing to crates.io waits for the operator's approval.
   The Version PR has no auto-merge, and only the operator merges it, because merging it publishes.
4. A change that builds on an unmerged one goes up as a stacked pull request with `gh stack`,
   one idea per pull request (operator, 2026-11-02).

### Languages

1. A language lives in `src/<language>/`.
2. Every language gets a hand-written lexer, in order of use, several in parallel (operator, 2026-11-02, revising the 2026-10-24 decision).
   With the oracle and a Shiki differential, Python took under an hour, not the days its line count suggested.
3. nishiki's own TextMate engine is built in parallel, as fast as it can be made.
   - Its one approved dependency beyond `memchr` is `regex-automata`.
   - It covers every bundled language not yet hand-written.
   - It checks the hand lexers as a second oracle.

### The Shiki oracle

1. `mise run oracle` runs Shiki on every input in `oracle/inputs.ts`
   and writes its scopes to `tests/oracle/<language>/`.
2. `tests/it/oracle.rs` checks every language in `Language::ALL`, in parallel.
   It fails when a non-whitespace byte gets a different scope stack or colour,
   so a new language needs only its fixtures there.
3. A new language adds its inputs and fixtures.
   `tests/oracle/SOURCES.md` records each input's source and licence.
4. `tests/oracle/.ignore` keeps typos, `rg` and `fd` out of the generated fixtures,
   which keep their sources' spelling.
   Search them with `rg -u`.

### Benchmarks

1. `mise run bench` measures, in release mode on the oracle fixtures:
   - throughput;
   - streaming latency per line;
   - cold start;
   - theme styling.
2. It is not part of `verify`, because timings depend on the machine.
   `bench/AGENTS.md` says where a run counts and how it is recorded.

### Limits

1. Every per-line limit is configurable and overridable by the caller (operator, 2026-11-02).
2. Defaults may follow the resources the caller has, such as memory or a time budget.
   A caller that pins the limits gets the same output on every machine.
