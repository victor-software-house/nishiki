# nishiki

A syntax highlighting engine in Rust, written from scratch.
Read [README.md](README.md) for what it is.
[CODING_STANDARDS.md](CODING_STANDARDS.md) holds the numbered coding rules, [docs/AGENTS.md](docs/AGENTS.md) the rules for research notes, and [bench/AGENTS.md](bench/AGENTS.md) the rules for measuring.
The decisions that created it live in a private planning repository: `openspec/decisions/2026-10-21-highlighting-*.md`, `openspec/decisions/2026-10-21-engine-name-nishiki.md` and `openspec/decisions/2026-10-24-highlighting-engine.md`, with the research notes `docs/research/2026-10-21-syntax-engine-direction.md` and `docs/research/2026-10-22-highlighting-engine-comparison.md`.

## Invariants

1. Written from scratch.
   No code is copied, translated or seeded from Pierre's `@pierre/highlights` (or its wasm2rs translation), Shiki, syntect, tree-sitter grammars or any other highlighter.
   They are references for behaviour and output only.
2. Permissively licensed: MIT OR Apache-2.0.
   No GPL or other copyleft code or grammar enters the crate; a language whose only grammar is copyleft gets a hand-written lexer.
   A third-party grammar is bundled or followed only after its licence is found at its source and permits it.
   A language whose grammar has no licence, such as YAML's, gets a hand-written lexer built from the language's specification and Shiki's output alone; nobody reads the grammar (operator, 2026-11-02).
   Its licence text goes in `THIRD-PARTY-NOTICES.md`, which the crate ships, and nowhere else.
   The README gets a section for open-source sources only once there are several of them.
3. Shiki is the fidelity target: scopes and colours for every language, measured against Shiki's samples.
   Themes are TextMate themes in Shiki's format.
4. Nothing is downloaded at run time; every supported language is built in.
5. The repository is private; the crate's published source is public on crates.io, so nothing private goes into a packaged file.
6. Conventional Commits; first release `0.0.1`.
   Versions follow semver but never take a major, and a release is a patch by default (operator, 2026-11-02).
   A change that breaks the public API is a minor only when the previous release was not itself a minor shortly before: two minors within hours or days make no sense, so later breaks ride on the pending or recent minor as patches.
   `0.1.0` (2026-11-02) was the one exception, published by an automation slip after an almost empty `0.0.1`.
   No AI attribution in commits, pull requests or comments.
7. `AGENTS.md` is the only guide file; add no `CLAUDE.md`.
8. Nothing is published to npm or PyPI until nishiki ships something real for them.

## Working here

1. The pull request's CI is the verifier: `mise run verify` runs format, lint, tests, doctests, docs as docs.rs builds them, `cargo deny`, and the repository checks.
   Format with `mise run format`, which runs rustfmt on the pinned nightly; the pre-commit hook checks it.
2. The README's Rust examples run as doctests (`ReadmeDoctests` in `src/lib.rs`).
   Its links to files use docs.rs source URLs, because the repository is private.
3. `check:local` (`mise-tasks/check/local.sh`) fails when the packaged crate lacks the README, a licence text or the notices file.
4. A change users see adds a `.changeset/*.md` fragment (`nishiki: patch`).
   verctl turns fragments into a Version PR that bumps `Cargo.toml` and writes `CHANGELOG.md`; never edit either by hand.
   Publishing to crates.io waits for the operator's approval: the Version PR has no auto-merge, and only the operator merges it, because merging it publishes.
   A change that builds on an unmerged one goes up as a stacked pull request with `gh stack`, one idea per pull request (operator, 2026-11-02).
5. A language lives in `src/<language>/`.
   Every language gets a hand-written lexer, in order of use, several in parallel: with the oracle and a Shiki differential, Python took under an hour, not the days its line count suggested (operator, 2026-11-02, revising the 2026-10-24 decision).
   nishiki's own TextMate engine is built in parallel, as fast as it can be made, with `regex-automata` as its one approved dependency beyond `memchr`; it covers every bundled language not yet hand-written and checks the hand lexers as a second oracle.
6. Shiki is the oracle.
   `mise run oracle` runs Shiki on every input in `oracle/inputs.ts` and writes its scopes to `tests/oracle/<language>/`; `tests/it/oracle.rs` fails when a non-whitespace byte gets a different scope stack.
   A new language adds its inputs and fixtures, and `tests/oracle/SOURCES.md` records each input's source and licence.
   `tests/oracle/.ignore` keeps typos, `rg` and `fd` out of the generated fixtures, which keep their sources' spelling; search them with `rg -u`.
7. `mise run bench` measures throughput, streaming latency per line, cold start and theme styling on the oracle fixtures, in release mode.
   It is not part of `verify`, because timings depend on the machine; `bench/AGENTS.md` says where a run counts and how it is recorded.
8. Every per-line limit is configurable and overridable by the caller (operator, 2026-11-02).
   Defaults may follow the resources the caller has, such as memory or a time budget, but a caller that pins the limits gets the same output on every machine.
