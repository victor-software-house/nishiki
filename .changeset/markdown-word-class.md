---
nishiki: patch
---

Markdown reads `\w` and `\s` from the classes measured on Shiki, so a combining mark such as U+093C or U+0951 after an emphasis delimiter keeps it from closing, as in Shiki.
`Theme::style` no longer allocates for each scope of the stack it styles.
