---
"nishiki": patch
---

`Highlighter` takes a type parameter, `S: Syntax`, which defaults to `Language`.
`Highlighter` alone still names a language highlighter, so existing code compiles as before.
`Syntax` is sealed.
It makes room for highlighters that lex a grammar the caller loads.
