// Writes src/tex/scope.rs and src/tex/grammar.rs from the `tex` grammar that Shiki 4.6.0 bundles
// (microsoft/vscode `extensions/latex`, MIT, from jlelong/vscode-latex-basics).
// It runs the generator in `oracle/rule-generator.ts`.
// The matchers that decide whether a pattern matches are written by hand in src/tex.
// LaTeX's grammar includes this one, and its generator reads TeX's expressions from here.
//
// Run with `bun run oracle/tex-grammar.ts` and commit the result.
// The output is generated; do not edit it.
import langs from "@shikijs/langs@4.6.0/tex";

import { type Config, type GrammarJson, generate } from "./rule-generator.ts";

export const tex: Config = {
  grammar: (langs as unknown as GrammarJson[]).find((lang) => lang.scopeName === "text.tex")!,
  generator: "tex-grammar.ts",
  id: "tex",
  suffix: "tex",
  dir: "tex",
  engine: "Tex",
  table: "TEX",
  prefixes: {},
  literal: [],
  // `\Sexpr{…}` in math hands its text to R, up to the `}` that closes it.
  externals: { "source.r": { lang: "R", feature: "lang-r" } },
  endHalts: { by: { "(?=})": "super::engine::closes_brace" }, otherwise: "super::engine::never" },
  // Math ends at a repeat of `$` or `$$`.
  backrefStart: "any",
  words: (read) => [
    {
      name: "CONTROL",
      doc: "The primitives `MacroControl` names: `\\else`, `\\fi`, `\\ifcase` and the rest.",
      body: read.inside("MacroControl", "(\\\\)(", ")(?![@-Za-z])"),
    },
    {
      name: "SYMBOL",
      doc: "The math symbols `MathContentAt3` names, such as `\\alpha` and `\\leq`.",
      body: read.inside("MathContentAt3", "(\\\\)(", ")(?![@-Za-z])"),
    },
    {
      name: "OPERATOR",
      doc: "The big operators `MathContentAt4` names, such as `\\sum` and `\\int`.",
      body: read.inside("MathContentAt4", "(\\\\)(", ")\\b"),
    },
    {
      name: "FUNCTION",
      doc: "The math functions `MathContentAt5` names, such as `\\sin` and `\\log`.",
      body: read.inside("MathContentAt5", "(\\\\)(", ")\\b"),
    },
  ],
};

if (import.meta.main) await generate(tex);
