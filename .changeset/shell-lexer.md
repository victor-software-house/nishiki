---
nishiki: patch
---

Highlight shell scripts, and the `sh`, `bash`, `shell` and `zsh` fences of Markdown, with the scopes of Shiki's `shellscript` grammar; `Scopes` now yields names borrowed from the highlighter instead of `&'static str`, so it can name scopes made from source text.
