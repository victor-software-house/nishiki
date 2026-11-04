# Tasks

## 1. Oracle

- [x] 1.1 Write `oracle/generate.ts` and `oracle/inputs.ts`, and the `oracle` task → verify: `mise run oracle` writes the fixtures, and a second run leaves them byte for byte the same
  - 2026-11-02 UTC: two runs on the operator's Mac gave the same SHA-1 for all three Rust fixtures.
- [x] 1.2 Write the oracle test as `tests/it/oracle.rs` in one `it` binary → verify: it passes on the Rust fixtures and fails when a fixture's scope is changed
  - 2026-11-02 UTC: on the operator's Mac it passed on all three fixtures (751 non-whitespace bytes in the sample alone); with one scope in `rust/sample.txt` renamed it failed with 4 of 751 bytes, naming line 2, column 4 and both stacks.
- [x] 1.3 List each input's source and licence in `tests/oracle/SOURCES.md` → verify: every licence link resolves at its pinned commit
  - 2026-11-02 UTC: `gh api` read the three licence files at their pinned commits.
- [x] 1.4 CI runs the oracle test → verify: `mise run verify` passes on the pull request
  - 2026-11-03 UTC: `mise run verify` runs the oracle test on every pull request and on main; [CI run 41674564525](https://github.com/victor-software-house/nishiki-archive/actions/runs/41674564525) passed at main `c50342f8`.
