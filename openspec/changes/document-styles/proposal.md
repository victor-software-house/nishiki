# Proposal

## Motivation

The [`streaming` change][streaming] adds `Document`, which gives each line's tokens with their scope stacks.
A renderer needs each token's colours and font style.
The operator asked that `Document` return styled tokens through the [throughput change][throughput]'s per-stack style cache (2026-11-05).
Styling each token through `Theme::style` would repeat the same work for every token of a known stack.

This change was part of the `streaming` change until that change was split to fit the OpenSpec word budgets.

## Changes

- `Document` owns one theme, given to `Document::new` and switched with `Document::set_theme`.
- `Document::lex` styles every token of each line it lexes through `Highlighter::style`.
- `DocumentLine::styled` reads each token's style from the cache through `&self`.
- `set_theme` drops the cache and styles every token again.

The design is in [design.md](design.md).
It also notes what a later WebAssembly or N-API binding needs.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `streaming`, added by the `streaming` change: a document's tokens come styled with its theme.

## Impact

- `src/document.rs`: the theme, `set_theme` and `DocumentLine::styled`.
- It reads the throughput change's `Highlighter::style` and its crate-internal `cached_style`.
- The public API gains `Document::set_theme` and `DocumentLine::styled`.
  The change is additive, so the release is a patch (invariant 6).
- npm stays closed (invariant 8).

[streaming]: ../streaming/proposal.md
[throughput]: https://github.com/victor-software-house/nishiki-archive/blob/99416359c42a215e43d27db1e9a950fe2b21fe25/openspec/changes/throughput/proposal.md
