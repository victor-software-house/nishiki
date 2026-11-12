---
nishiki: patch
---

A TextMate grammar loaded through `Grammar` fills back-references in an `end` or `while` pattern as vscode-textmate does:

- a reference is a backslash and every digit after it, so `\10` names group 10 and `\0` the whole match;
- a backslash before it, as in `\\1`, still leaves it a reference;
- the group's text is escaped wherever JavaScript's `\s` matches, such as at a no-break space.

No bundled grammar changes its output.
