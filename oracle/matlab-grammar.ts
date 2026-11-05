// Writes src/matlab/scope.rs and src/matlab/grammar.rs from the `matlab` grammar that Shiki 4.6.0 bundles
// (mathworks/MATLAB-Language-grammar, BSD-2-Clause).
// It runs the generator in `oracle/rule-generator.ts`.
// The matchers that decide whether a pattern matches are written by hand in src/matlab.
//
// Run with `bun run oracle/matlab-grammar.ts` and commit the result.
// The output is generated; do not edit it.
import langs from "@shikijs/langs@4.6.0/matlab";

import { type Config, type GrammarJson, generate } from "./rule-generator.ts";

export const matlab: Config = {
  grammar: (langs as unknown as GrammarJson[]).find((lang) => lang.scopeName === "source.matlab")!,
  generator: "matlab-grammar.ts",
  id: "matlab",
  suffix: "matlab",
  dir: "matlab",
  engine: "Matlab",
  table: "MATLAB",
  prefixes: {},
  literal: [],
  // The bracket rules include it after their other patterns.
  unresolved: ["block_keywords"],
  // The parens and indexing braces end at `(?<!\.\.\.).\n`, which can match at the last blank of a run.
  toLast: true,
  words: () => [],
};

if (import.meta.main) await generate(matlab);
