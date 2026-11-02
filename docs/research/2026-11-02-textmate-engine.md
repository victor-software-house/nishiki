# A TextMate engine inside nishiki

## Question

Can nishiki run Shiki's TextMate grammars with its own engine, written from scratch, with every grammar bundled and nothing downloaded?
What regex strategy, compiled form, loading and state model would that take, which of Shiki 4.6.0's grammars may be bundled, and how close does a first prototype get on fidelity and speed?

This note informs the decision.
It does not make it.

## Answer

1. Fidelity is not the obstacle.
   A prototype of 2,481 lines of Rust matches Shiki on 100.00% of non-whitespace bytes in all 23 oracle fixtures (Rust, shell, TypeScript, Markdown).
2. Speed is the obstacle.
   The unoptimised prototype runs at 0.06 to 8 MiB/s.
   The hand lexers run at 17 to 242 MiB/s on the same fixtures.
   The gap is 30x to 300x, and a design that closes it is described below but not built.
3. Licences exclude some grammars.
   Of 242 grammars in `tm-grammars` 1.32.22, 172 sit in MIT repositories and 23 in Apache-2.0 repositories.
   45 need an action: 7 are GPL or LGPL, 3 MPL-2.0, 13 have a licence file GitHub cannot classify, 21 have none, and 1 is not on GitHub.
4. The engine can add no dependency beyond `memchr`.
   One optional dependency, `regex-automata`, would help speed and needs approval.

## 1. What was run

All numbers come from one Apple M4 Pro (14 cores, 48 GiB), rustc 1.99.1, release builds, on 2026-11-02.
Shiki is 4.6.0 with per-line limits off, as `tests/oracle/SOURCES.md` records.
Grammars are the ones `shiki@4.6.0` ships in `@shikijs/langs` 4.6.0, which its `package.json` builds from `tm-grammars` `^1.32.22`.

1. Prototype commits: `67b37ce`, `ffa144b`, `19b6ada`, `211ef84` on a base of `322dd23`, in `research/textmate/`.
2. Grammars loaded together: `source.rust`, `source.shell`, `source.ts`, `text.html.markdown`.
   Their JSON is 14,801, 39,955, 163,900 and 56,874 bytes.
3. The oracle fixtures are the ones `tests/oracle/<language>/*.txt` holds.
   They were not split into development and held-out sets for this run, so the 100% below is not a held-out score.
4. Fidelity: `compare` scores each non-whitespace byte as equal when the whole scope stack equals Shiki's.
5. Speed: `compare --bench 5` for the prototype, and `mise run bench` three times for the hand lexers, median of the three runs per fixture.

## 2. Fidelity of the prototype

| Language | Fixtures | Bytes scored | Match |
|:--|--:|--:|--:|
| Rust | 3 | 93,510 | 100.00% |
| Shell | 4 | 225,583 | 100.00% |
| TypeScript | 5 | 152,548 | 100.00% |
| Markdown | 11 | 284,496 | 100.00% |

Two bugs took the first run from 20% (Markdown) and 5% (TypeScript) to 100%.
Both are in the engine's reading of Oniguruma and vscode-textmate, not in a grammar.

1. `^` must not match at the end of the text after a final newline.
   Oniguruma's `ANCHOR_BEGIN_LINE` requires that the position is not the end of the string.
   Without this, a `while` rule such as `^([\t ]*)$` matched at the end of every line, and no list ever closed.
2. vscode-textmate resets the enter position and anchor position of every open frame at the start of each line.
   Without this, a rule that opened at column 0 of one line compared equal to a zero-width end at column 0 of the next, and the end was refused.

Both are from running the prototype against the fixtures.
The vscode-textmate behaviours are reproduced from memory of its source, and the fixtures are the only check.
Where they disagree with Shiki, the prototype is wrong.

## 3. Speed of the prototype

Median MiB/s per fixture.
"Hand" is the median of three `mise run bench` runs of nishiki's hand lexers at `322dd23`.

| Fixture | Prototype | Hand | Ratio |
|:--|--:|--:|--:|
| rust/ripgrep-searcher | 0.86 | 149.0 | 173x |
| rust/serde-de | 1.20 | 228.2 | 190x |
| shellscript/nvm | 0.59 | 71.4 | 121x |
| shellscript/ruby-build | 0.81 | 73.5 | 91x |
| typescript/ky-core | 0.07 | 22.3 | 319x |
| typescript/zod-classic-schemas | 0.06 | 20.2 | 337x |
| markdown/book-ownership | 2.31 | 237.9 | 103x |
| markdown/nvm-readme | 2.14 | 172.8 | 81x |
| markdown/zod-api | 0.14 | 32.0 | 229x |

The other 14 fixtures are in the same range.
The prototype's own run was five repetitions in one process, not three sessions, so it meets `bench/AGENTS.md` rule 5 only in part.
The hand-lexer runs were on a laptop that was not quiet.
Neither set is a recorded run under rule 1.

The whole oracle set cost 421 million matcher steps on one pass.
Where the time goes was not profiled.
The causes below are reasoned from the design, not measured.

1. Every pattern of a rule is searched from the cursor on every step.
   vscode-textmate does the same, but Oniguruma's search is a compiled automaton with a literal prefilter.
2. The matcher is continuation-passing and recursive, with a heap closure per node.
3. TypeScript has 362 patterns and 257 lookbehinds across the four grammars, so most steps scan many alternatives.
4. Scope stacks are `Rc` lists with a `String` per name, and a token allocates a `Vec`.

## 4. Design

### 4.1 Regex strategy

Oniguruma semantics are needed, so a general regex crate does not fit.
Features the four grammars use, counted by `tools/survey.ts` over every `match`, `begin`, `end` and `while` pattern:

| Feature | Count | Feature | Count |
|:--|--:|:--|--:|
| Lookahead | 343 | `\G` | 95 |
| Lookbehind | 257 | Atomic groups | 3 |
| Backreferences | 19 | Possessive quantifiers | 42 |
| `\g<name>` calls | 7 | Class intersection `&&` | 11 |
| Named groups | 7 | POSIX classes (all TypeScript) | 188 |
| Inline flags | 65 | `\p{}` | 5 |

All 722 patterns of the four grammars parse and compile in the prototype (rust 90, shell 147, TypeScript 362, Markdown 123).

Options, checked by reading each project's documentation, not by running it here:

1. `onig` or `oniguruma-sys`: C code.
   It breaks the invariant that the crate is written from scratch, and the build needs a C toolchain.
2. `fancy-regex`: supports lookaround and backreferences, but not `\G` or `\g<name>`, and delegates the rest to the `regex` crate.
3. `regex-automata` alone: no lookaround and no backreferences.
4. An own backtracking matcher, as the prototype has.

The recommended path is 4, with a fast path.
Patterns without lookaround, backreference, `\G` or atomic constructs, about half of all patterns in these grammars (not counted), go to a `regex-automata` DFA when the dependency is approved.
The rest run on the own matcher, with a required literal prefix found by `memchr`, a first-byte set and an anchor class (`^`, `\G`, `\A`) computed at build time.
Whether the fast path pays for its dependency is open.

Semantics the prototype settled by running:

1. `\G` is the end of the last begin, while or end match, and the anchor is unset on a fresh line unless the frame began at the end of the previous line.
2. A `\1` in an `end` or `while` pattern is replaced by the escaped captured text when the rule opens.
   The result is a new pattern, so the engine needs a per-rule cache of resolved patterns.
3. Lookbehind tries start offsets between the minimum and maximum width of its body.
4. A step budget stops a runaway match.
   That budget would be `Limits::work`.

### 4.2 Compiled and bundled form

A build step reads each grammar's JSON and writes a compact table: rules in an array, patterns as a compiled program, scope names interned, includes resolved to rule indices.
The prototype keeps the same structure in memory.
Linking across grammars is by scope name, as Shiki does with `embeddedLangs`.

1. Where it runs: a generator in a separate workspace under `tools/` that writes a committed `src/<language>/table.rs` or a binary blob the crate includes.
   `build.rs` would put a JSON parser into every consumer's build, which `CODING_STANDARDS.md` forbids without approval.
2. Size: the 260 distinct grammars of Shiki 4.6.0 total 7.6 MB of JSON (`tools/inject-survey.ts`).
   The compiled size was not measured.
3. Rules that include an unbundled grammar are dropped, as Shiki does when an `embeddedLang` is not loaded.
   Markdown's fenced-code rules show this: Shiki drops every fence whose language is not loaded.
   Fidelity against Shiki therefore depends on which grammars are bundled together, and the note's 100% holds only with all four loaded.

### 4.3 Lazy per-grammar loading

Nothing is downloaded, so "lazy" means linked and parsed on first use.
A language is a feature or a module that costs nothing until `Highlighter::new(language)` runs.
Loading the four prototype grammars from JSON took 6.5 to 10 ms.
From a compiled table it should be less, and was not measured.

A grammar that embeds another (Markdown to Rust) loads the other on first reference.
The cross-grammar include graph was not surveyed beyond these four.

### 4.4 Running on the shared core

State must fit the arena.
A frame between lines needs only these fields, because the prototype resets the rest each line:

1. the rule index;
2. the resolved `end` or `while` pattern, as an interned id;
3. the name scope stack and the content scope stack, as arena ids;
4. one bit, "began at end of line".

That is a hash-consed frame, so `State` stays a small integer, as in the hand lexers.
The prototype uses `Rc` lists and does not do this; it is design, not tested.

Mapping to `Limits`:

1. `work`: matcher steps plus scan iterations.
2. `depth`: open frames.
3. `nodes`: arena growth.
4. `line_bytes`: unchanged.

The loop must also keep the guards the prototype has: a begin that pushes the same rule without advancing, a match rule that does not advance, and an end that does not advance at its enter position.
Without them the Markdown and TypeScript grammars loop on hostile lines.
`tests/it/limits.rs` and the fuzz test would apply unchanged.

### 4.5 Injections and embedded grammars

None of the four grammars has an `injections` key.
Across all 260 distinct grammars of Shiki 4.6.0, 22 have `injections` and 18 have an `injectionSelector` (`tools/inject-survey.ts`).
They are HTML-family and template grammars (HTML, Astro, Svelte, Liquid, Twig, ERB, Blade, Razor, Soy and others) and the `inline.es6-*` template-literal injections.
Injections need a scope selector matcher run against the current stack on every step, which the prototype does not have.
It is open work and not needed for the four languages here.

## 5. Licences of Shiki 4.6.0's grammars

`research/2026-11-02-textmate-engine/licences.tsv` lists every grammar with its source file, the licence `tm-grammars` 1.32.22 records, and the licence GitHub's licence API reports for the source repository.
The API was queried on 2026-11-02 for 171 distinct repositories.
GitHub reports a repository's licence, not each file's, so a repository-level MIT does not prove a vendored or generated grammar file is MIT.
Licence text was not read file by file, and `THIRD-PARTY-NOTICES.md` work is not started.

| Repository licence (GitHub) | Grammars |
|:--|--:|
| MIT | 172 |
| Apache-2.0 | 23 |
| BSD-3-Clause, ISC | 2 |
| MPL-2.0 | 3 |
| GPL-3.0 or GPL-2.0 | 6 |
| LGPL-3.0 | 1 |
| NOASSERTION (a licence file GitHub cannot classify) | 13 |
| none found | 21 |
| not on GitHub | 1 |

The four languages the prototype runs are all from `microsoft/vscode` (MIT): `rust`, `shellscript`, `typescript` and `markdown`.

Flagged under the licence decision, each of them needing a hand lexer, a different source grammar or a drop:

1. GPL or LGPL: ada, ahk2, gnuplot, nginx, org, racket, v (GitHub says GPL-2.0, `tm-grammars` says MIT: a disagreement).
2. MPL-2.0 (file-level copyleft): bird2, hcl, terraform.
3. NOASSERTION, to be read by hand: ahk, apache, apex, asciidoc, cue, elixir, llvm, luau, nim, purescript, r (`tm-grammars`: MIT), sass, wolfram.
4. None found: abap, apl, applescript, dax, dream-maker, erb, glsl, hurl, kusto, logo, matlab, mipsasm, po, razor (`tm-grammars`: MIT), rel, sparql, ssh-config, tcl, toml, turtle, yaml.
5. Not on GitHub: ts-tags.

A none-found repository is not a permissive one: no licence file means all rights reserved by default.
Whether `tm-grammars`' own licence field is enough where GitHub finds none was not decided.

## 6. Dependencies the engine would need

1. Required: `memchr` only, which the crate already has.
2. Optional, needs approval: `regex-automata`, for the lookaround-free fast path (4.1).
3. Build time only, outside the crate: `serde_json` in the generator workspace.
   The prototype uses `serde_json` and nothing else.

## 7. What to decide, and what is open

Open questions the prototype did not answer:

1. Whether an own matcher with prefilters reaches the per-line budget on TypeScript, which is 300x away.
   The prototype is a design baseline, not an optimised engine.
2. How much a `regex-automata` fast path gains, and how many patterns qualify.
3. The compiled table's size and load time.
4. Injection selectors and their cost.
5. Whether the other 256 grammars hold at 100%: only four were run.
6. Which grammars with an unclear licence can ship.

## 8. Not verified

1. No profile of the prototype exists, so the causes of slowness in section 3 are reasoned only.
2. The behaviours of vscode-textmate reproduced here were written from memory and checked only against the fixtures.
3. The assessment of `onig`, `fancy-regex` and `regex-automata` comes from their documentation as remembered, not from a run.
4. The oracle fixtures were not split, so no held-out score exists for the prototype.
5. The `\p{}` classes use Rust's character tests, which differ from Oniguruma's tables for some scripts; `}+` is not read as a possessive quantifier.
6. The hand-lexer speed was measured on a laptop that was not idle.
7. `tm-grammars` 1.32.22 is the dev dependency `@shikijs/langs` 4.6.0 declares as `^1.32.22`; the installed range was not checked.

## 9. Compute

1. One Apple M4 Pro laptop, builds, runs and surveys, not timed, at no cost.
2. Network: `gh api` calls for 171 repositories and the Shiki and `tm-grammars` downloads, at no cost.
3. No cloud, Namespace or paid run.
