// Writes a differential corpus: documents that Shiki tokenizes, in the
// fixture format (fixture.ts), for the ignored test `the_corpus_matches_shiki`
// in tests/it/oracle.rs to compare with nishiki.
//
//   bun run oracle/fuzz.ts <lang> <mode> <seed> <count> <outdir> [files...]
//
// Modes:
// - win: windows of the source files, mutated;
// - gen: documents of random pieces from oracle/fuzz/<lang>.ts;
// - mdw, mdg: the documents of win or gen in Markdown fences of the language,
//   bare, in block quotes and in list items.
//
// The source files default to the texts of the language's oracle fixtures.
// Each document is written to `<outdir>/<mode><seed>-<n>.<lang or md>.txt`.
// The same arguments write the same corpus.
// oracle/fuzz-texts.ts draws the documents.
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { createHighlighter } from "shiki@4.6.0";
import { fixture } from "./fixture.ts";
import { extrasOf, fixtureTexts, fuzzTexts, type Mode } from "./fuzz-texts.ts";
import { grammars } from "./grammars.ts";

const [lang, mode, seedArg, countArg, outdir, ...files] = process.argv.slice(2);
if (
  !lang ||
  !mode ||
  !seedArg ||
  !countArg ||
  !outdir ||
  !["win", "gen", "mdw", "mdg"].includes(mode)
) {
  console.error(
    "usage: bun run oracle/fuzz.ts <lang> <win|gen|mdw|mdg> <seed> <count> <outdir> [files...]",
  );
  process.exit(2);
}
const extras = await extrasOf(lang);
if ((mode === "gen" || mode === "mdg") && !extras.pieces) {
  console.error(`mode ${mode} needs pieces in oracle/fuzz/${lang}.ts`);
  process.exit(2);
}
const texts = files.length ? files.map((file) => readFileSync(file, "utf8")) : fixtureTexts(lang);
const documents = fuzzTexts(lang, mode as Mode, Number(seedArg), Number(countArg), extras, texts);

mkdirSync(outdir, { recursive: true });
const md = mode.startsWith("md");
const docLang = md ? "markdown" : lang;
// Every input language is loaded, as for the fixtures.
// A Vue block in TSX or SCSS, for one, stays plain without that grammar.
const highlighter = await createHighlighter({ langs: grammars, themes: ["github-dark"] });
for (const [n, text] of documents.entries()) {
  const name = `${mode}${seedArg}-${String(n).padStart(5, "0")}.${md ? "md" : lang}.txt`;
  writeFileSync(`${outdir}/${name}`, fixture(highlighter, text, docLang));
}
console.log(`✔ ${countArg} ${mode} documents in ${outdir}`);
