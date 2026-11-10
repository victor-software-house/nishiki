# Proposal

## Motivation

TypeScript scripts under `oracle/` write the generated Rust under `src/`:

- 48 per-language scripts, `oracle/<language>-grammar.ts`, write the rule-engine tables.
  With the three files they share, they hold 18,848 lines, and most of those lines are copies (design, section 1).
- 7 word scripts write `words.rs` files from grammar expressions.
- `oracle/unicode-tables.ts`, `oracle/case-folds.ts` and `oracle/classes.ts` measure Oniguruma and write the tables in the same run.

A fix to an emitter has to be made in each copy.
The scripts read grammars and regular expressions a second time, beside the compiler in `src/textmate/compile` and the parser in `src/textmate/regex`.

The rule engine is also private.
A caller can run a grammar of their own only on the TextMate engine.

The operator chose Rust for every generator (2026-11-10).

## Changes

- Generated Rust is written by Rust.
  A script under `oracle/` only asks Shiki or Oniguruma for facts.
  It records them as formatted JSON under `oracle/facts/`.
- A workspace crate, `codegen/` (`publish = false`), writes every committed generated file.
  `mise run codegen` runs it.
  CI runs it and fails when `git diff --exit-code` finds a change.
- nishiki gains two non-default features:
  - `codegen`: `proc-macro2` and `quote`, and the public code-writing functions;
  - `format`: `prettyplease` and `syn`, which lay the tokens out as a file.
- One stable public addition: a TextMate grammar a caller brings can be written as Rust tokens in a build script.
  The program then holds it as a `static`, with no load at run time.
- The rule engine becomes public under `--cfg nishiki_unstable`, as tokio does with `tokio_unstable`.
  `nishiki::unstable` holds the engine's grammar trait, its tables and its code writer.
  Each pattern gets a default matcher built from its regular expression, so a caller writes no matcher.
  Its items are exempt from semver.
- Languages move to the Rust writer one family at a time.
  Each pull request deletes the scripts it replaces.

## Capabilities

### New Capabilities

- `generated-rust`: every generated Rust file comes from a Rust writer that reads grammars and recorded facts.
- `unstable-rule-engine`: the rule engine under `nishiki::unstable`, with default matchers.

### Modified Capabilities

- `grammar-loading`: a build script can write a caller's grammar as Rust tokens.

## Impact

- `Cargo.toml`: the `codegen` and `format` features, with four optional dependencies.
  Default builds gain no crate.
  The `codegen/` crate joins the workspace.
- `src/grammar.rs`: one public item behind `codegen`.
- `src/textmate/compile.rs`: the unlinked model holds borrowed or owned data, so a `static` can hold a grammar.
- `src/unstable.rs`: a new module, compiled only with `--cfg nishiki_unstable`.
- `src/rules.rs`: its trait and tables become reachable through `nishiki::unstable`.
- Every generated file: a `// @generated` header, and `#[rustfmt::skip]` on its `mod`.
  prettyplease lays out the structured files, so their lines change; their tokens do not.
- `oracle/`: the 48 grammar scripts, their 3 shared files and the 7 word scripts go.
  `fence-stops.ts` becomes a check in `codegen/`.
- `oracle/facts/`: three fact files, written by the scripts that keep measuring.
- `tests/engine/grammars/`: the grammars of the 32 rule-engine languages it lacks.
  Each family appends its grammars to `oracle/engine-grammars.ts` and their licences to `oracle/licences.ts`.
- `AGENTS.md` and `CODING_STANDARDS.md`: the rule that Rust writes generated Rust, and the semver exemption of `nishiki::unstable`.
