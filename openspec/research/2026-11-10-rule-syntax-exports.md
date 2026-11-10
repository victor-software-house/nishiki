# Why a build with the cfg calls the arena through the GOT

Can a code shape on stable rustc keep nishiki's direct calls to `Arena::child` and `Arena::push` in a build with `--cfg nishiki_unstable`?

In that build, `impl<G: Grammar> Sealed for RuleSyntax<G>` compiles, and nishiki exports what its body reaches ([design section 2.1][design]).
The tree read here is [9ffce154][base], the head of [#727][pr727], whose tree is main at efc098d8.

## Answer

No shape tested reaches parity on stable rustc 1.99.1.

- Shapes a, b and c leave every arena call going through the GOT in the build with the cfg, or change the build without it.
- Shapes d and d3 bring the direct calls back.
  They leave Callgrind rows a little above 1.000, and they add six items to the public `Grammar` trait.

The cause is the relocation rustc emits for an exported function, which the linker cannot turn into a direct call.
Two nightly flags change it, and stable rustc has neither.

## 1. The relocation

- nishiki's object file holds 615 `R_X86_64_GOTPCREL` relocations to `Arena::child`, and none of the relaxable `R_X86_64_REX_GOTPCRELX` form.
  The linker, LLD 23.1.1, relaxes only the `X` forms.
  The symbol is not in the binary's dynamic table, so the GOT entry only adds a load.
- rustc 1.99.1's default target options set `relax_elf_relocations: false` ([`rustc_target/src/spec/mod.rs`, line 2958][spec]).
- On nightly-2026-11-01, with the cfg, the control makes 0 direct calls to `child` or `push`, with 26,306 GOT calls.
  - `-Zrelax-elf-relocations=yes` cuts the GOT calls to 2,673, and 23,633 calls become direct `addr32 call`s.
  - `-Zdefault-visibility=hidden` gives 608 direct calls to `child` and 250 to `push`, and 17,193 GOT calls.
- Stable 1.99.1 has neither flag: `rustc -C help` lists none.
  No stable attribute sets one symbol's visibility.

## 2. What the generic impl exports

The build with the cfg loses 1,270 direct calls, to 59 functions:

| Function | Direct calls without the cfg |
|:--|--:|
| `Arena::child` | 550 |
| `Arena::push` | 196 |
| `Arena::intern` | 93 |
| `Arena::child_or_insert` | 58 |
| `Arena::push_or_insert` | 54 |
| `rules::lexer::push_lowered` | 11 |
| `Arena::new` | 1 |
| `rules::lexer::Lexer<G>::path_miss`, in 52 grammars | 307 |

In the build with the cfg, each count is 0.
[`exports.tsv`][exports] lists all 59 functions, one row for each grammar's `path_miss`.

## 3. The shapes

The prototype is the branch `research/rule-syntax-exports`, with one commit per shape.
Each commit's tree is 9ffce154 with that shape alone, and each link below compares it with 9ffce154.
The branch is pushed with no pull request, so no CI runs.
Its code is written from scratch for the prototype.

Counts are from the `lex` example's disassembly at one codegen unit.
Rows are Callgrind's lexing pass at one codegen unit, with valgrind 3.27.1, against 9ffce154 built without the cfg.

| Shape | What changes | `child`/`push` direct calls, with the cfg | Without the cfg |
|:--|:--|:--|:--|
| Control | Nothing | 0/0, with 26,451 GOT calls | 550/196, with 25,257 GOT calls |
| [a][a] | `#[inline]` on `child` and `push` | 0/0, 26,177 GOT calls | 0/0, and the gate fails |
| [b][b] | The generic impl forwards to inherent generic methods | 0/0, 26,451 GOT calls | Not recorded |
| [c][c] | An `#[inline]` wrapper over generic out-of-line copies, so each crate builds its own | 0/0, 26,452 GOT calls | 550/196 |
| [d][d] | Six required `Grammar` items, `path_miss` called through `G` | 550/196, 25,258 GOT calls | 550/196 |
| [d3][d3] | The same, with `path_miss` called directly | 550/196 | 550/196 |

### 3.1 Shape a

LLVM inlines both functions at every call site, so the build without the cfg changes too.
34 of 73 rows rise above 1.000 ([`a-lex.tsv`][a-lex]).
The highest are JSON 1.0509, JSON Lines 1.0187, JSONC 1.0162, HCL 1.0135, Rust 1.0123 and INI 1.0111.

### 3.2 Shape b

rustc walks the inherent generic methods the impl calls, and exports what they reach.
Nothing changes.

### 3.3 Shape c

The build with the cfg still calls the exported entry.
`-Zshare-generics=no` on nightly also left 0 direct calls.

### 3.4 Shape d

`Grammar` gains six required items with no default body: `child`, `push`, `child_or_insert`, `push_or_insert`, `intern` and `push_lowered`.
The engine calls `G::child(arena, …)`, and each of nishiki's 56 grammar impls forwards to the arena.
The per-grammar wrappers inline away.

- Without the cfg, 46 of 73 rows rise above 1.000, the highest PHP at 1.0008 ([`d-lex.tsv`][d-lex]).
- With the cfg, 44 of 73 rows rise above 1.000, the highest PHP at 1.0007 ([`d-cfg-against-base.tsv`][d-cfg]).
- PHP's split per function puts the rise in `Lexer<Php>::path_miss`, 138,616 → 153,177 instructions.
  Its disassembly spills a register in the loop once the call goes through the trait.

### 3.5 Shape d3

Shape d with `path_miss` called directly, so `path_miss` stays exported with the cfg.

- Without the cfg, 8 of 73 rows rise above 1.000 ([`d3-lex.tsv`][d3-lex]).
  They are ERB 1.0004, shell 1.0003, Astro, Blade and Elixir 1.0002, and HAML, HTML and PHP 1.0001.
- With the cfg, 34 of 73 rows rise above 1.000, the highest Swift, PHP and Objective-C at 1.0006 ([`d3-cfg-against-base.tsv`][d3-cfg]).

Both d shapes pass the 0.5% gate, and neither keeps every row at or below 1.000.
The rest is register allocation in the engine's walk, which needs a split per function for each row.

## 4. Upstream

- Default symbol visibility has a tracking issue, [rust-lang/rust#131090][rust131090], open on 2026-11-10.
  [rust-lang/rust#105518][rust105518] proposes protected visibility by default on ELF, also open.
- Relaxed relocations have no tracking issue.
  A search of rust-lang/rust issues for `relax-elf-relocations`, `relax_elf_relocations` and `GOTPCRELX` on 2026-11-10 found none.
  - [rust-lang/rust#106511][rust106511] made `relax_elf_relocations` the default on 2023-08-24.
  - [rust-lang/rust#115267][rust115267] reverted it on 2023-08-28.
    Its reasons are link failures with the binutils that `cross` uses ([rust-lang/rust#115239][rust115239]), and miscompilations with the mold linker.
  - [rust-lang/rust#163951][rust163951] documents the option as x86-only, merged 2026-10-08.

## Not verified

- Shape b's counts without the cfg were not recorded.
- Shapes b and c ran no Callgrind rows; their call counts ruled them out.
- The nightly flags ran no Callgrind rows.
- The relocation counts are for `Arena::child` alone.
- Only x86-64 Linux with LLD was measured.

## Compute

- Build boxes of 4 and 8 vCPUs, about two hours in all, with no metered cost.
- This Mac: reading code, disassembly and Callgrind output, and the upstream search, about an hour.

[design]: ../changes/unstable-rule-engine/design.md
[base]: https://github.com/victor-software-house/nishiki/tree/9ffce1544671a9b41add3422cc863b40404b53c8
[pr727]: https://github.com/victor-software-house/nishiki-archive/pull/727
[spec]: https://github.com/rust-lang/rust/blob/58cf8f9c35b355199727627d07ae34a4e2846d67/compiler/rustc_target/src/spec/mod.rs#L2958
[exports]: 2026-11-10-rule-syntax-exports/exports.tsv
[a-lex]: 2026-11-10-rule-syntax-exports/a-lex.tsv
[d-lex]: 2026-11-10-rule-syntax-exports/d-lex.tsv
[d-cfg]: 2026-11-10-rule-syntax-exports/d-cfg-against-base.tsv
[d3-lex]: 2026-11-10-rule-syntax-exports/d3-lex.tsv
[d3-cfg]: 2026-11-10-rule-syntax-exports/d3-cfg-against-base.tsv
[a]: https://github.com/victor-software-house/nishiki-archive/compare/9ffce1544671a9b41add3422cc863b40404b53c8...ba2dc32ac9d3a32acc0b9dcd5b6fe316494d4479
[b]: https://github.com/victor-software-house/nishiki-archive/compare/9ffce1544671a9b41add3422cc863b40404b53c8...97f58c9390d6e96bd747b6e19b1d9bb072058bc2
[c]: https://github.com/victor-software-house/nishiki-archive/compare/9ffce1544671a9b41add3422cc863b40404b53c8...1a3e8540d8be78590a1ab1be6011a59a66cef368
[d]: https://github.com/victor-software-house/nishiki-archive/compare/9ffce1544671a9b41add3422cc863b40404b53c8...f7a9e6f16066c7f98cac38494a9c1f6a6a15c54e
[d3]: https://github.com/victor-software-house/nishiki-archive/compare/9ffce1544671a9b41add3422cc863b40404b53c8...7cfc5b23c730c60879a63280ddf776d2d6e8dad3
[rust105518]: https://github.com/rust-lang/rust/issues/105518
[rust106511]: https://github.com/rust-lang/rust/pull/106511
[rust115239]: https://github.com/rust-lang/rust/issues/115239
[rust115267]: https://github.com/rust-lang/rust/pull/115267
[rust131090]: https://github.com/rust-lang/rust/issues/131090
[rust163951]: https://github.com/rust-lang/rust/pull/163951
