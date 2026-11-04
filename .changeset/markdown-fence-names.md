---
nishiki: patch
---

Read Markdown fences as Shiki does for three more names: a `bat` or `batch` fence is lexed as Batch when `lang-bat` is on, a `php` fence's body is lexed as HTML under `meta.embedded.block.php`, and an `xsl` or `xslt` fence is plain, since nishiki does not lex the XSL grammar.
