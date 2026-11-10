---
"nishiki": patch
---

An HTML fence in a Markdown grammar loaded through `Grammar::from_json` now names attributes as Shiki does.
A name such as `close` on an unknown tag is `meta.attribute.unrecognized.close.html` there, as in nishiki's own Markdown.

Before, the loaded grammar's fences read HTML as another grammar embeds it, so some attribute names differed.
