---
nishiki: patch
---

Under `(?i)`, nishiki's TextMate engine now matches a character as every character Unicode's simple case folds join it to, as Shiki does: `s` matches `ſ`, `k` matches the Kelvin sign, and `[^k]` refuses it.
