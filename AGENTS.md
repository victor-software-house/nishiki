# nishiki

A syntax highlighting engine in Rust, written from scratch.
Read [README.md](README.md) for what it is.
The decisions that created it live in a private planning repository: `docs/decisions/2026-10-21-highlighting-*.md`, `docs/decisions/2026-10-21-engine-name-nishiki.md`, and the research note `docs/research/2026-10-21-syntax-engine-direction.md`.

## Invariants

1. Written from scratch.
   No code is copied, translated or seeded from Pierre's `@pierre/highlights` (or its wasm2rs translation), Shiki, syntect, tree-sitter grammars or any other highlighter.
   They are references for behaviour and output only.
2. Permissively licensed: MIT OR Apache-2.0.
   No GPL or other copyleft code or grammar enters the crate; a language whose only grammar is copyleft gets a hand-written lexer.
   A third-party grammar is bundled or followed only after its licence is found at its source and permits it.
   Its licence text goes in `THIRD-PARTY-NOTICES.md`, which the crate ships, and nowhere else.
   The README gets a section for open-source sources only once there are several of them.
3. Shiki is the fidelity target: scopes and colours for every language, measured against Shiki's samples.
   Themes are TextMate themes in Shiki's format.
4. Nothing is downloaded at run time; every supported language is built in.
5. The repository is private; the crate's published source is public on crates.io, so nothing private goes into a packaged file.
6. Conventional Commits; first release `0.0.1`, patch bumps only after that.
   No AI attribution in commits, pull requests or comments.
7. `AGENTS.md` is the only guide file; add no `CLAUDE.md`.
8. Nothing is published to npm or PyPI until nishiki ships something real for them.

## Working here

1. The pull request's CI is the verifier: `mise run verify` runs format, lint, tests, doctests, docs as docs.rs builds them, `cargo deny`, and the repository checks.
   Format with `cargo +nightly-2026-09-29 fmt --all`; the pre-commit hook checks it.
2. The README's Rust examples run as doctests (`ReadmeDoctests` in `src/lib.rs`).
   Its links to files use docs.rs source URLs, because the repository is private.
3. `check:local` (`mise-tasks/check/local.sh`) fails when the packaged crate lacks the README, a licence text or the notices file.
4. A change users see adds a `.changeset/*.md` fragment (`nishiki: patch`).
   verctl turns fragments into a Version PR that bumps `Cargo.toml` and writes `CHANGELOG.md`; never edit either by hand.
   Publishing to crates.io waits for the operator's approval.
5. A language lives in `src/<language>/`, with tests that compare its tokens with Shiki's.
   Rust is a hand lexer; how later languages are built is an open decision in the planning repository.
