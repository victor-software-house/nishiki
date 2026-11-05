// Writes src/hack/scope.rs, src/hack/grammar.rs and src/hack/words.rs from the
// `hack` grammar that Shiki 4.6.0 bundles (slackhq/vscode-hack, MIT, converted
// from hhvm/atom-ide-hack, MIT), with the generator that
// oracle/php-family-grammar.ts shares with PHP.
//
// The grammar's root lists HTML first and Hack's patterns after it.
// That list is the rule `Document`, opened above the root, which keeps Hack's
// patterns.
// The strings that start with an SQL keyword embed SQL.
//
// The matchers that decide whether a pattern matches are written by hand in
// src/hack/matchers.
//
// Run with `bun run oracle/hack-grammar.ts` and commit the result.
// The output is generated; do not edit it.
import hack from "@shikijs/langs@4.6.0/hack";
import { type Family, generate } from "./php-family-grammar.ts";

await generate({
  grammar: (hack as unknown as Family["grammar"][]).find((lang) => lang.name === "hack")!,
  name: "hack",
  script: "hack-grammar.ts",
  dir: "hack",
  engine: "Hack",
  display: "HACK",
  embeddable: {
    "text.html.basic": ["Html", "lang-html"],
    "source.sql": ["Sql", "lang-sql"],
  },
  tables: [
    { name: "Support", key: "support", fold: true, slash: false },
    { name: "Constants", key: "constants", fold: false, slash: true, deep: true },
    { name: "Classes", key: "class-builtin", fold: true, slash: true },
  ],
  supportAllLists: true,
  htmlFirst: "Document",
});
