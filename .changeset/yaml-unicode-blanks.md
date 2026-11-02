---
nishiki: patch
---

Read the Unicode spaces as blanks in YAML where Shiki's grammar reads `\s`: after the colon of a key, after a dash or a question mark, between a directive's words and around properties, and not before a `#` or after a tag, where only a space or a tab counts.
