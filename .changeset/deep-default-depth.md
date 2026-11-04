---
nishiki: patch
---

Raise the default depth limit from 256 to 4,096 open contexts.
A long Ruby spec file now keeps Shiki's scopes to its end.
Shiki leaves one call open for each `call(<<~RUBY)`, because the here document takes its `)`.
Shiki has no depth limit.
