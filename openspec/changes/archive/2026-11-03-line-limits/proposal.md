# Proposal

## Why

A line-state lexer can be driven into unbounded work or memory by one hostile line: the engine comparison (section 7) measured an 18 s Markdown line, out-of-memory runs in a compiled TextMate engine, and Shiki relying on a 500 ms time limit.
nishiki highlights while a reply streams, so one line must never stall a frame.
The operator's rule of 2026-11-02 sets the shape: every limit is configurable and overridable, its defaults may follow the resources the caller has, and pinned limits give the same output everywhere.

## What Changes

- A `Limits` value, given to every highlighter, with one field per limit: line length, work per line, nesting depth, and interned scope stacks.
- Each field can be set, raised, lowered or turned off by the caller; `Limits::default()` is fixed and set high enough that no oracle line reaches it.
- `Limits::for_budget` derives the limits from resources the caller states: a memory budget and a per-line time budget, converted to work units by a calibration the caller can also pin.
- A line that reaches a limit is reported, so a caller can tell a truncated line from a correct one.
- Every lexer, Rust first and then each new one, counts its work and honours the same limits.

## Capabilities

### New Capabilities

- `line-limits`: bounds on the work and memory one line can take, configurable by the caller, with truncated lines reported.

### Modified Capabilities

None.

## Impact

- Public API: `Limits`, `Highlighter::new_with_limits`, and a per-line report; `Highlighter::new` keeps the default limits.
- Every lexer's main loop and every lookahead scan count work.
- `State` becomes a `Copy` handle into a hash-consed persistent stack owned by the highlighter, and `Highlighter::scopes` returns an iterator (design, section 4); both break the 0.0.x API.
