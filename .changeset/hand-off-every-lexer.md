---
"nishiki": patch
---

A grammar loaded from JSON can include the grammar of any supported language.
That include now goes to the language's lexer when the language's feature is on and the `Grammars` set lacks that grammar.
Before, only eleven languages took such a hand-off, and an include of another language's grammar matched nothing.

`Grammars::hand_off` now accepts every language.
So `Highlighter::for_grammar` no longer fails with "no grammar can hand a region to" a language.

A list of rules that includes several such grammars no longer fails with "a list of rules includes two hand-offs".
One language takes the region: the one whose lexer also lexes the others' text, else the first.

- A Markdown `php` fence goes to PHP's lexer, which reads its HTML too.
- A Vue template goes to html-derivative's lexer.

With no language named through `Grammars::hand_off`, a region whose end pattern uses `\G` hands nothing off, and the grammar links.
A named language still makes such a grammar fail.

Some lexers, such as Rust's, cannot stop inside a line where the grammar takes the text back.
Such a language takes a region only when the region has no rules beside the include and no end pattern, as in a Markdown fence.

- With no language named, any other region hands nothing off.
- A named language makes such a grammar fail.

html-derivative's lexer now stops there.
So a Markdown paragraph's inline code, emphasis and links keep their scopes around its HTML.
