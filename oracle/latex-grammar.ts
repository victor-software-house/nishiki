// Writes src/latex/scope.rs and src/latex/grammar.rs from the `latex` grammar that Shiki 4.6.0 bundles
// (microsoft/vscode `extensions/latex`, MIT, from jlelong/vscode-latex-basics).
// It runs the generator in `oracle/rule-generator.ts`.
// The matchers that decide whether a pattern matches are written by hand in src/latex.
//
// The grammar includes `text.tex`, the TeX grammar, as its last root pattern and in places.
// TeX's matchers answer those expressions, so `Pattern::expr` sends them to `crate::tex::grammar::Rx`.
// Code environments include other grammars.
// A language nishiki lexes takes their text; the others, which Shiki does not load with LaTeX, resolve to nothing.
//
// Run with `bun run oracle/latex-grammar.ts` and commit the result.
// The output is generated; do not edit it.
import langs from "@shikijs/langs@4.6.0/latex";

import { type Config, type Embed, type GrammarJson, generate } from "./rule-generator.ts";
import { tex } from "./tex-grammar.ts";

const lang = (name: string, feature: string): Embed => ({ lang: name, feature });

export const latex: Config = {
  grammar: (langs as unknown as GrammarJson[]).find((lang) => lang.scopeName === "text.tex.latex")!,
  generator: "latex-grammar.ts",
  id: "latex",
  suffix: "latex",
  dir: "latex",
  engine: "Latex",
  table: "LATEX",
  prefixes: {},
  literal: [],
  grammars: { "text.tex": { grammar: tex.grammar, prefix: "Tex" } },
  externals: {
    "source.asy": null,
    "source.asymptote": null,
    "source.cpp.embedded.latex": null,
    "source.dot": null,
    "source.gnuplot": null,
    "text.tex.markdown_latex_combined": null,
    "source.css": lang("Css", "lang-css"),
    "source.haskell": lang("Haskell", "lang-haskell"),
    "source.java": lang("Java", "lang-java"),
    "source.js": lang("JavaScript", "lang-javascript"),
    "source.julia": lang("Julia", "lang-julia"),
    "source.lua": lang("Lua", "lang-lua"),
    "source.python": lang("Python", "lang-python"),
    "source.r": lang("R", "lang-r"),
    "source.ruby": lang("Ruby", "lang-ruby"),
    "source.rust": lang("Rust", "lang-rust"),
    "source.scala": lang("Scala", "lang-scala"),
    "source.shell": lang("Shell", "lang-shell"),
    "source.ts": lang("TypeScript", "lang-typescript"),
    "source.yaml": lang("Yaml", "lang-yaml"),
    "text.html.basic": lang("Html", "lang-html"),
    "text.xml": lang("Xml", "lang-xml"),
  },
  // Each embedding rule ends at a line that closes its environment, or at a `}`, as `\Sexpr{…}`'s R does.
  endHalts: {
    by: {
      "^\\s*(?=\\\\end\\{(?:minted|lstlisting|pyglist)})": "super::engine::closes_listing",
      "^\\s*(?=\\\\end\\{(?:RobExt)?(?:CacheMeCode|PlaceholderPathFromCode\\*?|PlaceholderFromCode\\*?|SetPlaceholderCode\\*?)})":
        "super::engine::closes_cache",
      "^\\s*(?=\\\\end\\{hscode\\*?})": "super::engine::closes_hscode",
      "^\\s*(?=\\\\end\\{java(?:code|verbatim|block|concode|console|converbatim)\\*?})":
        "super::engine::closes_java",
      "^\\s*(?=\\\\end\\{jl(?:code|verbatim|block|concode|console|converbatim)\\*?})":
        "super::engine::closes_jl",
      "^\\s*(?=\\\\end\\{julia(?:code|verbatim|block|concode|console|converbatim)\\*?})":
        "super::engine::closes_julia",
      "^\\s*(?=\\\\end\\{lua(?:code|draw)\\*?})": "super::engine::closes_lua",
      "^\\s*(?=\\\\end\\{py(?:code|verbatim|block|concode|console|converbatim)\\*?})":
        "super::engine::closes_py",
      "^\\s*(?=\\\\end\\{pylab(?:code|verbatim|block|concode|console|converbatim)\\*?})":
        "super::engine::closes_pylab",
      "^\\s*(?=\\\\end\\{(?:sageblock|sagesilent|sageverbatim|sageexample|sagecommandline|pythonq??|pythonrepl)\\*?})":
        "super::engine::closes_sage",
      "^\\s*(?=\\\\end\\{scalacode\\*?})": "super::engine::closes_scalacode",
      "^\\s*(?=\\\\end\\{sympy(?:code|verbatim|block|concode|console|converbatim)\\*?})":
        "super::engine::closes_sympy",
      "}": "super::engine::closes_brace",
      "(?=})": "super::engine::closes_brace",
    },
    otherwise: "super::engine::never",
  },
  shared: { path: "crate::tex::grammar::Rx", ids: await generate(tex, false) },
  backrefStart: "any",
  words: (read) => [
    {
      name: "TEXT_SYMBOL",
      doc: "The text symbols `AllOtherMacroAt4` names, such as `\\textdollar` and `\\textregistered`.",
      body: read.inside("AllOtherMacroAt4", "(\\\\)(", ")\\b"),
    },
  ],
};

if (import.meta.main) await generate(latex);
