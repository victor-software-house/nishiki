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
// It needs the reference colours that `reference` writes.
// It times both engines: Oniguruma in WebAssembly and the JavaScript engine.
// Setup and warm throughput are separate figures.
// A fresh Bun process per file gives the first-document time.
// Rows are tab-separated: engine, file, setup ms, first document ms, warm
// MiB/s, tokens, agreement.
// Tokens are the tokens one call returns.
// Agreement is the share of non-whitespace bytes whose colour matches the
// reference, in percent.
// It follows the Rust harness's rule, so the Oniguruma engine reads 100.
import { bundledLanguages, createHighlighter, type RegexEngine } from "shiki@4.6.0";
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

/** The grammars a highlighter for `file` loads. A language's module brings
 * the grammars it always embeds, as HTML brings CSS and JavaScript. Markdown
 * loads its fences' grammars only when asked. A Markdown file therefore loads
 * every corpus language, as `reference` does, so its fences are coloured. */
function langsFor(file: File): string[] {
  return file.lang === "markdown" ? [...new Set(files().map((each) => each.lang))] : [file.lang];
}

/** Shiki's text: CRLF read as LF, as Shiki splits lines. */
async function text(file: File): Promise<string> {
  return (await Bun.file(file.path).text()).replaceAll("\r\n", "\n");
}

/** A document's colours: per line, each token's byte end and colour, lowered. */
type Colours = [end: number, colour: string][][];

const encoder = new TextEncoder();

/** The colours of Shiki's tokens for one document. */
function coloursOf(tokens: { content: string; color?: string }[][]): Colours {
  return tokens.map((line) => {
    let end = 0;
    return line.map((token) => {
      end += encoder.encode(token.content).length;
      return [end, (token.color ?? "").toLowerCase()];
    });
  });
}

/** The reference colours of one file, as `reference` wrote them. */
async function referenceOf(file: File): Promise<Colours> {
  const path = join(out, `${file.name}.colors`);
  if (!(await Bun.file(path).exists())) throw new Error(`${path} is missing: run shiki.ts reference first`);
  return (await Bun.file(path).text())
    .replace(/\n$/, "")
    .split("\n")
    .map((row) =>
      row
        .split(" ")
        .filter((span) => span.includes(":"))
        .map((span) => {
          const at = span.indexOf(":");
          return [Number(span.slice(0, at)), span.slice(at + 1).toLowerCase()];
        }),
    );
}

/** The colour of each byte of a line of `length` bytes, from its spans. */
function perByte(spans: Colours[number], length: number): string[] {
  const colours: string[] = new Array(length).fill("");
  let start = 0;
  for (const [end, colour] of spans) {
    colours.fill(colour, start, Math.min(end, length));
    start = Math.max(start, Math.min(end, length));
  }
  return colours;
}

/** The share of `code`'s non-whitespace bytes that `ours` colours as
 * `theirs` does, in percent. A blank is a space or a tab, as in the Rust
 * harness's `agreement`. */
function agreement(code: string, ours: Colours, theirs: Colours): number {
  let same = 0;
  let total = 0;
  code.split("\n").forEach((line, index) => {
    const bytes = encoder.encode(line);
    const want = perByte(theirs[index] ?? [], bytes.length);
    const have = perByte(ours[index] ?? [], bytes.length);
    bytes.forEach((byte, at) => {
      if (byte === 0x20 || byte === 0x09) return;
      total++;
      if (want[at] === have[at]) same++;
    });
  });
  return total === 0 ? 100 : (same * 100) / total;
}

async function reference(): Promise<void> {
  const all = files();
  const highlighter = await createHighlighter({
    langs: [...new Set(all.map((file) => file.lang))],
    themes: [theme],
  });
  for (const file of all) {
    const tokens = highlighter.codeToTokens(await text(file), {
      lang: file.lang,
      theme,
      ...limitsOff,
    }).tokens;
    const rows = coloursOf(tokens).map((line) => line.map(([end, colour]) => `${end}:${colour}`).join(" "));
    await Bun.write(join(out, `${file.name}.colors`), `${rows.join("\n")}\n`);
  }
  console.log(`✔ ${all.length} reference files in ${out}`);
}

/** One engine, set up for one file's language. */
type Ready = {
  /** Colours `code`: the timed call. */
  run: (code: string) => { content: string; color?: string }[][];
  dispose: () => void;
};

/** A Shiki highlighter on `engine` for `file`. */
async function shikiReady(file: File, engine: RegexEngine | Promise<RegexEngine>): Promise<Ready> {
  const highlighter = await createHighlighter({ langs: langsFor(file), themes: [theme], engine });
  return {
    run: (code) => highlighter.codeToTokens(code, { lang: file.lang, theme, ...limitsOff }).tokens,
    dispose: () => highlighter.dispose(),
  };
}

/** github-dark as a Zed theme for `@pierre/highlights`. The Rust harness
 * writes it (`compare --pierre-theme`) from the map in its engines.rs. */
const pierreTheme = join(root, "target/compare/pierre-theme.json");

/** `@pierre/highlights` on its own WebAssembly instance. Its limit on line
 * length is off, as Shiki's is. Pierre compiles its WebAssembly when it is
 * imported. The import therefore sits inside the timed setup, as Shiki's
 * Oniguruma engine loads its WebAssembly there. */
async function pierreReady(file: File): Promise<Ready> {
  if (!(await Bun.file(pierreTheme).exists())) {
    throw new Error(`${pierreTheme} is missing: run mise run bench:compare, which writes it`);
  }
  const zedTheme = await Bun.file(pierreTheme).json();
  const { createHighlighter: createPierre } = await import("@pierre/highlights@0.4.0");
  const highlighter = createPierre();
  return {
    run: (code) => highlighter.codeToTokens(code, { lang: file.lang, theme: zedTheme, tokenizeMaxLineLength: 0 }).tokens,
    dispose: () => {},
  };
}

/** Each engine's setup, timed as a whole. */
const engines = {
  "shiki-oniguruma": (file: File) => shikiReady(file, createOnigurumaEngine(import("shiki@4.6.0/wasm"))),
  "shiki-javascript": (file: File) => shikiReady(file, createJavaScriptRegexEngine()),
  pierre: pierreReady,
} as const;
type EngineName = keyof typeof engines;

/** Times one fresh setup and one first document; prints both, in ms. */
async function sample(engine: EngineName, file: File): Promise<void> {
  const code = await text(file);
  const start = performance.now();
  const ready = await engines[engine](file);
  const setup = performance.now();
  ready.run(code);
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
    const bytes = encoder.encode(code).length;
    const shiki = await referenceOf(file);
    for (const engine of names) {
      const cold = Bun.spawnSync(["bun", import.meta.path, "sample", engine, file.name]);
      const [setup, first] = cold.stdout.toString().trim().split(" ").map(Number);
      const ready = await engines[engine](file);
      const run = () => ready.run(code);
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
      const tokens = run();
      const count = tokens.reduce((sum, line) => sum + line.length, 0);
      const score = agreement(code, coloursOf(tokens), shiki);
      console.log([engine, file.name, setup, first, median(speeds), count, score.toFixed(2)].join("\t"));
      ready.dispose();
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
