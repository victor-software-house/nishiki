// Writes src/blade/scope.rs, src/blade/grammar.rs and src/blade/words.rs from
// the PHP that the `blade` grammar of Shiki 4.6.0 holds
// (laravel/vs-code-extension, MIT), with the generator that
// oracle/php-family-grammar.ts shares with PHP and Hack.
//
// Blade's grammar carries its own copy of PHP's rules, the `language` entry,
// which differs from Shiki's `php` grammar.
// Blade's own rules, such as `{{ … }}` and `@if (…)`, are HTML rules in
// src/html; the PHP inside them is this grammar, from its root.
//
// A `<<<BLADE` heredoc lists HTML and then Blade's own rules.
// Those rules are the Blade language's, so the heredoc embeds Blade.
//
// `support` holds `(?i)\b(isset|unset|eval|empty|list)\b` as a begin with no end.
// vscode-textmate gives such a rule the end `\uFFFF`, and so does this script.
// The rule splits `support`'s word lists, so each part is a table:
// `support_head` before it, `support_output` for `print|echo`, which the
// root also holds alone, and `support_tail` after that.
//
// The matchers that decide whether a pattern matches are written by hand in
// src/blade/matchers.
//
// `RX_DUMP=<file>` also writes the expressions for oracle/onig.ts: those of
// the copy of PHP, then those of Blade's own rules that its probe answers.
//
// Run with `bun run oracle/blade-grammar.ts` and commit the result.
// The output is generated; do not edit it.
import blade from "@shikijs/langs@4.6.0/blade";
import { type Family, generate } from "./php-family-grammar.ts";

const grammar = (blade as unknown as Family["grammar"][]).find((lang) => lang.name === "blade")!;
/** The entry `key` with its `<<<BLADE` rule embedding Blade after the patterns before HTML. */
function bladeEmbedded(key: string) {
  const entry = grammar.repository[key]!;
  return {
    ...entry,
    patterns: entry.patterns!.map((node) => {
      if (!node.begin?.includes("(BLADE)")) return node;
      const html = node.patterns!.findIndex((p) => p.include === "text.html.basic");
      return {
        ...node,
        patterns: [...node.patterns!.slice(0, html), { include: "text.html.php.blade" }],
      };
    }),
  };
}
const supportList = grammar.repository.support!.patterns!;
const construct = supportList.findIndex(
  (node) => node.begin !== undefined && node.end === undefined,
);
const repository = {
  ...grammar.repository,
  support: {
    patterns: [
      { include: "#support_head" },
      { ...supportList[construct]!, end: "\uFFFF" },
      { include: "#support_output" },
      { include: "#support_tail" },
    ],
  },
  support_head: { patterns: supportList.slice(0, construct) },
  support_output: { patterns: supportList.slice(construct + 1, construct + 2) },
  support_tail: { patterns: supportList.slice(construct + 2) },
  heredoc_interior: bladeEmbedded("heredoc_interior"),
  nowdoc_interior: bladeEmbedded("nowdoc_interior"),
};

await generate({
  grammar: {
    ...grammar,
    repository,
    scopeName: "source.php",
    patterns: [{ include: "#language" }],
  },
  name: "blade",
  script: "blade-grammar.ts",
  dir: "blade",
  engine: "Blade",
  display: "BLADE",
  embeddable: {
    "text.html.basic": ["Html", "lang-html"],
    "text.html.php.blade": ["Blade", "lang-blade"],
    "text.xml": ["Xml", "lang-xml"],
    "source.sql": ["Sql", "lang-sql"],
    "source.js": ["JavaScript", "lang-javascript"],
    "source.json": ["Json", "lang-json"],
    "source.css": ["Css", "lang-css"],
  },
  tables: [
    { name: "SupportHead", key: "support_head", fold: true, slash: false },
    { name: "SupportOutput", key: "support_output", fold: true, slash: false },
    { name: "SupportTail", key: "support_tail", fold: true, slash: false },
    { name: "Constants", key: "constants", fold: false, slash: true },
    { name: "Classes", key: "class-builtin", fold: true, slash: true },
  ],
  supportAllLists: false,
  htmlFirst: null,
});

/** Blade's own expressions that `blade::tests::probe` answers through
 * src/html/blade.rs, by the path of their rule in the `blade` entry: the
 * indices of its nested `patterns`, then the role. An id spells the path, so
 * `BladeAt0At0End` is `#blade.patterns[0].patterns[0].end`. The injection's
 * rules repeat two of them, `BladeAt0At0` and `BladeAt0At2At0`. */
const own: [indices: number[], role: "begin" | "end" | "match"][] = [
  [[0, 0], "begin"],
  [[0, 0], "end"],
  [[0, 2, 0], "match"],
  [[1], "end"],
  [[2], "end"],
  [[3], "end"],
  [[4], "end"],
  [[9], "end"],
  [[14], "begin"],
];

// With RX_DUMP set, the generator above writes the expressions of Blade's copy
// of PHP. Blade's own expressions are added after them.
if (Bun.env["RX_DUMP"]) {
  const rows = own.map(([indices, role]) => {
    const node = indices.reduce((at, index) => at.patterns![index]!, grammar.repository.blade!);
    const captures =
      role === "begin"
        ? (node.beginCaptures ?? node.captures)
        : role === "end"
          ? (node.endCaptures ?? node.captures)
          : node.captures;
    const id = `Blade${indices.map((index) => `At${index}`).join("")}${role === "end" ? "End" : ""}`;
    return JSON.stringify({
      id,
      regex: node[role],
      groups: Object.keys(captures ?? {}).map(Number),
    });
  });
  const dump = Bun.file(Bun.env["RX_DUMP"]);
  await Bun.write(dump, `${await dump.text()}${rows.join("\n")}\n`);
}
