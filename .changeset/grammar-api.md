---
"nishiki": patch
---

nishiki highlights any TextMate grammar a caller brings, behind the `textmate` feature, which now turns on `json`.

- `Grammar::from_json` compiles a grammar in the JSON form Shiki and VS Code use.
- `Grammar::to_bytes` and `Grammar::from_bytes` let a build script compile a grammar ahead of time.
- `Grammars` holds the grammars an include can name, and `Grammars::hand_off` names languages to lex by hand.
- `Highlighter::for_grammar` and `Highlighter::for_grammar_with_limits` build a `Highlighter<Grammar>`.
- Errors come back as a `GrammarError`.

Each distinct grammar stays in memory until the process ends.
