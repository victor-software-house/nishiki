// Shiki's side of the comparison (bench/compare/README.md).
//
// `bun bench/compare/shiki.ts reference` writes Shiki's github-dark colours
// for every corpus file whose language Shiki has.
// They go to target/compare/shiki/<language>/<tier>/<file>.colors.
// Each row is one line, and each token is `<byte end>:<colour>`.
// Shiki's per-line limits are off.
// The Rust harness reads them to score every engine's agreement with Shiki.
//
// `bun bench/compare/shiki.ts time [rounds]` times Shiki itself on the same
// files.
// It times both engines: Oniguruma in WebAssembly and the JavaScript engine.
// Setup and warm throughput are separate figures.
// A fresh Bun process per file gives the first-document time.
// Rows are tab-separated: engine, file, setup ms, first document ms, warm
// MiB/s.
import { bundledLanguages, createHighlighter } from "shiki@4.6.0";
import { createJavaScriptRegexEngine } from "shiki@4.6.0/engine/javascript";
import { createOnigurumaEngine } from "shiki@4.6.0/engine/oniguruma";
import { readdirSync, statSync } from "node:fs";
import { join } from "node:path";

const root = new URL("../..", import.meta.url).pathname;
const corpus = join(root, "bench/corpus");
const out = join(root, "target/compare/shiki");
const theme = "github-dark";
const limitsOff = { tokenizeMaxLineLength: 0, tokenizeTimeLimit: 0 } as const;

type File = { lang: string; path: string; name: string };

/** Every corpus file whose language Shiki bundles, as `<language>/<tier>/<file>`. */
function files(): File[] {
  const found: File[] = [];
  for (const lang of readdirSync(corpus).sort()) {
    if (!(lang in bundledLanguages)) continue;
    const langDir = join(corpus, lang);
    if (!statSync(langDir).isDirectory()) continue;
    for (const tier of readdirSync(langDir).sort()) {
      for (const file of readdirSync(join(langDir, tier)).sort()) {
        const name = `${lang}/${tier}/${file}`;
        found.push({ lang, path: join(corpus, name), name });
      }
    }
  }
  return found;
}

/** Shiki's text: CRLF read as LF, as Shiki splits lines. */
async function text(file: File): Promise<string> {
  return (await Bun.file(file.path).text()).replaceAll("\r\n", "\n");
}

async function reference(): Promise<void> {
  const all = files();
  const highlighter = await createHighlighter({
    langs: [...new Set(all.map((file) => file.lang))],
    themes: [theme],
  });
  const encoder = new TextEncoder();
  for (const file of all) {
    const tokens = highlighter.codeToTokens(await text(file), {
      lang: file.lang,
      theme,
      ...limitsOff,
    }).tokens;
    const rows = tokens.map((line) => {
      let end = 0;
      return line
        .map((token) => {
          end += encoder.encode(token.content).length;
          return `${end}:${token.color ?? ""}`;
        })
        .join(" ");
    });
    await Bun.write(join(out, `${file.name}.colors`), `${rows.join("\n")}\n`);
  }
  console.log(`✔ ${all.length} reference files in ${out}`);
}

/** One engine's highlighter, as a factory so setup can be timed. */
const engines = {
  "shiki-oniguruma": () => createOnigurumaEngine(import("shiki@4.6.0/wasm")),
  "shiki-javascript": () => createJavaScriptRegexEngine(),
} as const;
type EngineName = keyof typeof engines;

/** Times one fresh setup and one first document; prints both, in ms. */
async function sample(engine: EngineName, file: File): Promise<void> {
  const code = await text(file);
  const start = performance.now();
  const highlighter = await createHighlighter({
    langs: [file.lang],
    themes: [theme],
    engine: engines[engine](),
  });
  const setup = performance.now();
  highlighter.codeToTokens(code, { lang: file.lang, theme, ...limitsOff });
  const first = performance.now();
  console.log(`${setup - start} ${first - setup}`);
}

/** The median of `values`. */
function median(values: number[]): number {
  const sorted = [...values].sort((a, b) => a - b);
  return sorted[Math.floor(sorted.length / 2)] ?? Number.NaN;
}

async function time(rounds: number): Promise<void> {
  const all = files();
  const names = Object.keys(engines) as EngineName[];
  for (const file of all) {
    const code = await text(file);
    const bytes = new TextEncoder().encode(code).length;
    for (const engine of names) {
      const cold = Bun.spawnSync(["bun", import.meta.path, "sample", engine, file.name]);
      const [setup, first] = cold.stdout.toString().trim().split(" ").map(Number);
      const highlighter = await createHighlighter({
        langs: [file.lang],
        themes: [theme],
        engine: engines[engine](),
      });
      const run = () => highlighter.codeToTokens(code, { lang: file.lang, theme, ...limitsOff });
      run();
      const speeds: number[] = [];
      for (let round = 0; round < rounds; round++) {
        let iterations = 0;
        const start = performance.now();
        while (performance.now() - start < 200) {
          run();
          iterations++;
        }
        const seconds = (performance.now() - start) / 1000;
        speeds.push((bytes * iterations) / seconds / (1 << 20));
      }
      console.log([engine, file.name, setup, first, median(speeds)].join("\t"));
      highlighter.dispose();
    }
  }
}

const [mode, ...rest] = Bun.argv.slice(2);
if (mode === "reference") {
  await reference();
} else if (mode === "time") {
  await time(Number(rest[0] ?? 5));
} else if (mode === "sample") {
  const [engine, name] = rest;
  const file = files().find((candidate) => candidate.name === name);
  if (file === undefined || !(engine in engines)) {
    console.error(`usage: shiki.ts sample <engine> <language/tier/file>`);
    process.exit(1);
  }
  await sample(engine as EngineName, file);
} else {
  console.error("usage: shiki.ts reference | time [rounds] | sample <engine> <file>");
  process.exit(1);
}
