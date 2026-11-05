---
"nishiki": patch
---

A long last line of a `Document` is lexed again in bounded steps as it grows.
Past 1 KiB, it waits until it has grown by an eighth, so its cost stays linear.
`Document::set_tail` and `Tail` change or turn off the rule.
