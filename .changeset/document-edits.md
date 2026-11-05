---
"nishiki": patch
---

`Document::replace` swaps a byte range for new text, for edits and diffs.
Lexing then stops at the first line whose end state matches its old one.
`Document::reset` starts a fresh highlighter when its arena fills.
