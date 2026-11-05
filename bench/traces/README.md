# Delta traces

The deltas the stream benchmarks replay, as a model or a tool would send them.
[`../traces.ts`](../traces.ts) writes them, and `mise run check:traces` holds them to their sources.
The reasons are in design decision 10 of the `streaming` change under `openspec/changes/`.

| Folder | Holds | Source |
|:--|:--|:--|
| `reply/` | each reply-tier corpus file, in token-sized deltas | `bench/corpus/`, licences in [`../corpus.toml`](../corpus.toml) |
| `tool-write/` | the first medium-tier corpus file of 13 languages, in token-sized deltas | the same |
| `edits/` | six nishiki files from their first commit, one replacement per hunk of each later commit | nishiki's history, MIT OR Apache-2.0 |

## Deltas

A token-sized delta follows the pre-tokenizer pattern of byte-pair tokenizers.
A piece longer than six characters is cut into four-character pieces.
One delta averages about 3.1 characters.
No trace comes from a recorded model transcript.

## Format

Plain text, so the bench workspace reads it with the standard library alone.

- Each header line is a key, a space and a value: `kind`, `language`, `source`, `url`, `licence`, `sha256`.
  An edit trace adds `from` and `to`, the commits it replays.
- An append trace ends its header with `deltas`.
  Then come the byte lengths of the deltas, 32 to a line.
- An edit trace ends its header with `base <bytes>`.
  That many bytes of text follow, then a line feed.
- Each replacement follows as `replace <start> <end> <bytes>`, then that many bytes of new text and a line feed.
  Offsets count bytes in the text before that replacement.

## Commands

```sh
bun bench/traces.ts write   # write every trace again
mise run check:traces       # fail when a trace and its source disagree
```

`check:traces` writes the append traces again and compares the bytes.
It replays each edit trace and compares the result with the trace's `sha256`.
Writing the edit traces needs nishiki's history up to the commit `traces.ts` names, so the check never writes them.
