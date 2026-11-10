---
nishiki: patch
---

SQL runs on the shared rule engine.
Lexing a SQL line allocates nothing once the document's states exist.
A pinned `Limits::depth` leaves out SQL's root context, as in the other languages.
