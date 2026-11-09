# Design

## Context

The engine's tables are static data today.
[`Grammar`][grammar] holds `&'static` slices, and the tokenizer's scratch keys its caches on a `&'static Grammar`.
A test survey already builds a grammar from JSON tables at run time by leaking them ([`survey::load`][survey]).

Operator decisions, 2026-11-09:

1. A caller adds a language through both paths: a grammar compiled at run time, and one compiled in a build script.
2. The crate bundles no grammar.
3. The release waits until CI matches Shiki on every non-whitespace byte of every Shiki grammar's sample, through the public API.

## Goals

- Any TextMate grammar in Shiki's JSON form highlights through a public API.
- Output equals Shiki's for every grammar Shiki 4.6.0 bundles, on its sample.
- A grammar compiled in a build script loads with no JSON parsing.
- The hand lexers' speed and output do not change.

## Non-Goals

- Loading Oniguruma itself, or any regex engine besides the engine's own.
- Bundling grammars, or downloading them.
- Unloading a grammar before the process ends, as the decision "Grammars live for the process" explains.

## Decisions

### 1. One compiler, in Rust

A Rust compiler turns a grammar's JSON into the engine's tables.
It follows includes inside the grammar and interns scope names.
It is written from scratch, with vscode-textmate's behaviour as the reference (invariant 1).

An include of another grammar stays a scope name in the compiled tables.
Loading links those names against a `Grammars` set, the same step for JSON and for bytes.
Linking drops a rule whose included grammar the set lacks, as `oracle/engine-tables.ts` does.

The run-time path and the build-script path both call it.
The TypeScript generator stays as its oracle.
A test compares the two compilers' tables for every Shiki grammar, and the TypeScript generator retires once they are equal.

### 2. Grammars live for the process

A compiled grammar is moved to the heap and leaked, so the engine keeps its `&'static Grammar`.

A process-wide set interns linked grammars.
Its key has two parts:

- the grammar's source, compared in full after a hash match;
- the handle each include linked to.

Loading the same grammar twice with the same includes returns the first one.
The same source loaded with other includes is a different grammar.
So a load with fewer grammars never shadows a later, fuller one.

Memory grows with each distinct grammar and stays until the process ends.
The crate docs state it, and warn a service not to load grammars from untrusted input without its own bound.

Alternatives considered:

- An owned grammar behind `Arc`.
  Every `&'static` path in the tokenizer and its scratch would change.
  The scratch caches by grammar address, so it would need its own lifetime story.
- A grammar borrowed with a lifetime, `Grammar<'g>`.
  `Highlighter` would take that lifetime, which every caller then carries.

Either becomes worth it when a caller needs to unload a grammar.
No caller does today.

### 3. The build-script path writes bytes

`Grammar::compile(json)` returns the compiled tables, with includes of other grammars kept as scope names.
`to_bytes` writes them in nishiki's own format.
A build script writes them to `OUT_DIR`.
The crate loads them with `Grammar::from_bytes(include_bytes!(…), &grammars)`, which links the includes as `from_json` does.

- The bytes open with a header that names the format version.
  `from_bytes` refuses bytes from another version with an error.
- `from_bytes` checks every length and every rule, pattern and scope index before it trusts the tables.
  Truncated or crafted bytes return `GrammarError`, never a panic or an out-of-range read.
- Loading the bytes parses no JSON.
  Patterns still compile on first use, as they do today.
- Generated Rust source was considered.
  It would make the engine's table types public, and every engine change would then break the API.

### 4. Public API

- `Grammar`: a `Copy` handle to a compiled grammar.
  `Grammar::from_json(&str, &Grammars)` and `Grammar::from_bytes(&[u8], &Grammars)` return `Result<Grammar, GrammarError>`.
- `Grammars`: the grammars an include can name, by scope.
  An include may name a scope that a hand lexer covers, such as `source.css`.
  Its region then goes to that lexer, as the engine's hand-offs do today.
- `Highlighter::for_grammar(grammar)` and `Highlighter::for_grammar_with_limits(grammar, limits)`.
- `GrammarError` names what failed: the JSON, a missing root scope, an unknown format version, malformed bytes.
  A pattern the engine cannot compile is not an error: it never matches, as in vscode-textmate.

The exact names are settled in the implementation PR and recorded here.

### 5. Fidelity in CI

A CI job runs the oracle with Shiki 4.6.0 under Bun.
For each grammar Shiki bundles, the oracle highlights Shiki's sample for that grammar.
The crate loads the same grammar through the public API, and the job compares the two scope streams.

Nothing from the grammars is committed: CI fetches them each run.
The job fails on the first non-whitespace byte whose scopes differ, as the `shiki-fidelity` spec measures fidelity.

Grammars whose licence is unknown are still read for testing, as invariant 2 allows.
None of them ships.

### 6. Limits

A loaded grammar runs under the caller's `Limits`, as every language does.
The work and depth limits keep a hostile grammar from stalling a line.

## Risks

- Some Shiki grammars may rely on Oniguruma features the engine lacks.
  Each one found gets a task in this change, since the gate allows no gap.
- The compile cost at run time is unmeasured.
  The bench gains a row for it before release.

## Open Questions

None. The operator answered the questions that shaped this design on 2026-11-09.

[grammar]: https://github.com/victor-software-house/nishiki-archive/blob/cd811df3/src/textmate/grammar.rs#L224-L269
[survey]: https://github.com/victor-software-house/nishiki-archive/blob/cd811df3/src/textmate/tests.rs#L1284-L1414
