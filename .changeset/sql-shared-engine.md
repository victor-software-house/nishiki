---
nishiki: patch
---

SQL runs on the shared rule engine: lexing a SQL line allocates nothing once the document's states exist, and a pinned `Limits::depth` no longer counts SQL's root context, as in the other languages.
