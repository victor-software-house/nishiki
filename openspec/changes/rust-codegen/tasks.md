# Tasks

`codegen/` arrives with [rule-table-codegen][rule-table-codegen] task 1.1.
The tasks below follow it.

## 1. The writer

- [ ] 1.1 `AGENTS.md`, `CODING_STANDARDS.md` and `oracle/AGENTS.md` gain the rules of the proposal and how to move one language.
  → verify: review
- [ ] 1.2 Unicode sets, case folds and classes are written from `oracle/facts/`.
  → verify: `codegen --compare` shows equal tokens for the three files
- [ ] 1.3 The word scripts and `fence-stops.ts` move into `codegen/`.
  Each `words.rs` blob is written one word list per line, with no backslash line continuation.
  → verify: each `words.rs` keeps its tokens, `check:literals` passes on it without the writer exemption, and the fence check fails on a wrong `pops_fence` list
  - 2026-11-10: `oracle/rule-generator.ts` writes the Objective-C, TeX and LaTeX blobs as `concat!` of one literal per line, with the same bytes.
    `check:literals` now checks every generated Rust file.
    The move into `codegen/` stays open.

[rule-table-codegen]: ../rule-table-codegen/tasks.md
