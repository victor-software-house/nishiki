#!/usr/bin/env bun
// The delta traces the stream benchmarks replay (openspec streaming, design
// decision 10).
//
// `bun bench/traces.ts write` writes every trace into bench/traces/.
// `bun bench/traces.ts check`, run by `mise run check:traces`, fails when a
// trace and its source disagree.
// An edit trace's source is the file at its `to` commit.
// Where the clone lacks that commit, as CI's shallow clone does, the check
// says so and holds the trace to the `sha256` that `write` read from git.
//
// Three kinds of trace:
//
// - reply: each reply-tier corpus file, cut into token-sized deltas;
// - tool-write: the first medium-tier corpus file of each language below,
//   cut the same way;
// - edits: the history of a few nishiki files, one replacement per hunk of
//   each commit's diff.
//
// The format is plain text, so the bench workspace reads it with std alone.
// A header line is a key, a space and a value.
// An append trace ends its header with `deltas`, then the byte length of each
// delta, 32 to a line.
// An edit trace ends its header with `base <bytes>`, then that many bytes of
// text and a line feed.
// Each replacement follows as `replace <start> <end> <bytes>`, then that many
// bytes of new text and a line feed.
// Offsets count bytes, as nishiki's API does.

import { $ } from "bun";
import { mkdirSync, readdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const root = new URL("..", import.meta.url).pathname;
const corpus = join(root, "bench/corpus");
const traces = join(root, "bench/traces");

/** The languages whose first medium-tier file becomes a tool-write trace. */
const TOOL_LANGUAGES = ["c", "cpp", "css", "go", "html", "java", "json", "python", "ruby", "rust", "shellscript", "typescript", "yaml"];

/** The nishiki files whose history becomes edit traces, with their language. */
const HISTORY: [path: string, language: string][] = [
  ["src/highlighter.rs", "rust"],
  ["src/limits.rs", "rust"],
  ["oracle/generate.ts", "typescript"],
  ["oracle/fuzz.ts", "typescript"],
  ["AGENTS.md", "markdown"],
  ["mise-tasks/callgrind.sh", "shellscript"],
];

/** The last commit the edit traces replay. */
const HISTORY_END = "68620d9ef4c7eb99a91c30f3a3d6788a2c4aaebf";
const REPOSITORY = "https://github.com/victor-software-house/nishiki";
const NISHIKI_LICENCE = "MIT OR Apache-2.0";

const encoder = new TextEncoder();
const decoder = new TextDecoder("utf-8", { fatal: true });

type Row = { path: string; language: string; tier: string; source: string; licence: string; sha256: string };

function corpusRows(): Map<string, Row> {
  const manifest = Bun.TOML.parse(readFileSync(join(root, "bench/corpus.toml"), "utf8")) as { file: Row[] };
  return new Map(manifest.file.map((row) => [row.path, row]));
}

function sha256(bytes: Uint8Array): string {
  return new Bun.CryptoHasher("sha256").update(bytes).digest("hex");
}

/** Cuts text into token-sized pieces, as the research note's probe does.
 * The split follows the pre-tokenizer pattern of byte-pair tokenizers, then
 * cuts any piece past six characters into four-character pieces. */
function tokenPieces(text: string): string[] {
  const pattern = /'(?:[sdmt]|ll|ve|re)| ?\p{L}+| ?\p{N}{1,3}| ?[^\s\p{L}\p{N}]+[\r\n]*|\s*[\r\n]+|\s+(?!\S)|\s+/gu;
  const pieces: string[] = [];
  for (const match of text.matchAll(pattern)) {
    const chars = [...match[0]];
    if (chars.length <= 6) {
      pieces.push(match[0]);
      continue;
    }
    for (let i = 0; i < chars.length; i += 4) pieces.push(chars.slice(i, i + 4).join(""));
  }
  if (pieces.join("") !== text) throw new Error("the pieces do not rebuild the text");
  return pieces;
}

function header(fields: [string, string][]): string {
  return fields.map(([key, value]) => `${key} ${value}\n`).join("");
}

/** An append trace of one corpus file. */
function appendTrace(kind: string, row: Row): string {
  const bytes = readFileSync(join(corpus, row.path));
  const lengths = tokenPieces(decoder.decode(bytes)).map((piece) => encoder.encode(piece).length);
  const rows: string[] = [];
  for (let i = 0; i < lengths.length; i += 32) rows.push(lengths.slice(i, i + 32).join(" "));
  return (
    header([
      ["kind", kind],
      ["language", row.language],
      ["source", `bench/corpus/${row.path}`],
      ["url", row.source],
      ["licence", row.licence],
      ["sha256", sha256(bytes)],
    ]) + `deltas\n${rows.join("\n")}\n`
  );
}

/** The append traces, by their path under bench/traces/. */
function appendTraces(): Map<string, string> {
  const rows = corpusRows();
  const out = new Map<string, string>();
  for (const row of [...rows.values()].filter((row) => row.tier === "reply").sort((a, b) => a.path.localeCompare(b.path))) {
    out.set(`reply/${row.path.split("/").pop()}.deltas`, appendTrace("reply", row));
  }
  for (const language of TOOL_LANGUAGES) {
    const first = [...rows.values()]
      .filter((row) => row.tier === "medium" && row.path.startsWith(`${language}/medium/`))
      .sort((a, b) => a.path.localeCompare(b.path))[0];
    if (first === undefined) throw new Error(`${language} has no medium-tier file`);
    out.set(`tool-write/${first.path.split("/").pop()}.deltas`, appendTrace("tool-write", first));
  }
  return out;
}

/** One hunk of a `-U0` diff: the old lines it replaces, from a 0-based line. */
type Hunk = { start: number; count: number; added: string[] };

function hunks(diff: string): Hunk[] {
  const out: Hunk[] = [];
  let current: Hunk | undefined;
  for (const row of diff.split("\n")) {
    const at = /^@@ -(\d+)(?:,(\d+))? \+\d+(?:,\d+)? @@/.exec(row);
    if (at) {
      const count = at[2] === undefined ? 1 : Number(at[2]);
      current = { start: count === 0 ? Number(at[1]) : Number(at[1]) - 1, count, added: [] };
      out.push(current);
    } else if (current && row.startsWith("+") && !row.startsWith("+++")) {
      current.added.push(row.slice(1));
    }
  }
  return out;
}

/** The byte offset where line `index` of `lines` starts. */
function lineOffset(lines: string[], index: number): number {
  let offset = 0;
  for (let i = 0; i < index; i++) offset += encoder.encode(lines[i]!).length + 1;
  return offset;
}

/** An edit trace of one nishiki file: its first version, then one
 * replacement per hunk, in order, each with offsets into the text before it. */
async function editTrace(path: string, language: string): Promise<string> {
  const commits = (await $`git -C ${root} log --format=%H --reverse ${HISTORY_END} -- ${path}`.text()).trim().split("\n");
  const base = await $`git -C ${root} show ${commits[0]}:${path}`.text();
  let lines = base.split("\n");
  let body = `base ${encoder.encode(base).length}\n${base}\n`;
  for (let c = 1; c < commits.length; c++) {
    const diff = await $`git -C ${root} diff --no-ext-diff -U0 ${commits[c - 1]} ${commits[c]} -- ${path}`.text();
    let shift = 0;
    for (const h of hunks(diff)) {
      const first = h.start + shift;
      const next = [...lines.slice(0, first), ...h.added, ...lines.slice(first + h.count)];
      const oldBytes = encoder.encode(lines.join("\n"));
      const newBytes = encoder.encode(next.join("\n"));
      const start = Math.min(lineOffset(lines, first), oldBytes.length, newBytes.length);
      const suffix = oldBytes.length - Math.min(lineOffset(lines, first + h.count), oldBytes.length);
      const text = decoder.decode(newBytes.subarray(start, newBytes.length - suffix));
      body += `replace ${start} ${oldBytes.length - suffix} ${encoder.encode(text).length}\n${text}\n`;
      lines = next;
      shift += h.added.length - h.count;
    }
  }
  const last = await $`git -C ${root} show ${commits[commits.length - 1]}:${path}`.text();
  const replayed = replay(parseEdits(body)).text;
  if (replayed !== last) throw new Error(`${path}: replaying the hunks does not give the file at ${commits[commits.length - 1]}`);
  return (
    header([
      ["kind", "edits"],
      ["language", language],
      ["source", path],
      ["url", `${REPOSITORY}/blob/${commits[0]}/${path}`],
      ["licence", NISHIKI_LICENCE],
      ["from", commits[0]!],
      ["to", commits[commits.length - 1]!],
      ["sha256", sha256(encoder.encode(last))],
    ]) + body
  );
}

type Edits = { base: Uint8Array; replacements: { start: number; end: number; text: Uint8Array }[] };

/** Reads the body of an edit trace, from its `base` line on. */
function parseEdits(body: string): Edits {
  const bytes = encoder.encode(body);
  let at = 0;
  const line = (): string => {
    const end = bytes.indexOf(10, at);
    const text = decoder.decode(bytes.subarray(at, end));
    at = end + 1;
    return text;
  };
  const blob = (length: number): Uint8Array => {
    const out = bytes.subarray(at, at + length);
    if (bytes[at + length] !== 10) throw new Error("a blob does not end with a line feed");
    at += length + 1;
    return out;
  };
  const first = line().split(" ");
  if (first[0] !== "base") throw new Error("an edit trace's body starts with `base`");
  const base = blob(Number(first[1]));
  const replacements: Edits["replacements"] = [];
  while (at < bytes.length) {
    const [word, start, end, length] = line().split(" ");
    if (word !== "replace") throw new Error(`unexpected line \`${word}\``);
    replacements.push({ start: Number(start), end: Number(end), text: blob(Number(length)) });
  }
  return { base, replacements };
}

/** Applies every replacement to the base text, checking each range. */
function replay(edits: Edits): { text: string; problems: string[] } {
  let text = edits.base;
  const problems: string[] = [];
  const boundary = (bytes: Uint8Array, i: number) => i === bytes.length || (bytes[i]! & 0xc0) !== 0x80;
  edits.replacements.forEach((r, i) => {
    if (r.start > r.end || r.end > text.length) problems.push(`replacement ${i + 1}: ${r.start}..${r.end} is outside the text`);
    else if (!boundary(text, r.start) || !boundary(text, r.end)) problems.push(`replacement ${i + 1}: ${r.start}..${r.end} cuts a character`);
    const next = new Uint8Array(text.length - (r.end - r.start) + r.text.length);
    next.set(text.subarray(0, r.start), 0);
    next.set(r.text, r.start);
    next.set(text.subarray(r.end), r.start + r.text.length);
    text = next;
  });
  return { text: decoder.decode(text), problems };
}

/** The header fields of a trace, and the rest of it. */
function split(trace: string, last: RegExp): { fields: Map<string, string>; rest: string } {
  const fields = new Map<string, string>();
  let at = 0;
  while (at < trace.length) {
    const end = trace.indexOf("\n", at);
    const row = trace.slice(at, end);
    if (last.test(row)) return { fields, rest: trace.slice(at) };
    const space = row.indexOf(" ");
    fields.set(row.slice(0, space), row.slice(space + 1));
    at = end + 1;
  }
  throw new Error("the trace has no body");
}

/** Builds every trace, then replaces the folders, so a failing git command
 * leaves the committed traces in place. */
async function write() {
  const all = appendTraces();
  for (const [path, language] of HISTORY) all.set(`edits/${path.replaceAll("/", "-")}.edits`, await editTrace(path, language));
  rmSync(join(traces, "reply"), { recursive: true, force: true });
  rmSync(join(traces, "tool-write"), { recursive: true, force: true });
  rmSync(join(traces, "edits"), { recursive: true, force: true });
  for (const [name, trace] of all) {
    mkdirSync(join(traces, name.split("/")[0]!), { recursive: true });
    writeFileSync(join(traces, name), trace);
  }
  console.log(`wrote ${all.size} traces`);
}

/** Whether the clone holds `commit`. */
async function holds(commit: string): Promise<boolean> {
  const object = `${commit}^{commit}`;
  return (await $`git -C ${root} cat-file -e ${object}`.nothrow().quiet()).exitCode === 0;
}

/** The file `path` at `commit`, or `undefined` where the commit has no such file. */
async function shown(commit: string, path: string): Promise<string | undefined> {
  const result = await $`git -C ${root} show ${commit}:${path}`.nothrow().quiet();
  return result.exitCode === 0 ? result.text() : undefined;
}

async function check() {
  const problems: string[] = [];
  const unread: string[] = [];
  const expected = appendTraces();
  for (const [name, trace] of expected) {
    let found: string;
    try {
      found = readFileSync(join(traces, name), "utf8");
    } catch {
      problems.push(`${name}: missing; run \`bun bench/traces.ts write\``);
      continue;
    }
    if (found !== trace) problems.push(`${name}: differs from its source; run \`bun bench/traces.ts write\``);
  }
  for (const kind of ["reply", "tool-write"]) {
    for (const file of readdirSync(join(traces, kind))) {
      if (!expected.has(`${kind}/${file}`)) problems.push(`${kind}/${file}: no source writes it`);
    }
  }
  const editFiles = readdirSync(join(traces, "edits")).sort();
  const wanted = HISTORY.map(([path]) => `${path.replaceAll("/", "-")}.edits`).sort();
  if (editFiles.join(" ") !== wanted.join(" ")) problems.push(`edits/: holds ${editFiles.join(", ")}, expected ${wanted.join(", ")}`);
  for (const file of editFiles) {
    const { fields, rest } = split(readFileSync(join(traces, "edits", file), "utf8"), /^base \d+$/);
    for (const key of ["kind", "language", "source", "url", "licence", "from", "to", "sha256"]) {
      if (!fields.has(key)) problems.push(`edits/${file}: no ${key} line`);
    }
    if (fields.get("licence") !== NISHIKI_LICENCE) problems.push(`edits/${file}: licence is not ${NISHIKI_LICENCE}`);
    const { text, problems: found } = replay(parseEdits(rest));
    problems.push(...found.map((problem) => `edits/${file}: ${problem}`));
    if (sha256(encoder.encode(text)) !== fields.get("sha256")) problems.push(`edits/${file}: replaying it does not give its sha256`);
    const to = fields.get("to");
    const path = fields.get("source");
    if (to === undefined || path === undefined) continue;
    if (!(await holds(to))) {
      unread.push(`edits/${file}`);
      continue;
    }
    const source = await shown(to, path);
    if (source === undefined) problems.push(`edits/${file}: ${path} does not exist at ${to}`);
    else if (source !== text) problems.push(`edits/${file}: replaying it does not give its source at its \`to\` commit`);
  }
  if (unread.length > 0) {
    console.error(`this clone lacks the \`to\` commit of ${unread.join(", ")}; those traces are held to their sha256 alone`);
  }
  if (problems.length > 0) {
    console.error(problems.join("\n"));
    process.exit(1);
  }
}

const mode = Bun.argv[2];
if (mode === "write") await write();
else if (mode === "check") await check();
else throw new Error("usage: traces.ts write|check");
