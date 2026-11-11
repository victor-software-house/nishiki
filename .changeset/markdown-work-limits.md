---
nishiki: patch
---

Markdown counts every byte that a link reference definition's title or a fence's `{.lang}` attributes read against the work limit.
It checks each in time linear in the line, so no hostile line can make the work quadratic in its length.
A fence line that runs out of interned nodes resumes the same whether its state is cached or not.
