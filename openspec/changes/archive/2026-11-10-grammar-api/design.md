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
It is written from scratch, with vscode-textmate's behaviour as the reference (invariant 1).
It works in two steps.

`compile` reads one grammar into plain data.

- Every rule object of the grammar is one rule, read once.
- Includes stay names, both those of the grammar's own rules and those of other grammars.
- A compiled grammar therefore depends on no other grammar.

`link` resolves those names when a highlighter is built (operator, 2026-11-10).

- It takes the root grammar and the caller's `Grammars` set.
- It follows the includes from the root, depth first, and numbers each rule where it is first reached.
- It interns patterns, scope names and lists.
- It writes one flat table that the tokenizer runs as it runs any table.
- It follows grammars that include each other once each, so they load in any order.
- The tokenizer does not change, and keeps no set.

Includes inside a grammar resolve when it is linked, not when it is compiled.
vscode-textmate compiles a repository rule with the repositories in scope where the rule is first reached.
With nested repositories, that place depends on the order the link visits rules in.

An include of another grammar's scope resolves in this order (operator, 2026-11-10):

1. a language lexed by hand that the caller names for hand-offs, even when the set holds its grammar;
2. a grammar in the set;
3. otherwise the rule is dropped, as `oracle/engine-tables.ts` drops it.

With no languages named, they are every language lexed by hand whose grammar the set lacks.
An include of one rule of another grammar, such as `source.ruby#escaped_char`, always resolves in the set.
A named language's grammar then serves only such includes.

The run-time path and the build-script path both call the compiler.
The TypeScript generator stays as its oracle.
A test compares the two compilers' tables for every Shiki grammar, and the TypeScript generator retires once they are equal.

### 2. Grammars live for the process

A compiled grammar is moved to the heap and leaked.
A process-wide set interns compiled grammars.

- `Grammar::from_json` keys it on the JSON text, compared in full after a hash match.
- `Grammar::from_bytes` keys it on the bytes, the same way.
- Loading the same text twice returns the first handle.
- One grammar loaded once from JSON and once from bytes gets two handles.
  Their output is the same.

A handle holds no link to another grammar, so one handle serves every `Grammars` set.

A linked table is leaked too, so the engine keeps its `&'static Grammar`.
A second process-wide cache interns linked tables.
Building the same highlighter twice therefore links once.
The cache's key has three parts:

- the root handle;
- every handle of the set, in the order it was added;
- the languages named for hand-offs.

A key of only the grammars the link read would be wrong.
It would miss the scopes the link looked up and did not find, and the grammars that inject themselves.

Memory grows with each distinct grammar and each distinct link, and stays until the process ends.
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

A build script calls `Grammar::from_json(json)?.to_bytes()` and writes the bytes to `OUT_DIR`.
The crate loads them with `Grammar::from_bytes(include_bytes!(…))`.
The bytes hold the compiled grammar, before linking.

- The bytes open with a 16-byte header:
  - the magic `nishiki` and a zero byte;
  - the format version, a little-endian `u32`;
  - the length of the body, a little-endian `u32`.
- The format version is its own counter, starting at 1.
  Any change to the layout bumps it.
  A release that keeps the layout keeps older bytes loadable.
- `from_bytes` accepts only the current version.
  Its error names the version found and the one expected.
- The body holds little-endian `u32` counts and indices, and strings as a length then UTF-8.
- `from_bytes` checks every count against the bytes left before it allocates.
  It checks every rule, list, pattern, name and capture index against its table's length.
  Truncated or crafted bytes return `GrammarError`, never a panic or an out-of-range read.
- Loading the bytes parses no JSON.
  Patterns still compile on first use, as they do today.

Alternatives considered:

- Generated Rust source.
  It would make the engine's table types public, and every engine change would then break the API.
- A separate `Grammar::compile` that returns the compiled form without leaking it.
  It adds a public type for a build script, a process that exits right after.
- A header with the crate version.
  It would refuse bytes from a release that changed nothing.
- A checksum.
  Crafted bytes pass it, and the checks above already refuse them.

### 4. Public API

Every item is behind the `textmate` feature, which turns on `json`.
A program that never calls `from_json` does not link `serde_json`'s parser.

- `Grammar`: a `Copy` handle to a compiled grammar, equal by address.
  - `Grammar::from_json(&str)` and `Grammar::from_bytes(&[u8])` return `Result<Grammar, GrammarError>`.
  - `Grammar::to_bytes` writes the bytes of decision 3.
  - `Grammar::scope` gives the root scope, such as `source.zig`.
- `Grammars`: the grammars an include can name, by scope.
  - `new`, `insert`, which returns the grammar it replaced, and `get`.
  - `FromIterator<Grammar>` and `Extend<Grammar>`.
  - `hand_off(Language)` names a language to lex by hand.
    A whole include of its scope goes to its lexer, even when the set holds that grammar.
    An include of a part of a grammar, such as `source.ruby#escaped_char`, always resolves from the set.
  - With no language named, an include goes to a hand lexer only when the set lacks its grammar.
    A grammar in the set wins, since the caller loaded it on purpose.
    The fidelity job of decision 5 relies on it, as it tests every Shiki grammar.
- `Highlighter<S = Language>` takes a type parameter, `S: Syntax`.
  - `Syntax` is a sealed trait, implemented for `Language` and `Grammar`.
  - `Highlighter` alone still names `Highlighter<Language>`, with `new`, `new_with_limits` and `language`.
  - `Highlighter<Grammar>` has `for_grammar(grammar, &grammars)` and `for_grammar_with_limits(grammar, &grammars, limits)`, and `grammar`.
    Both link, so both return `Result<Highlighter<Grammar>, GrammarError>`.
  - `line`, `limits`, `scopes` and `style` serve both.
  - A defaulted type parameter is a minor change in Cargo's SemVer rules, so the release stays a patch.
  - Each syntax's `line` is compiled in nishiki, not in the caller's crate.
    The hand lexers' path then runs the same code as before.
- `GrammarError`: an opaque struct, as `ThemeError` is, with `Display` and `Error::source`.
  It names what failed: the JSON, a missing root scope, an unknown format version, malformed bytes, or a link the engine refuses.
  A pattern the engine cannot compile is not an error: it never matches, as in vscode-textmate.

Alternatives considered for the highlighter:

- One `Highlighter` that holds a language or a grammar.
  `language()` would return `Option<Language>`, an API break.
  Every line of every hand lexer would also take a branch, which the Callgrind gate sees.
- A separate `GrammarHighlighter`.
  Every public method would be written and documented twice.

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
