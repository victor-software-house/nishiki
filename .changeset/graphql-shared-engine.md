---
nishiki: patch
---

GraphQL runs on the shared rule engine.
`Limits::depth` now counts only the contexts open above GraphQL's root, as in the other languages.
This changes the count for every depth limit, the default `Some(4096)` included, not only a pinned one.
Each interpolation's JavaScript spends the line's one work budget.
