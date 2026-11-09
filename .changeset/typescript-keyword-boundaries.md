---
nishiki: patch
---

Match Shiki in TypeScript and JavaScript where a keyword rule's word boundaries differ from its neighbours'.
`interface` and the doc-comment tags need `\b`, and `export` does not.
A `namespace` needs a name after it.
A cast at the start of a line has no blank after `<`.
An arrow's return type needs a type before `=>`.
