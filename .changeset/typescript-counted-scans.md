---
nishiki: patch
---

Match Shiki in TypeScript and JavaScript on letters and blanks beyond ASCII: names use Oniguruma's `[[:alpha:]]` and `[[:alnum:]]`, `\b` its `\w`, and a Unicode blank counts as `\s` wherever the grammar tests for one. Scans ahead in doc comments, JSX names and block comments count against the work limit, and a doc comment full of `[` is lexed in linear time.
