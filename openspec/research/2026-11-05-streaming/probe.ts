// Measures, with Shiki 4.6.0 and its per-line limits off, three questions of
// the streaming change (openspec/changes/streaming/design.md).
//
// - flicker: how often a byte already shown changes colour while a document
//   streams in token-sized deltas, for two ways of showing the line still
//   being written;
// - speculation: how many lines a lexer started from the initial state at a
//   viewport takes to reach the exact state, and how much of the first
//   screen it colours as the exact sweep would;
// - edits: how many lines past an edit must be lexed again before a line's
//   end state equals the one cached before the edit.
//
// Shiki's colours are nishiki's colours (the Identity and oracle tests), so
// these answers hold for nishiki's output.
// They count bytes, lines and states, never time.
//
// Usage: `bun openspec/research/2026-11-05-streaming/probe.ts flicker|speculation|edits`.
// Each mode prints tab-separated rows on stdout.
import {
  bundledLanguages,
  createHighlighter,
  type GrammarState,
  type ThemedToken,
} from "shiki@4.6.0";
import { readdirSync, statSync } from "node:fs";
import { join } from "node:path";
import { $ } from "bun";

const root = new URL("../../..", import.meta.url).pathname;
const corpus = join(root, "bench/corpus");
const theme = "github-dark";
const limitsOff = { tokenizeMaxLineLength: 0, tokenizeTimeLimit: 0 } as const;

/** Every corpus language Shiki bundles, so Markdown fences are coloured. */
const corpusLangs = readdirSync(corpus)
  .filter((lang) => lang in bundledLanguages && statSync(join(corpus, lang)).isDirectory())
  .sort();

const highlighter = await createHighlighter({ langs: corpusLangs, themes: [theme] });

/** One line's tokens and the state after it. */
function lexLine(line: string, lang: string, state: GrammarState | undefined) {
  const result = highlighter.codeToTokens(line, { lang, theme, grammarState: state, ...limitsOff });
  return { tokens: result.tokens[0] ?? [], state: result.grammarState };
}

/** The colour of each UTF-16 unit of a line, from its tokens. */
function perUnit(tokens: ThemedToken[]): string[] {
  const out: string[] = [];
  for (const token of tokens) {
    const colour = `${(token.color ?? "").toLowerCase()}/${token.fontStyle ?? 0}`;
    for (let i = 0; i < token.content.length; i++) out.push(colour);
  }
  return out;
}

function sameState(a: GrammarState | undefined, b: GrammarState | undefined): boolean {
  if (a === undefined || b === undefined) return a === b;
  return a.getInternalStack().equals(b.getInternalStack());
}

/** Splits text into token-sized pieces. The split follows the pre-tokenizer
 * pattern of byte-pair tokenizers, then cuts any piece past six characters
 * into four-character pieces, which approximates their subword merges. */
function tokenDeltas(text: string): string[] {
  const pattern =
    /'(?:[sdmt]|ll|ve|re)| ?\p{L}+| ?\p{N}{1,3}| ?[^\s\p{L}\p{N}]+[\r\n]*|\s*[\r\n]+|\s+(?!\S)|\s+/gu;
  const pieces: string[] = [];
  for (const match of text.matchAll(pattern)) {
    const piece = match[0];
    const chars = [...piece];
    if (chars.length <= 6) {
      pieces.push(piece);
      continue;
    }
    for (let i = 0; i < chars.length; i += 4) pieces.push(chars.slice(i, i + 4).join(""));
  }
  if (pieces.join("") !== text) throw new Error("the deltas do not rebuild the text");
  return pieces;
}

type Doc = { lang: string; name: string; text: string };

function doc(path: string): Doc {
  const [lang, tier, file] = path.split("/");
  const text = require("node:fs").readFileSync(join(corpus, path), "utf8").replaceAll("\r\n", "\n");
  return { lang: lang!, name: `${lang}/${tier}/${file}`, text };
}

/** The first file of `tier` for each language, sorted by name. */
function firstOf(tier: string, langs: string[]): Doc[] {
  return langs.flatMap((lang) => {
    const dir = join(corpus, lang, tier);
    try {
      const file = readdirSync(dir).sort()[0];
      return file === undefined ? [] : [doc(`${lang}/${tier}/${file}`)];
    } catch {
      return [];
    }
  });
}

const replies = readdirSync(join(corpus, "markdown/reply"))
  .sort()
  .map((file) => doc(`markdown/reply/${file}`));
const toolLangs = [
  "c",
  "cpp",
  "css",
  "go",
  "html",
  "java",
  "json",
  "python",
  "ruby",
  "rust",
  "shellscript",
  "typescript",
  "yaml",
];

/** How a render shows the line still being written: lexed as if it ended
 * there, with its last token or last two tokens held back, or held back
 * whole until its line ending arrives. */
const MODES = { "as-ended": 0, "hold-last": 1, "hold-last-2": 2, "hold-line": Infinity } as const;
type Mode = keyof typeof MODES;

/**
 * Streams `d` in token-sized deltas and renders after each.
 * A shown unit holds a colour. A held-back unit shows in the default colour
 * and counts as not yet coloured. A flicker is a coloured unit whose colour differs from its
 * colour in the previous render.
 */
function flicker(d: Doc, mode: Mode) {
  const deltas = tokenDeltas(d.text);
  let state: GrammarState | undefined;
  let line = "";
  let shown: (string | null)[] = [];
  let flickerUnits = 0;
  let rendersWithFlicker = 0;
  let pendingUnitRenders = 0;
  for (const delta of deltas) {
    const parts = (line + delta).split("\n");
    for (const done of parts.slice(0, -1)) {
      const lexed = lexLine(done, d.lang, state);
      const colours = perUnit(lexed.tokens);
      let changed = 0;
      for (let i = 0; i < shown.length; i++) {
        if (shown[i] !== null && shown[i] !== colours[i]) changed++;
      }
      flickerUnits += changed;
      if (changed > 0) rendersWithFlicker++;
      state = lexed.state;
      shown = [];
    }
    line = parts[parts.length - 1]!;
    const lexed = lexLine(line, d.lang, state);
    const colours: (string | null)[] = perUnit(lexed.tokens);
    const held = lexed.tokens.slice(Math.max(0, lexed.tokens.length - MODES[mode]));
    const heldUnits = held.reduce((sum, token) => sum + token.content.length, 0);
    for (let i = line.length - heldUnits; i < line.length; i++) colours[i] = null;
    let changed = 0;
    for (let i = 0; i < shown.length; i++) {
      if (shown[i] !== null && colours[i] !== null && shown[i] !== colours[i]) changed++;
    }
    flickerUnits += changed;
    if (changed > 0) rendersWithFlicker++;
    pendingUnitRenders += colours.filter((c) => c === null).length;
    shown = colours;
  }
  return {
    deltas: deltas.length,
    units: d.text.length,
    flickerUnits,
    rendersWithFlicker,
    pendingUnitRenders,
  };
}

function runFlicker() {
  console.log(
    [
      "trace",
      "file",
      "mode",
      "deltas",
      "units",
      "flicker_units",
      "renders_with_flicker",
      "flicker_per_kib",
      "pending_units_per_render",
    ].join("\t"),
  );
  const traces: [string, Doc[]][] = [
    ["reply", replies],
    ["tool-write", firstOf("medium", toolLangs)],
  ];
  for (const [trace, docs] of traces) {
    for (const d of docs) {
      for (const mode of Object.keys(MODES) as Mode[]) {
        const r = flicker(d, mode);
        console.log(
          [
            trace,
            d.name,
            mode,
            r.deltas,
            r.units,
            r.flickerUnits,
            r.rendersWithFlicker,
            ((r.flickerUnits * 1024) / r.units).toFixed(2),
            (r.pendingUnitRenders / r.deltas).toFixed(2),
          ].join("\t"),
        );
      }
    }
  }
}

/** Each line's tokens and end state, lexed in order from the start. */
function exact(d: Doc) {
  const lines = d.text.split("\n");
  const states: (GrammarState | undefined)[] = [];
  const colours: string[][] = [];
  let state: GrammarState | undefined;
  for (const line of lines) {
    const lexed = lexLine(line, d.lang, state);
    state = lexed.state;
    states.push(state);
    colours.push(perUnit(lexed.tokens));
  }
  return { lines, states, colours };
}

const SCREEN = 60;
const CAP = 2000;

/**
 * For viewports at a tenth, a quarter, a half, three quarters and nine
 * tenths of the file: lexes from the initial state at the viewport and
 * counts the lines until its end state equals the exact one (capped), and
 * the share of the first screen's non-blank units it colours as the exact
 * lexing does.
 */
function runSpeculation() {
  console.log("file\tlines\tviewport\tlines_to_converge\tscreen_fidelity_pct");
  for (const d of [...firstOf("medium", toolLangs), ...firstOf("large", toolLangs), ...replies]) {
    const e = exact(d);
    for (const at of [0.1, 0.25, 0.5, 0.75, 0.9]) {
      const start = Math.floor(e.lines.length * at);
      let state: GrammarState | undefined;
      let converged = -1;
      let same = 0;
      let total = 0;
      for (let i = start; i < Math.min(e.lines.length, start + CAP); i++) {
        const lexed = lexLine(e.lines[i]!, d.lang, state);
        state = lexed.state;
        if (i < start + SCREEN) {
          const ours = perUnit(lexed.tokens);
          const text = e.lines[i]!;
          for (let u = 0; u < text.length; u++) {
            if (text[u] === " " || text[u] === "\t") continue;
            total++;
            if (ours[u] === e.colours[i]![u]) same++;
          }
        }
        if (converged < 0 && sameState(state, e.states[i])) converged = i - start + 1;
        if (converged >= 0 && i >= start + SCREEN) break;
      }
      const pct = total === 0 ? 100 : (100 * same) / total;
      console.log(
        [d.name, e.lines.length, at, converged < 0 ? `>${CAP}` : converged, pct.toFixed(2)].join(
          "\t",
        ),
      );
    }
  }
}

/** One hunk of a `-U0` diff: old lines replaced by new lines. */
type Hunk = { oldStart: number; oldCount: number; added: string[] };

function hunks(diff: string): Hunk[] {
  const out: Hunk[] = [];
  let current: Hunk | undefined;
  for (const row of diff.split("\n")) {
    const header = /^@@ -(\d+)(?:,(\d+))? \+(\d+)(?:,(\d+))? @@/.exec(row);
    if (header) {
      const oldCount = header[2] === undefined ? 1 : Number(header[2]);
      const oldStart = oldCount === 0 ? Number(header[1]) : Number(header[1]) - 1;
      current = { oldStart, oldCount, added: [] };
      out.push(current);
    } else if (current && row.startsWith("+") && !row.startsWith("+++")) {
      current.added.push(row.slice(1));
    }
  }
  return out;
}

/**
 * Replays the history of a few nishiki files, one hunk at a time, as edits.
 * After each hunk, lexes from its first line through its new lines, then on
 * until a line's end state equals the state cached before the edit for the
 * same text line. Counts those lines past the hunk.
 */
async function runEdits() {
  console.log(
    "file\tcommits\thunks\tfile_lines_mean\ttail_mean\ttail_p50\ttail_p99\ttail_max\tto_end",
  );
  const files: [string, string][] = [
    ["src/highlighter.rs", "rust"],
    ["src/limits.rs", "rust"],
    ["oracle/generate.ts", "typescript"],
    ["oracle/fuzz.ts", "typescript"],
    ["AGENTS.md", "markdown"],
    ["mise-tasks/callgrind.sh", "shellscript"],
  ];
  for (const [file, lang] of files) {
    const commits = (await $`git -C ${root} log --format=%H --reverse 68620d9e -- ${file}`.text())
      .trim()
      .split("\n");
    const tails: number[] = [];
    let toEnd = 0;
    let lineSum = 0;
    for (let c = 1; c < commits.length; c++) {
      const before = await $`git -C ${root} show ${commits[c - 1]}:${file}`.nothrow().quiet();
      const diff =
        await $`git -C ${root} diff -U0 ${commits[c - 1]} ${commits[c]} -- ${file}`.text();
      if (before.exitCode !== 0) continue;
      let lines = before.text().split("\n");
      let shift = 0;
      for (const h of hunks(diff)) {
        let states: (GrammarState | undefined)[] = [];
        let state: GrammarState | undefined;
        for (const line of lines) {
          state = lexLine(line, lang, state).state;
          states.push(state);
        }
        const at = h.oldStart + shift;
        const next = [...lines.slice(0, at), ...h.added, ...lines.slice(at + h.oldCount)];
        state = at === 0 ? undefined : states[at - 1];
        let tail = 0;
        let reachedEnd = true;
        for (let i = at; i < next.length; i++) {
          if (i >= at + h.added.length) {
            const old = i - h.added.length + h.oldCount;
            if (sameState(state, old === 0 ? undefined : states[old - 1])) {
              reachedEnd = false;
              break;
            }
            tail++;
          }
          state = lexLine(next[i]!, lang, state).state;
        }
        if (reachedEnd) toEnd++;
        tails.push(tail);
        lineSum += next.length;
        shift += h.added.length - h.oldCount;
        lines = next;
      }
      shift = 0;
    }
    tails.sort((a, b) => a - b);
    const q = (p: number) => tails[Math.min(tails.length - 1, Math.floor(p * tails.length))] ?? 0;
    const mean = tails.reduce((a, b) => a + b, 0) / Math.max(1, tails.length);
    console.log(
      [
        file,
        commits.length,
        tails.length,
        (lineSum / Math.max(1, tails.length)).toFixed(0),
        mean.toFixed(2),
        q(0.5),
        q(0.99),
        tails[tails.length - 1] ?? 0,
        toEnd,
      ].join("\t"),
    );
  }
}

const mode = Bun.argv[2];
if (mode === "flicker") runFlicker();
else if (mode === "speculation") runSpeculation();
else if (mode === "edits") await runEdits();
else throw new Error("usage: probe.ts flicker|speculation|edits");
