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
   A third-party grammar is bundled only after its licence is found at its source and permits it, and its licence text ships with it.
3. Shiki is the fidelity target: scopes and colours for every language, measured against Shiki's samples.
   Themes are TextMate themes in Shiki's format.
4. Nothing is downloaded at run time; every supported language is built in.
5. The repository is private; the crate's published source is public on crates.io, so nothing private goes into a packaged file.
6. Conventional Commits; first release `0.0.1`, patch bumps only after that.
   No AI attribution in commits, pull requests or comments.
7. `AGENTS.md` is the only guide file; add no `CLAUDE.md`.
8. Nothing is published to npm or PyPI until nishiki ships something real for them.
