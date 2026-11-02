# docs

Rules for nishiki's research record, beside the root guide's invariants.

1. A research note is `research/YYYY-MM-DD-<topic>.md`, dated the day it was written, and opens with the question it answers.
   Its captures, tables and raw outputs sit in `research/YYYY-MM-DD-<topic>/`, beside it.
2. A note is frozen at the commits it names: a later finding is a new note that cites the old one, not an edit.
3. A claim cites its source: a file and line at a pinned commit, a run, or a page read on a stated date.
   A claim nobody checked says so, and so does one that comes only from reading code with no run behind it.
4. Shiki is the reference: a note names the Shiki version and the grammar commit it compared against, with Shiki's per-line limits off.
5. Inputs are pinned to commits and their licences read at the source before use.
   Held-out inputs stay unseen until the development inputs pass; the note records the held-out score before any fix.
6. Prototype code lives in the repository, in its own workspace under `research/<topic>/` or on a pushed branch the note links, never in a scratch directory: a prototype that only a note describes cannot be rerun or reused.
   A prototype is written from scratch, as the crate is, and states that the crate takes none of its code until a decision says so.
7. A note ends with what it did not verify and with its compute: each machine, its minutes, and its cost.
8. A note informs a decision; it does not make one.
   A decision is recorded in the design of the OpenSpec change it shapes, and cites the note.
9. No local paths, chat transcripts or private details: the crate's source is public.
10. Prose follows Semantic Line Breaks: one sentence a line.
