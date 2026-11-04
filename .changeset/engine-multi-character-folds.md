---
nishiki: patch
---

Under `(?i)`, nishiki's TextMate engine now applies case folds of more than one character, as Shiki does: `ß` matches `ss`, `class` matches `claß`, `ffi` matches `ﬃ`, and a class such as `[\w]` also matches the fold strings of its members.
