---
nishiki: patch
---

Under `(?i)`, nishiki's TextMate engine applies case folds of more than one character, as Shiki does.
`ß` matches `ss`, `class` matches `claß`, and `ffi` matches `ﬃ`.
A class such as `[\w]` also matches the fold strings of its members.
