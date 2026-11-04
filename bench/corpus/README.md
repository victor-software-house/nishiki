# Benchmark corpus

The files nishiki's wall-clock benchmarks and its comparison with other highlighters read.
The callgrind gate reads the oracle fixtures instead, because they reach more code paths.

[`../corpus.toml`](../corpus.toml) lists every file:

- its language and tier;
- its origin;
- its source, pinned to a commit;
- its licence, read at its source;
- its size and SHA-256.

## Tiers

| Tier | Size | Stands for |
|:--|:--|:--|
| small | 1 to 4 KiB | a snippet or a code block in a reply |
| medium | 16 to 64 KiB | a typical source file |
| large | 200 KiB to 1 MiB | a big real file, for the languages the comparison covers |
| reply | 4 to 40 KiB | a Markdown reply with code blocks in several languages |

CSS has no large file: every CSS file found in that range was generated or vendored.

## Rules

1. Every file is real code from a real project, byte for byte as fetched.
   None is generated, minified or repeated, except jQuery 2.1.4's `jquery.js`, which syntect's and giallo's benchmarks also use.
2. Only permissive licences: MIT, Apache-2.0, BSD, ISC, zlib, CC0, Unlicense and PSF.
3. `mise run check:corpus` fails when a file and its row disagree, a file has no row, or a licence is not on that list.
4. Adding, changing or removing a file raises `version` in `corpus.toml`.
   Figures from two versions are never compared.
