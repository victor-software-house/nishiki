---
"nishiki": patch
---

Vue lexes blocks in Markdown, JSON, JSONC, TOML and Vue with those languages' lexers, as Shiki does.
Before, their text was plain.
A `<!-- @name ... -->` comment holds JSON.
In Vue's tags, `generic` holds TypeScript type parameters and `style` holds CSS, with Vue's own scopes.
A directive's dynamic argument, such as `[eventName]` in `v-on:[eventName]`, holds a TypeScript expression.
