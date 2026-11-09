# Comparison with other highlighters

How fast nishiki colours code next to other highlighters, and how close each comes to Shiki's colours.
The reasons for each rule are in the [`bench-methodology` design](../../openspec/changes/archive/2026-11-04-bench-methodology/design.md), decision 13.

## Engines

| Engine | Version | How it runs |
|:--|:--|:--|
| nishiki | this tree | each line lexed, each token styled |
| syntect, with two-face's syntaxes | 5.3.0, 0.5.2 | Oniguruma, the backend bat uses; lines with their endings |
| tree-sitter-highlight | 0.27.1 | whole documents, each grammar crate's highlight query |
| giallo | 0.5.2 | on the bench box only, with the `giallo` feature |
| Shiki | 4.6.0 | in Bun, Oniguruma and JavaScript engines (`shiki.ts`) |
| @pierre/highlights | 0.4.0 | in Bun, its WebAssembly lexers (`shiki.ts`) |

giallo is EUPL-1.2, a copyleft licence.
It is a feature that is off by default, built only on the bench box, and never distributed.

@pierre/highlights is a timing and output comparison only.
nishiki never reads it as a grammar source (invariant 1).

## The rules

1. Every engine reads the same corpus files, and results are per file.
   An engine without a grammar for a language is left out of that language's rows.
2. Every engine colours with Shiki's github-dark.
   - syntect gets it as a theme built from Shiki's JSON rules.
   - tree-sitter's capture names map to the TextMate scope whose colour stands for them (`CAPTURES` in `src/engines.rs`).
   - Pierre takes Zed themes keyed by its token names.
     `compare --pierre-theme` writes one from `CAPTURES` and `PIERRE` in `src/engines.rs`.
     Pierre's fidelity depends on that map.
     A low figure can come from the map, not the engine.
3. Setup, the first document in a fresh process, and warm throughput are three figures, never one.
4. Every engine does the full work: Shiki's and Pierre's line limits are off, and the run stops if nishiki reports a limit reached.
5. Every Rust engine builds with the default release profile and the same rustc.
6. TypeScript's tree-sitter query is joined to JavaScript's, and C++'s to C's, because each extends its base.
7. Each row carries the engine's token count and its fidelity to Shiki.
   Fidelity is the share of non-whitespace bytes whose colour matches Shiki's.
8. The engine order rotates every round, five rounds at least, and the median counts.
9. Shiki and Pierre run in Bun.
   Their rows show what a user of each library gets.
   They do not show which algorithm is faster.

## Licences

Every comparator was read at its source before it entered this workspace.
None ships: `publish = false` and the crate's `include` both leave `bench/` out.

| Dependency | Version | Licence | Read at |
|:--|:--|:--|:--|
| syntect | 5.3.0 | MIT | crates.io manifest |
| two-face | 0.5.2 | MIT OR Apache-2.0 | crates.io manifest |
| two-face's bundled syntaxes, as a group | bat 0.26.1 | 50 MIT, 13 Apache-2.0, 3 BSD, 2 Unlicense, Sublime's Packages licence, WTFPL (GraphQL): all permissive | [acknowledgements][two-face-ack] |
| tree-sitter, tree-sitter-highlight | 0.27.1 | MIT | crates.io manifest |
| tree-sitter-bash | 0.25.1 | MIT | [LICENSE][ts-bash] |
| tree-sitter-c | 0.24.2 | MIT | [LICENSE][ts-c] |
| tree-sitter-c-sharp | 0.23.5 | MIT | [LICENSE][ts-c-sharp] |
| tree-sitter-cpp | 0.23.4 | MIT | [LICENSE][ts-cpp] |
| tree-sitter-css | 0.25.0 | MIT | [LICENSE][ts-css] |
| tree-sitter-go | 0.25.0 | MIT | [LICENSE][ts-go] |
| tree-sitter-html | 0.23.2 | MIT | [LICENSE][ts-html] |
| tree-sitter-java | 0.23.5 | MIT | [LICENSE][ts-java] |
| tree-sitter-javascript | 0.25.0 | MIT | [LICENSE][ts-javascript] |
| tree-sitter-json | 0.24.8 | MIT | [LICENSE][ts-json] |
| tree-sitter-python | 0.25.0 | MIT | [LICENSE][ts-python] |
| tree-sitter-ruby | 0.23.1 | MIT | [LICENSE][ts-ruby] |
| tree-sitter-rust | 0.24.2 | MIT | [LICENSE][ts-rust] |
| tree-sitter-typescript | 0.23.2 | MIT | [LICENSE][ts-typescript] |
| tree-sitter-yaml | 0.7.2 | MIT | [LICENSE][ts-yaml] |
| serde_json | 1.0.154 | MIT OR Apache-2.0 | crates.io manifest |
| giallo, behind the `giallo` feature | 0.5.2 | EUPL-1.2, copyleft | [Cargo.toml][giallo-cargo] |
| shiki, in Bun | 4.6.0 | MIT | already pinned by `oracle/generate.ts` |
| @shikijs/stream, in Bun, for the stream comparison only | 4.6.0 | MIT | [LICENSE][shiki-stream-licence] |
| @pierre/highlights, in Bun | 0.4.0 | Apache-2.0; its `NOTICE.md` names no licence for its grammars | [LICENSE.md][pierre-licence] |

Each grammar's licence was read in its repository at the commit its release tag names.
The two-face summary covers only the syntaxes two-face acknowledges.
A syntax it ships without a licence file would not appear there.

## The newest record

`mise run bench:compare-record` records a comparison as `results/namespace-8x16/<date>-<sha8>.json`.
It runs only on the reference host class, as `bench:record` does.
It runs every engine on the medium and large tiers, with an anchor at the start and the end.
`mise run bench:report` renders the record's page and the summary below.
`check:bench-report` fails when either differs from its JSON.

<!-- record summary: rendered by `mise run bench:report summary` -->

namespace-8x16, 2026-11-04, commit `855723c6`, tiers medium and large; [the full record](results/namespace-8x16/2026-11-04-855723c6.md).
These figures are indicative only.
No host class keeps two runs of one build within 2%, and this one moves by up to 11% ([probe](../../openspec/research/2026-11-04-bench-host-probe-criterion.md)).
Speed decisions rest on Callgrind instruction counts.

| Engine | Files | Warm MiB/s, median | First document ms, median | Fidelity, median |
|:--|--:|--:|--:|--:|
| nishiki | 62 | 22.9 | 4.78 | 100.00% |
| syntect | 60 | 2.4 | 18.76 | 83.32% |
| tree-sitter | 29 | 9.3 | 8.71 | 72.13% |
| giallo | 62 | 2.1 | 29.72 | 100.00% |
| shiki-oniguruma | 62 | 1.9 | 67.43 | 100.00% |
| shiki-javascript | 62 | 3.8 | 71.45 | 100.00% |
| pierre | 62 | 254.5 | 2.01 | 74.39% |

<!-- end of record summary -->

## Running

```sh
mise run bench:compare                      # medium tier, every language
mise run bench:compare -- --tier large rust # one language and tier
COMPARE_FEATURES=giallo mise run bench:compare
bun bench/compare/shiki.ts time             # Shiki's and Pierre's own figures
bun bench/compare/shiki.ts stream-reference # Shiki's colours for each trace's final text
bun bench/compare/shiki.ts stream           # Shiki and Pierre on the stream traces
```

`bench:compare` first writes Shiki's reference colours to `target/compare/shiki/`.
It also writes Pierre's theme to `target/compare/pierre-theme.json`.
`shiki.ts time` needs both.
Its rows carry tokens and fidelity as the harness's rows do.
The Oniguruma engine reads 100% fidelity, because it wrote the reference.

The comparison workspace builds C for Oniguruma and the tree-sitter grammars.
It is not part of `verify`.
The Bench job runs `bench:check` on a pull request labelled `bench`.
`bench:check` checks this workspace's formatting and runs clippy on it without giallo.
That keeps it building.
A recorded comparison runs on the reference host, under the rules in [`../AGENTS.md`](../AGENTS.md).

[shiki-stream-licence]: https://unpkg.com/@shikijs/stream@4.6.0/LICENSE
[pierre-licence]: https://unpkg.com/@pierre/highlights@0.4.0/LICENSE.md
[two-face-ack]: https://github.com/CosmicHorrorDev/two-face/blob/33f7e243b6174e8f0c63fd7698ec5736d715e355/generated/acknowledgements_full.md
[giallo-cargo]: https://github.com/getzola/giallo/blob/9136c2d78cd43321d2fbd845942ed07ff2e3f435/Cargo.toml
[ts-bash]: https://github.com/tree-sitter/tree-sitter-bash/blob/a06c2e4415e9bc0346c6b86d401879ffb44058f7/LICENSE
[ts-c]: https://github.com/tree-sitter/tree-sitter-c/blob/b780e47fc780ddc8da13afa35a3f4ed5c157823d/LICENSE
[ts-c-sharp]: https://github.com/tree-sitter/tree-sitter-c-sharp/blob/cac6d5fb595f5811a076336682d5d595ac1c9e85/LICENSE
[ts-cpp]: https://github.com/tree-sitter/tree-sitter-cpp/blob/f41e1a044c8a84ea9fa8577fdd2eab92ec96de02/LICENSE
[ts-css]: https://github.com/tree-sitter/tree-sitter-css/blob/dda5cfc5722c429eaba1c910ca32c2c0c5bb1a3f/LICENSE
[ts-go]: https://github.com/tree-sitter/tree-sitter-go/blob/1547678a9da59885853f5f5cc8a99cc203fa2e2c/LICENSE
[ts-html]: https://github.com/tree-sitter/tree-sitter-html/blob/5a5ca8551a179998360b4a4ca2c0f366a35acc03/LICENSE
[ts-java]: https://github.com/tree-sitter/tree-sitter-java/blob/94703d5a6bed02b98e438d7cad1136c01a60ba2c/LICENSE
[ts-javascript]: https://github.com/tree-sitter/tree-sitter-javascript/blob/44c892e0be055ac465d5eeddae6d3e194424e7de/LICENSE
[ts-json]: https://github.com/tree-sitter/tree-sitter-json/blob/ee35a6ebefcef0c5c416c0d1ccec7370cfca5a24/LICENSE
[ts-python]: https://github.com/tree-sitter/tree-sitter-python/blob/293fdc02038ee2bf0e2e206711b69c90ac0d413f/LICENSE
[ts-ruby]: https://github.com/tree-sitter/tree-sitter-ruby/blob/71bd32fb7607035768799732addba884a37a6210/LICENSE
[ts-rust]: https://github.com/tree-sitter/tree-sitter-rust/blob/77a3747266f4d621d0757825e6b11edcbf991ca5/LICENSE
[ts-typescript]: https://github.com/tree-sitter/tree-sitter-typescript/blob/f975a621f4e7f532fe322e13c4f79495e0a7b2e7/LICENSE
[ts-yaml]: https://github.com/tree-sitter-grammars/tree-sitter-yaml/blob/7708026449bed86239b1cd5bce6e3c34dbca6415/LICENSE
