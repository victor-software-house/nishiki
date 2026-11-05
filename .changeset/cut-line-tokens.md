---
"nishiki": patch
---

`Highlighter::line` leaves the tokens already in its buffer as they were.
A Markdown, Vue or Svelte line that popped a less fence cut earlier lines' tokens there.
