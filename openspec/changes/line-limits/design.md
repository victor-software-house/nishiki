# Design

## 1. The limits

| Limit | Unit | When reached |
|:--|:--|:--|
| Line length | bytes | the line is one token under the current stack and the state passes over it unchanged, as Shiki's `tokenizeMaxLineLength` does |
| Work per line | steps: `per_byte × length + base` | the rest of the line is one token under the current stack, and the next line starts from the state at that point, as Shiki does at its time limit |
| Nesting depth | open contexts | further opens are ignored and counted; deeper constructs add no scope |
| Interned stacks | stacks per highlighter | a token whose stack is new gets its nearest interned ancestor's stack |

A step is one iteration of a lexer's main loop or one byte read by a lookahead scan, so a linear lexer spends a few steps per byte and a superlinear path runs into the limit.
Work is counted, not timed, so the same limits give the same output on every machine (comparison, section 7.3).

## 2. Configurable and overridable

`Limits` has one public field per limit, each an `Option`, where `None` turns that limit off.
A caller builds it from `Limits::default()`, `Limits::unlimited()` or `Limits::for_budget(..)` and then overrides any field.
`Highlighter::new_with_limits(limits)` takes it; `Highlighter::new()` uses the default.
An embedding lexer, such as Markdown calling Rust for a fence, passes its own limits and remaining work to the embedded one.

## 3. Defaults that follow resources

`Limits::default()` is a fixed value, high enough that no line of any oracle fixture reaches it, so output never depends on the machine unless the caller asks.

`Limits::for_budget(Budget { memory, line_time, steps_per_second })` derives the limits from resources the caller states:

1. `memory` bounds the interned stacks, at their measured size per stack.
2. `line_time` times `steps_per_second` bounds the work per line.
3. `steps_per_second` defaults to a constant measured on a reference machine; a caller that wants the machine's own speed measures it once with `Budget::calibrate()` and can store the number, so later runs give the same output.

The library does not read the machine's memory or clock on its own: the caller decides which resources count, which keeps the crate free of platform code and its output reproducible.

## 4. State as a handle into a persistent stack

Today each lexer's `State` is a `Copy` value of 24 bytes, 32 frames of 4 bits in a `u128`, so nesting can never pass 32.
A depth limit the caller can raise needs frames that are not fixed in size.

Decision (2026-11-02, the operator left the design to best practice): `State` stays a small `Copy` value, but becomes a handle into a hash-consed persistent stack that the highlighter owns, as vscode-textmate's `StateStack` is a persistent linked list.

1. A frame is a node: its parent's id, its context kind, and a payload id for data such as a raw string's hash count or a heredoc's delimiter, interned in the same arena.
2. Push looks up the (parent, kind, payload) node in a hash map and creates it once; pop follows the parent link. Both are O(1).
3. Equal states have the same id, so the edit loop's test, "is the state after this line the old state again?", is one integer comparison.
4. Lines share their frames: a document of n lines with depth d costs at most one node per distinct frame, not n × d.
5. Depth is limited only by the nesting limit and the arena by the interned-stacks limit, both from `Limits`.

The rejected options were a fixed ceiling of 32 (not overridable), inline frames that spill to the heap (`State` stops being `Copy`, and each clone may allocate), and a const-generic capacity (a type parameter on every public type).

Scope stacks follow the same design: a stack is its parent's id and one scope, interned once, so memory grows linearly with depth instead of with its square (comparison, section 7.1, cause 1).
`Highlighter::scopes` then returns an iterator over a stack's scopes instead of a slice.

One shared module holds both arenas, so every lexer uses the same state and scope machinery instead of each carrying its own copy.
A `State` or `Stack` belongs to the highlighter that made it, as a `Stack` does today.

## 5. Report

`Highlighter::line` returns a `LineReport` naming each limit the line reached, empty for a line lexed in full.
The highlighter also keeps running counts, so a caller can log how often each limit fired.
