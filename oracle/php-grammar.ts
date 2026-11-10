// Writes src/php/scope.rs, src/php/grammar.rs and src/php/words.rs from the
// `php` grammar that Shiki 4.6.0 bundles (KapitanOczywisty/language-php
// through microsoft/vscode, MIT), with the generator that
// oracle/php-family-grammar.ts shares with Hack.
//
// The rules that include another grammar embed it (`CarriedState::Embed`):
// the heredocs and nowdocs tagged HTML, XML, SQL, JS, JSON or CSS, and the
// strings that start with an SQL keyword.
// A second root, `Fence`, serves a Markdown `php` fence, whose rule lists
// HTML and then PHP's patterns.
//
// The matchers that decide whether a pattern matches are written by hand in
// src/php/matchers.
//
// Run with `bun run oracle/php-grammar.ts` and commit the result.
// The output is generated; do not edit it.
import php from "@shikijs/langs@4.6.0/php";
import { type Family, generate } from "./php-family-grammar.ts";

await generate({
  grammar: (php as unknown as Family["grammar"][]).find((lang) => lang.name === "php")!,
  name: "php",
  script: "php-grammar.ts",
  dir: "php",
  engine: "Php",
  display: "PHP",
  embeddable: {
    "text.html.basic": ["Html", "lang-html"],
    "text.xml": ["Xml", "lang-xml"],
    "source.sql": ["Sql", "lang-sql"],
    "source.js": ["JavaScript", "lang-javascript"],
    "source.json": ["Json", "lang-json"],
    "source.css": ["Css", "lang-css"],
  },
  tables: [
    { name: "Support", key: "support", fold: true, slash: false },
    { name: "Constants", key: "constants", fold: false, slash: true },
    { name: "Classes", key: "class-builtin", fold: true, slash: true },
  ],
  supportAllLists: true,
  htmlFirst: "Fence",
});
