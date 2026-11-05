---
nishiki: patch
---

Read HTML's attribute list for an invalid tag, such as `<foo close for>`, in HTML that a Markdown or Svelte document holds, as Shiki does.
That covers Markdown `html` and `php` fences, and the HTML that PHP and Perl hand on inside them.
An HTML document still reads the MathML list there.
