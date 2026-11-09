# Tasks

## 1. Language detection

- [ ] 1.1 Write `design.md`: the evidence order, the public API, and how shell and Markdown hand a file name to an embedded language.
  → verify: the operator approves the design
- [ ] 1.2 File names and extensions → verify: every extension and file name in Shiki's language list names its language in a unit test
- [ ] 1.3 Shebangs, modelines and content → verify: a held-out set of extensionless files is named correctly, with the rate reported
- [ ] 1.4 Shell here-documents and file-printing commands.
  → verify: oracle fixtures of shell sessions highlight the written and printed lines as their file's language
