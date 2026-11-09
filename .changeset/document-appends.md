---
"nishiki": patch
---

A `Document` highlights text as it streams in, delta by delta.
A delta may end mid-line or inside a UTF-8 character.
A completed line is lexed once, and only the last line is lexed again on each delta.
The last line is provisional, and marks where its last token starts.

`Document::lex` lexes through a chosen line, so a viewport can come first.
`Document::take_changes` names the lines to draw again.
