---
"nishiki": patch
---

Vue reads a tag whose name runs straight into `/>`, such as `<div/>`, as Shiki does.
The tag closes itself, and the name's last letter reads as an attribute.
