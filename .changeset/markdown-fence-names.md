---
nishiki: patch
---

Read Markdown fences as Shiki does for three more names.
A `bat` or `batch` fence is lexed as Batch when `lang-bat` is on.
A `php` fence's body is lexed as HTML under `meta.embedded.block.php`.
An `xsl` or `xslt` fence is plain, because nishiki does not lex the XSL grammar.
