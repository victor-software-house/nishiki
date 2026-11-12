# Design

## 1. Measure

A fix is proven on a box at seeds 1, 2 and 3.
Each seed writes the corpora the [research note][note] used, for the language it fixes:

- 300 `win` and 100 `mdw` documents from `oracle/fuzz.ts`;
- 300 `gen` and 100 `mdg` documents, where `oracle/fuzz/<language>.ts` exists;
- 100 bare and 100 fenced documents from `oracle/differential.ts`, where it has the language.

`the_corpus_matches_shiki` in `tests/it/oracle.rs` compares each corpus.
The fix is done when no document differs, and the oracle test still passes.
A fix that changes output names its languages in an `identity: changes <languages>` line.

## 2. `$base`

vscode-textmate resolves `$base` to the root grammar of the document.
Where Markdown embeds Objective-C, that root is Markdown, so Markdown's rules run inside Objective-C's scopes.
`oracle/rule-generator.ts` resolves `$base` to the grammar's own root.
The TextMate engine already resolves it to the root it links.

The fix follows vscode-textmate and the TextMate engine:

- A `$base` include is a slot in a generated pattern list, capture lists included.
- At that slot, the embedder's root patterns compete in their order.
  The same leftmost-first rule as the grammar's own patterns applies.
- When one of them wins, the embedder's rule opens as a frame on the shared stack.
- When the grammar is the document, the slot is its own root, as now.

The rule engine holds the slot's runtime part.
`oracle/rule-generator.ts` only emits the slot, so the port to `codegen/` stays mechanical.
The pull request records the runtime design.
It also records the cost to languages with no `$base`, which stays at today's.

Objective-C and Objective-C++ are the two hand lexers whose grammars include `$base`.
Both change in one pull request.
Shiki's Markdown has no Objective-C++ fence, so no document embeds Objective-C++.
Its slot is always its own root.

## 3. The fuzz script's guard

`oracle/fuzz.ts` needs pieces for `gen` and `mdg`.
Its guard tests `mode.endsWith("g")`, which `gen` fails.
The guard names both modes, and the pull request records where the `gen` documents of C#, Lua, R and Swift came from.

## 4. Rust's classes

Shiki's engine reads `\d` and `\w` with Unicode classes.
Other lexers read them from the tables in `oracle/classes.ts`.
Rust takes the same tables where its grammar's classes need them.
The grammar's exact classes are read first.

## 5. Svelte and seed 2

A seed corpus at 100 `win` documents per language found two Svelte cases on main:

- `{#each items as item, $i}{/each}` reaches the work limit, which stops the corpus test;
- after a `<script lang="` whose value runs into the script, Svelte stays in the string where Shiki enters the script.

Each is one rule, found from its documents.
At seed 2, one Ruby document differs and one Vue document differs.
The Ruby document is task 5.1's case.
The Vue document gets task 6.3.

## 6. Ruby, Mermaid and Vue

Each case is one rule, found from the documents in the research note.
Vue's cause is not known yet, so its task starts with a diagnosis.

## 7. Go and LaTeX

Go's `Language` docs list three cases that differ from Shiki:

- the state after a struct or interface field that ends in text such as `interface{&=`;
- a non-ASCII character earlier in a line, which can change the expression Shiki's engine matches;
- a long line of many words, where Shiki's engine stops an expression at its retry limit.

LaTeX's docs list one: code in a braced argument ends at its first `}`.
That holds for Haskell, Java, Julia, Rust and Scala, which have no halts of their own.

Each gets Shiki's behaviour, and its doc line moves to "Behaviour kept from Shiki" or goes.
The retry limit is Oniguruma's.
Its task first measures the limit on Shiki's engine, as `oracle/classes.ts` measures the classes.

## 8. The textmate tier

The Shiki fidelity job runs the engine on one input per language.
A run over `win` and `mdw` corpora sends mutated text through it, with every grammar Shiki loads.
It reports what the hand lexers' runs report.
It is a tool, as `the_corpus_matches_shiki` is, and not a CI gate.

## 9. Bounds

A lexer's own bounds, such as C++'s nesting depths, are per-line limits.
The next minor release moves them into `Limits`.
They get no fidelity task (lead, 2026-11-11).
Each bound's `Language` doc lists it under "Behaviour kept from Shiki" or "Differences from Shiki".

[note]: ../../research/2026-11-11-shiki-exact-gaps.md
