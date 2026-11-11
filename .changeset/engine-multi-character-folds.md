---
nishiki: patch
---

Under `(?i)`, nishiki's TextMate engine folds a character into several, as Shiki does.
`ß` matches `ss`, `class` matches `claß`, and `ffi` matches `ﬃ`.
A class such as `[\w]` also matches the fold strings of its members.
