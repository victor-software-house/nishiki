// Shiki's side of the comparison (bench/compare/README.md).
//
// `bun bench/compare/shiki.ts reference` writes Shiki's github-dark colours
// for every corpus file whose language Shiki has.
// They go to target/compare/shiki/<language>/<tier>/<file>.colors.
// Each row is one line, and each token is `<byte end>:<colour>`.
// Shiki's per-line limits are off.
// The Rust harness reads them to score every engine's fidelity to Shiki.
//
// `bun bench/compare/shiki.ts time [--rounds N] [--cold N] [--tier T]... [--tsv]`
// times Shiki and Pierre themselves on the same files.
// By default it runs 5 rounds and one cold process on every tier.
// It needs the reference colours that `reference` writes.
// It times both engines: Oniguruma in WebAssembly and the JavaScript engine.
// Setup and warm throughput are separate figures.
// Fresh Bun processes give the setup and first-document times.
// Rows are tab-separated: engine, file, setup ms, first document ms, warm
// MiB/s, tokens, fidelity, each time a median.
// With `--tsv`, rows follow the Rust harness's `--tsv` columns instead.
// They hold every cold sample and every round, for bench/compare/record.ts.
// Tokens are the tokens one call returns.
// Fidelity is the share of non-whitespace bytes whose colour matches the
// reference, in percent.
// It follows the Rust harness's rule, so the Oniguruma engine reads 100.
//
// `bun bench/compare/shiki.ts stream-reference` writes Shiki's colours for
// the text each trace of bench/traces/ ends with, to
// target/compare/shiki/traces/<origin>/<file>.colors.
// `bench:stream` scores nishiki against them.
// `bun bench/compare/shiki.ts stream [--passes N]` replays every trace with
// Shiki's `ShikiStreamTokenizer` and Pierre's `StreamTokenizer` and
// `LiveTokenizer`, each as its users drive it.
// An engine with no edit API skips the edit traces.
// Rows are tab-separated: engine, trace, deltas, setup ms, first delta µs,
// steady p50, p99 and maximum µs, and the final colours' fidelity.
import { bundledLanguages, createHighlighter, type RegexEngine } from "shiki@4.6.0";
import { createJavaScriptRegexEngine } from "shiki@4.6.0/engine/javascript";
import { createOnigurumaEngine } from "shiki@4.6.0/engine/oniguruma";
import { ShikiStreamTokenizer } from "@shikijs/stream@4.6.0";
import { loadTraces, type Trace } from "../traces.ts";
import { readdirSync, statSync } from "node:fs";
import { join } from "node:path";
import { parseArgs } from "node:util";

const root = new URL("../..", import.meta.url).pathname;
const corpus = join(root, "bench/corpus");
const out = join(root, "target/compare/shiki");
const theme = "github-dark";
const limitsOff = { tokenizeMaxLineLength: 0, tokenizeTimeLimit: 0 } as const;

type File = { lang: string; path: string; name: string };

/** Every corpus document whose language Shiki bundles, as `<language>/<tier>/<file>`.
 * The grammar tier holds TextMate grammars, not documents, so it is skipped
 * (bench/corpus/README.md). */
function files(): File[] {
  const found: File[] = [];
  for (const lang of readdirSync(corpus).sort()) {
    if (!(lang in bundledLanguages)) continue;
    const langDir = join(corpus, lang);
    if (!statSync(langDir).isDirectory()) continue;
    for (const tier of readdirSync(langDir).sort()) {
      if (tier === "grammar") continue;
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
  return langsOf(file.lang);
}

/** The grammars a highlighter for `lang` loads, as `langsFor` says. */
function langsOf(lang: string): string[] {
  return lang === "markdown" ? [...new Set(files().map((each) => each.lang))] : [lang];
}

/** Shiki's text: CRLF read as LF, as Shiki splits lines. */
async function text(file: File): Promise<string> {
  return (await Bun.file(file.path).text()).replaceAll("\r\n", "\n");
}

/** A document's colours: per line, each token's byte end and colour, lowered. */
type Colours = [end: number, colour: string][][];

const encoder = new TextEncoder();
const decoder = new TextDecoder();

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
  return coloursAt(join(out, `${file.name}.colors`));
}

/** The colours a `.colors` file holds. */
async function coloursAt(path: string): Promise<Colours> {
  if (!(await Bun.file(path).exists()))
    throw new Error(`${path} is missing: run shiki.ts reference first`);
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
 * harness's `fidelity`. */
function fidelity(code: string, ours: Colours, theirs: Colours): number {
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
    const rows = coloursOf(tokens).map((line) =>
      line.map(([end, colour]) => `${end}:${colour}`).join(" "),
    );
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
    run: (code) =>
      highlighter.codeToTokens(code, { lang: file.lang, theme: zedTheme, tokenizeMaxLineLength: 0 })
        .tokens,
    dispose: () => {},
  };
}

/** Each engine's setup, timed as a whole. */
const engines = {
  "shiki-oniguruma": (file: File) =>
    shikiReady(file, createOnigurumaEngine(import("shiki@4.6.0/wasm"))),
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

/** What `time` measures and how it prints, from the command line. */
type TimeOptions = { rounds: number; cold: number; tiers: string[]; output: "medians" | "tsv" };

function timeOptions(args: string[]): TimeOptions {
  const { values } = parseArgs({
    args,
    options: {
      rounds: { type: "string", default: "5" },
      cold: { type: "string", default: "1" },
      tier: { type: "string", multiple: true, default: [] },
      tsv: { type: "boolean", default: false },
    },
  });
  const rounds = Number(values.rounds);
  const cold = Number(values.cold);
  if (!Number.isInteger(rounds) || rounds < 1)
    throw new Error("--rounds needs a whole number of at least 1");
  if (!Number.isInteger(cold) || cold < 1)
    throw new Error("--cold needs a whole number of at least 1");
  return { rounds, cold, tiers: values.tier, output: values.tsv ? "tsv" : "medians" };
}

async function time(options: TimeOptions): Promise<void> {
  const all = files().filter(
    (file) => options.tiers.length === 0 || options.tiers.includes(file.name.split("/")[1] ?? ""),
  );
  const names = Object.keys(engines) as EngineName[];
  if (options.output === "tsv")
    console.log("file\tengine\tsetup_ms\tfirst_ms\tmib_per_s\ttokens\tfidelity");
  for (const file of all) {
    const code = await text(file);
    const bytes = encoder.encode(code).length;
    const shiki = await referenceOf(file);
    for (const engine of names) {
      const setups: number[] = [];
      const firsts: number[] = [];
      for (let sampled = 0; sampled < options.cold; sampled++) {
        const cold = Bun.spawnSync(["bun", import.meta.path, "sample", engine, file.name]);
        const [setup, first] = cold.stdout.toString().trim().split(" ").map(Number);
        setups.push(setup ?? Number.NaN);
        firsts.push(first ?? Number.NaN);
      }
      const ready = await engines[engine](file);
      const run = () => ready.run(code);
      run();
      const speeds: number[] = [];
      for (let round = 0; round < options.rounds; round++) {
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
      const score = fidelity(code, coloursOf(tokens), shiki);
      const row =
        options.output === "tsv"
          ? [file.name, engine, setups.join(","), firsts.join(","), speeds.join(","), count, score]
          : [
              engine,
              file.name,
              median(setups),
              median(firsts),
              median(speeds),
              count,
              score.toFixed(2),
            ];
      console.log(row.join("\t"));
      ready.dispose();
    }
  }
}

/** Where `stream-reference` writes Shiki's colours for each trace's final
 * text, as `<origin>/<file>.colors`. `bench:stream` reads them too. */
const streamOut = join(out, "traces");

/** Writes Shiki's colours for the text each trace ends with. */
async function streamReference(): Promise<void> {
  const all = loadTraces();
  const highlighter = await createHighlighter({
    langs: [...new Set(all.flatMap((trace) => langsOf(trace.language)))],
    themes: [theme],
  });
  for (const trace of all) {
    const tokens = highlighter.codeToTokens(trace.final.replaceAll("\r\n", "\n"), {
      lang: trace.language,
      theme,
      ...limitsOff,
    }).tokens;
    const rows = coloursOf(tokens).map((line) =>
      line.map(([end, colour]) => `${end}:${colour}`).join(" "),
    );
    await Bun.write(join(streamOut, `${trace.name}.colors`), `${rows.join("\n")}\n`);
  }
  console.log(`✔ ${all.length} trace references in ${streamOut}`);
}

/** One replay: setup in ms, each step's µs, and the colours it ends with. */
type Replay = { setup: number; deltas: number[]; colours: Colours };

/** Shiki's own streaming: `ShikiStreamTokenizer` from `@shikijs/stream`, the
 * way its renderers drive it. Every step is a push. */
async function shikiStream(trace: Trace): Promise<Replay> {
  const start = performance.now();
  const highlighter = await createHighlighter({ langs: langsOf(trace.language), themes: [theme] });
  const tokenizer = new ShikiStreamTokenizer({
    highlighter,
    lang: trace.language,
    theme,
    ...limitsOff,
  });
  const setup = performance.now() - start;
  const deltas: number[] = [];
  for (const step of trace.steps) {
    if (!("push" in step)) throw new Error("shiki-stream only appends");
    const at = performance.now();
    await tokenizer.enqueue(step.push);
    deltas.push((performance.now() - at) * 1000);
  }
  const tokens = [...tokenizer.tokensStable, ...tokenizer.close().stable];
  const lines: { content: string; color?: string }[][] = [[]];
  for (const token of tokens) {
    if (token.content === "\n") lines.push([]);
    else lines[lines.length - 1]!.push(token);
  }
  highlighter.dispose();
  return { setup, deltas, colours: coloursOf(lines) };
}

/** Pierre's Zed theme, as the Rust harness writes it. */
async function zedTheme(): Promise<unknown> {
  if (!(await Bun.file(pierreTheme).exists())) {
    throw new Error(`${pierreTheme} is missing: run mise run bench:compare, which writes it`);
  }
  return Bun.file(pierreTheme).json();
}

/** Pierre's `StreamTokenizer`: completed lines only, the last held until it
 * ends. Every step is a push. */
async function pierreStream(trace: Trace): Promise<Replay> {
  const theme = await zedTheme();
  const start = performance.now();
  const { StreamTokenizer } = await import("@pierre/highlights@0.4.0");
  const tokenizer = new StreamTokenizer({
    lang: trace.language,
    theme,
    tokenizeMaxLineLength: 0,
  } as never);
  const setup = performance.now() - start;
  const deltas: number[] = [];
  const lines: { content: string; color?: string }[][] = [];
  for (const step of trace.steps) {
    if (!("push" in step)) throw new Error("pierre-stream only appends");
    const at = performance.now();
    lines.push(...tokenizer.pushCode(step.push));
    deltas.push((performance.now() - at) * 1000);
  }
  lines.push(...tokenizer.end());
  tokenizer.dispose();
  return { setup, deltas, colours: coloursOf(lines) };
}

/** Pierre's `LiveTokenizer`: each step is one edit, an append at the end or a
 * replacement, lexed at once. */
async function pierreLive(trace: Trace): Promise<Replay> {
  const theme = await zedTheme();
  const start = performance.now();
  const { LiveTokenizer } = await import("@pierre/highlights@0.4.0");
  const live = new LiveTokenizer({
    lang: trace.language,
    theme,
    code: trace.initial,
    tokenizeMaxLineLength: 0,
  } as never);
  const setup = performance.now() - start;
  let model = trace.initial;
  const position = (index: number) => {
    const before = model.slice(0, index);
    const line = before.split("\n").length - 1;
    return { line, character: index - (before.lastIndexOf("\n") + 1) };
  };
  const deltas: number[] = [];
  for (const step of trace.steps) {
    let edit;
    if ("push" in step) {
      const last = live.lineCount - 1;
      const end = { line: last, character: live.getLineLength(last) };
      edit = { range: { start: end, end }, newText: step.push };
      model += step.push;
    } else {
      const bytes = encoder.encode(model);
      const from = decoder.decode(bytes.subarray(0, step.replace.start)).length;
      const to = decoder.decode(bytes.subarray(0, step.replace.end)).length;
      edit = { range: { start: position(from), end: position(to) }, newText: step.replace.text };
      model = model.slice(0, from) + step.replace.text + model.slice(to);
    }
    const at = performance.now();
    live.applyEdits([edit]);
    deltas.push((performance.now() - at) * 1000);
  }
  const lines = Array.from(
    { length: live.lineCount },
    (_, index) => live.getLineTokens(index).tokens,
  );
  live.dispose();
  return { setup, deltas, colours: coloursOf(lines) };
}

/** The stream engines, and which traces each can replay. */
const streamEngines = {
  "shiki-stream": { run: shikiStream, edits: false },
  "pierre-stream": { run: pierreStream, edits: false },
  "pierre-live": { run: pierreLive, edits: true },
} as const;

/** The value at nearest rank `perMille` / 1000 of sorted `values`. */
function rank(sorted: number[], perMille: number): number {
  return sorted[Math.max(0, Math.ceil((sorted.length * perMille) / 1000) - 1)] ?? Number.NaN;
}

/** Times each stream engine on each trace, as `bench:stream` times nishiki. */
async function stream(passes: number): Promise<void> {
  console.log("engine\ttrace\tdeltas\tsetup_ms\tfirst_us\tp50_us\tp99_us\tmax_us\tfidelity");
  for (const trace of loadTraces()) {
    const reference = await coloursAt(join(streamOut, `${trace.name}.colors`));
    for (const [engine, { run, edits }] of Object.entries(streamEngines)) {
      if (trace.origin === "edits" && !edits) continue;
      const replays: Replay[] = [];
      for (let pass = 0; pass < passes; pass++) replays.push(await run(trace));
      const steady = replays[0]!.deltas
        .map((_, delta) =>
          median(replays.slice(1).map((replay) => replay.deltas[delta] ?? Number.NaN)),
        )
        .sort((a, b) => a - b);
      const score = fidelity(trace.final.replaceAll("\r\n", "\n"), replays[0]!.colours, reference);
      const row = [
        engine,
        trace.name,
        steady.length,
        median(replays.map((replay) => replay.setup)).toFixed(3),
        (replays[0]!.deltas[0] ?? Number.NaN).toFixed(3),
        rank(steady, 500).toFixed(3),
        rank(steady, 990).toFixed(3),
        (steady[steady.length - 1] ?? Number.NaN).toFixed(3),
        score.toFixed(2),
      ];
      console.log(row.join("\t"));
    }
  }
}

const [mode, ...rest] = Bun.argv.slice(2);
if (mode === "reference") {
  await reference();
} else if (mode === "time") {
  await time(timeOptions(rest));
} else if (mode === "stream-reference") {
  await streamReference();
} else if (mode === "stream") {
  const passes = Number(
    parseArgs({ args: rest, options: { passes: { type: "string", default: "5" } } }).values.passes,
  );
  if (!Number.isInteger(passes) || passes < 2)
    throw new Error("--passes needs a whole number of at least 2");
  await stream(passes);
} else if (mode === "sample") {
  const [engine, name] = rest;
  const file = files().find((candidate) => candidate.name === name);
  if (file === undefined || !(engine in engines)) {
    console.error(`usage: shiki.ts sample <engine> <language/tier/file>`);
    process.exit(1);
  }
  await sample(engine as EngineName, file);
} else {
  console.error(
    "usage: shiki.ts reference | time [--rounds N] [--cold N] [--tier T]... [--tsv] | sample <engine> <file> | stream-reference | stream [--passes N]",
  );
  process.exit(1);
}
