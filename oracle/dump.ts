// Writes Shiki's scopes for source files, in the fixture format (fixture.ts),
// as a corpus for the ignored test `the_corpus_matches_shiki` in
// tests/it/oracle.rs: held-out files that are not oracle inputs.
//
//   bun run oracle/dump.ts <lang> <outdir> files...
//
// Each file is written to `<outdir>/<file name>.<lang>.txt`.
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { basename } from "node:path";
import { createHighlighter } from "shiki@4.6.0";
import { fixture } from "./fixture.ts";
import { grammars } from "./grammars.ts";

const [lang, outdir, ...files] = process.argv.slice(2);
if (!lang || !outdir || !files.length) {
  console.error("usage: bun run oracle/dump.ts <lang> <outdir> files...");
  process.exit(2);
}
mkdirSync(outdir, { recursive: true });
// Every input language is loaded, as generate.ts loads them, so an embedded
// language is lexed as in the fixtures.
const highlighter = await createHighlighter({ langs: grammars, themes: ["github-dark"] });
for (const file of files) {
  const text = readFileSync(file, "utf8").replaceAll("\r\n", "\n");
  writeFileSync(`${outdir}/${basename(file)}.${lang}.txt`, fixture(highlighter, text, lang));
}
console.log(`✔ ${files.length} files in ${outdir}`);
