# Design

## Context

[proposal.md](proposal.md) states why.
This design records where the instructions go, ranks the levers, and orders the slices.

## 1. How the profile was taken

- Commit: [b22d35b4][base], main on 2026-11-05, before R landed on the rule engine.
- Harness: a scratch example outside the repository.
  It reads the bench corpus's medium, large and reply tiers: 7,364,626 bytes in 59 languages, 1,358,690 tokens.
  It splits every file into lines before any phase.
- Phases, each its own function that Callgrind toggles on:
  - setup: `Theme::from_json` with github-dark and one `Highlighter::new`;
  - first: the first pass of a fresh process, lexing and styling every token;
  - warm lex: a second pass, lexing only, a fresh highlighter per document;
  - warm style: `Theme::style` on every token of the already lexed corpus;
  - warm colour: lexing and styling, as the comparison's nishiki engine does.
- Builds: `--features textmate,json`, `MBX_DISABLE=1`, the release profile at one codegen unit and at the default sixteen.
  The repository sets no release profile of its own, so sixteen units is what a user's release build gets.
- Counts: valgrind 3.27.1 from the bench profile, on a box, each phase and language its own run.
  The counts are instructions (Ir).
- Scan counters: a second scratch build counted positions the rule scan visits, empty candidate lists, patterns tried and hits.

## 2. Where the instructions go

The whole corpus, at one codegen unit:

| Phase | Instructions | Per byte | Per token |
|:--|--:|--:|--:|
| setup | 1.1 M | | |
| warm lex | 3,003.6 M | 408 | 2,211 |
| warm style | 5,546.3 M | 753 | 4,082 |
| warm colour | 8,511.5 M | 1,156 | 6,264 |
| first, beyond warm colour | 780.4 M | | |

Sixteen units read within 2% of these totals.
Per language, sixteen units over one unit sits between 0.910 and 1.075 for warm lexing.

Styling is 65% of warm colour.
Over the 59 languages, the geometric mean of warm colour is 697 instructions per byte, and of warm lex 216.

## 3. Styling

`Theme::style` costs 936 to 12,554 instructions per token, by language.
JSON is the worst, because its stacks are deep.

| Function | Share of warm style |
|:--|--:|
| `Trie::apply` | 28.6% |
| SipHash of `&str`, `hash_one` and `Hasher::write` | 40.0% |
| `memchr`, from `str::split('.')` | 9.6% |
| `Theme::style` itself | 4.7% |
| `Scopes::next` | 3.4% |
| malloc, realloc and free | about 9% |
| `memcmp` | 2.5% |

The cause, read from [`theme.rs`][theme] and [`trie.rs`][trie]:

1. `Theme::style` collects the scope names into a `Vec`, which allocates on every token.
2. It then calls `Trie::apply` once per depth, so a stack of depth d walks the trie d times.
3. Each walk splits the scope at dots and looks up each part in a `HashMap<String, usize>` with SipHash.

A token's style depends only on its stack.
A stack's style is its parent stack's style with one `apply` for its top scope, because `apply` at depth d reads only the first d scopes.
Highlighters intern stacks, so a cache indexed by stack id computes each stack's style once.
vscode-textmate does the same: it computes a scope's style when it pushes the scope.

## 4. First document

Work a fresh process does once, by function, summed over the 59 languages:

| Function | Instructions |
|:--|--:|
| `rules::tables::candidate_lists` | 541.0 M |
| `OnceLock` initialisers, of which shell's `rule_info` 56.3 M | 119.8 M |
| `memcmp` | 37.7 M |
| `c::lists::Table::build` | 34.3 M |
| malloc and free | about 30 M |

By language: C++ 112 M, C 95 M, C# 80 M, shell 66 M, Objective-C++ 55 M, Go 45 M.

[`candidate_lists`][tables] builds each rule's 768 candidate lists, one per byte and place.
For each list it filters every pattern's start, then looks the list up in a `HashMap<Vec<usize>, _>` with SipHash.
That is 768 times the patterns, plus 768 hashed vectors, for every rule.

## 5. Lexing

Instructions per byte of warm lex at one codegen unit, for the most used languages:

| Language | Lexer | Per byte |
|:--|:--|--:|
| JavaScript | hand | 617 |
| Python | rule engine | 233 |
| TypeScript | hand | 368 |
| HTML | own engine | 167 |
| CSS | rule engine | 264 |
| Shell | own engine | 156 |
| Java | rule engine | 182 |
| C# | rule engine | 365 |
| C++ | rule engine | 2,030 |
| C | rule engine | 546 |
| Go | rule engine | 536 |
| Ruby | rule engine | 535 |
| Rust | hand | 99 |
| JSON | hand | 47 |
| Markdown | hand | 58 |

Over the 41 languages on the rule engine, each language weighted equally:

| Part | Share |
|:--|--:|
| matchers | 50.3% |
| the scan | 27.3% |
| engine bookkeeping: emit, paths, frames, `tokenize` | 11.1% |
| other | 7.0% |
| libc | 3.4% |
| arena | 0.8% |

The scan counters, per language:

| Language | Positions per byte | Empty lists | Tries per position | Tries per hit |
|:--|--:|--:|--:|--:|
| Python | 0.63 | 3.6% | 2.68 | 21.1 |
| Go | 0.58 | 5.4% | 8.43 | 25.2 |
| Ruby | 0.61 | 42.2% | 8.31 | 35.2 |
| C++ | 0.53 | 8.2% | 15.58 | 42.4 |
| C# | 0.39 | 40.3% | 6.20 | 17.6 |
| Java | 0.72 | 79.7% | 1.22 | 16.8 |
| CSS | 0.38 | 28.8% | 4.84 | 12.6 |

Empty lists are rare in the busiest languages.
The cost is failed tries: each hit follows 17 to 42 patterns that failed.

A second counter run counted tries and hits per pattern.
Failures spread over many patterns: Go's most failing pattern holds 4.9% of its failures, C++'s 3.3%.
Python's `DocstringPrompt` is the exception, with 27.4% of Python's failures.
Only 7% to 15% of failures in Go, C++ and Python happen at a word start.
Lexing costs 110 to 250 instructions per try in these languages, so a failing matcher does work before it fails.
In C++ that work is the shared character steps: `word_in` 8.0%, `gaps` 7.4%, `skip_space` 5.3%, `gap_word` 3.6% and `ident_ends` 2.5% of its instructions.
Every pattern tried at one position repeats them.

## 6. Levers, ranked by expected gain

The projections are arithmetic on the profile in sections 2 to 5.

| # | Lever | Where | Projected gain | Proof |
|:--|:--|:--|:--|:--|
| 1 | A per-stack style cache, `Highlighter::style` | `highlighter.rs`, `theme.rs` | warm colour 8,512 M to about 3,050 M; per language, the geometric mean falls from 697 to about 221 instructions per byte, 3.15 times | the colour row of Callgrind (lever 0); a test that the cached style equals `Theme::style` on every oracle fixture and theme |
| 2 | Linear candidate tables | `rules/tables.rs` | first document falls by 541 M over the corpus, up to 98 M for C++ | the first-document row of Callgrind; a one-off check that the new tables equal the old for every rule of every grammar |
| 3 | A memo of the shared character steps at one position | `rules/chars.rs`, `rules/lexer.rs` | not yet projected: C++ spends 27% in those steps, and task 5.1 counts how often a position repeats them | the lexing rows of Callgrind |
| 4 | `Theme::style` without allocation or SipHash | `theme.rs`, `theme/trie.rs` | about half of warm style for callers of `Theme::style`: the allocation is 9% and SipHash 40% | the style-only row; the colour test |
| 5 | Per-language matchers, by use and by instructions per byte | each language | per language | the lexing rows of Callgrind, the Shiki differential |
| 6 | Per-language one-time work: shell's `rule_info` 56 M, C's list tables 34 M | `shell/`, `c/` | first document of those languages | the first-document row |

Lever 0 is measurement, not speed.
`mise run callgrind` counts only the second lexing pass today.
It gains two rows per language, both gated like lexing:

- first: the first pass of a fresh process, lexing and styling;
- colour: a warm pass that lexes and styles every token.

The harness then needs github-dark, so both sides build with `json` too.

### The style cache's interface

`Highlighter::style(&mut self, theme: &Theme, stack: Stack) -> Style`.

- The highlighter keeps a `Vec` of styles indexed by stack id, filled on first use.
- A stack's style is its parent's cached style with one `apply` for its top scope.
- Each `Theme` gets a process-unique id when it is built, and a clone keeps it.
  The cache drops its entries when it is called with another theme.
- It allocates only when a new stack first appears, as the arena does.
- `Theme::style` stays, for scopes that come from elsewhere.

Agreed with the streaming design on 2026-11-05 (`openspec/changes/streaming/`):

- `Document` owns its theme and calls `Highlighter::style` for every token of each line it lexes.
- Its reads are `&self`, through one crate-internal method, `cached_style(&self, theme: &Theme, stack: Stack) -> Option<Style>`.
  It returns `None` for a stack not yet styled, and for a cache filled under another theme.
- A binding's style table is indexed by stack id.
  New stacks appear as a document grows, so a batch carries the entries it first uses, and a theme change resends the table.
- Either change can land first; the public method waits for the operator's approval with `Document`'s.

The crate docs' quick start and the comparison harness use the new method.
The comparison's Shiki and Pierre rows already show what a user of each library gets, so nishiki's row should too.

### Lexing beyond lever 3

Pierre's lexers reach about 63 instructions per byte on the comparison host.
That figure is an estimate from its throughput, not a count.
After lever 1, the geometric mean of colour is about 221, of which lexing is 216.
So the rest of the road is lexing.
Lever 3 is the one engine lever the profile supports now.
Lever 5 takes languages in this order, by use and then by instructions per byte:
JavaScript and TypeScript, Python (`DocstringPrompt`), C++, C#, C, Go, Ruby, SQL.
Each gets its own measurement before its slice.

## 7. Slices

One pull request per slice, in this order (lead, 2026-11-05).
This plan and slices 1 to 3 form one stack, the plan at the bottom.

1. Callgrind counts the first pass and the colour pass (lever 0).
   The theme trie's children move to the fixed FNV hasher in this slice.
   With std's seeded hasher, two builds of one tree read Python's colour row 0.64% apart.
2. `Theme::style` without allocation (lever 4).
3. Linear candidate tables (lever 2).
4. The shared character memo (lever 3), after task 5.1 measures it.
5. Per-language slices (levers 5 and 6), each with its measurement.
6. The per-stack style cache (lever 1), last.

The style cache needs a public method.
`Theme::style` takes scope names, not a stack id, so no cache can sit behind it.
The operator approves public API.
A document-level streaming API is being designed in `openspec/changes/streaming/`, and styled output per change belongs to it.
So the cache's surface is agreed with that design before its slice is sent.
The lead brings both APIs to the operator together.

Every slice keeps output unchanged: no `identity:` line, the oracle green, the Shiki differential at 0 for every language it touches.
Every slice gates at one codegen unit: every row at or below 1.000, and a row above it gets a per-function split.

### Collisions with language work in flight

- Slices 1, 2 and 6 touch no lexer and no `Grammar` trait.
- Slice 3 changes `Tables::build` only.
  Its inputs, `Grammar::pat_start` and the rule data, keep their signatures.
  A language that lands first needs no change.
- Slice 4 changes `rules::chars`, which every rule-engine matcher calls.
  Before it ships, every implementor on main and on the open language branches is checked.

## 8. Not verified

- Wall-clock gains.
  The figures here are instruction counts; the comparison host moves by up to 11% between runs.
- The projections in section 6 are arithmetic on the profile, not runs.

[base]: https://github.com/victor-software-house/nishiki-archive/commit/b22d35b4
[theme]: https://github.com/victor-software-house/nishiki-archive/blob/b22d35b4/src/theme.rs#L346-L354
[trie]: https://github.com/victor-software-house/nishiki-archive/blob/b22d35b4/src/theme/trie.rs#L281-L301
[tables]: https://github.com/victor-software-house/nishiki-archive/blob/b22d35b4/src/rules/tables.rs#L139-L169
