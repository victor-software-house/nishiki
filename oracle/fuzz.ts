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
import { mkdirSync, readFileSync, readdirSync, writeFileSync } from "node:fs";
import { createHighlighter } from "shiki@4.6.0";
import { fixture } from "./fixture.ts";
import { grammars } from "./grammars.ts";

type Extras = { pieces?: string[]; mutations?: string[]; fences?: string[] };

const [lang, mode, seedArg, countArg, outdir, ...files] = process.argv.slice(2);
if (!lang || !mode || !seedArg || !countArg || !outdir || !["win", "gen", "mdw", "mdg"].includes(mode)) {
  console.error("usage: bun run oracle/fuzz.ts <lang> <win|gen|mdw|mdg> <seed> <count> <outdir> [files...]");
  process.exit(2);
}
const root = new URL("..", import.meta.url).pathname;
const extras: Extras = await import(`./fuzz/${lang}.ts`).catch(() => ({}));
if (mode.endsWith("g") && !extras.pieces) {
  console.error(`mode ${mode} needs pieces in oracle/fuzz/${lang}.ts`);
  process.exit(2);
}

let state = (Number(seedArg) * 2654435761) >>> 0 || 1;
const random = () => {
  state ^= state << 13;
  state >>>= 0;
  state ^= state >>> 17;
  state ^= state << 5;
  state >>>= 0;
  return state / 4294967296;
};
const int = (n: number) => Math.floor(random() * n);
const pick = <T>(items: T[]): T => items[int(items.length)] as T;

/** Texts that break most grammars somewhere: brackets, quotes, comment
 * markers, escapes, a line continuation, and characters outside ASCII. */
const MUTATIONS = ["(", ")", "{", "}", "[", "]", "<", ">", ";", ",", ":", "::", "\"", "'", "`", "\\", "/*", "*/", "//", "#", "=", " ", "\t", "é", "日", "٣", " ", "́", "🦀", "\\\n"];
const mutations = [...MUTATIONS, ...(extras.mutations ?? [])];

/** `text` with a few insertions, deletions, and repeated, dropped or cut lines. */
function mutate(text: string): string {
  let out = text;
  for (let n = 1 + int(5); n > 0; n--) {
    const op = int(6);
    const at = int(out.length + 1);
    const lines = out.split("\n");
    const k = int(lines.length);
    const line = lines[k] ?? "";
    if (op === 0) out = out.slice(0, at) + pick(mutations) + out.slice(at);
    else if (op === 1) out = out.slice(0, at) + out.slice(at + 1 + int(4));
    else {
      if (op === 2) lines[k] = `${line}\\`;
      else if (op === 3) lines.splice(k, 1);
      else if (op === 4) lines.splice(k, 0, line);
      else lines[k] = line.slice(0, int(line.length + 1));
      out = lines.join("\n");
    }
  }
  return out;
}

/** The text of every fixture of `lang`, in name order: `readdirSync` order
 * varies between file systems, and the windows drawn depend on it. */
function fixtureTexts(): string[] {
  const dir = `${root}tests/oracle/${lang}`;
  return readdirSync(dir)
    .filter((name) => name.endsWith(".txt"))
    .sort()
    .map((name) =>
      readFileSync(`${dir}/${name}`, "utf8")
        .split("\n")
        .filter((record) => record.startsWith("L "))
        .map((record) => record.slice(2))
        .join("\n"),
    );
}

const sources = (files.length ? files.map((file) => readFileSync(file, "utf8")) : fixtureTexts()).map((text) =>
  text.replaceAll("\r\n", "\n").split("\n"),
);

function window(): string {
  const lines = pick(sources);
  const length = 8 + int(60);
  const start = int(Math.max(1, lines.length - length));
  return mutate(lines.slice(start, start + length).join("\n"));
}

function generated(): string {
  const pieces = extras.pieces ?? [];
  const out: string[] = [];
  for (let n = 3 + int(25); n > 0; n--) {
    let line = "";
    for (let k = int(14); k > 0; k--) {
      line += pick(pieces);
      if (random() < 0.4) line += " ";
    }
    out.push(line);
  }
  return out.join("\n");
}

function markdown(): string {
  const fences = extras.fences ?? [lang];
  const out: string[] = [];
  for (let blocks = 1 + int(4); blocks > 0; blocks--) {
    const code = (mode === "mdg" ? generated() : window()).split("\n").slice(0, 25);
    const fence = pick(["```", "```", "~~~", "````"]);
    const info = pick(fences);
    const prefix = pick(["", "", "> ", "  ", "   "]);
    const opener = prefix === "  " ? "- item" : prefix === "   " ? "1. item" : "";
    if (opener) out.push(opener);
    out.push(`${prefix}${fence}${info}`, ...code.map((line) => `${prefix}${line}`));
    out.push(random() < 0.8 ? `${prefix}${fence}` : "text after");
    out.push("", pick(["para", "# head", "- li", ""]));
  }
  return out.join("\n");
}

mkdirSync(outdir, { recursive: true });
const md = mode.startsWith("md");
const docLang = md ? "markdown" : lang;
// Every input language is loaded, as for the fixtures.
// A Vue block in TSX or SCSS, for one, stays plain without that grammar.
const highlighter = await createHighlighter({ langs: grammars, themes: ["github-dark"] });
for (let n = 0; n < Number(countArg); n++) {
  const text = md ? markdown() : mode === "gen" ? generated() : window();
  const name = `${mode}${seedArg}-${String(n).padStart(5, "0")}.${md ? "md" : lang}.txt`;
  writeFileSync(`${outdir}/${name}`, fixture(highlighter, text, docLang));
}
console.log(`✔ ${countArg} ${mode} documents in ${outdir}`);
