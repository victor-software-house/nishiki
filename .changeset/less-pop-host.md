---
nishiki: patch
---

A Svelte or Vue `<style lang="less">` body that starts with `)` or `}` now ends there, as in Shiki: the rest of the body is plain text, and the element closes at `</style>`.
Before, the rest of the document was highlighted as Less.
