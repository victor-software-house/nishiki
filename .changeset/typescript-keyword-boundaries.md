---
nishiki: patch
---

Match Shiki in TypeScript and JavaScript where a keyword rule's word boundaries differ from its neighbours': `interface` and the doc-comment tags need `\b`, `export` does not, a `namespace` needs a name after it, a cast at the start of a line has no blank after `<`, and an arrow's return type needs a type before `=>`.
