# Four shapes for CSS's emit_match after lowered capture names

## Question

[The earlier note][registers] found that CSS's `emit_match` runs 19,062 more instructions since [#519], from register allocation.
It left one shape untried: keeping the dynamic-name path out of `emit_match`'s hot loop.
This note asks whether any shape makes CSS's `emit_match` cheaper without moving cost to another grammar.

It informs the decision; it does not make it.

## Answer

1. No shape tried does.
2. The hot loop is the capture loop, not the span loop.
   It holds no code of `dynamic_path`.
   It shares one function with the span loop, where `dynamic_path` is inlined.
3. Each shape either costs CSS more, or moves cost to other grammars:

   | Shape | Change | css | scss | csharp | Rejected because |
   |:--|:--|--:|--:|--:|:--|
   | S1 | `emit_captures` out of line | 1.0097 | 1.0076 | 1.0058 | the call costs more than the registers save; 18 rows rise past 0.5% |
   | S2 | `emit_spans` out of line, behind an inline check for spans | 0.9970 | 0.9969 | 0.9951 | graphql 1.0080, asm 1.0079, make 1.0057 and c 1.0025, whose matches nearly all record spans |
   | S3 | only a span's dynamic name out of line | 0.9991 | 1.0025 | 1.0013 | scss, csharp and go 1.0045 rise from register allocation |
   | S4 | a `Grammar` constant compiles the dynamic-name branch out where no matcher records one | 1.0004 | 1.0002 | 0.9972 | CSS is unchanged, and tex 1.0039, php 1.0018, haskell 1.0015, matlab 1.0014 and java 1.0005 rise from code generation |

4. An open shape is noted under section 5.

## 1. What was run

| Field | Value |
|:--|:--|
| Host | a Boat `large` box: 8 vCPUs, 16 GB, x86-64 Linux |
| Toolchain | rustc 1.99.1, valgrind 3.27.1 from `mise.bench.toml` |
| Base | [`b7405d7e`][base] |
| When | 2026-11-05 |

- Each shape ran `mise run callgrind b7405d7e` on every language.
- The deciding rows were CSS, SCSS and C#.
- Rows that moved were split by function with `callgrind_annotate`.
- CSS's base binary ran under `valgrind --tool=callgrind --dump-instr=yes`.
  Its per-instruction costs were matched to `objdump` of the same binary.
- Each shape's change is beside this note as a `.diff` file.
  [`rows.md`](2026-11-05-emit-match-shapes/rows.md) holds every row, and [`splits.md`](2026-11-05-emit-match-shapes/splits.md) every split.

## 2. Where CSS's emit_match spends

- CSS's `emit_match` still runs 2,194,590 instructions at `b7405d7e`.
- `emit_captures` and `emit_spans` are both inlined into it.
- The block that runs most often, 7,880 times, is in the capture loop.
  It moves `%r8` to `%rbp` and back, and reloads a value from `0x68(%rsp)`.
- `dynamic_path` runs from the span loop, 1,287 times, for CSS's unit names.
- Three engines record spans with dynamic names: the CSS family through `CssLike`, C, and Make.
  C# records no spans at all.

## 3. The shapes

### S1: the capture loop out of line

- `emit_captures` is `#[inline(never)]`.
- CSS's capture loop costs 1,777,016 in its own function, against 1,605,012 that left `emit_match`: +175,960 net.
- SCSS and C# grow the same way.

### S2: the span loop out of line

- `emit_spans` is `#[inline(never)]`, and `emit_match` calls it only when the match recorded spans.
- CSS saves 88,055 in `emit_match` and spends 34,391 in `emit_spans`: −53,664 net.
- SCSS saves 105,478 net, and C# 290,878, since C# never calls `emit_spans`.
- graphql, asm, make and c record spans on nearly every match.
  Their span loop costs 11% to 33% more out of line: c 11%, graphql 16%, make 25% and asm 33%.
  c's `emit_spans` costs 4,341,412 against 3,921,384 that left `emit_match`.

### S3: the dynamic name out of line

- The span loop calls an `#[inline(never)]` wrapper of `dynamic_path`.
- CSS saves 14,433 in `emit_match`.
- SCSS's `emit_match` grows by 81,823, and C#'s by 65,625.
  C#'s growth is the same as under the earlier note's V1 and V5, though C# never builds a dynamic name.

### S4: the dynamic-name branch compiled out

- `Grammar::DYNAMIC_SPANS` is false by default and true for Css, Scss, C and Make.
  The span loop builds a dynamic name only where it is true, and `span_dyn` asserts it at build time.
- CSS, SCSS, C and Make keep today's code.
  Their rows move only in libc's `memcmp`, by 604 to 4,973 instructions, at equal calls.
- Most other rows fall, by up to 0.78% (graphql), and C#'s `emit_match` saves 152,757.
- tex, matlab and java grow in `emit_match`: +14,202, +18,844 and +30,209.
  php and haskell move `dynamic_path` into an out-of-line `path_one`: php +40,323 net, haskell +15,650.

## 4. Reading

- CSS's cost comes from sharing one function between the capture loop and the span loop.
- Splitting them helps the grammars whose matches seldom record spans, and hurts those whose matches nearly always do.
- Compiling the dynamic-name branch out changes the code of every grammar without it, and register allocation moves in five of them.

## 5. Open shape

- S2 applied only where matches seldom record spans: CSS and SCSS.
  A grammar constant would select it, so every other grammar keeps its code unchanged.
- CSS and SCSS would take S2's rows, 0.9970 and 0.9969.
- Nobody measured it.
  The constant would describe how often a grammar's matchers record spans, which nothing checks at build time.

## 6. Not verified

- The open shape was not built.
- Reruns of each binary were not made; the conclusions rest on per-function counts that move by thousands of instructions.
- Why LLVM spends the registers was not traced in its passes.
- Wall-clock time was not measured.
- The shapes' branches were not pushed; their changes survive as the `.diff` files beside this note.

## 7. Compute

One Boat `large` box, about 90 minutes for this work.
Its cost was not itemized.

[#519]: https://github.com/victor-software-house/nishiki-archive/pull/519
[registers]: 2026-11-05-emit-match-registers.md
[base]: https://github.com/victor-software-house/nishiki-archive/commit/b7405d7edc9d99060501cf8d44b63557e1c49480
