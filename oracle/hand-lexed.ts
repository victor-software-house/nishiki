// The languages an engine grammar can hand a region to: every language nishiki
// lexes by hand. The keys are Shiki's ids. Design section 14 of
// `openspec/changes/engine-handoff` describes the hand-off.
//
// The list is read from the rows of `languages!` in `src/language.rs`, which
// the Rust linker reads too, so a new language cannot be left out of either.
// `oracle/coverage.ts` checks each row's scope against Shiki's grammar.
//
// A spec names the languages its table hands off with `>`, as in
// `haml>ruby>javascript`. An include of one of their grammars becomes a
// hand-off to the language. The table holds no copy of the grammar's rules.
// The grammar needs no licence entry for that table.
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

/** A language an engine grammar can hand a region to. */
type HandLexed = {
  /** The root scope of Shiki's grammar for it. */
  scope: string;
  /** Its `Language` variant. */
  variant: string;
  /** The variants of the languages whose text its lexer also lexes. */
  reads: string[];
  /** Whether its lexer can halt where the grammar takes the line back. */
  halts: boolean;
};

const rows = readFileSync(fileURLToPath(new URL("../src/language.rs", import.meta.url)), "utf8");
const table = rows.slice(rows.search(/^languages! \{/m));
const row = new RegExp(
  [
    String.raw`\n {4}([A-Z][A-Za-z0-9]*) \{`,
    String.raw`\s*feature: "[^"]+",`,
    String.raw`\s*names: \["([^"]+)"[^\]]*\],`,
    String.raw`\s*scope: "([^"]+)",`,
  ].join(""),
  "g",
);

const matches = [...table.matchAll(row)];

/** The text of the row that `matches[i]` starts, which ends where the next row starts. */
const bodyOf = (i: number): string =>
  table.slice(matches[i].index, matches[i + 1]?.index ?? table.length);

/** The `reads` list of a row's text. */
const readsOf = (body: string): string[] =>
  body
    .match(/\breads: \[([^\]]+)\]/)?.[1]
    .split(",")
    .map((name) => name.trim()) ?? [];

export const handLexed: Record<string, HandLexed> = Object.fromEntries(
  matches.map(([, variant, id, scope], i) => {
    const body = bodyOf(i);
    return [id, { scope, variant, reads: readsOf(body), halts: /\bhalts: true\b/.test(body) }];
  }),
);

const features = table.match(/\bfeature: "/g)?.length ?? 0;
if (Object.keys(handLexed).length !== features) {
  const read = Object.keys(handLexed).length;
  throw new Error(`src/language.rs: read ${read} of ${features} languages! rows`);
}
