# Rule grammar patterns

Each file holds one rule-engine grammar's patterns, as its `src/<language>/grammar.rs` lists them.
A file is named for the grammar's directory under `src/`, such as `common_lisp.json`.

- Each key is a `Pattern` variant.
- Each value holds the pattern's `role`, one of `match`, `begin`, `end` and `while`, and its `regex`.

## Writer

Each `oracle/<language>-grammar.ts` writes its grammar's file beside its `grammar.rs`.
The files hold data only: the generators decide nothing from them.
`mise run check:rule-grammars` fails when a file differs from what its generator writes.

## Reader

`codegen/src/blank_runs.rs` reads every file and writes `src/rules/blank_runs.rs`.
`mise run codegen -- --check` fails when a file is missing or extra, or lists other ids than its grammar's `Pattern` enum.
