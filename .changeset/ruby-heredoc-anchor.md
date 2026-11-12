---
"nishiki": patch
---

Ruby's here documents leave `\G` where each of their lines starts, as Shiki does.
In a shell here document, a line after one that ends in `\` starts with a command name.
Before, that line had no command name.
