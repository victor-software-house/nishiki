---
"nishiki": patch
---

An HTML fence in a Markdown grammar loaded through `Grammar::from_json` now names attributes as Shiki does.
A name such as `close` on an unknown tag is `meta.attribute.unrecognized.close.html` there, as in nishiki's own Markdown.
This holds with or without the `lang-markdown` feature.
HTML in a loaded Svelte grammar follows Svelte's order the same way.

Before, the loaded grammar's fences read HTML as another grammar embeds it, so some attribute names differed.
