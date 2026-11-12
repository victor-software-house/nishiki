---
"nishiki": patch
---

Svelte opens a `<script>` or `<style>` body after a `>` inside the tag's attribute value, as Shiki does.
In `<script lang=">let one = 1;</script>`, the text after the `>` is JavaScript, and so are the lines after the tag.
Before, the text stayed in the attribute's string.
