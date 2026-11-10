# Oracle coverage gate checks

What did tasks 2.1 and 2.2 of the `oracle-coverage` change check?

The checks ran on 2026-11-04 (UTC).
They were recorded under those tasks, in the change's [tasks file at `c9dcc3b1`][tasks-source].
This note holds them in full, so that each task keeps a one-line note within its word budget.
The results and their wording are unchanged.

## 2.1 Waivers

- A waiver on Swift's exercised `#comments.patterns[5].begin` failed the run with "is exercised by a fixture: remove its waiver".
- A waiver on the unexercised `#comments.patterns[2].begin` failed without a `probe`, since Swift has probe support.
  It passed with one, the report reading `waived:` and the reason.
- A waiver on a path the grammar lacks failed with "names no unit the report counts".

## 2.2 The coverage check

Locally:

- the check passed on the committed report;
- with a line added to the report, it failed and printed the difference;
- with `rust` listed complete, it failed and named Rust's unexercised paths.

The first run on [PR 252][pr252] failed.
`fuzz.ts` read the fixtures in `readdirSync` order, so CI's Linux runner drew another fuzz corpus.
With the fixtures sorted, [the run on `c4ca16d9`][run-252] passed.

## Not verified

- Nobody ran these checks again for this note.
- They check the coverage report and its gate, not scopes, so they name no Shiki version.

## Compute

The task notes recorded no machine minutes or cost.
Writing this note ran nothing.

[tasks-source]: https://github.com/victor-software-house/nishiki-archive/blob/c9dcc3b1754dc1ad7f221e5a7eda1fcdea15ad49/openspec/changes/oracle-coverage/tasks.md
[pr252]: https://github.com/victor-software-house/nishiki-archive/pull/252
[run-252]: https://github.com/victor-software-house/nishiki-archive/actions/runs/41756482197
