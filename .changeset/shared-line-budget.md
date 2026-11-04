---
nishiki: patch
---

Every language on one line now spends that line's one work budget: a fence in Markdown, a Ruby heredoc's embedded language, an HTML document's scripts and styles, and the JavaScript of a GraphQL interpolation no longer start a budget of their own.
Under a pinned `Limits::work`, such a line can reach the limit where it did not before.
Every language still counts only its own contexts against the depth limit.
The languages on nishiki's TextMate engine keep their own budget for now.
