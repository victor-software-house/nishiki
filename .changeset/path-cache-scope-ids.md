---
"nishiki": patch
---

Fix scopes that one token took from another on the same stack.
Two neighbouring escapes in a CSS class name were merged under the first one's scope.
A Java record's `<super>` could take the `extends` scope of an earlier record.
