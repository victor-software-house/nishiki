# Design

## Context

nishiki has one crate, its oracle tools, tests, benches and an OpenSpec tree.
Each part grew its own documents, and they disagree on where things go.
A new contributor cannot tell which document states current truth.

## Goals

- One home for each kind of document, named by what it holds.
- A reader on GitHub can browse any top-level directory and learn what it holds from its README.
- A reader on docs.rs gets the API and each language's behaviour notes.
- Every document reads plainly.

## Non-goals

- Moving code between modules.
- Changing the benches: the `bench-methodology` change owns `bench/`.
- Rewriting history: moved files keep their Git history through `git mv`.

## Layout

```text
README.md            GitHub page
AGENTS.md            the guide for contributors and agents
CODING_STANDARDS.md  the numbered coding rules
openspec/            README.md, AGENTS.md
  specs/             what nishiki does now
  changes/           work in flight
  research/          research notes, YYYY-MM-DD-<topic>.md
src/                 README.md: the module map
oracle/              README.md, AGENTS.md: the Shiki oracle tools
tests/               README.md, AGENTS.md: test suites and fixtures
bench/               README.md, AGENTS.md
examples/            the harness binaries
mise-tasks/          README.md: what each task runs
nishiki-alloc-count/ the allocation-counting test helper
```

## Decisions

### OpenSpec holds intent

A spec says what nishiki does.
A change says what is being done and why.
A research note feeds a change, and the change's design cites it.

Research notes move from `docs/research/` to `openspec/research/`.
A note informs several changes, so it does not live inside one change.

The decisions that created nishiki stay in the planning repository, and `AGENTS.md` keeps linking them.
New decisions go in the design of the change they shape, as `docs/AGENTS.md` already requires.

### The prototype goes

`research/textmate/` was the prototype behind the 2026-11-02 note.
`src/textmate/` now does its job, written from scratch, and the crate took none of the prototype's code.
Keeping it costs a second Cargo workspace that nothing builds.
The note links the prototype at a pinned commit, so the code stays reachable.

### A README per browsable directory

A README says what a directory holds and where to start.
An `AGENTS.md` beside it holds the rules for working there, where the directory has rules.
The root guide links each one.

`check:layout` reads a list of directories and fails when one lacks a file it should have.

### README and crate docs

The README is the GitHub page: what the crate is, how to add it, one example, and links.
It does not copy the API docs.
It keeps the language table, because a reader picks a crate by its languages.

The crate-root docs become the docs.rs landing page.
Each language's behaviour notes, today under "Behaviour kept from Shiki", move to that language's `Language` variant, where docs.rs shows them.
The README's doctests move with the examples into the crate docs.

### Writing rules

`AGENTS.md` gets one section that every document follows:

- short sentences, one idea each, and Semantic Line Breaks;
- a list for a set, a table for a comparison, a heading per topic;
- plain words, and headers that name their content;
- no filler, no flourish, and no walls of prose.

`docs/AGENTS.md` rule 10 and the root guide's Documentation section fold into it.

## Risks

- Moved paths break links.
  Each move updates every link in the repository in the same commit, and `rg` proves no old path is left.
- The README's docs.rs source links point at files that move.
  The crate docs use intra-doc links instead.

## Migration

Each step is one pull request, in the order of `tasks.md`.
Open pull requests that touch a moved file rebase onto the move.
