---
"nishiki": patch
---

Ruby closes a percent literal that ends a line, such as `%w`, where the next line ends, as Shiki does.
The line feed is the literal's delimiter, so the literal holds one line.
Before, the literal held every line after it.
