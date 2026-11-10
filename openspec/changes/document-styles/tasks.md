# Tasks

Each slice is one pull request in one `gh stack`.
Crate slices run the oracle, Identity and Callgrind workflows; none may change output.
These tasks were task 3.4 of the [`streaming` change][streaming] until that change was split.

## 1. Styled tokens

- [x] 1.1 `Document` owns its theme, `lex` fills the per-stack style cache, and `DocumentLine::styled` reads it (design decision 1, after the cache lands).
  → verify: a test streams every oracle fixture with every oracle theme and switches the theme mid-stream.
  It finds every style equal to `Theme::style`.

  2026-11-05 UTC: the operator asked that `Document` return styled tokens through the throughput change's per-stack style cache.
  The surface was to be agreed with perf-34.
  perf-34 agreed the surface on 2026-11-05, with two amendments.
  `cached_style` takes the theme, and a binding receives style-table entries as batches first use them.
  It is design decision 1.

  2026-11-05 UTC: in the pull request of `feat/document-styled`.
  `every_language_streams_styled_as_the_theme_styles` streams every fixture under the three oracle themes.
  It switches the theme at the middle delta and again after the end, and finds every style equal to `Theme::style`.

## 2. Archive

- [ ] 2.1 `openspec archive document-styles -y`, after the `streaming` change is archived → verify: `mise run specs` passes

[streaming]: ../streaming/tasks.md
