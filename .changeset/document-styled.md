---
"nishiki": patch
---

A `Document` styles its tokens with its theme.
`DocumentLine::styled` gives each token's end and style, equal to `Theme::style`.
`Document::lex` keeps each stack's style, so reading a token costs one lookup.
`Document::set_theme` switches the theme and restyles every token once.
