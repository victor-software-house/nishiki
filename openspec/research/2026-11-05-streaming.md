# Streaming: flicker, speculation, edits and first contact

What does a streaming document layer need to decide, on measurements?

1. How should the line still being written be shown, so colours flicker least?
2. When one large delta lands, does a speculative pass at the viewport pay?
3. After an edit, how far past it must lexing run before the states agree again?
4. What does the first line of a language cost today, in a fresh highlighter?

It feeds the design of the [`streaming` change][design].

## Method

- Questions 1 to 3 ran Shiki 4.6.0 in Bun, with its per-line limits off, through [`probe.ts`][probe].
  - Shiki's colours are nishiki's colours: the oracle and Identity tests hold them equal.
    So the answers hold for nishiki's output.
  - The probe counts bytes, lines and states, never time.
  - It ran locally on 2026-11-05, against main [68620d9e][main].
- Question 4 ran `mise run bench:latency` (small and medium tiers) and `mise run bench:cold` on main [68620d9e][main].
  It ran on a box, a `4x8` build microVM ([log][log]).
  The box is not the reference host class, so its times are indicative only (bench/AGENTS.md rule 1).

### Inputs

| Trace | Files | Source and licence |
|:--|:--|:--|
| Model reply | the three `reply`-tier Markdown files | [`bench/corpus.toml`][corpus], each read at its source |
| Tool-written file | the first medium-tier file of 13 languages | the same |
| Speculation | those 13, the first large-tier file of 12 of them, and the replies | the same |
| Edits | the history of six nishiki files, up to [68620d9e][main] | nishiki, MIT OR Apache-2.0 |

### Deltas

The probe cuts each text into token-sized deltas.

- It splits on the pre-tokenizer pattern that byte-pair tokenizers use.
- It cuts any piece longer than six characters into four-character pieces.
  That stands in for their subword merges.
- One delta is one piece: about 3.1 characters on these files.

No recorded transcript was used.
A real model stream sends deltas of one to several tokens, so these traces hold more renders than a real one.

## 1. Flicker

A render follows each delta.
A shown character holds a colour.
A held-back character shows in the default colour and counts as not yet coloured.
A flicker is a coloured character whose colour differs from its colour in the render before.

Four ways to show the line still being written:

| Mode | Shows |
|:--|:--|
| `as-ended` | the line lexed as if it ended there, as `@shikijs/stream` does |
| `hold-last` | the same, with the last token held back |
| `hold-last-2` | the same, with the last two tokens held back |
| `hold-line` | nothing coloured until the line ends, as Pierre's `StreamTokenizer` does |

Totals over all 16 files, 524,064 characters in 169,821 deltas ([rows][flicker]):

| Mode | Flickers per KiB | Characters held back per render |
|:--|--:|--:|
| `as-ended` | 119.68 | 0 |
| `hold-last` | 4.18 | 18.09 |
| `hold-last-2` | 2.59 | 20.81 |
| `hold-line` | 0 | 29.16 |

By trace:

| Trace | `as-ended` | `hold-last` | `hold-last-2` |
|:--|--:|--:|--:|
| Model reply, flickers per KiB | 135.89 | 7.58 | 1.42 |
| Tool-written file, flickers per KiB | 118.11 | 3.85 | 2.71 |

- Holding back the last token removes 96.5% of the flicker.
- It holds back 18 characters a render, against 29 for the whole line.
- The rest of the flicker sits in a few files.
  TypeScript keeps 17.67 per KiB, HTML 12.57 and C++ 7.81.
  There the grammar looks ahead within the line, so text later in the line recolours earlier tokens.
  A second held-back token helps HTML (0) and the replies, but barely TypeScript (15.55) or C++ (7.18).
- In Markdown prose and YAML, the last token is often the whole run of text, so `hold-last` already holds most of the line.

## 2. Speculation at the viewport

For viewports at a tenth, a quarter, a half, three quarters and nine tenths of each file:

- lex from the initial state at the viewport;
- count the lines until the end state equals the exact one, up to 2,000 or the end of the file;
- score the first 60 lines, one screen, as the share of non-blank characters coloured as the exact lexing colours them.

140 viewports over 28 files ([rows][speculation]):

| Outcome | Viewports |
|:--|--:|
| The screen is coloured exactly | 84 (60%) |
| Under 90% of the screen is right | 23 (16%) |
| Under 50% of the screen is right | 8 (6%) |
| The state agrees within one screen | 70 (50%) |
| The state still differs at the end of the file or after 2,000 lines | 22 (16%) |

- The worst screens sit inside long contexts: Python docstrings (2.7% to 11.8% in `pandas-generic.py`), a shell heredoc (10.4%), a Java comment block (23.1%).
- A speculative screen cannot tell it is right.
  In C++, Java and TypeScript the screen was often exact while the state stayed different for hundreds of lines, because of an open namespace or class block.
  So a speculative pass must be redone by the exact sweep in any case.

What the exact sweep costs instead, at the large-tier lexing throughput of [the last record][record]:

| Language | Lex MiB/s | Text before the viewport lexed in one 60 Hz frame (16.7 ms) |
|:--|--:|--:|
| cpp | 7.28 | 124 KiB |
| sql | 23.98 | 409 KiB |
| javascript | 25.71 | 439 KiB |
| go | 30.42 | 519 KiB |
| ruby | 32.56 | 556 KiB |
| c | 35.97 | 614 KiB |
| csharp | 38.03 | 649 KiB |
| typescript | 49.82 | 851 KiB |
| python | 76.22 | 1.3 MiB |
| shellscript | 115.31 | 1.9 MiB |
| rust | 175.16 | 2.9 MiB |
| json | 465.80 | 7.8 MiB |

Speculation could only pay above those sizes, and only for a viewport far from the top.
Every medium-tier file, 16 to 64 KiB, sits below the lowest of them.

## 3. Edits

The probe replays each commit's `-U0` diff, one hunk at a time, on the file as it stood ([rows][edits]).
After a hunk, it lexes from the hunk's first line through its new lines.
Then it lexes on until a line's end state equals the state cached before the edit for that text line.

| File | Hunks | Lines lexed past the hunk, max |
|:--|--:|--:|
| `src/highlighter.rs` | 40 | 0 |
| `src/limits.rs` | 38 | 0 |
| `oracle/generate.ts` | 23 | 0 |
| `oracle/fuzz.ts` | 4 | 0 |
| `AGENTS.md` | 73 | 2 |
| `mise-tasks/callgrind.sh` | 61 | 0 |

- 238 of 239 hunks stopped at their own last line.
- One `AGENTS.md` hunk, at the end of the file, lexed two lines more.

## 4. First contact today

From the [box log][log], main [68620d9e][main], medium tier, in µs:

| Figure | Median over languages | Worst |
|:--|--:|--:|
| Steady p50 per line | 0.41 | 2.98 (cmake) |
| Steady p99 per line | 1.90 | 18.51 (cmake) |
| First-contact max per line | 124.4 | 965.6 (shellscript) |

Fresh process, small tier, in µs:

| Figure | Median over files | Worst |
|:--|--:|--:|
| `Highlighter::new` | 14.0 | 14.9 (html-derivative) |
| First line | 106.7 | 412.8 (shellscript) |
| Whole document | 415.5 | 4,940.3 (shellscript) |

- No hand-written language reaches a millisecond on its first line any more, but shell: 1.75 ms on its small file, 0.97 ms on its medium one.
- The rule-engine languages spend 100 to 530 µs on their first line, building their candidate tables.
- The `textmate` tier (Assembly, HAML) was not measured: the bench builds without it.

## Not verified

- No nishiki code was timed on a delta trace; nishiki has no document layer yet.
- The deltas are generated, not recorded from a model.
- Speculation and edits compare Shiki's states.
  nishiki's interned states may agree on fewer or more lines, while its tokens stay equal.
- The edit histories are short and come from one repository.
- The large tier was not rerun for first contact.
  The [last record][record] read 27.7 ms for C's large file at `0a58343a`.
- Times come from a box outside the reference host class.

## Compute

- Box: one `4x8` build microVM on a build host, about 25 minutes, unbilled.
- Locally: Bun, about 20 minutes over three runs of the probe.

[design]: ../changes/streaming/design.md
[probe]: 2026-11-05-streaming/probe.ts
[log]: 2026-11-05-streaming/latency-cold-68620d9e.txt
[flicker]: 2026-11-05-streaming/flicker.tsv
[speculation]: 2026-11-05-streaming/speculation.tsv
[edits]: 2026-11-05-streaming/edits.tsv
[corpus]: https://github.com/victor-software-house/nishiki-archive/blob/68620d9ef4c7eb99a91c30f3a3d6788a2c4aaebf/bench/corpus.toml
[main]: https://github.com/victor-software-house/nishiki-archive/commit/68620d9ef4c7eb99a91c30f3a3d6788a2c4aaebf
[record]: https://github.com/victor-software-house/nishiki-archive/blob/68620d9ef4c7eb99a91c30f3a3d6788a2c4aaebf/bench/results/namespace-8x16/2026-11-04-0a58343a.md
