# openspec

Rules for working in `openspec/`, beside the root guide.
The writing rules are in the root guide's Documentation section.

## Specs and changes

1. A spec in `specs/` states what nishiki does now, and only that.
2. A change in `changes/` states what is being done and why, in a proposal, a design and tasks.
3. A task is checked off only when its verify clause passed.
   A dated note under it records what ran, the result, and the linked pull request or run.
4. A finished change is archived with `mise x github:Fission-AI/OpenSpec@1.16.0 -- openspec archive <change> -y`, the version `mise-tasks/specs.sh` pins, in the pull request that finishes its last task.
5. `mise run specs` validates every change and spec, strictly.
6. Every reference is a clickable, reference-style link.
   A link to code or a file is pinned to a commit.

## Research notes

1. A note is `research/YYYY-MM-DD-<topic>.md`, dated the day it was written.
   It opens with the question it answers.
2. Its captures, tables and raw outputs sit in `research/YYYY-MM-DD-<topic>/`, beside it.
3. A note is frozen at the commits it names.
   A later finding is a new note that cites the old one.
4. A claim cites its source: a file and line at a pinned commit, a run, or a page read on a stated date.
   A claim nobody checked is marked as unchecked.
   So is one that comes only from reading code, with no run behind it.
5. Shiki is the reference.
   A note names the Shiki version and the grammar commit it compared against, with Shiki's per-line limits off.
6. Inputs are pinned to commits, and their licences are read at the source before use.
   Held-out inputs stay unseen until the development inputs pass.
   The note records the held-out score before any fix.
7. Prototype code stays on a pushed branch, and the note links it at a pinned commit.
   A prototype is written from scratch, as the crate is.
   The crate takes none of its code until a decision allows it.
8. A note ends with what it did not verify, and with its compute: each machine, its minutes and its cost.
9. A note informs a decision; it does not make one.
   The decision goes in the design of the change it shapes, and cites the note.
10. No local paths, chat transcripts or private details.
