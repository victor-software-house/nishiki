# Classes of the byte before a position in the rule engine's scan

Does it pay to split the scan's positions away from words by the byte before them, so that a pattern whose leading lookbehind of one character fails there is not tried?

## Context

- The rule engine's scan tries, at each position, the candidates its tables list for the place and the byte.
- Before this prototype, the places were the start of the line, inside a word, and anywhere else.
- After [B1], a lookbehind that needs a byte keeps a pattern off the start of the line.
  A positive lookbehind of one character with no ASCII word character keeps it out of words.
- What remained were tries away from words whose lookbehind fails on the byte before.
  Counted on B1 with a throwaway counter in `try_pat`, for one pass over each language's oracle fixtures:

  | Language | Tries | Lookbehind fails on the byte before |
  |:--|--:|--:|
  | swift | 902,807 | 68,417 |
  | ruby | 783,411 | 30,199 |
  | cpp | 2,452,016 | 6,029 |
  | go | 699,249 | 5,088 |
  | scala | 102,053 | 4,506 |
  | python | 427,146 | 4,525 |
  | perl | 30,152 | 3,255 |
  | sass | 27,255 | 2,490 |
  | css | 38,121 | 1,554 |

- Most are Swift's `(?<=\.)` and `(?<!\.)` after a space, Ruby's `(?<!\.)` after a space, and Ruby's `(?<=\{)` after a word byte before a blank.

## The prototype

The [prototype], on [B1]:

- Each grammar sorts the byte before a position away from words into at most three classes.
  Bytes in one class satisfy the same chosen leading lookbehinds of one character.
  Bytes past ASCII stay in class 0, where every lookbehind may hold.
- A lookbehind weighs the patterns that open with it times the ASCII bytes it fails after.
  The heaviest are chosen while the classes number at most three.
  Swift splits on `.`, Ruby on `{` and `.`, Go on `)`, Perl on `{`.
- `Place::Away` carries the class, and each rule's tables keep a row of 256 lists per class.
- A negative lookbehind counts only when its body is one literal character.
  The pattern analysis reads a character's bytes as a superset, such as every byte for `\S`.
  That is sound for a positive lookbehind, but a negative one would then fail after every byte.
  The first round showed it: Hack's XHP attribute `(?<!\S)` lost every ASCII class.
- A grammar with no such lookbehind keeps one class and no lookup.

## Results

On a 4x8 build box, [prototype] against [B1], at one codegen unit, with `mise run callgrind`.
The full tables are beside this note: [lexing], [first document], [splits], [size] and [cold].
clippy, the lib tests, `cargo test --test it` and Identity passed.
The Shiki corpus test on 3,600 documents from `oracle/fuzz.ts` gave the same 21 differing documents as B1.

### Lexing

- Swift and Ruby, against B1's base [A] then B2 against B1:

  | Language | B1 against A | B2 against B1 |
  |:--|--:|--:|
  | swift | 0.9690 | 0.9879 |
  | ruby | 0.9959 | 0.9874 |

- Falls below 0.9980: astro 0.9230, perl 0.9799, ruby 0.9874, swift 0.9879, sass 0.9950, erb 0.9950, svelte 0.9953, scala 0.9961, tex 0.9963, haml 0.9964, mermaid 0.9965, matlab 0.9966, postcss 0.9972, python 0.9974, go 0.9976.
- Rises in 24 grammars: bat 1.0048, kotlin 1.0040, julia 1.0031, dart 1.0027, java 1.0023, zig 1.0018, nix 1.0014, objective-c 1.0013, and 16 below 1.0010.
- Split by function, bat's +14,251 and Kotlin's +112,313 instructions are all in their `scan`: the class lookup at each position, where few tries are skipped.

### First contact

- Callgrind's first document rises in 44 languages by more than 0.5%:

  | Language | Base, M | Head, M | Head/base |
  |:--|--:|--:|--:|
  | latex | 83.4 | 123.7 | 1.4827 |
  | bat | 11.8 | 14.7 | 1.2427 |
  | haskell | 37.3 | 45.8 | 1.2271 |
  | csharp | 155.7 | 187.6 | 1.2053 |
  | perl | 33.5 | 38.5 | 1.1498 |
  | astro | 74.4 | 78.2 | 1.0499 |
  | swift | 390.7 | 404.5 | 1.0353 |
  | ruby | 2,652.7 | 2,670.6 | 1.0068 |

- Split by function, latex's +40.2 M is +35.0 M in `rules::tables::candidate_lists` (40.8 M to 75.8 M), the rest in `memcmp`, `memset` and the `OnceLock<Tables>` initializers (+2.16 M, +1.18 M and about +1.69 M).
  The extra rows double the work of building a rule's tables.
- `mise run bench:cold`, medians in µs, on the same box, so timings carry its noise:

  | File | B1 first line | B2 first line | B1 document | B2 document |
  |:--|--:|--:|--:|--:|
  | swift/small/snapkit-constraint-item.swift | 99.8 | 122.6 | 476.7 | 648.0 |
  | ruby/small/faraday-middleware.rb | 91.3 | 113.4 | 391.1 | 523.1 |

### Size

`size -A` of `examples/lex`, default features plus `json`, at one codegen unit:

| Section | B1 | B2 | Change |
|:--|--:|--:|--:|
| .rodata | 1,174,076 | 1,182,268 | +8,192 |
| .text | 6,362,387 | 6,365,315 | +2,928 |
| .relro_padding | 1,352 | 2,520 | +1,168 |
| .gcc_except_table | 33,992 | 34,000 | +8 |
| Total | 9,824,636 | 9,836,932 | +12,296 |

## The bug the first round caught

- The first round ran an earlier version of the prototype, never pushed.
  It failed `hack::tests::xhp_in_an_expression` and the oracle tests.
- Hack's XHP attribute pattern opens with `(?<!\S)`.
  The analysis read `\S` as every byte, so the negative lookbehind seemed to fail after every byte.
  The pattern left every ASCII class, and `class` in `<div class="a">` lost its scope.
- The fix counts a negative lookbehind only when its body is one literal character.
  `oracle/rule-grammar.test.ts` covers `(?<!\S)`, `(?<![a-z])` and `(?<!:)` at the [prototype].
- B1 does not have this hole: its rules read only positive lookbehinds and `(?<!\w)`, which has its own exact check.

## Reading

- The lexing win is real for four grammars, astro, perl, ruby and swift, at 1.2% to 7.7%.
- First contact pays far more: Swift's first document gains 13.8 M instructions against 1.9 M saved per pass.
- Giving classes only to those four grammars would still pay the build cost there.
- The prototype misses the bar set for it: 24 lexing rows rise above 1.000, and first contact rises in 44 languages.

## Open option: lazy rows

- Each rule could build a class's row only at the first position of that class.
- A retry is worth it only if building the class rows costs less than one pass's saving for the grammar.
  For Swift, the rows cost 13.8 M instructions at first contact today, and one pass saves 1.9 M.

## Not verified

- The weight is a static estimate of the tries a lookbehind saves.
  It was checked against measured try counts only for Swift and Ruby, where it picks the same lookbehinds.
- A narrower variant, with classes for four grammars only, was not built.
- Lazy rows were not built.
- bench:cold ran on a shared box and is noisy; only the direction is clear.

## Compute

- One 4x8 build box, about an hour for the prototype's two rounds, the size builds and the splits.
  The build box bills no cost per minute.

[A]: https://github.com/victor-software-house/nishiki-archive/commit/5a7f51183367ae8814f510e028103e327fa2333b
[B1]: https://github.com/victor-software-house/nishiki-archive/commit/a251d5efdecd7f5f569825624dd72e4638a05a1d
[prototype]: https://github.com/victor-software-house/nishiki-archive/commit/1f9d2a72f743d277b1d2230f8af4d9da6bb82b62
[lexing]: 2026-11-05-scan-prev-byte-classes/lexing.md
[first document]: 2026-11-05-scan-prev-byte-classes/first-document.md
[splits]: 2026-11-05-scan-prev-byte-classes/splits.md
[size]: 2026-11-05-scan-prev-byte-classes/size.md
[cold]: 2026-11-05-scan-prev-byte-classes/cold.md
