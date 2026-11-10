---
nishiki: patch
---

Every language on one line spends that line's one work budget.
A fence in Markdown starts no budget of its own.
Nor do a Ruby heredoc's embedded language, an HTML document's scripts and styles, or the JavaScript of a GraphQL interpolation.

Under a pinned `Limits::work`, such a line can reach the limit where it did not before.
Every language still counts only its own contexts against the depth limit.
The languages on nishiki's TextMate engine keep their own budget for now.
