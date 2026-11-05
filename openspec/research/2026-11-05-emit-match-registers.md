# CSS's emit_match after lowered capture names

## Question

[#519] let a grammar lower the text of a name built from a capture group.
Its body recorded a known cost: CSS's `Lexer<Css>::emit_match` ran 19,062 more instructions at one codegen unit, with the cause not found.
This note asks where that cost comes from, and whether moving code out of `emit_match` removes it.

It informs the decision to accept the cost; it does not make it.

## Answer

1. The cost is register allocation, not new work.
   CSS builds no name from a capture group, so it never runs the code [#519] added.
2. `dynamic_path` is inlined into `emit_match`.
   [#519] made it larger, and `emit_match`'s hot loop then shuffles one more register and reloads one value from the stack.
3. Four shapes moved code out of `emit_match`.
   None brings CSS back without costing another row more:
   - moving the capture-group arm, or the text-building match, out of line leaves CSS where it is or makes it worse;
   - moving all of `dynamic_path` out of line brings CSS below its cost before [#519], but SCSS's and C#'s `emit_match` grow by more than CSS saves.
4. C# builds no dynamic names either.
   Its `emit_match` still changes when `dynamic_path` leaves it, so the allocation of `emit_match` moves with any change to `dynamic_path`'s size or placement.

## 1. What was run

| Field | Value |
|:--|:--|
| Host | a Boat `large` box: 8 vCPUs, 15 GB, x86-64 Linux |
| Toolchain | rustc 1.99.1, valgrind 3.27.1 from `mise.bench.toml` |
| When | 2026-11-05 |

- The cause comes from [#519]'s merge, `ae1337dc`, counted against its parent, `9ad90e13`, with `mise run callgrind` on CSS, SCSS and PHP.
- Each side then ran once more under `valgrind --tool=callgrind --dump-instr=yes` on CSS's fixtures.
  The cost of each instruction of `emit_match` was summed and matched to `objdump` of the same binary.
- The four shapes each ran `mise run callgrind` against main at `5efc1b4b`, on every language, then the same per-function split.

## 2. The cost before any shape

At `ae1337dc` against its parent:

| Row | Head over base |
|:--|--:|
| css | 1.0004 |
| scss | 1.0001 |
| php | 0.9999 |

CSS's `emit_match` grows from 2,175,528 to 2,194,590 instructions: +19,062, as [#519] recorded.
No new function runs.

Each instruction costs one count each time it runs, so the per-instruction costs give each block's run count.
The gain sits in blocks that run often:

- the entry and exit, run 14,028 times, gain one move;
- the loop over a match's captures, run 7,880 times, gains a `mov %r8,%rbp` shuffle and a reload of a value kept on the stack;
- blocks run 2,564 times gain five instructions, and others lose a few.

## 3. The four shapes

| Shape | Change | CSS's `emit_match` | Other rows |
|:--|:--|--:|:--|
| V2 | the capture-group arm of `dynamic_path` moves into a helper that is not inlined | unchanged | every row at most 1.0006, but jinja 1.0012 |
| V4 | the match that builds a name's text moves into a helper that is not inlined | +11,366 | css 1.0008, graphql 1.0013 |
| V1 | `dynamic_path` is `#[inline(never)]` | −23,695 | scss 1.0020, csharp 1.0010 |
| V5 | `dynamic_path` is `#[cold]` and `#[inline(never)]` | −24,273 | scss 1.0016, go 1.0014, cmake 1.0012 |

- Under V1, SCSS's `emit_match` grows by 106,149 instructions and C#'s by 65,625.
- Under V5, SCSS's grows by 75,199 and C#'s by 65,625.
- [`rows.md`](2026-11-05-emit-match-registers/rows.md) holds every row of every shape.
  The `.diff` files beside it hold each shape's change to `src/rules/lexer.rs`.

## 4. Not verified

- Five reruns of each binary were not run.
  Rows within 0.1% of 1.0000 move by that much from run to run, as hashing makes callgrind counts vary.
  The conclusions rest on the per-function counts, which move by tens of thousands of instructions.
- Why LLVM spends the extra register was not traced in its passes.
- A shape that keeps the dynamic-name path out of `emit_match`'s hot loop altogether, such as resolving a span's dynamic name after the capture loop, was not tried.
- Wall-clock time was not measured.
- The shapes' branches were deleted after this note; their changes survive only as the `.diff` files here.

## 5. Compute

One Boat `large` box, about two hours for this work, shared with no other run.
Its cost was not itemized.

[#519]: https://github.com/victor-software-house/nishiki-archive/pull/519
