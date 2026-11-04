# Proposal

## Why

The repository grew without a plan.
Research sits in two places, `docs/research/` and `research/textmate/`, and `docs/AGENTS.md` names a third, `research/YYYY-MM-DD-<topic>.md`.
Six OpenSpec changes are open, and one is complete but never archived, so `openspec/specs/` does not exist and nothing states what nishiki does today.
The README is 539 lines, and it describes the API that docs.rs should carry.

## What Changes

- OpenSpec becomes the one home for intent:
  - `openspec/specs/` for what nishiki does now;
  - `openspec/changes/` for work in flight;
  - `openspec/research/` for research notes.
- Finished changes are archived into `openspec/specs/`.
- `docs/` is removed. Its rules for research notes move to `openspec/AGENTS.md`.
- `research/textmate/`, the prototype that `src/textmate/` replaced, is deleted. Git keeps it, and its note links a pinned commit.
- The directories a reader browses on GitHub get a `README.md` that says what is there, and an `AGENTS.md` beside it where the directory has rules:
  - `openspec/`, `oracle/`, `tests/` and `bench/` get both;
  - `src/` and `mise-tasks/` get a README only.
- A check task fails when a listed directory lacks its README or guide.
- The README becomes the GitHub page: what nishiki is, how to add it, one example, the language list, and links.
  The crate docs on docs.rs carry the API and each language's behaviour notes.
- The writing rules for every document move into `AGENTS.md`, and every guide and README is rewritten to them.

## Capabilities

### New Capabilities

None. This change moves documents and adds a repository check; the crate's behaviour does not change.

### Modified Capabilities

None.

## Impact

- Paths: `docs/` and `research/` disappear; `openspec/research/` and `openspec/specs/` appear.
- The README shrinks; its doctests move to the crate docs, which docs.rs renders.
- Links into moved files are updated, in the repository and in open pull requests' bodies where they point at moved paths.
- No code change and no changeset, except the crate docs, which users see on docs.rs and which get a changeset.
