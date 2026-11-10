---
nishiki: patch
---

Markdown reads `\w` and `\s` from the classes measured on Shiki.
A combining mark such as U+093C or U+0951 after an emphasis delimiter keeps it from closing, as in Shiki.
`Theme::style` does not allocate for each scope of the stack it styles.
