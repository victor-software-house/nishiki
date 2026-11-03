# What a TextMate-engine language costs in binary size

## Question

nishiki's `TextMate` engine runs a language from its grammar, compiled into static tables behind a `lang-*` feature in the opt-in `textmate` tier.
How much does the engine cost once, and how much does each grammar add after it?
Is one feature per grammar worth keeping, rather than one feature for the whole tier?

This note informs the decision.
It does not make it.

## Answer

1. The engine costs about 1.67 MB once, in a release build of the bench example on x86-64 Linux.
   That is the regex machine, the tokenizer, the scanners and `regex-automata`.
2. Each grammar then adds 30 to 98 KB: Zig 30 KB, PowerShell 66 KB, Scala 98 KB.
   That is 1.1 to 1.3 times the size of the grammar's generated `grammar.rs` source.
3. The first grammar pays for the engine: a build with Rust alone grows by 1.69 to 1.76 MB when one engine language is added, against 30 to 98 KB for each language after it.
4. So a feature per grammar saves little next to the engine.
   With three languages, turning one of them off saves 1.6 to 5.2 percent of what the tier adds.
   A feature per grammar still pays for itself as the tier grows, since the grammars add up: ten grammars of this size would add 0.3 to 1 MB.

## 1. What was run

One build of the bench example (`cargo build --release --locked --example bench --no-default-features --features <set>`) for each feature set below.
Each build used its own feature set in one target directory; the binary's size is its file size, unstripped, as Cargo's default release profile leaves it.

| Field | Value |
|:--|:--|
| Commit | `e774a9f62140d51b4a2682d0fcaa85d2c85b8110`, kept reachable by the tag `bench/e774a9f62140d51b4a2682d0fcaa85d2c85b8110` |
| Host | instabox `universal` box, x86-64 Debian |
| Toolchain | rustc 1.99.1 (58cf8f9c3 2026-10-13), the default release profile |
| When | 2026-11-03, 13:32:57 to 13:34:27 UTC |

The measured commit has PowerShell, Scala and Zig on the engine, before the engine languages moved to one row shape in `languages!`; that move only changes where the three entry points call the engine, and was not measured.

## 2. Sizes

| Features | Bytes |
|:--|--:|
| `lang-rust` | 705,736 |
| `lang-rust,lang-powershell` | 2,431,992 |
| `lang-rust,lang-scala` | 2,462,984 |
| `lang-rust,lang-zig` | 2,395,128 |
| `lang-rust,lang-powershell,lang-scala` | 2,539,976 |
| `lang-rust,lang-powershell,lang-zig` | 2,472,400 |
| `lang-rust,lang-scala,lang-zig` | 2,503,624 |
| `lang-rust,textmate` (all three) | 2,570,056 |
| `all` | 5,731,168 |
| `all,textmate` | 7,558,280 |

## 3. What each part costs

Each grammar's cost is the full tier less the build without it.

| Grammar | Added bytes | `grammar.rs` bytes | Rules | Patterns |
|:--|--:|--:|--:|--:|
| PowerShell | 66,432 | 59,199 | 86 | 88 |
| Scala | 97,656 | 83,406 | 156 | 113 |
| Zig | 30,080 | 23,793 | 64 | 51 |

The engine is the rest: 2,570,056 − 705,736 − (66,432 + 97,656 + 30,080) = 1,670,152 bytes.
On top of `all`, the tier adds 1,827,112 bytes, close to the 1,864,320 it adds on top of `lang-rust`.

## Not verified

1. Whether the move to one row shape changes any size.
2. One build per feature set, on one host and one target; another target, `opt-level`, LTO or stripping changes every figure.
3. Sizes of the bench example, not of a downstream binary, which links other code and may drop more.
4. Three grammars only. The rest of the planned grammars may not scale with their source size the same way.

## Compute

One instabox box for 90 seconds, at no cost beyond the plan.
