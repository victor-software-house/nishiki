---
nishiki: minor
---

One `Highlighter` for every language, chosen by `Language`, with configurable per-line `Limits` and a `LineReport` per line; `State` is now an interned handle, so nesting has no fixed ceiling, and `Highlighter::scopes` lists a stack innermost first.
