---
"nishiki": patch
---

CSS opens a selector at a Unicode space that follows another, such as U+00A0 or U+3000, as Shiki does.
`CSS` and three U+00A0 give the last two U+00A0 `meta.selector.css`.
