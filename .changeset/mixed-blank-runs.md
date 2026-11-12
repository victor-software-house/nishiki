---
nishiki: patch
---

In Dart, Go, Jinja, Kotlin, Nix, PowerShell, Protocol Buffers and Zig, a long run of spaces, tabs and Unicode spaces ends under the work limit.
These languages pass a run of such mixed blanks in one step.
Their patterns treat a Unicode space as a space, so the tokens stay the same.
