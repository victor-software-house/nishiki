---
nishiki: patch
---

Raise the default depth limit from 256 to 4,096 open contexts, so a long Ruby spec file keeps Shiki's scopes to its end: Shiki leaves one call open for each `call(<<~RUBY)`, whose `)` the here document takes, and has no depth limit.
