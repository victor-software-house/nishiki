# Tasks

Every slice is measured against its base on a box, with valgrind 3.27.1 from the bench profile.
Builds use one codegen unit and `MBX_DISABLE=1`, three builds per side with equal hashes.

The CI gate fails a row that rises more than 0.5%.
Review asks every row to read at or below 1.000, and a per-function split for a row above it.

## 1. Plan

- [x] 1.1 The lead agrees the design → verify: their reply recorded here
  - Lead, 2026-11-05: approved.
    One stack: this plan, then the Callgrind rows, then `Theme::style`, then the candidate tables.
    The Callgrind rows are approved with `json` on both sides.
    The style cache's public method is held.
    It goes last, with its surface agreed with the streaming design, and the lead takes it to the operator.

## 2. Callgrind rows for the first pass and the colour pass

- [ ] 2.1 `examples/lex.rs` counts the first pass and a colour pass per language, and `mise run callgrind` gates them.
  → verify: the Callgrind job prints three tables, and every lexing row reads 1.000 against the slice's base
- [ ] 2.2 The theme trie's children use the fixed FNV hasher.
  → verify: two builds of one tree read every first-pass and colour row within 0.1% of each other

## 3. `Theme::style` without allocation

- [ ] 3.1 Built and measured → verify: every colour row falls, every lexing row stays at or under 1.000, and the colour test passes

## 4. Linear candidate tables

- [ ] 4.1 `candidate_lists` works on bit sets of patterns per byte.
  → verify: a one-off check finds the old and new tables equal for every rule of every grammar
- [ ] 4.2 Measured → verify: every first-pass row of a rule-engine language falls, every other row stays at or under 1.000

## 5. The shared character memo

- [ ] 5.1 Count, per scan position, how often matchers repeat the word, blank and identifier steps, in C++, Go, Ruby and Python.
  → verify: the counts are recorded here, with the decision to build or drop the memo
- [ ] 5.2 Built and measured, if 5.1 keeps it.
  → verify: the lexing rows of the rule-engine languages fall, and no row rises above 1.000

## 6. Per-language slices

- [ ] 6.1 JavaScript and TypeScript
- [ ] 6.2 Python, starting with `DocstringPrompt`
- [ ] 6.3 C++
- [ ] 6.4 Shell's `rule_info` and C's list tables, for the first document

## 7. The per-stack style cache

- [x] 7.1 Its public surface agreed with the streaming design, and approved by the operator → verify: the agreement recorded here
  - 2026-11-05: agreed with the streaming change, as design section 6 records.
    The operator approved `Highlighter::style` together with `Document` (lead, 2026-11-05).
    `cached_style` lands with its caller, in the streaming change.
- [ ] 7.2 Built → verify: the oracle colour test finds it equal to `Theme::style` for every token of every oracle fixture with every oracle theme
- [ ] 7.3 Measured → verify: the colour pass with it costs fewer instructions than with `Theme::style` on the same build, for every language
- [x] 7.4 Once it lands, the Callgrind harness colours with it (lead, 2026-11-05).
  → verify: the gated colour row measures `Highlighter::style`, and an unchanged pull request reads 1.000 on it

## 8. What streaming needs from the lexers

The streaming change raised these needs on 2026-11-05 (`openspec/changes/streaming/`, design decision 11).
Its figures come from `bench:latency` and `bench:cold` at 68620d9e, on a box outside the reference class, so they are indicative.

- [ ] 8.1 A first line under 100 µs in every language.
  → verify: `bench:cold`'s first line and `bench:latency`'s first-contact maximum, medium tier, read under 100 µs for every language.
  Shell read 413 µs and 966 µs, C++ 531 µs.
- [ ] 8.2 A cold first document without the one-time table cost.
  → verify: `bench:cold`'s first document falls for shell, Objective-C++, Less and C++, and the large tier is rerun
- [ ] 8.3 `bench:latency` and `bench:cold` measure the textmate tier → verify: Assembly, HAML and the other textmate languages have rows
- [ ] 8.4 Lexing one line is linear in its length.
  → verify: a probe counts instructions at every prefix of a long minified JSON line, a long JavaScript line and a long Markdown paragraph.
  It finds no growth faster than linear.

## 9. Rows that do not follow other languages' layout

Some libc calls cost more or less with the address of their data.
A language's tables then move other languages' rows.

MATLAB rose 0.9% to 1.8% when Assembly's `.rodata` grew (lead, 2026-11-05).
`bytes.contains(&b)` on a static of 16 or more bytes calls core's `memchr_aligned`, which steps byte by byte to an aligned address.

tsx's colour row rose 0.7% at equal call counts when the candidate tables changed (2026-11-05).
Each theme trie lookup ended in `memcmp` on a few bytes; slice 2 removes it.

HCL's pull request moved Go's and CoffeeScript's rows ±0.2% at equal call counts (lead, 2026-11-05).
All of it was in `memcmp` on static word tables.
The sites were `Lexer<Go>::word_ends_at` (521,582 calls), `Chars::word_in` (63,590 calls) and `Lexer<Coffee>::listed` (59,354 calls).

`Chars::word_in` is shared by every rule-engine language.
The byte loop goes in one crate-level helper that the theme and the lexers share.

MATLAB's fix ([#565][pr565]) left these `contains` sites on byte sets of 16 or more bytes (lead, 2026-11-05):

- Java's `stops` in `code.rs` (17 bytes);
- Python's `strings.rs` (18);
- YAML's `lexer.rs` (18 and 22).

It added a `const fn table(bytes) -> [bool; 256]` in `src/matlab/matchers.rs`, and Markdown's `INLINE_START` builds one inline.
Both fold into the crate-level helper, so no copy remains.

- [ ] 9.1 List every `contains` on a byte static of 16 or more bytes, and every `memcmp` on short strings in a hot path, on main.
  → verify: the list recorded here, with the sites above
- [ ] 9.2 Replace them with `const [bool; 256]` tables or byte loops, one pull request → verify: Identity unchanged, and every row at or below 1.000

[pr565]: https://github.com/victor-software-house/nishiki-archive/pull/565
