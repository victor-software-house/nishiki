# Design

## Context

This change styles the tokens of the [`streaming` change][streaming]'s `Document`.
Its decisions were decisions 12 and 9 of that change's design until the change was split.

## Decisions

### 1. Styled tokens through the per-stack style cache

Agreed with perf-34, 2026-11-05.
The operator asked that `Document` return styled tokens through the throughput change's per-stack cache.

- The throughput change owns the public method, `Highlighter::style(&mut self, theme: &Theme, stack: Stack) -> Style`.
  - It keeps a `Vec` of styles indexed by stack id, filled on first use.
  - A stack's style is its parent's style with one `apply` for its top scope.
  - Each `Theme` gets a process-unique id when it is built, and a clone keeps it.
  - The cache drops its entries when it is called with another theme id.
  - It allocates only when a new stack first appears.
- `Document` owns one theme, given to `Document::new` and switched with `Document::set_theme`.
- `Document::lex` calls `Highlighter::style` for every token of each line it lexes, the provisional line included.
  A known stack is one lookup; a new one costs one `apply`.
- Reads take `&self`.
  `DocumentLine::styled` looks each stack up through `pub(crate) fn cached_style(&self, theme: &Theme, stack: Stack) -> Option<Style>`.
  It returns `None` for a stack not yet seen, and for a cache filled under another theme id.
  So no caller reads styles left from an earlier theme.
  Inside `Document` it never returns `None` after `lex`, and the code handles `None` without a panic.
- `set_theme` drops the cache through the theme-id rule, then walks every token once to fill it again.
  That costs one `apply` per distinct stack and one lookup per token.
- Stack ids are dense arena indices, bounded by `Limits::nodes`, so the style table is a flat array.
- The signature is fixed now, so either change can land first.
  If `Document` lands first, `styled` follows once the cache merges.
  The public method waits for the operator's approval, together with `Document`.

### 2. What a later binding needs

npm stays closed (invariant 8).
A WebAssembly or N-API binding would need:

- UTF-16 offsets.
  JavaScript strings index by UTF-16 unit.
  The binding converts per line, with an ASCII fast path.
- Packed token records, as Pierre's `packed24`: one `u32` per token for its end and its stack id.
  No JavaScript object per token.
- A style table indexed by stack id (decision 1).
  New stacks appear as the document grows, so each batch carries the table entries it uses first.
  A theme change sends the whole table again.
- Views borrowed from the document's memory, valid until the next mutation.
- Budgeted lexing for an idle callback.
  `lex(Until::Line(n))` already gives it in lines.

None of it changes the Rust API above.

[streaming]: ../streaming/design.md
