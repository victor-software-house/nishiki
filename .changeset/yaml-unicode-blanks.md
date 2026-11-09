---
nishiki: patch
---

Read the Unicode spaces as blanks in YAML where Shiki's grammar reads `\s`.
That is after the colon of a key, after a dash or a question mark, between a directive's words, and around properties.
Before a `#` and after a tag, only a space or a tab counts.
