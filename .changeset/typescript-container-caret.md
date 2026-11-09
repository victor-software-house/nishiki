---
nishiki: patch
---

Match `^` in TypeScript, JavaScript, JSX and TSX code only at the start of the line.
It no longer matches at the start of a Markdown fence's text inside a list or a quote.
There, as in Shiki, a type annotation runs on to the next line and a triple-slash directive is a plain comment.
