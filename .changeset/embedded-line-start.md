---
"nishiki": patch
---

Markdown and TOML that a Vue block embeds in a list item's or a quote's fence start after the line's start, as in Shiki.
Their patterns that need the line's start match nowhere there.
So an HTML block in such Markdown ends with its first line, and a TOML table header there is `invalid.illegal`.
