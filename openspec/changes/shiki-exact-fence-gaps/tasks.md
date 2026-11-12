# Tasks

Each fix is measured as design section 1 says: on a box, at seeds 1, 2 and 3.
Design section 2 names the runs that found each case.

## 1. Markdown

- [ ] 1.1 A `vue` fence opens in a paragraph's line after four spaces, as in Shiki.
  → verify: a test holds a paragraph, a blank line, and `x` and a `vue` fence each after four spaces, with Shiki's scopes
  - 2026-11-12: a Shiki probe in `shiki-exact-gaps` task 6.4's work shows it.
    The paragraph runs on past four spaces, and the injection opens the fence inside it.
    nishiki opens no fence there.
- [ ] 1.2 A `vue` fence in a Vue body inside a Markdown fence in a quote sits on Shiki's scopes.
  → verify: a test holds the case with Shiki's scopes, and no Markdown `mdw` document differs at any seed
  - 2026-11-12: Markdown's `mdw` corpus at seed 3 shows it in document 12, by 64 bytes, at `> ```vue:no-line-numbers`.
    Shiki puts `meta.tag-stuff` under the fence, and nishiki puts `source.ts`.

## 2. Vue fences in other languages

- [ ] 2.1 In a C# fence, the line after a `//` or `///` comment opens a `vue` fence on Shiki's scopes.
  → verify: a test holds both comments before a `vue` fence with Shiki's scopes, and the sweep's C# documents match
  - 2026-11-12: the sweep shows it in six of C#'s eight documents.
    Shiki's comment `while` rule fails on the fence's line, and the fence sits on the C# body.
    nishiki keeps the comment's scope under the fence.
- [ ] 2.2 A `vue` fence in a PHP heredoc that holds XML sits on Shiki's scopes.
  → verify: a test holds the case with Shiki's scopes, and the sweep's PHP documents match
  - 2026-11-12: the sweep shows it in PHP's document 1.
    Shiki puts `meta.tag.xml` under the fence, and nishiki leaves it out.
- [ ] 2.3 A Ruby heredoc's closing line ends where Shiki ends it, after a `vue` fence opens in its body.
  → verify: a Shiki probe names the rule that ends there, a test holds the case, and the sweep's Ruby documents match
  - 2026-11-12: the sweep shows it in Ruby's document 6.
    Both open the fence inside a `<<~RUBY` heredoc.
    At the `RUBY` line, Shiki ends the heredoc, and nishiki keeps the fence open.
- [ ] 2.4 In a `vue` fence, an attribute name that holds a `.` reads as a Vue directive, as in Shiki.
  → verify: a test holds `.size = size,` and `TEncoding.UTF8.GetString(` in an open tag with Shiki's scopes, and the sweep's Pascal and R documents match
  - 2026-11-12: the sweep shows it in Pascal's document 2 and R's document 4.
    Shiki gives `meta.attribute.directive.vue` and TypeScript.
    nishiki gives an unrecognized HTML attribute.

## 3. Svelte

- [ ] 3.1 A Svelte block in a TypeScript template string, in an `{@const}` type annotation, opens where Shiki opens it.
  → verify: a test holds `{#if` in the string with Shiki's scopes, and no Svelte bare differential document differs at any seed
  - 2026-11-12: Svelte's bare differential at seed 2 shows it in document 46, by 302 bytes, at line 20.
    Shiki opens `meta.special.if.svelte` inside `string.template.ts`, and nishiki keeps the string.

## 4. LaTeX

- [ ] 4.1 Lua in a LaTeX `\verb` argument in a quote gives its words Shiki's scopes.
  → verify: a test holds the case with Shiki's scopes, and no LaTeX `mdw` document differs at any seed
  - 2026-11-12: seed 2's `mdw` corpus of LaTeX shows it in document 87, by 13 bytes.
    Shiki gives `title` `variable.other.lua`, and nishiki gives it `source.lua` alone.

## 5. Bat, Python and YAML

- [ ] 5.1 Bat gives a `rem` comment in a quote's fence Shiki's scopes.
  → verify: a test holds the case with Shiki's scopes, and no Bat `mdw` document differs at any seed
  - 2026-11-12: seed 2's `mdw` corpus of Bat shows it in document 40, by 70 bytes.
    The line holds a non-ASCII digit, `٣`.
    Shiki opens `comment.line.rem.batchfile`, and nishiki opens nothing.
- [ ] 5.2 Python leaves a raw f-string where Shiki leaves it, in a fence in a list.
  → verify: a test holds the case with Shiki's scopes, and no Python `mdw` document differs at any seed
  - 2026-11-12: seed 2's `mdw` corpus of Python shows it in document 2, by 127 bytes.
    At line 10, nishiki keeps a raw f-string open, and Shiki has closed it.
- [ ] 5.3 YAML gives a key in a quote's fence Shiki's scopes, such as `complex4:`.
  → verify: a test holds the case with Shiki's scopes, and no YAML `mdw` document in a quote differs at any seed
  - 2026-11-12: seed 2's `mdw` corpus of YAML shows it in document 27, by 1 byte.
- [ ] 5.4 YAML gives `-:: !circle` Shiki's scopes.
  → verify: a test holds the line with Shiki's scopes, and no YAML `mdw` document differs at any seed
  - 2026-11-12: seed 2's `mdw` corpus of YAML shows it in document 28, by 2 bytes.
    Shiki gives the key `entity.name.tag.yaml`, and nishiki leaves it out.
