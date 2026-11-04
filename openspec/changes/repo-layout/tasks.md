# Tasks

## 1. OpenSpec as the home for intent

- [ ] 1.1 Verify the open tasks of `shiki-oracle`, `themes` and `line-limits` against main, record the evidence, and archive every change whose tasks all pass → verify: `openspec/specs/` holds their capabilities and `mise run specs` passes
- [ ] 1.2 Archive `textmate-engine` → verify: `openspec list` no longer shows it
- [ ] 1.3 Move `docs/research/` to `openspec/research/` with `git mv`, move the research rules from `docs/AGENTS.md` to `openspec/AGENTS.md`, add `openspec/README.md`, and delete `docs/` → verify: `rg 'docs/research|docs/AGENTS'` finds nothing outside archived changes
- [ ] 1.4 Delete `research/textmate/` and its `exclude` entry in `Cargo.toml`; point the note at the prototype's last commit → verify: `rg 'research/textmate'` finds only that pinned link

## 2. A README per browsable directory

- [ ] 2.1 Add `README.md` and `AGENTS.md` to `oracle/` and `tests/`, and a `README.md` to `src/` and `mise-tasks/`; move the oracle and test rules from the root guide into them → verify: the root guide links each, and no rule is stated twice
- [ ] 2.2 Add `check:layout`, run by `verify`, which fails when a listed directory lacks its README or guide → verify: removing `oracle/README.md` locally makes it fail

## 3. Writing rules

- [ ] 3.1 Put the writing rules in one section of `AGENTS.md` and fold in the old ones → verify: no other guide restates them
- [ ] 3.2 Rewrite every guide and README to the rules → verify: no table header names a vague concept, no paragraph runs past four lines, and `mise run lint:md` passes

## 4. README and crate docs

- [ ] 4.1 Make the crate-root docs the docs.rs landing page, with a quick start and the cargo features → verify: the docs job builds them as docs.rs does
- [ ] 4.2 Move each language's behaviour notes onto its `Language` variant → verify: every "Behaviour kept from Shiki" bullet appears on docs.rs and nowhere else
- [ ] 4.3 Cut the README to the GitHub page: what nishiki is, how to add it, one example, the language table and links → verify: the README holds no API reference and its example runs as a doctest
