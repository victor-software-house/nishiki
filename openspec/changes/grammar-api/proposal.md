# Proposal

## Motivation

nishiki highlights only the languages it ships a lexer for.
A caller who holds the grammar of any other language has no way to use it.

The operator ruled on 2026-11-09 that 0.1.1 ships only with a generic way to highlight any language.
That way is nishiki's own TextMate engine, opened to callers.

The engine exists today, but no caller can reach it:

- It sits behind the opt-in `textmate` feature, and every item is `pub(crate)`.
- No language runs on it, since every bundled language moved to a hand lexer.
- It runs only tables that [`oracle/engine-tables.ts`][engine-tables] writes when the crate is built.

## Changes

- A public `Grammar` type compiles a TextMate grammar, in the JSON form Shiki and VS Code use, at run time.
- A build script can compile the same grammar ahead of time into bytes, which the crate loads with no JSON parser.
  Both paths share one compiler.
- A highlighter takes a grammar where it takes a `Language` today.
  A grammar can include other grammars the caller loaded, and hand a region to a language nishiki lexes by hand.
- The crate bundles no grammar.
  Callers bring their own, such as the ones in Shiki's `tm-grammars` package.
- CI loads every grammar Shiki 4.6.0 bundles through the public API.
  It fails when any byte of a grammar's sample gets other scopes than Shiki gives it.
- The engine's own limits stay configurable, as every lexer's are.

## Capabilities

### New Capabilities

- `grammar-loading`: a caller compiles a TextMate grammar, at run time or in a build script, and highlights text with it.

### Modified Capabilities

- `textmate-engine`: the engine reads grammar text a caller passes it.
  It still makes no request and reads no file of its own.

## Impact

- Public API: `Grammar`, its error type, a set of grammars that resolves includes, and a highlighter constructor that takes a grammar.
  All of it is additive, so the release stays a patch.
- `Cargo.toml`: the `textmate` feature turns on `json`, whose `serde_json` dependency the crate already declares.
  No new dependency.
- `oracle/engine-tables.ts` stays the oracle for the Rust compiler.
  It retires once the Rust compiler writes the same tables for every Shiki grammar.
- `AGENTS.md`, invariant 4: grammars a caller brings run on the engine, and the crate bundles none.

[engine-tables]: https://github.com/victor-software-house/nishiki-archive/blob/cd811df3/oracle/engine-tables.ts
