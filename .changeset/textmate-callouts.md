---
nishiki: patch
---

A loaded grammar's pattern with an Oniguruma callout, such as `(*FAIL)`, never matches.
It no longer matches the callout as a group of literal characters.
