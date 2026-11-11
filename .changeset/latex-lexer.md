---
nishiki: patch
---

Highlight LaTeX with a lexer of its own, under the `lang-latex` feature, enabled by default and in `all` and `docs`.
Its code environments, such as `minted` and `pycode`, hand their text to the language they name.

Markdown `latex` and `tex` fences are now LaTeX, as in Shiki; they were plain text.
