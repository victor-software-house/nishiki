---
nishiki: patch
---

A loaded grammar's pattern with an Oniguruma callout, such as `(*FAIL)`, never matches.
The regex parser refuses callouts, where it read one as a group of literal characters.
