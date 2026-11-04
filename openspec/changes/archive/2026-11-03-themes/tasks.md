# Tasks

## 1. Themes

- [x] 1.1 Resolve theme rules into a trie with VS Code's specificity, parent selectors and inheritance → verify: unit tests for depth, parents, `>`, inheritance, overrides and defaults pass
  - 2026-11-02 UTC: seven tests in `src/theme.rs` pass on the operator's Mac.
- [x] 1.2 Write github-dark, one-light and catppuccin-mocha and Shiki's colours into the oracle → verify: the colour test matches on every Rust fixture and fails when one font style in a fixture is changed
  - 2026-11-02 UTC: all nine fixture and theme pairs match; with one font style changed in `rust/sample.one-light.colors` the test failed on 4 of 751 bytes, naming line 2, column 4.
- [x] 1.3 Add `json` with `serde_json` only → verify: `cargo deny` passes and the tree under `json` is `serde_json`, `itoa`, `serde_core`, `zmij`
  - 2026-11-02 UTC: `mise run deny` passed; `cargo tree -e normal --features json` lists those four.
- [x] 1.4 CI → verify: `mise run verify` passes on the pull request
  - 2026-11-03 UTC: `mise run verify` runs on every pull request and on main; [CI run 41674564525](https://github.com/victor-software-house/nishiki-archive/actions/runs/41674564525) passed at main `c50342f8`.
