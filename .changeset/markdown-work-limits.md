---
nishiki: patch
---

Markdown counts every byte a link reference definition's title or a fence's `{.lang}` attributes read against the work limit, and checks each in time linear in the line, so a hostile line no longer costs time quadratic in its length.
A fence line that runs out of interned nodes resumes the same whether its state is cached or not.
