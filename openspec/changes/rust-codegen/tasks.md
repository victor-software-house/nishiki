# Tasks

Task 1.3's word tables create `codegen/`, ahead of [rule-table-codegen][rule-table-codegen] task 1.1 (design section 7).

## 1. The writer

- [ ] 1.1 `AGENTS.md`, `CODING_STANDARDS.md` and `oracle/AGENTS.md` gain the rules of the proposal and how to move one language.
  → verify: review
- [ ] 1.2 Unicode sets, case folds and classes are written from `oracle/facts/`.
  → verify: `codegen --compare` shows equal tokens for the three files
- [ ] 1.3 The word scripts and `fence-stops.ts` move into `codegen/`.
  `numbers.rs` fills each word table into lines of up to 100 columns, with no backslash line continuation.
  → verify: each `words.rs` keeps its entries in order, and the size example's `.text` and `.rodata` keep their hashes.
  `check:literals` passes on it without the writer exemption, and the fence check fails on a wrong `pops_host` list.
  - 2026-11-10: `oracle/rule-generator.ts` writes the Objective-C, TeX and LaTeX blobs as `concat!` of one literal per line, with the same bytes.
    `check:literals` now checks every generated Rust file.
    The move into `codegen/` stays open.

[rule-table-codegen]: ../rule-table-codegen/tasks.md
